# 3.4.5 Complex Type Definition Information Set Contributions

Source: [XML Schema Part 1: Structures](https://www.w3.org/TR/2004/REC-xmlschema-1-20041028/) · 2nd edition, 2004-10-28.

[Document index](../README.md) · [Topic guide](../../../../README.md) · [Publisher](https://www.w3.org/TR/2004/REC-xmlschema-1-20041028/).

HTML normalized to Markdown. Complex tables and mathematical markup retain HTML; illustrations are stored locally.

[Previous](026-3-4-3-constraints-on-xml-representations-of-complex-type-definitions.md) · [Next](028-continuation-28.md)

3.2.2 The
attribute information item must be [·valid·](004-2-1-overview-of-xml-schema.md#key-vn) with respect to it as defined in [Item Valid (Wildcard) (§3.10.4)](039-3-10-4-wildcard-validation-rules.md#cvc-wildcard).

4 The [{attribute declaration}](030-3-5-attributeuses.md#attribute) of each attribute use in the [{attribute uses}](020-3-4-complex-type-definitions.md#ct-attribute_declarations) whose
[{required}](030-3-5-attributeuses.md#required) is *true* matches one of the attribute information items in the element information item's [[attributes]](https://www.w3.org/TR/xml-infoset/#infoitem.element) as per clause [3.1](026-3-4-3-constraints-on-xml-representations-of-complex-type-definitions.md#c-ctma) above.

5 Let 

<a id="key-ida"></a>

[Definition:]the **wild
IDs** be the set of all
attribute information item to which clause [3.2](026-3-4-3-constraints-on-xml-representations-of-complex-type-definitions.md#c-avaw) applied and whose [·validation·](004-2-1-overview-of-xml-schema.md#key-vn) resulted in
a [·context-determined declaration·](011-3-2-4-attribute-declaration-validation-rules.md#key-dd) of
*mustFind* or no [·context-determined
declaration·](011-3-2-4-attribute-declaration-validation-rules.md#key-dd) at all, and whose [[local name]](https://www.w3.org/TR/xml-infoset/#infoitem.attribute) and [[namespace name]](https://www.w3.org/TR/xml-infoset/#infoitem.attribute) resolve (as defined by [QName resolution (Instance) (§3.15.4)](050-3-15-2-1-references-to-schema-components.md#cvc-resolve-instance)) to an attribute declaration whose [{type definition}](009-3-2-attribute-declarations.md#a-simple_type_definition) is or is derived from [ID](https://www.w3.org/TR/2004/REC-xmlschema-2-20041028/datatypes.html#ID). Then
**all** of the following must be true:

5.1 There must be no more than one item in [·wild IDs·](027-3-4-5-complex-type-definition-information-set-contributions.md#key-ida).

5.2 If [·wild IDs·](027-3-4-5-complex-type-definition-information-set-contributions.md#key-ida) is non-empty, there must not be any attribute uses among the [{attribute uses}](020-3-4-complex-type-definitions.md#ct-attribute_declarations) whose [{attribute declaration}](030-3-5-attributeuses.md#attribute)'s [{type definition}](009-3-2-attribute-declarations.md#a-simple_type_definition) is or is derived from [ID](https://www.w3.org/TR/2004/REC-xmlschema-2-20041028/datatypes.html#ID).

**Note:**This clause serves to ensure that even via attribute
wildcards no element has more than one attribute of type ID, and that even when
an element legitimately lacks a declared attribute of type ID, a
wildcard-validated attribute must not supply it. That is, if an element has a
type whose attribute declarations include one of type ID, it either has that
attribute or no attribute of type ID.

**Note:**When an [{attribute wildcard}](020-3-4-complex-type-definitions.md#ct-attribute_wildcard) is present, this does
*not* introduce any ambiguity with respect to how attribute
information items for
which an attribute use is present amongst the [{attribute uses}](020-3-4-complex-type-definitions.md#ct-attribute_declarations) whose name and target namespace match are [·assessed·](004-2-1-overview-of-xml-schema.md#key-va). In such cases the attribute use *always* takes precedence, and the [·assessment·](004-2-1-overview-of-xml-schema.md#key-va) of such items stands or falls entirely on the basis of the attribute use and its [{attribute declaration}](030-3-5-attributeuses.md#attribute). This follows from the details of clause [3](026-3-4-3-constraints-on-xml-representations-of-complex-type-definitions.md#c-aam).

<a id="d0e8328"></a>

#### 3.4.5 Complex Type Definition Information Set Contributions

<a id="sic-attrDefault"></a>

**Schema Information Set Contribution: Attribute Default Value**  

For each attribute use in the [{attribute uses}](020-3-4-complex-type-definitions.md#ct-attribute_declarations) whose
[{required}](030-3-5-attributeuses.md#required) is *false* and whose [{value constraint}](030-3-5-attributeuses.md#au-value_constraint) is not [·absent·](008-3-schema-component-details.md#key-null) but whose [{attribute declaration}](030-3-5-attributeuses.md#attribute) does not match one of the attribute information items in the element information item's [[attributes]](https://www.w3.org/TR/xml-infoset/#infoitem.element) as per clause [3.1](026-3-4-3-constraints-on-xml-representations-of-complex-type-definitions.md#c-ctma) of [Element Locally Valid (Complex Type) (§3.4.4)](026-3-4-3-constraints-on-xml-representations-of-complex-type-definitions.md#cvc-complex-type) above, the [·post-schema-validation infoset·](004-2-1-overview-of-xml-schema.md#key-psvi) has an attribute information item whose properties are as below added to the
[[attributes]](https://www.w3.org/TR/xml-infoset/#infoitem.element) of the element information item.

[[local name]](https://www.w3.org/TR/xml-infoset/#infoitem.attribute)
:   The [{attribute declaration}](030-3-5-attributeuses.md#attribute)'s [{name}](009-3-2-attribute-declarations.md#a-name).

[[namespace name]](https://www.w3.org/TR/xml-infoset/#infoitem.attribute)
:   The [{attribute declaration}](030-3-5-attributeuses.md#attribute)'s [{target namespace}](009-3-2-attribute-declarations.md#a-target_namespace).

[[schema normalized value]](011-3-2-4-attribute-declaration-validation-rules.md#a-schema_normalized_value)
:   The [canonical lexical representation](https://www.w3.org/TR/2004/REC-xmlschema-2-20041028/datatypes.html#dt-canonical-representation) of the [·effective value constraint·](028-continuation-28.md#key-evc) value.

[[schema default]](012-3-2-6-constraints-on-attribute-declaration-schema-components.md#a-schema_default)
:   The [canonical lexical representation](https://www.w3.org/TR/2004/REC-xmlschema-2-20041028/datatypes.html#dt-canonical-representation) of the [·effective value constraint·](028-continuation-28.md#key-evc) value.

[[validation context]](011-3-2-4-attribute-declaration-validation-rules.md#a-validation_context)
:   The nearest ancestor element information item with a
    [[schema information]](051-3-15-5-schema-information-set-contributions.md#e-schema_information) property.

[[validity]](011-3-2-4-attribute-declaration-validation-rules.md#a-validity)
:   *valid*.

[[validation attempted]](011-3-2-4-attribute-declaration-validation-rules.md#a-validation_attempted)
:   *full*.

[[schema specified]](011-3-2-4-attribute-declaration-validation-rules.md#a-schema_specified)
:   *schema*.

The added items should also either have [[type definition]](011-3-2-4-attribute-declaration-validation-rules.md#a-type_definition) (and [[member type definition]](011-3-2-4-attribute-declaration-validation-rules.md#a-member_type_definition) if appropriate) properties, or their lighter-weight alternatives, as specified in [Attribute Validated by Type (§3.2.5)](011-3-2-4-attribute-declaration-validation-rules.md#sic-attrType).

<a id="coss-ct"></a>

#### 3.4.6 Constraints on Complex Type Definition Schema Components

All complex type definitions (see [Complex Type Definitions (§3.4)](020-3-4-complex-type-definitions.md#Complex_Type_Definitions)) must satisfy the following constraints.

<a id="ct-props-correct"></a>

**Schema Component Constraint: Complex Type Definition Properties Correct**  

**All** of the following must be true:

1 The values of the properties of a complex type definition must be as described in
the property tableau in
[The Complex Type Definition Schema Component (§3.4.1)](020-3-4-complex-type-definitions.md#Complex_Type_Definition_details), modulo the impact of [Missing Sub-components (§5.3)](058-5-3-missing-sub-components.md#conformance-missing).

2 If the [{base type definition}](020-3-4-complex-type-definitions.md#ct-base_type_definition) is a simple type
definition, the [{derivation method}](020-3-4-complex-type-definitions.md#derivation_method) must be *extension*.

3 Circular definitions are disallowed, except for the [·ur-type definition·](029-3-4-7-built-in-complex-type-definition.md#ur-type-itself). That is, it must be possible to reach the [·ur-type definition·](029-3-4-7-built-in-complex-type-definition.md#ur-type-itself) by repeatedly following the [{base type definition}](020-3-4-complex-type-definitions.md#ct-base_type_definition).

4 Two distinct attribute declarations in the [{attribute uses}](020-3-4-complex-type-definitions.md#ct-attribute_declarations) must not have identical [{name}](009-3-2-attribute-declarations.md#a-name)s and [{target namespace}](009-3-2-attribute-declarations.md#a-target_namespace)s.

5 Two distinct attribute declarations in the [{attribute uses}](020-3-4-complex-type-definitions.md#ct-attribute_declarations)
must not have [{type definition}](009-3-2-attribute-declarations.md#a-simple_type_definition)s which are or are derived from [ID](https://www.w3.org/TR/2004/REC-xmlschema-2-20041028/datatypes.html#ID).

<a id="cos-ct-extends"></a>

**Schema Component Constraint: Derivation Valid (Extension)**  

If the [{derivation method}](020-3-4-complex-type-definitions.md#derivation_method) is *extension*,
the appropriate **case** among the following
must be true:

1 **If** the [{base type definition}](020-3-4-complex-type-definitions.md#ct-base_type_definition) is a complex type
definition, **then** 
**all** of the following must be true:

1.1 The [{final}](020-3-4-complex-type-definitions.md#ct-final) of the [{base type definition}](020-3-4-complex-type-definitions.md#ct-base_type_definition) must not contain *extension*.

1.2

<a id="c-cte"></a>

Its [{attribute uses}](020-3-4-complex-type-definitions.md#ct-attribute_declarations) must be a subset
of the [{attribute uses}](020-3-4-complex-type-definitions.md#ct-attribute_declarations) of the complex type
definition itself, that is, for every attribute use in the
[{attribute uses}](020-3-4-complex-type-definitions.md#ct-attribute_declarations) of the
[{base type definition}](020-3-4-complex-type-definitions.md#ct-base_type_definition), there must be an attribute use in the [{attribute uses}](020-3-4-complex-type-definitions.md#ct-attribute_declarations) of the complex
type definition itself whose [{attribute declaration}](030-3-5-attributeuses.md#attribute) has the same [{name}](009-3-2-attribute-declarations.md#a-name),
[{target namespace}](009-3-2-attribute-declarations.md#a-target_namespace) and
[{type definition}](009-3-2-attribute-declarations.md#a-simple_type_definition) as its attribute declaration.

1.3 If it has an [{attribute wildcard}](020-3-4-complex-type-definitions.md#ct-attribute_wildcard), the complex
type definition must also have one, and the base type definition's [{attribute wildcard}](020-3-4-complex-type-definitions.md#ct-attribute_wildcard)'s [{namespace constraint}](038-3-10-wildcards.md#namespace_constraint) must be a subset of the complex type definition's [{attribute wildcard}](020-3-4-complex-type-definitions.md#ct-attribute_wildcard)'s [{namespace constraint}](038-3-10-wildcards.md#namespace_constraint), as defined by [Wildcard Subset (§3.10.6)](039-3-10-4-wildcard-validation-rules.md#cos-ns-subset).

1.4
**One** of the following must be true:

1.4.1 The [{content type}](020-3-4-complex-type-definitions.md#content_type) of
the [{base type definition}](020-3-4-complex-type-definitions.md#ct-base_type_definition) and the
[{content type}](020-3-4-complex-type-definitions.md#content_type) of the complex type definition itself must be the
same simple type definition.

1.4.2 The
[{content type}](020-3-4-complex-type-definitions.md#content_type) of both the [{base type definition}](020-3-4-complex-type-definitions.md#ct-base_type_definition) and the complex type definition itself
must be *empty*.

1.4.3 **All** of the following must be true:

1.4.3.1 The
[{content type}](020-3-4-complex-type-definitions.md#content_type) of the complex type definition itself must
specify a particle.

1.4.3.2 **One** of the following must be true:

1.4.3.2.1 The
[{content type}](020-3-4-complex-type-definitions.md#content_type) of the [{base type definition}](020-3-4-complex-type-definitions.md#ct-base_type_definition)
must be *empty*.

1.4.3.2.2
**All** of the following must be true:

1.4.3.2.2.1 Both
[{content type}](020-3-4-complex-type-definitions.md#content_type)s must be *mixed* or both must be
*element-only*.

1.4.3.2.2.2 The particle of the complex type
definition must be a [·valid
extension·](036-3-9-5-particle-information-set-contributions.md#cd-model-extension) of the [{base type definition}](020-3-4-complex-type-definitions.md#ct-base_type_definition)'s particle,
as defined in [Particle Valid (Extension) (§3.9.6)](036-3-9-5-particle-information-set-contributions.md#cos-particle-extend).

