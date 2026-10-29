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
            let Ok(bytes) = fs::read(&path) else {
                continue;
            };
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
    let Ok(face) = ttf_parser::Face::parse(bytes, 0) else {
        return false;
    };
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
