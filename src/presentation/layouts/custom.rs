//! Resolve inheritance iteratively so long chains do not consume the call stack.
use super::{Layout, Slot, merge};
use crate::model::PresentationLayout;
use std::collections::{BTreeMap, BTreeSet};

pub(super) fn resolve(
    declarations: &[PresentationLayout],
    resolved: &mut BTreeMap<String, Layout>,
) -> Result<(), String> {
    let mut custom = BTreeMap::new();
    for declaration in declarations {
        if declaration.name.trim().is_empty() {
            return Err("layout name must not be empty".into());
        }
        if super::is_builtin(&declaration.name) {
            return Err(format!(
                "layout name {:?} is reserved for a built-in presentation layout",
                declaration.name
            ));
        }
        let mut names = BTreeSet::new();
        for slot in &declaration.slots {
            if slot.name.trim().is_empty() || !names.insert(&slot.name) {
                return Err(format!(
                    "layout {:?} has an empty or duplicate slot name",
                    declaration.name
                ));
            }
            if !["", "content", "text", "image", "table", "chart"].contains(&slot.kind.as_str()) {
                return Err(format!(
                    "layout slot {:?} has unknown kind {:?}",
                    slot.name, slot.kind
                ));
            }
            if slot.remove
                && (!slot.properties.is_empty() || !slot.kind.is_empty() || slot.required.is_some())
            {
                return Err(format!(
                    "layout slot {:?} cannot set properties when remove = true",
                    slot.name
                ));
            }
        }
        custom.insert(declaration.name.as_str(), declaration);
    }
    for declaration in declarations {
        let mut name = declaration.name.as_str();
        let mut visiting = BTreeSet::new();
        let mut chain = Vec::new();
        while !name.is_empty() && !resolved.contains_key(name) {
            let declaration = custom
                .get(name)
                .ok_or_else(|| format!("unknown presentation layout {name:?}"))?;
            if !visiting.insert(name) {
                return Err(format!(
                    "presentation layout inheritance contains a cycle at {name:?}"
                ));
            }
            chain.push(*declaration);
            name = &declaration.based_on;
        }
        while let Some(declaration) = chain.pop() {
            let mut layout = if declaration.based_on.is_empty() {
                Layout::default()
            } else {
                resolved[&declaration.based_on].clone()
            };
            if declaration.properties.contains_key("background") {
                layout.background_source_name = declaration.source_name.clone();
            }
            layout.properties = merge(&layout.properties, &declaration.properties);
            for declaration_slot in &declaration.slots {
                let index = layout
                    .slots
                    .iter()
                    .position(|slot| slot.name == declaration_slot.name);
                if declaration_slot.remove {
                    let index = index.ok_or_else(|| {
                        format!(
                            "layout {:?} cannot remove unknown inherited slot {:?}",
                            declaration.name, declaration_slot.name
                        )
                    })?;
                    layout.slots.remove(index);
                    continue;
                }
                if let Some(index) = index {
                    let slot = &mut layout.slots[index];
                    if !declaration_slot.kind.is_empty() {
                        slot.kind.clone_from(&declaration_slot.kind);
                    }
                    if let Some(required) = declaration_slot.required {
                        slot.required = required;
                    }
                    slot.properties = merge(&slot.properties, &declaration_slot.properties);
                } else {
                    layout.slots.push(Slot {
                        name: declaration_slot.name.clone(),
                        kind: if declaration_slot.kind.is_empty() {
                            "content".into()
                        } else {
                            declaration_slot.kind.clone()
                        },
                        required: declaration_slot.required.unwrap_or(false),
                        properties: declaration_slot.properties.clone(),
                    });
                }
            }
            for slot in &layout.slots {
                if !slot.properties.contains_key("position") {
                    return Err(format!(
                        "layout {:?} slot {:?} has no resolved position",
                        declaration.name, slot.name
                    ));
                }
            }
            resolved.insert(declaration.name.clone(), layout);
        }
    }
    Ok(())
}
