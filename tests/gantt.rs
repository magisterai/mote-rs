//! The Gantt component of the library draws rows of dated tasks as one group,
//! set in the text of a document or placed on a slide.
use mote::{
    evaluation::EvaluationOptions,
    model::{Content, Element, ElementKind, Properties, Value},
    render,
    source::FileAssetResolver,
};

const ROWS: &str = r##"{
  { task = "Discovery", start = "2026-01-05", ["end"] = "2026-01-23" },
  { task = "Design", start = date("2026-01-19"), finish = "2026-02-20", color = "#7c3aed" },
  { task = "Sign-off", start = "2026-02-20" },
  { name = "Build", start = "2026-02-16", ["end"] = "2026-04-24" }
}"##;

fn compiled(source: &str) -> Result<mote::model::Document, mote::diagnostics::Diagnostic> {
    mote::compile_with_options(
        &format!("#import(\"components/gantt\")\n{source}"),
        EvaluationOptions {
            enable_lua: true,
            ..Default::default()
        },
    )
}

fn element(content: &Content) -> &Element {
    let Content::Element(element) = content else {
        panic!("element")
    };
    element
}

fn text(properties: &Properties, name: &str) -> String {
    properties.get(name).map(Value::to_text).unwrap_or_default()
}

/// Returns the shapes of a group in their order.
fn shapes(group: &Element) -> Vec<&Element> {
    group.children.iter().map(element).collect()
}

/// Returns the text that a shape holds.
fn label_of(shape: &Element) -> String {
    shape.children.iter().map(Content::to_text).collect()
}

fn position(element: &Element) -> &Properties {
    let Some(Value::Object(position)) = element.attributes.get("position") else {
        panic!("position")
    };
    position
}

/// Returns the warnings of native output, less those of a substituted font.
fn native_warnings(document: &mote::model::Document) -> Vec<String> {
    let resolver = FileAssetResolver::new(".").unwrap();
    let rendered = render::plan(document, &resolver, None).unwrap();
    rendered
        .warnings
        .into_iter()
        .chain(rendered.output.warnings)
        .filter(|warning| !warning.contains("was substituted"))
        .collect()
}

#[test]
fn a_gantt_of_a_document_is_a_group_set_in_the_text_at_its_size() {
    let document = compiled(&format!(
        "Before.\n\n#gantt({ROWS}, width = inch(6), height = inch(2), today = \"2026-03-10\", alignment = \"center\")\n\nAfter."
    ))
    .unwrap();
    let group = element(&document.content[1]);
    assert_eq!(group.kind, ElementKind::Group);
    assert_eq!(text(&group.attributes, "name"), "Gantt");
    assert_eq!(text(&group.attributes, "alignment"), "center");
    let frame = position(group);
    assert_eq!(text(frame, "width"), "432pt");
    assert_eq!(text(frame, "height"), "144pt");
    assert!(!frame.contains_key("x") && !frame.contains_key("y"));

    let shapes = shapes(group);
    assert!(shapes.iter().all(|shape| shape.kind == ElementKind::Shape));
    for label in ["Discovery", "Design", "Sign-off", "Build", "5 Jan"] {
        assert_eq!(
            shapes
                .iter()
                .filter(|shape| label_of(shape) == label)
                .count(),
            1,
            "{label}"
        );
    }
    let of_kind = |kind: &str| {
        shapes
            .iter()
            .filter(|shape| text(&shape.attributes, "kind") == kind)
            .collect::<Vec<_>>()
    };
    // Three rows state an end and are bars; the row without one is a milestone.
    let bars = of_kind("round_rect");
    assert_eq!(bars.len(), 3);
    assert_eq!(of_kind("diamond").len(), 1);
    assert_eq!(text(&bars[0].attributes, "fill"), "#2563eb");
    assert_eq!(text(&bars[1].attributes, "fill"), "#7c3aed");
    // The line of the stated day is the one red rule.
    assert_eq!(
        shapes
            .iter()
            .filter(|shape| text(&shape.attributes, "fill") == "#dc2626")
            .count(),
        1
    );

    let warnings = native_warnings(&document);
    assert!(warnings.is_empty(), "{warnings:?}");
    let resolver = FileAssetResolver::new(".").unwrap();
    let package = mote::docx::emit_with_assets(&document, &resolver, None).unwrap();
    assert!(package.warnings.is_empty(), "{:?}", package.warnings);
}

#[test]
fn a_gantt_takes_a_size_from_its_rows_and_a_place_on_a_slide() {
    let document = compiled(&format!("#gantt({ROWS})")).unwrap();
    let frame = position(element(&document.content[0]));
    assert_eq!(text(frame, "width"), "468pt");
    // A header of 28 points and 24 points for each of the four rows.
    assert_eq!(text(frame, "height"), "124pt");

    let deck = compiled(&format!(
        "#presentation(size = \"wide\")\n#slide[#gantt({ROWS}, x = inch(1), y = inch(2), width = inch(10), height = inch(4))]"
    ))
    .unwrap();
    let slide = element(&deck.content[0]);
    let frame = position(element(&slide.children[0]));
    assert_eq!(text(frame, "x"), "1in");
    assert_eq!(text(frame, "y"), "2in");
    let warnings = native_warnings(&deck);
    assert!(warnings.is_empty(), "{warnings:?}");
    let resolver = FileAssetResolver::new(".").unwrap();
    let package = mote::pptx::emit_with_assets(&deck, &resolver, None).unwrap();
    assert!(package.warnings.is_empty(), "{:?}", package.warnings);
}

#[test]
fn a_gantt_divides_its_axis_by_the_span_of_its_rows() {
    for (start, end, labels) in [
        // Days, with the month named above the tasks.
        ("2026-03-02", "2026-03-06", &["Mar 2026", "2", "6"][..]),
        // Weeks, each from its Monday.
        ("2026-01-07", "2026-03-27", &["5 Jan", "2 Feb"][..]),
        // Months, with the year on the first.
        ("2026-01-05", "2026-11-20", &["Jan 2026", "Mar"][..]),
        // Quarters.
        ("2024-02-01", "2027-11-30", &["Q1 24", "Q1 25"][..]),
        // Years.
        ("2020-06-01", "2031-03-01", &["2020", "2031"][..]),
    ] {
        let document = compiled(&format!(
            "#gantt({{ {{ task = \"Work\", start = \"{start}\", [\"end\"] = \"{end}\" }} }}, width = inch(12))"
        ))
        .unwrap();
        let group = element(&document.content[0]);
        for label in labels {
            assert!(
                shapes(group).iter().any(|shape| label_of(shape) == *label),
                "{start} to {end}: {label}"
            );
        }
    }
}

#[test]
fn a_gantt_rejects_rows_that_it_cannot_draw() {
    for (rows, message) in [
        ("{}", "gantt requires at least one row"),
        (r#"{ { task = "Work" } }"#, "gantt row 1 has no start"),
        (
            r#"{ { task = "Work", start = "2026-02-01", ["end"] = "2026-01-01" } }"#,
            "gantt row 1 ends before it starts",
        ),
    ] {
        let error = compiled(&format!("#gantt({rows})")).unwrap_err();
        assert!(error.message.contains(message), "{}", error.message);
    }
}
