//! Text that its frame cuts off is reported, and text that fits is not.

use mote::{
    layout::{DocumentTextPlan, PageOperation},
    render,
    source::FileAssetResolver,
};

const SHRINK_OR_ENLARGE: &str = "set text.fit = \"shrink\" or enlarge the frame";

fn resolver() -> FileAssetResolver {
    FileAssetResolver::new("tests/fixtures/images/assets").unwrap()
}

fn overflow_warnings(warnings: &[String]) -> Vec<String> {
    warnings
        .iter()
        .filter(|warning| {
            ["exceeds its", "cut off", "grow beyond"]
                .iter()
                .any(|phrase| warning.contains(phrase))
        })
        .cloned()
        .collect()
}

fn slide_plan(objects: &str) -> (DocumentTextPlan, Vec<String>) {
    let document = mote::compile(&format!("#presentation(size='wide')#slide[{objects}]")).unwrap();
    let planned = render::plan_with_layout(
        &document,
        &resolver(),
        None,
        render::LayoutMode::Presentation,
    )
    .unwrap();
    (planned.output, planned.warnings)
}

/// Returns the warnings of the native plan of one slide, having checked that
/// the editable package reports the same overflow.
fn slide_warnings(objects: &str) -> Vec<String> {
    let native = overflow_warnings(&slide_plan(objects).1);
    let document = mote::compile(&format!("#presentation(size='wide')#slide[{objects}]")).unwrap();
    let package = mote::pptx::emit_with_assets(&document, &resolver(), None).unwrap();
    assert_eq!(overflow_warnings(&package.warnings), native, "{objects}");
    native
}

fn document_warnings(source: &str) -> Vec<String> {
    let document = mote::compile(source).unwrap();
    let planned = render::plan(&document, &resolver(), None).unwrap();
    overflow_warnings(&planned.warnings)
}

fn text_box(arguments: &str, height: &str, text: &str) -> String {
    format!(
        "#box({arguments}position={{x=inch(1),y=inch(1),width=inch(4),height=inch({height})}})[{text}]"
    )
}

#[test]
fn a_slide_box_warns_once_where_its_text_passes_its_frame() {
    let title = "A slide title of several words that wraps";
    assert_eq!(
        slide_warnings(&text_box("name='Title',text={size=pt(60)},", "1", title)),
        [format!(
            "the text of content[0].slide[0].box \"Title\" exceeds its frame; {SHRINK_OR_ENLARGE}"
        )]
    );
    assert_eq!(
        slide_warnings(&text_box("text={size=pt(60)},", "1", title)),
        [format!(
            "the text of content[0].slide[0].box exceeds its frame; {SHRINK_OR_ENLARGE}"
        )]
    );
    for fitting in [
        text_box("text={size=pt(60)},", "5", title),
        text_box("text={size=pt(60),fit='shrink'},", "1", title),
        text_box("text={size=pt(60),fit='resize_shape'},", "1", title),
        text_box("text={size=pt(60)},", "1", "Title"),
    ] {
        assert_eq!(slide_warnings(&fitting), Vec::<String>::new(), "{fitting}");
    }
}

#[test]
fn a_word_wider_than_its_box_breaks_within_the_word_and_warns_where_it_passes_the_frame() {
    let word = "Supercalifragilisticexpialidocious";
    assert_eq!(
        slide_warnings(&text_box("text={size=pt(60)},", "1.5", word)),
        [format!(
            "the text of content[0].slide[0].box exceeds its frame; {SHRINK_OR_ENLARGE}"
        )]
    );
    assert_eq!(
        slide_warnings(&text_box("text={size=pt(60)},", "6", word)),
        Vec::<String>::new()
    );
}

#[test]
fn text_that_shrinking_cannot_fit_warns_without_proposing_to_shrink() {
    let text = "word ".repeat(400);
    assert_eq!(
        slide_warnings(&text_box("text={size=pt(60),fit='shrink'},", "0.2", &text)),
        [
            "the text of content[0].slide[0].box exceeds its frame at the least size of \
             text.fit = \"shrink\"; enlarge the frame or shorten the text"
        ]
    );
}

