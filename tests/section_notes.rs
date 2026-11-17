//! Native note placement in documents that hold explicit sections.
use mote::{
    layout::{DocumentTextPlan, PlacedTextLine, TextPage},
    render::{self, Rendered},
    source::FileAssetResolver,
};
use std::path::Path;

fn planned(source: &str) -> Rendered<DocumentTextPlan> {
    let document = mote::compile(source).unwrap();
    let resolver =
        FileAssetResolver::new(Path::new(env!("CARGO_MANIFEST_DIR")).join("tests")).unwrap();
    render::plan_with_fonts(&document, &resolver, None, None).unwrap()
}

fn line_text(line: &PlacedTextLine) -> String {
    line.line
        .fragments
        .iter()
        .map(|fragment| fragment.text.as_str())
        .collect()
}

fn line_containing<'page>(page: &'page TextPage, text: &str) -> &'page PlacedTextLine {
    page.lines
        .iter()
        .find(|line| line_text(line).contains(text))
        .unwrap_or_else(|| panic!("no line on the page contains {text:?}"))
}

fn page_holds(page: &TextPage, text: &str) -> bool {
    page.lines.iter().any(|line| line_text(line).contains(text))
}

/// Asserts that the line lies in the note area at the foot of the page: below
/// the reserved body boundary, and ending within one paragraph space of the
/// bottom margin.
fn assert_at_page_foot(page: &TextPage, line: &PlacedTextLine) {
    let content_bottom = page.geometry.unwrap().content_bottom();
    let body_bottom = page
        .note_body_bottom
        .expect("the page reserves a note area");
    let line_bottom = line.top + line.line.height;
    assert!(line.top > body_bottom, "{} > {body_bottom}", line.top);
    assert!(line_bottom <= content_bottom + 0.001);
    assert!(content_bottom - line_bottom < 20.0);
}

const LANDSCAPE_APPENDIX: &str = "A claim#footnote[A plain note.] in a paragraph.\n\n\
    #section(page = { orientation = \"landscape\" })[Appendix.]\n";

#[test]
fn a_note_before_an_explicit_section_sits_at_the_foot_of_its_page() {
    let plan = planned(LANDSCAPE_APPENDIX);
    assert_eq!(plan.warnings, Vec::<String>::new());
    assert_eq!(plan.output.pages.len(), 2);
    let first = &plan.output.pages[0];
    let reference = line_containing(first, "A claim");
    assert!(!line_text(reference).contains("A plain note."));
    assert_eq!(
        reference
            .line
            .fragments
            .iter()
            .filter_map(|fragment| fragment.footnote_number)
            .collect::<Vec<_>>(),
        [1]
    );
    let note = line_containing(first, "A plain note.");
    assert!(line_text(note).starts_with("1 "));
    assert_at_page_foot(first, note);
    assert!(!page_holds(&plan.output.pages[1], "A plain note."));
    assert!(plan.output.pages[1].note_body_bottom.is_none());
}

#[test]
fn a_single_section_document_places_the_same_note_at_the_same_position() {
    let with_section = planned(LANDSCAPE_APPENDIX);
    let alone = planned("A claim#footnote[A plain note.] in a paragraph.\n");
    assert_eq!(alone.warnings, Vec::<String>::new());
    let expected = line_containing(&alone.output.pages[0], "A plain note.");
    let actual = line_containing(&with_section.output.pages[0], "A plain note.");
    assert_eq!(actual, expected);
}

#[test]
fn a_note_in_a_landscape_section_uses_the_geometry_of_that_section() {
    let plan = planned(
        "Opening text.\n\n\
         #section(page = { orientation = \"landscape\" }, \
         margin = { left = inch(1.5), right = inch(1.5), bottom = inch(1.25) })[\n  \
         The appendix makes a claim#footnote[A note that is long enough to pass the text width \
         of a portrait page, so that the line shows the width of the landscape note area in \
         which the note is set.] of its own.\n]\n",
    );
    assert_eq!(plan.warnings, Vec::<String>::new());
    assert_eq!(plan.output.pages.len(), 2);
    let landscape = &plan.output.pages[1];
    let geometry = landscape.geometry.unwrap();
    assert!(geometry.width > geometry.height);
    assert_eq!(geometry.margin_left, 108.0);
    assert_eq!(geometry.margin_bottom, 90.0);
    let first_line = line_containing(landscape, "A note that is long");
    assert!(line_text(first_line).starts_with("1 "));
    assert_eq!(first_line.line.x, geometry.margin_left);
    let portrait_text_width = 612.0 - 144.0;
    assert!(first_line.line.width > portrait_text_width);
    assert!(first_line.line.width <= geometry.width - 216.0 + 0.001);
    let last_line = landscape
        .lines
        .iter()
        .rfind(|line| line.top > landscape.note_body_bottom.unwrap())
        .unwrap();
    assert_at_page_foot(landscape, last_line);
    assert!(plan.output.pages[0].note_body_bottom.is_none());
}

#[test]
fn footnote_numbering_continues_across_sections() {
    let plan = planned(
        "First#footnote[Note alpha.] and second#footnote[Note beta.].\n\n\
         #section(page = { orientation = \"landscape\" })[Third#footnote[Note gamma.].]\n\n\
         #section(page = { size = \"a5\" })[Fourth#footnote[Note delta.].]\n",
    );
    assert_eq!(plan.warnings, Vec::<String>::new());
    assert_eq!(plan.output.pages.len(), 3);
    for (page_index, marker, text) in [
        (0, "1 ", "Note alpha."),
        (0, "2 ", "Note beta."),
        (1, "3 ", "Note gamma."),
        (2, "4 ", "Note delta."),
    ] {
        let page = &plan.output.pages[page_index];
        let note = line_containing(page, text);
        assert!(line_text(note).starts_with(marker), "{text}");
        assert!(note.top > page.note_body_bottom.unwrap(), "{text}");
        assert_eq!(note.line.x, page.geometry.unwrap().margin_left, "{text}");
    }
    let first = &plan.output.pages[0];
    assert!(line_containing(first, "Note alpha.").top < line_containing(first, "Note beta.").top);
    assert_at_page_foot(first, line_containing(first, "Note beta."));
}

