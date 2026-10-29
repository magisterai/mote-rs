#[path = "support/json_compare.rs"]
mod json_compare;
use mote::layout::{
    FlowAdvance, FlowArea, FlowCursor, FlowOptions, FlowStep, LineFrame, PageGeometry,
    minimum_start_height, plan_paragraph_flow,
};
use serde_json::{Value, json};
use std::{fs, path::Path};
#[test]
fn paragraph_pagination_matches_captured_flow_cases() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/paragraph-flow");
    let requests: Value =
        serde_json::from_slice(&fs::read(root.join("requests.json")).unwrap()).unwrap();
    let expected: Value =
        serde_json::from_slice(&fs::read(root.join("expected.json")).unwrap()).unwrap();
    let mut cases = Vec::new();
    for case in requests["cases"].as_array().unwrap() {
        let heights: Vec<_> = case["heights"]
            .as_array()
            .unwrap()
            .iter()
            .map(|value| value.as_f64().unwrap())
            .collect();
        let geometry = PageGeometry {
            width: 300.0,
            height: 180.0,
            margin_top: 20.0,
            margin_bottom: 20.0,
            margin_left: 20.0,
            margin_right: 20.0,
            column_gap: 12.0,
            column_count: case["columns"].as_u64().unwrap() as usize,
            ..Default::default()
        };
        let mut cursor = FlowCursor {
            page_number: 1,
            current_top: 20.0,
            y: 20.0,
            ..Default::default()
        };
        match case["start"].as_str().unwrap() {
            "initial" => cursor = FlowCursor::default(),
            "middle" => {
                cursor.y = 80.0;
                cursor.body_started = true
            }
            "bottom" => {
                cursor.y = 150.0;
                cursor.body_started = true
            }
            _ => {}
        }
        let frames: Vec<_> = (0..heights.len())
            .map(|index| {
                if index == 0 {
                    LineFrame {
                        x: 5.0,
                        width: 50.0,
                    }
                } else {
                    LineFrame {
                        x: 0.0,
                        width: 60.0,
                    }
                }
            })
            .collect();
        let options = FlowOptions {
            leading_space: case["leading"].as_f64().unwrap(),
            keep_fit_leading_space: case["leading"].as_f64().unwrap(),
            minimum_start_space: case["minimum_start"].as_f64().unwrap(),
            keep_together: case["keep_together"].as_bool().unwrap(),
            widow_control: case["widow_control"].as_bool().unwrap(),
            ..Default::default()
        };
        let plan = plan_paragraph_flow(cursor, &heights, &frames, &options, |_| {
            FlowArea::from(geometry)
        })
        .unwrap();
        let cursor = plan.final_cursor;
        cases.push(json!({"id":case["id"],"minimum_start_height":minimum_start_height(&heights,options.widow_control,options.keep_together),"final_cursor":{"page_number":cursor.page_number,"column_index":cursor.column_index,"current_top":cursor.current_top,"y":cursor.y,"body_started":cursor.body_started},"steps":plan.steps.iter().map(|step|match step{FlowStep::Advance(advance)=>json!({"advance":match advance{FlowAdvance::AddPage=>1,FlowAdvance::NextColumn=>2,FlowAdvance::NextPage=>3}}),FlowStep::Line(line)=>json!({"line_index":line.line_index,"page_number":line.page_number,"column_index":line.column_index,"x":line.frame.x,"width":line.frame.width,"top":line.top,"height":line.height})}).collect::<Vec<_>>()}));
    }
    assert_eq!(cases.len(), 672);
    let actual = json!({"schema_version":1,"cases":cases});
    let mut differences = Vec::new();
    json_compare::compare(&expected, &actual, "root", &mut differences);
    assert!(
        differences.is_empty(),
        "{} differences:\n{}",
        differences.len(),
        differences
            .iter()
            .take(30)
            .cloned()
            .collect::<Vec<_>>()
            .join("\n")
    );
}
#[test]
fn invalid_flow_areas_and_line_geometry_fail_without_looping() {
    let heights = [10.0];
    let frames = [LineFrame {
        x: 0.0,
        width: 60.0,
    }];
    let options = FlowOptions::default();
    let cursor = FlowCursor::default();
    assert!(
        plan_paragraph_flow(cursor, &heights, &[], &options, |_| FlowArea::from(
            PageGeometry::default()
        ))
        .is_err()
    );
    assert!(
        plan_paragraph_flow(cursor, &[f64::NAN], &frames, &options, |_| FlowArea::from(
            PageGeometry::default()
        ))
        .is_err()
    );
    assert!(
        plan_paragraph_flow(cursor, &heights, &frames, &options, |_| FlowArea {
            content_bottom: 0.0,
            ..FlowArea::from(PageGeometry::default())
        })
        .is_err()
    );
    let options = FlowOptions {
        minimum_start_space: 630.0,
        ..Default::default()
    };
    assert!(
        plan_paragraph_flow(cursor, &heights, &frames, &options, |_| FlowArea {
            page_start: 100.0,
            ..FlowArea::from(PageGeometry::default())
        })
        .is_err()
    );
    let options = FlowOptions {
        widow_control: true,
        ..Default::default()
    };
    assert!(
        plan_paragraph_flow(cursor, &[630.0], &frames, &options, |_| FlowArea {
            page_start: 100.0,
            ..FlowArea::from(PageGeometry::default())
        })
        .is_err()
    );
    let geometry = PageGeometry {
        width: f64::MAX,
        margin_left: f64::MAX * 0.75,
        ..Default::default()
    };
    let frame = LineFrame {
        x: f64::MAX * 0.75,
        width: 10.0,
    };
    assert!(
        plan_paragraph_flow(cursor, &[10.0], &[frame], &FlowOptions::default(), |_| {
            geometry.into()
        })
        .is_err()
    );
}
