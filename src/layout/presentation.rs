//! Positioned slides use the same page operations as document layout.
mod images;
pub(crate) mod shapes;
mod tables;
use super::{
    DocumentTextPlan, HighlightRectangle, LayoutError, LineCap, PageGeometry, PageOperation,
    PathPaint, PathStroke, TextPage,
    drawing_text::{self, FittedText},
};
use crate::{
    measurement::TextMeasurer,
    model::{Content, Document, Element, ElementKind, Properties},
    presentation::{Deck, Theme, color, units},
    properties::{number, object, text},
};

pub(crate) struct PreparedBox {
    pub bounds: [f64; 4],
    pub fitted: FittedText,
}
pub(crate) fn prepare_box(
    document: &Document,
    element: &Element,
    theme: &Theme,
    measurer: &mut TextMeasurer<'_>,
) -> Result<PreparedBox, LayoutError> {
    if element.attributes.contains_key("shadow") {
        return Err(LayoutError::Unsupported(
            "presentation box shadows are not yet supported".into(),
        ));
    }
    let mut bounds = crate::presentation::box_bounds(&element.attributes)
        .map_err(LayoutError::Unsupported)?
        .map(|value| value as f64 / 12700.0);
    if bounds[2] <= 0.0 || bounds[3] <= 0.0 {
        return Ok(PreparedBox {
            bounds,
            fitted: FittedText {
                page: TextPage::default(),
                font_scale: 1.0,
                height_delta: 0.0,
            },
        });
    }
    let attributes = crate::drawings::text_attributes(&element.attributes);
    let paragraphs = crate::presentation::text::resolve(
        document,
        &element.children,
        &attributes,
        theme,
        crate::presentation::text::TextDefaults::Resolved,
    )?;
    let fitted = drawing_text::fit(&paragraphs, bounds, &attributes, measurer)?;
    bounds[3] += fitted.height_delta;
    Ok(PreparedBox { bounds, fitted })
}

/// Plan explicit slides and positioned text without reinterpreting ordinary Markdown.
pub fn plan_presentation(
    document: &Document,
    measurer: &mut TextMeasurer<'_>,
) -> Result<DocumentTextPlan, LayoutError> {
    let deck = Deck::resolve(document).map_err(LayoutError::Unsupported)?;
    if crate::presentation::images::elements(&deck)
        .next()
        .is_some()
    {
        return Err(LayoutError::Unsupported(
            "presentation images require an asset resolver".into(),
        ));
    }
    plan(
        document,
        measurer,
        deck,
        &mut crate::media::ImageCatalog::default(),
    )
}

pub(crate) fn plan_with_assets(
    document: &Document,
    measurer: &mut TextMeasurer<'_>,
    resolver: &dyn crate::source::AssetResolver,
    source_name: Option<&std::ffi::OsStr>,
) -> Result<DocumentTextPlan, LayoutError> {
    let deck = Deck::resolve(document).map_err(LayoutError::Unsupported)?;
    let mut images = crate::media::ImageCatalog::load_selected(
        crate::presentation::images::elements(&deck),
        resolver,
        source_name,
    );
    plan(document, measurer, deck, &mut images)
}

