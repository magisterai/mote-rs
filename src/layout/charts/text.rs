use super::{ChartPlanner, LayoutError, PageOperation, geometry::Rectangle};
use crate::layout::{LineSpacing, ParagraphStyle, PlacedTextLine, TextRun, TextStyle};

pub(super) struct ChartMeasurement<'measurement, 'assets> {
    font: String,
    color: String,
    variant: crate::measurement::FontVariant,
    measurer: &'measurement mut crate::measurement::TextMeasurer<'assets>,
    /// Records whether the last fitted text was reduced below its stated size.
    pub(super) reduced: bool,
}

impl<'measurement, 'assets> ChartMeasurement<'measurement, 'assets> {
    pub(super) fn new(
        font: &str,
        color: &str,
        measurer: &'measurement mut crate::measurement::TextMeasurer<'assets>,
    ) -> Self {
        Self {
            font: font.into(),
            color: color.into(),
            variant: Default::default(),
            measurer,
            reduced: false,
        }
    }

    fn text_style(&self, size: f64) -> TextStyle {
        let mut style = TextStyle::default();
        style.measurement.font = self.font.clone();
        style.measurement.variant = self.variant;
        style.color = self.color.clone();
        style.measurement.size_points = size;
        style
    }
    pub(super) fn text_width(&mut self, text: &str, size: f64) -> Result<f64, LayoutError> {
        let measured = self
            .measurer
            .measure(&self.text_style(size).measurement, text, false)?;
        Ok(measured
            .measurement
            .layout_width
            .max(measured.measurement.paint_width))
    }
    pub(super) fn text_plan(
        &mut self,
        text: &str,
        size: f64,
        alignment: &str,
        width: f64,
    ) -> Result<crate::layout::ParagraphText, LayoutError> {
        let text_style = self.text_style(size);
        let style = ParagraphStyle {
            text: text_style.clone(),
            alignment: alignment.into(),
            spacing_before: 0.0,
            spacing_after: 0.0,
            line_spacing: LineSpacing::Multiple(1.0),
            line_multiplier: 1.0,
            ..Default::default()
        };
        super::super::paragraphs::plan_resolved_paragraph(
            style,
            Default::default(),
            &[TextRun::new(text, text_style)],
            width,
            self.measurer,
            false,
            false,
        )
    }
    pub(super) fn fit_text(
        &mut self,
        text: &str,
        size: f64,
        alignment: &str,
        bounds: Rectangle,
    ) -> Result<crate::layout::ParagraphText, LayoutError> {
        let plan = self.text_plan(text, size, alignment, bounds.width)?;
        self.reduced = !text_fits(&plan, bounds);
        if !self.reduced {
            return Ok(plan);
        }
        let (mut lower, mut upper) = (0.0, size);
        let mut best = self.text_plan(
            text,
            (size / 65536.0).max(0.000001),
            alignment,
            bounds.width,
        )?;
        for _ in 0..16 {
            let candidate_size = (lower + upper) / 2.0;
            let candidate = self.text_plan(text, candidate_size, alignment, bounds.width)?;
            if text_fits(&candidate, bounds) {
                lower = candidate_size;
                best = candidate;
            } else {
                upper = candidate_size;
            }
        }
        Ok(best)
    }
}

impl ChartPlanner<'_, '_> {
    pub(super) fn text(
        &mut self,
        bounds: Rectangle,
        text: &str,
        size: f64,
        alignment: &str,
        rotation: f64,
    ) -> Result<(), LayoutError> {
        self.measurement.reduced = false;
        if text.trim().is_empty() || bounds.width <= 0.0 || bounds.height <= 0.0 {
            return Ok(());
        }
        let paragraph = self.measurement.fit_text(text, size, alignment, bounds)?;
        let height = text_height(&paragraph);
        let mut top = bounds.top + ((bounds.height - height) / 2.0).max(0.0);
        for mut line in paragraph.lines {
            let height = line.height;
            line.translate(bounds.left, top);
            if rotation != 0.0 {
                self.page.operations.push(PageOperation::PushRotation {
                    degrees: rotation,
                    center: bounds.center(),
                });
            }
            self.page
                .operations
                .push(PageOperation::PushClip(bounds.bounds()));
            self.page.push_line(PlacedTextLine { top, line });
            self.page.operations.push(PageOperation::PopClip);
            if rotation != 0.0 {
                self.page.operations.push(PageOperation::PopRotation);
            }
            top += height;
        }
        Ok(())
    }
}

