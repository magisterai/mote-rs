# 3.3.5 Element Declaration Information Set Contributions

Source: [XML Schema Part 1: Structures](https://www.w3.org/TR/2004/REC-xmlschema-1-20041028/) · 2nd edition, 2004-10-28.

[Document index](../README.md) · [Topic guide](../../../../README.md) · [Publisher](https://www.w3.org/TR/2004/REC-xmlschema-1-20041028/).

HTML normalized to Markdown. Complex tables and mathematical markup retain HTML; illustrations are stored locally.

[Previous](016-3-3-3-constraints-on-xml-representations-of-element-declarations.md) · [Next](018-3-3-6-constraints-on-element-declaration-schema-components.md)

3.1.1 The element information item's [[attributes]](https://www.w3.org/TR/xml-infoset/#infoitem.element) must be empty,
excepting those whose [[namespace name]](https://www.w3.org/TR/xml-infoset/#infoitem.attribute) is identical to `http://www.w3.org/2001/XMLSchema-instance` and whose [[local name]](https://www.w3.org/TR/xml-infoset/#infoitem.attribute) is one of `type`, `nil`, `schemaLocation` or `noNamespaceSchemaLocation`.

3.1.2 The element information item must have no element information item [[children]](https://www.w3.org/TR/xml-infoset/#infoitem.element).

3.1.3

<a id="c-sv1"></a>

If clause [3.2](016-3-3-3-constraints-on-xml-representations-of-element-declarations.md#c-nl) of [Element Locally Valid (Element) (§3.3.4)](016-3-3-3-constraints-on-xml-representations-of-element-declarations.md#cvc-elt) did not apply, then the [·normalized value·](008-3-schema-component-details.md#key-nv) must be [·valid·](004-2-1-overview-of-xml-schema.md#key-vn) with respect to the type definition as defined by [String Valid (§3.14.4)](047-3-14-3-constraints-on-xml-representations-of-simple-type-definitions.md#cvc-simple-type).

3.2 **If** the type definition is a complex type definition, **then** the element information item must be [·valid·](004-2-1-overview-of-xml-schema.md#key-vn) with respect to the type definition as per [Element Locally Valid (Complex Type) (§3.4.4)](026-3-4-3-constraints-on-xml-representations-of-complex-type-definitions.md#cvc-complex-type);

<a id="cvc-id"></a>

**Validation Rule: Validation Root Valid (ID/IDREF)**  

For an element information item which is the [·validation root·](057-5-schemas-and-schema-validity-assessment.md#key-vr) to be [·valid·](004-2-1-overview-of-xml-schema.md#key-vn)
**all** of the following must be true:

1 There must be no **ID/IDREF binding** in the item's
[[ID/IDREF table]](051-3-15-5-schema-information-set-contributions.md#e-ii_table) whose [[binding]](051-3-15-5-schema-information-set-contributions.md#iib-binding) is the
empty set.

2

<a id="c-uba"></a>

There must be no **ID/IDREF binding** in the item's [[ID/IDREF table]](051-3-15-5-schema-information-set-contributions.md#e-ii_table) whose [[binding]](051-3-15-5-schema-information-set-contributions.md#iib-binding) has more
than one member.

See [ID/IDREF Table (§3.15.5)](051-3-15-5-schema-information-set-contributions.md#sic-id) for the definition of **ID/IDREF binding**.

**Note:**The first clause above applies when there is a reference to an
undefined ID. The second applies when there is a multiply-defined ID. They
are separated out to ensure that distinct error codes (see [Outcome Tabulations (normative) (§C)](060-b-references-normative.md#outcomes)) are associated with these two cases.

**Note:**Although this rule applies at the [·validation
root·](057-5-schemas-and-schema-validity-assessment.md#key-vr), in practice processors, particularly streaming processors, may
wish to detect and signal the clause [2](017-3-3-5-element-declaration-information-set-contributions.md#c-uba) case as it arises.

**Note:**This reconstruction of [[XML 1.0 (Second Edition)]](060-b-references-normative.md#ref-xml)'s `ID/IDREF`
functionality is imperfect in that if the [·validation
root·](057-5-schemas-and-schema-validity-assessment.md#key-vr) is not the document element of an XML document, the results will
not necessarily be the same as those a validating parser would give were the
document to have a DTD with equivalent declarations.

<a id="cvc-assess-elt"></a>

**Validation Rule: Schema-Validity Assessment (Element)**  

The schema-validity assessment of an element information item depends
on its [·validation·](004-2-1-overview-of-xml-schema.md#key-vn) and the [·assessment·](004-2-1-overview-of-xml-schema.md#key-va) of its element information item
children and associated attribute information items, if any.So for an element information item's schema-validity to be assessed
**all** of the following must be true:

1

<a id="c-xd"></a>

**One** of the following must be true:

1.1

<a id="c-ed"></a>

**All** of the following must be true:

1.1.1 A non-[·absent·](008-3-schema-component-details.md#key-null) element declaration
must be known for it, because**one** of the following is true

1.1.1.1 A declaration was stipulated by the processor (see [Assessing Schema-Validity (§5.2)](057-5-schemas-and-schema-validity-assessment.md#validation_outcome)).

1.1.1.2 A declaration has been established as its [·context-determined declaration·](011-3-2-4-attribute-declaration-validation-rules.md#key-dd).

1.1.1.3 **All** of the following must be true:

1.1.1.3.1 Its [·context-determined declaration·](011-3-2-4-attribute-declaration-validation-rules.md#key-dd) is
not *skip*.

1.1.1.3.2 Its [[local name]](https://www.w3.org/TR/xml-infoset/#infoitem.element) and [[namespace name]](https://www.w3.org/TR/xml-infoset/#infoitem.element) resolve to an element declaration as defined by [QName resolution (Instance) (§3.15.4)](050-3-15-2-1-references-to-schema-components.md#cvc-resolve-instance).

1.1.2 Its [·validity·](004-2-1-overview-of-xml-schema.md#key-vn) with respect to that
declaration must have been evaluated as per [Element Locally Valid (Element) (§3.3.4)](016-3-3-3-constraints-on-xml-representations-of-element-declarations.md#cvc-elt).

1.1.3 If that evaluation involved the evaluation of [Element Locally Valid (Type) (§3.3.4)](016-3-3-3-constraints-on-xml-representations-of-element-declarations.md#cvc-type), clause [1](016-3-3-3-constraints-on-xml-representations-of-element-declarations.md#c-ct) thereof must be satisfied.

1.2

<a id="c-td"></a>

**All** of the following must be true:

1.2.1 A non-[·absent·](008-3-schema-component-details.md#key-null) type definition is known for
it because
**one** of the following is true

1.2.1.1 A type definition was stipulated by the processor
(see [Assessing Schema-Validity (§5.2)](057-5-schemas-and-schema-validity-assessment.md#validation_outcome)).

1.2.1.2 **All** of the following must be true:

1.2.1.2.1 There is an attribute information item among the element
information item's [[attributes]](https://www.w3.org/TR/xml-infoset/#infoitem.element) whose [[namespace name]](https://www.w3.org/TR/xml-infoset/#infoitem.attribute) is identical to `http://www.w3.org/2001/XMLSchema-instance` and whose [[local name]](https://www.w3.org/TR/xml-infoset/#infoitem.attribute) is `type`.

1.2.1.2.2 The [·normalized value·](008-3-schema-component-details.md#key-nv) of that attribute information item is
[·valid·](004-2-1-overview-of-xml-schema.md#key-vn) with respect to the built-in [QName](https://www.w3.org/TR/2004/REC-xmlschema-2-20041028/datatypes.html#QName) simple type, as defined by [String Valid (§3.14.4)](047-3-14-3-constraints-on-xml-representations-of-simple-type-definitions.md#cvc-simple-type).

1.2.1.2.3 The [·local name·](050-3-15-2-1-references-to-schema-components.md#q-local) and [·namespace name·](050-3-15-2-1-references-to-schema-components.md#q-uri) (as defined in [QName Interpretation (§3.15.3)](050-3-15-2-1-references-to-schema-components.md#src-qname)), of the [·actual value·](008-3-schema-component-details.md#key-vv) of that attribute information item resolve to a type definition, as defined in [QName resolution (Instance) (§3.15.4)](050-3-15-2-1-references-to-schema-components.md#cvc-resolve-instance) -- 

<a id="key-ltd1"></a>

[Definition:]call this type definition the **local type definition**.

1.2.1.2.4 If there is also a processor-stipulated type definition, the [·local type definition·](017-3-3-5-element-declaration-information-set-contributions.md#key-ltd1) must be
validly derived from that type definition given its [{prohibited substitutions}](020-3-4-complex-type-definitions.md#ct-exact),
as defined in [Type Derivation OK (Complex) (§3.4.6)](028-continuation-28.md#cos-ct-derived-ok) (if it is a complex type
definition), or given the empty set, as defined in [Type Derivation OK (Simple) (§3.14.6)](048-3-14-7-built-in-simple-type-definition.md#cos-st-derived-ok) (if it is a simple type definition).

1.2.2 The element information item's [·validity·](004-2-1-overview-of-xml-schema.md#key-vn) with respect to the [·local type definition·](017-3-3-5-element-declaration-information-set-contributions.md#key-ltd1) (if present and validly derived)
or the processor-stipulated type definition (if no [·local
type definition·](017-3-3-5-element-declaration-information-set-contributions.md#key-ltd1) is present) has been evaluated as per [Element Locally Valid (Type) (§3.3.4)](016-3-3-3-constraints-on-xml-representations-of-element-declarations.md#cvc-type).

2 The schema-validity of all the element information items among its
[[children]](https://www.w3.org/TR/xml-infoset/#infoitem.element) has been assessed as per [Schema-Validity Assessment (Element) (§3.3.4)](017-3-3-5-element-declaration-information-set-contributions.md#cvc-assess-elt), and the
schema-validity of all the attribute information items among its
[[attributes]](https://www.w3.org/TR/xml-infoset/#infoitem.element) has been assessed as per [Schema-Validity Assessment (Attribute) (§3.2.4)](011-3-2-4-attribute-declaration-validation-rules.md#cvc-assess-attr).

<a id="key-sva"></a>

[Definition:]If either case of
clause [1](017-3-3-5-element-declaration-information-set-contributions.md#c-xd) above holds, the element information item has been **strictly assessed**.If the item cannot be [·strictly
assessed·](017-3-3-5-element-declaration-information-set-contributions.md#key-sva), because neither clause [1.1](017-3-3-5-element-declaration-information-set-contributions.md#c-ed) nor clause [1.2](017-3-3-5-element-declaration-information-set-contributions.md#c-td) above are satisfied,

<a id="key-lva"></a>

[Definition:]an element information item's
schema validity may be **laxly assessed** if its [·context-determined declaration·](011-3-2-4-attribute-declaration-validation-rules.md#key-dd) is not *skip* by [·validating·](004-2-1-overview-of-xml-schema.md#key-vn) with respect to the [·ur-type definition·](029-3-4-7-built-in-complex-type-definition.md#ur-type-itself) as per [Element Locally Valid (Type) (§3.3.4)](016-3-3-3-constraints-on-xml-representations-of-element-declarations.md#cvc-type).

**Note:**In general if clause [1.1](017-3-3-5-element-declaration-information-set-contributions.md#c-ed) above holds
clause [1.2](017-3-3-5-element-declaration-information-set-contributions.md#c-td) does not, and vice versa. When an
`xsi:type` [[attribute]](https://www.w3.org/TR/xml-infoset/#infoitem.element) is involved, however, clause [1.2](017-3-3-5-element-declaration-information-set-contributions.md#c-td) takes precedence,
as is made clear in [Element Locally Valid (Element) (§3.3.4)](016-3-3-3-constraints-on-xml-representations-of-element-declarations.md#cvc-elt).

**Note:**The [{name}](013-3-3-element-declarations.md#e-name) and [{target namespace}](013-3-3-element-declarations.md#e-target_namespace) properties are not
mentioned above because they are checked during particle [·validation·](004-2-1-overview-of-xml-schema.md#key-vn), as per
[Element Sequence Locally Valid (Particle) (§3.9.4)](035-3-9-particles.md#cvc-particle).

<a id="d0e5070"></a>

#### 3.3.5 Element Declaration Information Set Contributions

<a id="sic-e-outcome"></a>

**Schema Information Set Contribution: Assessment Outcome (Element)**  

If the schema-validity of an element information item has been assessed
as per [Schema-Validity Assessment (Element) (§3.3.4)](017-3-3-5-element-declaration-information-set-contributions.md#cvc-assess-elt), then in the [·post-schema-validation infoset·](004-2-1-overview-of-xml-schema.md#key-psvi) it has properties as follows:

PSVI Contributions
for element information items

<a id="e-validation_context"></a>

[validation context]
:   The nearest ancestor element information
    item with a [[schema information]](051-3-15-5-schema-information-set-contributions.md#e-schema_information) property (or this element item itself if it has such a property).

<a id="e-validity"></a>

[validity]
:   The appropriate **case** among the following:

    1 **If** it was [·strictly
    assessed·](017-3-3-5-element-declaration-information-set-contributions.md#key-sva), **then** 
    the appropriate **case** among the following:

    1.1 **If** 
    **all** of the following are true

    1.1.1

    1.1.1.1 clause [1.1](017-3-3-5-element-declaration-information-set-contributions.md#c-ed) of [Schema-Validity Assessment (Element) (§3.3.4)](017-3-3-5-element-declaration-information-set-contributions.md#cvc-assess-elt)
    applied and the item was
    [·valid·](004-2-1-overview-of-xml-schema.md#key-vn) as defined by [Element Locally Valid (Element) (§3.3.4)](016-3-3-3-constraints-on-xml-representations-of-element-declarations.md#cvc-elt);

    1.1.1.2 clause [1.2](017-3-3-5-element-declaration-information-set-contributions.md#c-td) of [Schema-Validity Assessment (Element) (§3.3.4)](017-3-3-5-element-declaration-information-set-contributions.md#cvc-assess-elt)
    applied and the item was
    [·valid·](004-2-1-overview-of-xml-schema.md#key-vn) as defined by [Element Locally Valid (Type) (§3.3.4)](016-3-3-3-constraints-on-xml-representations-of-element-declarations.md#cvc-type).

    1.1.2 Neither its [[children]](https://www.w3.org/TR/xml-infoset/#infoitem.element) nor its
    [[attributes]](https://www.w3.org/TR/xml-infoset/#infoitem.element) contains an information item (element or attribute respectively) whose [validity] is *invalid*.

