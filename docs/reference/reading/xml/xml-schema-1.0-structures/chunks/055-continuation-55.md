# Continuation 55

Source: [XML Schema Part 1: Structures](https://www.w3.org/TR/2004/REC-xmlschema-1-20041028/) · 2nd edition, 2004-10-28.

[Document index](../README.md) · [Topic guide](../../../../README.md) · [Publisher](https://www.w3.org/TR/2004/REC-xmlschema-1-20041028/).

HTML normalized to Markdown. Complex tables and mathematical markup retain HTML; illustrations are stored locally.

[Previous](054-4-2-3-references-to-schema-components-across-namespaces.md) · [Next](056-4-3-layer-3-schema-document-access-and-web-interoperability.md)

The
[·actual value·](008-3-schema-component-details.md#key-vv) of the `schemaLocation`, if present, gives a
hint as to where a serialization of a [·schema document·](008-3-schema-component-details.md#key-schemaDoc) with declarations and definitions for that
namespace (or none) may be found. When no
`schemaLocation` [[attribute]](https://www.w3.org/TR/xml-infoset/#infoitem.element) is present, the schema author is leaving the
identification of that schema to the instance, application or user, via the mechanisms described
below in [Layer 3: Schema Document Access and Web-interoperability (§4.3)](056-4-3-layer-3-schema-document-access-and-web-interoperability.md#composition-instances). When a `schemaLocation` is present, it
must contain a single URI reference which the schema author
warrants will resolve to a serialization of a [·schema document·](008-3-schema-component-details.md#key-schemaDoc) containing the component(s) in the
[<import>](054-4-2-3-references-to-schema-components-across-namespaces.md#element-import)ed namespace referred to elsewhere in the containing
schema document.

**Note:**Since both the `namespace` and `schemaLocation`
[[attribute]](https://www.w3.org/TR/xml-infoset/#infoitem.element) are optional, a bare `<import/>` information item
is allowed. This simply allows unqualified reference to foreign
components with no target namespace without giving any hints as to where to find them.

Example

The same namespace may be used both for real work, and in the course of
defining schema components in terms of foreign components:



```
<schema xmlns="http://www.w3.org/2001/XMLSchema"
        xmlns:html="http://www.w3.org/1999/xhtml"
        targetNamespace="uri:mywork" xmlns:my="uri:mywork">

 <import namespace="http://www.w3.org/1999/xhtml"/>

 <annotation>
  <documentation>
   <html:p>[Some documentation for my schema]</html:p>
  </documentation>
 </annotation>

 . . .

 <complexType name="myType">
  <sequence>
   <element ref="html:p" minOccurs="0"/>
  </sequence>
  . . .
 </complexType>

 <element name="myElt" type="my:myType"/>
</schema>
```




The treatment of references as [·QNames·](050-3-15-2-1-references-to-schema-components.md#gloss-QName) implies that since (with the exception of
the schema for schemas) the target namespace and the XML Schema namespace
differ, without massive redeclaration of the default namespace
*either* internal references to the names being defined in a schema document
*or* the schema declaration and definition elements themselves must
be explicitly qualified. This example takes the first option -- most other
examples in this specification have taken the second.

<a id="src-import"></a>

**Schema Representation Constraint: Import Constraints and Semantics**  

In addition to the conditions imposed on [<import>](054-4-2-3-references-to-schema-components-across-namespaces.md#element-import) element
information items by the schema for schemas
**all** of the following must be true:

1 The appropriate **case** among the following
must be true:

1.1 **If** the `namespace` [[attribute]](https://www.w3.org/TR/xml-infoset/#infoitem.element) is present, **then** its [·actual value·](008-3-schema-component-details.md#key-vv) must not match the [·actual value·](008-3-schema-component-details.md#key-vv) of the
enclosing [<schema>](049-3-15-schemas-as-a-whole.md#element-schema)'s `targetNamespace` [[attribute]](https://www.w3.org/TR/xml-infoset/#infoitem.element).

1.2 **If** the `namespace` [[attribute]](https://www.w3.org/TR/xml-infoset/#infoitem.element) is not present, **then** the enclosing [<schema>](049-3-15-schemas-as-a-whole.md#element-schema) must have a `targetNamespace` [[attribute]](https://www.w3.org/TR/xml-infoset/#infoitem.element)

2

<a id="c-ims"></a>

If the application schema reference strategy using the [·actual value·](008-3-schema-component-details.md#key-vv)s of
the `schemaLocation` and `namespace` [[attributes]](https://www.w3.org/TR/xml-infoset/#infoitem.element),
provides a referent, as defined by [Schema Document Location Strategy (§4.3.2)](056-4-3-layer-3-schema-document-access-and-web-interoperability.md#schema_reference),
**one** of the following must be true:

2.1 The referent is (a fragment of) a resource which is an XML document
(see clause [1.1](052-4-schemas-and-namespaces-access-and-composition.md#c-vxd)), which in turn corresponds to a [<schema>](049-3-15-schemas-as-a-whole.md#element-schema)
element information item in a well-formed information set, which in turn
corresponds to a valid schema.

2.2 The referent is a [<schema>](049-3-15-schemas-as-a-whole.md#element-schema)
element information item in a well-formed information set, which in turn
corresponds to a valid schema.

In either case call the [<schema>](049-3-15-schemas-as-a-whole.md#element-schema) item **SII** and the valid schema **I**.

3 The appropriate **case** among the following
must be true:

3.1 **If** there is a `namespace` [[attribute]](https://www.w3.org/TR/xml-infoset/#infoitem.element), **then** its [·actual value·](008-3-schema-component-details.md#key-vv) must be identical to the [·actual value·](008-3-schema-component-details.md#key-vv) of the `targetNamespace` [[attribute]](https://www.w3.org/TR/xml-infoset/#infoitem.element) of **SII**.

3.2 **If** there is no `namespace` [[attribute]](https://www.w3.org/TR/xml-infoset/#infoitem.element), **then** **SII**
must have no `targetNamespace` [[attribute]](https://www.w3.org/TR/xml-infoset/#infoitem.element)

It is *not* an error for the application schema reference
strategy to fail. It
*is* an error for it to resolve but the rest of clause [2](055-continuation-55.md#c-ims) above to
fail to be satisfied. Failure to find a referent may well cause less than complete
[·assessment·](004-2-1-overview-of-xml-schema.md#key-va) outcomes, of course.The [·schema components·](004-2-1-overview-of-xml-schema.md#key-component) (that is
[{type definitions}](049-3-15-schemas-as-a-whole.md#type_definitions), [{attribute declarations}](049-3-15-schemas-as-a-whole.md#attribute_declarations), [{element declarations}](049-3-15-schemas-as-a-whole.md#element_declarations), [{attribute group definitions}](049-3-15-schemas-as-a-whole.md#attribute_group_definitions), [{model group definitions}](049-3-15-schemas-as-a-whole.md#model_group_definitions), [{notation declarations}](049-3-15-schemas-as-a-whole.md#notation_declarations)) of a schema corresponding to a
[<schema>](049-3-15-schemas-as-a-whole.md#element-schema) element information item with one or more [<import>](054-4-2-3-references-to-schema-components-across-namespaces.md#element-import) element information items must include not only definitions or declarations corresponding to the appropriate members of its [[children]](https://www.w3.org/TR/xml-infoset/#infoitem.element), but also, for each of those [<import>](054-4-2-3-references-to-schema-components-across-namespaces.md#element-import) element information items for which clause [2](055-continuation-55.md#c-ims) above is satisfied, a set of [·schema components·](004-2-1-overview-of-xml-schema.md#key-component) identical to all the [·schema components·](004-2-1-overview-of-xml-schema.md#key-component) of **I**.

**Note:**The above is carefully worded so that multiple [<import>](054-4-2-3-references-to-schema-components-across-namespaces.md#element-import)ing of the same schema document will not constitute a violation of
clause [2](051-3-15-5-schema-information-set-contributions.md#c-nmd) of [Schema Properties Correct (§3.15.6)](051-3-15-5-schema-information-set-contributions.md#sch-props-correct), but applications are
allowed, indeed encouraged, to avoid [<import>](054-4-2-3-references-to-schema-components-across-namespaces.md#element-import)ing the same schema document more than once to forestall the necessity of establishing identity
component by component. Given that the `schemaLocation`
[[attribute]](https://www.w3.org/TR/xml-infoset/#infoitem.element) is only a hint, it is open to applications to ignore all but the
first [<import>](054-4-2-3-references-to-schema-components-across-namespaces.md#element-import) for a given namespace, regardless of the [·actual value·](008-3-schema-component-details.md#key-vv) of
`schemaLocation`, but such a strategy risks missing useful
information when new `schemaLocation`s are offered.

