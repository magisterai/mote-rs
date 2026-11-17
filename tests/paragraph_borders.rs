//! The distance of a paragraph border from its text, and the shading within it.

use mote::{layout::PageOperation, render, source::FileAssetResolver};

const SOURCE: &str = "#paragraph(border={all={style='single',width=pt(2),color='#2563eb',space=SPACE}},fill='#eef4ff')[A boxed paragraph.]";

#[test]
fn a_border_space_is_a_length_of_at_most_thirty_one_points() {
    for space in ["8", "pt(8)", "inch(0.111)"] {
        let document = mote::compile(&SOURCE.replace("SPACE", space)).unwrap();
        let package = mote::docx::emit(&document).unwrap();
        let mut archive = zip::ZipArchive::new(std::io::Cursor::new(package)).unwrap();
        let mut xml = String::new();
        std::io::Read::read_to_string(&mut archive.by_name("word/document.xml").unwrap(), &mut xml)
            .unwrap();
        assert_eq!(xml.matches(r#"w:space="8""#).count(), 4, "{space}");
    }
    for space in ["pt(32)", "-1", "'wide'", "true"] {
        let error = mote::compile(&SOURCE.replace("SPACE", space))
            .unwrap_err()
            .to_string();
        assert!(
            error.contains("border space must be a length from 0 to 31 points"),
            "{space}: {error}"
        );
    }
}

#[test]
fn the_shading_of_a_paragraph_reaches_its_side_rules() {
    let resolver = FileAssetResolver::new("tests/fixtures/images/assets").unwrap();
    let shading = |space: &str| {
        let document = mote::compile(&SOURCE.replace("SPACE", space)).unwrap();
        let planned = render::plan(&document, &resolver, None).unwrap();
        assert_eq!(planned.warnings, Vec::<String>::new());
        planned.output.pages[0]
            .operations
            .iter()
            .find_map(|operation| match operation {
                PageOperation::Rectangle(rectangle) if rectangle.color == "EEF4FF" => {
                    Some([rectangle.x, rectangle.width])
                }
                _ => None,
            })
            .unwrap()
    };
    let [near, narrow] = shading("0");
    let [far, wide] = shading("pt(8)");
    // Each side rule is two points wide, and the shading meets its middle.
    assert!((near - far - 8.0).abs() < 0.001, "{near} {far}");
    assert!((wide - narrow - 16.0).abs() < 0.001, "{wide} {narrow}");
}
