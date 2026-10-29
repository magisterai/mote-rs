use super::Error;
use std::io::{Cursor, Write};
use zip::{ZipWriter, write::SimpleFileOptions};

pub(super) fn write(
    document: &str,
    styles: &str,
    font_table: &str,
    numbering: &str,
    relationships: &str,
    media: &[(String, crate::media::ImageAsset)],
    stories: &[super::sections::StoryPart],
) -> Result<Vec<u8>, Error> {
    let now = time::OffsetDateTime::now_utc();
    let timestamp = format!(
        "{:04}-{:02}-{:02}T{:02}:{:02}:{:02}Z",
        now.year(),
        u8::from(now.month()),
        now.day(),
        now.hour(),
        now.minute(),
        now.second()
    );
    let core_properties = format!(
        r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<cp:coreProperties xmlns:cp="http://schemas.openxmlformats.org/package/2006/metadata/core-properties" xmlns:dc="http://purl.org/dc/elements/1.1/" xmlns:dcterms="http://purl.org/dc/terms/" xmlns:dcmitype="http://purl.org/dc/dcmitype/" xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance">
  <dc:title/>
  <dc:creator>Mote</dc:creator>
  <cp:lastModifiedBy>Mote</cp:lastModifiedBy>
  <dcterms:created xsi:type="dcterms:W3CDTF">{timestamp}</dcterms:created>
  <dcterms:modified xsi:type="dcterms:W3CDTF">{timestamp}</dcterms:modified>
</cp:coreProperties>"#
    );
    let mut archive = ZipWriter::new(Cursor::new(Vec::new()));
    let options = SimpleFileOptions::default().compression_method(zip::CompressionMethod::Deflated);
    let mut content_types = CONTENT_TYPES.to_owned();
    let mut types = std::collections::BTreeMap::new();
    for (_, asset) in media {
        types.insert(asset.extension.trim_start_matches('.'), asset.content_type);
    }
    let entries = types
        .iter()
        .map(|(extension, content_type)| {
            format!("<Default Extension=\"{extension}\" ContentType=\"{content_type}\"/>")
        })
        .collect::<String>();
    content_types = content_types.replace(
        "  <Override PartName=\"/word/document.xml\"",
        &format!("{entries}  <Override PartName=\"/word/document.xml\""),
    );
    for story in stories
        .iter()
        .filter(|story| story.kind == "header")
        .chain(stories.iter().filter(|story| story.kind == "footer"))
    {
        content_types = content_types.replace("</Types>", &format!(r#"<Override PartName="/word/{}" ContentType="application/vnd.openxmlformats-officedocument.wordprocessingml.{}+xml"/></Types>"#, story.name, story.kind));
    }
    for kind in ["footnotes", "endnotes"] {
        if let Some(story) = stories.iter().find(|story| story.kind == kind) {
            let marker = r#"  <Override PartName="/docProps/core.xml""#;
            content_types = content_types.replace(marker, &format!(r#"<Override PartName="/word/{}" ContentType="application/vnd.openxmlformats-officedocument.wordprocessingml.{kind}+xml"/>{marker}"#, story.name));
        }
    }
    let mut settings = SETTINGS.to_owned();
    for kind in ["footnote", "endnote"] {
        if stories.iter().any(|story| story.kind == format!("{kind}s")) {
            settings = settings.replace("  <w:compat>", &format!(r#"<w:{kind}Pr><w:{kind} w:id="-1"/><w:{kind} w:id="0"/></w:{kind}Pr>  <w:compat>"#));
        }
    }
    for (name, contents) in [
        ("[Content_Types].xml", content_types.as_str()),
        ("_rels/.rels", ROOT_RELATIONSHIPS),
        ("docProps/app.xml", APP_PROPERTIES),
        ("docProps/core.xml", &core_properties),
        ("word/document.xml", document),
        ("word/_rels/document.xml.rels", relationships),
        ("word/styles.xml", styles),
        ("word/settings.xml", &settings),
        ("word/fontTable.xml", font_table),
        ("word/numbering.xml", numbering),
    ] {
        archive.start_file(name, options)?;
        archive.write_all(contents.as_bytes())?;
    }
    for story in stories {
        archive.start_file(format!("word/{}", story.name), options)?;
        archive.write_all(story.content.as_bytes())?;
        if let Some(relationships) = &story.relationships {
            archive.start_file(format!("word/_rels/{}.rels", story.name), options)?;
            archive.write_all(relationships.as_bytes())?;
        }
    }
    for (name, asset) in media {
        archive.start_file(format!("word/{name}"), options)?;
        archive.write_all(&asset.original)?;
    }
    Ok(archive.finish()?.into_inner())
}

pub(super) const CONTENT_TYPES: &str = r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<Types xmlns="http://schemas.openxmlformats.org/package/2006/content-types">
  <Default Extension="rels" ContentType="application/vnd.openxmlformats-package.relationships+xml"/>
  <Default Extension="xml" ContentType="application/xml"/>
  <Override PartName="/word/document.xml" ContentType="application/vnd.openxmlformats-officedocument.wordprocessingml.document.main+xml"/>
  <Override PartName="/word/styles.xml" ContentType="application/vnd.openxmlformats-officedocument.wordprocessingml.styles+xml"/>
  <Override PartName="/word/settings.xml" ContentType="application/vnd.openxmlformats-officedocument.wordprocessingml.settings+xml"/>
  <Override PartName="/word/fontTable.xml" ContentType="application/vnd.openxmlformats-officedocument.wordprocessingml.fontTable+xml"/>
  <Override PartName="/word/numbering.xml" ContentType="application/vnd.openxmlformats-officedocument.wordprocessingml.numbering+xml"/>
  <Override PartName="/docProps/core.xml" ContentType="application/vnd.openxmlformats-package.core-properties+xml"/>
  <Override PartName="/docProps/app.xml" ContentType="application/vnd.openxmlformats-officedocument.extended-properties+xml"/>
</Types>"#;
pub(super) const ROOT_RELATIONSHIPS: &str = r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<Relationships xmlns="http://schemas.openxmlformats.org/package/2006/relationships">
  <Relationship Id="rId1" Type="http://schemas.openxmlformats.org/officeDocument/2006/relationships/officeDocument" Target="word/document.xml"/>
  <Relationship Id="rId2" Type="http://schemas.openxmlformats.org/package/2006/relationships/metadata/core-properties" Target="docProps/core.xml"/>
  <Relationship Id="rId3" Type="http://schemas.openxmlformats.org/officeDocument/2006/relationships/extended-properties" Target="docProps/app.xml"/>
</Relationships>"#;
pub(super) const DOCUMENT_RELATIONSHIPS: &str = r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<Relationships xmlns="http://schemas.openxmlformats.org/package/2006/relationships">
  <Relationship Id="rId1" Type="http://schemas.openxmlformats.org/officeDocument/2006/relationships/styles" Target="styles.xml"/>
  <Relationship Id="rId2" Type="http://schemas.openxmlformats.org/officeDocument/2006/relationships/settings" Target="settings.xml"/>
  <Relationship Id="rId3" Type="http://schemas.openxmlformats.org/officeDocument/2006/relationships/fontTable" Target="fontTable.xml"/>
  <Relationship Id="rId4" Type="http://schemas.openxmlformats.org/officeDocument/2006/relationships/numbering" Target="numbering.xml"/>
</Relationships>"#;
pub(super) const SETTINGS: &str = r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<w:settings xmlns:w="http://schemas.openxmlformats.org/wordprocessingml/2006/main">
  <w:zoom w:percent="100"/>
  <w:defaultTabStop w:val="720"/>
  <w:compat>
    <w:doNotExpandShiftReturn/>
    <w:compatSetting w:name="compatibilityMode" w:uri="http://schemas.microsoft.com/office/word" w:val="15"/>
  </w:compat>
</w:settings>"#;
pub(super) const APP_PROPERTIES: &str = r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<Properties xmlns="http://schemas.openxmlformats.org/officeDocument/2006/extended-properties" xmlns:vt="http://schemas.openxmlformats.org/officeDocument/2006/docPropsVTypes">
  <Application>Mote</Application>
  <DocSecurity>0</DocSecurity>
  <ScaleCrop>false</ScaleCrop>
  <LinksUpToDate>false</LinksUpToDate>
  <SharedDoc>false</SharedDoc>
  <HyperlinksChanged>false</HyperlinksChanged>
  <AppVersion>16.0000</AppVersion>
</Properties>"#;
