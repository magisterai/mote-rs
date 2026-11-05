use super::super::{
    RULE_COLOR, RULE_WIDTH, TitleStyle,
    appearance::{Border, DataLabels, Gridlines, ShapeProperties},
};
use super::{Chart, Error, Options};
use crate::xml::escape;

pub(super) fn title(text: &str) -> Result<String, Error> {
    styled_title(text, &TitleStyle::default())
}
pub(super) fn chart_title(chart: &Chart) -> Result<String, Error> {
    if chart.title.is_empty() {
        return Ok(r#"<c:autoTitleDeleted val="1"/>"#.into());
    }
    styled_title(&chart.title, &chart.title_style)
}
fn styled_title(text: &str, style: &TitleStyle) -> Result<String, Error> {
    if text.is_empty() {
        return Ok(String::new());
    }
    let mut attributes = String::new();
    if let Some(size) = style.size {
        attributes.push_str(&format!(r#" sz="{}""#, (size * 100.0).round() as i64));
    }
    for (name, value) in [("b", style.bold), ("i", style.italic)] {
        if let Some(value) = value {
            attributes.push_str(&format!(r#" {name}="{}""#, u8::from(value)));
        }
    }
    let mut children = String::new();
    if let Some(color) = &style.color {
        children.push_str(&format!(
            r#"<a:solidFill><a:srgbClr val="{}"/></a:solidFill>"#,
            escape(color)?
        ));
    }
    if let Some(font) = &style.font {
        children.push_str(&format!(r#"<a:latin typeface="{}"/>"#, escape(font)?));
    }
    let properties = if children.is_empty() {
        format!(r#"<a:rPr lang="en-US"{attributes}/>"#)
    } else {
        format!(r#"<a:rPr lang="en-US"{attributes}>{children}</a:rPr>"#)
    };
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
pub(super) fn label_text(baseline: i32, color: &str) -> Result<String, Error> {
    if baseline == 0 && color.is_empty() {
        return Ok(String::new());
    }
    let baseline = if baseline == 0 {
        String::new()
    } else {
        format!(r#" baseline="{baseline}""#)
    };
    let properties = if color.is_empty() {
        format!("<a:defRPr{baseline}/>")
    } else {
        format!(
            r#"<a:defRPr{baseline}><a:solidFill><a:srgbClr val="{}"/></a:solidFill></a:defRPr>"#,
            escape(color)?
        )
    };
    Ok(format!(
        r#"<c:txPr><a:bodyPr/><a:lstStyle/><a:p><a:pPr>{properties}</a:pPr><a:endParaRPr lang="en-US"/></a:p></c:txPr>"#
    ))
}
pub(super) fn data_labels(labels: &DataLabels, kind: &str) -> Result<String, Error> {
    if !labels.enabled {
        return Ok(String::new());
    }
    let mut output = String::from("<c:dLbls>");
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
    let labels = label_text(options.label_baseline, &options.axis_label_color)?;
    let category_line = rule_line(RULE_WIDTH, RULE_COLOR)?;
    let mut output = format!(
        r#"<c:catAx><c:axId val="{category}"/><c:scaling><c:orientation val="minMax"/></c:scaling><c:delete val="0"/><c:axPos val="b"/>{}<c:numFmt formatCode="General" sourceLinked="1"/><c:majorTickMark val="none"/><c:minorTickMark val="none"/><c:tickLblPos val="{}"/><c:spPr><a:noFill/>{category_line}</c:spPr>{labels}<c:crossAx val="{value}"/><c:crosses val="autoZero"/><c:auto val="1"/><c:lblAlgn val="ctr"/><c:lblOffset val="100"/><c:noMultiLvlLbl val="0"/></c:catAx>"#,
        title(&chart.category_title)?,
        chart.category_label_position()
    );
    output.push_str(&format!(r#"<c:valAx><c:axId val="{value}"/><c:scaling><c:orientation val="minMax"/></c:scaling><c:delete val="0"/><c:axPos val="l"/>{}{}<c:numFmt formatCode="General" sourceLinked="1"/><c:majorTickMark val="none"/><c:minorTickMark val="none"/><c:tickLblPos val="nextTo"/><c:spPr><a:noFill/><a:ln><a:noFill/></a:ln></c:spPr>{labels}<c:crossAx val="{category}"/><c:crosses val="autoZero"/><c:crossBetween val="between"/></c:valAx>"#, gridlines(&chart.gridlines)?, title(&chart.value_title)?));
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
