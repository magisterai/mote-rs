use super::super::{PageOperation, Rule, RuleRole};
use super::*;
use crate::model::{Document, Value};
use std::collections::BTreeMap;
#[derive(Clone, Default, PartialEq)]
pub(super) struct Candidate {
    specified: bool,
    priority: u8,
    weight: u8,
    pub width: f64,
    color: String,
    style: String,
}
fn candidate(value: &Value, priority: u8, fallback: &str) -> Candidate {
    let properties = match value {
        Value::Object(properties) => properties.clone(),
        _ => Properties::from([("style".into(), value.clone())]),
    };
    let mut style = property_values::text(&properties, "style");
    if style.is_empty() {
        style = property_values::text(&properties, "val");
    }
    if style.is_empty() {
        style = "single".into();
    }
    let width = if matches!(style.as_str(), "nil" | "none") {
        0.0
    } else if let Some(value) = properties.get("width") {
        match value {
            Value::Number(number) => *number,
            _ => cells::points(Some(value)).unwrap_or(0.5),
        }
    } else if let Some(value) = properties.get("size") {
        match value {
            Value::Number(number) => number / 8.0,
            _ => cells::points(Some(value)).unwrap_or(0.5),
        }
    } else {
        0.5
    };
    Candidate {
        specified: true,
        priority,
        weight: match style.as_str() {
            "double" => 3,
            "thick" => 2,
            _ => 1,
        },
        width: width.max(0.0),
        color: super::super::style_values::color(&properties, "color")
            .unwrap_or_else(|| fallback.into()),
        style,
    }
}
pub(super) fn resolve(
    document: &Document,
    table: &Element,
    attributes: &Properties,
    position: [usize; 6],
) -> [Candidate; 4] {
    let [row, column, row_span, column_span, row_count, column_count] = position;
    let style =
        tables::styles::resolved(document, &property_values::text(&table.attributes, "style"));
    let style_borders = property_values::object(&style, "borders");
    let direct = property_values::object(&table.attributes, "borders");
    let mut borders = style_borders.cloned().unwrap_or_default();
    if let Some(direct) = direct {
        borders.extend(direct.clone());
    }
    let use_defaults = direct.is_none_or(Properties::is_empty)
        && style_borders.is_none_or(Properties::is_empty)
        && property_values::object(&style, "cell")
            .is_none_or(|cell| property_values::object(cell, "borders").is_none());
    let cell = property_values::object(attributes, "borders");
    let outside = [
        row == 0,
        column + column_span >= column_count,
        row + row_span >= row_count,
        column == 0,
    ];
    std::array::from_fn(|side| {
        let name = ["top", "right", "bottom", "left"][side];
        if let Some(value) = cell.and_then(|cell| cell.get(name).or_else(|| cell.get("all"))) {
            return candidate(value, 2, "D1D5DB");
        }
        let aliases = if outside[side] {
            [name, "outside", "all", ""]
        } else if side.is_multiple_of(2) {
            ["insideH", "inside_h", "inside", "all"]
        } else {
            ["insideV", "inside_v", "inside", "all"]
        };
        if let Some(value) = aliases.iter().find_map(|name| borders.get(*name)) {
            return candidate(value, 1, "BFBFBF");
        }
        if use_defaults {
            Candidate {
                specified: false,
                priority: 0,
                weight: 1,
                width: 0.5,
                color: if outside[side] { "BFBFBF" } else { "D9D9D9" }.into(),
                style: "single".into(),
            }
        } else {
            Candidate::default()
        }
    })
}
fn winner<'candidate>(
    preferred: &'candidate Candidate,
    alternate: &'candidate Candidate,
) -> &'candidate Candidate {
    if !preferred.specified && preferred.width <= 0.0 {
        return alternate;
    }
    if !alternate.specified && alternate.width <= 0.0 {
        return preferred;
    }
    if preferred.priority != alternate.priority {
        return if alternate.priority > preferred.priority {
            alternate
        } else {
            preferred
        };
    }
    if alternate.width != preferred.width {
        return if alternate.width > preferred.width {
            alternate
        } else {
            preferred
        };
    }
    if alternate.weight != preferred.weight {
        return if alternate.weight > preferred.weight {
            alternate
        } else {
            preferred
        };
    }
    fn brightness(color: &str) -> f64 {
        u32::from_str_radix(color, 16)
            .map(|value| {
                0.299 * ((value >> 16) & 255) as f64
                    + 0.587 * ((value >> 8) & 255) as f64
                    + 0.114 * (value & 255) as f64
            })
            .unwrap_or(0.0)
    }
    if brightness(&alternate.color) < brightness(&preferred.color) {
        alternate
    } else {
        preferred
    }
}
pub(super) fn add_row_heights(plan: &mut TablePlan, grid: &tables::TableGrid) {
    let borders: BTreeMap<_, _> = plan
        .rows
        .iter()
        .enumerate()
        .flat_map(|(row, cells)| {
            cells
                .cells
                .iter()
                .map(move |cell| ((row, cell.column), cell.borders.clone()))
        })
        .collect();
    for (row_index, row) in plan.rows.iter_mut().enumerate() {
        let mut width: f64 = 0.0;
        for column in 0..grid.column_count {
            let above = grid.rows[row_index]
                .get(column)
                .and_then(Option::as_ref)
                .map(|cell| (cell.origin_row, cell.origin_column));
            let below = grid
                .rows
                .get(row_index + 1)
                .and_then(|row| row.get(column))
                .and_then(Option::as_ref)
                .map(|cell| (cell.origin_row, cell.origin_column));
            if above == below {
                continue;
            }
            let empty = Candidate::default();
            let above = above
                .and_then(|origin| borders.get(&origin))
                .map(|borders| &borders[2])
                .unwrap_or(&empty);
            let below = below
                .and_then(|origin| borders.get(&origin))
                .map(|borders| &borders[0])
                .unwrap_or(&empty);
            width = width.max(winner(above, below).width);
        }
        row.height += width;
    }
}
type EdgeGroups = BTreeMap<(usize, bool, i64), Vec<(f64, f64, Candidate)>>;
#[derive(Default)]
pub(super) struct Collector {
    segments: EdgeGroups,
}
impl Collector {
    pub fn cell(
        &mut self,
        page: usize,
        bounds: [f64; 4],
        borders: &[Candidate; 4],
        first: bool,
        last: bool,
    ) {
        let [left, top, width, height] = bounds;
        for (side, horizontal, fixed, start, end, include) in [
            (0, true, top, left, left + width, first),
            (2, true, top + height, left, left + width, last),
            (3, false, left, top, top + height, true),
            (1, false, left + width, top, top + height, true),
        ] {
            if include && end - start > 0.01 {
                self.segments
                    .entry((page, !horizontal, (fixed * 1000.0).round() as i64))
                    .or_default()
                    .push((start, end, borders[side].clone()));
            }
        }
    }
    pub fn append(self, pages: &mut [super::super::TextPage]) {
        for ((page, vertical, fixed), segments) in self.segments {
            let mut points: Vec<f64> = segments
                .iter()
                .flat_map(|(start, end, _)| [*start, *end])
                .collect();
            points.sort_by(f64::total_cmp);
            points.dedup_by(|left, right| (*left - *right).abs() <= 0.01);
            let mut resolved: Vec<(f64, f64, Candidate)> = Vec::new();
            for endpoints in points.windows(2) {
                let middle = (endpoints[0] + endpoints[1]) / 2.0;
                let mut selected = Candidate::default();
                for (start, end, candidate) in &segments {
                    if middle > *start - 0.01 && middle < *end + 0.01 {
                        selected = winner(&selected, candidate).clone();
                    }
                }
                if selected.width <= 0.0 {
                    continue;
                }
                if let Some((_, end, previous)) = resolved.last_mut()
                    && *end == endpoints[0]
                    && previous.width == selected.width
                    && previous.color == selected.color
                    && previous.style == selected.style
                {
                    *end = endpoints[1];
                } else {
                    resolved.push((endpoints[0], endpoints[1], selected));
                }
            }
            for (start, end, candidate) in resolved {
                let fixed = fixed as f64 / 1000.0;
                let rule = Rule {
                    role: RuleRole::TableBorder,
                    start: if vertical {
                        [fixed, start]
                    } else {
                        [start, fixed]
                    },
                    end: if vertical { [fixed, end] } else { [end, fixed] },
                    width: candidate.width,
                    color: candidate.color,
                };
                let (dash, round_cap) = match candidate.style.as_str() {
                    "dashed" => (vec![3.0 * candidate.width; 2], false),
                    "dotted" => (
                        vec![(candidate.width / 100.0).max(0.01), 4.0 * candidate.width],
                        true,
                    ),
                    _ => (Vec::new(), false),
                };
                pages[page].operations.push(PageOperation::Stroke {
                    rule,
                    dash,
                    round_cap,
                });
            }
        }
    }
}
