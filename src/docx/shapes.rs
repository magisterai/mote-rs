//! Absolute Word shapes with editable text and DrawingML visual properties.
use super::{
    Error, document::DocumentState, images, paragraphs, run_properties, styles::Styles, values, xml,
};
use crate::{
    drawings,
    model::{Content, Element, ElementKind, Properties},
    properties::{self, object, text},
};

const SHAPE_URI: &str = "http://schemas.microsoft.com/office/word/2010/wordprocessingShape";
const GROUP_URI: &str = "http://schemas.microsoft.com/office/word/2010/wordprocessingGroup";

/// Writes a shape or a group as one drawing in a paragraph of its own. A
/// drawing that states a place is anchored there on the page, in a paragraph
/// collapsed to a negligible height; a drawing that states none stands inline
/// in a paragraph that its height sets.
pub(super) fn write(
    element: &Element,
    styles: &Styles<'_>,
    state: &mut DocumentState,
    leading_column_break: bool,
) -> Result<String, Error> {
    let attributes = &element.attributes;
    let identifier = state.resources.next_drawing();
    let mut name = text(attributes, "name");
    if name.is_empty() {
        name = format!(
            "{} {identifier}",
            if element.kind == ElementKind::Group {
                "Group"
            } else {
                "Shape"
            }
        );
    }
    let name = xml::escape(&name)?;
    let (bookmarks, bookmark_identifiers) = super::references::start_pending(&mut state.resources)?;
    let ([horizontal, vertical], [width, height], uri, graphic) =
        if element.kind == ElementKind::Group {
            let size = drawings::group_size(element);
            (
                drawings::group_corner(element),
                size,
                GROUP_URI,
                format!(
                    "<wpg:wgp><wpg:cNvGrpSpPr/>{}{}</wpg:wgp>",
                    group_properties(element, [0, 0], size)?,
                    members(element, styles, state)?
                ),
            )
        } else {
            let position = object(attributes, "position").cloned().unwrap_or_default();
            let coordinate = |name: &str, fallback| length(&position, name, fallback);
            let size = [
                coordinate("width", 3657600)?,
                coordinate("height", 1828800)?,
            ];
            (
                [coordinate("x", 0)?, coordinate("y", 0)?],
                size,
                SHAPE_URI,
                format!(
                    "<wps:wsp><wps:cNvSpPr/>{}</wps:wsp>",
                    shape(element, [0, 0, size[0], size[1]], styles, state)?
                ),
            )
        };
    let bookmark_ends = super::references::end_pending(&bookmark_identifiers);
    let described = format!(
        r#"<wp:extent cx="{width}" cy="{height}"/><wp:effectExtent l="0" t="0" r="0" b="0"/>"#
    );
    // The document root declares the shape namespace and leaves the group
    // namespace to the one drawing that needs it.
    let declaration = if element.kind == ElementKind::Group {
        format!(" xmlns:wpg=\"{GROUP_URI}\" mc:Ignorable=\"wpg\"")
    } else {
        String::new()
    };
    let graphic = format!(
        r#"<wp:docPr id="{identifier}" name="{name}"/><wp:cNvGraphicFramePr/><a:graphic><a:graphicData{declaration} uri="{uri}">{graphic}</a:graphicData></a:graphic>"#
    );
    if drawings::flows(element) {
        let mut paragraph = Properties::new();
        // The alignment of a shape is that of its text.
        let alignment = text(attributes, "alignment");
        if element.kind == ElementKind::Group && !alignment.is_empty() {
            paragraph.insert("alignment".into(), crate::model::Value::String(alignment));
        }
        super::paragraph_properties::fit_drawing(&mut paragraph);
        let paragraph = super::paragraph_properties::write(&paragraph)?;
        let column_break = if leading_column_break {
            "<w:r><w:br w:type=\"column\"/></w:r>"
        } else {
            ""
        };
        return Ok(format!(
            r#"<w:p><w:pPr>{paragraph}</w:pPr>{bookmarks}{column_break}<w:r><w:drawing><wp:inline distT="0" distB="0" distL="0" distR="0">{described}{graphic}</wp:inline></w:drawing></w:r>{bookmark_ends}</w:p>"#
        ));
    }
    Ok(format!(
        r#"<w:p><w:pPr><w:spacing w:before="0" w:after="0" w:line="20" w:lineRule="exact"/><w:rPr><w:sz w:val="2"/><w:szCs w:val="2"/></w:rPr></w:pPr>{bookmarks}<w:r><w:drawing><wp:anchor distT="0" distB="0" distL="0" distR="0" simplePos="0" relativeHeight="251658240" behindDoc="0" locked="0" layoutInCell="1" allowOverlap="1"><wp:simplePos x="0" y="0"/><wp:positionH relativeFrom="page"><wp:posOffset>{horizontal}</wp:posOffset></wp:positionH><wp:positionV relativeFrom="page"><wp:posOffset>{vertical}</wp:posOffset></wp:positionV>{described}<wp:wrapNone/>{graphic}</wp:anchor></w:drawing></w:r>{bookmark_ends}</w:p>"#
    ))
}

