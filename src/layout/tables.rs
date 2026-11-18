//! Table measurement, cell content and physical edges shared by both native painters.
mod borders;
mod cells;
mod columns;
pub(super) mod flow;
mod pagination;
use super::{LayoutError, LayoutStyleResolver, blocks::Block, spacing::collapsed_spacing};
use crate::{
    measurement::TextMeasurer,
    media::ImageCatalog,
    model::{Element, Properties},
    properties as property_values, tables,
};

pub(super) struct TablePlan {
    pub anchors: Vec<String>,
    pub rows: Vec<RowPlan>,
    pub widths: Vec<f64>,
    pub alignment: String,
}
pub(super) struct RowPlan {
    pub cells: Vec<CellPlan>,
    pub height: f64,
    /// The authored height, and whether its rule is exact rather than a
    /// minimum. An automatic rule leaves it absent.
    pub authored_height: Option<(f64, bool)>,
    pub repeat_header: bool,
    pub cant_split: bool,
    /// Whether an exact height cuts off the content of a cell of the row.
    pub cut_off: bool,
}
pub(super) struct CellPlan {
    pub column: usize,
    pub column_span: usize,
    pub row_span: usize,
    pub attributes: Properties,
    /// Top, right, bottom, left in points.
    pub margins: [f64; 4],
    pub blocks: Vec<Block>,
    pub block_frames: Vec<super::LineFrame>,
    pub content_tops: Vec<f64>,
    pub content_height: f64,
    borders: [borders::Candidate; 4],
}
impl TablePlan {
    pub fn width(&self) -> f64 {
        self.widths.iter().sum()
    }
    pub fn height(&self) -> f64 {
        self.rows.iter().map(|row| row.height).sum()
    }
    /// Returns the width of the table's top border, which Word sets above the
    /// first row.
    pub fn top_border(&self) -> f64 {
        self.row_top_border(0)
    }
    /// Returns the number of rows that the area in which the table begins must
    /// hold: Word keeps the header rows of a table on the page of the row
    /// that follows them, as its export of the table-header-rows probe shows.
    pub fn opening_rows(&self) -> usize {
        let headers = self.rows.iter().take_while(|row| row.repeat_header).count();
        if headers < self.rows.len() {
            headers + 1
        } else {
            1
        }
    }
    /// Returns the height of the rows that the area in which the table begins
    /// must hold.
    pub fn opening_height(&self) -> f64 {
        self.rows
            .iter()
            .take(self.opening_rows())
            .map(|row| row.height)
            .sum()
    }
    /// Word draws the table's bottom border beneath the last row on every page,
    /// in place of that row's own bottom border. The function returns the room
    /// the row needs at the foot of a page beyond its own height: the excess of
    /// the table's bottom border over the row's.
    pub fn closing_border_excess(&self, row: usize) -> f64 {
        let bottom_border = |row: Option<&RowPlan>| {
            row.map_or(0.0, |row| {
                row.cells
                    .iter()
                    .map(|cell| cell.borders[2].width)
                    .fold(0.0, f64::max)
            })
        };
        (bottom_border(self.rows.last()) - bottom_border(self.rows.get(row))).max(0.0)
    }
    /// Returns the width of the top border of a row, which Word sets above the
    /// row that begins a page.
    pub fn row_top_border(&self, row: usize) -> f64 {
        self.rows.get(row).map_or(0.0, |row| {
            row.cells
                .iter()
                .map(|cell| cell.borders[0].width)
                .fold(0.0, f64::max)
        })
    }
    /// Returns the room the table takes in a cell: its grid and the halves of
    /// its left and right borders that lie outside the grid, which Word adds
    /// to the column that holds a nested table.
    pub fn outer_width(&self) -> f64 {
        let side = |edge: fn(&RowPlan) -> Option<&CellPlan>, border: usize| {
            self.rows
                .iter()
                .filter_map(edge)
                .map(|cell| cell.borders[border].width)
                .fold(0.0, f64::max)
                / 2.0
        };
        self.width() + side(|row| row.cells.first(), 3) + side(|row| row.cells.last(), 1)
    }
    /// Word sets the outer edge of a left-aligned table's left border on the
    /// margin, so the table starts half that border's width inside it.
    pub fn left(&self, left: f64, available: f64) -> f64 {
        left + match self.alignment.as_str() {
            "center" => (available - self.width()) / 2.0,
            "right" => available - self.width(),
            _ => {
                self.rows
                    .iter()
                    .filter_map(|row| row.cells.first())
                    .map(|cell| cell.borders[3].width)
                    .fold(0.0, f64::max)
                    / 2.0
            }
        }
    }
}
/// Word sizes an automatic column to its content, in whole twips.
fn whole_twips(width: f64) -> f64 {
    (width * 20.0 - 1e-6).ceil() / 20.0
}
pub(super) fn prepare(
    table: &Element,
    available: f64,
    styles: &LayoutStyleResolver<'_>,
    measurer: &mut TextMeasurer<'_>,
    images: &ImageCatalog,
    numbers: &mut super::blocks::BlockNumbers,
) -> Result<TablePlan, LayoutError> {
    let grid = tables::grid(table).map_err(LayoutError::Unsupported)?;
    let defaults = cells::defaults(styles.document, table);
    let source_rows = tables::rows(table);
    let mut plan = TablePlan {
        anchors: Vec::new(),
        rows: Vec::new(),
        widths: Vec::new(),
        alignment: property_values::text(&table.attributes, "alignment"),
    };
    if grid.column_count == 0 {
        return Ok(plan);
    }
    let default_margins = cells::margins(&defaults);
    let default_style = styles.paragraph(&Element {
        source_name: None,
        source_location: None,
        kind: crate::model::ElementKind::Paragraph,
        attributes: defaults.clone(),
        children: Vec::new(),
    });
    let default_width = (default_style.text.measurement.size_points * 2.0
        + default_margins[1]
        + default_margins[3])
        .ceil();
    let mut constraints = Vec::new();
    let mut contents = Vec::new();
    for (row_index, row) in grid.rows.iter().enumerate() {
        let mut row_plan = RowPlan {
            cells: Vec::new(),
            height: 0.0,
            repeat_header: property_values::boolean(
                &source_rows[row_index].attributes,
                "repeat_header",
            ),
            cant_split: property_values::boolean(&source_rows[row_index].attributes, "cant_split"),
            cut_off: false,
            authored_height: {
                let attributes = &source_rows[row_index].attributes;
                let rule = property_values::text(attributes, "height_rule");
                cells::points(attributes.get("height"))
                    .filter(|_| rule != "auto")
                    .map(|height| (height, rule == "exact"))
            },
        };
        for (column, slot) in row.iter().enumerate() {
            let Some(cell) = slot
                .as_ref()
                .filter(|cell| !cell.continuation && cell.origin_column == column)
            else {
                continue;
            };
            let attributes = cells::attributes(
                styles.document,
                table,
                &cell.element,
                &defaults,
                row_index,
                grid.rows.len(),
            );
            let margins = cells::margins(&attributes);
            let mut content = cells::content(&cell.element.children, &attributes);
            cells::inherit_image_alignment(
                &mut content,
                styles.document,
                table,
                row_index,
                grid.rows.len(),
            );
            let intrinsic = columns::intrinsic(
                &content,
                &attributes,
                available,
                styles,
                measurer,
                images,
                numbers,
            )?;
            constraints.push(columns::Constraint {
                column,
                span: cell.column_span,
                explicit: cells::points(attributes.get("width")).unwrap_or(0.0),
                preferred: whole_twips(intrinsic.0 + margins[1] + margins[3]),
                minimum: whole_twips(intrinsic.1 + margins[1] + margins[3]),
            });
            let borders = borders::resolve(
                styles.document,
                table,
                &attributes,
                [
                    row_index,
                    column,
                    cell.row_span,
                    cell.column_span,
                    grid.rows.len(),
                    grid.column_count,
                ],
            );
            row_plan.cells.push(CellPlan {
                column,
                column_span: cell.column_span,
                row_span: cell.row_span.min(grid.rows.len() - row_index),
                attributes,
                margins,
                blocks: Vec::new(),
                block_frames: Vec::new(),
                content_tops: Vec::new(),
                content_height: 0.0,
                borders,
            });
            contents.push(content);
        }
        plan.rows.push(row_plan);
    }
    plan.widths = columns::resolve(
        grid.column_count,
        available,
        cells::points(table.attributes.get("width")).unwrap_or(0.0),
        default_width,
        &constraints,
    );
    let mut contents = contents.into_iter();
    let mut spans = Vec::new();
    for (row_index, row) in plan.rows.iter_mut().enumerate() {
        for cell in &mut row.cells {
            let width = plan
                .widths
                .iter()
                .skip(cell.column)
                .take(cell.column_span)
                .sum::<f64>()
                - cell.margins[1]
                - cell.margins[3];
            for node in contents.next().unwrap() {
                let frame = cells::object_frame(&node, &cell.attributes, width.max(1.0), styles);
                let blocks = super::blocks::prepare_content(
                    std::slice::from_ref(&node),
                    frame.width,
                    styles,
                    measurer,
                    images,
                    numbers,
                    true,
                )?
                .blocks;
                cell.block_frames
                    .extend(std::iter::repeat_n(frame, blocks.len()));
                cell.blocks.extend(blocks);
            }
            super::blocks::join_identical_borders(&mut cell.blocks);
            let mut previous_after: f64 = 0.0;
            for block in &cell.blocks {
                let (height, before, after) = cells::block_geometry(block);
                cell.content_height += collapsed_spacing(previous_after, before);
                // Word sets the top border of a nested table above its first row,
                // as it does for a table in the body.
                if let Block::Table(nested) = block {
                    cell.content_height += nested.top_border();
                }
                cell.content_tops.push(cell.content_height);
                cell.content_height += height;
                previous_after = after;
            }
            cell.content_height += previous_after;
            let height = (cell.content_height + cell.margins[0] + cell.margins[2])
                .max(cells::points(cell.attributes.get("height")).unwrap_or(0.0));
            if cell.row_span > 1 {
                spans.push((row_index, cell.row_span, height));
            } else {
                row.height = row.height.max(height);
            }
        }
        if row.height == 0.0 {
            row.height = default_style.line_height + default_margins[0] + default_margins[2];
        }
        match row.authored_height {
            // Word sets an exact height above the cells' bottom margin and
            // draws the row's border within it.
            Some((height, true)) => {
                row.height = height
                    + row
                        .cells
                        .iter()
                        .map(|cell| cell.margins[2])
                        .fold(0.0, f64::max);
                row.cut_off = row.cells.iter().any(|cell| {
                    cell.row_span == 1
                        && cell.margins[0] + cells::visible_height(cell)
                            > row.height + cells::CUT_OFF_TOLERANCE
                });
            }
            Some((height, false)) => {
                // Word holds a minimum height for the content within the
                // cells' top and bottom margins.
                let margins = row
                    .cells
                    .iter()
                    .map(|cell| cell.margins[0] + cell.margins[2])
                    .fold(default_margins[0] + default_margins[2], f64::max);
                row.height = row.height.max(height + margins);
            }
            None => {}
        }
    }
    for (start, span, required) in spans {
        let height: f64 = plan.rows[start..start + span]
            .iter()
            .map(|row| row.height)
            .sum();
        if required > height {
            plan.rows[start + span - 1].height += required - height;
        }
    }
    borders::add_row_heights(&mut plan, &grid);
    Ok(plan)
}
