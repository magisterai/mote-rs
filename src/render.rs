//! Native PDF and PNG painters for resolved document geometry.
mod fonts;
mod pdf;
mod png;
pub use pdf::paint_pdf;
pub use png::{PngOptions, RasterPage, paint_png};

use crate::{
    layout::{DocumentTextPlan, LayoutError, plan_document_text},
    measurement::{MeasurementError, TextMeasurer},
    model::Document,
    source::AssetResolver,
};
use std::{collections::BTreeSet, ffi::OsStr, fmt};

#[derive(Debug)]
pub enum RenderError {
    Layout(LayoutError),
    Measurement(MeasurementError),
    Invalid(String),
    Io(std::io::Error),
}
impl fmt::Display for RenderError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Layout(error) => error.fmt(formatter),
            Self::Measurement(error) => error.fmt(formatter),
            Self::Invalid(message) => formatter.write_str(message),
            Self::Io(error) => error.fmt(formatter),
        }
    }
}
impl std::error::Error for RenderError {}
impl From<LayoutError> for RenderError {
    fn from(error: LayoutError) -> Self {
        Self::Layout(error)
    }
}
impl From<MeasurementError> for RenderError {
    fn from(error: MeasurementError) -> Self {
        Self::Measurement(error)
    }
}
impl From<std::io::Error> for RenderError {
    fn from(error: std::io::Error) -> Self {
        Self::Io(error)
    }
}

#[derive(Debug)]
pub struct Rendered<T> {
    pub output: T,
    pub warnings: Vec<String>,
}

pub fn plan(
    document: &Document,
    resolver: &dyn AssetResolver,
    source_name: Option<&OsStr>,
) -> Result<Rendered<DocumentTextPlan>, RenderError> {
    let mut measurer = TextMeasurer::new(document, resolver, source_name);
    let output = plan_document_text(document, &mut measurer)?;
    let mut warnings = BTreeSet::new();
    for fragment in output
        .pages
        .iter()
        .flat_map(|page| page.text_lines())
        .flat_map(|line| &line.fragments)
    {
        if let Some(reason) = &fragment.font.substitution {
            warnings.insert(reason.clone());
        }
        for character in &fragment.missing_codepoints {
            warnings.insert(format!(
                "font {:?} {} replaces missing glyph U+{:04X} with '?'",
                fragment.font.family,
                fragment.font.variant.as_text(),
                *character as u32
            ));
        }
    }
    Ok(Rendered {
        output,
        warnings: warnings.into_iter().collect(),
    })
}

pub fn emit_pdf(
    document: &Document,
    resolver: &dyn AssetResolver,
    source_name: Option<&OsStr>,
) -> Result<Rendered<Vec<u8>>, RenderError> {
    let planned = plan(document, resolver, source_name)?;
    Ok(Rendered {
        output: paint_pdf(&planned.output)?,
        warnings: planned.warnings,
    })
}

pub fn emit_png(
    document: &Document,
    resolver: &dyn AssetResolver,
    source_name: Option<&OsStr>,
    options: &PngOptions,
) -> Result<Rendered<Vec<RasterPage>>, RenderError> {
    let planned = plan(document, resolver, source_name)?;
    let mut painted = paint_png(&planned.output, options)?;
    painted.warnings.extend(planned.warnings);
    painted.warnings.sort();
    painted.warnings.dedup();
    Ok(painted)
}

fn color(value: &str) -> Result<[u8; 3], RenderError> {
    if value.len() != 6 {
        return Err(RenderError::Invalid(format!(
            "invalid paint color: {value:?}"
        )));
    }
    let number = u32::from_str_radix(value, 16)
        .map_err(|_| RenderError::Invalid(format!("invalid paint color: {value:?}")))?;
    Ok([(number >> 16) as u8, (number >> 8) as u8, number as u8])
}

