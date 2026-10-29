use ttf_parser::{
    Face, GlyphId, Tag,
    gpos::{PairAdjustment, PositioningSubtable},
    opentype_layout::LookupSubtable,
};

/// Pair positioning selected from the default Latin language system, falling
/// back to the default script. This is character-pair measurement, not shaping.
pub(super) struct Kerning<'font> {
    pairs: Vec<PairAdjustment<'font>>,
    legacy: Option<ttf_parser::kern::Subtable<'font>>,
}

impl<'font> Kerning<'font> {
    pub(super) fn new(face: &Face<'font>) -> Self {
        let mut pairs = Vec::new();
        if let Some(table) = face.tables().gpos {
            let language = [b"latn", b"DFLT"].into_iter().find_map(|tag| {
                table
                    .scripts
                    .find(Tag::from_bytes(tag))?
                    .default_language
                    .filter(|language| !language.feature_indices.is_empty())
            });
            if let Some(language) = language {
                for feature_index in language.feature_indices {
                    let Some(feature) = table.features.get(feature_index) else {
                        continue;
                    };
                    if feature.tag != Tag::from_bytes(b"kern") {
                        continue;
                    }
                    for lookup_index in feature.lookup_indices {
                        let Some(lookup) = table.lookups.get(lookup_index) else {
                            continue;
                        };
                        if lookup.flags.0 & 0x0010 != 0 {
                            continue;
                        }
                        for subtable_index in 0..lookup.subtables.len() {
                            if let Some(HorizontalPair(pair)) = lookup.subtables.get(subtable_index)
                            {
                                pairs.push(pair);
                            }
                        }
                    }
                }
            }
        }
        let legacy = face
            .raw_face()
            .table(Tag::from_bytes(b"kern"))
            .filter(|data| data.get(..2) == Some(&[0, 0]) && data.get(8..10) == Some(&[0, 1]))
            .and_then(|_| face.tables().kern)
            .and_then(|table| table.subtables.into_iter().next());
        Self { pairs, legacy }
    }

    pub(super) fn adjustment(&self, left: GlyphId, right: GlyphId) -> i16 {
        for pair in &self.pairs {
            let values = match pair {
                PairAdjustment::Format1 { coverage, sets } => coverage
                    .get(left)
                    .and_then(|index| sets.get(index))
                    .and_then(|set| set.get(right)),
                PairAdjustment::Format2 {
                    coverage,
                    classes,
                    matrix,
                } => coverage
                    .get(left)
                    .and_then(|_| matrix.get((classes.0.get(left), classes.1.get(right)))),
            };
            if let Some((first, _)) = values {
                return first.x_advance;
            }
        }
        if self.pairs.is_empty() {
            return self
                .legacy
                .as_ref()
                .and_then(|table| table.glyphs_kerning(left, right))
                .unwrap_or(0);
        }
        0
    }
}

struct HorizontalPair<'font>(PairAdjustment<'font>);

impl<'font> LookupSubtable<'font> for HorizontalPair<'font> {
    fn parse(mut data: &'font [u8], kind: u16) -> Option<Self> {
        if kind == 9 {
            if data.get(..4)? != [0, 1, 0, 2] {
                return None;
            }
            let offset = u32::from_be_bytes(data.get(4..8)?.try_into().ok()?) as usize;
            data = data.get(offset..)?;
        } else if kind != 2 {
            return None;
        }
        // Only first-glyph horizontal advance, without device adjustments.
        if data.get(4..8)? != [0, 4, 0, 0] {
            return None;
        }
        match PositioningSubtable::parse(data, 2)? {
            PositioningSubtable::Pair(pair) => Some(Self(pair)),
            _ => None,
        }
    }
}
