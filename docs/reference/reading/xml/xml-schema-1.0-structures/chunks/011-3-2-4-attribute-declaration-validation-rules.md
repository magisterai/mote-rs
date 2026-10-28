# 3.2.4 Attribute Declaration Validation Rules

Source: [XML Schema Part 1: Structures](https://www.w3.org/TR/2004/REC-xmlschema-1-20041028/) · 2nd edition, 2004-10-28.

[Document index](../README.md) · [Topic guide](../../../../README.md) · [Publisher](https://www.w3.org/TR/2004/REC-xmlschema-1-20041028/).

HTML normalized to Markdown. Complex tables and mathematical markup retain HTML; illustrations are stored locally.

[Previous](010-3-2-3-constraints-on-xml-representations-of-attribute-declarations.md) · [Next](012-3-2-6-constraints-on-attribute-declaration-schema-components.md)

5 The corresponding attribute
declaration must satisfy the conditions set out in
[Constraints on Attribute Declaration Schema Components (§3.2.6)](012-3-2-6-constraints-on-attribute-declaration-schema-components.md#coss-attribute).

<a id="d0e2483"></a>

#### 3.2.4 Attribute Declaration Validation Rules

<a id="cvc-attribute"></a>

**Validation Rule: Attribute Locally Valid**  

For an attribute information item to be locally [·valid·](004-2-1-overview-of-xml-schema.md#key-vn) with respect to an
attribute declaration
**all** of the following must be true:

1

<a id="c-a1"></a>

The declaration must not be [·absent·](008-3-schema-component-details.md#key-null) (see [Missing Sub-components (§5.3)](058-5-3-missing-sub-components.md#conformance-missing) for
how this can fail to be the case).

2

<a id="c-a2"></a>

Its [{type definition}](009-3-2-attribute-declarations.md#a-simple_type_definition) must not be absent.

3

<a id="c-sva"></a>

The item's [·normalized value·](008-3-schema-component-details.md#key-nv) must be locally [·valid·](004-2-1-overview-of-xml-schema.md#key-vn) with respect to that [{type definition}](009-3-2-attribute-declarations.md#a-simple_type_definition) as per [String Valid (§3.14.4)](047-3-14-3-constraints-on-xml-representations-of-simple-type-definitions.md#cvc-simple-type).

4 The item's [·actual value·](008-3-schema-component-details.md#key-vv) must match the value of the [{value constraint}](009-3-2-attribute-declarations.md#a-value_constraint), if it is
present and *fixed*.

<a id="cvc-assess-attr"></a>

**Validation Rule: Schema-Validity Assessment (Attribute)**  

The schema-validity assessment of an attribute information item depends
on its [·validation·](004-2-1-overview-of-xml-schema.md#key-vn) alone.

<a id="key-dd"></a>

[Definition:]During [·validation·](004-2-1-overview-of-xml-schema.md#key-vn), associations
between element and attribute information items among the [[children]](https://www.w3.org/TR/xml-infoset/#infoitem.element)
and [[attributes]](https://www.w3.org/TR/xml-infoset/#infoitem.element) on the one hand, and element and attribute
declarations on the other, are established as a side-effect. Such
declarations are called the **context-determined declarations**.
See clause [3.1](026-3-4-3-constraints-on-xml-representations-of-complex-type-definitions.md#c-ctma) (in [Element Locally Valid (Complex Type) (§3.4.4)](026-3-4-3-constraints-on-xml-representations-of-complex-type-definitions.md#cvc-complex-type)) for
attribute declarations, clause [2](035-3-9-particles.md#c-cdde) (in [Element Sequence Locally Valid (Particle) (§3.9.4)](035-3-9-particles.md#cvc-particle)) for element
declarations.For an attribute information item's schema-validity to have been assessed
**all** of the following must be true:

1 A non-[·absent·](008-3-schema-component-details.md#key-null) attribute declaration
must be known for it, namely
**one** of the following:

1.1 A declaration which has been established as its [·context-determined declaration·](011-3-2-4-attribute-declaration-validation-rules.md#key-dd);

1.2

<a id="c-adbyr"></a>

A declaration resolved to by its [[local name]](https://www.w3.org/TR/xml-infoset/#infoitem.attribute) and [[namespace name]](https://www.w3.org/TR/xml-infoset/#infoitem.attribute) as defined by [QName resolution (Instance) (§3.15.4)](050-3-15-2-1-references-to-schema-components.md#cvc-resolve-instance), provided its [·context-determined declaration·](011-3-2-4-attribute-declaration-validation-rules.md#key-dd) is
not *skip*.

2 Its [·validity·](004-2-1-overview-of-xml-schema.md#key-vn) with respect to that
declaration must have been evaluated as per [Attribute Locally Valid (§3.2.4)](011-3-2-4-attribute-declaration-validation-rules.md#cvc-attribute).

3 Both clause [1](011-3-2-4-attribute-declaration-validation-rules.md#c-a1) and clause [2](011-3-2-4-attribute-declaration-validation-rules.md#c-a2) of [Attribute Locally Valid (§3.2.4)](011-3-2-4-attribute-declaration-validation-rules.md#cvc-attribute) must be satisfied.

<a id="key-svaa"></a>

[Definition:]For attributes, there is no
difference between assessment and strict assessment, so if the above holds, the attribute information item has been **strictly assessed**.

<a id="d0e2622"></a>

#### 3.2.5 Attribute Declaration Information Set Contributions

<a id="sic-a-outcome"></a>

**Schema Information Set Contribution: Assessment Outcome (Attribute)**  

If the schema-validity of an attribute information item has been assessed
as per [Schema-Validity Assessment (Attribute) (§3.2.4)](011-3-2-4-attribute-declaration-validation-rules.md#cvc-assess-attr), then in the [·post-schema-validation infoset·](004-2-1-overview-of-xml-schema.md#key-psvi) it has properties as follows:

PSVI Contributions
for attribute information items

<a id="a-validation_context"></a>

[validation context]
:   The nearest ancestor element information
    item with a [[schema information]](051-3-15-5-schema-information-set-contributions.md#e-schema_information) property.

<a id="a-validity"></a>

[validity]
:   The appropriate **case** among the following:

    1 **If** it was [·strictly assessed·](011-3-2-4-attribute-declaration-validation-rules.md#key-svaa), **then** 
    the appropriate **case** among the following:

    1.1 **If** it was
    [·valid·](004-2-1-overview-of-xml-schema.md#key-vn) as defined by [Attribute Locally Valid (§3.2.4)](011-3-2-4-attribute-declaration-validation-rules.md#cvc-attribute), **then** *valid*;

    1.2 **otherwise** *invalid*.

    2 **otherwise** *notKnown*.

<a id="a-validation_attempted"></a>

[validation attempted]
:   The appropriate **case** among the following:

    1 **If** it was [·strictly assessed·](011-3-2-4-attribute-declaration-validation-rules.md#key-svaa), **then** *full*;

    2 **otherwise** *none*.

<a id="a-schema_specified"></a>

[schema specified]
:   *infoset*. See [Attribute Default Value (§3.4.5)](027-3-4-5-complex-type-definition-information-set-contributions.md#sic-attrDefault) for the other possible value.

<a id="sic-attr-error-code"></a>

**Schema Information Set Contribution: Validation Failure (Attribute)**  

If the local [·validity·](004-2-1-overview-of-xml-schema.md#key-vn), as defined by [Attribute Locally Valid (§3.2.4)](011-3-2-4-attribute-declaration-validation-rules.md#cvc-attribute)
above, of an attribute information item has been assessed,
in the [·post-schema-validation infoset·](004-2-1-overview-of-xml-schema.md#key-psvi) the item has a
property:

PSVI Contributions
for attribute information items

<a id="a-schema_error_code"></a>

[schema error code]
:   The appropriate **case** among the following:

    1 **If** the item is not [·valid·](004-2-1-overview-of-xml-schema.md#key-vn), **then** a list. Applications wishing to provide
    information as to the reason(s) for the [·validation·](004-2-1-overview-of-xml-schema.md#key-vn) failure are encouraged to record one or more
    error codes (see [Outcome Tabulations (normative) (§C)](060-b-references-normative.md#outcomes)) herein.

    2 **otherwise** [·absent·](008-3-schema-component-details.md#key-null).

<a id="sic-attr-decl"></a>

**Schema Information Set Contribution: Attribute Declaration**  

If an attribute information item is [·valid·](004-2-1-overview-of-xml-schema.md#key-vn) with respect to an attribute
declaration as per [Attribute Locally Valid (§3.2.4)](011-3-2-4-attribute-declaration-validation-rules.md#cvc-attribute) then in the [·post-schema-validation infoset·](004-2-1-overview-of-xml-schema.md#key-psvi) the attribute
information item may, at processor option, have a property:

PSVI Contributions
for attribute information items

<a id="a-declaration"></a>

[attribute declaration]
:   An [·item isomorphic·](051-3-15-5-schema-information-set-contributions.md#key-iso) to the declaration component itself.

<a id="sic-attrType"></a>

**Schema Information Set Contribution: Attribute Validated by Type**  

If clause [3](011-3-2-4-attribute-declaration-validation-rules.md#c-sva) of [Attribute Locally Valid (§3.2.4)](011-3-2-4-attribute-declaration-validation-rules.md#cvc-attribute) applies with
respect to an attribute information item, in the [·post-schema-validation infoset·](004-2-1-overview-of-xml-schema.md#key-psvi) the attribute
information item has a property:

PSVI Contributions
for attribute information items

<a id="a-schema_normalized_value"></a>

[schema normalized value]
:   The [·normalized value·](008-3-schema-component-details.md#key-nv) of the item as [·validated·](004-2-1-overview-of-xml-schema.md#key-vn).

Furthermore, the item has one of the following alternative sets of properties:Either

PSVI Contributions
for attribute information items

<a id="a-type_definition"></a>

[type definition]
:   An [·item isomorphic·](051-3-15-5-schema-information-set-contributions.md#key-iso) to the relevant attribute declaration's
    [{type definition}](009-3-2-attribute-declarations.md#a-simple_type_definition) component.

<a id="a-member_type_definition"></a>

[member type definition]
:   If
    and only if that type definition has [{variety}](045-3-14-simple-type-definitions.md#variety) *union*, then an
    [·item isomorphic·](051-3-15-5-schema-information-set-contributions.md#key-iso) to that member of its [{member type definitions}](045-3-14-simple-type-definitions.md#st-member_type_definitions) which actually [·validated·](004-2-1-overview-of-xml-schema.md#key-vn) the attribute item's [[normalized value]](https://www.w3.org/TR/xml-infoset/#infoitem.attribute).

or

PSVI Contributions
for attribute information items

<a id="a-type_definition_type"></a>

[type definition type]
:   *simple*.

<a id="a-type_definition_namespace"></a>

[type definition namespace]
:   The [{target namespace}](045-3-14-simple-type-definitions.md#st-target_namespace) of the [·type definition·](004-2-1-overview-of-xml-schema.md#key-typeDefn).

<a id="a-type_definition_anonymous"></a>

[type definition anonymous]
:   *true* if the [{name}](045-3-14-simple-type-definitions.md#st-name) of the [·type definition·](004-2-1-overview-of-xml-schema.md#key-typeDefn) is [·absent·](008-3-schema-component-details.md#key-null), otherwise *false*.

<a id="a-type_definition_name"></a>

[type definition name]
:   The [{name}](045-3-14-simple-type-definitions.md#st-name) of the [·type definition·](004-2-1-overview-of-xml-schema.md#key-typeDefn), if it is not [·absent·](008-3-schema-component-details.md#key-null). If it is
    [·absent·](008-3-schema-component-details.md#key-null), schema processors may, but need not,
    provide a value unique to the definition.

If the [·type definition·](004-2-1-overview-of-xml-schema.md#key-typeDefn) has [{variety}](045-3-14-simple-type-definitions.md#variety) *union*, then calling

<a id="a-key-amt"></a>

[Definition:] that
member of the [{member type definitions}](045-3-14-simple-type-definitions.md#st-member_type_definitions) which actually
[·validated·](004-2-1-overview-of-xml-schema.md#key-vn) the attribute item's [·normalized value·](008-3-schema-component-details.md#key-nv) the
**actual member type definition**, there are three additional properties:

PSVI Contributions
for attribute information items

<a id="a-member_type_definition_namespace"></a>

[member type definition namespace]
:   The [{target namespace}](045-3-14-simple-type-definitions.md#st-target_namespace) of the [·actual
    member type definition·](011-3-2-4-attribute-declaration-validation-rules.md#a-key-amt).

<a id="a-member_type_definition_anonymous"></a>

[member type definition anonymous]
:   *true* if the [{name}](045-3-14-simple-type-definitions.md#st-name) of the [·actual member type definition·](011-3-2-4-attribute-declaration-validation-rules.md#a-key-amt) is [·absent·](008-3-schema-component-details.md#key-null), otherwise *false*.

<a id="a-member_type_definition_name"></a>

[member type definition name]
:   The [{name}](045-3-14-simple-type-definitions.md#st-name) of the [·actual member type definition·](011-3-2-4-attribute-declaration-validation-rules.md#a-key-amt), if it is not [·absent·](008-3-schema-component-details.md#key-null). If it is
    [·absent·](008-3-schema-component-details.md#key-null), schema processors may, but need not,
    provide a value unique to the definition.

The first ([·item isomorphic·](051-3-15-5-schema-information-set-contributions.md#key-iso)) alternative above is provided for applications such as query
processors which need access to the full range of details about an item's
[·assessment·](004-2-1-overview-of-xml-schema.md#key-va), for example the type hierarchy; the second, for lighter-weight
processors for whom representing the significant parts of the type hierarchy as
information items might be a significant burden.Also, if the declaration has a [{value constraint}](009-3-2-attribute-declarations.md#a-value_constraint), the
item has a property:

PSVI Contributions
for attribute information items

