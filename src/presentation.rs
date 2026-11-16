//! Resolved deck geometry and defaults shared by editable and native slides.
pub(crate) mod animation;
mod automatic;
pub(crate) mod backgrounds;
pub(crate) mod fields;
pub(crate) mod images;
pub(crate) mod layouts;
mod segmentation;
mod stacks;
pub(crate) mod support;
pub(crate) mod tables;
pub(crate) mod text;
use crate::{
    model::{Document, Element, ElementKind, Properties, Value},
    properties::{object, text},
};

pub(crate) struct Deck {
    pub slides: Vec<Slide>,
    pub width: i64,
    pub height: i64,
    pub size_type: &'static str,
    pub theme: Theme,
    pub warnings: Vec<String>,
    /// The time in seconds at which the slides are rendered. Without one a
    /// slide is rendered as authored, its effects disregarded.
    pub time: Option<f64>,
}
pub(crate) struct Slide {
    pub element: Element,
    pub background: backgrounds::Background,
    pub metadata: fields::SlideMetadata,
}
impl Deck {
    pub fn resolve(
        document: &Document,
        measurer: &mut crate::measurement::TextMeasurer<'_>,
    ) -> Result<Self, String> {
        let properties = &document.presentation.properties;
        let dimension = |name: &str| {
            document
                .presentation
                .dimensions
                .get(name)
                .map(|value| {
                    units(Some(value)).ok_or_else(|| format!("invalid presentation {name}"))
                })
                .transpose()
        };
        let width = dimension("width")?;
        let height = dimension("height")?;
        let (width, height, size_type) = match (width, height) {
            (Some(width), Some(height)) => (width, height, "custom"),
            _ if text(properties, "size") == "standard" => (9144000, 6858000, "screen4x3"),
            _ => (12192000, 6858000, "screen16x9"),
        };
        if width <= 0 || height <= 0 {
            return Err("presentation dimensions must be positive".into());
        }
        let mut warnings = Vec::new();
        for name in properties.keys() {
            if ![
                "size",
                "width",
                "height",
                "background",
                "theme",
                "footer",
                "date",
                "slide_number",
                "render_at",
            ]
            .contains(&name.as_str())
            {
                warnings.push(format!(
                    "presentation.{name} is retained without an output effect"
                ));
            }
        }
        let mut slides = segmentation::slides(&document.content);
        if slides.is_empty() {
            slides.push(Element {
                source_name: None,
                source_location: None,
                kind: ElementKind::Slide,
                attributes: Properties::new(),
                children: Vec::new(),
            });
        }
        let layouts = layouts::Layouts::resolve(&document.presentation, [width, height])?;
        let theme = Theme::resolve(document);
        for element in &mut slides {
            if let Some(resolved) = layouts.apply(element)? {
                *element = resolved;
            } else {
                element.children =
                    automatic::elements(&element.children, [width, height], |chart, size| {
                        crate::layout::charts::automatic_frame_fits(
                            document, chart, &theme, size, measurer,
                        )
                        .map_err(|error| error.to_string())
                    })?;
            }
        }
        warnings.extend(support::warnings(document, &slides));
        let today = fields::current_date(document, &slides, &mut warnings);
        let deck = Self {
            slides: slides
                .into_iter()
                .map(|element| {
                    let background = backgrounds::Background::resolve(document, &element);
                    let metadata = fields::SlideMetadata::resolve(document, &element, &today);
                    Slide {
                        metadata,
                        element,
                        background,
                    }
                })
                .collect(),
            width,
            height,
            size_type,
            theme,
            warnings,
            time: document.time,
        };
        Ok(deck)
    }
}

impl Deck {
    /// Replaces each box of several pieces by a group of boxes stacked down
    /// its frame. The step follows the loading of the pictures, whose
    /// proportions give their heights.
    pub fn stack(
        &mut self,
        document: &Document,
        measurer: &mut crate::measurement::TextMeasurer<'_>,
        images: &crate::media::ImageCatalog,
    ) -> Result<(), String> {
        for slide in &mut self.slides {
            slide.element.children = stacks::elements(
                std::mem::take(&mut slide.element.children),
                &mut stacks::Measures {
                    text_height: |text_box: &Element| {
                        crate::layout::presentation::prepare_box(
                            document,
                            text_box,
                            &self.theme,
                            measurer,
                        )
                        .map(|prepared| (prepared.bounds[3] * 12700.0).ceil() as i64)
                        .map_err(|error| error.to_string())
                    },
                    picture_proportion: |image: &Element| {
                        let asset = images.get(image).ok()?;
                        let [width, height] = images::Crop::resolve(&image.attributes).dimensions(
                            f64::from(asset.natural_width),
                            f64::from(asset.natural_height),
                        );
                        (width > 0.0 && height > 0.0).then_some(height / width)
                    },
                },
                &mut self.warnings,
            )?;
        }
        Ok(())
    }
}