impl ChartPlanner<'_, '_> {
    pub(super) fn axis_label(
        &mut self,
        kind: LabelKind,
        bounds: Rectangle,
        text: &str,
        size: f64,
        alignment: &str,
    ) -> Result<(), LayoutError> {
        let previous =
            std::mem::replace(&mut self.measurement.color, self.axis_label_color.clone());
        let result = self.label(kind, bounds, text, size, alignment);
        self.measurement.color = previous;
        result
    }
    /// Draws a label and records its kind if the label was reduced to fit.
    pub(super) fn label(
        &mut self,
        kind: LabelKind,
        bounds: Rectangle,
        text: &str,
        size: f64,
        alignment: &str,
    ) -> Result<(), LayoutError> {
        self.text(bounds, text, size, alignment, 0.0)?;
        if self.measurement.reduced {
            self.reduced_labels.insert(kind);
        }
        Ok(())
    }
    pub(super) fn title(
        &mut self,
        bounds: Rectangle,
        chart: &crate::charts::Chart,
    ) -> Result<(), LayoutError> {
        use crate::measurement::FontVariant;
        let style = &chart.title_style;
        let variant = match (style.bold.unwrap_or(false), style.italic.unwrap_or(false)) {
            (true, true) => FontVariant::BoldItalic,
            (true, false) => FontVariant::Bold,
            (false, true) => FontVariant::Italic,
            (false, false) => FontVariant::Regular,
        };
        let font = style
            .font
            .clone()
            .unwrap_or_else(|| self.measurement.font.clone());
        let color = style
            .color
            .clone()
            .unwrap_or_else(|| self.measurement.color.clone());
        let previous = (
            std::mem::replace(&mut self.measurement.font, font),
            std::mem::replace(&mut self.measurement.color, color),
            std::mem::replace(&mut self.measurement.variant, variant),
        );
        let result = self.text(
            bounds,
            &chart.title,
            super::geometry::title_size(chart),
            "center",
            0.0,
        );
        (
            self.measurement.font,
            self.measurement.color,
            self.measurement.variant,
        ) = previous;
        result
    }
}

pub(super) fn text_height(plan: &crate::layout::ParagraphText) -> f64 {
    plan.lines.iter().map(|line| line.height).sum()
}

fn text_fits(plan: &crate::layout::ParagraphText, bounds: Rectangle) -> bool {
    text_height(plan) <= bounds.height + 1e-9
        && plan
            .lines
            .iter()
            .all(|line| line.width <= bounds.width + 1e-9)
}

/// The labels of a chart that take its label size, in the order of a warning.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) enum LabelKind {
    Category,
    ValueAxis,
    Legend,
    Data,
}

/// The warning for labels drawn below the label size of their chart.
pub(crate) fn reduced_labels_warning(
    kinds: &std::collections::BTreeSet<LabelKind>,
    described: &str,
) -> Option<String> {
    let names: Vec<_> = kinds
        .iter()
        .map(|kind| match kind {
            LabelKind::Category => "category labels",
            LabelKind::ValueAxis => "value axis labels",
            LabelKind::Legend => "legend entries",
            LabelKind::Data => "data labels",
        })
        .collect();
    let list = match names.as_slice() {
        [] => return None,
        [only] => (*only).to_owned(),
        [first @ .., last] => format!("{} and {last}", first.join(", ")),
    };
    Some(format!(
        "the {list} of {described} are reduced below the chart's text size to fit; shorten the labels, widen the chart or reduce text.size"
    ))
}
