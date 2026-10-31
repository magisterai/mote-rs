use super::{Error, escape, package::Relationship};
use crate::{
    drawings::paragraphs::DrawingParagraph,
    layout::{LineSpacing, TextRun, TextStyle},
    measurement::FontVariant,
    model::{Content, Document, Properties},
    presentation::font_name,
};
use std::collections::BTreeMap;

pub(super) fn write(
    document: &Document,
    content: &[Content],
    attributes: &Properties,
    theme: &crate::presentation::Theme,
    relationships: &mut Vec<Relationship>,
) -> Result<String, Error> {
    let paragraphs = crate::presentation::text::resolve(
        document,
        content,
        attributes,
        theme,
        crate::presentation::text::TextDefaults::Inherited,
    )
    .map_err(|error| Error(error.to_string()))?;
    if paragraphs.is_empty() {
        return Ok("<a:p/>".into());
    }
    let mut output = String::new();
    for paragraph in paragraphs {
        output.push_str("<a:p>");
        output.push_str(&properties(&paragraph)?);
        if let Some(checked) = paragraph.task {
            let mut style = TextStyle::default();
            style.measurement.font = "MS Gothic".into();
            style.measurement.size_points = 0.0;
            style.color.clear();
            output.push_str(&run_xml(
                document,
                &TextRun::new(if checked { "☒" } else { "☐" }, style),
                None,
            )?);
            let mut style = TextStyle::default();
            style.measurement.font.clear();
            style.measurement.size_points = 0.0;
            style.color.clear();
            output.push_str(&run_xml(document, &TextRun::new(" ", style), None)?);
        }
        let mut link_ids = BTreeMap::new();
        let mut at_start = true;
        for (index, run) in paragraph.runs.iter().enumerate() {
            if run.text == "\n" && !run.preserve_space {
                output.push_str("<a:br/>");
                at_start = true;
                continue;
            }
            let mut run = run.clone();
            if !run.preserve_space {
                if run.text.trim().is_empty() && run.text != "\t" {
                    let next = paragraph.runs[index + 1..]
                        .iter()
                        .find(|next| !next.text.trim().is_empty() || next.text == "\n");
                    if at_start || next.is_some_and(|next| next.text == "\n") {
                        continue;
                    }
                    run.text = " ".into();
                } else if at_start {
                    run.text = run.text.trim_start_matches([' ', '\t', '\r', '\n']).into();
                }
            }
            let relationship = run.link.as_ref().map(|url| {
                *link_ids.entry(url.clone()).or_insert_with(|| {
                    relationships.push(Relationship {
                        kind: "hyperlink",
                        target: url.clone(),
                        external: true,
                    });
                    relationships.len()
                })
            });
            output.push_str(&run_xml(document, &run, relationship)?);
            at_start = run.preserve_space && run.text.ends_with('\n');
        }
        output.push_str("<a:endParaRPr lang=\"en-US\"/></a:p>");
    }
    Ok(output)
}
fn properties(paragraph: &DrawingParagraph) -> Result<String, Error> {
    let style = &paragraph.style;
    let mut attributes = String::new();
    let mut children = String::new();
    if let Some(marker) = &paragraph.marker {
        attributes.push_str(&format!(" lvl=\"{}\"", marker.level));
    }
    let alignment = match style.alignment.as_str() {
        "left" => "l",
        "center" => "ctr",
        "right" => "r",
        "justify" => "just",
        _ => "l",
    };
    if !alignment.is_empty() {
        attributes.push_str(&format!(" algn=\"{alignment}\""));
    }
    for (name, value) in [
        ("marL", style.indent_left),
        ("marR", style.indent_right),
        ("indent", style.first_line),
    ] {
        if value != 0.0 {
            attributes.push_str(&format!(" {name}=\"{}\"", (value * 12700.0).round() as i64));
        }
    }
    for (name, value) in [
        ("spcBef", style.spacing_before),
        ("spcAft", style.spacing_after),
    ] {
        if value != 0.0 {
            children.push_str(&format!(
                "<a:{name}><a:spcPts val=\"{}\"/></a:{name}>",
                (value * 100.0).round() as i64
            ));
        }
    }
    let line = match style.line_spacing {
        LineSpacing::Multiple(value) if value != 1.0 => Some(("spcPct", value * 100000.0)),
        LineSpacing::Exact(value) | LineSpacing::AtLeast(value) => Some(("spcPts", value * 100.0)),
        _ => None,
    };
    if let Some((name, value)) = line {
        children.push_str(&format!(
            "<a:lnSpc><a:{name} val=\"{}\"/></a:lnSpc>",
            value.round() as i64
        ));
    }
    if let Some(marker) = &paragraph.marker {
        if paragraph.task.is_some() {
            children.push_str("<a:buNone/>");
        } else if marker.ordered {
            let format = match marker.format.as_str() {
                "lower_alpha" => "alphaLcPeriod",
                "upper_alpha" => "alphaUcPeriod",
                "lower_roman" => "romanLcPeriod",
                "upper_roman" => "romanUcPeriod",
                _ => "arabicPeriod",
            };
            let start = if marker.number > 1 {
                format!(" startAt=\"{}\"", marker.number)
            } else {
                String::new()
            };
            children.push_str(&format!("<a:buAutoNum type=\"{format}\"{start}/>"));
        } else {
            children.push_str(&format!("<a:buChar char=\"{}\"/>", escape(&marker.text)?));
        }
    }
    Ok(if attributes.is_empty() && children.is_empty() {
        String::new()
    } else if children.is_empty() {
        format!("<a:pPr{attributes}/>")
    } else {
        format!("<a:pPr{attributes}>{children}</a:pPr>")
    })
}
fn run_xml(
    document: &Document,
    run: &TextRun,
    relationship: Option<usize>,
) -> Result<String, Error> {
    if run.text.is_empty() {
        return Ok(String::new());
    }
    let style = &run.style;
    let mut output = String::from("<a:r><a:rPr lang=\"en-US\"");
    if matches!(
        style.measurement.variant,
        FontVariant::Bold | FontVariant::BoldItalic
    ) {
        output.push_str(" b=\"1\"");
    }
    if matches!(
        style.measurement.variant,
        FontVariant::Italic | FontVariant::BoldItalic
    ) {
        output.push_str(" i=\"1\"");
    }
    if style.underline {
        let underline = match style.underline_style.as_str() {
            "double" => "dbl",
            "dotted" => "dotted",
            "dashed" => "dash",
            "wave" => "wavy",
            "none" => "none",
            _ => "sng",
        };
        output.push_str(&format!(" u=\"{underline}\""));
    }
    if style.double_strike {
        output.push_str(" strike=\"dblStrike\"");
    } else if style.strike {
        output.push_str(" strike=\"sngStrike\"");
    }
    if style.subscript {
        output.push_str(" baseline=\"-25000\"");
    }
    if style.superscript {
        output.push_str(" baseline=\"30000\"");
    }
    if style.small_caps {
        output.push_str(" cap=\"small\"");
    }
    if style.measurement.size_points > 0.0 {
        output.push_str(&format!(
            " sz=\"{}\"",
            (style.measurement.size_points * 100.0).round() as i64
        ));
    }
    output.push('>');
    if !style.color.is_empty() {
        output.push_str(&format!(
            "<a:solidFill><a:srgbClr val=\"{}\"/></a:solidFill>",
            escape(&style.color)?
        ));
    }
    if style.underline && !style.underline_color.is_empty() {
        output.push_str(&format!(
            "<a:uFill><a:solidFill><a:srgbClr val=\"{}\"/></a:solidFill></a:uFill>",
            escape(&style.underline_color)?
        ));
    }
    if let Some(color) = &style.highlight {
        output.push_str(&format!(
            "<a:highlight><a:srgbClr val=\"{}\"/></a:highlight>",
            escape(color)?
        ));
    }
    if !style.measurement.font.is_empty() {
        output.push_str(&format!(
            "<a:latin typeface=\"{}\"/>",
            escape(&font_name(document, &style.measurement.font))?
        ));
    }
    if let Some(identifier) = relationship {
        output.push_str(&format!("<a:hlinkClick r:id=\"rId{identifier}\"/>"));
    }
    output.push_str("</a:rPr><a:t");
    if run.preserve_space || run.text.trim() != run.text {
        output.push_str(" xml:space=\"preserve\"");
    }
    output.push('>');
    output.push_str(&escape(&run.text)?);
    output.push_str("</a:t></a:r>");
    Ok(output)
}
