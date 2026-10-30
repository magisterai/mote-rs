use super::super::appearance::{Border, DataLabels, Gridlines, ShapeProperties};
use super::{Chart, Error, Options};
use crate::xml::escape;

pub(super) fn title(text: &str) -> Result<String, Error> {
    if text.is_empty() {
        return Ok(String::new());
    }
    Ok(format!(
        r#"<c:title><c:tx><c:rich><a:bodyPr/><a:lstStyle/><a:p><a:r><a:rPr lang="en-US"/><a:t>{}</a:t></a:r></a:p></c:rich></c:tx><c:layout/><c:overlay val="0"/></c:title>"#,
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
pub(super) fn label_text(baseline: i32) -> String {
    if baseline == 0 {
        return String::new();
    }
    format!(
        r#"<c:txPr><a:bodyPr/><a:lstStyle/><a:p><a:pPr><a:defRPr baseline="{baseline}"/></a:pPr><a:endParaRPr lang="en-US"/></a:p></c:txPr>"#
    )
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
    let labels = label_text(options.label_baseline);
    let mut output = format!(
        r#"<c:catAx><c:axId val="{category}"/><c:scaling><c:orientation val="minMax"/></c:scaling><c:delete val="0"/><c:axPos val="b"/>{}<c:numFmt formatCode="General" sourceLinked="1"/><c:majorTickMark val="out"/><c:minorTickMark val="none"/><c:tickLblPos val="{}"/>{labels}<c:crossAx val="{value}"/><c:crosses val="autoZero"/><c:auto val="1"/><c:lblAlgn val="ctr"/><c:lblOffset val="100"/><c:noMultiLvlLbl val="0"/></c:catAx>"#,
        title(&chart.category_title)?,
        chart.category_label_position()
    );
    output.push_str(&format!(r#"<c:valAx><c:axId val="{value}"/><c:scaling><c:orientation val="minMax"/></c:scaling><c:delete val="0"/><c:axPos val="l"/>{}<c:numFmt formatCode="General" sourceLinked="1"/>{}<c:majorTickMark val="out"/><c:minorTickMark val="none"/><c:tickLblPos val="nextTo"/>{labels}<c:crossAx val="{category}"/><c:crosses val="autoZero"/><c:crossBetween val="between"/></c:valAx>"#, title(&chart.value_title)?, gridlines(&chart.gridlines)?));
    Ok(output)
}
fn gridlines(gridlines: &Gridlines) -> Result<String, Error> {
    let mut output = String::new();
    for (name, enabled) in [
        ("majorGridlines", gridlines.major),
        ("minorGridlines", gridlines.minor),
    ] {
        if !enabled {
            continue;
        }
        if gridlines.color.is_empty() && gridlines.width <= 0 {
            output.push_str(&format!("<c:{name}/>"));
        } else {
            let border = Border {
                enabled: true,
                color: gridlines.color.clone(),
                width: gridlines.width,
            };
            output.push_str(&format!(
                "<c:{name}><c:spPr>{}</c:spPr></c:{name}>",
                border_line(&border)?
            ));
        }
    }
    Ok(output)
}
