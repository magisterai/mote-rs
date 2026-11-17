//! A figure keeps its content and its caption on one page in native and
//! editable output, and a paragraph kept with the next block shares the page of
//! the chart, the table or the picture after it.
use mote::{
    layout::{DocumentTextPlan, PageOperation},
    render,
    source::FileAssetResolver,
};
use std::{
    io::{Cursor, Read},
    path::{Path, PathBuf},
};

const PICTURE_FIGURE: &str = r#"#figure(numbering = { enabled = true })[
  #image("photo.png", width = inch(3), alignment = "center")
  #caption[A caption below the picture.]
]
"#;
const TABLE_FIGURE: &str = r#"#figure(kind = "table", numbering = { enabled = true })[
  #caption(position = "above")[A caption above the table.]
  #table[
    #row[#cell[Alpha] #cell[Beta]]
    #row[#cell[Gamma] #cell[Delta]]
  ]
]
"#;
const CAPTIONED_TABLE_FIGURE: &str = r#"#figure(kind = "table", numbering = { enabled = true })[
  #table[
    #row[#cell[Alpha] #cell[Beta]]
    #row[#cell[Gamma] #cell[Delta]]
  ]
  #caption[A caption below the table.]
]
"#;

fn assets() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/skill")
}

/// Returns a document of `count` one-line paragraphs, the figure, and a
/// closing paragraph.
fn document_with_preceding_lines(count: usize, figure: &str) -> String {
    let mut source = String::new();
    for index in 1..=count {
        source.push_str(&format!("Preceding line {index}.\n\n"));
    }
    source.push_str(figure);
    source.push_str("\nThe text after the figure.\n");
    source
}

fn planned(source: &str) -> DocumentTextPlan {
    let document = mote::compile(source).unwrap();
    let resolver = FileAssetResolver::new(assets()).unwrap();
    let rendered = render::plan_with_fonts(&document, &resolver, None, None).unwrap();
    // The text of a chart is set in Aptos, which no font resolver supplies here.
    let warnings: Vec<_> = rendered
        .warnings
        .iter()
        .filter(|warning| !warning.starts_with("font \"Aptos\" regular was substituted"))
        .collect();
    assert!(
        warnings.is_empty() && rendered.output.warnings.is_empty(),
        "{warnings:?} {:?}",
        rendered.output.warnings
    );
    rendered.output
}

fn document_xml(source: &str) -> String {
    let document = mote::compile(source).unwrap();
    let resolver = FileAssetResolver::new(assets()).unwrap();
    let package = mote::docx::emit_with_assets(&document, &resolver, None).unwrap();
    let mut archive = zip::ZipArchive::new(Cursor::new(package.bytes)).unwrap();
    let mut xml = String::new();
    archive
        .by_name("word/document.xml")
        .unwrap()
        .read_to_string(&mut xml)
        .unwrap();
    xml
}

/// Returns the number of each page that holds a line containing `text`.
fn pages_with_text(plan: &DocumentTextPlan, text: &str) -> Vec<usize> {
    let wanted: String = text.split_whitespace().collect();
    plan.pages
        .iter()
        .enumerate()
        .filter(|(_, page)| {
            page.lines.iter().any(|placed| {
                let line: String = placed
                    .line
                    .fragments
                    .iter()
                    .flat_map(|fragment| fragment.text.split_whitespace())
                    .collect();
                line.contains(&wanted)
            })
        })
        .map(|(index, _)| index + 1)
        .collect()
}

/// Returns the number of the page of each picture, in page order.
fn pages_with_pictures(plan: &DocumentTextPlan) -> Vec<usize> {
    plan.pages
        .iter()
        .enumerate()
        .flat_map(|(index, page)| {
            page.operations
                .iter()
                .filter(|operation| matches!(operation, PageOperation::Image(_)))
                .map(move |_| index + 1)
        })
        .collect()
}

/// Returns the paragraphs of the body, each as its XML text.
fn paragraphs(xml: &str) -> Vec<&str> {
    xml.split("<w:p>")
        .skip(1)
        .map(|rest| rest.split("</w:p>").next().unwrap())
        .collect()
}

#[test]
fn a_picture_and_the_caption_below_it_share_a_page() {
    let mut moved_together = false;
    for count in 1..=40 {
        let plan = planned(&document_with_preceding_lines(count, PICTURE_FIGURE));
        let pictures = pages_with_pictures(&plan);
        let captions = pages_with_text(&plan, "A caption below the picture.");
        assert_eq!(pictures.len(), 1, "{count} preceding lines");
        assert_eq!(
            pictures, captions,
            "{count} preceding lines: the picture and its caption are on different pages"
        );
        let last_line = pages_with_text(&plan, &format!("Preceding line {count}."));
        moved_together |= last_line == [1] && pictures == [2];
    }
    assert!(
        moved_together,
        "no count of preceding lines put the figure at the foot of the first page"
    );
}