#[test]
fn a_slide_shape_warns_once_where_its_text_passes_its_frame() {
    let shape = |height: &str, arguments: &str, text: &str| {
        format!(
            "#box(position={{x=inch(8),y=inch(5),width=inch(1),height=inch(1)}})[Other]\
             #group[#shape(kind='rect',name='Badge',{arguments}\
             position={{x=inch(1),y=inch(1),width=inch(2),height=inch({height})}})[{text}]]"
        )
    };
    let text = "A statement too long for a badge";
    assert_eq!(
        slide_warnings(&shape("0.5", "text={size=pt(24)},", text)),
        [format!(
            "the text of content[0].slide[1].group[0].shape \"Badge\" exceeds its frame; \
             {SHRINK_OR_ENLARGE}"
        )]
    );
    for fitting in [
        shape("3", "text={size=pt(24)},", text),
        shape("0.5", "text={size=pt(24),fit='shrink'},", text),
        shape("0.5", "text={size=pt(24)},", "Badge"),
        shape("0.1", "", ""),
    ] {
        assert_eq!(slide_warnings(&fitting), Vec::<String>::new(), "{fitting}");
    }
}

fn slide_table(height: &str, header: &str) -> String {
    format!(
        "#box(name='Launchers',text={{size=pt(18)}},\
         position={{x=inch(1),y=inch(1),width=inch(8),height=inch({height})}})[\
         #table[#row[#cell[Vehicle]#cell[{header}]#cell[Year]#cell[Stages]]\
         #row[#cell[A]#cell[1]#cell[2]#cell[3]]]]"
    )
}

/// Returns the top and the height of each cell fill of the first slide.
fn cell_fills(plan: &DocumentTextPlan) -> Vec<[f64; 2]> {
    plan.pages[0]
        .operations
        .iter()
        .filter_map(|operation| match operation {
            PageOperation::Rectangle(rectangle) => Some([rectangle.top, rectangle.height]),
            _ => None,
        })
        .skip(1)
        .collect()
}

#[test]
fn a_slide_table_row_grows_to_hold_its_text_and_warns_where_the_table_passes_its_frame() {
    let header = "Payload to low Earth orbit (tonnes)";
    assert_eq!(
        slide_warnings(&slide_table("1", header)),
        [
            "the rows of the table in content[0].slide[0].box \"Launchers\" grow beyond its \
             frame to hold their text; enlarge the frame or shorten the text"
        ]
    );
    let fills = cell_fills(&slide_plan(&slide_table("1", header)).0);
    assert_eq!(fills.len(), 8);
    let [header_top, header_height] = fills[0];
    assert!(
        header_height > 36.0 + 20.0,
        "the header row holds more than one line: {header_height}"
    );
    for fill in &fills[..4] {
        assert_eq!(*fill, [header_top, header_height]);
    }
    for fill in &fills[4..] {
        assert_eq!(*fill, [header_top + header_height, 36.0]);
    }

    assert_eq!(
        slide_warnings(&slide_table("1", "Payload")),
        Vec::<String>::new()
    );
    let unchanged = cell_fills(&slide_plan(&slide_table("1", "Payload")).0);
    assert!(unchanged.iter().all(|fill| fill[1] == 36.0));
    assert_eq!(
        slide_warnings(&slide_table("4", header)),
        Vec::<String>::new()
    );
}

#[test]
fn a_slide_table_row_that_grows_within_the_frame_does_not_warn() {
    let table = "#box(text={size=pt(18)},\
                 position={x=inch(1),y=inch(1),width=inch(4),height=inch(3)})[\
                 #table[#row(height=pt(10))[#cell[One]#cell[Two]]\
                 #row(height=pt(10))[#cell[Three]#cell[Four]]]]";
    assert_eq!(slide_warnings(table), Vec::<String>::new());
    let fills = cell_fills(&slide_plan(table).0);
    assert!(fills.iter().all(|fill| fill[1] > 20.0), "{fills:?}");
}

