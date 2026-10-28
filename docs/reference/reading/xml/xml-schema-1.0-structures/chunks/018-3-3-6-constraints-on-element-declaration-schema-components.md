# 3.3.6 Constraints on Element Declaration Schema Components

Source: [XML Schema Part 1: Structures](https://www.w3.org/TR/2004/REC-xmlschema-1-20041028/) · 2nd edition, 2004-10-28.

[Document index](../README.md) · [Topic guide](../../../../README.md) · [Publisher](https://www.w3.org/TR/2004/REC-xmlschema-1-20041028/).

HTML normalized to Markdown. Complex tables and mathematical markup retain HTML; illustrations are stored locally.

[Previous](017-3-3-5-element-declaration-information-set-contributions.md) · [Next](019-continuation-19.md)

    1.1.3 Neither its [[children]](https://www.w3.org/TR/xml-infoset/#infoitem.element) nor its
    [[attributes]](https://www.w3.org/TR/xml-infoset/#infoitem.element) contains an information item (element or attribute respectively) with a [·context-determined declaration·](011-3-2-4-attribute-declaration-validation-rules.md#key-dd) of
    *mustFind* whose [validity]
    is *notKnown*.

    , **then** *valid*;

    1.2 **otherwise** *invalid.*.

    2 **otherwise** *notKnown*.

<a id="e-validation_attempted"></a>

[validation attempted]
:   The appropriate **case** among the following:

    1 **If** it was [·strictly assessed·](017-3-3-5-element-declaration-information-set-contributions.md#key-sva) and neither its [[children]](https://www.w3.org/TR/xml-infoset/#infoitem.element) nor its
    [[attributes]](https://www.w3.org/TR/xml-infoset/#infoitem.element) contains an information item (element or attribute
    respectively) whose [validation attempted] is not
    *full*, **then** *full*;

    2 **If** it was not [·strictly assessed·](017-3-3-5-element-declaration-information-set-contributions.md#key-sva) and neither its [[children]](https://www.w3.org/TR/xml-infoset/#infoitem.element) nor its
    [[attributes]](https://www.w3.org/TR/xml-infoset/#infoitem.element) contains an information item (element or attribute
    respectively) whose [validation attempted] is not
    *none*, **then** *none*;

    3 **otherwise** *partial*.

<a id="sic-elt-error-code"></a>

**Schema Information Set Contribution: Validation Failure (Element)**  

If the local [·validity·](004-2-1-overview-of-xml-schema.md#key-vn), as defined by
[Element Locally Valid (Element) (§3.3.4)](016-3-3-3-constraints-on-xml-representations-of-element-declarations.md#cvc-elt) above and/or [Element Locally Valid (Type) (§3.3.4)](016-3-3-3-constraints-on-xml-representations-of-element-declarations.md#cvc-type)
below, of an element information item has been assessed,
in the [·post-schema-validation infoset·](004-2-1-overview-of-xml-schema.md#key-psvi) the item has a
property:

PSVI Contributions
for element information items

<a id="e-schema_error_code"></a>

[schema error code]
:   The appropriate **case** among the following:

    1 **If** the item is not [·valid·](004-2-1-overview-of-xml-schema.md#key-vn), **then** a list. Applications wishing to provide
    information as to the reason(s) for the [·validation·](004-2-1-overview-of-xml-schema.md#key-vn) failure are encouraged to record one or more
    error codes (see [Outcome Tabulations (normative) (§C)](060-b-references-normative.md#outcomes)) herein.

    2 **otherwise** [·absent·](008-3-schema-component-details.md#key-null).

<a id="sic-elt-decl"></a>

**Schema Information Set Contribution: Element Declaration**  

If an element information item is [·valid·](004-2-1-overview-of-xml-schema.md#key-vn) with respect to an element
declaration as per [Element Locally Valid (Element) (§3.3.4)](016-3-3-3-constraints-on-xml-representations-of-element-declarations.md#cvc-elt) then in the [·post-schema-validation infoset·](004-2-1-overview-of-xml-schema.md#key-psvi)
the element
information item must, at processor option, have either:

PSVI Contributions
for element information items

<a id="e-declaration"></a>

[element declaration]
:   an [·item isomorphic·](051-3-15-5-schema-information-set-contributions.md#key-iso) to the declaration component itself

or

PSVI Contributions
for element information items

<a id="e-nil"></a>

[nil]
:   *true* if clause [3.2](016-3-3-3-constraints-on-xml-representations-of-element-declarations.md#c-nl) of [Element Locally Valid (Element) (§3.3.4)](016-3-3-3-constraints-on-xml-representations-of-element-declarations.md#cvc-elt) above is satisfied,
    otherwise *false*

<a id="sic-eltType"></a>

**Schema Information Set Contribution: Element Validated by Type**  

If an element information item is [·valid·](004-2-1-overview-of-xml-schema.md#key-vn) with respect to a [·type definition·](004-2-1-overview-of-xml-schema.md#key-typeDefn)
as per [Element Locally Valid (Type) (§3.3.4)](016-3-3-3-constraints-on-xml-representations-of-element-declarations.md#cvc-type), in the [·post-schema-validation infoset·](004-2-1-overview-of-xml-schema.md#key-psvi) the item has a property:

PSVI Contributions
for element information items

<a id="e-schema_normalized_value"></a>

[schema normalized value]
:   The appropriate **case** among the following:

    1 **If** clause [3.2](016-3-3-3-constraints-on-xml-representations-of-element-declarations.md#c-nl) of [Element Locally Valid (Element) (§3.3.4)](016-3-3-3-constraints-on-xml-representations-of-element-declarations.md#cvc-elt) and [Element Default Value (§3.3.5)](018-3-3-6-constraints-on-element-declaration-schema-components.md#sic-eltDefault) above have
    *not* applied and either the [·type definition·](004-2-1-overview-of-xml-schema.md#key-typeDefn) is a simple type definition or its [{content type}](020-3-4-complex-type-definitions.md#content_type) is a simple type definition, **then** 
    the [·normalized value·](008-3-schema-component-details.md#key-nv) of the item as [·validated·](004-2-1-overview-of-xml-schema.md#key-vn).

    2 **otherwise** [·absent·](008-3-schema-component-details.md#key-null).

Furthermore, the item has one of the following alternative sets of properties:Either

PSVI Contributions
for element information items

<a id="e-type_definition"></a>

[type definition]
:   An
    [·item isomorphic·](051-3-15-5-schema-information-set-contributions.md#key-iso) to the [·type definition·](004-2-1-overview-of-xml-schema.md#key-typeDefn) component itself.

<a id="e-member_type_definition"></a>

[member type definition]
:   If
    and only if that type definition is a
    simple type definition with [{variety}](045-3-14-simple-type-definitions.md#variety) *union*, or
    a complex type definition whose [{content type}](020-3-4-complex-type-definitions.md#content_type) is a
    simple type
    definition with [{variety}](045-3-14-simple-type-definitions.md#variety) *union*, then an
    [·item isomorphic·](051-3-15-5-schema-information-set-contributions.md#key-iso) to that member of the
    union's [{member type definitions}](045-3-14-simple-type-definitions.md#st-member_type_definitions) which actually [·validated·](004-2-1-overview-of-xml-schema.md#key-vn) the element item's [·normalized value·](008-3-schema-component-details.md#key-nv).

or

PSVI Contributions
for element information items

<a id="e-type_definition_type"></a>

[type definition type]
:   *simple* or *complex*, depending on the [·type definition·](004-2-1-overview-of-xml-schema.md#key-typeDefn).

<a id="e-type_definition_namespace"></a>

[type definition namespace]
:   The {target namespace} of the [·type definition·](004-2-1-overview-of-xml-schema.md#key-typeDefn).

<a id="e-type_definition_anonymous"></a>

[type definition anonymous]
:   *true* if the {name} of the [·type definition·](004-2-1-overview-of-xml-schema.md#key-typeDefn) is [·absent·](008-3-schema-component-details.md#key-null), otherwise *false*.

<a id="e-type_definition_name"></a>

[type definition name]
:   The {name} of the [·type definition·](004-2-1-overview-of-xml-schema.md#key-typeDefn), if it is not [·absent·](008-3-schema-component-details.md#key-null). If it is
    [·absent·](008-3-schema-component-details.md#key-null), schema processors may, but need not,
    provide a value unique to the definition.

If the [·type definition·](004-2-1-overview-of-xml-schema.md#key-typeDefn) is a
simple type definition or its [{content type}](020-3-4-complex-type-definitions.md#content_type) is a
simple type definition, and that type
definition has [{variety}](045-3-14-simple-type-definitions.md#variety) *union*, then calling

<a id="key-amt"></a>

[Definition:] that
member of the [{member type definitions}](045-3-14-simple-type-definitions.md#st-member_type_definitions) which actually
[·validated·](004-2-1-overview-of-xml-schema.md#key-vn) the element item's [·normalized value·](008-3-schema-component-details.md#key-nv) the
**actual member type definition**, there are three additional properties:

PSVI Contributions
for element information items

<a id="e-member_type_definition_namespace"></a>

[member type definition namespace]
:   The [{target namespace}](045-3-14-simple-type-definitions.md#st-target_namespace) of the [·actual
    member type definition·](018-3-3-6-constraints-on-element-declaration-schema-components.md#key-amt).

<a id="e-member_type_definition_anonymous"></a>

[member type definition anonymous]
:   *true* if the [{name}](045-3-14-simple-type-definitions.md#st-name) of the [·actual member type definition·](018-3-3-6-constraints-on-element-declaration-schema-components.md#key-amt) is [·absent·](008-3-schema-component-details.md#key-null), otherwise *false*.

<a id="e-member_type_definition_name"></a>

[member type definition name]
:   The [{name}](045-3-14-simple-type-definitions.md#st-name) of the [·actual member type definition·](018-3-3-6-constraints-on-element-declaration-schema-components.md#key-amt), if it is not [·absent·](008-3-schema-component-details.md#key-null). If it is
    [·absent·](008-3-schema-component-details.md#key-null), schema processors may, but need not,
    provide a value unique to the definition.

The first ([·item isomorphic·](051-3-15-5-schema-information-set-contributions.md#key-iso)) alternative above is provided for applications such as query
processors which need access to the full range of details about an item's
[·assessment·](004-2-1-overview-of-xml-schema.md#key-va), for example the type hierarchy; the second, for lighter-weight
processors for whom representing the significant parts of the type hierarchy as
information items might be a significant burden.Also, if the declaration has a [{value constraint}](013-3-3-element-declarations.md#e-value_constraint), the item has a property:

PSVI Contributions
for element information items

<a id="e-schema_default"></a>

[schema default]
:   The [canonical lexical representation](https://www.w3.org/TR/2004/REC-xmlschema-2-20041028/datatypes.html#dt-canonical-representation) of
    the declaration's [{value constraint}](013-3-3-element-declarations.md#e-value_constraint) value.

Note that if an element is [·laxly assessed·](017-3-3-5-element-declaration-information-set-contributions.md#key-lva), then the [[type definition]](018-3-3-6-constraints-on-element-declaration-schema-components.md#e-type_definition) and
[[member type definition]](018-3-3-6-constraints-on-element-declaration-schema-components.md#e-member_type_definition) properties, or their
alternatives, are based on the [·ur-type definition·](029-3-4-7-built-in-complex-type-definition.md#ur-type-itself).

<a id="sic-eltDefault"></a>

**Schema Information Set Contribution: Element Default Value**  

If the local [·validity·](004-2-1-overview-of-xml-schema.md#key-vn), as defined by [Element Locally Valid (Element) (§3.3.4)](016-3-3-3-constraints-on-xml-representations-of-element-declarations.md#cvc-elt)
above, of an element information item has been assessed,
in the [·post-schema-validation infoset·](004-2-1-overview-of-xml-schema.md#key-psvi) the item has a
property:

PSVI Contributions
for element information items

<a id="e-schema_specified"></a>

[schema specified]
:   The appropriate **case** among the following:

    1 **If** the item is [·valid·](004-2-1-overview-of-xml-schema.md#key-vn) with respect to an element
    declaration as per [Element Locally Valid (Element) (§3.3.4)](016-3-3-3-constraints-on-xml-representations-of-element-declarations.md#cvc-elt) and the [{value constraint}](013-3-3-element-declarations.md#e-value_constraint) is present, but clause [3.2](016-3-3-3-constraints-on-xml-representations-of-element-declarations.md#c-nl)
    of [Element Locally Valid (Element) (§3.3.4)](016-3-3-3-constraints-on-xml-representations-of-element-declarations.md#cvc-elt) above is not satisfied and the item has no element or character information item [[children]](https://www.w3.org/TR/xml-infoset/#infoitem.element), **then** 
    *schema*. Furthermore, the
    [·post-schema-validation infoset·](004-2-1-overview-of-xml-schema.md#key-psvi) has the [canonical lexical representation](https://www.w3.org/TR/2004/REC-xmlschema-2-20041028/datatypes.html#dt-canonical-representation) of the [{value constraint}](013-3-3-element-declarations.md#e-value_constraint) value as the
    item's [[schema normalized value]](018-3-3-6-constraints-on-element-declaration-schema-components.md#e-schema_normalized_value)
    property.

    2 **otherwise** *infoset*.

<a id="coss-element"></a>

#### 3.3.6 Constraints on Element Declaration Schema Components

All element declarations (see [Element Declarations (§3.3)](013-3-3-element-declarations.md#cElement_Declarations)) must satisfy the following constraint.

<a id="e-props-correct"></a>

**Schema Component Constraint: Element Declaration Properties Correct**  

**All** of the following must be true:

