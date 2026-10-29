use super::{Error, check_attributes, fonts, styles::Styles, values, xml};
use crate::model::{Properties, Value};

pub(super) struct Controls {
    next_identifier: u32,
}

impl Default for Controls {
    fn default() -> Self {
        Self {
            next_identifier: 1000,
        }
    }
}

impl Controls {
    pub fn write_checkbox(
        &mut self,
        properties: &Properties,
        checked: bool,
        styles: &Styles<'_>,
    ) -> Result<String, Error> {
        check_attributes(
            properties,
            &["checked", "kind", "name", "title"],
            "checkbox",
        )?;
        let kind = properties
            .get("kind")
            .map(Value::to_text)
            .unwrap_or_default();
        if !matches!(kind.as_str(), "" | "content_control" | "legacy_form_field") {
            return Err(Error::Unsupported(format!("checkbox kind {kind:?}")));
        }
        if kind == "legacy_form_field" && properties.contains_key("title") {
            return Err(Error::Unsupported("title on a legacy checkbox".into()));
        }
        let identifier = self.next_identifier;
        self.next_identifier = identifier
            .checked_add(1)
            .filter(|value| *value <= i32::MAX as u32)
            .ok_or_else(|| Error::InvalidValue("too many Word controls".into()))?;
        let checked_value = if checked { 1 } else { 0 };
        let name = properties
            .get("name")
            .map(Value::to_text)
            .unwrap_or_default();
        if kind == "legacy_form_field" {
            let name = if name.is_empty() {
                format!("Check{identifier}")
            } else {
                name
            };
            return Ok(format!(
                "<w:r><w:fldChar w:fldCharType=\"begin\"><w:ffData><w:name w:val=\"{}\"/><w:enabled/><w:calcOnExit w:val=\"0\"/><w:checkBox><w:sizeAuto/><w:default w:val=\"{checked_value}\"/><w:checked w:val=\"{checked_value}\"/></w:checkBox></w:ffData></w:fldChar></w:r><w:r><w:instrText xml:space=\"preserve\"> FORMCHECKBOX </w:instrText></w:r><w:r><w:fldChar w:fldCharType=\"end\"/></w:r>",
                xml::escape(&name)?
            ));
        }
        let mut content = format!("<w:sdt><w:sdtPr><w:id w:val=\"{identifier}\"/>");
        let title = properties
            .get("title")
            .map(Value::to_text)
            .unwrap_or_default();
        if !title.is_empty() {
            content.push_str(&format!("<w:alias w:val=\"{}\"/>", xml::escape(&title)?));
        }
        if !name.is_empty() {
            content.push_str(&format!("<w:tag w:val=\"{}\"/>", xml::escape(&name)?));
        }
        let symbol = if checked { '☒' } else { '☐' };
        content.push_str(&format!("<w14:checkbox><w14:checked w14:val=\"{checked_value}\"/><w14:checkedState w14:val=\"2612\" w14:font=\"MS Gothic\"/><w14:uncheckedState w14:val=\"2610\" w14:font=\"MS Gothic\"/></w14:checkbox></w:sdtPr><w:sdtContent><w:r><w:rPr>{}</w:rPr><w:t xml:space=\"preserve\">{symbol}</w:t></w:r></w:sdtContent></w:sdt>", fonts::run_fonts(styles.document, "MS Gothic")?));
        Ok(content)
    }
}

pub(super) fn checkbox_properties(attributes: &Properties) -> Result<Properties, Error> {
    attributes
        .get("checkbox")
        .map(values::object)
        .transpose()
        .map(|properties| properties.cloned().unwrap_or_default())
}
