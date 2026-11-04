use super::*;
use crate::{
    layout::{
        HighlightRectangle, LineOptions, ListMarker, ListMarkerKind, PaintedPath, PathPaint,
        PositionedImage, Rule, RuleRole, TextRun, TextStyle, plan_text,
    },
    measurement::TextMeasurer,
    media::{PreparedImage, RasterImage},
    render,
    source::FileAssetResolver,
};
use std::sync::Arc;

fn prepared_content() -> TextPage {
    let document = crate::compile("").unwrap();
    let resolver = FileAssetResolver::new(".").unwrap();
    let mut measurer = TextMeasurer::new(&document, &resolver, None);
    let style = TextStyle {
        underline: true,
        highlight: Some("FFF0A0".into()),
        ..Default::default()
    };
    let mut run = TextRun::new("Measured", style);
    run.anchors.push("label".into());
    let mut line = plan_text(&[run], &LineOptions::default(), &mut measurer)
        .unwrap()
        .remove(0);
    line.translate(20.0, 20.0);
    let mut page = TextPage::default();
    page.push_line(PlacedTextLine {
        top: 20.0,
        line: line.clone(),
    });
    let rule = Rule {
        role: RuleRole::Generic,
        start: [10.0, 50.0],
        end: [60.0, 60.0],
        width: 2.0,
        color: "224488".into(),
    };
    let fill = PathPaint {
        color: "AA6622".into(),
        alpha: 200,
    };
    page.operations.extend([
        PageOperation::PushClip([10.0, 10.0, 100.0, 100.0]),
        PageOperation::PushRotation {
            degrees: 30.0,
            center: [60.0, 60.0],
        },
        PageOperation::Path(PaintedPath {
            points: vec![[20.0, 40.0], [60.0, 40.0], [40.0, 70.0]],
            closed: true,
            fill: Some(fill.clone()),
            stroke: None,
        }),
        PageOperation::PaintedRectangle {
            bounds: [60.0, 40.0, 30.0, 20.0],
            fill: Some(fill),
            stroke: None,
        },
        PageOperation::OutlineRectangle {
            bounds: [60.0, 65.0, 30.0, 20.0],
            width: 1.5,
            color: "000000".into(),
        },
        PageOperation::Rule(rule.clone()),
        PageOperation::Stroke {
            rule,
            dash: vec![3.0, 2.0],
            round_cap: true,
        },
        PageOperation::Rectangle(HighlightRectangle {
            x: 10.0,
            top: 85.0,
            width: 40.0,
            height: 10.0,
            color: "33AA66".into(),
        }),
        PageOperation::Image(Box::new(PositionedImage {
            image: Arc::new(PreparedImage {
                raster: RasterImage {
                    width: 1,
                    height: 1,
                    pixels: Arc::from([200, 30, 100, 255]),
                },
                raster_width: 1,
                raster_height: 1,
                original_width: 1,
                original_height: 1,
                offset: [0.0, 0.0],
                jpeg: None,
            }),
            bounds: [70.0, 70.0, 30.0, 30.0],
            clip: [75.0, 75.0, 20.0, 20.0],
            link: None,
        })),
        PageOperation::ListMarker(Box::new(ListMarker {
            kind: ListMarkerKind::Ordered,
            checked: false,
            slot: [20.0, 20.0, 50.0, 20.0],
            baseline: line.baseline,
            font_size: 11.0,
            text: Some(line),
        })),
        PageOperation::PopRotation,
        PageOperation::PopClip,
    ]);
    page
}

#[test]
fn repeated_content_keeps_local_line_identity_and_page_state() {
    let content = prepared_content();
    let original = content.clone();
    let geometry = PageGeometry::default();
    let mut page = TextPage {
        geometry: Some(geometry),
        section_index: 2,
        page_number: 7,
        note_body_bottom: Some(650.0),
        ..Default::default()
    };
    page.append_content(content.clone());
    page.append_content_at(content.clone(), [150.0, 180.0]);

    assert_eq!(content, original);
    assert_eq!(page.geometry, Some(geometry));
    assert_eq!(page.section_index, 2);
    assert_eq!(page.page_number, 7);
    assert_eq!(page.note_body_bottom, Some(650.0));
    assert_eq!(page.lines.len(), 2);
    assert_eq!(page.lines[1].top, 200.0);
    assert_eq!(page.anchors["label"], [170.0, 200.0]);
    assert_eq!(
        page.operations
            .iter()
            .filter_map(|operation| match operation {
                PageOperation::Text(index) => Some(*index),
                _ => None,
            })
            .collect::<Vec<_>>(),
        [0, 1],
    );
    let plan = DocumentTextPlan {
        geometry,
        pages: vec![page],
        warnings: Vec::new(),
    };
    assert!(render::paint_pdf(&plan).unwrap().starts_with(b"%PDF-"));
    assert_eq!(
        render::paint_png(&plan, &Default::default())
            .unwrap()
            .output
            .len(),
        1
    );
}

#[test]
fn translating_content_preserves_pixels_with_clipping_rotation_and_images() {
    let content = prepared_content();
    let mut translated = TextPage::default();
    translated.append_content_at(content.clone(), [128.0, 128.0]);
    let geometry = PageGeometry {
        width: 256.0,
        height: 256.0,
        ..Default::default()
    };
    let plan = DocumentTextPlan {
        geometry,
        pages: vec![content, translated],
        warnings: Vec::new(),
    };
    let rendered = render::paint_png(&plan, &Default::default()).unwrap();
    let pixels: Vec<_> = rendered
        .output
        .iter()
        .map(|page| {
            let mut reader = png::Decoder::new(std::io::Cursor::new(&page.bytes))
                .read_info()
                .unwrap();
            let mut pixels = vec![0; reader.output_buffer_size().unwrap()];
            let frame = reader.next_frame(&mut pixels).unwrap();
            assert_eq!((frame.width, frame.height), (256, 256));
            assert_eq!(frame.color_type, png::ColorType::Rgba);
            pixels
        })
        .collect();
    for vertical in 0..128 {
        for horizontal in 0..128 {
            let original = (vertical * 256 + horizontal) * 4;
            let shifted = ((vertical + 128) * 256 + horizontal + 128) * 4;
            for channel in 0..4 {
                assert!(
                    pixels[0][original + channel].abs_diff(pixels[1][shifted + channel]) <= 1,
                    "translated pixel at ({horizontal}, {vertical}) changed"
                );
            }
        }
    }
}
