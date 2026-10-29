use super::{Error, check_attributes, values, xml};
use crate::model::{Properties, Value};

pub(super) const ATTRIBUTES: &[&str] = &[
    "alignment",
    "spacing",
    "spacing_before",
    "spacing_after",
    "line_spacing",
    "indent",
    "keep_with_next",
    "keep_together",
    "page_break_before",
    "widow_control",
    "outline_level",
    "border",
    "fill",
    "tabs",
];

pub(super) fn write(properties: &Properties) -> Result<String, Error> {
    write_properties(properties, false)
}

pub(super) fn write_declaration(properties: &Properties) -> Result<String, Error> {
    write_properties(properties, true)
}

fn write_properties(properties: &Properties, declaration: bool) -> Result<String, Error> {
    let mut supported = ATTRIBUTES.to_vec();
    supported.push("text_direction");
    if declaration {
        supported.extend_from_slice(&["background", "first_line_indent", "hanging_indent"]);
    }
    check_attributes(properties, &supported, "paragraph")?;
    let mut result = if declaration {
        String::new()
    } else {
        pagination(properties)?
    };
    if declaration {
        for name in ["fill", "background"] {
            if let Some(fill) = properties.get(name) {
                result.push_str(&shading(fill)?);
            }
        }
    }
    if let Some(border) = properties.get("border") {
        result.push_str(&borders(border)?);
    }
    if !declaration && let Some(fill) = properties.get("fill") {
        result.push_str(&shading(fill)?);
    }
    if let Some(stops) = properties.get("tabs") {
        result.push_str(&tabs(stops)?);
    }
    if properties
        .get("text_direction")
        .is_some_and(|value| value.to_text() == "rtl")
    {
        result.push_str("<w:bidi/>");
    }
    result.push_str(&spacing(properties)?);
    let mut indentation = properties
        .get("indent")
        .map(values::object)
        .transpose()?
        .cloned()
        .unwrap_or_default();
    for (name, target) in [
        ("first_line_indent", "first_line"),
        ("hanging_indent", "hanging"),
    ] {
        if let Some(value) = properties.get(name) {
            if indentation.contains_key(target) {
                return Err(Error::InvalidValue(format!(
                    "duplicate Word indentation property {target}"
                )));
            }
            indentation.insert(target.into(), value.clone());
        }
    }
    if properties.contains_key("indent") || !indentation.is_empty() {
        check_attributes(
            &indentation,
            &["left", "right", "first_line", "hanging"],
            "indent",
        )?;
        result.push_str("<w:ind");
        for (name, word_name) in [
            ("left", "left"),
            ("right", "right"),
            ("first_line", "firstLine"),
            ("hanging", "hanging"),
        ] {
            if let Some(value) = indentation.get(name) {
                result.push_str(&format!(" w:{word_name}=\"{}\"", values::twips(value)?));
            }
        }
        result.push_str("/>");
    }
    if let Some(alignment) = properties.get("alignment") {
        let alignment = alignment.to_text();
        let word_alignment = match alignment.as_str() {
            "left" | "center" | "right" => &alignment,
            "justify" => "both",
            _ => {
                return Err(Error::InvalidValue(format!(
                    "invalid paragraph alignment {alignment:?}"
                )));
            }
        };
        result.push_str(&format!("<w:jc w:val=\"{word_alignment}\"/>"));
    }
    if declaration {
        result.push_str(&pagination(properties)?);
    }
    if let Some(level) = properties.get("outline_level") {
        let level = values::number(level)?;
        if level.fract() != 0.0 || !(0.0..=9.0).contains(&level) {
            return Err(Error::InvalidValue(
                "Word outline levels must be integers from 0 through 9".into(),
            ));
        }
        result.push_str(&format!("<w:outlineLvl w:val=\"{level}\"/>"));
    }
    Ok(result)
}

pub(super) fn pagination(properties: &Properties) -> Result<String, Error> {
    let mut result = String::new();
    for (name, word_name) in [
        ("keep_with_next", "keepNext"),
        ("keep_together", "keepLines"),
        ("page_break_before", "pageBreakBefore"),
        ("widow_control", "widowControl"),
    ] {
        if let Some(value) = properties.get(name) {
            result.push_str(&if values::boolean(value)? {
                format!("<w:{word_name}/>")
            } else {
                format!("<w:{word_name} w:val=\"0\"/>")
            });
        }
    }
    Ok(result)
}

