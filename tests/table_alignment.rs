//! Focused, editable cases assert formatting rules without frozen output files.
use mote::{
    layout::{DocumentTextPlan, PlacedTextLine, TextPage},
    model::Document,
    render::{self, LayoutMode},
    source::FileAssetResolver,
};
use std::{
    fs,
    io::{Cursor, Read},
    path::Path,
};

fn case(name: &str) -> (Document, DocumentTextPlan) {
    case_with_layout(name, LayoutMode::Presentation)
}

fn case_with_layout(name: &str, layout: LayoutMode) -> (Document, DocumentTextPlan) {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/cases");
    let path = root.join(format!("{name}.mote"));
    let resolver = FileAssetResolver::new(&root).unwrap();
    let document = mote::compile(&fs::read_to_string(&path).unwrap()).unwrap();
    let plan = render::plan_with_layout_and_fonts(
        &document,
        &resolver,
        Some(path.as_os_str()),
        layout,
        None,
    )
    .unwrap()
    .output;
    (document, plan)
}

fn line<'a>(page: &'a TextPage, expected: &str) -> &'a PlacedTextLine {
    page.lines
        .iter()
        .find(|placed| {
            placed
                .line
                .fragments
                .iter()
                .map(|fragment| fragment.text.as_str())
                .collect::<String>()
                == expected
        })
        .unwrap_or_else(|| panic!("missing text: {expected}"))
}

fn close(actual: f64, expected: f64, context: &str) {
    assert!(
        (actual - expected).abs() < 1e-8,
        "{context}: {actual} != {expected}"
    );
}

fn slide(document: &Document, number: usize) -> String {
    let bytes = mote::pptx::emit(document).unwrap();
    package_part(bytes, &format!("ppt/slides/slide{number}.xml"))
}

fn package_part(bytes: Vec<u8>, name: &str) -> String {
    let mut archive = zip::ZipArchive::new(Cursor::new(bytes)).unwrap();
    let mut output = String::new();
    archive
        .by_name(name)
        .unwrap()
        .read_to_string(&mut output)
        .unwrap();
    output
}

#[test]
fn markdown_delimiters_align_columns_in_documents_and_slides() {
    for layout in [LayoutMode::Document, LayoutMode::Presentation] {
        let (document, plan) = case_with_layout("markdown-table-alignment", layout);
        assert_eq!(plan.pages.len(), 1);
        for (column, names) in [
            ["Left", "Contract", "Invoice", "Credit"],
            ["Center", "Active", "Pending", "Applied"],
            ["Right", "$12,500.00", "$2,575.00", "-$125.00"],
        ]
        .iter()
        .enumerate()
        {
            let anchors: Vec<_> = names
                .iter()
                .map(|name| {
                    let placed = line(&plan.pages[0], name);
                    let start = placed.line.fragments.first().unwrap().x;
                    let last = placed.line.fragments.last().unwrap();
                    let end = last.x + last.advance;
                    match column {
                        0 => start,
                        1 => (start + end) / 2.0,
                        _ => end,
                    }
                })
                .collect();
            for (name, anchor) in names.iter().zip(&anchors) {
                close(*anchor, anchors[0], name);
            }
        }
        let (xml, namespace, property, attribute, alignments) = match layout {
            LayoutMode::Document => (
                package_part(mote::docx::emit(&document).unwrap(), "word/document.xml"),
                "http://schemas.openxmlformats.org/wordprocessingml/2006/main",
                "jc",
                "val",
                ["left", "center", "right"],
            ),
            _ => (
                slide(&document, 1),
                "http://schemas.openxmlformats.org/drawingml/2006/main",
                "pPr",
                "algn",
                ["l", "ctr", "r"],
            ),
        };
        let tree = roxmltree::Document::parse(&xml).unwrap();
        let cells: Vec<_> = tree
            .descendants()
            .filter(|node| node.has_tag_name((namespace, "tc")))
            .collect();
        assert_eq!(cells.len(), 12);
        for (index, cell) in cells.iter().enumerate() {
            let properties = cell
                .descendants()
                .find(|node| node.has_tag_name((namespace, property)))
                .unwrap();
            let actual = properties
                .attribute((namespace, attribute))
                .or_else(|| properties.attribute(attribute));
            assert_eq!(actual, Some(alignments[index % 3]));
        }
    }
}

