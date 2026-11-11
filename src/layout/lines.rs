use super::{
    LayoutError, LineOptions, LineSpacing, TextFragment, TextLine, TextRun, placement, wrapping,
};
use crate::measurement::TextMeasurer;

/// Plan relative line geometry without selecting pages or painting glyphs.
pub fn plan_text(
    runs: &[TextRun],
    options: &LineOptions,
    measurer: &mut TextMeasurer<'_>,
) -> Result<Vec<TextLine>, LayoutError> {
    if !options.width.is_finite() || !options.first_indent.is_finite() {
        return Err(LayoutError::InvalidGeometry(
            "line frames and spacing must be finite",
        ));
    }
    let spacing_value = match options.spacing {
        LineSpacing::Multiple(value) | LineSpacing::Exact(value) | LineSpacing::AtLeast(value) => {
            value
        }
    };
    if !spacing_value.is_finite() || spacing_value < 0.0 {
        return Err(LayoutError::InvalidGeometry(
            "line spacing must be finite and non-negative",
        ));
    }
    if runs.iter().any(|run| {
        run.fixed_advance
            .is_some_and(|advance| !advance.is_finite() || advance < 0.0)
    }) {
        return Err(LayoutError::InvalidGeometry(
            "fixed text advance must be finite and non-negative",
        ));
    }
    for style in std::iter::once(&options.base_style).chain(runs.iter().map(|run| &run.style)) {
        measurer.measure(&style.measurement, "", options.layout_kerning)?;
        if style
            .word_line_height
            .is_some_and(|height| !height.is_finite())
        {
            return Err(LayoutError::InvalidGeometry(
                "font line height must be finite",
            ));
        }
    }
    if options
        .base_line_height
        .is_some_and(|height| !height.is_finite())
    {
        return Err(LayoutError::InvalidGeometry(
            "base line height must be finite",
        ));
    }
    let mut base_style = options.base_style.clone();
    if base_style.measurement.size_points <= 0.0 {
        base_style.measurement.size_points = 11.0;
    }
    let mut runs = runs.to_vec();
    for run in &mut runs {
        if run.style.measurement.size_points <= 0.0 {
            run.style.measurement.size_points = 11.0;
        }
        typeset(run, measurer, options.word_exact_baseline);
    }
    let runs = small_caps(&runs);
    let wrapped = wrapping::wrap(&runs, options, measurer)?;
    let mut lines = Vec::new();
    for (index, wrapped) in wrapped.into_iter().enumerate() {
        let origin = if index == 0 {
            options.first_indent
        } else {
            0.0
        };
        let width = (options.width - origin).max(1.0);
        let frame = placement::LineFrame {
            width,
            origin,
            alignment: options.alignment,
            line_break: wrapped.line_break,
            kerning: options.layout_kerning,
            tab_stops: options.tab_stops.as_deref(),
            tab_origin: options.tab_origin,
        };
        let (x, width, placed) = placement::place(&wrapped.runs, &frame, measurer)?;
        let mut ordinary_size = base_style.measurement.size_points;
        let natural_height = options.spacing.height(&base_style);
        let mut height = options
            .base_line_height
            .filter(|height| *height > 0.0)
            .unwrap_or(natural_height);
        let mut base_height = height;
        if !matches!(options.spacing, LineSpacing::Exact(_)) {
            height = height.max(natural_height);
        }
        let (mut ascent, mut descent) = (base_style.word_ascent(), base_style.word_descent());
        for run in &wrapped.runs {
            if run.style.superscript || run.style.subscript {
                continue;
            }
            ordinary_size = ordinary_size.max(run.style.measurement.size_points);
            ascent = ascent.max(run.style.word_ascent());
            descent = descent.max(run.style.word_descent());
            if !matches!(options.spacing, LineSpacing::Exact(_)) {
                height = height.max(options.spacing.height(&run.style));
            }
        }
        if height <= 0.0 {
            height = base_style.measurement.size_points * 1.18 * 1.15 * 1.02;
            base_height = height;
        }
        let word_exact =
            options.word_exact_baseline && matches!(options.spacing, LineSpacing::Exact(_));
        // Word sets an automatic line's baseline below the line gap and
        // ascender of its tallest font. A multiple above one adds its space
        // beneath the descender, a multiple below one shrinks the line about
        // its baseline in proportion, and a minimum adds its space above.
        let mut baseline = match options.spacing {
            // PowerPoint sets a single line's baseline below its top by a
            // fixed part of the font size, whatever the font. A multiple above
            // one adds seven tenths of its space above the baseline, and a
            // multiple below one removes rather more of its space from there.
            LineSpacing::Multiple(multiple)
                if options.whole_point_baseline
                    && !options.word_exact_baseline
                    && multiple > 0.0 =>
            {
                let change = height - height / multiple;
                0.93 * ordinary_size + change * if multiple >= 1.0 { 0.7 } else { 0.87 }
            }
            _ if !options.word_exact_baseline => 0.5 * height + 0.3 * ordinary_size,
            LineSpacing::Exact(_) => 0.8 * height,
            LineSpacing::Multiple(multiple) => ascent * multiple.min(1.0),
            LineSpacing::AtLeast(_) => height - descent,
        };
        let formulas: Vec<_> = wrapped
            .runs
            .iter()
            .filter_map(|run| run.formula.as_ref())
            .collect();
        if !formulas.is_empty() {
            // Office measures an equation against the single-spaced line and
            // enlarges it once the equation comes within a clearance of an
            // edge, whatever the line spacing. PowerPoint keeps more room
            // above an equation than Word does. Word keeps more above a
            // display equation and less below it, and more on both sides of
            // one that opens with ordinary text.
            let display = formulas.iter().find(|formula| formula.display);
            let (above, below) = match (options.word_exact_baseline, display) {
                (true, Some(formula)) if formula.leading_text => (0.3156, 0.0924),
                (true, Some(_)) => (0.1636, 0.0104),
                (true, None) => (0.0866, 0.0334),
                (false, _) => (0.13, 0.06),
            };
            let (above, below) = (above * ordinary_size, below * ordinary_size);
            let single = std::iter::once(&base_style)
                .chain(
                    wrapped
                        .runs
                        .iter()
                        .filter(|run| !run.style.superscript && !run.style.subscript)
                        .map(|run| &run.style),
                )
                .map(|style| LineSpacing::Multiple(1.0).height(style))
                .fold(0.0, f64::max);
            let single_baseline = if options.word_exact_baseline {
                ascent
            } else {
                0.5 * single + 0.3 * ordinary_size
            };
            let ascent = formulas
                .iter()
                .map(|formula| formula.ascent)
                .fold(0.0, f64::max);
            let descent = formulas
                .iter()
                .map(|formula| formula.descent)
                .fold(0.0, f64::max);
            let raise = (ascent + above - single_baseline).max(0.0);
            let growth = (descent + below - (single - single_baseline)).max(0.0);
            match options.spacing {
                // PowerPoint keeps an exact height but lifts a line whose
                // equation reaches further below than half its rise above.
                LineSpacing::Exact(_) if !options.word_exact_baseline => {
                    baseline -= (growth - raise / 2.0).max(0.0);
                }
                LineSpacing::Exact(_) => {}
                LineSpacing::AtLeast(_) => {
                    // The minimum absorbs the enlargement until the line
                    // outgrows it.
                    let natural = single + raise + growth;
                    if natural > height {
                        height = natural;
                        baseline = single_baseline + raise;
                    }
                }
                LineSpacing::Multiple(_) => {
                    baseline += raise;
                    height += raise + growth;
                }
            }
        }
        if !height.is_finite() || !baseline.is_finite() {
            return Err(LayoutError::InvalidGeometry("line dimensions overflowed"));
        }
        let mut fragments = Vec::new();
        for placed in placed {
            if let Some(formula) = &placed.run.formula {
                fragments.push(TextFragment {
                    text: formula.text(),
                    kerned: false,
                    x: placed.x,
                    baseline,
                    top: baseline - formula.ascent,
                    height: formula.ascent + formula.descent,
                    width: placed.width,
                    advance: placed.advance,
                    font: formula.font.clone(),
                    missing_codepoints: Vec::new(),
                    link: placed.run.link,
                    tooltip: placed.run.tooltip,
                    internal_target: placed.run.internal_target,
                    anchors: placed.run.anchors,
                    current_page_field: None,
                    footnote_number: None,
                    formula: placed.run.formula.clone(),
                    style: placed.run.style,
                });
                continue;
            }
            if placed.run.fixed_advance.is_some() {
                continue;
            }
            let measured = measurer.measure(
                &placed.run.style.measurement,
                &placed.run.text,
                options.layout_kerning,
            )?;
            let measurement = measured.measurement;
            let style = &placed.run.style;
            let mut baseline = baseline;
            if style.subscript || style.superscript {
                if style.presentation_script {
                    if style.superscript {
                        baseline -= style.measurement.size_points * 0.42;
                    }
                    if style.subscript {
                        baseline += style.measurement.size_points * 0.26;
                    }
                } else if options.word_exact_baseline {
                    // Word moves a script by a part of the size of the text
                    // around it, whatever the line spacing.
                    if style.superscript {
                        baseline -= ordinary_size * style.word_superscript_rise.unwrap_or(0.275);
                    }
                    if style.subscript {
                        baseline += ordinary_size * 0.073;
                    }
                } else {
                    if !word_exact {
                        baseline += 0.3 * (style.measurement.size_points - ordinary_size);
                    }
                    if style.superscript {
                        baseline -= base_height * 0.22;
                    }
                    if style.subscript {
                        baseline += base_height * 0.18;
                    }
                }
            }
            fragments.push(TextFragment {
                link: placed.run.link,
                tooltip: placed.run.tooltip,
                internal_target: placed.run.internal_target,
                anchors: placed.run.anchors,
                current_page_field: placed.run.current_page_field,
                footnote_number: placed.run.footnote_number,
                formula: None,
                text: measurement.prepared_text,
                x: placed.x,
                baseline,
                top: baseline - measurement.extents.ascent,
                height: measurement.extents.ascent + measurement.extents.descent,
                width: placed.width,
                advance: placed.advance,
                kerned: options.layout_kerning,
                style: placed.run.style,
                font: measured.font,
                missing_codepoints: measurement.missing_codepoints,
            });
        }
        let (highlights, decorations) =
            super::decorations::plan(&fragments, height, base_height, options);
        lines.push(TextLine {
            x,
            baseline,
            height,
            width,
            line_break: wrapped.line_break,
            fragments,
            highlights,
            decorations,
        });
    }
    Ok(lines)
}