/// Returns the left, top, right and bottom, in drawing units, of the smallest
/// rectangle about the frame of an object or about the objects of a group.
pub(crate) fn extent(element: &Element) -> Option<[i64; 4]> {
    match element.kind {
        ElementKind::Group => element
            .children
            .iter()
            .filter_map(|child| match child {
                crate::model::Content::Element(child) => extent(child),
                crate::model::Content::Text(_) => None,
            })
            .reduce(|first, second| {
                [
                    first[0].min(second[0]),
                    first[1].min(second[1]),
                    first[2].max(second[2]),
                    first[3].max(second[3]),
                ]
            }),
        ElementKind::Line => {
            let endpoint = |name| {
                let fields = object(&element.attributes, name);
                ["x", "y"]
                    .map(|field| units(fields.and_then(|fields| fields.get(field))).unwrap_or(0))
            };
            let (start, end) = (endpoint("from"), endpoint("to"));
            Some([
                start[0].min(end[0]),
                start[1].min(end[1]),
                start[0].max(end[0]),
                start[1].max(end[1]),
            ])
        }
        _ => box_bounds(&element.attributes)
            .ok()
            .map(|[left, top, width, height]| [left, top, left + width, top + height]),
    }
}

pub(crate) fn box_bounds(attributes: &Properties) -> Result<[i64; 4], String> {
    let position = object(attributes, "position").ok_or("box position is missing")?;
    let coordinate = |name: &str, fallback| {
        position
            .get(name)
            .map(|value| units(Some(value)).ok_or_else(|| format!("invalid box position.{name}")))
            .transpose()
            .map(|value| value.unwrap_or(fallback))
    };
    Ok([
        coordinate("x", 0)?,
        coordinate("y", 0)?,
        coordinate("width", 3657600)?,
        coordinate("height", 1828800)?,
    ])
}

pub(crate) fn units(value: Option<&Value>) -> Option<i64> {
    crate::drawings::length_in_drawing_units(value)
}
pub(crate) fn color(value: Option<&Value>) -> Option<String> {
    crate::properties::hex_color(&value?.to_text())
}
pub(crate) fn font_name(document: &Document, name: &str) -> String {
    document
        .fonts
        .get(name)
        .map(|font| text(&font.properties, "latin"))
        .filter(|name| !name.is_empty())
        .unwrap_or_else(|| name.into())
}
pub(crate) struct Theme {
    pub name: String,
    pub colors: [String; 12],
    pub heading: String,
    pub body: String,
}
impl Theme {
    pub(crate) fn apply_chart_palette(&self, chart: &mut crate::charts::Chart) {
        let mut colors = self.colors[4..10].to_vec();
        colors.resize(colors.len().max(chart.colors.len()), String::new());
        for (index, color) in chart.colors.iter().enumerate() {
            if !color.is_empty() {
                colors[index] = color.clone();
            }
        }
        chart.colors = colors;
    }

    fn resolve(document: &Document) -> Self {
        let mut theme = Self {
            name: "Mote".into(),
            colors: [
                "111827", "FFFFFF", "1F2937", "F9FAFB", "2563EB", "059669", "D97706", "DC2626",
                "7C3AED", "0891B2", "2563EB", "7C3AED",
            ]
            .map(str::to_owned),
            heading: "Aptos Display".into(),
            body: "Aptos".into(),
        };
        let declared = |names: &[&str]| {
            names.iter().find_map(|name| {
                document
                    .fonts
                    .get(*name)
                    .map(|font| text(&font.properties, "latin"))
                    .filter(|name| !name.is_empty())
            })
        };
        theme.heading = declared(&["display", "heading"]).unwrap_or(theme.heading);
        theme.body = declared(&["body", "default"])
            .or_else(|| {
                let name = text(&document.defaults, "default_font");
                (!name.is_empty()).then(|| font_name(document, &name))
            })
            .unwrap_or(theme.body);
        if let Some(properties) = object(&document.presentation.properties, "theme") {
            let name = text(properties, "name");
            if !name.is_empty() {
                theme.name = name;
            }
            if let Some(colors) = object(properties, "colors") {
                for (index, names) in [
                    vec!["dark1", "text"],
                    vec!["light1", "background"],
                    vec!["dark2"],
                    vec!["light2"],
                    vec!["accent1"],
                    vec!["accent2"],
                    vec!["accent3"],
                    vec!["accent4"],
                    vec!["accent5"],
                    vec!["accent6"],
                    vec!["hyperlink", "hlink"],
                    vec!["followed_hyperlink", "fol_hlink"],
                ]
                .iter()
                .enumerate()
                {
                    if let Some(color) = names.iter().find_map(|name| color(colors.get(*name))) {
                        theme.colors[index] = color;
                    }
                }
            }
            if let Some(fonts) = object(properties, "fonts") {
                for (names, target) in [
                    (["heading", "major"], &mut theme.heading),
                    (["body", "minor"], &mut theme.body),
                ] {
                    if let Some(name) = names
                        .iter()
                        .map(|name| text(fonts, name))
                        .find(|name| !name.is_empty())
                    {
                        *target = font_name(document, &name);
                    }
                }
            }
        }
        theme
    }
}