#[test]
fn rotated_text_too_long_for_its_slide_table_cell_warns() {
    let table = |text: &str| {
        format!(
            "#box(text={{size=pt(18)}},\
             position={{x=inch(1),y=inch(1),width=inch(4),height=inch(1)}})[\
             #table[#row[#cell[One]#cell(text_direction='bt_lr')[{text}]]]]"
        )
    };
    assert_eq!(
        slide_warnings(&table(
            "Rotated text far too long for the height of its row"
        )),
        [
            "the text of row 1 column 2 of the table in content[0].slide[0].box exceeds its \
             cell; enlarge the cell or shorten the text"
        ]
    );
    assert_eq!(slide_warnings(&table("Up")), Vec::<String>::new());
}

#[test]
fn a_footer_too_long_for_its_native_frame_warns() {
    let warnings = |footer: &str| {
        let document = mote::compile(&format!(
            "#presentation(size='wide',footer='{footer}')#slide[#box(position={{x=inch(1),y=inch(1),width=inch(4),height=inch(1)}})[Body]]"
        ))
        .unwrap();
        let planned = render::plan_with_layout(
            &document,
            &resolver(),
            None,
            render::LayoutMode::Presentation,
        )
        .unwrap();
        overflow_warnings(&planned.warnings)
    };
    assert_eq!(
        warnings(&"A footer of very many words ".repeat(6)),
        [
            "the footer of content[0].slide exceeds its frame; shorten the text or reduce \
             field_text.size"
        ]
    );
    assert_eq!(warnings("Quarterly review"), Vec::<String>::new());
}

#[test]
fn a_document_shape_warns_once_where_its_text_passes_its_frame() {
    let shape = |arguments: &str, height: &str, text: &str| {
        format!(
            "Body.\n\n#shape(kind='rect',{arguments}\
             position={{x=inch(1),y=inch(2),width=inch(1.5),height=inch({height})}})[{text}]"
        )
    };
    let text = "A note far too long for the frame of the shape";
    assert_eq!(
        document_warnings(&shape("name='Note',", "0.4", text)),
        [format!(
            "the text of shape \"Note\" on page 1 exceeds its frame; {SHRINK_OR_ENLARGE}"
        )]
    );
    assert_eq!(
        document_warnings(&shape("", "0.4", text)),
        [format!(
            "the text of a shape on page 1 exceeds its frame; {SHRINK_OR_ENLARGE}"
        )]
    );
    for fitting in [
        shape("", "3", text),
        shape("text={fit='shrink'},", "0.4", text),
        shape("", "0.4", "Note"),
        shape("", "0.05", ""),
    ] {
        assert_eq!(
            document_warnings(&fitting),
            Vec::<String>::new(),
            "{fitting}"
        );
    }
}

#[test]
fn a_document_table_row_warns_once_where_its_exact_height_cuts_off_its_content() {
    let table = |row: &str, text: &str| {
        format!(
            "#table[#row[#cell[Head]#cell[Other]]\
             #row({row})[#cell(width=inch(1))[{text}]#cell[#table[#row[#cell[Inner]]]]]]"
        )
    };
    let text = "A cell of far too many words for a single line";
    assert_eq!(
        document_warnings(&table("height=pt(14),height_rule='exact'", text)),
        [
            "row 2 of the table on page 1 is cut off at its exact height; set height_rule = \
             \"at_least\" or a greater height"
        ]
    );
    for fitting in [
        table("height=pt(200),height_rule='exact'", text),
        table("height=pt(14),height_rule='at_least'", text),
        table("height=pt(14)", text),
        table("height=pt(40),height_rule='exact'", "Short"),
    ] {
        assert_eq!(
            document_warnings(&fitting),
            Vec::<String>::new(),
            "{fitting}"
        );
    }
    assert_eq!(
        document_warnings(
            "#table[#row[#cell[#table[#row(height=pt(4),height_rule='exact')[#cell[Inner]]]]]]"
        ),
        [
            "row 1 of a table nested in the table on page 1 is cut off at its exact height; set \
             height_rule = \"at_least\" or a greater height"
        ]
    );
}
