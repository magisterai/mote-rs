# 4.3 Layer 3: Schema Document Access and Web-interoperability

Source: [XML Schema Part 1: Structures](https://www.w3.org/TR/2004/REC-xmlschema-1-20041028/) · 2nd edition, 2004-10-28.

[Document index](../README.md) · [Topic guide](../../../../README.md) · [Publisher](https://www.w3.org/TR/2004/REC-xmlschema-1-20041028/).

HTML normalized to Markdown. Complex tables and mathematical markup retain HTML; illustrations are stored locally.

[Previous](055-continuation-55.md) · [Next](057-5-schemas-and-schema-validity-assessment.md)

<a id="composition-instances"></a>

### 4.3 Layer 3: Schema Document Access and Web-interoperability

4.3.1 [Standards for representation of schemas and retrieval of schema documents on the Web](056-4-3-layer-3-schema-document-access-and-web-interoperability.md#schema-repr)  
        4.3.2 [How schema definitions are located on the Web](056-4-3-layer-3-schema-document-access-and-web-interoperability.md#schema-loc)

Layers 1 and 2 provide a framework for [·assessment·](004-2-1-overview-of-xml-schema.md#key-va) and XML definition of schemas in a broad variety of environments. Over time, a range
of standards and conventions may well evolve to support interoperability of XML
Schema implementations on the World Wide Web. Layer 3 defines the minimum level
of function required of all conformant processors operating on the Web:
it is intended that, over time, future standards (e.g. XML Packages) for interoperability on the
Web and in other environments can be introduced without the need to republish
this specification.

<a id="schema-repr"></a>

#### 4.3.1 Standards for representation of schemas and retrieval of schema documents on the Web

For interoperability, serialized [·schema documents·](008-3-schema-component-details.md#key-schemaDoc), like all other Web resources, may be identified by
URI and retrieved using the standard mechanisms of the Web (e.g. http, https,
etc.) Such documents on the Web must be part of XML documents (see clause [1.1](052-4-schemas-and-namespaces-access-and-composition.md#c-vxd)), and are represented in the
standard XML schema definition form described by layer 2 (that is as [<schema>](049-3-15-schemas-as-a-whole.md#element-schema)
element information items).

**Note:** there will often be times when a schema document will be a
complete XML 1.0 document whose document element is [<schema>](049-3-15-schemas-as-a-whole.md#element-schema). There will be
other occasions in which [<schema>](049-3-15-schemas-as-a-whole.md#element-schema) items will be contained in other
documents, perhaps referenced using fragment and/or XPointer notation.

**Note:**The variations among server software and web site administration policies
make it difficult to recommend any particular approach to retrieval requests
intended to retrieve serialized [·schema documents·](008-3-schema-component-details.md#key-schemaDoc). An `Accept` header of `application/xml,
text/xml; q=0.9, */*` is perhaps a reasonable starting point.

<a id="schema-loc"></a>

#### 4.3.2 How schema definitions are located on the Web

As described in [Layer 1: Summary of the Schema-validity Assessment Core (§4.1)](052-4-schemas-and-namespaces-access-and-composition.md#layer1), processors are responsible for providing the
schema components (definitions and declarations) needed for [·assessment·](004-2-1-overview-of-xml-schema.md#key-va). This
section introduces a set of normative conventions to facilitate interoperability
for instance and schema documents retrieved and processed from the Web.

**Note:** As discussed above in [Layer 2: Schema Documents, Namespaces and Composition (§4.2)](052-4-schemas-and-namespaces-access-and-composition.md#layer2), other non-Web mechanisms for delivering schemas for [·assessment·](004-2-1-overview-of-xml-schema.md#key-va) may exist, but are outside the scope of this specification.

Processors on the Web are free to undertake [·assessment·](004-2-1-overview-of-xml-schema.md#key-va) against arbitrary
schemas in any of the ways set out in [Assessing Schema-Validity (§5.2)](057-5-schemas-and-schema-validity-assessment.md#validation_outcome). However, it
is useful to have a common convention for determining the schema to use. Accordingly, general-purpose schema-aware processors (i.e. those not
specialized to one or a fixed set of pre-determined schemas)
undertaking [·assessment·](004-2-1-overview-of-xml-schema.md#key-va) of a document on the web
must behave as follows:

- unless directed otherwise by the user, [·assessment·](004-2-1-overview-of-xml-schema.md#key-va) is undertaken on the document
  element information item of the specified document;
- unless directed otherwise by the user, the
  processor is required to construct a schema corresponding to a schema document
  whose `targetNamespace` is
  identical to the
  namespace name, if any, of the element information item on which [·assessment·](004-2-1-overview-of-xml-schema.md#key-va) is undertaken.

The composition of the complete
schema for use in [·assessment·](004-2-1-overview-of-xml-schema.md#key-va) is discussed in [Layer 2: Schema Documents, Namespaces and Composition (§4.2)](052-4-schemas-and-namespaces-access-and-composition.md#layer2) above.
The means used to locate appropriate schema document(s) are processor and
application dependent, subject to the following requirements:

1. Schemas are represented on the Web in the form specified above in [Standards for representation of schemas and retrieval of schema documents on the Web (§4.3.1)](056-4-3-layer-3-schema-document-access-and-web-interoperability.md#schema-repr);
2. The author of a document uses namespace declarations to indicate the intended
   interpretation of names appearing therein; there may or may not be a schema
   retrievable via the namespace name. Accordingly whether a processor's default
   behavior is or is not to attempt such dereferencing, it must always provide
   for user-directed overriding of that default.

   **Note:** Experience suggests that it is not in all cases safe or desirable from
   a performance point of view to dereference namespace names as a matter of course. User community and/or
   consumer/provider agreements may establish circumstances in which such dereference is a sensible
   default strategy: this specification allows but does not require particular communities to
   establish and implement such conventions. Users are always free to supply namespace names as schema location information when dereferencing *is* desired: see below.
3. On the other hand, in case a document author (human or not) created a
   document with a particular schema in view, and warrants that some or
   all of the document conforms to that schema, the
   `schemaLocation` and `noNamespaceSchemaLocation` [[attributes]](https://www.w3.org/TR/xml-infoset/#infoitem.element) (in the XML Schema instance namespace,
   that is, `http://www.w3.org/2001/XMLSchema-instance`) (hereafter
   `xsi:schemaLocation` and
   `xsi:noNamespaceSchemaLocation`) are provided. The first records
   the author's warrant with pairs of URI references (one for the namespace name, and
   one for a hint as to the location of a schema document defining names for that
   namespace name). The second similarly provides a URI reference as a hint as to
   the location of a schema document with no `targetNamespace` [[attribute]](https://www.w3.org/TR/xml-infoset/#infoitem.element).Unless directed otherwise, for example by the invoking application
   or by command line option, processors
   should attempt to dereference each schema document location URI in the
   [·actual value·](008-3-schema-component-details.md#key-vv) of such
   `xsi:schemaLocation` and `xsi:noNamespaceSchemaLocation` [[attributes]](https://www.w3.org/TR/xml-infoset/#infoitem.element), see details below.
4. `xsi:schemaLocation` and
   `xsi:noNamespaceSchemaLocation` [[attributes]](https://www.w3.org/TR/xml-infoset/#infoitem.element) can occur on any
   element. However, it is an error if such an attribute occurs
   *after* the first appearance of an element or attribute information
   item within an
   element information item initially [·validated·](004-2-1-overview-of-xml-schema.md#key-vn) whose [[namespace name]](https://www.w3.org/TR/xml-infoset/#infoitem.attribute) it addresses. According to the rules of
   [Layer 1: Summary of the Schema-validity Assessment Core (§4.1)](052-4-schemas-and-namespaces-access-and-composition.md#layer1), the corresponding schema may be lazily assembled, but is otherwise
   stable throughout [·assessment·](004-2-1-overview-of-xml-schema.md#key-va). Although schema location attributes can occur
   on any element, and can be processed incrementally as discovered, their effect
   is essentially global to the [·assessment·](004-2-1-overview-of-xml-schema.md#key-va). Definitions and declarations remain
   in effect beyond the scope of the element on which the binding is declared.

Example

Multiple schema bindings can be declared using a single
attribute. For example consider a stylesheet:



```

 <stylesheet xmlns="http://www.w3.org/1999/XSL/Transform"
            xmlns:html="http://www.w3.org/1999/xhtml"
            xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance"
            xsi:schemaLocation="http://www.w3.org/1999/XSL/Transform
                                http://www.w3.org/1999/XSL/Transform.xsd
                                http://www.w3.org/1999/xhtml
                                http://www.w3.org/1999/xhtml.xsd">
```




The namespace names used in `schemaLocation` can, but need not
be identical to those actually qualifying the element within whose start tag
it is found or its other attributes. For example, as above, all
schema location information can be declared on the document element
of a document, if desired,
regardless of where the namespaces are actually used.

<a id="schema_reference"></a>

**Schema Representation Constraint: Schema Document Location Strategy**  

Given a namespace name (or none) and (optionally) a URI reference from
`xsi:schemaLocation` or `xsi:noNamespaceSchemaLocation`,
schema-aware processors may implement any combination of the following
strategies, in any order:

1
Do nothing, for instance because a schema containing components for the
given namespace name is already known to be available, or because it
is known in advance that no efforts to locate schema documents will be successful
(for
example in embedded systems);

2
Based on the location URI,
identify an existing schema document,
either as a resource which is an XML document or a [<schema>](049-3-15-schemas-as-a-whole.md#element-schema) element
information item, in some local schema repository;

3
Based on the namespace name, identify an existing schema document,
either as a resource which is an XML document or a [<schema>](049-3-15-schemas-as-a-whole.md#element-schema) element
information item, in some local schema repository;

4
Attempt to resolve the location URI,
to locate a
resource on the web which is or contains or references a [<schema>](049-3-15-schemas-as-a-whole.md#element-schema) element;

5
Attempt to resolve the namespace name to locate such a resource.

Whenever possible configuration and/or invocation options for selecting and/or ordering
the implemented strategies should be provided.

Improved or alternative conventions for Web interoperability can
be standardized in the future without reopening this specification. For
example, the W3C is currently considering initiatives to standardize the
packaging of resources relating to particular documents and/or namespaces: this
would be an addition to the mechanisms described here for layer 3. This
architecture also facilitates innovation at layer 2: for example, it would be
possible in the future to define an additional standard for the representation of
schema components which allowed e.g. type definitions to be specified piece by
piece, rather than all at once.