#[test]
fn a_caption_and_the_table_below_it_share_a_page() {
    let mut moved_together = false;
    for count in 1..=50 {
        let plan = planned(&document_with_preceding_lines(count, TABLE_FIGURE));
        let captions = pages_with_text(&plan, "A caption above the table.");
        let first_rows = pages_with_text(&plan, "Alpha");
        assert_eq!(captions.len(), 1, "{count} preceding lines");
        assert_eq!(
            captions, first_rows,
            "{count} preceding lines: the caption and the first row are on different pages"
        );
        let last_line = pages_with_text(&plan, &format!("Preceding line {count}."));
        moved_together |= last_line == [1] && captions == [2];
    }
    assert!(moved_together);
}

#[test]
fn a_table_and_the_caption_below_it_share_a_page() {
    for count in 1..=50 {
        let plan = planned(&document_with_preceding_lines(
            count,
            CAPTIONED_TABLE_FIGURE,
        ));
        let captions = pages_with_text(&plan, "A caption below the table.");
        assert_eq!(captions.len(), 1, "{count} preceding lines");
        assert_eq!(
            pages_with_text(&plan, "Alpha"),
            captions,
            "{count} preceding lines"
        );
        assert_eq!(
            pages_with_text(&plan, "Gamma"),
            captions,
            "{count} preceding lines"
        );
    }
}

const CHART: &str = r#"#chart(kind = "column", width = inch(5), height = inch(2), legend = false, data = { categories = { "North", "South" }, series = { { name = "Cover", values = { 12, 31 } } } })
"#;
const TABLE: &str =
    "#table[\n  #row[#cell[Alpha] #cell[Beta]]\n  #row[#cell[Gamma] #cell[Delta]]\n]\n";
const PICTURE: &str = "#image(\"photo.png\", width = inch(3))\n";

#[test]
fn a_paragraph_kept_with_the_next_block_shares_the_page_of_that_block() {
    for (kept, block) in [
        (
            "#paragraph(keep_with_next = true)[The kept paragraph.]",
            CHART,
        ),
        (
            "#paragraph(keep_with_next = true)[The kept paragraph.]",
            TABLE,
        ),
        (
            "#paragraph(keep_with_next = true)[The kept paragraph.]",
            PICTURE,
        ),
        ("## The kept paragraph.", CHART),
        ("## The kept paragraph.", TABLE),
        ("## The kept paragraph.", PICTURE),
    ] {
        let mut moved_together = false;
        for count in 1..=50 {
            let plan = planned(&document_with_preceding_lines(
                count,
                &format!("{kept}\n\n{block}"),
            ));
            let paragraph = pages_with_text(&plan, "The kept paragraph.");
            let block_start = if block == CHART {
                pages_with_text(&plan, "North")
            } else if block == TABLE {
                pages_with_text(&plan, "Alpha")
            } else {
                pages_with_pictures(&plan)
            };
            assert_eq!(
                paragraph, block_start,
                "{count} preceding lines before {kept} and {block}"
            );
            let last_line = pages_with_text(&plan, &format!("Preceding line {count}."));
            moved_together |= last_line == [1] && paragraph == [2];
        }
        assert!(moved_together, "{kept} and {block}");
    }
}

#[test]
fn a_paragraph_that_is_not_kept_leaves_the_next_block_to_move_alone() {
    for block in [CHART, TABLE, PICTURE] {
        let separated = (1..=50).any(|count| {
            let plan = planned(&document_with_preceding_lines(
                count,
                &format!("#paragraph(keep_with_next = false)[The free paragraph.]\n\n{block}"),
            ));
            pages_with_text(&plan, "The free paragraph.") == [1]
                && pages_with_text(&plan, "The text after the figure.") == [2]
        });
        assert!(separated, "{block}");
    }
}

#[test]
fn a_block_outside_a_figure_is_not_kept_with_the_block_after_it() {
    let source = |count| {
        document_with_preceding_lines(
            count,
            "#image(\"photo.png\", width = inch(3))\n\nA paragraph below the picture.\n",
        )
    };
    let separated = (1..=40).any(|count| {
        let plan = planned(&source(count));
        pages_with_pictures(&plan) == [1]
            && pages_with_text(&plan, "A paragraph below the picture.") == [2]
    });
    assert!(separated);
}

