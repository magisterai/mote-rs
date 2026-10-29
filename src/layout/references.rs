//! Native reference resolution and painted destination discovery.
use super::{DocumentTextPlan, TextPage};
use crate::model::{Content, ElementKind};
use std::collections::{BTreeMap, BTreeSet};

pub(super) fn leading_names(content: &[Content]) -> Vec<String> {
    let mut names = Vec::new();
    for node in content {
        match node {
            Content::Text(text) if text.trim().is_empty() => {}
            Content::Element(element)
                if matches!(element.kind, ElementKind::Label | ElementKind::Bookmark) =>
            {
                let name = crate::properties::text(&element.attributes, "name")
                    .trim()
                    .to_owned();
                if !name.is_empty() && !names.contains(&name) {
                    names.push(name);
                }
                if element.kind == ElementKind::Bookmark
                    && !crate::references::text(&element.children).trim().is_empty()
                {
                    break;
                }
            }
            _ => break,
        }
    }
    names
}
pub(crate) fn destinations(pages: &[TextPage]) -> BTreeMap<String, (usize, [f64; 2])> {
    let mut destinations = BTreeMap::new();
    for (index, page) in pages.iter().enumerate() {
        for (name, position) in &page.anchors {
            destinations
                .entry(name.clone())
                .or_insert((index, *position));
        }
    }
    destinations
}
pub(super) fn finalize(plan: &mut DocumentTextPlan) {
    let destinations = destinations(&plan.pages);
    let mut unresolved = BTreeSet::new();
    for page in &mut plan.pages {
        for placed in &mut page.lines {
            for fragment in &mut placed.line.fragments {
                if let Some(target) = &fragment.internal_target
                    && !destinations.contains_key(target)
                {
                    unresolved.insert(target.clone());
                    fragment.internal_target = None;
                }
            }
        }
    }
    for target in unresolved {
        plan.warnings.push(format!("reference target {target:?} has no placed native destination; visible text is retained without a link"));
    }
}
