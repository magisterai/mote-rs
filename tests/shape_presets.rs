//! The geometry of every shape kind is evaluated from its DrawingML preset
//! definition, of which the library holds a copy.
use std::{fs, path::Path};

/// Returns the lists of a definition that give its geometry, each element by
/// its name, its attributes in order of name, and its children.
fn geometry(definition: roxmltree::Node<'_, '_>) -> Vec<String> {
    fn written(node: roxmltree::Node<'_, '_>) -> String {
        let mut attributes: Vec<_> = node
            .attributes()
            .map(|attribute| format!(" {}={:?}", attribute.name(), attribute.value()))
            .collect();
        attributes.sort();
        let children: String = node
            .children()
            .filter(roxmltree::Node::is_element)
            .map(written)
            .collect();
        format!(
            "<{}{}>{children}</>",
            node.tag_name().name(),
            attributes.concat()
        )
    }
    ["avLst", "gdLst", "cxnLst", "rect", "pathLst"]
        .into_iter()
        .filter_map(|name| {
            definition
                .children()
                .find(|node| node.has_tag_name(name))
                .filter(|node| name == "rect" || node.has_children())
                .map(written)
        })
        .collect()
}

#[test]
fn the_library_holds_the_definitions_of_the_standard_unchanged() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let held = fs::read_to_string(root.join("src/drawings/presets.xml")).unwrap();
    let standard = fs::read_to_string(root.join(
        "docs/reference/ooxml/ecma-376/part-1/drawingml-geometries/presetShapeDefinitions.xml",
    ))
    .unwrap();
    let held = roxmltree::Document::parse(&held).unwrap();
    let standard = roxmltree::Document::parse(standard.trim_start_matches('\u{feff}')).unwrap();
    let mut count = 0;
    for definition in held
        .root_element()
        .children()
        .filter(roxmltree::Node::is_element)
    {
        let name = definition.tag_name().name();
        // The copy of the standard lacks the up arrow, which the library
        // defines as the down arrow turned over.
        if name == "upArrow" {
            continue;
        }
        let original = standard
            .root_element()
            .children()
            .find(|node| node.tag_name().name() == name)
            .unwrap_or_else(|| panic!("the standard defines no {name}"));
        assert_eq!(geometry(definition), geometry(original), "{name}");
        count += 1;
    }
    assert!(count >= 10);
}
