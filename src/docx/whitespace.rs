//! Trim ordinary paragraph line edges without changing explicitly preserved text.
use crate::model::{Content, ElementKind, Value};

pub(super) fn trim_line_edges(children: &[Content]) -> Vec<Content> {
    let mut result = children.to_vec();
    trim_direction(&mut result, true, true);
    trim_direction(&mut result, false, true);
    result
}

fn trim_direction(children: &mut Vec<Content>, leading: bool, mut at_edge: bool) -> bool {
    let indices: Vec<_> = if leading {
        (0..children.len()).collect()
    } else {
        (0..children.len()).rev().collect()
    };
    for index in indices {
        match &mut children[index] {
            Content::Text(text) => {
                let mut lines: Vec<_> = text.split('\n').map(str::to_owned).collect();
                let line_indices: Vec<_> = if leading {
                    (0..lines.len()).collect()
                } else {
                    (0..lines.len()).rev().collect()
                };
                for line_index in line_indices {
                    if at_edge {
                        lines[line_index] = if leading {
                            lines[line_index]
                                .trim_start_matches([' ', '\t', '\r'])
                                .into()
                        } else {
                            lines[line_index].trim_end_matches([' ', '\t', '\r']).into()
                        };
                    }
                    if !lines[line_index].is_empty() {
                        at_edge = false;
                    }
                    if (leading && line_index + 1 < lines.len()) || (!leading && line_index > 0) {
                        at_edge = true;
                    }
                }
                *text = lines.join("\n");
            }
            Content::Element(element) if element.kind == ElementKind::LineBreak => at_edge = true,
            Content::Element(element) => {
                if element.attributes.get("preserve_space") == Some(&Value::Boolean(true)) {
                    if let Some(boundary) = preserved_boundary(&element.children, leading) {
                        at_edge = boundary;
                    }
                } else {
                    at_edge = trim_direction(&mut element.children, leading, at_edge);
                }
            }
        }
    }
    children.retain(|child| !matches!(child, Content::Text(text) if text.is_empty()));
    at_edge
}

fn preserved_boundary(children: &[Content], from_end: bool) -> Option<bool> {
    let indices: Vec<_> = if from_end {
        (0..children.len()).rev().collect()
    } else {
        (0..children.len()).collect()
    };
    for index in indices {
        match &children[index] {
            Content::Text(text) if !text.is_empty() => {
                return Some(if from_end {
                    text.ends_with('\n')
                } else {
                    text.starts_with('\n')
                });
            }
            Content::Element(element) if element.kind == ElementKind::LineBreak => {
                return Some(true);
            }
            Content::Element(element) => {
                if let Some(boundary) = preserved_boundary(&element.children, from_end) {
                    return Some(boundary);
                }
            }
            _ => {}
        }
    }
    None
}
