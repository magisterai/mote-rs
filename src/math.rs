//! Office Math parsing and serialization, shared by document and slide writers.
mod nodes;
mod parser;
mod symbols;
pub(crate) mod typesetting;
use crate::model::{Element, Properties};
use crate::properties;

#[derive(Clone, Default)]
pub(crate) struct Options {
    pub size_half_points: i64,
    pub color: String,
    pub presentation: bool,
}
pub(crate) struct Equation {
    pub xml: String,
    pub warnings: Vec<String>,
}
pub(crate) fn convert(
    element: &Element,
    mut options: Options,
) -> Result<Equation, crate::xml::InvalidCharacter> {
    if let Some(size) = element.attributes.get("size").and_then(half_points) {
        options.size_half_points = size;
    }
    let source = properties::text(&element.attributes, "source");
    crate::xml::escape(&source)?;
    crate::xml::escape(&options.color)?;
    let display = properties::text(&element.attributes, "display") == "block";
    if properties::text(&element.attributes, "format") == "omml" {
        return Ok(raw(&source, display, options.presentation));
    }
    let mut parser = parser::Parser::new(&source);
    let mut expression = parser.sequence(&[]);
    if parser.incomplete_delimiters {
        parser
            .warnings
            .push("incomplete math delimiters; equation emitted as source text".into());
        expression = nodes::Node::Text(source.clone(), nodes::TextKind::Normal);
    }
    let body = expression.xml(&options);
    let mut xml = format!("<m:oMath>{body}</m:oMath>");
    if display {
        xml = format!("<m:oMathPara>{xml}</m:oMathPara>");
    }
    if options.presentation {
        xml = format!("<a14:m>{xml}</a14:m>");
    }
    Ok(Equation {
        xml,
        warnings: parser.warnings,
    })
}
fn raw(source: &str, display: bool, presentation: bool) -> Equation {
    let namespaces = if presentation {
        crate::xml::PRESENTATION_ROOT_ATTRIBUTES
    } else {
        crate::xml::WORD_ROOT_ATTRIBUTES
    };
    let wrapped = format!("<root {namespaces}>{source}</root>");
    let valid = roxmltree::Document::parse(&wrapped)
        .ok()
        .is_some_and(|document| {
            let root = document.root_element();
            let elements: Vec<_> = root.children().filter(|node| node.is_element()).collect();
            elements.len() == 1
                && !root.children().any(|node| {
                    node.is_text() && !node.text().unwrap_or_default().trim().is_empty()
                })
                && {
                    let name = elements[0].tag_name();
                    name.namespace()
                        == Some("http://schemas.openxmlformats.org/officeDocument/2006/math")
                        && (name.name() == "oMath" || (display && name.name() == "oMathPara"))
                }
        });
    if valid {
        Equation {
            xml: if presentation {
                format!("<a14:m>{source}</a14:m>")
            } else {
                source.into()
            },
            warnings: Vec::new(),
        }
    } else {
        Equation {
            xml: String::new(),
            warnings: vec!["invalid OMML source emitted as visible text".into()],
        }
    }
}
pub(crate) fn normalize(attributes: &mut Properties) -> Result<(), String> {
    let source = properties::text(attributes, "source").trim().to_owned();
    if source.is_empty() {
        return Err("math requires a nonempty source".into());
    }
    attributes.insert("source".into(), crate::model::Value::String(source));
    let format = properties::text(attributes, "format").trim().to_owned();
    let format = if format.is_empty() { "latex" } else { &format };
    if !matches!(format, "latex" | "omml") {
        return Err("math format must be latex or omml".into());
    }
    attributes.insert("format".into(), crate::model::Value::String(format.into()));
    let display = properties::text(attributes, "display");
    let display = match display.trim() {
        "true" | "block" | "display" => "block",
        "" | "false" | "inline" => "inline",
        _ => return Err("math display must be true, false, block or inline".into()),
    };
    attributes.insert(
        "display".into(),
        crate::model::Value::String(display.into()),
    );
    Ok(())
}

fn half_points(value: &crate::model::Value) -> Option<i64> {
    use crate::model::{Unit, Value};
    let half_points = match value {
        Value::Length {
            value,
            unit: Unit::Point,
        } => value * 2.0,
        Value::Length { value, unit } => {
            let twips = value
                * match unit {
                    Unit::Point => 20.0,
                    Unit::Inch => 1440.0,
                    Unit::Centimetre => 1440.0 / 2.54,
                    Unit::Millimetre => 1440.0 / 25.4,
                    Unit::Twip => 1.0,
                };
            twips.round() / 10.0
        }
        _ => value.to_text().parse::<f64>().ok()? * 2.0,
    };
    half_points
        .is_finite()
        .then_some(half_points.round() as i64)
}
