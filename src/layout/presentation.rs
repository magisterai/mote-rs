//! Positioned slides use the same page operations as document layout.
mod images;
pub(crate) mod shapes;
pub(crate) mod tables;
use super::{
    DocumentTextPlan, HighlightRectangle, LayoutError, LineCap, PageGeometry, PageOperation,
    PathPaint, PathStroke, TextPage,
    drawing_text::{self, FittedText},
    overflow,
};
use crate::{
    measurement::TextMeasurer,
    model::{Content, Document, Element, ElementKind, Properties},
    presentation::{Deck, Theme, animation::Appearance, color, units},
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
    let mut bounds = crate::presentation::box_bounds(&element.attributes)
        .map_err(LayoutError::Unsupported)?
        .map(|value| value as f64 / 12700.0);
    if bounds[2] <= 0.0 || bounds[3] <= 0.0 {
        return Ok(PreparedBox {
            bounds,
            fitted: FittedText::empty(),
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

/// Plan a document explicitly selected for presentation layout.
pub fn plan_presentation(
    document: &Document,
    measurer: &mut TextMeasurer<'_>,
) -> Result<DocumentTextPlan, LayoutError> {
    let deck = Deck::resolve(document, measurer).map_err(LayoutError::Unsupported)?;
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
    let deck = Deck::resolve(document, measurer).map_err(LayoutError::Unsupported)?;
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
    deck.stack(document, measurer, images)
        .map_err(LayoutError::Unsupported)?;
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
        let slide_path = format!("content[{index}].slide");
        place(
            &slide.element.children,
            &slide_path,
            &mut Placement {
                document,
                theme: &deck.theme,
                measurer,
                images,
                warnings: &mut deck.warnings,
                time: deck.time,
                wipe_margin: geometry.width + geometry.height,
            },
            &mut page,
        )?;
        for field in slide.metadata.fields(index + 1) {
            let element = field.native_box([geometry.width, geometry.height], &slide.metadata.text);
            let prepared = prepare_box(document, &element, &deck.theme, measurer)?;
            if prepared.fitted.is_cut_off() {
                deck.warnings
                    .push(overflow::field_text(field.kind, &slide_path));
            }
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
/// The setting in which the objects of a slide are placed.
struct Placement<'deck, 'measurer> {
    document: &'deck Document,
    theme: &'deck Theme,
    measurer: &'deck mut TextMeasurer<'measurer>,
    images: &'deck mut crate::media::ImageCatalog,
    warnings: &'deck mut Vec<String>,
    time: Option<f64>,
    /// The distance by which the region of a wipe passes the extent of its
    /// object on the sides that the wipe does not cut.
    wipe_margin: f64,
}

/// Places objects in their order. An object or a group whose appearance at
/// the time of rendering differs from the authored one is displaced, and is
/// drawn as a layer where it is enlarged, turned or faint. An object that a
/// wipe leaves partly in view is clipped within its layer, after its
/// displacement, so that the wipe is enlarged and turned with the object.
fn place(
    content: &[Content],
    path: &str,
    context: &mut Placement<'_, '_>,
    page: &mut TextPage,
) -> Result<(), LayoutError> {
    for (index, child) in content.iter().enumerate() {
        let Content::Element(element) = child else {
            return Err(LayoutError::Unsupported(
                "presentation output currently requires positioned slide objects".into(),
            ));
        };
        let appearance = Appearance::at(element, context.time);
        if appearance.opacity <= 0.0 {
            continue;
        }
        let mut piece = TextPage::default();
        let path = format!("{path}[{index}].{}", element.kind.as_text());
        if element.kind == ElementKind::Group {
            place(&element.children, &path, context, &mut piece)?;
        } else {
            place_object(element, &path, context, &mut piece)?;
        }
        if piece.operations.is_empty() {
            continue;
        }
        let layered =
            appearance.opacity < 1.0 || appearance.scale != 1.0 || appearance.rotation != 0.0;
        let [left, top, right, bottom] = crate::presentation::extent(element)
            .unwrap_or_default()
            .map(|value| value as f64 / 12700.0);
        let [left, right] = [left, right].map(|value| value + appearance.offset[0]);
        let [top, bottom] = [top, bottom].map(|value| value + appearance.offset[1]);
        if layered {
            page.operations.push(PageOperation::PushLayer {
                matrix: appearance.matrix([(left + right) / 2.0, (top + bottom) / 2.0]),
                opacity: appearance.opacity.min(1.0),
            });
        }
        if let Some(reveal) = appearance.reveal {
            page.operations.push(PageOperation::PushClip(
                reveal.clip([left, top, right, bottom], context.wipe_margin),
            ));
        }
        page.append_content_at(piece, appearance.offset);
        if appearance.reveal.is_some() {
            page.operations.push(PageOperation::PopClip);
        }
        if layered {
            page.operations.push(PageOperation::PopLayer);
        }
    }
    Ok(())
}

fn place_object(
    element: &Element,
    path: &str,
    context: &mut Placement<'_, '_>,
    page: &mut TextPage,
) -> Result<(), LayoutError> {
    if element.kind == ElementKind::Shape {
        let cut_off = shapes::place(
            context.document,
            element,
            context.theme,
            context.measurer,
            page,
        )?;
        if cut_off {
            context
                .warnings
                .push(overflow::slide_frame_text(element, path));
        }
        return Ok(());
    }
    if element.kind == ElementKind::Line {
        page.operations
            .extend(shapes::connector(&element.attributes));
        return Ok(());
    }
    if element.kind != ElementKind::Box {
        return Err(LayoutError::Unsupported(format!(
            "native slides do not yet support {} objects",
            element.kind.as_text()
        )));
    }
    if element.children.is_empty() {
        context
            .warnings
            .push("native slides omit empty boxes".into());
        return Ok(());
    }
    if let [Content::Element(chart)] = element.children.as_slice()
        && chart.kind == ElementKind::Chart
    {
        context
            .warnings
            .extend(crate::charts::support::native_warnings(&chart.attributes));
        let planned = super::charts::plan_slide(
            context.document,
            element,
            chart,
            context.theme,
            context.measurer,
        )?;
        context.warnings.extend(
            planned.reduced_labels_warning(&overflow::slide_object(
                chart,
                &format!("{path}[0].chart"),
            )),
        );
        page.append_content(planned.page);
        return Ok(());
    }
    if let [Content::Element(table)] = element.children.as_slice()
        && table.kind == ElementKind::Table
    {
        context.warnings.extend(tables::place(
            context.document,
            element,
            table,
            context.theme,
            context.measurer,
            page,
            &overflow::slide_object(element, path),
        )?);
        return Ok(());
    }
    if let Some(image) = crate::presentation::images::child(element) {
        images::place(
            element,
            image,
            images::ImageContainer::Box,
            context.images,
            &mut page.operations,
            context.warnings,
        )?;
        return Ok(());
    }
    let prepared = prepare_box(context.document, element, context.theme, context.measurer)?;
    if prepared.bounds[2] <= 0.0 || prepared.bounds[3] <= 0.0 {
        return Ok(());
    }
    if let Some(shadow) = object(&element.attributes, "shadow")
        && number(shadow, "opacity") != Some(0.0)
    {
        context
            .warnings
            .push("native slides omit text-box shadows".into());
    }
    if prepared.fitted.is_cut_off() {
        context
            .warnings
            .push(overflow::slide_frame_text(element, path));
    }
    if let Some(rectangle) = rectangle(prepared.bounds, &element.attributes) {
        page.operations.push(rectangle);
    }
    page.append_content(prepared.fitted.page);
    Ok(())
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
