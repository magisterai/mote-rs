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
fn render_template(source: &str, values: &[(&str, String)]) -> Result<String, Error> {
    let source = source.replace(">\n<", "><");
    let mut remaining = source.trim_end();
    let mut output = String::new();
    // Scan only the template. Authored values may themselves contain tokens.
    while let Some((literal, token)) = remaining.split_once("{{") {
        output.push_str(literal);
        let (name, rest) = token
            .split_once("}}")
            .ok_or_else(|| Error("unterminated PPTX template token".into()))?;
        let (_, value) = values
            .iter()
            .find(|(key, _)| *key == name)
            .ok_or_else(|| Error(format!("unknown PPTX template token: {name}")))?;
        output.push_str(value);
        remaining = rest;
    }
    output.push_str(remaining);
    Ok(output)
}
pub(super) fn write(
    document: &Document,
    deck: &mut Deck,
    measurer: &mut crate::measurement::TextMeasurer<'_>,
    images: &crate::media::ImageCatalog,
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
    ] {
        parts.insert(name.to_owned(), render_template(value, &[])?);
    }
    parts.insert(
        "docProps/core.xml".into(),
        render_template(
            include_str!("templates/core-properties.xml"),
            &[(
                "title",
                escape(&crate::properties::text(&document.properties, "title"))?,
            )],
        )?,
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
    let master_values = [
        ("content_width", (deck.width - 914400).max(0)),
        ("body_height", (deck.height - 2332037).max(0)),
        ("footer_y", (deck.height - 501650).max(0)),
        ("footer_center", ((deck.width - 2895600) / 2).max(0)),
        ("slide_number_x", (deck.width - 2590800).max(0)),
    ]
    .map(|(name, value)| (name, value.to_string()));
    let master = render_template(include_str!("templates/slide-master.xml"), &master_values)?;
    parts.insert("ppt/slideMasters/slideMaster1.xml".into(), master);
    let color_names: Vec<_> = (0..deck.theme.colors.len())
        .map(|index| format!("color_{index}"))
        .collect();
    let mut theme_values = vec![
        ("theme_name", escape(&deck.theme.name)?),
        ("heading_font", escape(&deck.theme.heading)?),
        ("body_font", escape(&deck.theme.body)?),
    ];
    theme_values.extend(
        color_names
            .iter()
            .zip(&deck.theme.colors)
            .map(|(name, color)| (name.as_str(), color.clone())),
    );
    let theme = render_template(include_str!("templates/theme.xml"), &theme_values)?;
    parts.insert("ppt/theme/theme1.xml".into(), theme);
    let mut slide_ids = String::new();
    let mut presentation_relations = vec![relation("slideMaster", "slideMasters/slideMaster1.xml")];
    let mut next_shape = 2;
    let mut pictures = super::pictures::Pictures::new(images);
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
            if let [Content::Element(table)] = element.children.as_slice()
                && table.kind == ElementKind::Table
            {
                output.push_str(&super::tables::write(
                    document,
                    element,
                    table,
                    &deck.theme,
                    &mut next_shape,
                    &mut relations,
                    &mut deck.warnings,
                )?);
                continue;
            }
            let mut placeholder;
            let element = if let Some(image) = crate::presentation::images::child(element) {
                if let Some(picture) =
                    pictures.write(element, image, &mut next_shape, &mut relations)?
                {
                    output.push_str(&picture);
                    continue;
                }
                placeholder = element.clone();
                let source = crate::properties::text(&image.attributes, "src");
                let label = if source.is_empty() {
                    "[missing image]".into()
                } else {
                    format!("[image: {source}]")
                };
                placeholder.children = vec![Content::Text(label)];
                &placeholder
            } else {
                element
            };
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
    let presentation = render_template(
        include_str!("templates/presentation.xml"),
        &[
            ("slide_ids", slide_ids),
            ("width", deck.width.to_string()),
            ("height", deck.height.to_string()),
            ("size_type", deck.size_type.into()),
        ],
    )?;
    parts.insert("ppt/presentation.xml".into(), presentation);
    let titles = (1..=deck.slides.len())
        .map(|number| format!("<vt:lpstr>Slide {number}</vt:lpstr>"))
        .collect::<String>();
    parts.insert(
        "docProps/app.xml".into(),
        render_template(
            include_str!("templates/application-properties.xml"),
            &[
                ("slide_count", deck.slides.len().to_string()),
                ("titles", titles),
            ],
        )?,
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
    for (name, bytes) in pictures.media {
        archive.start_file(
            name,
            zip::write::SimpleFileOptions::default()
                .compression_method(zip::CompressionMethod::Deflated),
        )?;
        archive.write_all(&bytes)?;
    }
    Ok(archive.finish()?.into_inner())
}
