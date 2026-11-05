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
                    "111827"
                } else {
                    "595959"
                };
                assert_eq!(fragment.style.color, expected, "{kind}: {}", fragment.text);
            }
        }
    }
}
