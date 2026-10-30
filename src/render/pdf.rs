mod fonts;
mod images;
mod paths;
use super::{RenderError, color, validate_plan};
use crate::{
    layout::{
        DocumentTextPlan, HighlightRectangle, ListMarker, ListMarkerKind, PageOperation, Rule,
        TextLine,
    },
    measurement::{glyphs, win_ansi},
};
use flate2::{Compression, write::ZlibEncoder};
use pdf_writer::{Content, Filter, Name, Pdf, Rect, Ref, Str, TextStr};
use std::io::Write;

pub fn paint_pdf(plan: &DocumentTextPlan) -> Result<Vec<u8>, RenderError> {
    validate_plan(plan)?;
    let mut document = Pdf::new();
    document.set_version(1, 7);
    let mut objects = Objects::default();
    let catalog = objects.allocate();
    let page_tree = objects.allocate();
    document.catalog(catalog).pages(page_tree);
    let fonts = fonts::Fonts::write(plan, &mut document, &mut objects)?;
    let page_ids: Vec<_> = (0..plan.pages.len().max(1))
        .map(|_| objects.allocate())
        .collect();
    let destinations = crate::layout::references::destinations(&plan.pages);
    // An empty source still produces a useful, valid one-page PDF.
    for page_index in 0..plan.pages.len().max(1) {
        let geometry = plan
            .pages
            .get(page_index)
            .and_then(|page| page.geometry)
            .unwrap_or(plan.geometry);
        let page_id = page_ids[page_index];
        let content_id = objects.allocate();

        let mut content = Content::new();
        let mut page_images = Vec::new();
        let mut opacities = paths::Opacities::default();
        if let Some(page) = plan.pages.get(page_index) {
            for operation in &page.operations {
                match operation {
                    PageOperation::PushRotation { degrees, center } => {
                        let (sine, cosine) = degrees.to_radians().sin_cos();
                        let [horizontal, vertical] = [center[0], geometry.height - center[1]];
                        let matrix = [
                            cosine,
                            -sine,
                            sine,
                            cosine,
                            horizontal - cosine * horizontal - sine * vertical,
                            vertical + sine * horizontal - cosine * vertical,
                        ]
                        .map(|value| ((value * 100000.0).round() / 100000.0) as f32);
                        content.save_state().transform(matrix);
                    }
                    PageOperation::PopRotation => {
                        content.restore_state();
                    }

                    PageOperation::PaintedRectangle {
                        bounds,
                        fill,
                        stroke,
                    } => paths::draw_rectangle(
                        &mut content,
                        *bounds,
                        fill.as_ref(),
                        stroke.as_ref(),
                        geometry.height,
                        &mut document,
                        &mut objects,
                        &mut opacities,
                    )?,
                    PageOperation::Path(path) => paths::draw(
                        &mut content,
                        path,
                        geometry.height,
                        &mut document,
                        &mut objects,
                        &mut opacities,
                    )?,
                    PageOperation::PushClip(bounds) => {
                        content
                            .save_state()
                            .rect(
                                rounded(bounds[0]),
                                rounded(geometry.height - bounds[1]),
                                rounded(bounds[2]),
                                -rounded(bounds[3]),
                            )
                            .clip_nonzero()
                            .end_path();
                    }
                    PageOperation::PopClip => {
                        content.restore_state();
                    }
                    PageOperation::Stroke {
                        rule,
                        dash,
                        round_cap,
                    } => {
                        content
                            .save_state()
                            .set_dash_pattern(dash.iter().map(|value| rounded(*value)), 0.0)
                            .set_line_cap(if *round_cap {
                                pdf_writer::types::LineCapStyle::RoundCap
                            } else {
                                pdf_writer::types::LineCapStyle::ButtCap
                            });
                        paint_rule(&mut content, rule, geometry.height)?;
                        content.restore_state();
                    }
                    PageOperation::OutlineRectangle {
                        bounds,
                        width,
                        color: color_value,
                    } => {
                        let [red, green, blue] = color(color_value)?.map(|channel| {
                            ((f64::from(channel) / 255.0 * 1000.0).round() / 1000.0) as f32
                        });
                        content
                            .set_stroke_rgb(red, green, blue)
                            .set_line_width(rounded(*width))
                            .rect(
                                rounded(bounds[0]),
                                rounded(geometry.height - bounds[1]),
                                rounded(bounds[2]),
                                -rounded(bounds[3]),
                            )
                            .stroke();
                    }
                    PageOperation::Image(image) => {
                        let reference = images::write(image, &mut document, &mut objects)?;
                        let name = format!("Image{}", page_images.len() + 1);
                        images::paint(image, &mut content, &name, geometry.height);
                        page_images.push((name, reference));
                    }
                    PageOperation::Text(index) => paint_line(
                        &mut content,
                        &page.lines[*index].line,
                        geometry.height,
                        &fonts,
                        "",
                    )?,
                    PageOperation::Rectangle(rectangle) => {
                        paint_rectangle(&mut content, rectangle, geometry.height)?
                    }
                    PageOperation::Rule(rule) => paint_rule(&mut content, rule, geometry.height)?,
                    PageOperation::ListMarker(marker) => {
                        paint_marker(&mut content, marker, geometry.height, &fonts)?
                    }
                }
            }
        }
        let content_bytes = compress(&content.finish())?;
        document
            .stream(content_id, &content_bytes)
            .filter(Filter::FlateDecode);
        let mut annotations = Vec::new();
        if let Some(page) = plan.pages.get(page_index) {
            let mut rotations = Vec::new();
            for operation in &page.operations {
                match operation {
                    PageOperation::PushRotation { degrees, center } => {
                        rotations.push((*degrees, *center))
                    }
                    PageOperation::PopRotation => {
                        rotations.pop();
                    }
                    _ => {}
                }
                if let PageOperation::Image(image) = operation
                    && let Some(target) = &image.link
                {
                    let reference = objects.allocate();
                    annotations.push(reference);
                    let mut annotation = document.annotation(reference);
                    annotation
                        .subtype(pdf_writer::types::AnnotationType::Link)
                        .rect(images::annotation_bounds(
                            image,
                            geometry.height,
                            &rotations,
                        ))
                        .border(0.0, 0.0, 0.0, None);
                    annotation
                        .action()
                        .action_type(pdf_writer::types::ActionType::Uri)
                        .uri(Str(target.as_bytes()));
                }
            }
            for placed in &page.lines {
                for fragment in &placed.line.fragments {
                    if let Some(target) =
                        fragment.link.as_ref().or(fragment.internal_target.as_ref())
                    {
                        if fragment.advance <= 0.0 {
                            continue;
                        }
                        let reference = objects.allocate();
                        annotations.push(reference);
                        let mut annotation = document.annotation(reference);
                        annotation
                            .subtype(pdf_writer::types::AnnotationType::Link)
                            .rect(Rect::new(
                                rounded(fragment.x),
                                rounded(geometry.height - placed.top - placed.line.height),
                                rounded(fragment.x + fragment.advance),
                                rounded(geometry.height - placed.top),
                            ))
                            .border(0.0, 0.0, 0.0, None);
                        if fragment.internal_target.is_some() {
                            if let Some((destination_page, position)) = destinations.get(target) {
                                let destination_height = plan.pages[*destination_page]
                                    .geometry
                                    .unwrap_or(plan.geometry)
                                    .height;
                                annotation
                                    .action()
                                    .action_type(pdf_writer::types::ActionType::GoTo)
                                    .destination()
                                    .page(page_ids[*destination_page])
                                    .xyz(0.0, rounded(destination_height - position[1]), None);
                            }
                        } else {
                            annotation
                                .action()
                                .action_type(pdf_writer::types::ActionType::Uri)
                                .uri(Str(target.as_bytes()));
                        }
                    }
                }
            }
        }
        let mut page = document.page(page_id);
        if !annotations.is_empty() {
            page.annotations(annotations);
        }
        page.parent(page_tree)
            .media_box(Rect::new(
                0.0,
                0.0,
                geometry.width as f32,
                geometry.height as f32,
            ))
            .contents(content_id);
        let mut resources = page.resources();
        if !page_images.is_empty() {
            let mut images = resources.x_objects();
            for (name, reference) in page_images {
                images.pair(Name(name.as_bytes()), reference);
            }
        }
        if !opacities.entries.is_empty() {
            let mut states = resources.ext_g_states();
            for (_, (name, reference)) in opacities.entries {
                states.pair(Name(name.as_bytes()), reference);
            }
        }
        let mut page_fonts = resources.fonts();
        for font in &fonts.entries {
            page_fonts.pair(Name(font.resource.as_bytes()), font.reference);
        }
    }
    document
        .pages(page_tree)
        .count(page_ids.len() as i32)
        .kids(page_ids);
    Ok(document.finish())
}
fn paint_line(
    content: &mut Content,
    line: &TextLine,
    page_height: f64,
    fonts: &fonts::Fonts,
    actual_text_suffix: &str,
) -> Result<(), RenderError> {
    for rectangle in &line.highlights {
        paint_rectangle(content, rectangle, page_height)?;
    }
    let mut actual_text = String::new();
    let mut previous: Option<&crate::layout::TextFragment> = None;
    for fragment in &line.fragments {
        if let Some(previous) = previous
            && !previous.text.ends_with(char::is_whitespace)
            && !fragment.text.starts_with(char::is_whitespace)
            && fragment.x - previous.x - previous.width > 0.5
        {
            actual_text.push(' ');
        }
        actual_text.push_str(&fragment.text);
        previous = Some(fragment);
    }
    actual_text.push_str(actual_text_suffix);
    let preserve_actual_text = !actual_text.is_empty()
        && line
            .fragments
            .iter()
            .any(|fragment| fragment.font.bytes.is_some());
    if preserve_actual_text {
        content
            .begin_marked_content_with_properties(Name(b"Span"))
            .properties()
            .actual_text(TextStr(&actual_text));
    }
    for fragment in &line.fragments {
        if fragment.text.is_empty() {
            continue;
        }
        let font = fonts.find(&fragment.font)?;
        let [red, green, blue] = color(&fragment.style.color)?
            .map(|channel| ((f64::from(channel) / 255.0 * 1000.0).round() / 1000.0) as f32);
        content
            .set_fill_rgb(red, green, blue)
            .begin_text()
            .set_font(
                Name(font.resource.as_bytes()),
                fragment.style.measurement.size_points as f32,
            );
        let compact = font
            .selection
            .bytes
            .as_ref()
            .is_some_and(|bytes| bytes.starts_with(b"OTTO"));
        let baseline = if compact {
            (page_height - fragment.baseline) as f32
        } else {
            rounded(page_height - fragment.baseline)
        };
        if let Some(bytes) = &font.selection.bytes {
            let face = ttf_parser::Face::parse(bytes, 0)
                .map_err(|_| RenderError::Invalid("invalid PDF font".into()))?;
            if face.tables().cff.is_some() {
                for placement in glyphs::placements(
                    &face,
                    &fragment.text,
                    fragment.style.measurement.size_points,
                )? {
                    let code = font.code(placement.character)?;
                    content
                        .set_text_matrix([
                            1.0,
                            0.0,
                            0.0,
                            1.0,
                            (fragment.x + placement.offset) as f32,
                            baseline,
                        ])
                        .show(Str(&code.to_be_bytes()));
                }
            } else {
                let mut encoded = Vec::new();
                for character in fragment.text.chars() {
                    encoded.extend(font.code(character)?.to_be_bytes());
                }
                content
                    .set_text_matrix([1.0, 0.0, 0.0, 1.0, rounded(fragment.x), baseline])
                    .show(Str(&encoded));
            }
        } else {
            let encoded = fragment
                .text
                .chars()
                .map(|character| {
                    win_ansi(character)
                        .ok_or_else(|| RenderError::Invalid("unprepared core-font text".into()))
                })
                .collect::<Result<Vec<_>, _>>()?;
            content
                .set_text_matrix([1.0, 0.0, 0.0, 1.0, rounded(fragment.x), baseline])
                .show(Str(&encoded));
        }
        content.end_text();
    }
    if preserve_actual_text {
        content.end_marked_content();
    }
    for stroke in &line.decorations {
        let [red, green, blue] = color(&stroke.color)?
            .map(|channel| ((f64::from(channel) / 255.0 * 1000.0).round() / 1000.0) as f32);
        content
            .set_stroke_rgb(red, green, blue)
            .set_line_width(rounded(stroke.width))
            .move_to(rounded(stroke.start), rounded(page_height - stroke.y))
            .line_to(rounded(stroke.end), rounded(page_height - stroke.y))
            .stroke();
    }
    Ok(())
}
#[derive(Default)]
struct Objects {
    next: i32,
}
impl Objects {
    fn allocate(&mut self) -> Ref {
        self.next += 1;
        Ref::new(self.next)
    }
}
fn compress(bytes: &[u8]) -> Result<Vec<u8>, RenderError> {
    let mut encoder = ZlibEncoder::new(Vec::new(), Compression::default());
    encoder.write_all(bytes)?;
    Ok(encoder.finish()?)
}

