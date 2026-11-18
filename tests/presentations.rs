use mote::{
    evaluation::EvaluationOptions,
    model::{Content, ElementKind},
    source::FileAssetResolver,
};
use serde_json::Value;
use sha2::{Digest, Sha256};
use std::{fs, path::Path};

fn normalize_numbers(value: &mut Value) {
    match value {
        Value::Number(number) => {
            *number = serde_json::Number::from_f64(number.as_f64().unwrap()).unwrap()
        }
        Value::Array(values) => values.iter_mut().for_each(normalize_numbers),
        Value::Object(fields) => fields.values_mut().for_each(normalize_numbers),
        _ => {}
    }
}

#[test]
fn presentation_declarations_slides_and_boxes_match_captured_models() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/presentations");
    let resolver = FileAssetResolver::new(&root).unwrap();
    for case in ["basics", "properties", "merging", "content", "lua"] {
        let path = root.join(format!("{case}.mote"));
        let source = fs::read_to_string(&path).unwrap();
        let folder = root.join("captured").join(case);
        let manifest: Value =
            serde_json::from_slice(&fs::read(folder.join("capture.json")).unwrap()).unwrap();
        assert_eq!(
            format!("{:x}", Sha256::digest(source.as_bytes())),
            manifest["source_sha256"]
        );
        let expected = fs::read(folder.join("model.json")).unwrap();
        assert_eq!(
            format!("{:x}", Sha256::digest(&expected)),
            manifest["model_sha256"]
        );
        let document = mote::compile_with_options(
            &source,
            EvaluationOptions {
                enable_lua: true,
                asset_resolver: Some(&resolver),
                source_name: Some(path.as_os_str()),
                ..Default::default()
            },
        )
        .unwrap();
        let mut actual: Value =
            serde_json::from_str(&mote::json::to_json(&document).unwrap()).unwrap();
        let mut expected: Value = serde_json::from_slice(&expected).unwrap();
        normalize_numbers(&mut actual);
        normalize_numbers(&mut expected);
        assert_eq!(actual, expected, "{case}");
        assert!(
            mote::docx::emit(&document).is_err(),
            "{case} has no document output mapping"
        );
        assert!(
            mote::render::plan_with_layout(
                &document,
                &resolver,
                None,
                mote::render::LayoutMode::Document
            )
            .is_err(),
            "{case} needs presentation layout"
        );
    }
}

#[test]
fn presentation_signatures_and_content_boundaries_are_explicit() {
    for source in [
        "#presentation('wide')",
        "#presentation()[Body]",
        "#presentation(size='a4')",
        "#presentation(size={width=-1})",
        "#presentation(width=-1)",
        "#presentation(theme=true)",
        "#presentation(background={fit='tile'})",
        "#presentation(footer=1)",
        "#presentation(width=true,properties={width=1})",
        "#slide",
        "#slide(notes=3)[Text]",
        "#box(position={width=1,height=1})",
        "#box[Text]",
        "#box(position={width=1})[Text]",
        "#box(position={width=1,height=1},alignment='middle')[Text]",
        "#box(position={width=1,height=1},text={fit='expand'})[Text]",
        "#slide[Text #shape(position={width=1,height=1})]",
        "#slide[Text #line(from={x=1,y=1},to={x=2,y=2})]",
        "#slide(layout='title')[Text]",
        "#box(slot='title')[Text]",
    ] {
        assert!(mote::compile(source).is_err(), "{source}");
    }
    assert!(
        mote::compile("#box(position={width=1,height=1})[Text #image(src='image.png')]").is_ok(),
        "a box holds text and an image together"
    );
    let document = mote::compile("#slide[] #slide[#box(position={width=1,height=1})[]]").unwrap();
    assert_eq!(document.warnings.len(), 2);
    let plain = mote::compile("#presentation()Plain text.").unwrap();
    assert!(plain.presentation.properties.is_empty());
    assert!(mote::docx::emit(&plain).is_ok());
}

#[test]
fn groups_hold_positioned_objects_and_nothing_else() {
    let document = mote::compile(
        "#slide[#group(name=7)[\
         #shape(position={width=pt(40),height=pt(20)})\
         #group[#line(from={x=pt(0),y=pt(0)},to={x=pt(10),y=pt(10)})]\
         #box(position={width=pt(40),height=pt(20)})[Text]]]",
    )
    .unwrap();
    let Content::Element(slide) = &document.content[0] else {
        panic!("a slide is an element");
    };
    let [Content::Element(group)] = slide.children.as_slice() else {
        panic!("the slide holds one group");
    };
    assert_eq!(group.kind, ElementKind::Group);
    assert!(group.is_block());
    assert_eq!(group.attributes["name"].to_text(), "7");
    let kinds: Vec<_> = group
        .children
        .iter()
        .map(|child| match child {
            Content::Element(element) => element.kind,
            Content::Text(_) => panic!("a group holds elements only"),
        })
        .collect();
    assert_eq!(
        kinds,
        [ElementKind::Shape, ElementKind::Group, ElementKind::Box]
    );

    let document = mote::compile(
        "#slide[\n#group[\n#box(position={width=pt(200),height=pt(200)})[\n\
         ## Heading\n\nA paragraph.\n\n- first\n- second\n\n> A quotation.\n]\n]\n]",
    )
    .unwrap();
    let Content::Element(slide) = &document.content[0] else {
        panic!("a slide is an element");
    };
    let [Content::Element(group)] = slide.children.as_slice() else {
        panic!("the slide holds one group");
    };
    let [Content::Element(text_box)] = group.children.as_slice() else {
        panic!("the group holds one box");
    };
    let blocks: Vec<_> = text_box
        .children
        .iter()
        .filter_map(|child| match child {
            Content::Element(element) => Some(element.kind),
            Content::Text(_) => None,
        })
        .collect();
    assert_eq!(
        blocks,
        [
            ElementKind::Heading,
            ElementKind::Paragraph,
            ElementKind::List,
            ElementKind::Blockquote
        ]
    );

    for (source, message) in [
        (
            "#slide[#group[#paragraph[Text]]]",
            "group accepts only boxes, shapes, lines and groups; text belongs in a box or a shape",
        ),
        (
            "#slide[#group[Loose text]]",
            "group accepts only boxes, shapes, lines and groups; text belongs in a box or a shape",
        ),
        (
            "#slide(layout='title')[#group[#box(slot='title')[Title]]]",
            "group cannot contain a box assigned to a layout slot",
        ),
        (
            "#slide[#group('Card')[#shape(position={width=pt(4),height=pt(4)})]]",
            "unexpected positional argument",
        ),
        (
            "#slide[Loose text.#group[#shape(position={width=pt(4),height=pt(4)})]]",
            "slide cannot mix positioned objects and loose content",
        ),
    ] {
        let error = mote::compile(source).unwrap_err().to_string();
        assert!(error.contains(message), "{source}: {error}");
    }
}