#[test]
fn a_figure_taller_than_a_page_keeps_all_its_content() {
    let pictures = r#"#figure(numbering = { enabled = true })[
  #image("photo.png", width = inch(6))
  #image("photo.png", width = inch(6))
  #image("photo.png", width = inch(6))
  #image("photo.png", width = inch(6))
  #caption[A caption below four pictures.]
]
"#;
    for count in [0, 1, 20, 38] {
        let plan = planned(&document_with_preceding_lines(count, pictures));
        assert_eq!(pages_with_pictures(&plan).len(), 4, "{count}");
        assert_eq!(
            pages_with_text(&plan, "A caption below four pictures.").len(),
            1,
            "{count}"
        );
        assert_eq!(
            pages_with_text(&plan, "The text after the figure.").len(),
            1,
            "{count}"
        );
        assert!(plan.pages.len() <= 5, "{count}: {}", plan.pages.len());
    }

    let mut rows = String::new();
    for index in 1..=80 {
        rows.push_str(&format!(
            "    #row[#cell[Row {index} begins] #cell[ends]]\n"
        ));
    }
    let table = format!(
        "#figure(kind = \"table\", numbering = {{ enabled = true }})[\n  #table[\n{rows}  ]\n  #caption[A caption below a long table.]\n]\n"
    );
    for count in [0, 20, 38] {
        let plan = planned(&document_with_preceding_lines(count, &table));
        for index in 1..=80 {
            assert_eq!(
                pages_with_text(&plan, &format!("Row {index} begins")).len(),
                1,
                "{count} preceding lines, row {index}"
            );
        }
        assert_eq!(
            pages_with_text(&plan, "A caption below a long table.").len(),
            1
        );
        assert!(plan.pages.len() <= 5, "{count}: {}", plan.pages.len());
    }
}

#[test]
fn the_package_keeps_a_picture_with_the_caption_below_it() {
    let xml = document_xml(&document_with_preceding_lines(1, PICTURE_FIGURE));
    let paragraphs = paragraphs(&xml);
    assert_eq!(paragraphs.len(), 4);
    let kept: Vec<_> = paragraphs
        .iter()
        .map(|paragraph| paragraph.contains("<w:keepNext/>"))
        .collect();
    assert!(paragraphs[1].contains("<w:drawing>"));
    assert!(paragraphs[2].contains("A caption below the picture."));
    assert_eq!(kept, [false, true, false, false]);
    assert!(paragraphs[1].starts_with("<w:pPr><w:keepNext/>"));
}

#[test]
fn the_package_keeps_a_caption_with_the_table_below_it() {
    let xml = document_xml(&document_with_preceding_lines(1, TABLE_FIGURE));
    assert_eq!(xml.matches("<w:keepNext/>").count(), 1);
    let caption = paragraphs(&xml)
        .into_iter()
        .find(|paragraph| paragraph.contains("A caption above the table."))
        .unwrap();
    assert!(caption.contains("<w:keepNext/>"));
    let table = &xml[xml.find("<w:tbl>").unwrap()..xml.find("</w:tbl>").unwrap()];
    assert!(!table.contains("<w:keepNext/>"));
}

#[test]
fn the_package_keeps_each_row_of_a_table_with_the_caption_below_it() {
    let xml = document_xml(&document_with_preceding_lines(1, CAPTIONED_TABLE_FIGURE));
    let table = &xml[xml.find("<w:tbl>").unwrap()..xml.find("</w:tbl>").unwrap()];
    let cell_paragraphs = paragraphs(table);
    assert_eq!(cell_paragraphs.len(), 4);
    assert!(
        cell_paragraphs
            .iter()
            .all(|paragraph| paragraph.contains("<w:keepNext/>")),
        "{table}"
    );
    assert_eq!(xml.matches("<w:keepNext/>").count(), 4);
    let caption = paragraphs(&xml)
        .into_iter()
        .find(|paragraph| paragraph.contains("A caption below the table."))
        .unwrap();
    assert!(!caption.contains("<w:keepNext/>"));
}

#[test]
fn the_package_keeps_nothing_outside_a_figure() {
    let xml = document_xml(
        "A paragraph.\n\n#image(\"photo.png\", width = inch(3))\n\n#table[\n  #row[#cell[Alpha]]\n]\n\n#caption[A caption alone.]\n",
    );
    assert!(!xml.contains("keepNext"));
}
