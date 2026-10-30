//! Resolved deck geometry and defaults shared by editable and native slides.
pub(crate) mod backgrounds;
pub(crate) mod fields;
pub(crate) mod images;
mod support;
pub(crate) mod tables;
pub(crate) mod text;
use crate::{
    model::{Content, Document, Element, ElementKind, Properties, Value},
    properties::{object, text},
};

pub(crate) struct Deck {
    pub slides: Vec<Slide>,
    pub width: i64,
    pub height: i64,
    pub size_type: &'static str,
    pub theme: Theme,
    pub warnings: Vec<String>,
}
pub(crate) struct Slide {
    pub element: Element,
    pub background: backgrounds::Background,
    pub metadata: fields::SlideMetadata,
}
impl Deck {
    pub fn resolve(document: &Document) -> Result<Self, String> {
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
        let mut warnings: Vec<_> = support::warnings(document).into_iter().collect();
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
            ]
            .contains(&name.as_str())
            {
                warnings.push(format!(
                    "presentation.{name} is retained without an output effect"
                ));
            }
        }
        let mut slides = Vec::new();
        for node in &document.content {
            match node {
                Content::Text(value) if value.trim().is_empty() => {}
                Content::Element(element) if element.kind == ElementKind::Slide => {
                    slides.push(element.clone());
                }
                _ => {
                    return Err(
                        "presentation output currently requires explicit slide bodies".into(),
                    );
                }
            }
        }
        if slides.is_empty() {
            slides.push(Element {
                kind: ElementKind::Slide,
                attributes: Properties::new(),
                children: Vec::new(),
            });
        }
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
            theme: Theme::resolve(document),
            warnings,
        };
        Ok(deck)
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
    let value = value?.to_text();
    let value = value.trim().trim_start_matches('#');
    (value.len() == 6 && value.bytes().all(|byte| byte.is_ascii_hexdigit()))
        .then(|| value.to_ascii_uppercase())
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
