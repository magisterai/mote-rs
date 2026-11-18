use super::{Error, check_attributes, diagnostics, values};
use crate::{
    model::{Properties, Value},
    xml::OrderedChildren,
};

/// The children of `w:pPr` in the sequence of CT_PPrBase and CT_PPr.
const SEQUENCE: &[&str] = &[
    "w:pStyle",
    "w:keepNext",
    "w:keepLines",
    "w:pageBreakBefore",
    "w:framePr",
    "w:widowControl",
    "w:numPr",
    "w:suppressLineNumbers",
    "w:pBdr",
    "w:shd",
    "w:tabs",
    "w:suppressAutoHyphens",
    "w:kinsoku",
    "w:wordWrap",
    "w:overflowPunct",
    "w:topLinePunct",
    "w:autoSpaceDE",
    "w:autoSpaceDN",
    "w:bidi",
    "w:adjustRightInd",
    "w:snapToGrid",
    "w:spacing",
    "w:ind",
    "w:contextualSpacing",
    "w:mirrorIndents",
    "w:suppressOverlap",
    "w:jc",
    "w:textDirection",
    "w:textAlignment",
    "w:textboxTightWrap",
    "w:outlineLvl",
    "w:divId",
    "w:cnfStyle",
    "w:rPr",
    "w:sectPr",
    "w:pPrChange",
];

/// The children of one `w:pPr`, to which any writer may add.
pub(super) fn children() -> OrderedChildren {
    OrderedChildren::new(SEQUENCE)
}

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
    let mut children = children();
    collect(properties, &mut children)?;
    Ok(children.write())
}

pub(super) fn write_declaration(properties: &Properties) -> Result<String, Error> {
    let mut children = children();
    collect_properties(properties, true, &mut children)?;
    Ok(children.write())
}

/// Adds the direct properties of a paragraph to the children of its `w:pPr`.
pub(super) fn collect(
    properties: &Properties,
    children: &mut OrderedChildren,
) -> Result<(), Error> {
    collect_properties(properties, false, children)
}

fn collect_properties(
    properties: &Properties,
    declaration: bool,
    children: &mut OrderedChildren,
) -> Result<(), Error> {
    let mut supported = ATTRIBUTES.to_vec();
    supported.push("text_direction");
    if declaration {
        supported.extend_from_slice(&["background", "first_line_indent", "hanging_indent"]);
    }
    // The properties of a declaration are those of its paragraph group, and
    // direct properties are the attributes of the element being written.
    let path = if declaration {
        "paragraph"
    } else {
        diagnostics::construct("paragraph")
    };
    check_attributes(properties, &supported, path);
    pagination(properties, children)?;
    if let Some(border) = properties.get("border") {
        children.push(borders(border)?);
    }
    let fills: &[&str] = if declaration {
        &["fill", "background"]
    } else {
        &["fill"]
    };
    // The schema admits one `w:shd`, so the first fill that is stated is written.
    if let Some(fill) = fills.iter().find_map(|name| properties.get(*name)) {
        children.push(shading(fill)?);
    }
    if let Some(stops) = properties.get("tabs") {
        children.push(tabs(stops)?);
    }
    if properties
        .get("text_direction")
        .is_some_and(|value| value.to_text() == "rtl")
    {
        children.push("<w:bidi/>");
    }
    children.push(spacing(properties)?);
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
        );
        let mut indent = String::from("<w:ind");
        for (name, word_name) in [
            ("left", "left"),
            ("right", "right"),
            ("first_line", "firstLine"),
            ("hanging", "hanging"),
        ] {
            if let Some(value) = indentation.get(name) {
                indent.push_str(&format!(" w:{word_name}=\"{}\"", values::twips(value)?));
            }
        }
        indent.push_str("/>");
        children.push(indent);
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
        children.push(format!("<w:jc w:val=\"{word_alignment}\"/>"));
    }
    if let Some(level) = properties.get("outline_level") {
        let level = values::number(level)?;
        if level.fract() != 0.0 || !(0.0..=9.0).contains(&level) {
            return Err(Error::InvalidValue(
                "Word outline levels must be integers from 0 through 9".into(),
            ));
        }
        children.push(format!("<w:outlineLvl w:val=\"{level}\"/>"));
    }
    Ok(())
}

/// Word clips an inline drawing to an exact line height inherited from the
/// paragraph style, whereas native layout grows the line to the drawing. A
/// paragraph that holds a block drawing therefore uses single automatic spacing
/// unless the paragraph states its own line spacing.
pub(super) fn fit_drawing(properties: &mut Properties) {
    if properties.contains_key("line_spacing") {
        return;
    }
    if let Value::Object(spacing) = properties
        .entry("spacing".into())
        .or_insert_with(|| Value::Object(Properties::new()))
        && !spacing.contains_key("line")
        && !spacing.contains_key("line_spacing")
    {
        spacing.insert(
            "line".into(),
            Value::Object(Properties::from([
                ("rule".into(), Value::String("multiple".into())),
                ("value".into(), Value::Number(1.0)),
            ])),
        );
    }
}

fn pagination(properties: &Properties, children: &mut OrderedChildren) -> Result<(), Error> {
    for (name, word_name) in [
        ("keep_with_next", "keepNext"),
        ("keep_together", "keepLines"),
        ("page_break_before", "pageBreakBefore"),
        ("widow_control", "widowControl"),
    ] {
        if let Some(value) = properties.get(name) {
            children.push(if values::boolean(value)? {
                format!("<w:{word_name}/>")
            } else {
                format!("<w:{word_name} w:val=\"0\"/>")
            });
        }
    }
    Ok(())
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
    );
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
            check_attributes(line, &["rule", "value"], "spacing.line");
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
        &["top", "left", "bottom", "right", "all", "width", "color"],
        "border",
    );
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
                &format!("border.{side}"),
            );
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
        let space = values::rounded(match properties.get("space") {
            Some(Value::Number(points)) => *points,
            Some(space) => values::twips(space)? as f64 / 20.0,
            None => 0.0,
        })?;
        let color = properties
            .get("color")
            .filter(|value| !value.to_text().trim().trim_start_matches('#').is_empty())
            .map(values::color)
            .transpose()?
            .unwrap_or_else(|| "BFBFBF".into());
        result.push_str(&format!(
            "<w:{side} w:val=\"{}\" w:sz=\"{size}\" w:space=\"{space}\" w:color=\"{color}\"/>",
            values::border_style(&style, &format!("border.{side}"))
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
            "tabs",
        );
        let position = values::first(&stop, &["position", "pos", "right", "left", "center"])
            .ok_or_else(|| Error::InvalidValue("tab stop requires a position".into()))?;
        let alignment =
            values::first_text(&stop, &["alignment", "align"]).unwrap_or_else(|| "left".into());
        result.push_str(&format!(
            "<w:tab w:val=\"{}\" w:pos=\"{}\"",
            values::tab_alignment(&alignment),
            values::twips(position)?
        ));
        if let Some(leader) =
            values::first_text(&stop, &["leader"]).and_then(|leader| values::tab_leader(&leader))
        {
            result.push_str(&format!(" w:leader=\"{leader}\""));
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
