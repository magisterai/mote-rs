//! Resolve each physical edge once, preserving explicit border suppression.
use crate::{
    layout::{LineCap, PageOperation, PaintedPath, PathPaint, PathStroke},
    model::Properties,
    presentation::{
        color,
        tables::{SlideTable, border_for_side},
        units,
    },
    properties::{number, object, text},
};
use std::collections::BTreeMap;

#[derive(Default)]
struct Edge {
    specified: bool,
    stroke: Option<PathStroke>,
}
type Origin = (usize, usize);

pub(super) fn operations(
    table: &SlideTable,
    origin: [f64; 2],
    columns: &[f64],
    rows: &[f64],
) -> Vec<PageOperation> {
    let mut edges = BTreeMap::<Origin, [Edge; 4]>::new();
    for (row_index, row) in table.grid.rows.iter().enumerate() {
        for (column, cell) in row.iter().enumerate() {
            let Some(cell) = cell
                .as_ref()
                .filter(|cell| !cell.continuation && cell.origin_column == column)
            else {
                continue;
            };
            let format = table.formats[row_index][column]
                .as_ref()
                .expect("origin format");
            let empty = Properties::new();
            let borders = if !table.neutral_style && format.cell.contains_key("fill") {
                &empty
            } else {
                &table.borders
            };
            edges.insert(
                (row_index, column),
                ["top", "right", "bottom", "left"].map(|side| {
                    let border = border_for_side(
                        object(&format.cell, "borders"),
                        borders,
                        side,
                        cell,
                        table.grid.rows.len(),
                        table.grid.column_count,
                    );
                    Edge {
                        specified: border.is_some(),
                        stroke: border.as_ref().and_then(stroke).or_else(|| {
                            (border.is_none() && !table.neutral_style)
                                .then(|| stroke(&Properties::new()).unwrap())
                        }),
                    }
                }),
            );
        }
    }
    let cell_origin = |row: usize, column: usize| {
        table
            .grid
            .rows
            .get(row)
            .and_then(|row| row.get(column))
            .and_then(Option::as_ref)
            .map(|cell| (cell.origin_row, cell.origin_column))
    };
    let edge = |origin: Option<Origin>, side: usize| {
        origin
            .and_then(|origin| edges.get(&origin))
            .map(|edges| &edges[side])
    };
    let mut paths = Vec::new();
    for (boundary, vertical) in rows.iter().enumerate() {
        for column in 0..table.grid.column_count {
            let above = boundary
                .checked_sub(1)
                .and_then(|row| cell_origin(row, column));
            let below = cell_origin(boundary, column);
            if above.is_some() && above == below {
                continue;
            }
            append(
                &mut paths,
                [origin[0] + columns[column], origin[1] + vertical],
                [origin[0] + columns[column + 1], origin[1] + vertical],
                preferred(edge(above, 2), edge(below, 0)),
            );
        }
    }
    for (boundary, horizontal) in columns.iter().enumerate() {
        for row in 0..table.grid.rows.len() {
            let left = boundary
                .checked_sub(1)
                .and_then(|column| cell_origin(row, column));
            let right = cell_origin(row, boundary);
            if left.is_some() && left == right {
                continue;
            }
            append(
                &mut paths,
                [origin[0] + horizontal, origin[1] + rows[row]],
                [origin[0] + horizontal, origin[1] + rows[row + 1]],
                preferred(edge(left, 1), edge(right, 3)),
            );
        }
    }
    paths
        .into_iter()
        .map(|path| {
            let stroke = path.stroke.as_ref().expect("border stroke");
            if stroke.paint.alpha == 255 && stroke.cap != LineCap::Square {
                PageOperation::Stroke {
                    rule: crate::layout::Rule {
                        role: crate::layout::RuleRole::TableBorder,
                        start: path.points[0],
                        end: path.points[1],
                        width: stroke.width,
                        color: stroke.paint.color.clone(),
                    },
                    dash: stroke.dash.clone(),
                    round_cap: stroke.cap == LineCap::Round,
                }
            } else {
                PageOperation::Path(path)
            }
        })
        .collect()
}
fn preferred<'a>(first: Option<&'a Edge>, second: Option<&'a Edge>) -> Option<&'a PathStroke> {
    if let Some(first) = first.filter(|edge| edge.specified) {
        return first.stroke.as_ref();
    }
    if let Some(second) = second.filter(|edge| edge.specified) {
        return second.stroke.as_ref();
    }
    first
        .and_then(|edge| edge.stroke.as_ref())
        .or_else(|| second.and_then(|edge| edge.stroke.as_ref()))
}
fn stroke(attributes: &Properties) -> Option<PathStroke> {
    if text(attributes, "style") == "none" {
        return None;
    }
    let width = units(attributes.get("width"))
        .map(|value| value as f64 / 12700.0)
        .unwrap_or(1.0);
    if width <= 0.0 {
        return None;
    }
    let alpha = number(attributes, "opacity")
        .map(|value| {
            let percentage = if value <= 1.0 { value * 100.0 } else { value };
            (percentage.clamp(0.0, 100.0) * 255.0 / 100.0).round() as u8
        })
        .unwrap_or(255);
    let mut cap = match text(attributes, "cap").as_str() {
        "round" => LineCap::Round,
        "square" | "sq" => LineCap::Square,
        _ => LineCap::Flat,
    };
    let dash = match text(attributes, "style").as_str() {
        "dashed" => vec![3.0 * width, 3.0 * width],
        "dotted" => {
            cap = LineCap::Round;
            vec![(width / 100.0).max(0.01), 4.0 * width]
        }
        _ => Vec::new(),
    };
    Some(PathStroke {
        paint: PathPaint {
            color: color(attributes.get("color")).unwrap_or_else(|| "FFFFFF".into()),
            alpha,
        },
        width,
        dash,
        cap,
    })
}
fn append(
    paths: &mut Vec<PaintedPath>,
    start: [f64; 2],
    end: [f64; 2],
    stroke: Option<&PathStroke>,
) {
    let Some(stroke) =
        stroke.filter(|stroke| stroke.paint.alpha > 0 && stroke.width > 0.0 && start != end)
    else {
        return;
    };
    if let Some(last) = paths.last_mut()
        && last.points[1] == start
        && ((last.points[0][0] == start[0] && start[0] == end[0])
            || (last.points[0][1] == start[1] && start[1] == end[1]))
        && last.stroke.as_ref() == Some(stroke)
    {
        last.points[1] = end;
        return;
    }
    paths.push(PaintedPath {
        points: vec![start, end],
        closed: false,
        fill: None,
        stroke: Some(stroke.clone()),
    });
}
