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

struct PlannedChart {
    frame: [f64; 4],
    plot: [f64; 4],
    tick_labels: Vec<f64>,
    bars: Vec<[f64; 4]>,
    path_points: Vec<[f64; 2]>,
    painted_extent: [f64; 4],
}

/// The chart and plot borders give the frame and the plot as outline rectangles.
fn plan_bordered_chart(arguments: &str) -> PlannedChart {
    let resolver = FileAssetResolver::new(".").unwrap();
    let source = format!("#chart({arguments},legend=false,border=true,plot_area={{border=true}})");
    let document = mote::compile(&source).unwrap();
    let planned = render::plan(&document, &resolver, None).unwrap();
    assert_eq!(planned.warnings, Vec::<String>::new(), "{source}");
    let page = &planned.output.pages[0];
    let outlines: Vec<[f64; 4]> = page
        .operations
        .iter()
        .filter_map(|operation| match operation {
            PageOperation::OutlineRectangle { bounds, .. } => Some(*bounds),
            _ => None,
        })
        .collect();
    let [frame, plot] = outlines[..] else {
        panic!("{outlines:?}")
    };
    let mut extent = [f64::MAX, f64::MAX, f64::MIN, f64::MIN];
    let mut include = |left: f64, top: f64, right: f64, bottom: f64| {
        extent = [
            extent[0].min(left),
            extent[1].min(top),
            extent[2].max(right),
            extent[3].max(bottom),
        ];
    };
    let mut bars = Vec::new();
    let mut path_points = Vec::new();
    for operation in &page.operations {
        match operation {
            PageOperation::Rectangle(bar) => {
                bars.push([bar.x, bar.top, bar.width, bar.height]);
                include(bar.x, bar.top, bar.x + bar.width, bar.top + bar.height);
            }
            PageOperation::Rule(rule) => {
                for point in [rule.start, rule.end] {
                    include(point[0], point[1], point[0], point[1]);
                }
            }
            PageOperation::Path(path) => {
                for point in &path.points {
                    path_points.push(*point);
                    include(point[0], point[1], point[0], point[1]);
                }
            }
            _ => {}
        }
    }
    let mut tick_labels = Vec::new();
    for placed in &page.lines {
        let left = placed.line.fragments[0].x;
        include(
            left,
            placed.top,
            left + placed.line.width,
            placed.top + placed.line.height,
        );
        let text: String = placed
            .line
            .fragments
            .iter()
            .map(|fragment| fragment.text.as_str())
            .collect();
        if let Ok(value) = text.parse::<f64>() {
            tick_labels.push(value);
        }
    }
    PlannedChart {
        frame,
        plot,
        tick_labels,
        bars,
        path_points,
        painted_extent: extent,
    }
}

fn assert_painted_within_frame(chart: &PlannedChart, context: &str) {
    let [left, top, width, height] = chart.frame;
    let [painted_left, painted_top, painted_right, painted_bottom] = chart.painted_extent;
    assert!(
        painted_left >= left
            && painted_top >= top
            && painted_right <= left + width
            && painted_bottom <= top + height,
        "{context}: {:?} leaves {:?}",
        chart.painted_extent,
        chart.frame
    );
}

fn assert_close(actual: f64, expected: f64, context: &str) {
    assert!(
        (actual - expected).abs() < 1e-6,
        "{context}: {actual} is not {expected}"
    );
}