#[test]
fn continuous_sections_share_the_note_area_of_their_page() {
    let plan = planned(
        "First#footnote[Note alpha.].\n\n\
         #section(properties = { start = \"continuous\" })[Second#footnote[Note beta.].]\n",
    );
    assert_eq!(plan.warnings, Vec::<String>::new());
    assert_eq!(plan.output.pages.len(), 1);
    let page = &plan.output.pages[0];
    let first_note = line_containing(page, "Note alpha.");
    let second_note = line_containing(page, "Note beta.");
    assert!(line_text(second_note).starts_with("2 "));
    assert!(first_note.top < second_note.top);
    assert!(line_containing(page, "Second").top < page.note_body_bottom.unwrap());
    assert_at_page_foot(page, second_note);
}

#[test]
fn a_reference_in_a_section_moves_to_the_next_page_with_its_note() {
    let filler = "Filler line.\n\n".repeat(60);
    let plan = planned(&format!(
        "Opening.\n\n#section(page = {{ orientation = \"landscape\" }})[\n{filler}\
         Claim#footnote[Note one.]. Another#footnote[Note two.].\n\n{filler}]\n"
    ));
    assert_eq!(plan.warnings, Vec::<String>::new());
    let mut noted_pages = 0;
    for page in &plan.output.pages {
        let Some(body_bottom) = page.note_body_bottom else {
            continue;
        };
        noted_pages += 1;
        assert!(page.geometry.unwrap().width > page.geometry.unwrap().height);
        for line in &page.lines {
            let in_note_area = line.top > body_bottom;
            assert_eq!(
                in_note_area,
                line_text(line).contains("Note "),
                "{}",
                line_text(line)
            );
            if !in_note_area {
                assert!(line.top + line.line.height <= body_bottom + 0.001);
            }
        }
        assert_at_page_foot(page, line_containing(page, "Note two."));
    }
    assert_eq!(noted_pages, 1);
}

#[test]
fn endnotes_cited_in_several_sections_follow_the_last_section() {
    let plan = planned(
        "A source#endnote[Endnote alpha.] and a claim#footnote[Note alpha.].\n\n\
         #section(page = { orientation = \"landscape\" })[An archive#endnote[Endnote beta.].]\n",
    );
    assert_eq!(plan.warnings, Vec::<String>::new());
    assert_eq!(plan.output.pages.len(), 2);
    let first = &plan.output.pages[0];
    assert!(!page_holds(first, "Endnote alpha."));
    assert_at_page_foot(first, line_containing(first, "Note alpha."));
    let last = &plan.output.pages[1];
    let reference = line_containing(last, "An archive");
    let first_endnote = line_containing(last, "Endnote alpha.");
    let second_endnote = line_containing(last, "Endnote beta.");
    assert!(line_text(first_endnote).starts_with("i "));
    assert!(line_text(second_endnote).starts_with("ii "));
    assert!(reference.top < first_endnote.top);
    assert!(first_endnote.top < second_endnote.top);
    assert_eq!(first_endnote.line.x, last.geometry.unwrap().margin_left);
}

#[test]
fn a_flattened_note_names_the_context_that_cannot_hold_it() {
    for (source, kind, context, fallback) in [
        (
            "Body.\n\n#section[\n  #table[#row[#cell[Cell#footnote[Cell note.]]]]\n]\n",
            "footnote",
            "in a table cell",
            "Cell note.",
        ),
        (
            "Body.\n\n#section(columns = { count = 2 })[Column#footnote[Column note.].]\n",
            "footnote",
            "in a section with more than one column",
            "Column note.",
        ),
        (
            "Body#endnote[Late note.].\n\n#section(columns = { count = 2 })[Columns.]\n",
            "endnote",
            "after a last section with more than one column",
            "Late note.",
        ),
        (
            "Body.\n\n#section[\n  #figure[#paragraph[Figure#footnote[Figure note.]]]\n]\n",
            "footnote",
            "inside a figure",
            "Figure note.",
        ),
        (
            "#header[Head#footnote[Head note.]]\nBody.\n\n#section[Text.]\n",
            "footnote",
            "in a header",
            "Head note.",
        ),
        (
            "Body#footnote[Outer#footnote[Inner note.]].\n\n#section[Text.]\n",
            "footnote",
            "inside another note",
            "Inner note.",
        ),
    ] {
        let plan = planned(source);
        let expected_start = format!("native {kind} content is flattened inline because ");
        let warnings: Vec<_> = plan
            .warnings
            .iter()
            .filter(|warning| warning.contains("flattened inline"))
            .collect();
        assert_eq!(warnings.len(), 1, "{source}: {:?}", plan.warnings);
        assert!(warnings[0].starts_with(&expected_start), "{}", warnings[0]);
        assert!(warnings[0].ends_with(context), "{}", warnings[0]);
        // The enclosing note of the nested case is itself placed at the foot.
        if context != "inside another note" {
            assert!(
                plan.output
                    .pages
                    .iter()
                    .all(|page| page.note_body_bottom.is_none()),
                "{source}"
            );
        }
        assert!(
            plan.output
                .pages
                .iter()
                .any(|page| page_holds(page, fallback)),
            "{source}: the note text stays visible"
        );
    }
}
