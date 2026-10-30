use super::{Error, escape};
use crate::{
    model::{Content, Document, ElementKind},
    presentation::Deck,
    xml::HEADER,
};
use std::{
    collections::BTreeMap,
    io::{Cursor, Write},
};

pub(super) struct Relationship {
    pub kind: &'static str,
    pub target: String,
    pub external: bool,
}
pub(super) fn relationships(values: &[Relationship]) -> Result<String, Error> {
    let mut output = format!(
        r#"{HEADER}<Relationships xmlns="http://schemas.openxmlformats.org/package/2006/relationships">"#
    );
    for (index, value) in values.iter().enumerate() {
        let target = escape(&value.target)?;
        let mode = if value.external {
            r#" TargetMode="External""#
        } else {
            ""
        };
        let kind = if value.kind == "core-properties" {
            "http://schemas.openxmlformats.org/package/2006/relationships/metadata/core-properties"
                .into()
        } else {
            format!(
                "http://schemas.openxmlformats.org/officeDocument/2006/relationships/{}",
                value.kind
            )
        };
        output.push_str(&format!(
            r#"<Relationship Id="rId{}" Type="{kind}" Target="{target}"{mode}/>"#,
            index + 1
        ));
    }
    output.push_str("</Relationships>");
    Ok(output)
}
fn relation(kind: &'static str, target: impl Into<String>) -> Relationship {
    Relationship {
        kind,
        target: target.into(),
        external: false,
    }
}
fn template(value: &str) -> String {
    value.replace(">\n<", "><").trim_end().to_owned()
}
pub(super) fn write(
    document: &Document,
    deck: &mut Deck,
    measurer: &mut crate::measurement::TextMeasurer<'_>,
) -> Result<Vec<u8>, Error> {
    let mut parts = BTreeMap::new();
    for (name, value) in [
        (
            "ppt/slideLayouts/slideLayout1.xml",
            include_str!("templates/blank-layout.xml"),
        ),
        (
            "ppt/presProps.xml",
            include_str!("templates/presentation-properties.xml"),
        ),
        (
            "ppt/viewProps.xml",
            include_str!("templates/view-properties.xml"),
        ),
        (
            "ppt/tableStyles.xml",
            include_str!("templates/table-styles.xml"),
        ),
        (
            "docProps/core.xml",
            include_str!("templates/core-properties.xml"),
        ),
    ] {
        parts.insert(name.to_owned(), template(value));
    }
    parts.insert(
        "docProps/core.xml".into(),
        template(include_str!("templates/core-properties.xml")).replace(
            "{{title}}",
            &escape(&crate::properties::text(&document.properties, "title"))?,
        ),
    );
    parts.insert(
        "_rels/.rels".into(),
        relationships(&[
            relation("officeDocument", "ppt/presentation.xml"),
            relation("core-properties", "docProps/core.xml"),
            relation("extended-properties", "docProps/app.xml"),
        ])?,
    );
    parts.insert(
        "ppt/slideMasters/_rels/slideMaster1.xml.rels".into(),
        relationships(&[
            relation("slideLayout", "../slideLayouts/slideLayout1.xml"),
            relation("theme", "../theme/theme1.xml"),
        ])?,
    );
    parts.insert(
        "ppt/slideLayouts/_rels/slideLayout1.xml.rels".into(),
        relationships(&[relation("slideMaster", "../slideMasters/slideMaster1.xml")])?,
    );
    let mut master = template(include_str!("templates/slide-master.xml"));
    for (name, value) in [
        ("content_width", (deck.width - 914400).max(0)),
        ("body_height", (deck.height - 2332037).max(0)),
        ("footer_y", (deck.height - 501650).max(0)),
        ("footer_center", ((deck.width - 2895600) / 2).max(0)),
        ("slide_number_x", (deck.width - 2590800).max(0)),
    ] {
        master = master.replace(&format!("{{{{{name}}}}}"), &value.to_string());
    }
    parts.insert("ppt/slideMasters/slideMaster1.xml".into(), master);
    let mut theme = template(include_str!("templates/theme.xml"))
        .replace("{{theme_name}}", &escape(&deck.theme.name)?)
        .replace("{{heading_font}}", &escape(&deck.theme.heading)?)
        .replace("{{body_font}}", &escape(&deck.theme.body)?);
    for (index, color) in deck.theme.colors.iter().enumerate() {
        theme = theme.replace(&format!("{{{{color_{index}}}}}"), color);
    }
    parts.insert("ppt/theme/theme1.xml".into(), theme);
    let mut slide_ids = String::new();
    let mut presentation_relations = vec![relation("slideMaster", "slideMasters/slideMaster1.xml")];
    let mut next_shape = 2;
    for (index, slide) in deck.slides.iter().enumerate() {
        let number = index + 1;
        slide_ids.push_str(&format!(
            r#"<p:sldId id="{}" r:id="rId{}"/>"#,
            256 + index,
            index + 2
        ));
        presentation_relations.push(relation("slide", format!("slides/slide{number}.xml")));
        let mut output = format!("{HEADER}<p:sld {}><p:cSld>", super::ROOT_ATTRIBUTES);
        if let Some(color) = deck.background(document, slide).map_err(Error)? {
            output.push_str(&format!(r#"<p:bg><p:bgPr><a:solidFill><a:srgbClr val="{color}"/></a:solidFill></p:bgPr></p:bg>"#));
        }
        output.push_str("<p:spTree>");
        output.push_str(super::GROUP);
        let mut relations = vec![relation("slideLayout", "../slideLayouts/slideLayout1.xml")];
        for content in &slide.children {
            let Content::Element(element) = content else {
                return Err(Error(
                    "PPTX currently requires positioned slide objects".into(),
                ));
            };
            if element.kind != ElementKind::Box {
                return Err(Error(format!(
                    "PPTX does not yet support {} slide objects",
                    element.kind.as_text()
                )));
            }
            if element.children.is_empty() {
                deck.warnings.push("PPTX omits empty boxes".into());
                continue;
            }
            output.push_str(&super::boxes::write(
                element,
                document,
                &deck.theme,
                measurer,
                &mut deck.warnings,
                &mut next_shape,
                &mut relations,
            )?);
        }
        output.push_str(
            "</p:spTree></p:cSld><p:clrMapOvr><a:masterClrMapping/></p:clrMapOvr></p:sld>",
        );
        parts.insert(format!("ppt/slides/slide{number}.xml"), output);
        parts.insert(
            format!("ppt/slides/_rels/slide{number}.xml.rels"),
            relationships(&relations)?,
        );
    }
    for (kind, target) in [
        ("presProps", "presProps.xml"),
        ("viewProps", "viewProps.xml"),
        ("theme", "theme/theme1.xml"),
        ("tableStyles", "tableStyles.xml"),
    ] {
        presentation_relations.push(relation(kind, target));
    }
    parts.insert(
        "ppt/_rels/presentation.xml.rels".into(),
        relationships(&presentation_relations)?,
    );
    let presentation = template(include_str!("templates/presentation.xml"))
        .replace("{{slide_ids}}", &slide_ids)
        .replace("{{width}}", &deck.width.to_string())
        .replace("{{height}}", &deck.height.to_string())
        .replace("{{size_type}}", deck.size_type);
    parts.insert("ppt/presentation.xml".into(), presentation);
    let titles = (1..=deck.slides.len())
        .map(|number| format!("<vt:lpstr>Slide {number}</vt:lpstr>"))
        .collect::<String>();
    parts.insert(
        "docProps/app.xml".into(),
        template(include_str!("templates/application-properties.xml"))
            .replace("{{slide_count}}", &deck.slides.len().to_string())
            .replace("{{titles}}", &titles),
    );
    let mut types = format!(
        r#"{HEADER}<Types xmlns="http://schemas.openxmlformats.org/package/2006/content-types"><Default Extension="rels" ContentType="application/vnd.openxmlformats-package.relationships+xml"/><Default Extension="xml" ContentType="application/xml"/>"#
    );
    for (extension, kind) in [
        ("png", "png"),
        ("jpg", "jpeg"),
        ("jpeg", "jpeg"),
        ("gif", "gif"),
    ] {
        types.push_str(&format!(
            r#"<Default Extension="{extension}" ContentType="image/{kind}"/>"#
        ));
    }
    for (name, kind) in [
        ("docProps/core.xml", "package.core-properties+xml"),
        ("docProps/app.xml", "extended-properties+xml"),
        (
            "ppt/presentation.xml",
            "presentationml.presentation.main+xml",
        ),
        (
            "ppt/slideMasters/slideMaster1.xml",
            "presentationml.slideMaster+xml",
        ),
        (
            "ppt/slideLayouts/slideLayout1.xml",
            "presentationml.slideLayout+xml",
        ),
        ("ppt/theme/theme1.xml", "theme+xml"),
        ("ppt/presProps.xml", "presentationml.presProps+xml"),
        ("ppt/viewProps.xml", "presentationml.viewProps+xml"),
        ("ppt/tableStyles.xml", "presentationml.tableStyles+xml"),
    ] {
        let prefix = if kind.starts_with("package.") {
            "application/vnd.openxmlformats-"
        } else {
            "application/vnd.openxmlformats-officedocument."
        };
        types.push_str(&format!(
            r#"<Override PartName="/{name}" ContentType="{prefix}{kind}"/>"#
        ));
    }
    for number in 1..=deck.slides.len() {
        types.push_str(&format!(r#"<Override PartName="/ppt/slides/slide{number}.xml" ContentType="application/vnd.openxmlformats-officedocument.presentationml.slide+xml"/>"#));
    }
    types.push_str("</Types>");
    parts.insert("[Content_Types].xml".into(), types);
    let mut archive = zip::ZipWriter::new(Cursor::new(Vec::new()));
    for (name, value) in parts {
        archive.start_file(
            name,
            zip::write::SimpleFileOptions::default()
                .compression_method(zip::CompressionMethod::Deflated),
        )?;
        archive.write_all(value.as_bytes())?;
    }
    Ok(archive.finish()?.into_inner())
}