fn plan(
    document: &Document,
    measurer: &mut TextMeasurer<'_>,
    mut deck: Deck,
    images: &mut crate::media::ImageCatalog,
) -> Result<DocumentTextPlan, LayoutError> {
    if !document
        .content
        .iter()
        .any(|node| matches!(node, Content::Element(element) if element.kind == ElementKind::Slide))
    {
        return Err(LayoutError::Unsupported(
            "native presentation output requires at least one explicit slide".into(),
        ));
    }
    let geometry = PageGeometry {
        width: deck.width as f64 / 12700.0,
        height: deck.height as f64 / 12700.0,
        margin_top: 0.0,
        margin_right: 0.0,
        margin_bottom: 0.0,
        margin_left: 0.0,
        header_space: 0.0,
        footer_space: 0.0,
        column_count: 1,
        column_gap: 0.0,
        column_separator: false,
    };
    let mut pages = Vec::new();
    for (index, slide) in deck.slides.iter().enumerate() {
        let mut page = TextPage {
            geometry: Some(geometry),
            page_number: index + 1,
            ..Default::default()
        };
        let background = slide
            .background
            .color
            .clone()
            .unwrap_or_else(|| deck.theme.colors[1].clone());
        page.operations
            .push(PageOperation::Rectangle(HighlightRectangle {
                x: 0.0,
                top: 0.0,
                width: geometry.width,
                height: geometry.height,
                color: background,
            }));
        page.operations.push(PageOperation::PushClip([
            0.0,
            0.0,
            geometry.width,
            geometry.height,
        ]));
        if let Some(image) = &slide.background.image {
            images::place(
                &slide.background.container(deck.width, deck.height),
                image,
                images::ImageContainer::Background,
                images,
                &mut page.operations,
                &mut deck.warnings,
            )?;
        }
        for child in &slide.element.children {
            let Content::Element(element) = child else {
                return Err(LayoutError::Unsupported(
                    "presentation output currently requires positioned slide objects".into(),
                ));
            };
            if element.kind == ElementKind::Shape {
                shapes::place(document, element, &deck.theme, measurer, &mut page)?;
                continue;
            }
            if element.kind == ElementKind::Line {
                page.operations
                    .extend(shapes::connector(&element.attributes));
                continue;
            }
            if element.kind != ElementKind::Box {
                return Err(LayoutError::Unsupported(format!(
                    "native slides do not yet support {} objects",
                    element.kind.as_text()
                )));
            }
            if element.children.is_empty() {
                deck.warnings.push("native slides omit empty boxes".into());
                continue;
            }
            if let [Content::Element(chart)] = element.children.as_slice()
                && chart.kind == ElementKind::Chart
            {
                deck.warnings
                    .extend(crate::charts::support::native_warnings(&chart.attributes));
                page.append_content(super::charts::plan_slide(
                    document,
                    element,
                    chart,
                    &deck.theme,
                    measurer,
                )?);
                continue;
            }
            if let [Content::Element(table)] = element.children.as_slice()
                && table.kind == ElementKind::Table
            {
                tables::place(
                    document,
                    element,
                    table,
                    &deck.theme,
                    measurer,
                    &mut page,
                    &mut deck.warnings,
                )?;
                continue;
            }
            if let Some(image) = crate::presentation::images::child(element) {
                images::place(
                    element,
                    image,
                    images::ImageContainer::Box,
                    images,
                    &mut page.operations,
                    &mut deck.warnings,
                )?;
                continue;
            }
            let prepared = prepare_box(document, element, &deck.theme, measurer)?;
            if prepared.bounds[2] <= 0.0 || prepared.bounds[3] <= 0.0 {
                continue;
            }
            if let Some(rectangle) = rectangle(prepared.bounds, &element.attributes) {
                page.operations.push(rectangle);
            }
            page.append_content(prepared.fitted.page);
        }
        for field in slide.metadata.fields(index + 1) {
            let element = field.native_box([geometry.width, geometry.height]);
            let prepared = prepare_box(document, &element, &deck.theme, measurer)?;
            page.append_content(prepared.fitted.page);
        }
        page.operations.push(PageOperation::PopClip);
        pages.push(page);
    }
    Ok(DocumentTextPlan {
        geometry,
        pages,
        warnings: deck.warnings,
    })
}
fn rectangle(bounds: [f64; 4], attributes: &Properties) -> Option<PageOperation> {
    let fill = color(attributes.get("fill")).map(|color| PathPaint {
        color,
        alpha: number(attributes, "opacity")
            .map(|value| {
                let percentage = if value <= 1.0 { value * 100.0 } else { value };
                (percentage.clamp(0.0, 100.0) * 255.0 / 100.0).round() as u8
            })
            .unwrap_or(255),
    });
    let stroke = object(attributes, "border").and_then(|border| {
        if text(border, "style") == "none" {
            return None;
        }
        let color = color(border.get("color"))?;
        let width = units(border.get("width"))
            .map(|width| width as f64 / 12700.0)
            .unwrap_or(1.0);
        if width <= 0.0 {
            return None;
        }
        let alpha = number(border, "opacity")
            .map(|value| (value.trunc().clamp(0.0, 100.0) * 255.0 / 100.0) as u8)
            .unwrap_or(255);
        let (dash, cap) = match text(border, "style").as_str() {
            "dashed" => (vec![3.0 * width, 3.0 * width], LineCap::Flat),
            "dotted" => (vec![(width / 100.0).max(0.01), 4.0 * width], LineCap::Round),
            _ => (Vec::new(), LineCap::Flat),
        };
        Some(PathStroke {
            paint: PathPaint { color, alpha },
            width,
            dash,
            cap,
        })
    });
    if fill.is_none() && stroke.is_none() {
        return None;
    }
    Some(PageOperation::PaintedRectangle {
        bounds,
        fill,
        stroke,
    })
}