fn validate_plan(plan: &DocumentTextPlan) -> Result<(), RenderError> {
    let geometry = plan.geometry;
    if !geometry.width.is_finite()
        || !geometry.height.is_finite()
        || geometry.width <= 0.0
        || geometry.height <= 0.0
        || geometry.width > 14400.0
        || geometry.height > 14400.0
    {
        return Err(RenderError::Invalid(
            "painted page dimensions must be positive and at most 14,400 points".into(),
        ));
    }
    if plan.pages.len() > i32::MAX as usize {
        return Err(RenderError::Invalid("too many pages".into()));
    }
    for page in &plan.pages {
        let mut seen = vec![false; page.lines.len()];
        for operation in &page.operations {
            match operation {
                crate::layout::PageOperation::Text(index) => {
                    let Some(visited) = seen.get_mut(*index) else {
                        return Err(RenderError::Invalid(
                            "page operation refers to a missing text line".into(),
                        ));
                    };
                    if *visited {
                        return Err(RenderError::Invalid("page repeats a text line".into()));
                    }
                    *visited = true;
                }
                crate::layout::PageOperation::Rectangle(rectangle) => {
                    if ![
                        rectangle.x,
                        rectangle.top,
                        rectangle.width,
                        rectangle.height,
                    ]
                    .iter()
                    .all(|value| value.is_finite())
                        || rectangle.width <= 0.0
                        || rectangle.height <= 0.0
                    {
                        return Err(RenderError::Invalid("invalid page rectangle".into()));
                    }
                    color(&rectangle.color)?;
                }
                crate::layout::PageOperation::Rule(rule) => {
                    if !rule
                        .start
                        .iter()
                        .chain(&rule.end)
                        .chain([&rule.width])
                        .all(|value| value.is_finite())
                        || rule.width <= 0.0
                    {
                        return Err(RenderError::Invalid("invalid page rule".into()));
                    }
                    color(&rule.color)?;
                }
                crate::layout::PageOperation::ListMarker(marker) => {
                    if !marker
                        .slot
                        .iter()
                        .chain([&marker.baseline, &marker.font_size])
                        .all(|value| value.is_finite())
                        || marker.slot[2] <= 0.0
                        || marker.slot[3] <= 0.0
                        || marker.font_size <= 0.0
                    {
                        return Err(RenderError::Invalid("invalid list marker slot".into()));
                    }
                }
            }
        }
        if seen.iter().any(|visited| !visited) {
            return Err(RenderError::Invalid("page omits a text line".into()));
        }
    }
    for line in plan.pages.iter().flat_map(|page| page.text_lines()) {
        if ![line.x, line.baseline, line.height, line.width]
            .iter()
            .all(|value| value.is_finite())
            || line.height <= 0.0
            || line.width < 0.0
        {
            return Err(RenderError::Invalid(
                "invalid painted text line geometry".into(),
            ));
        }
        for fragment in &line.fragments {
            if ![
                fragment.x,
                fragment.baseline,
                fragment.width,
                fragment.style.measurement.size_points,
            ]
            .iter()
            .all(|value| value.is_finite())
                || fragment.style.measurement.size_points <= 0.0
            {
                return Err(RenderError::Invalid(
                    "invalid painted text fragment geometry".into(),
                ));
            }
            color(&fragment.style.color)?;
        }
        for rectangle in &line.highlights {
            if ![
                rectangle.x,
                rectangle.top,
                rectangle.width,
                rectangle.height,
            ]
            .iter()
            .all(|value| value.is_finite())
                || rectangle.width < 0.0
                || rectangle.height < 0.0
            {
                return Err(RenderError::Invalid(
                    "invalid painted rectangle geometry".into(),
                ));
            }
            color(&rectangle.color)?;
        }
        for stroke in &line.decorations {
            if ![stroke.start, stroke.end, stroke.y, stroke.width]
                .iter()
                .all(|value| value.is_finite())
                || stroke.width <= 0.0
            {
                return Err(RenderError::Invalid(
                    "invalid painted stroke geometry".into(),
                ));
            }
            color(&stroke.color)?;
        }
    }
    Ok(())
}
