use super::{
    LayoutError,
    numbering::{integer, text_or},
    style_values as values,
};
use crate::model::{Content, Element, ElementKind, Properties, Value};
use std::{cell::RefCell, collections::BTreeMap, rc::Rc};

#[derive(Clone, Copy)]
pub(super) struct ListGeometry {
    pub left: f64,
    pub level_indent: f64,
    pub hanging: f64,
    pub after: f64,
}
impl Default for ListGeometry {
    fn default() -> Self {
        Self {
            left: 36.0,
            level_indent: 18.0,
            hanging: 18.0,
            after: 2.0,
        }
    }
}
impl ListGeometry {
    pub fn apply(mut self, properties: &Properties) -> Self {
        if values::boolean(properties, "tight") {
            self.after = 0.0;
        }
        if let Some(Value::Object(indent)) = properties.get("indent") {
            for (name, target) in [
                ("left", &mut self.left),
                ("level", &mut self.level_indent),
                ("hanging", &mut self.hanging),
            ] {
                if let Some(value) = indent.get(name).and_then(values::points) {
                    *target = value;
                }
            }
        }
        if let Some(Value::Object(spacing)) = properties.get("spacing")
            && let Some(value) = spacing.get("after").and_then(values::points)
        {
            self.after = value;
        }
        self
    }
    pub fn item(self, properties: &Properties, level: usize) -> Self {
        let mut geometry = self.apply(properties);
        geometry.left += level as f64 * geometry.level_indent;
        if let Some(Value::Object(indent)) = properties.get("indent")
            && let Some(value) = indent.get("left").and_then(values::points)
        {
            geometry.left = value;
        }
        geometry
    }
}
pub(super) fn level(properties: &Properties, fallback: usize) -> usize {
    integer(properties, "level", fallback as u64 + 1) as usize - 1
}

type CounterInstance = Rc<RefCell<BTreeMap<usize, u64>>>;
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord)]
struct ListKey {
    format: String,
    identity: String,
    geometry: [i64; 4],
}
#[derive(Default)]
pub(super) struct ListNumbers {
    instances: BTreeMap<ListKey, CounterInstance>,
}
impl Clone for ListNumbers {
    fn clone(&self) -> Self {
        Self {
            instances: self
                .instances
                .iter()
                .map(|(key, counter)| {
                    (key.clone(), Rc::new(RefCell::new(counter.borrow().clone())))
                })
                .collect(),
        }
    }
}
pub(super) struct ListCursor {
    key: ListKey,
    level: usize,
    start: u64,
    restart: bool,
    instance: Option<CounterInstance>,
}
impl ListCursor {
    pub fn new(list: &Element, level: usize, geometry: ListGeometry) -> Self {
        let identity = values::text(&list.attributes, "id");
        let restart = if list.attributes.contains_key("restart") {
            values::boolean(&list.attributes, "restart")
        } else {
            identity.is_empty()
        };
        let format = text_or(&list.attributes, "format", "decimal");
        let key = ListKey {
            format,
            identity: if identity.is_empty() {
                format!("level:{level}")
            } else {
                identity
            },
            geometry: [
                geometry.left,
                geometry.level_indent,
                geometry.hanging,
                geometry.after,
            ]
            .map(|value| (value * 20.0).round() as i64),
        };
        let source_start = list
            .children
            .iter()
            .find_map(|child| match child {
                Content::Element(item) if item.kind == ElementKind::ListItem => Some(
                    source_marker_start(&values::text(&item.attributes, "marker")),
                ),
                _ => None,
            })
            .flatten()
            .unwrap_or(1);
        Self {
            key,
            level,
            start: integer(&list.attributes, "start", source_start),
            restart,
            instance: None,
        }
    }
    pub fn next(
        &mut self,
        sequence: &mut ListNumbers,
        level: usize,
    ) -> Result<String, LayoutError> {
        if self.instance.is_none() {
            if self.restart || !sequence.instances.contains_key(&self.key) {
                sequence.instances.insert(
                    self.key.clone(),
                    Rc::new(RefCell::new(BTreeMap::from([(self.level, self.start)]))),
                );
            }
            let instance = sequence.instances[&self.key].clone();
            instance.borrow_mut().entry(self.level).or_insert(1);
            self.instance = Some(instance);
        }
        let mut instance = self.instance.as_ref().unwrap().borrow_mut();
        let number = *instance.get(&level).unwrap_or(&1);
        instance.insert(
            level,
            number
                .checked_add(1)
                .ok_or(LayoutError::InvalidGeometry("list counter overflow"))?,
        );
        instance.retain(|candidate, _| *candidate <= level);
        Ok(crate::numbering::format_counter(number, &self.key.format)
            .map_err(super::LayoutError::InvalidGeometry)?
            + ".")
    }
}
fn source_marker_start(marker: &str) -> Option<u64> {
    let mut marker = marker.trim();
    for suffix in ["[ ]", "[x]", "[X]"] {
        if let Some(prefix) = marker.strip_suffix(suffix)
            && prefix.ends_with([' ', '\t'])
        {
            marker = prefix.trim();
            break;
        }
    }
    let digits = marker.strip_suffix('.')?;
    if digits.is_empty() || !digits.bytes().all(|byte| byte.is_ascii_digit()) {
        return None;
    }
    digits.parse::<u64>().ok().filter(|number| *number > 0)
}