#[test]
fn a_stated_value_range_fixes_the_ticks_and_the_scale_of_the_bars() {
    for (kind, text) in [
        ("column", ""),
        ("bar", ""),
        ("column", ",text={size=pt(16)}"),
        ("bar", ",text={size=pt(16)}"),
    ] {
        let chart = plan_bordered_chart(&format!(
            "kind='{kind}',axis={{value={{minimum=0,maximum=200}}}},data_labels={},data={{categories={{'A','B','C'}},values={{50,200,10}}}}{text}",
            !text.is_empty()
        ));
        // The data labels of the enlarged chart repeat the values of the data.
        let ticks = chart.tick_labels.iter().copied();
        assert!(
            ticks.clone().filter(|tick| *tick == 0.0).count() == 1
                && ticks.clone().filter(|tick| *tick == 200.0).count()
                    == 1 + usize::from(!text.is_empty()),
            "{kind}{text}: {:?}",
            chart.tick_labels
        );
        let minimum = chart.tick_labels.iter().copied().fold(f64::MAX, f64::min);
        let maximum = chart.tick_labels.iter().copied().fold(f64::MIN, f64::max);
        assert_eq!((minimum, maximum), (0.0, 200.0), "{kind}");
        let [plot_left, plot_top, plot_width, plot_height] = chart.plot;
        let (lengths, plot_length): (Vec<f64>, f64) = if kind == "bar" {
            (chart.bars.iter().map(|bar| bar[2]).collect(), plot_width)
        } else {
            (chart.bars.iter().map(|bar| bar[3]).collect(), plot_height)
        };
        assert_eq!(lengths.len(), 3, "{kind}");
        let mut lengths = lengths;
        lengths.sort_by(f64::total_cmp);
        for (length, value) in lengths.iter().zip([10.0, 50.0, 200.0]) {
            assert_close(*length, plot_length * value / 200.0, kind);
        }
        for bar in &chart.bars {
            if kind == "bar" {
                assert_close(bar[0], plot_left, kind);
            } else {
                assert_close(bar[1] + bar[3], plot_top + plot_height, kind);
            }
        }
        assert_painted_within_frame(&chart, kind);
    }
}

#[test]
fn a_single_stated_bound_leaves_the_other_automatic() {
    let data = "data={categories={'A','B'},values={50,120}}";
    let automatic = plan_bordered_chart(data);
    let raised = plan_bordered_chart(&format!("axis={{value={{maximum=1000}}}},{data}"));
    let lifted = plan_bordered_chart(&format!("axis={{value={{minimum=40}}}},{data}"));
    let extremes = |chart: &PlannedChart| {
        (
            chart.tick_labels.iter().copied().fold(f64::MAX, f64::min),
            chart.tick_labels.iter().copied().fold(f64::MIN, f64::max),
        )
    };
    assert_eq!(extremes(&automatic), (0.0, 140.0));
    assert_eq!(extremes(&raised), (0.0, 1000.0));
    assert_eq!(extremes(&lifted), (40.0, 140.0));
    assert_close(
        raised.bars[0][3],
        raised.plot[3] * 50.0 / 1000.0,
        "raised maximum",
    );
    assert_close(
        lifted.bars[0][3],
        lifted.plot[3] * (50.0 - 40.0) / 100.0,
        "lifted minimum",
    );
}

#[test]
fn data_beyond_a_stated_bound_is_clipped_to_the_plot() {
    for kind in ["column", "bar", "stacked_column", "line", "area"] {
        // A native area chart has no data labels, and warns of one that is requested.
        let range = format!(
            "axis={{value={{minimum=20,maximum=100}}}},data_labels={}",
            kind != "area"
        );
        let chart = plan_bordered_chart(&format!(
            "kind='{kind}',{range},data={{categories={{'A','B','C','D'}},series={{{{name='One',values={{50,10,130,-40}}}},{{name='Two',values={{90,300,5,60}}}}}}}}"
        ));
        let [plot_left, plot_top, plot_width, plot_height] = chart.plot;
        for bar in &chart.bars {
            assert!(
                bar[0] >= plot_left - 1e-6
                    && bar[1] >= plot_top - 1e-6
                    && bar[0] + bar[2] <= plot_left + plot_width + 1e-6
                    && bar[1] + bar[3] <= plot_top + plot_height + 1e-6,
                "{kind}: {bar:?} leaves {:?}",
                chart.plot
            );
        }
        // A line marker is a circle of radius 2.5 points about a point in range.
        for point in &chart.path_points {
            assert!(
                point[1] >= plot_top - 2.5 - 1e-6
                    && point[1] <= plot_top + plot_height + 2.5 + 1e-6,
                "{kind}: {point:?} leaves {:?}",
                chart.plot
            );
        }
        assert_painted_within_frame(&chart, kind);
        if kind == "column" {
            let heights: Vec<f64> = chart.bars.iter().map(|bar| bar[3]).collect();
            assert!(
                heights
                    .iter()
                    .any(|height| (height - plot_height).abs() < 1e-6),
                "a value beyond both bounds fills the plot: {heights:?}"
            );
            assert_eq!(
                heights.len(),
                5,
                "a bar wholly outside the range is not painted"
            );
        }
    }
}

