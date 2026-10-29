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
    if !options.width.is_finite()
        || !options.first_indent.is_finite()
        || !options.fallback_multiplier.is_finite()
        || options.fallback_multiplier <= 0.0
    {
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
    let mut runs = small_caps(runs);
    for run in &mut runs {
        if run.style.measurement.size_points <= 0.0 {
            run.style.measurement.size_points = 11.0;
        }
    }
    let wrapped = wrapping::wrap(&runs, options, measurer)?;
    let mut lines = Vec::new();
    for (index, wrapped) in wrapped.into_iter().enumerate() {
        let origin = if index == 0 {
            options.first_indent
        } else {
            0.0
        };
        let width = (options.width - origin).max(1.0);
        let (x, width, placed) = placement::place(
            &wrapped.runs,
            width,
            origin,
            options.alignment,
            wrapped.line_break,
            measurer,
            options.layout_kerning,
        )?;
        let mut ordinary_size = base_style.measurement.size_points;
        let natural_height = options
            .spacing
            .height(&base_style, options.fallback_multiplier);
        let mut height = options
            .base_line_height
            .filter(|height| *height > 0.0)
            .unwrap_or(natural_height);
        let mut base_height = height;
        if !matches!(options.spacing, LineSpacing::Exact(_)) {
            height = height.max(natural_height);
        }
        for run in &wrapped.runs {
            if run.style.superscript {
                continue;
            }
            ordinary_size = ordinary_size.max(run.style.measurement.size_points);
            if !matches!(options.spacing, LineSpacing::Exact(_)) {
                height = height.max(
                    options
                        .spacing
                        .height(&run.style, options.fallback_multiplier),
                );
            }
        }
        if height <= 0.0 {
            height = base_style.measurement.size_points * 1.18 * 1.15 * 1.02;
            base_height = height;
        }
        let baseline = 0.5 * height + 0.3 * ordinary_size;
        if !height.is_finite() || !baseline.is_finite() {
            return Err(LayoutError::InvalidGeometry("line dimensions overflowed"));
        }
        let mut fragments = Vec::new();
        for placed in placed {
            if placed.run.fixed_advance.is_some() {
                continue;
            }
            let measured = measurer.measure(
                &placed.run.style.measurement,
                &placed.run.text,
                options.layout_kerning,
            )?;
            let measurement = measured.measurement;
            let baseline = if placed.run.style.superscript {
                baseline + 0.3 * (placed.run.style.measurement.size_points - ordinary_size)
                    - base_height * 0.22
            } else {
                baseline
            };
            fragments.push(TextFragment {
                link: placed.run.link,
                internal_target: placed.run.internal_target,
                anchors: placed.run.anchors,
                current_page_field: placed.run.current_page_field,
                footnote_number: placed.run.footnote_number,
                text: measurement.prepared_text,
                x: placed.x,
                baseline,
                top: baseline - measurement.extents.ascent,
                height: measurement.extents.ascent + measurement.extents.descent,
                width: placed.width,
                advance: placed.advance,
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
