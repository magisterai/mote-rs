#[path = "support/language_capture.rs"]
mod language_capture;
use language_capture::xml_support as package_support;

#[test]
fn annotations_preserve_complete_packages_and_native_text_in_every_story() {
    language_capture::compare(
        "annotations",
        &[
            "document",
            "stories",
            "slides",
            "contexts",
            "inline-boundaries",
        ],
    );
}

#[test]
fn authored_comment_ids_do_not_conflict_with_package_ids() {
    let parts = package_support::package(
        r#"#comment(id="same",author="Ada",date=date("2024-02-29"))[First] #comment(id="same")[Second]"#,
    );
    let comments = roxmltree::Document::parse(&parts["word/comments.xml"]).unwrap();
    let namespace = "http://schemas.openxmlformats.org/wordprocessingml/2006/main";
    let identifiers: Vec<_> = comments
        .descendants()
        .filter(|node| node.has_tag_name((namespace, "comment")))
        .map(|node| node.attribute((namespace, "id")).unwrap())
        .collect();
    assert_eq!(identifiers, ["0", "1"]);
    assert!(parts["word/comments.xml"].contains("2024-02-29T00:00:00Z"));
    let main = roxmltree::Document::parse(&parts["word/document.xml"]).unwrap();
    for identifier in identifiers {
        for tag in ["commentRangeStart", "commentRangeEnd", "commentReference"] {
            assert_eq!(
                main.descendants()
                    .filter(|node| node.has_tag_name((namespace, tag))
                        && node.attribute((namespace, "id")) == Some(identifier))
                    .count(),
                1
            );
        }
    }
}

#[test]
fn task_and_standalone_controls_share_unique_identifiers() {
    let parts = package_support::package("#checkbox()\n\n- [x] Task\n\n#checkbox(checked=true)");
    let xml = roxmltree::Document::parse(&parts["word/document.xml"]).unwrap();
    let namespace = "http://schemas.openxmlformats.org/wordprocessingml/2006/main";
    let identifiers: Vec<_> = xml
        .descendants()
        .filter(|node| {
            node.has_tag_name((namespace, "id"))
                && node
                    .parent()
                    .is_some_and(|parent| parent.has_tag_name((namespace, "sdtPr")))
        })
        .map(|node| node.attribute((namespace, "val")).unwrap())
        .collect();
    assert_eq!(identifiers, ["1000", "1001", "1002"]);
}

#[test]
fn metadata_merges_without_losing_unmapped_values() {
    let document = mote::compile(
        r#"#metadata(title="First",author="Ada",company="Example") #metadata(title="Final") Text"#,
    )
    .unwrap();
    assert_eq!(document.metadata["title"].to_text(), "Final");
    assert_eq!(document.metadata["company"].to_text(), "Example");
    let parts = package_support::unpack(mote::docx::emit(&document).unwrap());
    assert!(parts["docProps/core.xml"].contains("<dc:title>Final</dc:title>"));
    assert!(parts["docProps/core.xml"].contains("<dc:creator>Ada</dc:creator>"));
    assert!(!parts["docProps/core.xml"].contains("Example"));
}

#[test]
fn annotation_argument_and_content_contracts_are_enforced() {
    for source in [
        r#"#checkbox(true)"#,
        r#"#checkbox(checked="yes")"#,
        r#"#checkbox(kind="toggle")"#,
        r#"#checkbox()[body]"#,
        r#"#field()"#,
        r#"#field(1)"#,
        r#"#field("PAGE")[body]"#,
        r#"#comment()"#,
        r#"#comment(author=true)[body]"#,
        r#"#comment(date="2023-02-29")[body]"#,
        r#"#metadata("title")"#,
        r#"#metadata()[body]"#,
    ] {
        assert!(mote::compile(source).is_err(), "{source}");
    }
}
