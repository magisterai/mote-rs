//! Speaker-note text uses the same theme and text serializer as slide boxes.
use super::Error;
use crate::{
    measurement::TextMeasurer,
    model::{Content, Document, Element, ElementKind, Properties, Value},
    presentation::Theme,
    xml::HEADER,
};
pub(super) fn write(
    notes: &str,
    document: &Document,
    theme: &Theme,
    measurer: &mut TextMeasurer<'_>,
    warnings: &mut Vec<String>,
    next_shape: &mut usize,
) -> Result<String, Error> {
    let element = Element {
        kind: ElementKind::Box,
        attributes: Properties::from([
            ("name".into(), Value::String("Speaker Notes".into())),
            (
                "position".into(),
                Value::Object(Properties::from([
                    ("x".into(), Value::Number(457200.0)),
                    ("y".into(), Value::Number(457200.0)),
                    ("width".into(), Value::Number(5943600.0)),
                    ("height".into(), Value::Number(8229600.0)),
                ])),
            ),
        ]),
        children: vec![Content::Text(notes.into())],
    };
    let content = super::boxes::write(
        &element,
        document,
        theme,
        measurer,
        warnings,
        next_shape,
        &mut Vec::new(),
    )?;
    Ok(format!(
        "{HEADER}<p:notes {}><p:cSld><p:spTree>{}{content}</p:spTree></p:cSld><p:clrMapOvr><a:masterClrMapping/></p:clrMapOvr></p:notes>",
        super::ROOT_ATTRIBUTES,
        super::GROUP
    ))
}
pub(super) fn master() -> String {
    format!(
        r#"{HEADER}<p:notesMaster {}><p:cSld><p:spTree>{}</p:spTree></p:cSld><p:clrMap bg1="lt1" tx1="dk1" bg2="lt2" tx2="dk2" accent1="accent1" accent2="accent2" accent3="accent3" accent4="accent4" accent5="accent5" accent6="accent6" hlink="hlink" folHlink="folHlink"/><p:notesStyle/><p:clrMapOvr><a:masterClrMapping/></p:clrMapOvr></p:notesMaster>"#,
        super::ROOT_ATTRIBUTES,
        super::GROUP
    )
}
