//! Native PDF and PNG painters for resolved document geometry.
mod fonts;
mod pdf;
mod png;
mod support;
pub use pdf::paint_pdf;
pub use png::{PngOptions, RasterPage, paint_png};

use crate::{
    layout::{DocumentTextPlan, LayoutError, plan_document_with_images},
    measurement::{FontResolver, MeasurementError, SystemFontResolver, TextMeasurer},
    model::Document,
    source::AssetResolver,
};
use std::{ffi::OsStr, fmt};

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
    let fonts = SystemFontResolver::new();
    plan_with_fonts(document, resolver, source_name, Some(&fonts))
}

/// Passing no font resolver makes layout independent of installed font discovery.
pub fn plan_with_fonts(
    document: &Document,
    resolver: &dyn AssetResolver,
    source_name: Option<&OsStr>,
    fonts: Option<&dyn FontResolver>,
) -> Result<Rendered<DocumentTextPlan>, RenderError> {
    plan_with_layout_and_fonts(
        document,
        resolver,
        source_name,
        LayoutMode::Automatic,
        fonts,
    )
}

/// Automatic mode selects slides only for an explicit presentation declaration.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum LayoutMode {
    #[default]
    Automatic,
    Document,
    Presentation,
}
pub fn plan_with_layout(
    document: &Document,
    resolver: &dyn AssetResolver,
    source_name: Option<&OsStr>,
    mode: LayoutMode,
) -> Result<Rendered<DocumentTextPlan>, RenderError> {
    let fonts = SystemFontResolver::new();
    plan_with_layout_and_fonts(document, resolver, source_name, mode, Some(&fonts))
}
pub fn plan_with_layout_and_fonts(
    document: &Document,
    resolver: &dyn AssetResolver,
    source_name: Option<&OsStr>,
    mode: LayoutMode,
    fonts: Option<&dyn FontResolver>,
) -> Result<Rendered<DocumentTextPlan>, RenderError> {
    let presentation = mode == LayoutMode::Presentation
        || (mode == LayoutMode::Automatic && !document.presentation.properties.is_empty());
    let mut measurer = TextMeasurer::new(document, resolver, source_name);
    if let Some(fonts) = fonts {
        measurer = measurer.with_font_resolver(fonts);
    }
    let (output, mut warnings) = if presentation {
        (
            crate::layout::plan_presentation(document, &mut measurer)?,
            std::collections::BTreeSet::new(),
        )
    } else {
        let mut images = crate::media::ImageCatalog::load(document, resolver, source_name);
        images.prepare_all(&document.content);
        let output = plan_document_with_images(document, &mut measurer, &images)?;
        let mut warnings = support::warnings(document);
        warnings.extend(images.warnings);
        (output, warnings)
    };
    warnings.extend(output.warnings.iter().cloned());
    for fragment in output
        .pages
        .iter()
        .flat_map(|page| page.text_lines())
        .flat_map(|line| &line.fragments)
    {
        if let Some(reason) = &fragment.font.substitution {
            warnings.insert(format!(
                "font {:?} {} was substituted: {reason}",
                fragment.style.measurement.font,
                fragment.font.variant.as_text()
            ));
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
    for geometry in
        std::iter::once(plan.geometry).chain(plan.pages.iter().filter_map(|page| page.geometry))
    {
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
    }
    if plan.pages.len() > i32::MAX as usize {
        return Err(RenderError::Invalid("too many pages".into()));
    }
    for page in &plan.pages {
        let mut seen = vec![false; page.lines.len()];
        let mut clip_depth = 0usize;
        let mut rotation_clips = Vec::new();
        for operation in &page.operations {
            match operation {
                crate::layout::PageOperation::PushRotation { degrees, center } => {
                    if !degrees.is_finite()
                        || center.iter().any(|value| !value.is_finite())
                        || rotation_clips.len() >= 16
                    {
                        return Err(RenderError::Invalid(
                            "invalid or excessively nested page rotation".into(),
                        ));
                    }
                    rotation_clips.push(clip_depth);
                }
                crate::layout::PageOperation::PopRotation => {
                    if rotation_clips.pop() != Some(clip_depth) {
                        return Err(RenderError::Invalid("unbalanced page rotation".into()));
                    }
                }
                crate::layout::PageOperation::Path(path) => {
                    if path.points.len() < if path.closed { 3 } else { 2 }
                        || path.points.iter().flatten().any(|value| !value.is_finite())
                    {
                        return Err(RenderError::Invalid("invalid painted path geometry".into()));
                    }
                    if let Some(fill) = &path.fill {
                        color(&fill.color)?;
                    }
                    if let Some(stroke) = &path.stroke {
                        color(&stroke.paint.color)?;
                        if !stroke.width.is_finite()
                            || stroke.width <= 0.0
                            || stroke
                                .dash
                                .iter()
                                .any(|value| !value.is_finite() || *value <= 0.0)
                        {
                            return Err(RenderError::Invalid("invalid painted path stroke".into()));
                        }
                    }
                }

                crate::layout::PageOperation::PaintedRectangle {
                    bounds,
                    fill,
                    stroke,
                } => {
                    if bounds.iter().any(|value| !value.is_finite())
                        || bounds[2] <= 0.0
                        || bounds[3] <= 0.0
                    {
                        return Err(RenderError::Invalid(
                            "invalid painted rectangle geometry".into(),
                        ));
                    }
                    if let Some(fill) = fill {
                        color(&fill.color)?;
                    }
                    if let Some(stroke) = stroke {
                        color(&stroke.paint.color)?;
                        if !stroke.width.is_finite()
                            || stroke.width <= 0.0
                            || stroke
                                .dash
                                .iter()
                                .any(|value| !value.is_finite() || *value <= 0.0)
                        {
                            return Err(RenderError::Invalid(
                                "invalid painted rectangle stroke".into(),
                            ));
                        }
                    }
                }
                crate::layout::PageOperation::PushClip(bounds) => {
                    if !bounds.iter().all(|value| value.is_finite())
                        || bounds[2] <= 0.0
                        || bounds[3] <= 0.0
                    {
                        return Err(RenderError::Invalid("invalid page clip".into()));
                    }
                    clip_depth += 1;
                }
                crate::layout::PageOperation::PopClip => {
                    if rotation_clips
                        .last()
                        .is_some_and(|depth| clip_depth <= *depth)
                    {
                        return Err(RenderError::Invalid(
                            "page clip crosses a rotation boundary".into(),
                        ));
                    }
                    clip_depth = clip_depth
                        .checked_sub(1)
                        .ok_or_else(|| RenderError::Invalid("unbalanced page clip".into()))?;
                }
                crate::layout::PageOperation::Stroke { rule, dash, .. } => {
                    if !rule
                        .start
                        .iter()
                        .chain(&rule.end)
                        .chain([&rule.width])
                        .all(|value| value.is_finite())
                        || rule.width <= 0.0
                        || dash.iter().any(|value| !value.is_finite() || *value <= 0.0)
                    {
                        return Err(RenderError::Invalid("invalid patterned rule".into()));
                    }
                    color(&rule.color)?;
                }
                crate::layout::PageOperation::OutlineRectangle {
                    bounds,
                    width,
                    color: color_value,
                } => {
                    if !bounds.iter().chain([width]).all(|value| value.is_finite())
                        || bounds[2] <= 0.0
                        || bounds[3] <= 0.0
                        || *width <= 0.0
                    {
                        return Err(RenderError::Invalid("invalid rectangle outline".into()));
                    }
                    color(color_value)?;
                }
                crate::layout::PageOperation::Image(image) => {
                    let raster = &image.image.raster;
                    let byte_count = u64::from(raster.width)
                        .checked_mul(u64::from(raster.height))
                        .and_then(|pixels| pixels.checked_mul(4));
                    if !image
                        .bounds
                        .iter()
                        .chain(&image.clip)
                        .all(|value| value.is_finite())
                        || image.bounds[2] <= 0.0
                        || image.bounds[3] <= 0.0
                        || image.clip[2] <= 0.0
                        || image.clip[3] <= 0.0
                        || raster.width == 0
                        || raster.height == 0
                        || byte_count != Some(raster.pixels.len() as u64)
                    {
                        return Err(RenderError::Invalid("invalid positioned image".into()));
                    }
                }
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
        if !rotation_clips.is_empty() {
            return Err(RenderError::Invalid("unbalanced page rotation".into()));
        }
        if clip_depth != 0 {
            return Err(RenderError::Invalid("unbalanced page clip".into()));
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
