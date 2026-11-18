use super::{
    LayoutError, LineFrame, PageGeometry,
    tolerances::{AREA_START_TOLERANCE, FIT_TOLERANCE},
    widows,
};

#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub struct FlowCursor {
    pub page_number: usize,
    pub column_index: usize,
    pub current_top: f64,
    pub y: f64,
    pub body_started: bool,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FlowAdvance {
    AddPage,
    NextColumn,
    NextPage,
}
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct FlowArea {
    pub geometry: PageGeometry,
    pub content_top: f64,
    pub content_bottom: f64,
    pub page_start: f64,
    /// Holds whether the area must hold the trailing space of its last line,
    /// as the height of balanced columns must.
    pub whole_lines: bool,
}
impl From<PageGeometry> for FlowArea {
    fn from(geometry: PageGeometry) -> Self {
        Self {
            content_top: geometry.margin_top,
            content_bottom: geometry.content_bottom(),
            page_start: geometry.margin_top,
            whole_lines: false,
            geometry,
        }
    }
}
#[derive(Debug, Clone, Default, PartialEq)]
pub struct FlowOptions {
    pub leading_space: f64,
    pub minimum_start_space: f64,
    pub keep_together: bool,
    pub widow_control: bool,
    /// Holds, for each line, the space at its foot that an area need not hold.
    /// An empty vector states that every line must fit whole.
    pub trailing_spaces: Vec<f64>,
}
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct LinePlacement {
    pub line_index: usize,
    pub page_number: usize,
    pub column_index: usize,
    pub frame: LineFrame,
    pub top: f64,
    pub height: f64,
}
#[derive(Debug, Clone, PartialEq)]
pub enum FlowStep {
    Advance(FlowAdvance),
    Line(LinePlacement),
}
#[derive(Debug, Clone, PartialEq)]
pub struct ParagraphFlow {
    pub steps: Vec<FlowStep>,
    pub final_cursor: FlowCursor,
}

