//! A header or a footer taller than its margin. Word begins the body at the foot
//! of such a header and ends it at the top of such a footer, as its export of the
//! headers-and-footers probe shows, and omits no line of either story.
use mote::{layout::PlacedTextLine, render, source::FileAssetResolver};

fn text(placed: &PlacedTextLine) -> String {
    placed
        .line
        .fragments
        .iter()
        .map(|fragment| fragment.text.as_str())
        .collect()
}

#[test]
fn a_tall_header_and_footer_push_the_body_rather_than_lose_their_lines() {
    let body: String = (1..=30)
        .map(|index| format!("#paragraph[Body line {index}.]\n"))
        .collect();
    let source = format!(
        "#document(page={{width=pt(360),height=pt(300)}},margin={{top=pt(30),bottom=pt(30),left=pt(24),right=pt(24)}})\n\
         #header(distance=pt(12))[#paragraph[Header one]#paragraph[Header two]#paragraph[Header three]]\n\
         #footer(distance=pt(12))[#paragraph[Footer one]#paragraph[Footer two]]\n{body}"
    );
    let document = mote::compile(&source).unwrap();
    let resolver = FileAssetResolver::new(".").unwrap();
    let planned = render::plan(&document, &resolver, None).unwrap();
    assert_eq!(planned.warnings, Vec::<String>::new());
    for page in &planned.output.pages {
        let find = |prefix: &str| {
            page.lines
                .iter()
                .filter(|placed| text(placed).starts_with(prefix))
                .collect::<Vec<_>>()
        };
        let header = find("Header");
        let footer = find("Footer");
        let body = find("Body");
        assert_eq!((header.len(), footer.len()), (3, 2));
        let header_foot = header[2].top + header[2].line.height;
        assert!((body[0].top - header_foot).abs() < 0.01, "{header_foot}");
        // The space that the line multiple adds beneath the last line of a page
        // may cross the foot of the body.
        let last = body.last().unwrap();
        let body_foot = last.top + last.line.height - last.line.trailing_space;
        assert!(body_foot <= footer[0].top + 0.01, "{body_foot}");
    }
}
