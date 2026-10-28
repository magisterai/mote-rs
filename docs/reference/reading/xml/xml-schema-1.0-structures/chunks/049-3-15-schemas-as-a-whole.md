# 3.15 Schemas as a Whole

Source: [XML Schema Part 1: Structures](https://www.w3.org/TR/2004/REC-xmlschema-1-20041028/) · 2nd edition, 2004-10-28.

[Document index](../README.md) · [Topic guide](../../../../README.md) · [Publisher](https://www.w3.org/TR/2004/REC-xmlschema-1-20041028/).

HTML normalized to Markdown. Complex tables and mathematical markup retain HTML; illustrations are stored locally.

[Previous](048-3-14-7-built-in-simple-type-definition.md) · [Next](050-3-15-2-1-references-to-schema-components.md)

<a id="Schemas"></a>

### 3.15 Schemas as a Whole

3.15.1 [The Schema Itself](049-3-15-schemas-as-a-whole.md#Schema_details)  
        3.15.2 [XML Representations of Schemas](049-3-15-schemas-as-a-whole.md#declare-schema)  
        3.15.3 [Constraints on XML Representations of Schemas](050-3-15-2-1-references-to-schema-components.md#d0e16860)  
        3.15.4 [Validation Rules for Schemas as a Whole](050-3-15-2-1-references-to-schema-components.md#d0e17196)  
        3.15.5 [Schema Information Set Contributions](051-3-15-5-schema-information-set-contributions.md#d0e17275)  
        3.15.6 [Constraints on Schemas as a Whole](051-3-15-5-schema-information-set-contributions.md#coss-schema)

A schema consists of a set of schema components.

Example



```
<xs:schema
    xmlns:xs="http://www.w3.org/2001/XMLSchema"
    targetNamespace="http://www.example.com/example">
  . . .
</xs:schema>
```




The XML representation of the skeleton of a schema.

<a id="Schema_details"></a>

#### 3.15.1 The Schema Itself

At the abstract level, the schema itself is just a container for its components.

Schema Component: [Schema](004-2-1-overview-of-xml-schema.md#key-schema)

<a id="type_definitions"></a>

{type definitions}
:   A set of
    named simple and complex type definitions.

<a id="attribute_declarations"></a>

{attribute declarations}
:   A set of
    named (top-level) attribute declarations.

<a id="element_declarations"></a>

{element declarations}
:   A set of
    named (top-level) element declarations.

<a id="attribute_group_definitions"></a>

{attribute group definitions}
:   A set of named
    attribute group definitions.

<a id="model_group_definitions"></a>

{model group definitions}
:   A set of named
    model group definitions.

<a id="notation_declarations"></a>

{notation declarations}
:   A set of
    notation declarations.

<a id="annotations"></a>

{annotations}
:   A set of annotations.

<a id="declare-schema"></a>

#### 3.15.2 XML Representations of Schemas

A schema is represented in XML by one or more [·schema documents·](008-3-schema-component-details.md#key-schemaDoc), that is, one or more [<schema>](049-3-15-schemas-as-a-whole.md#element-schema) element information items. A [·schema document·](008-3-schema-component-details.md#key-schemaDoc) contains representations for a collection of schema components, e.g. type definitions and element declarations, which have a common {target namespace}. A [·schema document·](008-3-schema-component-details.md#key-schemaDoc) which has one or more [<import>](054-4-2-3-references-to-schema-components-across-namespaces.md#element-import) element information items corresponds to a schema with components with more than one {target namespace}, see [Import Constraints and Semantics (§4.2.3)](055-continuation-55.md#src-import).

XML Representation Summary: `schema` Element Information Item

<a id="element-schema"></a>

<schema  
  attributeFormDefault = (qualified | unqualified) : unqualified  
  blockDefault =
(#all | List of (extension | restriction | substitution))
 : ''  
  elementFormDefault = (qualified | unqualified) : unqualified  
  finalDefault =
(#all | List of (extension | restriction | list | union))
 : ''  
  id = [ID](https://www.w3.org/TR/2004/REC-xmlschema-2-20041028/datatypes.html#ID)  
  targetNamespace = [anyURI](https://www.w3.org/TR/2004/REC-xmlschema-2-20041028/datatypes.html#anyURI)  
  version = [token](https://www.w3.org/TR/2004/REC-xmlschema-2-20041028/datatypes.html#token)  
  xml:lang = [language](https://www.w3.org/TR/2004/REC-xmlschema-2-20041028/datatypes.html#language)  
  *{any attributes with non-schema namespace . . .}*>  
*Content:* (([include](052-4-schemas-and-namespaces-access-and-composition.md#element-include) | [import](054-4-2-3-references-to-schema-components-across-namespaces.md#element-import) | [redefine](053-4-2-2-including-modified-component-definitions.md#element-redefine) | [annotation](044-3-13-annotations.md#element-annotation))\*, ((([simpleType](045-3-14-simple-type-definitions.md#element-simpleType) | [complexType](021-3-4-2-xml-representation-of-complex-type-definitions.md#element-complexType) | [group](032-3-7-model-group-definitions.md#element-group) | [attributeGroup](030-3-5-attributeuses.md#element-attributeGroup)) | [element](013-3-3-element-declarations.md#element-element) | [attribute](009-3-2-attribute-declarations.md#element-attribute) | [notation](043-3-12-notation-declarations.md#element-notation)), [annotation](044-3-13-annotations.md#element-annotation)\*)\*)  
</schema>

<table class="reprcomp"><thead><tr><th><a href="004-2-1-overview-of-xml-schema.md#key-schema">Schema</a><strong> Schema Component</strong></th></tr></thead><tbody><tr><td><table border="0" cellpadding="3"><thead><tr><th align="left">Property</th><th align="left">Representation</th></tr></thead><tbody valign="top"><tr valign="top"><td><a class="propref" href="049-3-15-schemas-as-a-whole.md#type_definitions">{type definitions}</a></td><td>The simple and complex type definitions
corresponding to all the <a class="eltref" href="045-3-14-simple-type-definitions.md#element-simpleType">&lt;simpleType&gt;</a> and <a class="eltref" href="021-3-4-2-xml-representation-of-complex-type-definitions.md#element-complexType">&lt;complexType&gt;</a> element information items in the
<a class="xpropref" href="https://www.w3.org/TR/xml-infoset/#infoitem.element">[children]</a>, if any, plus any included or imported definitions, see <a href="052-4-schemas-and-namespaces-access-and-composition.md#compound-schema">Assembling a schema for a single target namespace from multiple schema definition documents (§4.2.1)</a> and <a href="054-4-2-3-references-to-schema-components-across-namespaces.md#composition-schemaImport">References to schema components across namespaces (§4.2.3)</a>.</td></tr><tr valign="top"><td><a class="propref" href="049-3-15-schemas-as-a-whole.md#attribute_declarations">{attribute declarations}</a></td><td>The (top-level) attribute declarations
corresponding to all the <a class="eltref" href="009-3-2-attribute-declarations.md#element-attribute">&lt;attribute&gt;</a> element information items in the
<a class="xpropref" href="https://www.w3.org/TR/xml-infoset/#infoitem.element">[children]</a>, if any, plus any included or imported declarations, see <a href="052-4-schemas-and-namespaces-access-and-composition.md#compound-schema">Assembling a schema for a single target namespace from multiple schema definition documents (§4.2.1)</a> and <a href="054-4-2-3-references-to-schema-components-across-namespaces.md#composition-schemaImport">References to schema components across namespaces (§4.2.3)</a>.</td></tr><tr valign="top"><td><a class="propref" href="049-3-15-schemas-as-a-whole.md#element_declarations">{element declarations}</a></td><td>The (top-level) element declarations
corresponding to all the <a class="eltref" href="013-3-3-element-declarations.md#element-element">&lt;element&gt;</a> element information items in the
<a class="xpropref" href="https://www.w3.org/TR/xml-infoset/#infoitem.element">[children]</a>, if any, plus any included or imported declarations, see <a href="052-4-schemas-and-namespaces-access-and-composition.md#compound-schema">Assembling a schema for a single target namespace from multiple schema definition documents (§4.2.1)</a> and <a href="054-4-2-3-references-to-schema-components-across-namespaces.md#composition-schemaImport">References to schema components across namespaces (§4.2.3)</a>.</td></tr><tr valign="top"><td><a class="propref" href="049-3-15-schemas-as-a-whole.md#attribute_group_definitions">{attribute group definitions}</a></td><td>The attribute group definitions
corresponding to all the <a class="eltref" href="030-3-5-attributeuses.md#element-attributeGroup">&lt;attributeGroup&gt;</a> element information items in the
<a class="xpropref" href="https://www.w3.org/TR/xml-infoset/#infoitem.element">[children]</a>, if any, plus any included or imported definitions, see <a href="052-4-schemas-and-namespaces-access-and-composition.md#compound-schema">Assembling a schema for a single target namespace from multiple schema definition documents (§4.2.1)</a> and <a href="054-4-2-3-references-to-schema-components-across-namespaces.md#composition-schemaImport">References to schema components across namespaces (§4.2.3)</a>.</td></tr><tr valign="top"><td><a class="propref" href="049-3-15-schemas-as-a-whole.md#model_group_definitions">{model group definitions}</a></td><td>The model group definitions
corresponding to all the <a class="eltref" href="032-3-7-model-group-definitions.md#element-group">&lt;group&gt;</a> element information items in the
<a class="xpropref" href="https://www.w3.org/TR/xml-infoset/#infoitem.element">[children]</a>, if any, plus any included or imported definitions, see <a href="052-4-schemas-and-namespaces-access-and-composition.md#compound-schema">Assembling a schema for a single target namespace from multiple schema definition documents (§4.2.1)</a> and <a href="054-4-2-3-references-to-schema-components-across-namespaces.md#composition-schemaImport">References to schema components across namespaces (§4.2.3)</a>.</td></tr><tr valign="top"><td><a class="propref" href="049-3-15-schemas-as-a-whole.md#notation_declarations">{notation declarations}</a></td><td>The notation declarations
corresponding to all the <a class="eltref" href="043-3-12-notation-declarations.md#element-notation">&lt;notation&gt;</a> element information items in the
<a class="xpropref" href="https://www.w3.org/TR/xml-infoset/#infoitem.element">[children]</a>, if any, plus any included or imported declarations, see <a href="052-4-schemas-and-namespaces-access-and-composition.md#compound-schema">Assembling a schema for a single target namespace from multiple schema definition documents (§4.2.1)</a> and <a href="054-4-2-3-references-to-schema-components-across-namespaces.md#composition-schemaImport">References to schema components across namespaces (§4.2.3)</a>.</td></tr><tr valign="top"><td><a class="propref" href="049-3-15-schemas-as-a-whole.md#annotations">{annotations}</a></td><td>The annotations
corresponding to all the <a class="eltref" href="044-3-13-annotations.md#element-annotation">&lt;annotation&gt;</a> element information items in the
<a class="xpropref" href="https://www.w3.org/TR/xml-infoset/#infoitem.element">[children]</a>, if any.</td></tr></tbody></table></td></tr></tbody></table>

Note that none of the attribute information items displayed above
correspond directly to properties of schemas. The `blockDefault`,
`finalDefault`, `attributeFormDefault`, `elementFormDefault`and `targetNamespace` attributes are appealed to in the sub-sections above, as they provide
global information applicable to many representation/component correspondences. The
other attributes (`id` and `version`) are for user
convenience, and this specification defines no semantics for them.

The definition of the schema abstract data model in [XML Schema Abstract Data Model (§2.2)](004-2-1-overview-of-xml-schema.md#concepts-data-model) makes clear that most components have a {target namespace}. Most components corresponding to representations within a given [<schema>](049-3-15-schemas-as-a-whole.md#element-schema) element information item will have a {target namespace} which corresponds to the `targetNamespace` attribute.

Since the empty string is not a legal namespace name, supplying
an empty string for `targetNamespace` is incoherent, and is *not* the same
as not specifying it at all. The appropriate form of schema document
corresponding to a [·schema·](004-2-1-overview-of-xml-schema.md#key-schema) whose components have no
[{target namespace}](013-3-3-element-declarations.md#e-target_namespace) is one which has no
`targetNamespace` attribute specified at all.

**Note:**The XML namespaces Recommendation discusses only instance document syntax for
elements and attributes; it therefore provides no direct framework for managing
the names of type definitions, attribute group definitions, and so on.
Nevertheless, the specification applies the target namespace facility uniformly to all
schema components, i.e. not only declarations but also definitions have a {target namespace}.

Although the example schema at the beginning of this section might be a complete XML document, [<schema>](049-3-15-schemas-as-a-whole.md#element-schema)
need not be the document element, but can appear within other documents.
Indeed there is no requirement that a schema correspond to a (text) document
at all: it could correspond to an element information item constructed 'by
hand', for instance via a DOM-conformant API.

