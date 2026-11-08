//! Fractions, radicals, scripts, limits, operators, fences, accents and arrays,
//! positioned by the rules of the OpenType MATH specification.
use super::{Layout, Level, Node, Style, TextKind, Typesetter};
use ttf_parser::math::MathValue;

impl Typesetter<'_> {
    /// The space TeX leaves on each side of a fraction, its null delimiters.
    fn fraction_padding(&self, style: Style) -> f64 {
        self.size(style) * 0.12
    }

    pub(super) fn fraction(
        &mut self,
        numerator: &Node,
        denominator: &Node,
        bar: bool,
        style: Style,
    ) -> Layout {
        let mut numerator = self.node(numerator, style.numerator());
        let mut denominator = self.node(denominator, style.denominator());
        let strut = self.character('(', '(', style.numerator());
        numerator.descent = numerator.descent.max(strut.descent);
        denominator.ascent = denominator.ascent.max(strut.ascent);
        let constants = self.constants;
        let scale = self.scale(style);
        let value = |value: MathValue<'_>| f64::from(value.value) * scale;
        let display = style.display();
        let padding = self.fraction_padding(style);
        let inner = numerator.width.max(denominator.width);
        let mut layout = Layout {
            width: inner + 2.0 * padding,
            ..Layout::default()
        };
        let axis = self.axis(style);
        let thickness = if bar {
            value(constants.fraction_rule_thickness())
        } else {
            0.0
        };
        let (shift_up, gap_above, shift_down, gap_below) = if display && !style.enclosed {
            (
                constants.fraction_numerator_display_style_shift_up(),
                constants.fraction_num_display_style_gap_min(),
                constants.fraction_denominator_display_style_shift_down(),
                constants.fraction_denom_display_style_gap_min(),
            )
        } else if style.full_fractions && style.display_gaps {
            (
                constants.fraction_numerator_shift_up(),
                constants.fraction_num_display_style_gap_min(),
                constants.fraction_denominator_shift_down(),
                constants.fraction_denom_display_style_gap_min(),
            )
        } else {
            (
                constants.fraction_numerator_shift_up(),
                constants.fraction_numerator_gap_min(),
                constants.fraction_denominator_shift_down(),
                constants.fraction_denominator_gap_min(),
            )
        };
        let up = value(shift_up).max(axis + thickness / 2.0 + value(gap_above) + numerator.descent);
        let down =
            value(shift_down).max(denominator.ascent + value(gap_below) - axis + thickness / 2.0);
        if bar {
            layout.rule(padding, axis - thickness / 2.0, inner, thickness);
        }
        let numerator_x = padding + (inner - numerator.width) / 2.0;
        let denominator_x = padding + (inner - denominator.width) / 2.0;
        layout.place(numerator, numerator_x, up);
        layout.place(denominator, denominator_x, -down);
        layout
    }

    pub(super) fn radical(
        &mut self,
        radicand: &Node,
        degree: Option<&Node>,
        style: Style,
    ) -> Layout {
        let body = self.node(radicand, style.cramped());
        let constants = self.constants;
        let scale = self.scale(style);
        let value = |value: MathValue<'_>| f64::from(value.value) * scale;
        let thickness = value(constants.radical_rule_thickness());
        let mut gap = value(if style.display() {
            constants.radical_display_style_vertical_gap()
        } else {
            constants.radical_vertical_gap()
        });
        let target = body.ascent + body.descent + gap + thickness;
        let surd = self.stretched('√', target, true, style);
        let surd_height = surd.ascent + surd.descent;
        if surd_height > target {
            gap += (surd_height - target) / 2.0;
        }
        let rule = body.ascent + gap;
        let surd_rise = rule + thickness - surd.ascent;
        let mut layout = Layout::default();
        let mut x = 0.0;
        if let Some(degree) = degree {
            let degree = self.node(
                degree,
                Style {
                    level: Level::ScriptScript,
                    cramped: false,
                    full_fractions: false,
                    display_gaps: false,
                    enclosed: false,
                },
            );
            let raise = f64::from(constants.radical_degree_bottom_raise_percent()) / 100.0;
            let before = value(constants.radical_kern_before_degree());
            let after = value(constants.radical_kern_after_degree());
            let width = degree.width;
            layout.place(
                degree,
                before,
                surd_rise - surd.descent + raise * surd_height,
            );
            x = (before + width + after).max(0.0);
        }
        let surd_width = surd.width;
        layout.place(surd, x, surd_rise);
        x += surd_width;
        let body_width = body.width;
        layout.place(body, x, 0.0);
        layout.rule(x, rule, body_width, thickness);
        layout.ascent = layout
            .ascent
            .max(rule + thickness + value(constants.radical_extra_ascender()));
        layout.width = x + body_width;
        layout
    }

    pub(super) fn scripts(
        &mut self,
        base: Layout,
        subscript: Option<&Node>,
        superscript: Option<&Node>,
        style: Style,
    ) -> Layout {
        let superscript = superscript.map(|node| self.node(node, style.script()));
        let subscript = subscript.map(|node| self.node(node, style.subscript()));
        let constants = self.constants;
        let scale = self.scale(style);
        let value = |value: MathValue<'_>| f64::from(value.value) * scale;
        let (mut up, mut down) = if base.single {
            (0.0, 0.0)
        } else {
            (
                base.ascent - value(constants.superscript_baseline_drop_max()),
                base.descent + value(constants.subscript_baseline_drop_min()),
            )
        };
        if let Some(superscript) = &superscript {
            let shift = if style.cramped {
                constants.superscript_shift_up_cramped()
            } else {
                constants.superscript_shift_up()
            };
            up = up
                .max(value(shift))
                .max(value(constants.superscript_bottom_min()) + superscript.descent);
        }
        if let Some(subscript) = &subscript {
            down = down
                .max(value(constants.subscript_shift_down()))
                .max(subscript.ascent - value(constants.subscript_top_max()));
        }
        if let (Some(superscript), Some(subscript)) = (&superscript, &subscript) {
            let gap = (up - superscript.descent) - (subscript.ascent - down);
            let minimum = value(constants.sub_superscript_gap_min());
            if gap < minimum {
                down += minimum - gap;
                let lift = value(constants.superscript_bottom_max_with_subscript())
                    - (up - superscript.descent);
                if lift > 0.0 {
                    up += lift;
                    down -= lift;
                }
            }
        }
        let (width, italic) = (base.width, base.italic);
        let mut layout = Layout::default();
        layout.place(base, 0.0, 0.0);
        let mut end = width;
        if let Some(superscript) = superscript {
            end = end.max(width + italic + superscript.width);
            layout.place(superscript, width + italic, up);
        }
        if let Some(subscript) = subscript {
            end = end.max(width + subscript.width);
            layout.place(subscript, width, -down);
        }
        layout.width = end + value(constants.space_after_script());
        layout
    }

    /// Places limits above and below a base, centred, with the upper limit
    /// shifted right and the lower left by half the base's italic correction.
    pub(super) fn limits(
        &mut self,
        base: Layout,
        over: Option<Layout>,
        under: Option<Layout>,
        style: Style,
    ) -> Layout {
        let constants = self.constants;
        let scale = self.scale(style);
        let value = |value: MathValue<'_>| f64::from(value.value) * scale;
        let width = [Some(&base), over.as_ref(), under.as_ref()]
            .into_iter()
            .flatten()
            .map(|layout| layout.width)
            .fold(0.0, f64::max);
        let shift = base.italic / 2.0;
        let (base_ascent, base_descent) = (base.ascent, base.descent);
        let mut layout = Layout {
            width,
            ..Layout::default()
        };
        if let Some(over) = over {
            let gap = value(constants.upper_limit_gap_min())
                .max(value(constants.upper_limit_baseline_rise_min()) - over.descent);
            let rise = base_ascent + gap + over.descent;
            let x = (width - over.width) / 2.0 + shift;
            layout.place(over, x, rise);
        }
        if let Some(under) = under {
            let gap = value(constants.lower_limit_gap_min())
                .max(value(constants.lower_limit_baseline_drop_min()) - under.ascent);
            let drop = base_descent + gap + under.ascent;
            let x = (width - under.width) / 2.0 - shift;
            layout.place(under, x, -drop);
        }
        let x = (width - base.width) / 2.0;
        layout.place(base, x, 0.0);
        layout
    }

    pub(super) fn operator(&mut self, symbol: &str, style: Style) -> Layout {
        let character = symbol.chars().next().unwrap_or('∑');
        let mut layout = if style.display() {
            // Word sets Cambria Math's display operators between 1.11 and 1.56
            // times DisplayOperatorMinHeight tall; every factor in that range
            // selects the same variants.
            let minimum =
                1.3 * f64::from(self.constants.display_operator_min_height()) * self.scale(style);
            self.stretched(character, minimum, true, style)
        } else {
            self.character(character, character, style)
        };
        if symbol.chars().count() > 1 && self.face.glyph_index(character).is_none() {
            layout = self.word(symbol, style);
        }
        let italic = layout.italic;
        let mut centred = layout.centred(self.axis(style));
        centred.italic = italic;
        centred
    }

    pub(super) fn fenced(&mut self, begin: &str, end: &str, body: &Node, style: Style) -> Layout {
        let inner = match body {
            Node::Matrix(rows) => self.matrix(rows, style, begin == "{" && end.is_empty()),
            body => self.node(body, style.enclosed()),
        };
        let axis = self.axis(style);
        let reach = (inner.ascent - axis).max(inner.descent + axis);
        let target = (2.0 * reach * 0.901).max(2.0 * reach - self.size(style) * 0.5);
        let mut layout = Layout::default();
        let mut x = 0.0;
        let null = self.fraction_padding(style);
        let delimiter =
            |typesetter: &mut Self, symbol: &str, layout: &mut Layout, x: &mut f64| match symbol
                .chars()
                .next()
            {
                Some(character) => {
                    let glyph = typesetter
                        .stretched(character, target, true, style)
                        .centred(axis);
                    let width = glyph.width;
                    layout.place(glyph, *x, 0.0);
                    *x += width;
                }
                None => *x += null,
            };
        delimiter(self, begin, &mut layout, &mut x);
        let inner_width = inner.width;
        layout.place(inner, x, 0.0);
        x += inner_width;
        delimiter(self, end, &mut layout, &mut x);
        layout.width = x;
        layout
    }

    pub(super) fn accent(&mut self, character: &str, body: &Node, style: Style) -> Layout {
        let base = self.node(body, style.cramped());
        let mark = character.chars().next().unwrap_or('\u{302}');
        let accent = if !base.single && matches!(mark, '\u{302}' | '\u{303}') {
            self.stretched(mark, base.width, false, style)
        } else {
            self.character(mark, mark, style)
        };
        let base_attachment = base.accent.unwrap_or(base.width / 2.0);
        let accent_attachment = accent
            .accent
            .unwrap_or_else(|| self.ink_centre(&accent, style));
        let rise = (base.ascent - self.value(self.constants.accent_base_height(), style)).max(0.0);
        let mut layout = Layout {
            width: base.width,
            italic: base.italic,
            ..Layout::default()
        };
        layout.place(base, 0.0, 0.0);
        layout.place(accent, base_attachment - accent_attachment, rise);
        layout
    }

    pub(super) fn bar(&mut self, over: bool, body: &Node, style: Style) -> Layout {
        let body = self.node(body, if over { style.cramped() } else { style });
        let constants = self.constants;
        let scale = self.scale(style);
        let value = |value: MathValue<'_>| f64::from(value.value) * scale;
        let mut layout = Layout {
            width: body.width,
            italic: body.italic,
            ..Layout::default()
        };
        let (ascent, descent, width) = (body.ascent, body.descent, body.width);
        layout.place(body, 0.0, 0.0);
        if over {
            let thickness = value(constants.overbar_rule_thickness());
            let rise = ascent + value(constants.overbar_vertical_gap());
            layout.rule(0.0, rise, width, thickness);
            layout.ascent += value(constants.overbar_extra_ascender());
        } else {
            let thickness = value(constants.underbar_rule_thickness());
            let drop = descent + value(constants.underbar_vertical_gap()) + thickness;
            layout.rule(0.0, -drop, width, thickness);
            layout.descent += value(constants.underbar_extra_descender());
        }
        layout
    }

    pub(super) fn brace(
        &mut self,
        character: &str,
        over: bool,
        body: &Node,
        style: Style,
    ) -> Layout {
        let body = self.node(body, style);
        let brace = self.stretched(
            character.chars().next().unwrap_or('⏞'),
            body.width,
            false,
            style,
        );
        let constants = self.constants;
        let scale = self.scale(style);
        let value = |value: MathValue<'_>| f64::from(value.value) * scale;
        let width = body.width.max(brace.width);
        let mut layout = Layout {
            width,
            ..Layout::default()
        };
        let (ascent, descent) = (body.ascent, body.descent);
        let brace_x = (width - brace.width) / 2.0;
        let body_x = (width - body.width) / 2.0;
        if over {
            // Word sets the top of an overbrace 0.44 em above the body, though
            // the brace's bounding box reaches down to its baseline.
            let rise = ascent + 0.44 * self.size(style) - brace.ascent;
            layout.place(brace, brace_x, rise);
        } else {
            let drop = descent + value(constants.stretch_stack_gap_above_min()) + brace.ascent;
            layout.place(brace, brace_x, -drop);
        }
        layout.place(body, body_x, 0.0);
        layout
    }

    /// The cells of `cases` align left; those of other matrices are centred.
    pub(super) fn matrix(&mut self, rows: &[Vec<Node>], style: Style, left: bool) -> Layout {
        let cells: Vec<Vec<Layout>> = rows
            .iter()
            .map(|row| row.iter().map(|cell| self.node(cell, style)).collect())
            .collect();
        let em = self.size(style);
        self.grid(cells, em, em, |_| if left { 0.0 } else { 0.5 }, true, style)
    }

    /// Columns alternate between right and left alignment, as in TeX's `aligned`.
    pub(super) fn equations(&mut self, rows: &[Node], style: Style) -> Layout {
        let mut cells = Vec::new();
        for row in rows {
            let nodes = match row {
                Node::Sequence(nodes) => nodes.as_slice(),
                node => std::slice::from_ref(node),
            };
            let mut row_cells = Vec::new();
            for (index, part) in nodes
                .split(|node| matches!(node, Node::Text(text, TextKind::Ordinary) if text == "&"))
                .enumerate()
            {
                let part = match part {
                    [Node::Sequence(nodes)] => nodes.as_slice(),
                    part => part,
                };
                row_cells.push(self.list(part, style, index > 0));
            }
            cells.push(row_cells);
        }
        let columns = cells.iter().map(Vec::len).max().unwrap_or(0);
        let em = self.size(style);
        let alignments = move |column: usize| {
            if columns == 1 {
                0.5
            } else if column.is_multiple_of(2) {
                1.0
            } else {
                0.0
            }
        };
        self.grid(cells, 0.0, 2.0 * em, alignments, false, style)
    }

    /// Leaves `gap` between columns, or `pair_gap` before each later pair of
    /// aligned columns. Word separates the ink of successive rows by 0.38 em,
    /// with their baselines at least a line of the math font apart. A matrix
    /// row reaches at least the descent of that line; an aligned row reaches
    /// only as deep as its ink.
    fn grid(
        &mut self,
        cells: Vec<Vec<Layout>>,
        gap: f64,
        pair_gap: f64,
        alignment: impl Fn(usize) -> f64,
        matrix: bool,
        style: Style,
    ) -> Layout {
        let scale = self.scale(style);
        let hhea = self.face.tables().hhea;
        let strut = (
            f64::from(hhea.ascender) * scale,
            -f64::from(hhea.descender) * scale,
        );
        let columns = cells.iter().map(Vec::len).max().unwrap_or(0);
        let mut widths = vec![0.0f64; columns];
        for row in &cells {
            for (column, cell) in row.iter().enumerate() {
                widths[column] = widths[column].max(cell.width);
            }
        }
        let mut starts = Vec::with_capacity(columns);
        let mut x = 0.0;
        for (column, width) in widths.iter().enumerate() {
            if column > 0 {
                x += if gap > 0.0 {
                    gap
                } else if column.is_multiple_of(2) {
                    pair_gap
                } else {
                    0.0
                };
            }
            starts.push(x);
            x += width;
        }
        let ink: Vec<(f64, f64)> = cells
            .iter()
            .map(|row| {
                row.iter()
                    .fold((0.0f64, 0.0f64), |(ascent, descent), cell| {
                        (ascent.max(cell.ascent), descent.max(cell.descent))
                    })
            })
            .collect();
        let row_gap = 0.38 * self.size(style);
        let mut baselines = vec![0.0];
        for pair in ink.windows(2) {
            let above = if matrix {
                pair[0].1.max(strut.1)
            } else {
                pair[0].1
            };
            let pitch = (above + pair[1].0 + row_gap).max(strut.0 + strut.1);
            baselines.push(baselines.last().unwrap() - pitch);
        }
        let top = ink.first().map_or(0.0, |row| row.0.max(strut.0));
        let bottom = ink.last().map_or(0.0, |row| row.1.max(strut.1));
        let height = top - baselines.last().unwrap() + bottom;
        let shift = height / 2.0 + self.axis(style) - top;
        let mut layout = Layout {
            width: x,
            ..Layout::default()
        };
        for (row, baseline) in cells.into_iter().zip(baselines) {
            for (column, cell) in row.into_iter().enumerate() {
                let offset = (widths[column] - cell.width) * alignment(column);
                layout.place(cell, starts[column] + offset, baseline + shift);
            }
        }
        layout
    }
}