#[test]
fn table_cells_and_explicit_paragraphs_resolve_alignment_in_the_same_order() {
    let (document, plan) = case("slide-table-alignment");
    assert_eq!(plan.pages.len(), 2);
    let page_names = [
        [
            ["Left top", "Center top", "Right top"],
            ["Left middle", "Center middle", "Right middle"],
            ["Left bottom", "Paragraph center", "Paragraph right"],
        ],
        [
            ["Left", "Center", "Right"],
            ["Contract", "Active", "$12,500.00"],
            ["Invoice", "Pending", "$2,575.00"],
        ],
    ];
    for (page_index, names) in page_names.iter().enumerate() {
        for (row, names) in names.iter().enumerate() {
            for (column, name) in names.iter().enumerate() {
                let placed = line(&plan.pages[page_index], name);
                let start = placed.line.fragments.first().unwrap().x;
                let end = placed
                    .line
                    .fragments
                    .last()
                    .map(|fragment| fragment.x + fragment.advance)
                    .unwrap();
                let left = 30.0 + column as f64 * 180.0 + 8.0;
                let right = left + 164.0;
                match column {
                    0 => close(start, left, name),
                    1 => close((start + end) / 2.0, (left + right) / 2.0, name),
                    _ => close(end, right, name),
                }
                let top = 80.0 + row as f64 * 80.0 + 8.0;
                let expected = top
                    + match (page_index, row) {
                        (1, _) | (_, 0) => 0.0,
                        (_, 1) => (64.0 - placed.line.height) / 2.0,
                        _ => 64.0 - placed.line.height,
                    };
                close(placed.top, expected, name);
            }
        }
    }
    for number in [1, 2] {
        let xml = slide(&document, number);
        let tree = roxmltree::Document::parse(&xml).unwrap();
        let cells: Vec<_> = tree
            .descendants()
            .filter(|node| {
                node.has_tag_name((
                    "http://schemas.openxmlformats.org/drawingml/2006/main",
                    "tc",
                ))
            })
            .collect();
        assert_eq!(cells.len(), 9);
        for (index, cell) in cells.iter().enumerate() {
            let paragraph = cell
                .descendants()
                .find(|node| {
                    node.has_tag_name((
                        "http://schemas.openxmlformats.org/drawingml/2006/main",
                        "pPr",
                    ))
                })
                .unwrap();
            assert_eq!(
                paragraph.attribute("algn"),
                Some(["l", "ctr", "r"][index % 3])
            );
        }
    }
}

#[test]
fn table_paragraph_spacing_contributes_to_top_middle_and_bottom_alignment() {
    let (document, plan) = case("slide-table-spacing");
    assert_eq!(plan.pages.len(), 2);
    for (page_index, page) in plan.pages.iter().enumerate() {
        for (column, name) in ["top", "middle", "bottom"].iter().enumerate() {
            let first = line(page, &format!("First {name}"));
            let second = line(page, &format!("Second {name}"));
            let gap = if page_index == 0 { 16.0 } else { 0.0 };
            close(second.top - first.top, first.line.height + gap, name);
            let height = first.line.height + second.line.height + 16.0 + gap;
            let offset = match column {
                0 => 0.0,
                1 => (220.0 - height) / 2.0,
                _ => 220.0 - height,
            };
            close(first.top, 90.0 + offset + 4.0, name);
        }
    }
    let xml = slide(&document, 1);
    assert!(xml.contains("<a:spcBef><a:spcPts val=\"400\"/></a:spcBef>"));
    assert!(xml.contains("<a:spcAft><a:spcPts val=\"1200\"/></a:spcAft>"));
}

#[test]
fn table_margin_shorthands_apply_to_defaults_direct_cells_and_explicit_zero() {
    let (document, plan) = case("slide-table-margins");
    assert_eq!(plan.pages.len(), 1);
    for (name, left, top) in [
        ("Inherited top", 46.0, 88.0),
        ("Direct top", 46.0, 208.0),
        ("Explicit zero", 390.0, 80.0),
        ("Zero left", 390.0, 206.0),
    ] {
        let placed = line(&plan.pages[0], name);
        close(placed.line.fragments[0].x, left, name);
        close(placed.top, top, name);
    }
    for (name, bottom) in [("Inherited bottom", 190.0), ("Direct bottom", 310.0)] {
        let placed = line(&plan.pages[0], name);
        let last = placed.line.fragments.last().unwrap();
        close(last.x + last.advance, 378.0, name);
        close(placed.top + placed.line.height, bottom, name);
    }
    let xml = slide(&document, 1);
    let tree = roxmltree::Document::parse(&xml).unwrap();
    let cells: Vec<_> = tree
        .descendants()
        .filter(|node| node.tag_name().name() == "tcPr")
        .collect();
    for index in [0, 1, 3, 4] {
        for (attribute, expected) in [
            ("marL", "203200"),
            ("marR", "152400"),
            ("marT", "101600"),
            ("marB", "127000"),
        ] {
            assert_eq!(cells[index].attribute(attribute), Some(expected));
        }
    }
    for attribute in ["marL", "marR", "marT", "marB"] {
        assert_eq!(cells[2].attribute(attribute), Some("0"));
    }
    assert_eq!(cells[5].attribute("marL"), Some("0"));
}
