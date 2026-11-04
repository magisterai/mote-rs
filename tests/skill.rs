//! The agent skill must stay correct: a model copies its examples literally.

use mote::{evaluation::EvaluationOptions, render, source::FileAssetResolver};
use std::{
    fs,
    path::{Path, PathBuf},
};

fn skill() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("skills/mote")
}

fn pages() -> Vec<PathBuf> {
    let mut pages = vec![skill().join("SKILL.md")];
    let mut references: Vec<_> = fs::read_dir(skill().join("references"))
        .unwrap()
        .map(|entry| entry.unwrap().path())
        .collect();
    references.sort();
    pages.extend(references);
    pages
}

/// Returns the targets of the page's relative Markdown links, outside code.
fn links(markdown: &str) -> Vec<String> {
    let prose: String = markdown
        .split("```")
        .step_by(2)
        .flat_map(|text| text.split('`').step_by(2))
        .collect();
    prose
        .split("](")
        .skip(1)
        .filter_map(|rest| rest.split_once(')').map(|(target, _)| target.to_owned()))
        .filter(|target| !target.contains("://"))
        .collect()
}

#[test]
fn the_frontmatter_names_the_skill_and_says_when_to_use_it() {
    let text = fs::read_to_string(skill().join("SKILL.md")).unwrap();
    let frontmatter = text
        .strip_prefix("---\n")
        .and_then(|rest| rest.split_once("\n---\n"))
        .map(|(frontmatter, _)| frontmatter)
        .expect("SKILL.md begins with YAML frontmatter");
    let field = |name: &str| {
        frontmatter
            .lines()
            .find_map(|line| line.strip_prefix(&format!("{name}: ")))
            .unwrap_or_else(|| panic!("frontmatter has no {name}"))
    };
    assert_eq!(field("name"), "mote", "the name matches the directory");
    let description = field("description");
    assert!(!description.is_empty() && description.chars().count() <= 1024);
}

#[test]
fn the_index_lists_every_page_and_every_link_resolves() {
    let index = fs::read_to_string(skill().join("SKILL.md")).unwrap();
    let listed = links(&index);
    for page in pages().iter().skip(1) {
        let relative = format!("references/{}", page.file_name().unwrap().to_str().unwrap());
        assert!(listed.contains(&relative), "{relative} is not in the index");
    }
    for page in pages() {
        let text = fs::read_to_string(&page).unwrap();
        for target in links(&text) {
            let path = target.split('#').next().unwrap();
            assert!(
                page.parent().unwrap().join(path).exists(),
                "{}: {target} does not exist",
                page.display()
            );
        }
    }
}

#[test]
fn every_example_compiles_and_renders_without_warnings() {
    let resolver =
        FileAssetResolver::new(Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/skill"))
            .unwrap();
    let mut count = 0;
    for page in pages() {
        let text = fs::read_to_string(&page).unwrap();
        let name = page.file_name().unwrap().to_string_lossy().into_owned();
        for (index, block) in text.split("```mote").skip(1).enumerate() {
            let (info, rest) = block.split_once('\n').unwrap();
            let (source, _) = rest.split_once("\n```").unwrap();
            let label = format!("{name} example {}", index + 1);
            let document = mote::compile_with_options(
                source,
                EvaluationOptions {
                    enable_lua: info.trim() == "lua",
                    asset_resolver: Some(&resolver),
                    ..Default::default()
                },
            )
            .unwrap_or_else(|error| panic!("{label}: {error}"));
            assert!(
                document.warnings.is_empty(),
                "{label}: {:?}",
                document.warnings
            );
            let planned = render::plan(&document, &resolver, None)
                .unwrap_or_else(|error| panic!("{label}: {error}"));
            assert!(
                planned.warnings.is_empty(),
                "{label}: {:?}",
                planned.warnings
            );
            let painted = render::paint_png(&planned.output, &Default::default())
                .unwrap_or_else(|error| panic!("{label}: {error}"));
            assert!(
                painted.warnings.is_empty(),
                "{label}: {:?}",
                painted.warnings
            );
            let warnings = if source.contains("#presentation") {
                mote::pptx::emit_with_assets(&document, &resolver, None)
                    .unwrap_or_else(|error| panic!("{label}: {error}"))
                    .warnings
            } else {
                mote::docx::emit_with_assets(&document, &resolver, None)
                    .unwrap_or_else(|error| panic!("{label}: {error}"))
                    .warnings
            };
            assert!(warnings.is_empty(), "{label}: {warnings:?}");
            count += 1;
        }
    }
    assert!(count > 0);
}
