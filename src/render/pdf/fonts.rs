use super::{Objects, RenderError, compress};
use crate::{layout::DocumentTextPlan, measurement::FontSelection, render::fonts::core_pdf_name};
use pdf_writer::{
    Filter, Finish, Name, Pdf, Rect, Ref, Str,
    types::{CidFontType, FontFlags, SystemInfo, UnicodeCmap},
};
use std::collections::{BTreeMap, BTreeSet};
use subsetter::GlyphRemapper;
use ttf_parser::{Face, Tag};

pub(super) struct Font {
    pub selection: FontSelection,
    pub resource: String,
    pub reference: Ref,
    codes: BTreeMap<char, u16>,
}
impl Font {
    pub fn code(&self, character: char) -> Result<u16, RenderError> {
        self.codes.get(&character).copied().ok_or_else(|| {
            RenderError::Invalid(format!(
                "missing PDF character mapping for U+{:04X}",
                character as u32
            ))
        })
    }
}
pub(super) struct Fonts {
    pub entries: Vec<Font>,
}
impl Fonts {
    pub fn find(&self, selection: &FontSelection) -> Result<&Font, RenderError> {
        self.entries
            .iter()
            .find(|font| font.selection == *selection)
            .ok_or_else(|| RenderError::Invalid("unregistered PDF font".into()))
    }
    pub fn write(
        plan: &DocumentTextPlan,
        document: &mut Pdf,
        objects: &mut Objects,
    ) -> Result<Self, RenderError> {
        let mut selections: Vec<(FontSelection, BTreeSet<char>)> = Vec::new();
        for fragment in plan
            .pages
            .iter()
            .flat_map(|page| page.text_lines())
            .flat_map(|line| &line.fragments)
        {
            if let Some((_, characters)) = selections
                .iter_mut()
                .find(|(selection, _)| *selection == fragment.font)
            {
                characters.extend(fragment.text.chars());
            } else {
                selections.push((fragment.font.clone(), fragment.text.chars().collect()));
            }
        }
        let mut entries = Vec::new();
        for (index, (selection, characters)) in selections.into_iter().enumerate() {
            let reference = objects.allocate();
            let codes = if let Some(bytes) = &selection.bytes {
                embed_font(document, objects, reference, bytes, &characters, index)?
            } else {
                document
                    .type1_font(reference)
                    .base_font(Name(core_pdf_name(&selection)?.as_bytes()))
                    .encoding_predefined(Name(b"WinAnsiEncoding"));
                BTreeMap::new()
            };
            entries.push(Font {
                selection,
                resource: format!("F{}", index + 1),
                reference,
                codes,
            });
        }
        Ok(Self { entries })
    }
}
fn embed_font(
    document: &mut Pdf,
    objects: &mut Objects,
    reference: Ref,
    bytes: &[u8],
    characters: &BTreeSet<char>,
    index: usize,
) -> Result<BTreeMap<char, u16>, RenderError> {
    let face =
        Face::parse(bytes, 0).map_err(|_| RenderError::Invalid("invalid PDF font".into()))?;
    let mut remapper = GlyphRemapper::new();
    let mut codes = BTreeMap::new();
    for character in characters {
        let glyph = face
            .glyph_index(*character)
            .ok_or_else(|| RenderError::Invalid("unprepared embedded-font text".into()))?;
        codes.insert(*character, remapper.remap(glyph.0));
    }
    let subset = subsetter::subset(bytes, 0, &remapper)
        .map_err(|error| RenderError::Invalid(format!("font subsetting failed: {error:?}")))?;
    let compact = face.tables().cff.is_some();
    let program = if compact {
        ttf_parser::RawFace::parse(&subset, 0)
            .map_err(|_| RenderError::Invalid("invalid font subset".into()))?
            .table(Tag::from_bytes(b"CFF "))
            .ok_or_else(|| RenderError::Invalid("CFF subset lacks outlines".into()))?
    } else {
        &subset
    };
    let descendant = objects.allocate();
    let descriptor = objects.allocate();
    let stream = objects.allocate();
    let unicode = objects.allocate();
    let name = format!("MOTEAA+NativeFont{index}");
    document
        .type0_font(reference)
        .base_font(Name(name.as_bytes()))
        .encoding_predefined(Name(b"Identity-H"))
        .descendant_font(descendant)
        .to_unicode(unicode);
    let info = SystemInfo {
        registry: Str(b"Adobe"),
        ordering: Str(b"Identity"),
        supplement: 0,
    };
    let mut cid = document.cid_font(descendant);
    cid.subtype(if compact {
        CidFontType::Type0
    } else {
        CidFontType::Type2
    })
    .base_font(Name(name.as_bytes()))
    .system_info(info)
    .font_descriptor(descriptor)
    .default_width(0.0);
    if !compact {
        cid.cid_to_gid_map_predefined(Name(b"Identity"));
    }
    let scale = 1000.0 / f64::from(face.units_per_em());
    let mut widths = BTreeMap::new();
    for character in characters {
        let glyph = face.glyph_index(*character).unwrap();
        widths.insert(
            codes[character],
            (f64::from(face.glyph_hor_advance(glyph).unwrap_or(0)) * scale).round() as f32,
        );
    }
    {
        let mut array = cid.widths();
        for (code, width) in widths {
            array.consecutive(code, [width]);
        }
    }
    cid.finish();
    let bounds = face.global_bounding_box();
    let scaled = |value: i16| (f64::from(value) * scale) as f32;
    let mut description = document.font_descriptor(descriptor);
    description
        .name(Name(name.as_bytes()))
        .flags(FontFlags::SYMBOLIC)
        .bbox(Rect::new(
            scaled(bounds.x_min),
            scaled(bounds.y_min),
            scaled(bounds.x_max),
            scaled(bounds.y_max),
        ))
        .italic_angle(face.italic_angle())
        .ascent(scaled(face.ascender()))
        .descent(scaled(face.descender()))
        .cap_height(scaled(face.capital_height().unwrap_or(face.ascender())))
        .stem_v(if face.is_bold() { 120.0 } else { 80.0 });
    if compact {
        description.font_file3(stream);
    } else {
        description.font_file2(stream);
    }
    description.finish();
    let compressed = compress(program)?;
    let mut font_stream = document.stream(stream, &compressed);
    font_stream.filter(Filter::FlateDecode);
    if compact {
        font_stream.pair(Name(b"Subtype"), Name(b"CIDFontType0C"));
    } else {
        font_stream.pair(Name(b"Length1"), program.len() as i32);
    }
    font_stream.finish();
    let mut mapping = UnicodeCmap::new(
        Name(b"MoteUnicode"),
        SystemInfo {
            registry: Str(b"Adobe"),
            ordering: Str(b"UCS"),
            supplement: 0,
        },
    );
    let mut mapped = BTreeSet::new();
    for (character, code) in &codes {
        if mapped.insert(*code) {
            mapping.pair(*code, *character);
        }
    }
    let encoded = mapping.finish();
    document.cmap(unicode, &encoded);
    Ok(codes)
}