/// Writes the properties, the text and the body properties of a shape whose
/// frame has the stated bounds within its container.
fn shape(
    element: &Element,
    bounds: [i64; 4],
    styles: &Styles<'_>,
    state: &mut DocumentState,
) -> Result<String, Error> {
    Ok(format!(
        "<wps:spPr>{}</wps:spPr>{}{}",
        crate::drawings::drawingml::shape(&element.attributes, bounds)
            .map_err(Error::InvalidValue)?,
        text_box(element, styles, state)?,
        body_properties(&element.attributes)?
    ))
}

/// Writes the frame of a group at a corner within its container. The child
/// frame begins at zero and has the size of the frame, so that the objects of
/// the group are measured from its corner at their own scale.
fn group_properties(element: &Element, corner: [i64; 2], size: [i64; 2]) -> Result<String, Error> {
    let rotation = match properties::number(&element.attributes, "rotation") {
        Some(degrees) if degrees != 0.0 => format!(
            " rot=\"{}\"",
            (degrees.rem_euclid(360.0) * 60000.0).round() as i64
        ),
        _ => String::new(),
    };
    let [width, height] = size;
    Ok(format!(
        "<wpg:grpSpPr><a:xfrm{rotation}><a:off x=\"{}\" y=\"{}\"/><a:ext cx=\"{width}\" cy=\"{height}\"/><a:chOff x=\"0\" y=\"0\"/><a:chExt cx=\"{width}\" cy=\"{height}\"/></a:xfrm></wpg:grpSpPr>",
        corner[0], corner[1]
    ))
}

/// Writes the objects of a group in their order: shapes, lines and groups.
fn members(
    group: &Element,
    styles: &Styles<'_>,
    state: &mut DocumentState,
) -> Result<String, Error> {
    let mut output = String::new();
    for child in &group.children {
        let Content::Element(child) = child else {
            continue;
        };
        let identifier = state.resources.next_drawing();
        let stated = text(&child.attributes, "name");
        let named = |kind: &str| {
            xml::escape(&if stated.is_empty() {
                format!("{kind} {identifier}")
            } else {
                stated.clone()
            })
        };
        match child.kind {
            ElementKind::Shape => {
                let position = object(&child.attributes, "position")
                    .cloned()
                    .unwrap_or_default();
                let coordinate = |name: &str, fallback| length(&position, name, fallback);
                let bounds = [
                    coordinate("x", 0)?,
                    coordinate("y", 0)?,
                    coordinate("width", 3657600)?,
                    coordinate("height", 1828800)?,
                ];
                output.push_str(&format!(
                    "<wps:wsp><wps:cNvPr id=\"{identifier}\" name=\"{}\"/><wps:cNvSpPr/>{}</wps:wsp>",
                    named("Shape")?,
                    shape(child, bounds, styles, state)?
                ));
            }
            ElementKind::Line => output.push_str(&format!(
                "<wps:wsp><wps:cNvPr id=\"{identifier}\" name=\"{}\"/><wps:cNvCnPr/><wps:spPr>{}</wps:spPr><wps:bodyPr/></wps:wsp>",
                named("Line")?,
                crate::drawings::drawingml::connector(&child.attributes)
                    .map_err(Error::InvalidValue)?
            )),
            ElementKind::Group => output.push_str(&format!(
                "<wpg:grpSp><wpg:cNvPr id=\"{identifier}\" name=\"{}\"/><wpg:cNvGrpSpPr/>{}{}</wpg:grpSp>",
                named("Group")?,
                group_properties(
                    child,
                    drawings::group_corner(child),
                    drawings::group_size(child)
                )?,
                members(child, styles, state)?
            )),
            _ => state
                .resources
                .warnings
                .push("DOCX omits a box in a group; a document sets no box".into()),
        }
    }
    Ok(output)
}

