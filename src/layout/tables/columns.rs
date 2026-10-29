use super::*;
use crate::model::{Content, ElementKind};
pub(super) struct Constraint {
    pub column: usize,
    pub span: usize,
    pub explicit: f64,
    pub minimum: f64,
    pub preferred: f64,
}
pub(super) fn intrinsic(
    content: &[Content],
    attributes: &Properties,
    available: f64,
    styles: &LayoutStyleResolver<'_>,
    measurer: &mut TextMeasurer<'_>,
    images: &ImageCatalog,
    numbers: &super::super::blocks::BlockNumbers,
) -> Result<(f64, f64), LayoutError> {
    let mut preferred: f64 = 0.0;
    let mut minimum: f64 = 0.0;
    for node in content {
        let Content::Element(element) = node else {
            continue;
        };
        let (inset, floor) = cells::object_inset(element, attributes, styles);
        if element.kind == ElementKind::Table {
            let nested = prepare(
                element,
                (available - inset).max(1.0),
                styles,
                measurer,
                images,
                &mut numbers.clone(),
            )?;
            preferred = preferred.max(nested.width() + inset).max(floor);
            minimum = minimum.max(nested.width() + inset).max(floor);
            continue;
        }
        for block in super::super::blocks::prepare_content(
            std::slice::from_ref(node),
            if element.kind == ElementKind::Image {
                (available - inset).max(1.0)
            } else {
                100000.0
            },
            styles,
            measurer,
            images,
            &mut numbers.clone(),
            true,
        )? {
            match block {
                Block::Paragraph(paragraph) => {
                    let style = &paragraph.text.style;
                    let inset = style.indent_left.max(0.0)
                        + style.indent_right.max(0.0)
                        + style.first_line.max(0.0);
                    preferred = preferred.max(style.text.measurement.size_points * 2.0);
                    minimum = minimum.max(style.text.measurement.size_points * 2.0);
                    for line in &paragraph.text.lines {
                        let runs: Vec<_> = line
                            .fragments
                            .iter()
                            .map(|fragment| {
                                super::super::TextRun::new(&fragment.text, fragment.style.clone())
                            })
                            .collect();
                        let mut line_width = 0.0;
                        for group in super::super::wrapping::groups(&runs) {
                            let width = super::super::wrapping::measure(&group, measurer, true)?;
                            line_width += width;
                            if !super::super::wrapping::join(&group).trim().is_empty() {
                                minimum = minimum.max(width + inset);
                            }
                        }
                        preferred = preferred.max(line_width + inset);
                    }
                }
                Block::Image(image) => {
                    preferred = preferred.max(image.width + inset).max(floor);
                    minimum = minimum.max(floor);
                }
                _ => {}
            }
        }
    }
    Ok((preferred, minimum))
}
pub(super) fn resolve(
    count: usize,
    available: f64,
    requested: f64,
    default: f64,
    cells: &[Constraint],
) -> Vec<f64> {
    let mut explicit = vec![0.0_f64; count];
    let mut minimum = vec![default; count];
    let mut preferred = minimum.clone();
    let mut has_explicit = false;
    for cell in cells {
        for column in cell.column..(cell.column + cell.span).min(count) {
            explicit[column] = explicit[column].max(cell.explicit / cell.span as f64);
            minimum[column] = minimum[column].max(cell.minimum / cell.span as f64);
            preferred[column] = preferred[column].max(cell.preferred / cell.span as f64);
        }
        has_explicit |= cell.explicit > 0.0;
    }
    let maximum = if requested > 0.0 {
        available.min(requested)
    } else {
        available
    };
    if has_explicit {
        for (explicit, preferred) in explicit.iter_mut().zip(preferred) {
            if *explicit <= 0.0 {
                *explicit = preferred;
            }
        }
        return fit(explicit, maximum);
    }
    if requested > 0.0 {
        return vec![maximum / count as f64; count];
    }
    if preferred.iter().sum::<f64>() <= maximum {
        return preferred;
    }
    let minimum_total: f64 = minimum.iter().sum();
    if minimum_total >= maximum {
        return fit(minimum, maximum);
    }
    let remaining = maximum - minimum_total;
    let demand: f64 = minimum
        .iter()
        .zip(&preferred)
        .map(|(minimum, preferred)| (preferred - minimum).max(0.0))
        .sum();
    if demand > 0.0 {
        for (minimum, preferred) in minimum.iter_mut().zip(preferred) {
            *minimum += ((preferred - *minimum).max(0.0) * remaining / demand)
                .min((preferred - *minimum).max(0.0));
        }
    }
    minimum
}
fn fit(mut widths: Vec<f64>, maximum: f64) -> Vec<f64> {
    let total: f64 = widths.iter().sum();
    if total > maximum && maximum > 0.0 {
        for width in &mut widths {
            *width *= maximum / total;
        }
    }
    widths
}
