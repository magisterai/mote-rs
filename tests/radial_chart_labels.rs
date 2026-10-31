use mote::{layout::PageOperation, render, source::FileAssetResolver};

#[test]
fn radial_labels_remain_visible_and_separate_for_small_and_nonpositive_values() {
    let resolver = FileAssetResolver::new(".").unwrap();
    for kind in ["pie", "doughnut"] {
        for values in ["1,1,1,97", "4,-2,0,1"] {
            let source = format!(
                "#chart(kind='{kind}',width=pt(330),height=pt(250),legend=false,data={{categories={{'Alpha','Beta','Gamma','Delta'}},values={{{values}}}}},data_labels={{category=true,value=true,percent=true}})"
            );
            let planned = render::plan(&mote::compile(&source).unwrap(), &resolver, None).unwrap();
            let page = &planned.output.pages[0];
            let text: String = page
                .lines
                .iter()
                .flat_map(|placed| &placed.line.fragments)
                .map(|fragment| fragment.text.as_str())
                .collect();
            for label in ["Alpha", "Beta", "Gamma", "Delta"] {
                assert!(text.contains(label), "missing {label}: {text}");
            }
            let mut frames = Vec::new();
            let mut label_seen = false;
            for operation in &page.operations {
                match operation {
                    PageOperation::Text(_) => label_seen = true,
                    PageOperation::Path(path) if path.fill.is_some() => {
                        assert!(!label_seen, "a sector or doughnut hole paints over labels")
                    }
                    PageOperation::PushClip(bounds) if !frames.contains(bounds) => {
                        frames.push(*bounds)
                    }
                    _ => {}
                }
            }
            assert_eq!(frames.len(), 4);
            for (index, first) in frames.iter().enumerate() {
                for second in &frames[index + 1..] {
                    assert!(
                        first[0] + first[2] <= second[0]
                            || second[0] + second[2] <= first[0]
                            || first[1] + first[3] <= second[1]
                            || second[1] + second[3] <= first[1],
                        "radial label frames overlap"
                    );
                }
            }
            assert!(
                render::paint_pdf(&planned.output)
                    .unwrap()
                    .starts_with(b"%PDF-")
            );
            assert_eq!(
                render::paint_png(&planned.output, &Default::default())
                    .unwrap()
                    .output
                    .len(),
                1
            );
        }
    }
}

#[test]
fn a_finite_large_sector_does_not_overflow_while_computing_its_angle() {
    let resolver = FileAssetResolver::new(".").unwrap();
    for kind in ["pie", "doughnut"] {
        let document = mote::compile(&format!(
            "#chart(kind='{kind}',data={{values={{'1e308'}}}})"
        ))
        .unwrap();
        let plan = render::plan(&document, &resolver, None).unwrap();
        assert!(
            render::paint_pdf(&plan.output)
                .unwrap()
                .starts_with(b"%PDF-")
        );
        assert_eq!(
            render::paint_png(&plan.output, &Default::default())
                .unwrap()
                .output
                .len(),
            1
        );
    }
}