#[test]
fn a_stated_value_step_sets_the_ticks_from_the_minimum_in_force() {
    let data = "data={categories={'A','B'},values={50,120}}";
    for (kind, value_axis, expected) in [
        (
            "column",
            "minimum=0,maximum=150,step=50",
            vec![0.0, 50.0, 100.0, 150.0],
        ),
        (
            "bar",
            "minimum=0,maximum=150,step=50",
            vec![0.0, 50.0, 100.0, 150.0],
        ),
        ("line", "step=50", vec![0.0, 50.0, 100.0, 150.0]),
        (
            "column",
            "minimum=10,step=40",
            vec![10.0, 50.0, 90.0, 130.0, 170.0],
        ),
        (
            "column",
            "maximum=3,step=0.75",
            vec![0.0, 0.75, 1.5, 2.25, 3.0],
        ),
    ] {
        let chart = plan_bordered_chart(&format!(
            "kind='{kind}',axis={{value={{{value_axis}}}}},{data}"
        ));
        assert_painted_within_frame(&chart, value_axis);
        let mut ticks = chart.tick_labels.clone();
        ticks.sort_by(f64::total_cmp);
        assert_eq!(ticks, expected, "{kind} {value_axis}");
        assert_painted_within_frame(&chart, value_axis);
    }

    let resolver = FileAssetResolver::new(".").unwrap();
    let source = format!("#chart(axis={{value={{step=0.5}}}},{data})");
    let error = render::plan(&mote::compile(&source).unwrap(), &resolver, None)
        .err()
        .map(|error| error.to_string())
        .unwrap_or_default();
    assert!(error.contains("more than 100 intervals"), "{error}");
}

#[test]
fn a_label_reduced_below_the_chart_text_size_is_reported_once_for_its_chart() {
    let resolver = FileAssetResolver::new(".").unwrap();
    let chart = |categories: &str, name: &str| {
        format!(
            "#chart(kind='column'{name},width=pt(300),height=pt(200),data={{categories={{{categories}}},values={{1,2,3,4,5,6}}}})"
        )
    };
    let short = "'A','B','C','D','E','F'";
    let long = "'A','B','Falcon Heavy Extended','D','E','F'";
    let reduced = |warnings: &[String]| -> Vec<String> {
        warnings
            .iter()
            .filter(|warning| warning.contains("reduced below the chart's text size"))
            .cloned()
            .collect()
    };

    let plan = |source: &str| {
        render::plan(&mote::compile(source).unwrap(), &resolver, None)
            .unwrap()
            .warnings
    };
    assert_eq!(plan(&chart(short, "")), Vec::<String>::new());
    assert_eq!(
        reduced(&plan(&chart(long, ""))),
        [
            "the category labels of a chart on page 1 are reduced below the chart's text size to fit; shorten the labels, widen the chart or reduce text.size"
        ]
    );
    let named = reduced(&plan(&chart(long, ",name='Launches'")));
    assert_eq!(named.len(), 1, "{named:?}");
    assert!(
        named[0].starts_with("the category labels of chart \"Launches\" on page 1 "),
        "{named:?}"
    );

    let slide = |categories: &str| {
        format!(
            "#presentation(size='wide')\n#slide[#box(position={{x=pt(40),y=pt(40),width=pt(300),height=pt(200)}})[{}]]",
            chart(categories, "")
        )
    };
    assert_eq!(reduced(&plan(&slide(short))), Vec::<String>::new());
    let on_slide = reduced(&plan(&slide(long)));
    assert_eq!(on_slide.len(), 1, "{on_slide:?}");
    assert!(
        on_slide[0]
            .starts_with("the category labels of content[0].slide[0].box[0].chart are reduced"),
        "{on_slide:?}"
    );
}
