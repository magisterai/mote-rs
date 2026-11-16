use mote::{
    layout::{PageOperation, RuleRole},
    render,
    source::FileAssetResolver,
};

#[test]
fn narrow_chart_axes_keep_zero_and_separate_numeric_labels() {
    let resolver = FileAssetResolver::new(".").unwrap();
    for kind in ["bar", "column"] {
        let source = format!(
            "#chart(kind='{kind}',width=pt(120),height=pt(120),legend=false,data={{categories={{'A','B'}},values={{-0.3,0.6}}}})"
        );
        let document = mote::compile(&source).unwrap();
        let planned = render::plan(&document, &resolver, None).unwrap();
        let page = &planned.output.pages[0];
        let labels: Vec<_> = page
            .lines
            .iter()
            .filter_map(|placed| {
                let text: String = placed
                    .line
                    .fragments
                    .iter()
                    .map(|fragment| fragment.text.as_str())
                    .collect();
                text.parse::<f64>().ok().map(|value| (value, placed))
            })
            .collect();
        assert!(labels.iter().any(|(value, _)| *value == 0.0));
        let gridlines = page.operations.iter().filter(|operation| matches!(operation,PageOperation::Rule(rule) if rule.role == RuleRole::ChartGridline)).count();
        if kind == "bar" {
            assert!(
                labels.len() < gridlines,
                "dense axes should retain gridlines while omitting colliding text"
            );
        } else {
            assert_eq!(
                labels.len(),
                gridlines,
                "a vertical value axis labels every major gridline, as Office does"
            );
        }
        for (index, (_, first)) in labels.iter().enumerate() {
            for (_, second) in labels.iter().skip(index + 1) {
                if kind == "bar" {
                    let first_left = first.line.fragments[0].x;
                    let second_left = second.line.fragments[0].x;
                    assert!(
                        first_left + first.line.width <= second_left
                            || second_left + second.line.width <= first_left
                    );
                } else {
                    assert!(
                        first.top + first.line.height <= second.top
                            || second.top + second.line.height <= first.top
                    );
                }
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

#[test]
fn unstyled_axes_follow_the_office_default_chart_style() {
    let resolver = FileAssetResolver::new(".").unwrap();
    for (kind, gridlines, grid_color, grid_width) in [
        ("column", "true", "D9D9D9", 0.75),
        ("bar", "true", "D9D9D9", 0.75),
        ("line", "{color='#FF0000',width=pt(2)}", "FF0000", 2.0),
    ] {
        let source = format!(
            "#chart(kind='{kind}',legend=false,title='Totals',gridlines={gridlines},data={{categories={{'A','B'}},values={{1,2}}}})"
        );
        let document = mote::compile(&source).unwrap();
        let planned = render::plan(&document, &resolver, None).unwrap();
        let page = &planned.output.pages[0];
        let rules: Vec<_> = page
            .operations
            .iter()
            .filter_map(|operation| match operation {
                PageOperation::Rule(rule) => Some(rule),
                _ => None,
            })
            .collect();
        let axes: Vec<_> = rules
            .iter()
            .filter(|rule| rule.role == RuleRole::ChartAxis)
            .collect();
        assert_eq!(axes.len(), 1, "{kind}: only the category axis is drawn");
        assert_eq!((axes[0].color.as_str(), axes[0].width), ("D9D9D9", 0.75));
        let category_axis_is_vertical = axes[0].start[0] == axes[0].end[0];
        assert_eq!(category_axis_is_vertical, kind == "bar", "{kind}");
        assert!(
            rules
                .iter()
                .any(|rule| rule.role == RuleRole::ChartGridline)
        );
        for rule in rules
            .iter()
            .filter(|rule| rule.role == RuleRole::ChartGridline)
        {
            assert_eq!((rule.color.as_str(), rule.width), (grid_color, grid_width));
        }
        for placed in &page.lines {
            for fragment in &placed.line.fragments {
                let expected = if fragment.text == "Totals" {
                    "000000"
                } else {
                    "595959"
                };
                assert_eq!(fragment.style.color, expected, "{kind}: {}", fragment.text);
            }
        }
    }
}

const TITLES: [&str; 3] = ["Totals", "Regions", "Units"];

fn label_rectangles(page: &mote::layout::TextPage) -> Vec<(String, f64, [f64; 4])> {
    page.lines
        .iter()
        .filter_map(|placed| {
            let fragments = &placed.line.fragments;
            let text: String = fragments
                .iter()
                .map(|fragment| fragment.text.as_str())
                .collect();
            (!TITLES.contains(&text.as_str())).then(|| {
                let left = fragments[0].x;
                (
                    text,
                    fragments[0].style.measurement.size_points,
                    [left, placed.top, placed.line.width, placed.line.height],
                )
            })
        })
        .collect()
}

#[test]
fn chart_text_size_sets_every_label_and_leaves_the_titles() {
    let resolver = FileAssetResolver::new(".").unwrap();
    for kind in ["column", "bar", "line", "pie", "stacked_column", "doughnut"] {
        for (argument, expected) in [("", 10.0), (",text={size=pt(14)}", 14.0)] {
            let source = format!(
                "#presentation(size='wide')\n#slide[#box(position={{x=pt(40),y=pt(40),width=pt(640),height=pt(400)}})[#chart(kind='{kind}',title='Totals',legend='bottom',data_labels=true,axis={{category={{title='Regions'}},value={{title='Units'}}}},data={{categories={{'North','South','East'}},series={{{{name='First',values={{4,7,5}}}},{{name='Second',values={{6,3,8}}}}}}}}{argument})]]"
            );
            let document = mote::compile(&source).unwrap();
            let planned = render::plan(&document, &resolver, None).unwrap();
            let page = &planned.output.pages[0];
            let painted = render::paint_png(&planned.output, &Default::default()).unwrap();
            let in_document = source.split_once("[#box").unwrap().1;
            let in_document = &in_document[in_document.find("#chart").unwrap()..];
            let in_document = in_document.strip_suffix("]]").unwrap();
            let document_plan =
                render::plan(&mote::compile(in_document).unwrap(), &resolver, None).unwrap();
            for warning in planned
                .warnings
                .iter()
                .chain(&painted.warnings)
                .chain(&document_plan.warnings)
            {
                assert!(!warning.contains("text"), "{kind}: {warning}");
            }
            let labels = label_rectangles(page);
            assert!(labels.len() >= 5, "{kind}: {labels:?}");
            for (text, size, _) in &labels {
                assert_eq!(*size, expected, "{kind}: {text}");
            }
            for placed in &page.lines {
                for fragment in &placed.line.fragments {
                    if TITLES.contains(&fragment.text.as_str()) {
                        assert_eq!(fragment.style.measurement.size_points, 18.0, "{kind}");
                    }
                }
            }
        }
    }
}

#[test]
fn large_column_chart_labels_stay_clear_and_inside_the_chart_frame() {
    let resolver = FileAssetResolver::new(".").unwrap();
    let frame = [40.0, 40.0, 640.0, 400.0];
    for legend in ["bottom", "top", "right", "left"] {
        let source = format!(
            "#presentation(size='wide')\n#slide[#box(position={{x=pt(40),y=pt(40),width=pt(640),height=pt(400)}})[#chart(kind='column',title='Totals',legend='{legend}',data_labels=true,text={{size=pt(16)}},axis={{category={{title='Regions'}},value={{title='Units'}}}},data={{categories={{'North','South','East','West'}},series={{{{name='First',values={{40,70,50,96}}}},{{name='Second',values={{60,30,98,45}}}}}}}})]]"
        );
        let document = mote::compile(&source).unwrap();
        let planned = render::plan(&document, &resolver, None).unwrap();
        let labels = label_rectangles(&planned.output.pages[0]);
        // The chart has four category labels, eight data labels, two legend entries
        // and the value axis labels.
        assert!(labels.len() > 14, "{legend}: {labels:?}");
        for (index, (text, size, bounds)) in labels.iter().enumerate() {
            assert_eq!(
                *size, 16.0,
                "{legend}: {text} is not reduced to fit its frame"
            );
            assert!(
                bounds[0] >= frame[0]
                    && bounds[1] >= frame[1]
                    && bounds[0] + bounds[2] <= frame[0] + frame[2]
                    && bounds[1] + bounds[3] <= frame[1] + frame[3],
                "{legend}: {text} leaves the chart frame at {bounds:?}"
            );
            for (other_text, _, other) in labels.iter().skip(index + 1) {
                assert!(
                    bounds[0] + bounds[2] <= other[0]
                        || other[0] + other[2] <= bounds[0]
                        || bounds[1] + bounds[3] <= other[1]
                        || other[1] + other[3] <= bounds[1],
                    "{legend}: {text} at {bounds:?} overlaps {other_text} at {other:?}"
                );
            }
        }
    }
}
