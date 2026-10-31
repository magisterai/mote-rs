//! Comment ranges and their independently related annotation story.
use super::{
    Error,
    document::{DocumentState, write_blocks},
    resources::Resources,
    run_properties::RunFormatting,
    runs,
    styles::Styles,
    xml,
};
use crate::{
    model::{Content, Element},
    properties,
};

pub(super) fn write_reference(
    element: &Element,
    formatting: &RunFormatting,
    styles: &Styles<'_>,
    resources: &mut Resources,
) -> Result<String, Error> {
    if !resources.in_main_story() {
        resources
            .warnings
            .push("DOCX flattened a comment in a running or annotation story".into());
        return runs::write_formatted(&flatten(&element.children), formatting, styles, resources);
    }
    let identifier = resources.comments.len();
    resources.comments.push(element.clone());
    let children =
        runs::write_formatted(&flatten(&element.children), formatting, styles, resources)?;
    let formatting = formatting.write(styles)?;
    let formatting = if formatting.is_empty() {
        formatting
    } else {
        format!("<w:rPr>{formatting}</w:rPr>")
    };
    Ok(format!(
        r#"<w:commentRangeStart w:id="{identifier}"/>{children}<w:commentRangeEnd w:id="{identifier}"/><w:r>{formatting}<w:commentReference w:id="{identifier}"/></w:r>"#
    ))
}
fn flatten(content: &[Content]) -> Vec<Content> {
    content
        .iter()
        .flat_map(|node| match node {
            Content::Element(element) if element.is_block() || element.kind.is_table_part() => {
                flatten(&element.children)
            }
            _ => vec![node.clone()],
        })
        .collect()
}
pub(super) fn write_part(styles: &Styles<'_>, state: &mut DocumentState) -> Result<(), Error> {
    if state.resources.comments.is_empty() {
        return Ok(());
    }
    let comments = state.resources.comments.clone();
    state.resources.story("comments", "comments.xml");
    let parent = state.resources.enter_story();
    let mut content = format!("{}<w:comments{}>", xml::HEADER, xml::ROOT_ATTRIBUTES);
    for (identifier, comment) in comments.iter().enumerate() {
        let author = properties::text(&comment.attributes, "author");
        let author = if author.is_empty() { "Mote" } else { &author };
        content.push_str(&format!(
            r#"<w:comment w:id="{identifier}" w:author="{}""#,
            xml::escape(author)?
        ));
        let initials = properties::text(&comment.attributes, "initials");
        if !initials.is_empty() {
            content.push_str(&format!(r#" w:initials="{}""#, xml::escape(&initials)?));
        }
        let date = properties::text(&comment.attributes, "date");
        if !date.is_empty() {
            content.push_str(&format!(r#" w:date="{}T00:00:00Z""#, xml::escape(&date)?));
        }
        content.push('>');
        if comment.children.is_empty() {
            content.push_str("<w:p/>");
        } else {
            content.push_str(&write_blocks(
                &crate::model::wrap_block_content(&comment.children),
                styles,
                state,
            )?);
        }
        content.push_str("</w:comment>");
    }
    content.push_str("</w:comments>");
    let relationships = state.resources.leave_story(parent)?;
    state.resources.stories.push(super::sections::StoryPart {
        kind: "comments",
        name: "comments.xml".into(),
        content,
        relationships,
    });
    Ok(())
}