fn rounded(value: f64) -> f32 {
    format!("{value:.2}")
        .parse()
        .expect("finite PDF coordinate")
}

fn paint_rectangle(
    content: &mut Content,
    rectangle: &HighlightRectangle,
    page_height: f64,
) -> Result<(), RenderError> {
    let [red, green, blue] = color(&rectangle.color)?
        .map(|channel| ((f64::from(channel) / 255.0 * 1000.0).round() / 1000.0) as f32);
    content
        .set_fill_rgb(red, green, blue)
        .rect(
            rounded(rectangle.x),
            rounded(page_height - rectangle.top),
            rounded(rectangle.width),
            -rounded(rectangle.height),
        )
        .fill_nonzero();
    Ok(())
}
fn paint_rule(content: &mut Content, rule: &Rule, page_height: f64) -> Result<(), RenderError> {
    let [red, green, blue] = color(&rule.color)?
        .map(|channel| ((f64::from(channel) / 255.0 * 1000.0).round() / 1000.0) as f32);
    content
        .set_stroke_rgb(red, green, blue)
        .set_line_width(rounded(rule.width))
        .move_to(rounded(rule.start[0]), rounded(page_height - rule.start[1]))
        .line_to(rounded(rule.end[0]), rounded(page_height - rule.end[1]))
        .stroke();
    Ok(())
}
fn paint_marker(
    content: &mut Content,
    marker: &ListMarker,
    page_height: f64,
    fonts: &fonts::Fonts,
) -> Result<(), RenderError> {
    let [left, top, width, height] = marker.slot;
    match marker.kind {
        ListMarkerKind::Ordered => {
            if let Some(line) = &marker.text {
                paint_line(content, line, page_height, fonts, " ")?;
            }
        }
        ListMarkerKind::Bullet => {
            let radius = marker.font_size * 1.7 / 12.0;
            let center_x = left + (radius * 2.0).min(width) / 2.0;
            let center_y = top + height * 0.56;
            let color_value = marker
                .text
                .as_ref()
                .and_then(|line| line.fragments.first())
                .map(|fragment| fragment.style.color.as_str())
                .unwrap_or("111827");
            let [red, green, blue] = color(color_value)?
                .map(|channel| ((f64::from(channel) / 255.0 * 1000.0).round() / 1000.0) as f32);
            content.set_fill_rgb(red, green, blue);
            circle(content, center_x, page_height - center_y, radius);
        }
        ListMarkerKind::Task => {
            let size = marker.font_size * 7.5 / 12.0;
            let horizontal = left + ((width - size) / 2.0).max(0.0);
            let vertical = top + (height - size) / 2.0 + marker.font_size * 0.5 / 12.0;
            content
                .set_stroke_rgb(0.067, 0.094, 0.153)
                .set_line_width(rounded(marker.font_size * 0.75 / 12.0))
                .rect(
                    rounded(horizontal),
                    rounded(page_height - vertical),
                    rounded(size),
                    -rounded(size),
                )
                .stroke();
            if marker.checked {
                let segments = if marker.task_mark == crate::layout::TaskMark::Cross {
                    [([0.18, 0.18], [0.82, 0.82]), ([0.82, 0.18], [0.18, 0.82])]
                } else {
                    [
                        ([1.4 / 7.5, 4.2 / 7.5], [3.2 / 7.5, 6.0 / 7.5]),
                        ([3.2 / 7.5, 6.0 / 7.5], [6.5 / 7.5, 1.4 / 7.5]),
                    ]
                };
                for (start, end) in segments {
                    content
                        .move_to(
                            rounded(horizontal + size * start[0]),
                            rounded(page_height - vertical - size * start[1]),
                        )
                        .line_to(
                            rounded(horizontal + size * end[0]),
                            rounded(page_height - vertical - size * end[1]),
                        )
                        .stroke();
                }
            }
        }
    }
    Ok(())
}
fn circle(content: &mut Content, horizontal: f64, vertical: f64, radius: f64) {
    // Six cubic segments retain the established circular marker contour.
    let step = std::f64::consts::TAU / 6.0;
    let tangent_scale = step / 3.0;
    let precise = |value: f64| {
        format!("{value:.5}")
            .parse::<f32>()
            .expect("finite marker coordinate")
    };
    content.move_to(rounded(horizontal + radius), rounded(vertical));
    for segment in 0..6 {
        let start = segment as f64 * step;
        let end = (segment + 1) as f64 * step;
        content.cubic_to(
            precise(horizontal + radius * (start.cos() - start.sin() * tangent_scale)),
            precise(vertical + radius * (start.sin() + start.cos() * tangent_scale)),
            precise(horizontal + radius * (end.cos() + end.sin() * tangent_scale)),
            precise(vertical + radius * (end.sin() - end.cos() * tangent_scale)),
            precise(horizontal + radius * end.cos()),
            precise(vertical + radius * end.sin()),
        );
    }
    content.fill_nonzero();
}
