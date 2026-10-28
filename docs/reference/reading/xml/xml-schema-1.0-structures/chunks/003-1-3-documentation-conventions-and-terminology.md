# 1.3 Documentation Conventions and Terminology

Source: [XML Schema Part 1: Structures](https://www.w3.org/TR/2004/REC-xmlschema-1-20041028/) · 2nd edition, 2004-10-28.

[Document index](../README.md) · [Topic guide](../../../../README.md) · [Publisher](https://www.w3.org/TR/2004/REC-xmlschema-1-20041028/).

HTML normalized to Markdown. Complex tables and mathematical markup retain HTML; illustrations are stored locally.

[Previous](002-appendices.md) · [Next](004-2-1-overview-of-xml-schema.md)

<a id="intro-terminology"></a>

### 1.3 Documentation Conventions and Terminology

The section introduces the highlighting and typography as used in
this document to present technical material.

<a id="key-sampledef"></a>

Special terms are defined at their point of
introduction in the text. For example [Definition:]a **term** is
something used with a special meaning. The definition is
labeled as such and the term it defines is displayed in boldface. The end of the definition is not specially marked
in the displayed or printed text. Uses of defined terms are links to
their definitions, set off with middle dots, for instance [·term·](003-1-3-documentation-conventions-and-terminology.md#key-sampledef).

Non-normative examples are set off in boxes and accompanied by a brief
explanation:

Example



```
<schema targetNamespace="http://www.example.com/XMLSchema/1.0/mySchema">
```




And an explanation of the example.

The definition of each kind of schema component consists of a list of
its properties and their contents, followed by descriptions of the
semantics of the properties:

Schema Component: [Example](003-1-3-documentation-conventions-and-terminology.md#intro-terminology)

<a id="xmpl-prop"></a>

{example property}
:   Definition of the property.

References to properties of schema components are links to
the relevant definition as exemplified above, set off with curly braces, for instance [{example property}](003-1-3-documentation-conventions-and-terminology.md#xmpl-prop).

The correspondence between an element information item which is part
of the XML representation of a schema and one or more schema components is presented in a tableau
which illustrates the element information item(s) involved.
This is followed by a tabulation of the correspondence between properties of the component
and properties of the information item. Where context may determine which of
several different components may arise, several tabulations, one per context,
are given. The property correspondences are normative,
as are the illustrations of the XML representation element information items.

In the XML representation, bold-face
attribute names (e.g. **count** below) indicate a required
attribute information item, and the rest are
optional. Where an attribute information item has an enumerated type
definition, the values are shown separated by vertical bars, as for
`size` below; if there is a default value, it is shown
following a colon. Where an attribute information item has a built-in simple
type definition defined in [[XML Schemas: Datatypes]](060-b-references-normative.md#ref-xsp2), a hyperlink to its
definition therein is given.

The allowed content of the information item is
shown as a grammar fragment, using the Kleene operators `?`,
`*` and `+`. Each element name therein is a hyperlink to
its own illustration.

**Note:**The illustrations are derived automatically from the [Schema for Schemas (normative) (§A)](058-5-3-missing-sub-components.md#normative-schemaSchema). In the case of apparent conflict, the [Schema for Schemas (normative) (§A)](058-5-3-missing-sub-components.md#normative-schemaSchema) takes precedence, as it, together with the [·Schema Representation Constraints·](006-2-3-constraints-and-validation-rules.md#gloss-src), provide the normative statement of
the form of XML representations.

XML Representation Summary: `example` Element Information Item

<a id="element-example"></a>

<example  
  **count** = [integer](https://www.w3.org/TR/2004/REC-xmlschema-2-20041028/datatypes.html#integer)  
  size = (large | medium | small) : medium>  
*Content:* ([all](033-3-7-6-constraints-on-model-group-definition-schema-components.md#element-all) | [any](038-3-10-wildcards.md#element-any)\*)  
</example>

<table class="reprcomp"><thead><tr><th><a href="003-1-3-documentation-conventions-and-terminology.md#intro-terminology">Example</a><strong> Schema Component</strong></th></tr></thead><tbody><tr><td><table border="0" cellpadding="3"><thead><tr><th align="left">Property</th><th align="left">Representation</th></tr></thead><tbody valign="top"><tr valign="top"><td><a class="propref" href="003-1-3-documentation-conventions-and-terminology.md#xmpl-prop">{example property}</a></td><td>Description of what the property corresponds to, e.g. the value of the <code>size</code>
<a class="xpropref" href="https://www.w3.org/TR/xml-infoset/#infoitem.element">[attribute]</a>
</td></tr></tbody></table></td></tr></tbody></table>

References to elements in the text are links to
the relevant illustration as exemplified above, set off with angle brackets, for instance [<example>](003-1-3-documentation-conventions-and-terminology.md#element-example).

References to properties of information items as defined in [[XML-Infoset]](060-b-references-normative.md#ref-xmlinfo) are notated as links to the relevant section thereof, set off with square brackets, for example [[children]](https://www.w3.org/TR/xml-infoset/#infoitem.element).

Properties which this specification defines for information items are
introduced as follows:

PSVI Contributions
for example information items

<a id="ex-foo"></a>

[new property]
:   The value the property gets.

References to properties of information items defined in this specification
are notated as links to their introduction as exemplified above, set off with square brackets, for example [[new property]](003-1-3-documentation-conventions-and-terminology.md#ex-foo).

The following highlighting is used for non-normative commentary in
this document:

**Note:**General comments directed to all readers.

Following [[XML 1.0 (Second Edition)]](060-b-references-normative.md#ref-xml), within normative prose in this specification, the words
*may* and *must* are defined as follows:

may
:   Conforming documents and XML Schema-aware processors are permitted to but need not behave as described.

must
:   Conforming documents and XML Schema-aware processors are required to behave as described; otherwise they are in error.

Note however that this specification provides a definition of error and of conformant processors'
responsibilities with respect to errors (see [Schemas and Schema-validity Assessment (§5)](057-5-schemas-and-schema-validity-assessment.md#conformance)) which is considerably
more complex than that of [[XML 1.0 (Second Edition)]](060-b-references-normative.md#ref-xml).

<a id="concepts"></a>

## 2 Conceptual Framework

This chapter gives an overview of *XML Schema: Structures* at the level of its abstract data model. [Schema Component Details (§3)](008-3-schema-component-details.md#components) provides details on this model, including
a normative representation in XML for the components of the model.
Readers interested primarily in learning to write schema documents may wish to
first read [[XML Schema: Primer]](067-h-analysis-of-the-unique-particle-attribution-constraint-non-normative.md#bib-expo) for a tutorial introduction, and only then consult the sub-sections of
[Schema Component Details (§3)](008-3-schema-component-details.md#components) named *XML Representation of ...* for
the details.

