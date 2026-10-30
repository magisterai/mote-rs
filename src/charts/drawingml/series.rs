use super::super::{Chart, Error, Series, workbook};
use crate::xml::escape;

#[derive(Clone, Copy, PartialEq)]
pub(super) enum Kind {
    Bar,
    Line,
    Area,
    Pie,
}
pub(super) fn write(
    index: usize,
    series: &Series,
    chart: &Chart,
    kind: Kind,
) -> Result<String, Error> {
    let color = escape(chart.color(index))?;
    let mut output = format!(
        r#"<c:ser><c:idx val="{index}"/><c:order val="{index}"/><c:tx><c:strRef><c:f>{}</c:f><c:strCache><c:ptCount val="1"/><c:pt idx="0"><c:v>{}</c:v></c:pt></c:strCache></c:strRef></c:tx>"#,
        workbook::cell_reference(index + 2, 1),
        escape(&series.name)?
    );
    match kind {
        Kind::Bar | Kind::Area => output.push_str(&format!(r#"<c:spPr><a:solidFill><a:srgbClr val="{color}"/></a:solidFill><a:ln><a:noFill/></a:ln></c:spPr>"#)),
        Kind::Line => output.push_str(&format!(r#"<c:spPr><a:ln w="25400"><a:solidFill><a:srgbClr val="{color}"/></a:solidFill></a:ln></c:spPr>"#)),
        Kind::Pie => {},
    }
    if kind == Kind::Bar {
        output.push_str(r#"<c:invertIfNegative val="0"/>"#);
    }
    if kind == Kind::Line {
        output.push_str(&format!(r#"<c:marker><c:symbol val="circle"/><c:size val="5"/><c:spPr><a:solidFill><a:srgbClr val="{color}"/></a:solidFill><a:ln><a:solidFill><a:srgbClr val="{color}"/></a:solidFill></a:ln></c:spPr></c:marker>"#));
    }
    if kind == Kind::Pie {
        for index in 0..series.values.len() {
            output.push_str(&format!(r#"<c:dPt><c:idx val="{index}"/><c:spPr><a:solidFill><a:srgbClr val="{}"/></a:solidFill><a:ln><a:noFill/></a:ln></c:spPr></c:dPt>"#, escape(chart.color(index))?));
        }
    }
    output.push_str(&format!(
        r#"<c:cat><c:strRef><c:f>{}</c:f><c:strCache><c:ptCount val="{}"/>"#,
        workbook::range_reference(1, chart.categories.len()),
        chart.categories.len()
    ));
    for (index, category) in chart.categories.iter().enumerate() {
        output.push_str(&format!(
            r#"<c:pt idx="{index}"><c:v>{}</c:v></c:pt>"#,
            escape(category)?
        ));
    }
    output.push_str(&format!(r#"</c:strCache></c:strRef></c:cat><c:val><c:numRef><c:f>{}</c:f><c:numCache><c:formatCode>General</c:formatCode><c:ptCount val="{}"/>"#, workbook::range_reference(index + 2, series.values.len()), series.values.len()));
    for (index, value) in series.values.iter().enumerate() {
        output.push_str(&format!(r#"<c:pt idx="{index}"><c:v>{value}</c:v></c:pt>"#));
    }
    output.push_str("</c:numCache></c:numRef></c:val>");
    if kind == Kind::Line {
        output.push_str(r#"<c:smooth val="0"/>"#);
    }
    output.push_str("</c:ser>");
    Ok(output)
}
