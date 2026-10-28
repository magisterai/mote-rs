# 2.5 Names and Symbol Spaces

Source: [XML Schema Part 1: Structures](https://www.w3.org/TR/2004/REC-xmlschema-1-20041028/) · 2nd edition, 2004-10-28.

[Document index](../README.md) · [Topic guide](../../../../README.md) · [Publisher](https://www.w3.org/TR/2004/REC-xmlschema-1-20041028/).

HTML normalized to Markdown. Complex tables and mathematical markup retain HTML; illustrations are stored locally.

[Previous](006-2-3-constraints-and-validation-rules.md) · [Next](008-3-schema-component-details.md)

<a id="concepts-nameSymbolSpaces"></a>

### 2.5 Names and Symbol Spaces

As discussed in [XML Schema Abstract Data Model (§2.2)](004-2-1-overview-of-xml-schema.md#concepts-data-model), most schema
components (may) have [·names·](004-2-1-overview-of-xml-schema.md#key-compName).
If all such names were assigned from the same "pool", then
it would be impossible to have, for example, a simple type definition and an element
declaration both with the name
"title" in a given [·target namespace·](004-2-1-overview-of-xml-schema.md#key-targetNS).

<a id="key-symbolSpace"></a>

Therefore [Definition:]this specification introduces the term
**symbol space** to denote a
collection of names, each of which is unique with respect to the others. A symbol space is similar to the non-normative concept of [namespace partition](https://www.w3.org/TR/REC-xml-names/#ns-breakdown) introduced in [[XML-Namespaces]](060-b-references-normative.md#ref-xml-namespaces).
There is a single distinct symbol space within a given [·target
namespace·](004-2-1-overview-of-xml-schema.md#key-targetNS) for each kind of definition and declaration component
identified in [XML Schema Abstract Data Model (§2.2)](004-2-1-overview-of-xml-schema.md#concepts-data-model), except that within a target namespace, simple
type definitions and complex type definitions share a symbol space.
Within a given symbol space, names are unique, but the same name may appear in more than one symbol space without conflict. For example, the same name can appear in both a type definition and an element declaration, without conflict or necessary relation between the two.

Locally scoped attribute and element
declarations are special with regard to symbol spaces.
Every complex type definition defines its own local attribute and element declaration symbol
spaces, where these symbol spaces are distinct from each other and from any of the other
symbol spaces. So, for example, two complex type definitions having
the same target namespace can contain
a local attribute declaration for the unqualified name "priority", or contain a local element declaration
for the name "address", without conflict or necessary relation between
the two.

<a id="Instance_Document_Constructions"></a>

### 2.6 Schema-Related Markup in Documents Being Validated

2.6.1 [xsi:type](007-2-5-names-and-symbol-spaces.md#xsi_type)  
        2.6.2 [xsi:nil](007-2-5-names-and-symbol-spaces.md#xsi_nil)  
        2.6.3 [xsi:schemaLocation, xsi:noNamespaceSchemaLocation](007-2-5-names-and-symbol-spaces.md#xsi_schemaLocation)

The XML representation of schema components uses a vocabulary
identified by the namespace name `http://www.w3.org/2001/XMLSchema`. For brevity, the text and examples in this specification use the prefix
`xs:` to stand for this namespace; in practice,
any prefix can be used.

*XML Schema: Structures* also defines several attributes for direct use in any XML documents. These attributes are in a different namespace,
which has the namespace name `http://www.w3.org/2001/XMLSchema-instance`.
For brevity, the text and examples in this specification use the prefix
`xsi:` to stand for this latter namespace; in practice,
any prefix can be used. All schema processors have appropriate attribute
declarations for these attributes built in, see [Attribute Declaration for the 'type' attribute (§3.2.7)](012-3-2-6-constraints-on-attribute-declaration-schema-components.md#xsi.type),
[Attribute Declaration for the 'nil' attribute (§3.2.7)](012-3-2-6-constraints-on-attribute-declaration-schema-components.md#xsi.nil), [Attribute Declaration for the 'schemaLocation' attribute (§3.2.7)](012-3-2-6-constraints-on-attribute-declaration-schema-components.md#xsi.schemaLocation) and [Attribute Declaration for the 'noNamespaceSchemaLocation' attribute (§3.2.7)](012-3-2-6-constraints-on-attribute-declaration-schema-components.md#xsi.noNamespaceSchemaLocation).

<a id="xsi_type"></a>

#### 2.6.1 xsi:type

The [Simple Type Definition (§2.2.1.2)](004-2-1-overview-of-xml-schema.md#Simple_Type_Definition) or [Complex Type Definition (§2.2.1.3)](005-2-2-1-3-complex-type-definition.md#Complex_Type_Definition) used in [·validation·](004-2-1-overview-of-xml-schema.md#key-vn) of an element is usually
determined by reference to the appropriate schema components.
An element information item in an instance may, however,
explicitly assert its type using the attribute `xsi:type`.
The value of this attribute is a [·QName·](050-3-15-2-1-references-to-schema-components.md#gloss-QName); see [QName Interpretation (§3.15.3)](050-3-15-2-1-references-to-schema-components.md#src-qname) for
the means by which the [·QName·](050-3-15-2-1-references-to-schema-components.md#gloss-QName) is
associated with a type definition.

<a id="xsi_nil"></a>

#### 2.6.2 xsi:nil

*XML Schema: Structures* introduces a mechanism for signaling that an element should
be accepted as [·valid·](004-2-1-overview-of-xml-schema.md#key-vn) when it has no
content despite a content type which does not require or even necessarily allow empty content. An
element may be
[·valid·](004-2-1-overview-of-xml-schema.md#key-vn) without content if it has the attribute `xsi:nil` with
the value `true`. An element so labeled must be empty, but can
carry attributes if permitted by the corresponding complex type.

<a id="xsi_schemaLocation"></a>

#### 2.6.3 xsi:schemaLocation, xsi:noNamespaceSchemaLocation

The `xsi:schemaLocation` and `xsi:noNamespaceSchemaLocation` attributes can be used in a document to provide
hints as to the physical location of schema documents which may be used for [·assessment·](004-2-1-overview-of-xml-schema.md#key-va).
See [How schema definitions are located on the Web (§4.3.2)](056-4-3-layer-3-schema-document-access-and-web-interoperability.md#schema-loc) for details on the use of these attributes.

<a id="web-representation"></a>

### 2.7 Representation of Schemas on the World Wide Web

On the World Wide Web, schemas are conventionally represented as XML
documents (preferably of MIME type
`application/xml` or `text/xml`, but see clause [1.1](052-4-schemas-and-namespaces-access-and-composition.md#c-vxd) of [Inclusion Constraints and Semantics (§4.2.1)](052-4-schemas-and-namespaces-access-and-composition.md#src-include)), conforming to the specifications in [Layer 2: Schema Documents, Namespaces and Composition (§4.2)](052-4-schemas-and-namespaces-access-and-composition.md#layer2). For more information on
the representation and use of schema documents on the World Wide Web see [Standards for representation of schemas and retrieval of schema documents on the Web (§4.3.1)](056-4-3-layer-3-schema-document-access-and-web-interoperability.md#schema-repr) and
[How schema definitions are located on the Web (§4.3.2)](056-4-3-layer-3-schema-document-access-and-web-interoperability.md#schema-loc).