/// Replaces an equation by its typeset formula, which wraps as one object, or
/// leaves its source text when no math font is available.
fn typeset(run: &mut TextRun, measurer: &mut TextMeasurer<'_>, word: bool) {
    let Some(element) = &run.math else {
        return;
    };
    if run.formula.is_some() {
        return;
    }
    let attributes = &element.attributes;
    if crate::properties::text(attributes, "format") == "omml" {
        measurer.warn("raw Office Math is shown as its source text in native output");
        return;
    }
    let Some(font) = measurer.math_font() else {
        measurer.warn(
            "no math font (Cambria Math or STIX Two Math) is available; equations are shown as italic source text",
        );
        return;
    };
    let mut upright = run.style.measurement.clone();
    let (bold, _) = super::styles::emphasis(upright.variant);
    upright.variant = crate::measurement::FontVariant::from_emphasis(bold, false);
    let text_font = measurer
        .measure(&upright, "", false)
        .ok()
        .map(|measured| measured.font);
    let Some((formula, warnings)) = crate::math::typesetting::typeset(
        &crate::properties::text(attributes, "source"),
        crate::properties::text(attributes, "display") == "block",
        attributes.contains_key("lone"),
        word,
        run.style.measurement.size_points,
        &font,
        text_font.as_ref(),
    ) else {
        return;
    };
    for warning in warnings {
        measurer.warn(warning);
    }
    run.text = "\u{FFFC}".into();
    run.fixed_advance = Some(formula.width);
    run.formula = Some(std::sync::Arc::new(formula));
}

