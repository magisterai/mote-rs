use super::{LayoutError, style_values};
use crate::model::{Document, Properties};

/// Physical page and equal-column geometry, expressed in points.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PageGeometry {
    pub width: f64,
    pub height: f64,
    pub margin_top: f64,
    pub margin_right: f64,
    pub margin_bottom: f64,
    pub margin_left: f64,
    pub header_space: f64,
    pub footer_space: f64,
    pub column_count: usize,
    pub column_gap: f64,
    pub column_separator: bool,
}
impl Default for PageGeometry {
    fn default() -> Self {
        Self {
            width: 612.0,
            height: 792.0,
            margin_top: 72.0,
            margin_right: 72.0,
            margin_bottom: 72.0,
            margin_left: 72.0,
            header_space: 36.0,
            footer_space: 36.0,
            column_count: 1,
            column_gap: 18.0,
            column_separator: false,
        }
    }
}
impl PageGeometry {
    pub fn from_document(document: &Document) -> Result<Self, LayoutError> {
        let (width, height) = match style_values::text(&document.page, "size")
            .to_lowercase()
            .as_str()
        {
            "legal" | "us_legal" => (612.0, 1008.0),
            "tabloid" | "ledger" => (792.0, 1224.0),
            "a3" => (841.9, 1190.55),
            "a4" => (595.3, 841.9),
            "a5" => (419.55, 595.3),
            _ => (612.0, 792.0),
        };
        let mut geometry = Self {
            width: distance(&document.page, "width", width),
            height: distance(&document.page, "height", height),
            ..Default::default()
        };
        if style_values::text(&document.page, "orientation") == "landscape"
            && geometry.height > geometry.width
        {
            std::mem::swap(&mut geometry.width, &mut geometry.height);
        }
        geometry.header_space = distance(&document.margin, "header", 36.0);
        geometry.footer_space = distance(&document.margin, "footer", 36.0);
        geometry.margin_top = distance(&document.margin, "top", 72.0).max(geometry.header_space);
        geometry.margin_bottom =
            distance(&document.margin, "bottom", 72.0).max(geometry.footer_space);
        geometry.margin_left = distance(&document.margin, "left", 72.0);
        geometry.margin_right = distance(&document.margin, "right", 72.0);
        geometry.column_count = document.columns.count.unwrap_or(1).max(1) as usize;
        geometry.column_gap = distance(&document.columns.properties, "gap", 18.0);
        geometry.column_separator =
            style_values::boolean(&document.columns.properties, "separator");
        geometry.validate()?;
        Ok(geometry)
    }
    pub fn content_width(&self) -> f64 {
        self.width - self.margin_left - self.margin_right
    }
    pub fn content_bottom(&self) -> f64 {
        self.height - self.margin_bottom
    }
    pub fn column_width(&self) -> f64 {
        let width = (self.content_width()
            - (self.column_count.saturating_sub(1)) as f64 * self.column_gap)
            / self.column_count.max(1) as f64;
        if width < 1.0 {
            self.content_width()
        } else {
            width
        }
    }
    pub fn column_left(&self, index: usize) -> f64 {
        self.margin_left + index as f64 * (self.column_width() + self.column_gap)
    }
    pub fn validate(&self) -> Result<(), LayoutError> {
        if [
            self.width,
            self.height,
            self.margin_top,
            self.margin_right,
            self.margin_bottom,
            self.margin_left,
            self.header_space,
            self.footer_space,
            self.column_gap,
        ]
        .iter()
        .any(|value| !value.is_finite() || *value < 0.0)
            || self.column_count == 0
            || self.content_width() <= 0.0
            || self.content_bottom() <= self.margin_top
        {
            return Err(LayoutError::InvalidGeometry(
                "page geometry must have finite positive content dimensions",
            ));
        }
        Ok(())
    }
}
fn distance(properties: &Properties, name: &str, fallback: f64) -> f64 {
    properties
        .get(name)
        .and_then(style_values::points)
        .unwrap_or(fallback)
}
