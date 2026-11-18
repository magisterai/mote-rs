use super::super::{
    DEFAULT_AXIS_TITLE_SIZE, DEFAULT_LABEL_SIZE, DEFAULT_TITLE_SIZE, RULE_COLOR, RULE_WIDTH,
    TitleStyle,
    appearance::{Border, DataLabels, Gridlines, ShapeProperties},
};
use super::{Chart, Error, Options};
use crate::xml::escape;

pub(super) fn axis_title(
    text: &str,
    style: &TitleStyle,
    options: &Options,
) -> Result<String, Error> {
    styled_title(text, style, DEFAULT_AXIS_TITLE_SIZE, options)
}
pub(super) fn chart_title(chart: &Chart, options: &Options) -> Result<String, Error> {
    if chart.title.is_empty() {
        return Ok(r#"<c:autoTitleDeleted val="1"/>"#.into());
    }
    styled_title(
        &chart.title,
        &chart.title_style,
        DEFAULT_TITLE_SIZE,
        options,
    )
}
// Office draws rich title text at 18 points and in its own default typeface and
// colour unless the run names them, so the run names all three.
fn styled_title(
    text: &str,
    style: &TitleStyle,
    default_size: f64,
    options: &Options,
) -> Result<String, Error> {
    if text.is_empty() {
        return Ok(String::new());
    }
    let mut attributes = format!(
        r#" sz="{}""#,
        crate::xml::drawing_font_size(style.size.unwrap_or(default_size))
    );
    for (name, value) in [("b", style.bold), ("i", style.italic)] {
        if let Some(value) = value {
            attributes.push_str(&format!(r#" {name}="{}""#, u8::from(value)));
        }
    }
    let children = format!(
        r#"<a:solidFill><a:srgbClr val="{}"/></a:solidFill><a:latin typeface="{}"/>"#,
        escape(style.color.as_deref().unwrap_or(&options.text_color))?,
        escape(style.font.as_deref().unwrap_or(&options.font))?
    );
    let properties = format!(r#"<a:rPr lang="en-US"{attributes}>{children}</a:rPr>"#);
    Ok(format!(
        r#"<c:title><c:tx><c:rich><a:bodyPr/><a:lstStyle/><a:p><a:r>{properties}<a:t>{}</a:t></a:r></a:p></c:rich></c:tx><c:layout/><c:overlay val="0"/></c:title>"#,
        escape(text)?
    ))
}
pub(super) fn shape_properties(properties: &ShapeProperties) -> Result<String, Error> {
    let fill = if properties.fill.is_empty() {
        "<a:noFill/>".into()
    } else {
        format!(
            r#"<a:solidFill><a:srgbClr val="{}"/></a:solidFill>"#,
            escape(&properties.fill)?
        )
    };
    Ok(format!(
        "<c:spPr>{fill}{}</c:spPr>",
        border_line(&properties.border)?
    ))
}
fn border_line(border: &Border) -> Result<String, Error> {
    if !border.enabled {
        return Ok("<a:ln><a:noFill/></a:ln>".into());
    }
    let width = if border.width > 0 {
        border.width
    } else {
        12700
    };
    let color = if border.color.is_empty() {
        "D1D5DB"
    } else {
        &border.color
    };
    Ok(format!(
        r#"<a:ln w="{width}"><a:solidFill><a:srgbClr val="{}"/></a:solidFill></a:ln>"#,
        escape(color)?
    ))
}
// A label element states its size only when the chart departs from the default,
// which the chart space already supplies. The chart space default is left alone,
// so that no title can inherit an authored label size.
pub(super) fn label_text(chart: &Chart, color: &str) -> Result<String, Error> {
    let size = if chart.label_size == DEFAULT_LABEL_SIZE {
        String::new()
    } else {
        format!(
            r#" sz="{}""#,
            crate::xml::drawing_font_size(chart.label_size)
        )
    };
    if color.is_empty() && size.is_empty() {
        return Ok(String::new());
    }
    let properties = if color.is_empty() {
        format!("<a:defRPr{size}/>")
    } else {
        format!(
            r#"<a:defRPr{size}><a:solidFill><a:srgbClr val="{}"/></a:solidFill></a:defRPr>"#,
            escape(color)?
        )
    };
    Ok(format!(
        r#"<c:txPr><a:bodyPr/><a:lstStyle/><a:p><a:pPr>{properties}</a:pPr><a:endParaRPr lang="en-US"/></a:p></c:txPr>"#
    ))
}
pub(super) fn data_labels(chart: &Chart) -> Result<String, Error> {
    let labels: &DataLabels = &chart.data_labels;
    let kind = chart.kind.as_str();
    if !labels.enabled {
        return Ok(String::new());
    }
    let mut output = format!("<c:dLbls>{}", label_text(chart, "")?);
    // Office repairs doughnut charts with an explicit label position.
    if !labels.position.is_empty() && kind != "doughnut" {
        output.push_str(&format!(
            r#"<c:dLblPos val="{}"/>"#,
            escape(&labels.position)?
        ));
    }
    for (name, value) in [
        ("showLegendKey", false),
        ("showVal", labels.value),
        ("showCatName", labels.category),
        ("showSerName", labels.series),
        ("showPercent", labels.percentage),
        ("showBubbleSize", false),
    ] {
        output.push_str(&format!(r#"<c:{name} val="{}"/>"#, u8::from(value)));
    }
    output.push_str("</c:dLbls>");
    Ok(output)
}
pub(super) fn axes(chart: &Chart, options: &Options) -> Result<String, Error> {
    let category = options.category_axis();
    let value = options.value_axis();
    let labels = label_text(chart, &options.axis_label_color)?;
    let category_line = rule_line(RULE_WIDTH, RULE_COLOR)?;
    let mut output = format!(
        r#"<c:catAx><c:axId val="{category}"/><c:scaling><c:orientation val="minMax"/></c:scaling><c:delete val="0"/><c:axPos val="b"/>{}<c:numFmt formatCode="General" sourceLinked="1"/><c:majorTickMark val="none"/><c:minorTickMark val="none"/><c:tickLblPos val="{}"/><c:spPr><a:noFill/>{category_line}</c:spPr>{labels}<c:crossAx val="{value}"/><c:crosses val="autoZero"/><c:auto val="1"/><c:lblAlgn val="ctr"/><c:lblOffset val="100"/><c:noMultiLvlLbl val="0"/></c:catAx>"#,
        axis_title(&chart.category_title, &chart.category_title_style, options)?,
        chart.category_label_position()
    );
    // The schema places the maximum before the minimum.
    let range: String = [("max", chart.value_maximum), ("min", chart.value_minimum)]
        .into_iter()
        .filter_map(|(name, bound)| bound.map(|bound| format!(r#"<c:{name} val="{bound}"/>"#)))
        .collect();
    let major_unit = chart
        .value_step
        .map(|step| format!(r#"<c:majorUnit val="{step}"/>"#))
        .unwrap_or_default();
    output.push_str(&format!(r#"<c:valAx><c:axId val="{value}"/><c:scaling><c:orientation val="minMax"/>{range}</c:scaling><c:delete val="0"/><c:axPos val="l"/>{}{}<c:numFmt formatCode="General" sourceLinked="1"/><c:majorTickMark val="none"/><c:minorTickMark val="none"/><c:tickLblPos val="nextTo"/><c:spPr><a:noFill/><a:ln><a:noFill/></a:ln></c:spPr>{labels}<c:crossAx val="{category}"/><c:crosses val="autoZero"/><c:crossBetween val="between"/>{major_unit}</c:valAx>"#, gridlines(&chart.gridlines)?, axis_title(&chart.value_title, &chart.value_title_style, options)?));
    Ok(output)
}
fn gridlines(gridlines: &Gridlines) -> Result<String, Error> {
    let width = if gridlines.width > 0 {
        gridlines.width
    } else {
        RULE_WIDTH
    };
    let color = if gridlines.color.is_empty() {
        RULE_COLOR
    } else {
        &gridlines.color
    };
    let line = rule_line(width, color)?;
    let mut output = String::new();
    for (name, enabled) in [
        ("majorGridlines", gridlines.major),
        ("minorGridlines", gridlines.minor),
    ] {
        if enabled {
            output.push_str(&format!("<c:{name}><c:spPr>{line}</c:spPr></c:{name}>"));
        }
    }
    Ok(output)
}
// Office writes the default chart style's rules with these explicit line attributes.
fn rule_line(width: i64, color: &str) -> Result<String, Error> {
    Ok(format!(
        r#"<a:ln w="{width}" cap="flat" cmpd="sng" algn="ctr"><a:solidFill><a:srgbClr val="{}"/></a:solidFill><a:round/></a:ln>"#,
        escape(color)?
    ))
}