fn small_caps(runs: &[TextRun]) -> Vec<TextRun> {
    let mut output = Vec::new();
    for run in runs {
        if !run.style.small_caps || run.text.is_empty() || run.fixed_advance.is_some() {
            output.push(run.clone());
            continue;
        }
        let mut segment = String::new();
        let mut previous_small = None;
        for character in run.text.chars() {
            let small = unicode_general_category::get_general_category(character)
                == unicode_general_category::GeneralCategory::LowercaseLetter;
            if previous_small.is_some_and(|previous| previous != small) && !segment.is_empty() {
                append_caps(run, &segment, previous_small.unwrap(), &mut output);
                segment.clear();
            }
            let uppercase: Vec<_> = character.to_uppercase().collect();
            segment.push(if small && uppercase.len() == 1 {
                uppercase[0]
            } else {
                character
            });
            previous_small = Some(small);
        }
        if !segment.is_empty() {
            append_caps(run, &segment, previous_small.unwrap(), &mut output);
        }
    }
    output
}

fn append_caps(run: &TextRun, text: &str, small: bool, output: &mut Vec<TextRun>) {
    let mut run = run.with_text(text);
    run.style.small_caps = false;
    if small {
        run.style.measurement.size_points *= 0.8;
    }
    output.push(run);
}
