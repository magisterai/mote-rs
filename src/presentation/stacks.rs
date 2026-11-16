//! A box of several pieces becomes a group of boxes stacked down its frame.
//!
//! A text frame of a slide holds text alone, and a table, a picture or a chart
//! is an object of its own. A piece is therefore either a run of text flow or a
//! single table, picture or chart.
use crate::{
    model::{Content, Element, ElementKind, Properties, Value},
    properties::{object, text},
};

const PIECE_GAP: i64 = 91440;
const LEAST_CHART_HEIGHT: i64 = 914400;
const FRAME_ATTRIBUTES: [&str; 4] = ["fill", "border", "opacity", "shadow"];

/// The measurements a stack requires of its pieces.
pub(super) struct Measures<TextHeight, PictureProportion> {
    /// The height in drawing units that the text of a box requires at the
    /// width of the box.
    pub text_height: TextHeight,
    /// The height of a picture divided by its width, where the picture loads.
    pub picture_proportion: PictureProportion,
}

/// Replaces each box of several pieces, at any depth of grouping, by its group.
pub(super) fn elements<TextHeight, PictureProportion>(
    content: Vec<Content>,
    measures: &mut Measures<TextHeight, PictureProportion>,
    warnings: &mut Vec<String>,
) -> Result<Vec<Content>, String>
where
    TextHeight: FnMut(&Element) -> Result<i64, String>,
    PictureProportion: FnMut(&Element) -> Option<f64>,
{
    content
        .into_iter()
        .map(|node| match node {
            Content::Element(mut element) if element.kind == ElementKind::Group => {
                element.children =
                    elements(std::mem::take(&mut element.children), measures, warnings)?;
                Ok(Content::Element(element))
            }
            Content::Element(element)
                if element.kind == ElementKind::Box && pieces(&element.children).len() > 1 =>
            {
                stack(&element, measures, warnings).map(Content::Element)
            }
            node => Ok(node),
        })
        .collect()
}

fn stands_alone(node: &Content) -> bool {
    matches!(node, Content::Element(element) if matches!(element.kind, ElementKind::Image | ElementKind::Table | ElementKind::Chart))
}

fn pieces(children: &[Content]) -> Vec<Vec<Content>> {
    let mut output = Vec::new();
    let mut flow = Vec::new();
    for child in children {
        if stands_alone(child) {
            if !flow.is_empty() {
                output.push(std::mem::take(&mut flow));
            }
            output.push(vec![child.clone()]);
        } else {
            flow.push(child.clone());
        }
    }
    if !flow.is_empty() {
        output.push(flow);
    }
    output
}

fn positioned(source: &Element, attributes: Properties, children: Vec<Content>) -> Element {
    Element {
        source_name: source.source_name.clone(),
        source_location: source.source_location.clone(),
        kind: ElementKind::Box,
        attributes,
        children,
    }
}

fn position(bounds: [i64; 4]) -> Value {
    Value::Object(
        ["x", "y", "width", "height"]
            .into_iter()
            .zip(bounds)
            .map(|(name, value)| (name.into(), Value::Number(value as f64)))
            .collect(),
    )
}

/// The height a piece takes of the frame.
enum Height {
    /// A run of text takes the height of its lines.
    Text(i64),
    /// A picture takes the height its proportion gives at the width of the
    /// frame, and no more than an equal share of the space the text leaves.
    Picture(i64),
    /// A table or a chart takes an equal share of the space left over.
    Share,
}

