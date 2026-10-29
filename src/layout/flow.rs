use super::{LayoutError, LineFrame, PageGeometry, widows};

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
    pub bottom_slack: f64,
}
impl From<PageGeometry> for FlowArea {
    fn from(geometry: PageGeometry) -> Self {
        Self {
            content_top: geometry.margin_top,
            content_bottom: geometry.content_bottom(),
            page_start: geometry.margin_top,
            bottom_slack: 0.0,
            geometry,
        }
    }
}
#[derive(Debug, Clone, Default, PartialEq)]
pub struct FlowOptions {
    pub leading_space: f64,
    pub leading_fit_slack: f64,
    pub keep_fit_leading_space: f64,
    pub minimum_start_space: f64,
    pub keep_together: bool,
    pub widow_control: bool,
    pub fresh_fit_tolerance: f64,
    pub keep_fit_tolerance: f64,
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
            options.leading_fit_slack,
            options.keep_fit_leading_space,
            options.minimum_start_space,
            options.fresh_fit_tolerance,
            options.keep_fit_tolerance,
        ]
        .iter()
        .any(|value| !value.is_finite())
    {
        return Err(LayoutError::InvalidGeometry(
            "flow requires matching finite line heights, frames and spacing",
        ));
    }
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
            planner.ensure_space(
                options.leading_space + heights[0],
                options.leading_fit_slack,
                0.0,
                false,
            )?;
            if planner.cursor.page_number == previous.page_number
                && planner.cursor.column_index == previous.column_index
            {
                planner.cursor.y += options.leading_space;
            }
        }
        if options.minimum_start_space > 0.0 {
            planner.ensure_fresh(options.minimum_start_space, options.fresh_fit_tolerance)?;
        }
        let full_height: f64 = heights.iter().sum();
        if !full_height.is_finite() {
            return Err(LayoutError::InvalidGeometry("paragraph height overflowed"));
        }
        let area = planner.area()?;
        if options.keep_together
            && options.keep_fit_leading_space + full_height
                <= area.content_bottom - area.content_top
                    + area.bottom_slack
                    + options.keep_fit_tolerance
        {
            planner.ensure_fresh(full_height, options.fresh_fit_tolerance)?;
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
                    area.content_bottom + area.bottom_slack - planner.cursor.y,
                    area.content_bottom + area.bottom_slack - area.content_top,
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
                    planner.ensure_space(heights[index], area.bottom_slack, 0.0, false)?;
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
        if [
            area.content_top,
            area.content_bottom,
            area.page_start,
            area.bottom_slack,
        ]
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
        Ok(self.cursor.current_top <= area.content_top + 0.001
            && self.cursor.y <= area.content_top + 0.001)
    }
    fn ensure_space(
        &mut self,
        height: f64,
        slack: f64,
        tolerance: f64,
        fresh: bool,
    ) -> Result<bool, LayoutError> {
        let area = self.area()?;
        if (fresh && height > area.content_bottom - area.content_top + slack + tolerance)
            || self.cursor.y + height <= area.content_bottom + slack + tolerance
        {
            return Ok(false);
        }
        self.advance_next()?;
        Ok(true)
    }
    fn ensure_fresh(&mut self, height: f64, tolerance: f64) -> Result<(), LayoutError> {
        // A caller-supplied area may vary by page; reject a non-progressing series.
        let limit = self.area()?.geometry.column_count.saturating_add(2);
        for _ in 0..limit {
            let area = self.area()?;
            if !self.ensure_space(height, area.bottom_slack, tolerance, true)? {
                return Ok(());
            }
        }
        Err(LayoutError::InvalidGeometry(
            "fresh flow areas do not provide space for the requested block",
        ))
    }
}
