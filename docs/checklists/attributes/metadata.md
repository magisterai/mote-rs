# Core document properties

[Inventory guide](README.md) · [Metadata directive](../metadata.md)

Source: [OPC core-properties schema](../../reference/ooxml/ecma-376/part-2/xsd/opc-coreProperties.xsd).
All fifteen optional singleton properties of `CT_CoreProperties` are listed
below. XML names are standard fields, not proposed Mote attributes.

- [x] `dc:title`: metadata `title` in DOCX and PPTX.
- [ ] `dc:creator`: metadata `author` in DOCX; PPTX retains its generated creator.
- [ ] `dc:subject`: retained `subject`, without a package mapping.
- [ ] `dc:description`: retained `description`, without a package mapping.
- [ ] `dc:identifier`: no package mapping.
- [ ] `dc:language`: no package mapping.
- [ ] `category`: retained `category`, without a package mapping.
- [ ] `contentStatus`: string, without a package mapping.
- [ ] `keywords`: mixed text and repeated `value` strings; both the container and
  each value permit `xml:lang`. Retained `keywords` does not map to this structure.
- [ ] `lastModifiedBy`: string, without an authoring mapping.
- [ ] `lastPrinted`: date-time, without an authoring mapping.
- [ ] `revision`: string, without an authoring mapping.
- [ ] `version`: string, without an authoring mapping.
- [ ] `dcterms:created`: generated timestamp; no authored override.
- [ ] `dcterms:modified`: generated timestamp; no authored override.

Dublin Core text properties use their imported simple-literal representation,
including language tags; created/modified use the OPC date-time restrictions.
No language-tag authoring is exposed. Generated values are distinct from support
for an authored metadata attribute.

The complete [extended and custom property families](metadata-properties.md)
include application statistics, template, manager/company, document security,
links, signatures, custom property identities and every nested variant value.
None gains a package mapping merely because an identically named value can be
retained in `metadata`. Arbitrary future extension payloads are not enumerable.

Evidence: [language definition](../../specification/annotations.md),
[metadata and annotation tests](../../../tests/annotations.rs), and
[complete package captures](../../../tests/fixtures/annotations/README.md).
