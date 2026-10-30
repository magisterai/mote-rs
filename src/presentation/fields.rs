//! Resolved slide metadata and fixed footer-field placement.
use crate::model::{Content, Document, Element, ElementKind, Properties, Unit, Value};

pub(crate) struct SlideMetadata {
    footer: String,
    date: String,
    automatic_date: bool,
    slide_number: bool,
    pub notes: String,
}
#[derive(Clone, Copy)]
pub(crate) enum FieldKind {
    Footer,
    Date { automatic: bool },
    SlideNumber,
}
pub(crate) struct Field {
    pub kind: FieldKind,
    pub text: String,
}
fn effective<'value>(
    document: &'value Document,
    slide: &'value Element,
    name: &str,
) -> Option<&'value Value> {
    slide
        .attributes
        .get(name)
        .or_else(|| document.presentation.properties.get(name))
}
fn is_true(value: Option<&Value>) -> bool {
    value.is_some_and(|value| value.to_text().trim() == "true")
}
impl SlideMetadata {
    pub fn resolve(document: &Document, slide: &Element, today: &str) -> Self {
        let footer = match effective(document, slide, "footer") {
            Some(Value::Boolean(true)) => match document.presentation.properties.get("footer") {
                Some(Value::String(value)) => value.clone(),
                _ => String::new(),
            },
            Some(Value::Boolean(false)) | None => String::new(),
            Some(value) => value.to_text(),
        };
        let value = effective(document, slide, "date");
        let automatic_date = is_true(value);
        let date = if automatic_date {
            today.into()
        } else {
            let text = value.map(Value::to_text).unwrap_or_default();
            if text.trim() == "false" {
                String::new()
            } else {
                text.trim().into()
            }
        };
        Self {
            footer: footer.trim().into(),
            date,
            automatic_date,
            slide_number: is_true(effective(document, slide, "slide_number")),
            notes: crate::properties::text(&slide.attributes, "notes")
                .trim()
                .into(),
        }
    }
    pub fn fields(&self, slide_number: usize) -> Vec<Field> {
        let mut fields = Vec::new();
        if !self.footer.is_empty() {
            fields.push(Field {
                kind: FieldKind::Footer,
                text: self.footer.clone(),
            });
        }
        if !self.date.is_empty() {
            fields.push(Field {
                kind: FieldKind::Date {
                    automatic: self.automatic_date,
                },
                text: self.date.clone(),
            });
        }
        if self.slide_number {
            fields.push(Field {
                kind: FieldKind::SlideNumber,
                text: slide_number.to_string(),
            });
        }
        fields
    }
}
impl Field {
    pub fn editable_bounds(&self, [width, height]: [i64; 2]) -> [i64; 4] {
        let (horizontal, width) = match self.kind {
            FieldKind::Footer => (457200, width / 3),
            FieldKind::Date { .. } => ((width - width / 4) / 2, width / 4),
            FieldKind::SlideNumber => (width - 457200 - 731520, 731520),
        };
        [horizontal, height - 457200, width, 274320]
    }
    pub fn native_box(&self, [width, height]: [f64; 2]) -> Element {
        let (horizontal, width, alignment) = match self.kind {
            FieldKind::Footer => (36.0, width / 3.0, "left"),
            FieldKind::Date { .. } => ((width - width / 4.0) / 2.0, width / 4.0, "center"),
            FieldKind::SlideNumber => (width - 36.0 - 57.6, 57.6, "right"),
        };
        let points = |value| Value::Length {
            value,
            unit: Unit::Point,
        };
        Element {
            kind: ElementKind::Box,
            attributes: Properties::from([
                (
                    "position".into(),
                    Value::Object(Properties::from([
                        ("x".into(), points(horizontal)),
                        ("y".into(), points(height - 36.0)),
                        ("width".into(), points(width)),
                        ("height".into(), points(21.6)),
                    ])),
                ),
                (
                    "margin".into(),
                    Value::Object(Properties::from([
                        ("top".into(), points(3.6)),
                        ("bottom".into(), points(3.6)),
                        ("left".into(), points(7.2)),
                        ("right".into(), points(7.2)),
                    ])),
                ),
                ("size".into(), points(10.0)),
                ("color".into(), Value::String("#6B7280".into())),
                ("alignment".into(), Value::String(alignment.into())),
                ("vertical_alignment".into(), Value::String("bottom".into())),
            ]),
            children: vec![Content::Text(self.text.clone())],
        }
    }
}

pub(super) fn current_date(
    document: &Document,
    slides: &[Element],
    warnings: &mut Vec<String>,
) -> String {
    if !slides
        .iter()
        .any(|slide| is_true(effective(document, slide, "date")))
    {
        return String::new();
    }
    let now = time::OffsetDateTime::now_local().unwrap_or_else(|_| {
        warnings.push(
            "automatic slide date uses UTC because the local time zone could not be determined"
                .into(),
        );
        time::OffsetDateTime::now_utc()
    });
    format_date(now.date())
}
fn format_date(date: time::Date) -> String {
    format!("{} {}, {}", date.month(), date.day(), date.year())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn automatic_dates_and_overrides_resolve_with_a_fixed_calendar_date() {
        let document = crate::compile("#presentation(footer=' Deck ',date=true,slide_number=true)#slide[]#slide(footer=true,date=' Fixed ')[]#slide(footer=false,date=false,slide_number=false,notes=' Notes ')[]").unwrap();
        let today =
            format_date(time::Date::from_calendar_date(2026, time::Month::January, 2).unwrap());
        assert_eq!(today, "January 2, 2026");
        let slides: Vec<_> = document
            .content
            .iter()
            .filter_map(|content| match content {
                Content::Element(element) => Some(element),
                _ => None,
            })
            .collect();
        let inherited = SlideMetadata::resolve(&document, slides[0], &today);
        let fields = inherited.fields(1);
        assert_eq!(
            fields
                .iter()
                .map(|field| field.text.as_str())
                .collect::<Vec<_>>(),
            ["Deck", "January 2, 2026", "1"]
        );
        assert!(matches!(
            fields[1].kind,
            FieldKind::Date { automatic: true }
        ));
        let fixed = SlideMetadata::resolve(&document, slides[1], &today).fields(2);
        assert_eq!(fixed[0].text, "Deck");
        assert_eq!(fixed[1].text, "Fixed");
        assert!(matches!(
            fixed[1].kind,
            FieldKind::Date { automatic: false }
        ));
        let suppressed = SlideMetadata::resolve(&document, slides[2], &today);
        assert!(suppressed.fields(3).is_empty());
        assert_eq!(suppressed.notes, "Notes");
    }
}