fn stack<TextHeight, PictureProportion>(
    element: &Element,
    measures: &mut Measures<TextHeight, PictureProportion>,
    warnings: &mut Vec<String>,
) -> Result<Element, String>
where
    TextHeight: FnMut(&Element) -> Result<i64, String>,
    PictureProportion: FnMut(&Element) -> Option<f64>,
{
    let [left, top, width, height] = super::box_bounds(&element.attributes)?;
    let name = text(&element.attributes, "name");
    let described = if name.is_empty() {
        "a box of several pieces".to_owned()
    } else {
        format!("box {name:?}")
    };
    let mut text_attributes = element.attributes.clone();
    for name in ["name", "slot", "fit", "animate"]
        .into_iter()
        .chain(FRAME_ATTRIBUTES)
    {
        text_attributes.remove(name);
    }
    // The height of a text piece is the height of its text, so that neither
    // shrinking nor growth has anything to act upon.
    if let Some(Value::Object(fields)) = text_attributes.get_mut("text")
        && fields.remove("fit").is_some()
    {
        warnings.push(format!("{described} ignores text.fit"));
    }
    let margin = |side| {
        object(
            &crate::drawings::text_attributes(&text_attributes),
            "margin",
        )
        .and_then(|margin| super::units(margin.get(side)))
        .unwrap_or(0)
    };
    let margins = margin("top") + margin("bottom");

    let probe = |children: Vec<Content>| {
        let mut attributes = text_attributes.clone();
        attributes.insert("position".into(), position([left, top, width, 12700]));
        let mut fields = object(&attributes, "text").cloned().unwrap_or_default();
        fields.insert("fit".into(), Value::String("resize_shape".into()));
        attributes.insert("text".into(), Value::Object(fields));
        positioned(element, attributes, children)
    };
    let pieces = pieces(&element.children);
    let mut heights = Vec::new();
    for piece in &pieces {
        heights.push(match &piece[0] {
            Content::Element(image) if image.kind == ElementKind::Image => {
                match (measures.picture_proportion)(image) {
                    Some(proportion) => Height::Picture((width as f64 * proportion).round() as i64),
                    None => Height::Share,
                }
            }
            node if stands_alone(node) => Height::Share,
            _ => {
                let measured = (measures.text_height)(&probe(piece.clone()))?;
                Height::Text(measured + margins)
            }
        });
    }
    let unfixed = heights
        .iter()
        .filter(|height| !matches!(height, Height::Text(_)))
        .count() as i64;
    let mut remaining = height
        - PIECE_GAP * (pieces.len() as i64 - 1)
        - heights
            .iter()
            .map(|height| match height {
                Height::Text(height) => *height,
                _ => 0,
            })
            .sum::<i64>();
    let limit = remaining.max(0) / unfixed.max(1);
    for height in &mut heights {
        if let Height::Picture(natural) = height {
            *natural = (*natural).min(limit);
            remaining -= *natural;
        }
    }
    let shares = heights
        .iter()
        .filter(|height| matches!(height, Height::Share))
        .count() as i64;
    let share = remaining.max(0) / shares.max(1);
    // A row of a table requires a line of the text of the box, and a chart
    // the least height at which its plot remains legible.
    let mut required = 1;
    for (piece, height) in pieces.iter().zip(&heights) {
        if let (Content::Element(object), Height::Share) = (&piece[0], height) {
            required = required.max(if object.kind == ElementKind::Table {
                let line = (measures.text_height)(&probe(vec![Content::Text("A".into())]))?;
                crate::tables::rows(object).len() as i64 * line
            } else {
                LEAST_CHART_HEIGHT
            });
        }
    }
    if remaining < unfixed || (shares > 0 && share < required) {
        warnings.push(format!("{described} exceeds its frame"));
    }

    let mut children = Vec::new();
    let frame: Properties = FRAME_ATTRIBUTES
        .into_iter()
        .filter_map(|name| Some((name.to_owned(), element.attributes.get(name)?.clone())))
        .collect();
    if frame.keys().any(|name| name != "opacity") {
        let mut attributes = frame;
        attributes.insert("kind".into(), Value::String("rect".into()));
        attributes.insert("position".into(), position([left, top, width, height]));
        children.push(Content::Element(Element {
            kind: ElementKind::Shape,
            ..positioned(element, attributes, Vec::new())
        }));
    }
    let mut offset = top;
    for (piece, height) in pieces.into_iter().zip(heights) {
        let (mut attributes, height) = match height {
            Height::Text(height) => (text_attributes.clone(), height),
            Height::Picture(height) => {
                let fit = element
                    .attributes
                    .get("fit")
                    .cloned()
                    .unwrap_or_else(|| Value::String("contain".into()));
                (Properties::from([("fit".to_owned(), fit)]), height)
            }
            // The text of a table takes the run formatting of its box.
            Height::Share => (
                match (&piece[0], text_attributes.get("text")) {
                    (Content::Element(table), Some(text)) if table.kind == ElementKind::Table => {
                        Properties::from([("text".to_owned(), text.clone())])
                    }
                    _ => Properties::new(),
                },
                share,
            ),
        };
        if height > 0 {
            attributes.insert("position".into(), position([left, offset, width, height]));
            children.push(Content::Element(positioned(element, attributes, piece)));
        }
        offset += height + PIECE_GAP;
    }
    let mut attributes = Properties::new();
    for name in ["name", "animate"] {
        if let Some(value) = element.attributes.get(name) {
            attributes.insert(name.into(), value.clone());
        }
    }
    Ok(Element {
        kind: ElementKind::Group,
        ..positioned(element, attributes, children)
    })
}