fn spacing(properties: &Properties) -> Result<String, Error> {
    let empty = Properties::new();
    let spacing = properties
        .get("spacing")
        .map(values::object)
        .transpose()?
        .unwrap_or(&empty);
    check_attributes(
        spacing,
        &[
            "before",
            "after",
            "line",
            "spacing_before",
            "spacing_after",
            "line_spacing",
        ],
        "spacing",
    )?;
    let mut attributes = String::new();
    for (name, alias) in [("before", "spacing_before"), ("after", "spacing_after")] {
        if let Some(value) =
            values::first(spacing, &[name, alias]).or_else(|| properties.get(alias))
        {
            attributes.push_str(&format!(" w:{name}=\"{}\"", values::twips(value)?));
        }
    }
    if let Some(value) =
        values::first(spacing, &["line", "line_spacing"]).or_else(|| properties.get("line_spacing"))
    {
        let (line, rule) = if let Value::Object(line) = value {
            check_attributes(line, &["rule", "value"], "line spacing")?;
            let rule = line.get("rule").map(Value::to_text).unwrap_or_default();
            let value = line
                .get("value")
                .ok_or_else(|| Error::InvalidValue("line spacing requires a value".into()))?;
            match rule.as_str() {
                "multiple" => {
                    let multiple = values::number(value)?;
                    if multiple <= 0.0 {
                        return Err(Error::InvalidValue(
                            "line spacing multiple must be positive".into(),
                        ));
                    }
                    (values::rounded(multiple * 240.0)?, "auto")
                }
                "exact" => (values::twips(value)?, "exact"),
                "at_least" => (values::twips(value)?, "atLeast"),
                _ => {
                    return Err(Error::InvalidValue(format!(
                        "invalid line spacing rule {rule:?}"
                    )));
                }
            }
        } else {
            (values::twips(value)?, "exact")
        };
        attributes.push_str(&format!(" w:line=\"{line}\" w:lineRule=\"{rule}\""));
    }
    Ok(if attributes.is_empty() {
        String::new()
    } else {
        format!("<w:spacing{attributes}/>")
    })
}

fn borders(value: &Value) -> Result<String, Error> {
    let properties = values::object(value)?;
    check_attributes(
        properties,
        &["top", "left", "bottom", "right", "all"],
        "border",
    )?;
    let mut result = String::new();
    for side in ["top", "left", "bottom", "right"] {
        let Some(value) = values::first(properties, &[side, "all"]) else {
            continue;
        };
        let empty = Properties::new();
        let (properties, style) = if let Value::Object(properties) = value {
            check_attributes(
                properties,
                &["style", "val", "size", "width", "space", "color"],
                "border side",
            )?;
            (
                properties,
                values::first_text(properties, &["style", "val"])
                    .unwrap_or_else(|| "single".into()),
            )
        } else {
            (&empty, value.to_text())
        };
        if style.is_empty() {
            continue;
        }
        if matches!(style.as_str(), "none" | "nil") {
            result.push_str(&format!("<w:{side} w:val=\"nil\"/>"));
            continue;
        }
        let mut size = properties
            .get("size")
            .map(values::number)
            .transpose()?
            .unwrap_or(4.0);
        if let Some(width) = properties.get("width") {
            size = if let Value::Number(number) = width {
                number * 8.0
            } else {
                values::twips(width)? as f64 * 2.0 / 5.0
            };
        }
        let size = values::rounded(size)?;
        let space = values::rounded(
            properties
                .get("space")
                .map(values::number)
                .transpose()?
                .unwrap_or(0.0),
        )?;
        let color = properties
            .get("color")
            .filter(|value| !value.to_text().trim().trim_start_matches('#').is_empty())
            .map(values::color)
            .transpose()?
            .unwrap_or_else(|| "BFBFBF".into());
        result.push_str(&format!(
            "<w:{side} w:val=\"{}\" w:sz=\"{size}\" w:space=\"{space}\" w:color=\"{color}\"/>",
            xml::escape(&style)?
        ));
    }
    Ok(if result.is_empty() {
        result
    } else {
        format!("<w:pBdr>{result}</w:pBdr>")
    })
}

fn tabs(value: &Value) -> Result<String, Error> {
    let mut stops = Vec::new();
    match value {
        Value::Object(properties) if properties.len() == 1 => {
            let (alignment, position) = properties.first_key_value().unwrap();
            stops.push(Properties::from([
                ("alignment".into(), Value::String(alignment.clone())),
                ("position".into(), position.clone()),
            ]));
        }
        Value::Object(properties) => stops.push(properties.clone()),
        Value::Array(values) => {
            for value in values {
                stops.push(values::object(value)?.clone());
            }
        }
        _ => {
            return Err(Error::InvalidValue(
                "tab stops require an object or array".into(),
            ));
        }
    }
    let mut result = String::new();
    for stop in stops {
        check_attributes(
            &stop,
            &[
                "position",
                "pos",
                "alignment",
                "align",
                "leader",
                "right",
                "left",
                "center",
            ],
            "tab stop",
        )?;
        let position = values::first(&stop, &["position", "pos", "right", "left", "center"])
            .ok_or_else(|| Error::InvalidValue("tab stop requires a position".into()))?;
        let alignment =
            values::first_text(&stop, &["alignment", "align"]).unwrap_or_else(|| "left".into());
        result.push_str(&format!(
            "<w:tab w:val=\"{}\" w:pos=\"{}\"",
            xml::escape(&alignment)?,
            values::twips(position)?
        ));
        if let Some(leader) = values::first_text(&stop, &["leader"]) {
            result.push_str(&format!(" w:leader=\"{}\"", xml::escape(&leader)?));
        }
        result.push_str("/>");
    }
    Ok(if result.is_empty() {
        result
    } else {
        format!("<w:tabs>{result}</w:tabs>")
    })
}

fn shading(color: &Value) -> Result<String, Error> {
    Ok(format!(
        "<w:shd w:val=\"clear\" w:color=\"auto\" w:fill=\"{}\"/>",
        values::color(color)?
    ))
}