fn length(properties: &Properties, name: &str, fallback: i64) -> Result<i64, Error> {
    properties
        .get(name)
        .map(drawing_units)
        .transpose()
        .map(|value| value.unwrap_or(fallback))
}

fn drawing_units(value: &crate::model::Value) -> Result<i64, Error> {
    if matches!(value, crate::model::Value::Length { .. }) {
        images::length_in_drawing_units(value)
    } else {
        values::rounded(values::number(value)?)
    }
}

fn text_box(
    element: &Element,
    styles: &Styles<'_>,
    state: &mut DocumentState,
) -> Result<String, Error> {
    let mut children: Vec<_> = element
        .children
        .iter()
        .filter(|child| !matches!(child,Content::Text(text) if text.trim().is_empty()))
        .cloned()
        .collect();
    if children.is_empty() {
        return Ok(String::new());
    }
    let text = drawings::text_properties(&element.attributes);
    // Native layout sets the text of a shape at single spacing unless a style
    // states a line spacing, whereas Word would otherwise apply the line
    // spacing of the document defaults.
    let normal = crate::formatting::StyleResolver::new(styles.document)
        .resolve_style("Normal")
        .paragraph;
    let single = !normal.contains_key("line_spacing")
        && !object(&normal, "spacing").is_some_and(|spacing| {
            spacing.contains_key("line") || spacing.contains_key("line_spacing")
        });
    let mut paragraph = drawings::paragraph_properties(&text);
    if single && !paragraph.contains_key("style") {
        super::paragraph_properties::fit_drawing(&mut paragraph);
    }
    let previous = state.resources.active_text.clone();
    state.resources.active_text.properties.extend(
        text.into_iter()
            .filter(|(name, _)| run_properties::ATTRIBUTES.contains(&name.as_str())),
    );
    let inline = children
        .iter()
        .all(|child| !matches!(child,Content::Element(element) if element.is_block()));
    let content = if inline {
        paragraphs::write(
            &paragraph,
            &children,
            styles,
            None,
            false,
            &mut state.resources,
        )
    } else {
        for child in &mut children {
            if let Content::Element(element) = child
                && matches!(
                    element.kind,
                    ElementKind::Paragraph | ElementKind::Heading | ElementKind::Blockquote
                )
            {
                for (name, value) in &paragraph {
                    element
                        .attributes
                        .entry(name.clone())
                        .or_insert_with(|| value.clone());
                }
                if single
                    && !element.attributes.contains_key("style")
                    && element.kind != ElementKind::Heading
                {
                    super::paragraph_properties::fit_drawing(&mut element.attributes);
                }
            }
        }
        super::sections::write_story_blocks(&children, styles, state)
    };
    state.resources.active_text = previous;
    Ok(format!(
        "<wps:txbx><w:txbxContent>{}</w:txbxContent></wps:txbx>",
        content?
    ))
}

fn body_properties(attributes: &Properties) -> Result<String, Error> {
    let text = drawings::text_properties(attributes);
    let anchor = match properties::text(&text, "vertical_alignment").as_str() {
        "center" | "middle" => "ctr",
        "bottom" => "b",
        _ => "t",
    };
    let margin = object(&text, "margin").cloned().unwrap_or_default();
    let mut output = format!("<wps:bodyPr wrap=\"square\" anchor=\"{anchor}\"");
    for (name, field) in [
        ("left", "lIns"),
        ("right", "rIns"),
        ("top", "tIns"),
        ("bottom", "bIns"),
    ] {
        output.push_str(&format!(" {field}=\"{}\"", length(&margin, name, 0)?));
    }
    match properties::text(&text, "fit").as_str() {
        "clip" => output.push_str("><a:noAutofit/></wps:bodyPr>"),
        "shrink" => output.push_str("><a:normAutofit/></wps:bodyPr>"),
        "resize_shape" => output.push_str("><a:spAutoFit/></wps:bodyPr>"),
        _ => output.push_str("/>"),
    }
    Ok(output)
}
