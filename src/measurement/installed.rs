use super::{FontVariant, OpenTypeFont, resolution::validate_embedding};
use crate::source::ResolvedAsset;
use std::{fs, path::PathBuf};

/// Supplies an exact named font face. Missing variants must not be synthesized.
pub trait FontResolver {
    fn resolve_font(
        &self,
        family: &str,
        variant: FontVariant,
    ) -> Result<Option<ResolvedAsset>, String>;
}
impl<Resolver> FontResolver for Resolver
where
    Resolver: Fn(&str, FontVariant) -> Result<Option<ResolvedAsset>, String>,
{
    fn resolve_font(
        &self,
        family: &str,
        variant: FontVariant,
    ) -> Result<Option<ResolvedAsset>, String> {
        self(family, variant)
    }
}

/// A deliberately narrow, deterministic catalogue of installed Office fonts.
/// Word's bundled face takes precedence over PowerPoint and system copies.
pub struct SystemFontResolver {
    home_directory: Option<PathBuf>,
}
impl Default for SystemFontResolver {
    fn default() -> Self {
        Self::new()
    }
}
impl SystemFontResolver {
    pub fn new() -> Self {
        Self {
            home_directory: std::env::var_os("HOME").map(PathBuf::from),
        }
    }
    fn candidates(&self, family: &str, variant: FontVariant) -> Vec<PathBuf> {
        let variant_index = match variant {
            FontVariant::Regular => 0,
            FontVariant::Bold => 1,
            FontVariant::Italic => 2,
            FontVariant::BoldItalic => 3,
        };
        let (application, system) = match family {
            "aptos" => (
                [
                    "Aptos.ttf",
                    "Aptos-Bold.ttf",
                    "Aptos-Italic.ttf",
                    "Aptos-Bold-Italic.ttf",
                ][variant_index],
                None,
            ),
            "arial" => (
                ["arial.ttf", "arialbd.ttf", "ariali.ttf", "arialbi.ttf"][variant_index],
                Some(
                    [
                        "Arial.ttf",
                        "Arial Bold.ttf",
                        "Arial Italic.ttf",
                        "Arial Bold Italic.ttf",
                    ][variant_index],
                ),
            ),
            "courier new" => (
                ["cour.ttf", "courbd.ttf", "couri.ttf", "courbi.ttf"][variant_index],
                Some(
                    [
                        "Courier New.ttf",
                        "Courier New Bold.ttf",
                        "Courier New Italic.ttf",
                        "Courier New Bold Italic.ttf",
                    ][variant_index],
                ),
            ),
            "cambria math" if variant == FontVariant::Regular => ("Cambria.ttc", None),
            "stix two math" if variant == FontVariant::Regular => {
                return vec![PathBuf::from(
                    "/System/Library/Fonts/Supplemental/STIXTwoMath.otf",
                )];
            }
            "aptos display" => return self.cloud_candidates(),
            _ => return Vec::new(),
        };
        let mut candidates = vec![
            PathBuf::from("/Applications/Microsoft Word.app/Contents/Resources/DFonts")
                .join(application),
            PathBuf::from("/Applications/Microsoft PowerPoint.app/Contents/Resources/DFonts")
                .join(application),
        ];
        if let Some(system) = system {
            candidates.push(PathBuf::from("/System/Library/Fonts/Supplemental").join(system));
        }
        candidates
    }
    fn cloud_candidates(&self) -> Vec<PathBuf> {
        let Some(home_directory) = &self.home_directory else {
            return Vec::new();
        };
        let directory = home_directory.join("Library/Group Containers/UBF8T346G9.Office/FontCache");
        let mut candidates = Vec::new();
        if let Ok(entries) = fs::read_dir(directory) {
            for entry in entries.flatten() {
                let directory = entry.path().join("CloudFonts/Aptos Display");
                if let Ok(files) = fs::read_dir(directory) {
                    candidates.extend(files.flatten().map(|entry| entry.path()).filter(|path| {
                        path.extension().is_some_and(|extension| extension == "ttf")
                    }));
                }
            }
        }
        candidates.sort();
        candidates
    }
}
impl FontResolver for SystemFontResolver {
    fn resolve_font(
        &self,
        family: &str,
        variant: FontVariant,
    ) -> Result<Option<ResolvedAsset>, String> {
        let family = normalize_family(family);
        let mut reason = None;
        for path in self.candidates(&family, variant) {
            if !path.is_file() {
                continue;
            }
            let Ok(mut bytes) = fs::read(&path) else {
                continue;
            };
            if bytes.starts_with(b"ttcf") {
                match collection_face(&bytes, &family, variant) {
                    Some(face) => bytes = face,
                    None => continue,
                }
            }
            if let Err(error) = validate_embedding(&bytes) {
                reason.get_or_insert(error);
                continue;
            }
            if matches_face(&bytes, &family, variant) && OpenTypeFont::parse(&bytes).is_ok() {
                return Ok(Some(ResolvedAsset {
                    name: path.into_os_string(),
                    bytes,
                }));
            }
        }
        match reason {
            Some(reason) => Err(format!(
                "installed font {family:?} {} cannot be embedded: {reason}",
                variant.as_text()
            )),
            None => Ok(None),
        }
    }
}
fn normalize_family(family: &str) -> String {
    family
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ")
        .to_lowercase()
}
fn matches_face(bytes: &[u8], family: &str, variant: FontVariant) -> bool {
    ttf_parser::Face::parse(bytes, 0).is_ok_and(|face| face_matches(&face, family, variant))
}
fn face_matches(face: &ttf_parser::Face<'_>, family: &str, variant: FontVariant) -> bool {
    let mut family_matches = false;
    let mut variant_matches = false;
    for name in face.names() {
        let Some(text) = name.to_string() else {
            continue;
        };
        if [16, 21, 1].contains(&name.name_id) && normalize_family(&text) == family {
            family_matches = true;
        }
        if [17, 22, 2].contains(&name.name_id) && exact_variant(&text) == Some(variant) {
            variant_matches = true;
        }
    }
    family_matches && variant_matches
}
/// Copies the matching face of a font collection into a standalone font, since
/// measurement and embedding read the first face of a font file.
fn collection_face(bytes: &[u8], family: &str, variant: FontVariant) -> Option<Vec<u8>> {
    let count = ttf_parser::fonts_in_collection(bytes)?;
    let index = (0..count).find(|index| {
        ttf_parser::Face::parse(bytes, *index)
            .is_ok_and(|face| face_matches(&face, family, variant))
    })?;
    let read = |offset: usize| -> Option<u32> {
        Some(u32::from_be_bytes(
            bytes.get(offset..offset + 4)?.try_into().ok()?,
        ))
    };
    let start = read(12 + 4 * index as usize)? as usize;
    let header = bytes.get(start..start + 12)?;
    let table_count = usize::from(u16::from_be_bytes([header[4], header[5]]));
    let records = bytes.get(start + 12..start + 12 + 16 * table_count)?;
    let mut output = header.to_vec();
    let mut data = Vec::new();
    let mut offset = 12 + 16 * table_count;
    for record in records.as_chunks::<16>().0 {
        let source = u32::from_be_bytes(record[8..12].try_into().ok()?) as usize;
        let length = u32::from_be_bytes(record[12..16].try_into().ok()?) as usize;
        data.extend_from_slice(bytes.get(source..source.checked_add(length)?)?);
        data.resize(data.len().next_multiple_of(4), 0);
        output.extend_from_slice(&record[..8]);
        output.extend_from_slice(&(offset as u32).to_be_bytes());
        output.extend_from_slice(&record[12..16]);
        offset = 12 + 16 * table_count + data.len();
    }
    output.extend(data);
    Some(output)
}
fn exact_variant(name: &str) -> Option<FontVariant> {
    let mut bold = false;
    let mut italic = false;
    for token in name
        .to_lowercase()
        .split(|character: char| character.is_whitespace() || matches!(character, '-' | '_'))
    {
        match token {
            "" | "regular" | "roman" | "normal" | "display" => {}
            "bold" => bold = true,
            "italic" | "oblique" => italic = true,
            _ => return None,
        }
    }
    Some(FontVariant::from_emphasis(bold, italic))
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn exact_faces_reject_condensed_and_intermediate_weights() {
        assert_eq!(exact_variant("Bold Italic"), Some(FontVariant::BoldItalic));
        assert_eq!(exact_variant("Regular"), Some(FontVariant::Regular));
        for name in [
            "Semibold",
            "Extra Bold",
            "Light Italic",
            "Condensed",
            "Black",
        ] {
            assert_eq!(exact_variant(name), None);
        }
        let bytes =
            include_bytes!("../../tests/fixtures/measurements/fonts/SourceSans3-Regular.ttf");
        assert!(matches_face(bytes, "source sans 3", FontVariant::Regular));
        assert!(!matches_face(bytes, "arial", FontVariant::Regular));
        assert!(!matches_face(bytes, "source sans 3", FontVariant::Bold));
    }
    #[test]
    fn installed_candidates_preserve_authority_order_and_variant() {
        let resolver = SystemFontResolver::default();
        let candidates = resolver.candidates("arial", FontVariant::BoldItalic);
        assert_eq!(
            candidates[0],
            PathBuf::from("/Applications/Microsoft Word.app/Contents/Resources/DFonts/arialbi.ttf")
        );
        assert_eq!(
            candidates[1],
            PathBuf::from(
                "/Applications/Microsoft PowerPoint.app/Contents/Resources/DFonts/arialbi.ttf"
            )
        );
        assert_eq!(
            candidates[2],
            PathBuf::from("/System/Library/Fonts/Supplemental/Arial Bold Italic.ttf")
        );
        assert!(
            resolver
                .candidates("helvetica", FontVariant::Regular)
                .is_empty()
        );
    }
}
