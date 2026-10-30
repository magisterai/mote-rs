use super::{LayoutError, PageOperation, Planner, geometry::Rectangle};
use crate::layout::{LineSpacing, ParagraphStyle, PlacedTextLine, TextRun, TextStyle};

impl Planner<'_, '_> {
    fn text_style(&self, size: f64) -> TextStyle {
        let mut style = TextStyle::default();
        style.measurement.font = "Aptos".into();
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
    pub(super) fn text(
        &mut self,
        bounds: Rectangle,
        text: &str,
        size: f64,
        alignment: &str,
        rotation: f64,
    ) -> Result<(), LayoutError> {
        if text.trim().is_empty() || bounds.width <= 0.0 || bounds.height <= 0.0 {
            return Ok(());
        }
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
        let paragraph = super::super::paragraphs::plan_resolved_paragraph(
            style,
            None,
            &[TextRun::new(text, text_style)],
            bounds.width,
            self.measurer,
            false,
        )?;
        let height: f64 = paragraph.lines.iter().map(|line| line.height).sum();
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