/// Place measured lines through finite page/column areas without remeasuring text.
pub fn plan_paragraph_flow(
    cursor: FlowCursor,
    heights: &[f64],
    frames: &[LineFrame],
    options: &FlowOptions,
    area_for_page: impl Fn(usize) -> FlowArea,
) -> Result<ParagraphFlow, LayoutError> {
    if heights.len() != frames.len()
        || (!options.trailing_spaces.is_empty() && options.trailing_spaces.len() != heights.len())
        || options
            .trailing_spaces
            .iter()
            .zip(heights)
            .any(|(space, height)| !(0.0..=*height).contains(space))
        || heights
            .iter()
            .any(|height| !height.is_finite() || *height < 0.0)
        || frames
            .iter()
            .any(|frame| !frame.x.is_finite() || !frame.width.is_finite())
        || [
            cursor.current_top,
            cursor.y,
            options.leading_space,
            options.minimum_start_space,
        ]
        .iter()
        .any(|value| !value.is_finite())
    {
        return Err(LayoutError::InvalidGeometry(
            "flow requires matching finite line heights, frames and spacing",
        ));
    }
    let no_trailing_spaces = vec![0.0; heights.len()];
    let trailing_spaces = |area: &FlowArea| {
        if area.whole_lines || options.trailing_spaces.is_empty() {
            no_trailing_spaces.as_slice()
        } else {
            options.trailing_spaces.as_slice()
        }
    };
    let mut planner = FlowPlanner {
        cursor,
        steps: Vec::new(),
        area_for_page,
    };
    if cursor.page_number == 0 {
        planner.advance(FlowAdvance::AddPage)?;
    }
    let initial = planner.cursor;
    let mut at_start = planner.at_start()?;
    if !heights.is_empty() {
        if options.leading_space > 0.0 {
            let previous = planner.cursor;
            let trailing_space = trailing_spaces(&planner.area()?)[0];
            planner.ensure_space(options.leading_space + heights[0] - trailing_space, false)?;
            planner.cursor.add_leading(&previous, options.leading_space);
        }
        if options.minimum_start_space > 0.0 {
            planner.ensure_fresh(options.minimum_start_space)?;
        }
        let area = planner.area()?;
        let full_height: f64 =
            heights.iter().sum::<f64>() - trailing_spaces(&area)[heights.len() - 1];
        if !full_height.is_finite() {
            return Err(LayoutError::InvalidGeometry("paragraph height overflowed"));
        }
        if options.keep_together
            && options.leading_space + full_height
                <= area.content_bottom - area.content_top + FIT_TOLERANCE
        {
            planner.ensure_fresh(full_height)?;
        }
        if initial.page_number != planner.cursor.page_number
            || initial.column_index != planner.cursor.column_index
        {
            at_start = planner.at_start()?;
        }
        let mut index = 0;
        let mut empty_advances = 0;
        while index < heights.len() {
            let area = planner.area()?;
            let count = if options.widow_control {
                widows::line_count(
                    &heights[index..],
                    &trailing_spaces(&area)[index..],
                    area.content_bottom - planner.cursor.y,
                    area.content_bottom - area.content_top,
                    at_start,
                )
            } else {
                1
            };
            if count == 0 {
                empty_advances += 1;
                if empty_advances > area.geometry.column_count.saturating_add(1) {
                    return Err(LayoutError::InvalidGeometry(
                        "paragraph cannot make progress in successive flow areas",
                    ));
                }
                planner.advance_next()?;
                at_start = planner.at_start()?;
                continue;
            }
            empty_advances = 0;
            let end = (index + count).min(heights.len());
            while index < end {
                if !options.widow_control {
                    let area = planner.area()?;
                    planner.ensure_space(heights[index] - trailing_spaces(&area)[index], false)?;
                }
                let area = planner.area()?;
                let mut frame = frames[index];
                frame.x += area.geometry.column_left(planner.cursor.column_index);
                if !frame.x.is_finite() || !(planner.cursor.y + heights[index]).is_finite() {
                    return Err(LayoutError::InvalidGeometry(
                        "placed paragraph geometry overflowed",
                    ));
                }
                planner.steps.push(FlowStep::Line(LinePlacement {
                    line_index: index,
                    page_number: planner.cursor.page_number,
                    column_index: planner.cursor.column_index,
                    frame,
                    top: planner.cursor.y,
                    height: heights[index],
                }));
                planner.cursor.y += heights[index];
                index += 1;
            }
            if options.widow_control && index < heights.len() {
                planner.advance_next()?;
                at_start = planner.at_start()?;
            }
        }
        planner.cursor.body_started = true;
    }
    Ok(ParagraphFlow {
        steps: planner.steps,
        final_cursor: planner.cursor,
    })
}
struct FlowPlanner<Resolver> {
    cursor: FlowCursor,
    steps: Vec<FlowStep>,
    area_for_page: Resolver,
}
impl<Resolver: Fn(usize) -> FlowArea> FlowPlanner<Resolver> {
    fn area(&self) -> Result<FlowArea, LayoutError> {
        let area = (self.area_for_page)(self.cursor.page_number);
        area.geometry.validate()?;
        if self.cursor.column_index >= area.geometry.column_count {
            return Err(LayoutError::InvalidGeometry(
                "flow cursor column is outside the page",
            ));
        }
        if [area.content_top, area.content_bottom, area.page_start]
            .iter()
            .any(|value| !value.is_finite())
            || area.content_bottom <= area.content_top
            || area.page_start < area.content_top
            || area.page_start >= area.content_bottom
        {
            return Err(LayoutError::InvalidGeometry(
                "flow area must have usable finite bounds",
            ));
        }
        Ok(area)
    }
    fn advance(&mut self, advance: FlowAdvance) -> Result<(), LayoutError> {
        self.steps.push(FlowStep::Advance(advance));
        match advance {
            FlowAdvance::NextColumn => {
                self.cursor.column_index += 1;
                self.cursor.y = self.cursor.current_top;
            }
            FlowAdvance::AddPage | FlowAdvance::NextPage => {
                self.cursor.page_number = self
                    .cursor
                    .page_number
                    .checked_add(1)
                    .ok_or(LayoutError::InvalidGeometry("page count overflow"))?;
                self.cursor.column_index = 0;
                let area = self.area()?;
                self.cursor.current_top = area.content_top;
                self.cursor.y = area.page_start;
                self.cursor.body_started = false;
            }
        }
        Ok(())
    }
    fn advance_next(&mut self) -> Result<(), LayoutError> {
        let area = self.area()?;
        self.advance(
            if self.cursor.column_index + 1 < area.geometry.column_count {
                FlowAdvance::NextColumn
            } else {
                FlowAdvance::NextPage
            },
        )
    }
    fn at_start(&self) -> Result<bool, LayoutError> {
        let area = self.area()?;
        Ok(
            self.cursor.current_top <= area.content_top + AREA_START_TOLERANCE
                && self.cursor.y <= area.content_top + AREA_START_TOLERANCE,
        )
    }
    fn ensure_space(&mut self, height: f64, fresh: bool) -> Result<bool, LayoutError> {
        let area = self.area()?;
        if (fresh && height > area.content_bottom - area.content_top + FIT_TOLERANCE)
            || self.cursor.y + height <= area.content_bottom + FIT_TOLERANCE
        {
            return Ok(false);
        }
        self.advance_next()?;
        Ok(true)
    }
    fn ensure_fresh(&mut self, height: f64) -> Result<(), LayoutError> {
        // A caller-supplied area may vary by page; reject a non-progressing series.
        let limit = self.area()?.geometry.column_count.saturating_add(2);
        for _ in 0..limit {
            if !self.ensure_space(height, true)? {
                return Ok(());
            }
        }
        Err(LayoutError::InvalidGeometry(
            "fresh flow areas do not provide space for the requested block",
        ))
    }
}
