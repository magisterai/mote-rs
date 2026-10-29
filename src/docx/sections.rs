//! Section boundaries and independently related header/footer parts.
use super::{
    Error, configuration,
    document::{DocumentState, write_blocks},
    styles::Styles,
    values, xml,
};
use crate::model::{Content, Document, Element, ElementKind, HeaderFooter, Section, Unit, Value};

pub(super) struct StoryPart {
    pub kind: &'static str,
    pub name: String,
    pub content: String,
    pub relationships: Option<String>,
}

pub(super) fn write(
    document: &Document,
    styles: &Styles<'_>,
    state: &mut DocumentState,
) -> Result<String, Error> {
    if document.sections.is_empty() {
        let mut body = write_blocks(&document.content, styles, state)?;
        body.push_str(&configuration::section_properties(document)?);
        return Ok(body);
    }
    let mut body = String::new();
    let mut previous_header = false;
    let mut previous_footer = false;
    for (index, section) in document.sections.iter().enumerate() {
        body.push_str(&write_blocks(&section.content, styles, state)?);
        let mut prefix = String::new();
        for (kind, story, previous) in [
            ("header", &section.header, previous_header),
            ("footer", &section.footer, previous_footer),
        ] {
            if !story.content.is_empty() || previous {
                let relationship = write_story(kind, story, styles, state)?;
                prefix.push_str(&format!(
                    r#"<w:{kind}Reference w:type="default" r:id="{relationship}"/>"#
                ));
            }
        }
        previous_header = !section.header.content.is_empty();
        previous_footer = !section.footer.content.is_empty();
        if !section.start.is_empty() {
            let start = match section.start.as_str() {
                "continuous" => "continuous",
                "even_page" => "evenPage",
                "odd_page" => "oddPage",
                "next_column" => "nextColumn",
                _ => "nextPage",
            };
            prefix.push_str(&format!(r#"<w:type w:val="{start}"/>"#));
        }
        let geometry = geometry_document(document, section)?;
        let numbering = page_numbering(section)?;
        let alignment = crate::properties::text(&section.section_properties, "vertical_alignment");
        let suffix = if alignment.is_empty() {
            String::new()
        } else {
            let alignment = if alignment == "justified" {
                "both"
            } else {
                &alignment
            };
            format!(r#"<w:vAlign w:val="{}"/>"#, xml::escape(alignment)?)
        };
        let properties = configuration::section_geometry(&geometry, &prefix, &numbering, &suffix)?;
        if index + 1 == document.sections.len() {
            body.push_str(&properties);
        } else {
            body.push_str(&format!("<w:p><w:pPr>{properties}</w:pPr></w:p>"));
        }
    }
    Ok(body)
}

fn geometry_document(document: &Document, section: &Section) -> Result<Document, Error> {
    let mut geometry = Document {
        properties: document.properties.clone(),
        page: section.page.clone(),
        margin: section.margin.clone(),
        columns: section.columns.clone(),
        ..Document::default()
    };
    for (side, kind, story) in [
        ("top", "header", &section.header),
        ("bottom", "footer", &section.footer),
    ] {
        if !geometry.margin.contains_key(kind)
            && let Some(distance) = story.properties.get("distance")
        {
            geometry.margin.insert(kind.into(), distance.clone());
        }
        if let Some(height) = story.properties.get("height") {
            let height = values::twips(height)?;
            let distance = geometry
                .margin
                .get(kind)
                .map(values::twips)
                .transpose()?
                .unwrap_or(720);
            let margin = geometry
                .margin
                .get(side)
                .map(values::twips)
                .transpose()?
                .unwrap_or(1440);
            geometry.margin.insert(
                side.into(),
                Value::Length {
                    value: margin.max(height + distance) as f64 / 20.0,
                    unit: Unit::Point,
                },
            );
        }
    }
    Ok(geometry)
}

fn page_numbering(section: &Section) -> Result<String, Error> {
    let mut start =
        crate::properties::number(&section.page_numbering, "start").unwrap_or(0.0) as i64;
    if start == 0 && crate::properties::boolean(&section.page_numbering, "restart") {
        start = 1;
    }
    let format = crate::properties::text(&section.page_numbering, "format");
    if start == 0 && format.is_empty() {
        return Ok(String::new());
    }
    let mut result = String::from("<w:pgNumType");
    if start > 0 {
        result.push_str(&format!(r#" w:start="{start}""#));
    }
    if !format.is_empty() {
        let format = match format.as_str() {
            "lower_alpha" => "lowerLetter",
            "upper_alpha" => "upperLetter",
            "lower_roman" => "lowerRoman",
            "upper_roman" => "upperRoman",
            _ => "decimal",
        };
        result.push_str(&format!(r#" w:fmt="{format}""#));
    }
    result.push_str("/>");
    Ok(result)
}

fn write_story(
    kind: &'static str,
    story: &HeaderFooter,
    styles: &Styles<'_>,
    state: &mut DocumentState,
) -> Result<String, Error> {
    let number = state
        .resources
        .stories
        .iter()
        .filter(|story| story.kind == kind)
        .count()
        + 1;
    let name = format!("{kind}{number}.xml");
    let relationship = state.resources.story(kind, &name);
    let parent = state.resources.enter_story();
    let style = if kind == "header" { "Header" } else { "Footer" };
    let tag = if kind == "header" { "hdr" } else { "ftr" };
    let mut content = story.content.clone();
    if content.is_empty() {
        content.push(Content::Element(Element {
            kind: ElementKind::Paragraph,
            attributes: Default::default(),
            children: Vec::new(),
        }));
    }
    for child in &mut content {
        if let Content::Element(element) = child
            && element.kind == ElementKind::Paragraph
            && crate::properties::text(&element.attributes, "style").is_empty()
        {
            element
                .attributes
                .insert("style".into(), Value::String(style.into()));
        }
    }
    let written = write_blocks(&content, styles, state);
    let relationships = state.resources.leave_story(parent)?;
    let content = format!(
        "{}<w:{tag}{}>{}</w:{tag}>",
        xml::HEADER,
        xml::ROOT_ATTRIBUTES,
        written?
    );
    state.resources.stories.push(StoryPart {
        kind,
        name,
        content,
        relationships,
    });
    Ok(relationship)
}

pub(super) fn field_number_format(format: &str) -> &'static str {
    match format {
        "lower_alpha" => "alphabetic",
        "upper_alpha" => "ALPHABETIC",
        "lower_roman" => "roman",
        "upper_roman" => "ROMAN",
        _ => "ARABIC",
    }
}
