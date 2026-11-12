//! Single glyphs, size variants and glyph assemblies from the math font.
use super::{FormulaGlyph, Layout, Style, Typesetter};
use ttf_parser::{
    GlyphId,
    math::{GlyphAssembly, GlyphPart},
};

impl Typesetter<'_> {
    /// Falls back to the original character when the font lacks its
    /// mathematical form.
    pub(super) fn character(&mut self, mapped: char, original: char, style: Style) -> Layout {
        let found = self
            .face
            .glyph_index(mapped)
            .map(|glyph| (glyph, mapped))
            .or_else(|| {
                self.face
                    .glyph_index(original)
                    .map(|glyph| (glyph, original))
            });
        let (glyph, character) = found.unwrap_or_else(|| {
            self.missing.insert(original);
            (GlyphId(0), original)
        });
        self.glyph(glyph, character.to_string(), style)
    }
    pub(super) fn glyph(&self, glyph: GlyphId, text: String, style: Style) -> Layout {
        let scale = self.scale(style);
        let (ascent, descent) = self
            .face
            .glyph_bounding_box(glyph)
            .map(|bounds| {
                (
                    f64::from(bounds.y_max) * scale,
                    -f64::from(bounds.y_min) * scale,
                )
            })
            .unwrap_or_default();
        let info = self.glyph_info.as_ref();
        let italic = info
            .and_then(|info| info.italic_corrections)
            .and_then(|values| values.get(glyph))
            .map(|value| f64::from(value.value) * scale)
            .unwrap_or_default();
        let accent = info
            .and_then(|info| info.top_accent_attachments)
            .and_then(|values| values.get(glyph))
            .map(|value| f64::from(value.value) * scale);
        Layout {
            width: f64::from(self.face.glyph_hor_advance(glyph).unwrap_or_default()) * scale,
            ascent: ascent.max(0.0),
            descent: descent.max(0.0),
            glyphs: vec![FormulaGlyph {
                glyph: glyph.0,
                text,
                x: 0.0,
                rise: 0.0,
                size: self.size(style),
                in_text_font: false,
            }],
            rules: Vec::new(),
            italic,
            accent,
            single: true,
        }
    }
    pub(super) fn ink_centre(&self, layout: &Layout, style: Style) -> f64 {
        match layout.glyphs.as_slice() {
            [glyph] => self
                .face
                .glyph_bounding_box(GlyphId(glyph.glyph))
                .map(|bounds| {
                    f64::from(bounds.x_min + bounds.x_max) / 2.0 * self.scale(style) + glyph.x
                })
                .unwrap_or(layout.width / 2.0),
            _ => layout.width / 2.0,
        }
    }

    /// Returns the first size variant of a character whose extent reaches the
    /// target, or else an assembly of parts stretched to the target.
    pub(super) fn stretched(
        &mut self,
        character: char,
        target: f64,
        vertical: bool,
        style: Style,
    ) -> Layout {
        self.stretched_within(character, target, 0.0, vertical, style)
    }

    /// The whole vertical variants of a character in ascending order, each
    /// with its height and the height of its top above the baseline as
    /// designed.
    pub(super) fn vertical_variants(
        &self,
        character: char,
        style: Style,
    ) -> Vec<(GlyphId, f64, f64)> {
        let scale = self.scale(style);
        self.face
            .glyph_index(character)
            .and_then(|glyph| self.variants?.vertical_constructions.get(glyph))
            .map(|construction| {
                construction
                    .variants
                    .into_iter()
                    .map(|variant| {
                        let top = self
                            .face
                            .glyph_bounding_box(variant.variant_glyph)
                            .map_or(0.0, |bounds| f64::from(bounds.y_max) * scale);
                        (
                            variant.variant_glyph,
                            f64::from(variant.advance_measurement) * scale,
                            top,
                        )
                    })
                    .collect()
            })
            .unwrap_or_default()
    }

    /// Returns the first size variant of a character that falls no more than
    /// the shortfall short of the target, or else an assembly of parts
    /// stretched to the target.
    pub(super) fn stretched_within(
        &mut self,
        character: char,
        target: f64,
        shortfall: f64,
        vertical: bool,
        style: Style,
    ) -> Layout {
        let Some(glyph) = self.face.glyph_index(character) else {
            return self.character(character, character, style);
        };
        let text = character.to_string();
        let construction = self.variants.and_then(|variants| {
            if vertical {
                variants.vertical_constructions.get(glyph)
            } else {
                variants.horizontal_constructions.get(glyph)
            }
        });
        let Some(construction) = construction else {
            return self.glyph(glyph, text, style);
        };
        let scale = self.scale(style);
        let mut largest = glyph;
        for variant in construction.variants {
            largest = variant.variant_glyph;
            if f64::from(variant.advance_measurement) * scale >= target - shortfall {
                return self.glyph(variant.variant_glyph, text, style);
            }
        }
        match construction.assembly {
            Some(assembly) => self.assembly(assembly, target, shortfall, text, vertical, style),
            None => self.glyph(largest, text, style),
        }
    }

    /// Repeats the extenders of an assembly until it reaches the target, then
    /// overlaps the connectors evenly so that the assembly meets it exactly. If
    /// one more extender would overshoot the target by more than the shorter
    /// assembly falls short of it, the shorter assembly is taken. A shortfall
    /// replaces that comparison: the first assembly within it is taken.
    fn assembly(
        &self,
        assembly: GlyphAssembly<'_>,
        target: f64,
        shortfall: f64,
        text: String,
        vertical: bool,
        style: Style,
    ) -> Layout {
        let scale = self.scale(style);
        let minimum_overlap = self
            .variants
            .map(|variants| f64::from(variants.min_connector_overlap) * scale)
            .unwrap_or_default();
        let parts: Vec<_> = assembly.parts.into_iter().collect();
        let build = |repeats: usize| -> (Vec<GlyphPart>, f64, f64) {
            let sequence: Vec<_> = parts
                .iter()
                .flat_map(|part| {
                    let count = if part.part_flags.extender() {
                        repeats
                    } else {
                        1
                    };
                    std::iter::repeat_n(*part, count)
                })
                .collect();
            let full: f64 = sequence
                .iter()
                .map(|part| f64::from(part.full_advance) * scale)
                .sum();
            let maximum_overlap = sequence
                .windows(2)
                .map(|pair| {
                    f64::from(
                        pair[0]
                            .end_connector_length
                            .min(pair[1].start_connector_length),
                    ) * scale
                })
                .fold(f64::INFINITY, f64::min);
            (sequence, full, maximum_overlap)
        };
        let reach = |(sequence, full, _): &(Vec<GlyphPart>, f64, f64)| {
            full - sequence.len().saturating_sub(1) as f64 * minimum_overlap
        };
        let shortest = |(sequence, full, maximum): &(Vec<GlyphPart>, f64, f64)| {
            full - sequence.len().saturating_sub(1) as f64 * maximum.max(minimum_overlap)
        };
        let mut chosen = build(0);
        for repeats in 1..=64 {
            if reach(&chosen) >= target - shortfall {
                break;
            }
            let longer = build(repeats);
            if shortfall == 0.0
                && reach(&longer) >= target
                && shortest(&longer) - target > target - reach(&chosen)
            {
                break;
            }
            chosen = longer;
        }
        let (sequence, full, maximum_overlap) = chosen;
        let joins = sequence.len().saturating_sub(1);
        let overlap = if joins == 0 {
            0.0
        } else {
            ((full - target) / joins as f64)
                .min(maximum_overlap)
                .max(minimum_overlap)
        };
        let mut layout = Layout {
            italic: f64::from(assembly.italics_correction.value) * scale,
            ..Layout::default()
        };
        let mut position = 0.0;
        let mut extent = (f64::NEG_INFINITY, f64::NEG_INFINITY);
        for (index, part) in sequence.iter().enumerate() {
            let piece = self.glyph(
                part.glyph_id,
                if index == 0 {
                    text.clone()
                } else {
                    String::new()
                },
                style,
            );
            let bounds = self.face.glyph_bounding_box(part.glyph_id);
            if vertical {
                let bottom = bounds.map_or(0.0, |bounds| f64::from(bounds.y_min) * scale);
                layout.width = layout.width.max(piece.width);
                layout.place(piece, 0.0, position - bottom);
            } else {
                let left = bounds.map_or(0.0, |bounds| f64::from(bounds.x_min) * scale);
                extent = (extent.0.max(piece.ascent), extent.1.max(piece.descent));
                layout.place(piece, position - left, 0.0);
            }
            position += f64::from(part.full_advance) * scale - overlap;
        }
        if !vertical {
            // A horizontal assembly, such as a brace, may lie wholly above or
            // below its baseline.
            layout.width = position + overlap;
            (layout.ascent, layout.descent) = extent;
        }
        layout
    }
}
