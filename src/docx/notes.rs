//! Note references and independently related note stories.
use super::{
    Error,
    document::{DocumentState, write_blocks},
    resources::Resources,
    run_properties::RunFormatting,
    runs,
    styles::Styles,
    xml,
};
use crate::model::{Content, Element, ElementKind};

#[derive(Default)]
pub(super) struct NoteCollection {
    pub writing: bool,
    footnotes: Vec<Element>,
    endnotes: Vec<Element>,
}

pub(super) fn write_reference(
    element: &Element,
    formatting: &RunFormatting,
    styles: &Styles<'_>,
    resources: &mut Resources,
) -> Result<String, Error> {
    super::check_attributes(&element.attributes, &["id"], element.kind.as_text())?;
    if !resources.in_main_story() {
        resources.warnings.push(format!("DOCX flattened {} content in a running or note story because nested annotation references are unsupported", element.kind.as_text()));
        return runs::write_formatted(
            &flatten_blocks(&element.children),
            formatting,
            styles,
            resources,
        );
    }
    let notes = if element.kind == ElementKind::Footnote {
        &mut resources.notes.footnotes
    } else {
        &mut resources.notes.endnotes
    };
    notes.push(element.clone());
    let identifier = notes.len();
    let properties = format!(
        r#"{}<w:vertAlign w:val="superscript"/>"#,
        formatting.write(styles)?
    );
    let properties = if properties.is_empty() {
        properties
    } else {
        format!("<w:rPr>{properties}</w:rPr>")
    };
    Ok(format!(
        r#"<w:r>{properties}<w:{}Reference w:id="{identifier}"/></w:r>"#,
        element.kind.as_text()
    ))
}

pub(super) fn write_parts(styles: &Styles<'_>, state: &mut DocumentState) -> Result<(), Error> {
    for (kind, notes) in [
        ("footnote", state.resources.notes.footnotes.clone()),
        ("endnote", state.resources.notes.endnotes.clone()),
    ] {
        if notes.is_empty() {
            continue;
        }
        let plural = if kind == "footnote" {
            "footnotes"
        } else {
            "endnotes"
        };
        let name = format!("{plural}.xml");
        state.resources.story(plural, &name);
        let parent = state.resources.enter_story();
        state.resources.notes.writing = true;
        let mut content = format!(
            r#"{}<w:{plural}{}><w:{kind} w:type="separator" w:id="-1"><w:p><w:r><w:separator/></w:r></w:p></w:{kind}><w:{kind} w:type="continuationSeparator" w:id="0"><w:p><w:r><w:continuationSeparator/></w:r></w:p></w:{kind}>"#,
            xml::HEADER,
            xml::ROOT_ATTRIBUTES
        );
        for (index, note) in notes.iter().enumerate() {
            content.push_str(&format!(r#"<w:{kind} w:id="{}">"#, index + 1));
            let (children, remaining) = match note.children.first() {
                Some(Content::Element(first)) if first.kind == ElementKind::Paragraph => {
                    (first.children.as_slice(), &note.children[1..])
                }
                _ => (&[][..], note.children.as_slice()),
            };
            content.push_str(&format!("<w:p><w:r><w:{kind}Ref/></w:r>"));
            if !children.is_empty() {
                content.push_str(r#"<w:r><w:t xml:space="preserve"> </w:t></w:r>"#);
                content.push_str(&runs::write_formatted(
                    &super::whitespace::trim_line_edges(children),
                    &RunFormatting::default(),
                    styles,
                    &mut state.resources,
                )?);
            }
            content.push_str("</w:p>");
            let remaining = crate::model::wrap_block_content(remaining);
            content.push_str(&write_blocks(&remaining, styles, state)?);
            content.push_str(&format!("</w:{kind}>"));
        }
        content.push_str(&format!("</w:{plural}>"));
        state.resources.notes.writing = false;
        let relationships = state.resources.leave_story(parent)?;
        state.resources.stories.push(super::sections::StoryPart {
            kind: plural,
            name,
            content,
            relationships,
        });
    }
    Ok(())
}

fn flatten_blocks(content: &[Content]) -> Vec<Content> {
    content
        .iter()
        .flat_map(|node| match node {
            Content::Element(element)
                if element.kind.is_block() || element.kind.is_table_part() =>
            {
                flatten_blocks(&element.children)
            }
            _ => vec![node.clone()],
        })
        .collect()
}