#[test]
fn effects_times_and_displacements_are_normalized_and_checked() {
    let source = "#presentation(render_at=1.5)\
         #slide(duration=4)[\
         #shape(position={width=pt(40),height=pt(20)},animate={\
         {effect='fade_in'},\
         {effect='move',by={x=pt(30)},start=1,duration=2,easing='ease_in_out'}})[At #time]\
         #group(position={x=pt(5),y=pt(-5)},rotation='15deg',animate={effect='rotate',by=90})[\
         #line(from={x=pt(0),y=pt(0)},to={x=pt(10),y=pt(10)},animate={effect='disappear',start=2})]]";
    let document = mote::compile(source).unwrap();
    assert_eq!(document.time, Some(1.5));
    let mut model: Value = serde_json::from_str(&mote::json::to_json(&document).unwrap()).unwrap();
    normalize_numbers(&mut model);
    assert_eq!(model["presentation"]["properties"]["render_at"], 1.5);
    let slide = &model["content"][0];
    assert_eq!(slide["attributes"]["duration"], 4.0);
    let shape = &slide["children"][0];
    assert_eq!(
        shape["attributes"]["animate"],
        serde_json::json!([
            {"effect": "fade_in"},
            {
                "effect": "move",
                "by": {"x": {"text": "30pt", "unit": "pt", "value": 30.0}},
                "start": 1.0,
                "duration": 2.0,
                "easing": "ease_in_out"
            }
        ])
    );
    assert_eq!(shape["children"][0]["children"][1]["text"], "1.5");
    let group = &slide["children"][1];
    assert_eq!(group["attributes"]["rotation"], 15.0);
    assert_eq!(group["attributes"]["position"]["y"]["value"], -5.0);
    assert_eq!(
        group["attributes"]["animate"],
        serde_json::json!([{"effect": "rotate", "by": 90.0}])
    );
    assert_eq!(
        group["children"][0]["attributes"]["animate"],
        serde_json::json!([{"effect": "disappear", "start": 2.0}])
    );

    let timed = |source: &str, time| {
        let document = mote::compile_with_options(
            source,
            EvaluationOptions {
                time,
                ..Default::default()
            },
        )
        .unwrap();
        (document.time, mote::json::to_json(&document).unwrap())
    };
    let (time, model) = timed(source, Some(3.0));
    assert_eq!(time, Some(3.0));
    assert!(model.contains(r#""text": "3""#), "{model}");
    let (time, model) = timed("The time is #time.", None);
    assert_eq!(time, None);
    assert!(model.contains(r#""text": "0""#), "{model}");
    assert!(
        !timed("The time is #time.", Some(2.0))
            .1
            .contains("presentation")
    );
    assert_eq!(timed("#presentation(render_at=0)", None).0, Some(0.0));
    assert_eq!(timed("#presentation(render_at=.5)", None).0, Some(0.5));

    let object = |arguments: &str| {
        format!("#slide[#shape(position={{width=pt(4),height=pt(4)}},{arguments})]")
    };
    for (source, message) in [
        (
            object("animate='fade_in'"),
            "animate must have collection type",
        ),
        (object("animate={effect='wipe'}"), "animate effect"),
        (
            object("animate={effect='fade_in',start=-1}"),
            "animate start must be a number of seconds, not negative",
        ),
        (
            object("animate={effect='fade_in',duration='1s'}"),
            "animate duration must be a number of seconds, not negative",
        ),
        (
            object("animate={effect='fade_in',easing='bounce'}"),
            "animate easing",
        ),
        (
            object("animate={effect='move'}"),
            "animate move requires by",
        ),
        (
            object("animate={effect='rotate',by={x=pt(1)}}"),
            "animate rotate by must be a number",
        ),
        (
            object("animate={effect='scale',to=0}"),
            "animate scale to must be a positive number",
        ),
        (
            "#slide(duration=-1)[]".to_owned(),
            "slide duration must be a number of seconds, not negative",
        ),
        (
            "#slide(advance_after='soon')[]".to_owned(),
            "slide advance_after must be a number of seconds, not negative",
        ),
        ("#presentation(render_at='end')".to_owned(), "render_at"),
        (
            "#slide(field_text={size=pt(-1)})[]".to_owned(),
            "field_text.size",
        ),
        (
            "#slide[#group(rotation='quarter')[#shape(position={width=pt(4),height=pt(4)})]]"
                .to_owned(),
            "group rotation must be a number of degrees",
        ),
    ] {
        let error = mote::compile(&source).unwrap_err().to_string();
        assert!(error.contains(message), "{source}: {error}");
    }
}
