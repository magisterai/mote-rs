# Continuation 64

Source: [XML Schema Part 1: Structures](https://www.w3.org/TR/2004/REC-xmlschema-1-20041028/) · 2nd edition, 2004-10-28.

[Document index](../README.md) · [Topic guide](../../../../README.md) · [Publisher](https://www.w3.org/TR/2004/REC-xmlschema-1-20041028/).

HTML normalized to Markdown. Complex tables and mathematical markup retain HTML; illustrations are stored locally.

[Previous](063-d-required-information-set-items-and-properties-normative.md) · [Next](065-g-dtd-for-schemas-non-normative.md)

[Schema Component Constraint](006-2-3-constraints-and-validation-rules.md#gloss-cos)
:   Constraints on the schema components themselves, i.e.
    conditions components must satisfy to be components at all. Located in the
    sixth sub-section of the per-component sections of [Schema Component Details (§3)](008-3-schema-component-details.md#components)
    and tabulated in [Schema Component Constraints (§C.4)](062-c-4-schema-component-constraints.md#outcome-cos)

[schema document](008-3-schema-component-details.md#key-schemaDoc)
:   A document in
    this form (i.e. a [<schema>](049-3-15-schemas-as-a-whole.md#element-schema) element information item) is a **schema document**

[Schema Information Set Contribution](006-2-3-constraints-and-validation-rules.md#gloss-sic)
:   Augmentations to [·post-schema-validation infoset·](004-2-1-overview-of-xml-schema.md#key-psvi)s
    expressed by schema components, which follow
    as a consequence of [·validation·](004-2-1-overview-of-xml-schema.md#key-vn) and/or [·assessment·](004-2-1-overview-of-xml-schema.md#key-va).
    Located in the
    fifth sub-section of the per-component sections of [Schema Component Details (§3)](008-3-schema-component-details.md#components)
    and tabulated in [Contributions to the post-schema-validation infoset (§C.2)](060-b-references-normative.md#PSVI_contributions)

[Schema Representation Constraint](006-2-3-constraints-and-validation-rules.md#gloss-src)
:   Constraints on the
    representation of schema components in XML beyond those which are expressed
    in [Schema for Schemas (normative) (§A)](058-5-3-missing-sub-components.md#normative-schemaSchema). Located in the
    third sub-section of the per-component sections of [Schema Component Details (§3)](008-3-schema-component-details.md#components)
    and tabulated in [Schema Representation Constraints (§C.3)](061-c-3-schema-representation-constraints.md#outcome-src)

[simple ur-type definition](004-2-1-overview-of-xml-schema.md#key-simpleUrType)
:   the **simple
    ur-type definition**, a special restriction of the
    [·ur-type
    definition·](004-2-1-overview-of-xml-schema.md#key-urType), whose name is **anySimpleType** in the XML Schema namespace

[substitution group](019-continuation-19.md#key-eq)
:   Every element
    declaration (call this **HEAD**)
    in the [{element declarations}](049-3-15-schemas-as-a-whole.md#element_declarations) of a schema defines a
    **substitution group**, a subset of those [{element declarations}](049-3-15-schemas-as-a-whole.md#element_declarations), as follows:

[symbol space](007-2-5-names-and-symbol-spaces.md#key-symbolSpace)
:   this specification introduces the term
    **symbol space** to denote a
    collection of names, each of which is unique with respect to the others

[target namespace](004-2-1-overview-of-xml-schema.md#key-targetNS)
:   Several kinds
    of component have a **target namespace**, which is either
    [·absent·](008-3-schema-component-details.md#key-null) or a namespace name, also as
    defined by [[XML-Namespaces]](060-b-references-normative.md#ref-xml-namespaces)

[type definition](004-2-1-overview-of-xml-schema.md#key-typeDefn)
:   This specification uses
    the phrase **type definition** in cases where no distinction
    need be made between simple and complex types

[Type Definition Hierarchy](004-2-1-overview-of-xml-schema.md#key-typeDefinitionHierarchy)
:   Except for a distinguished [·ur-type definition·](004-2-1-overview-of-xml-schema.md#key-urType), every [·type definition·](004-2-1-overview-of-xml-schema.md#key-typeDefn) is, by construction, either a
    [·restriction·](004-2-1-overview-of-xml-schema.md#key-typeRestriction) or an [·extension·](004-2-1-overview-of-xml-schema.md#key-typeExtension) of some other type definition. The graph of these relationships forms a tree known as the **Type Definition Hierarchy**

[ur-type definition](004-2-1-overview-of-xml-schema.md#key-urType)
:   A distinguished complex
    type definition, the **ur-type
    definition**, whose
    name is *anyType* in the XML Schema namespace, is present in each [·XML Schema·](004-2-1-overview-of-xml-schema.md#key-schema), serving as the root of the type
    definition hierarchy for that schema

[valid](004-2-1-overview-of-xml-schema.md#key-vn)
:   the
    word **valid** and its derivatives are used to refer to
    clause [1](004-2-1-overview-of-xml-schema.md#c-lsv) above, the determination of local
    schema-validity

[valid extension](028-continuation-28.md#cd-ct-extension)
:   If this
    constraint [Derivation Valid (Extension) (§3.4.6)](027-3-4-5-complex-type-definition-information-set-contributions.md#cos-ct-extends) holds of a complex type definition, it is a **valid
    extension** of its [{base type definition}](020-3-4-complex-type-definitions.md#ct-base_type_definition)

[valid restriction](028-continuation-28.md#cd-ct-restriction)
:   If this
    constraint [Derivation Valid (Restriction, Complex) (§3.4.6)](028-continuation-28.md#derivation-ok-restriction) holds of a complex type definition, it is a **valid
    restriction** of its [{base type definition}](020-3-4-complex-type-definitions.md#ct-base_type_definition)

[valid restriction](048-3-14-7-built-in-simple-type-definition.md#cd-st-restriction)
:   If this
    constraint [Derivation Valid (Restriction, Simple) (§3.14.6)](047-3-14-3-constraints-on-xml-representations-of-simple-type-definitions.md#cos-st-restricts) holds of a simple type definition, it is a **valid
    restriction** of its [·base type definition·](045-3-14-simple-type-definitions.md#st-base_type_definition)

[validation root](057-5-schemas-and-schema-validity-assessment.md#key-vr)
:   This item, that is the element information item at which [·assessment·](004-2-1-overview-of-xml-schema.md#key-va) began, is called the **validation root**

[Validation Rules](006-2-3-constraints-and-validation-rules.md#gloss-cvc)
:   Contributions to [·validation·](004-2-1-overview-of-xml-schema.md#key-vn) associated
    with schema components. Located in the
    fourth sub-section of the per-component sections of [Schema Component Details (§3)](008-3-schema-component-details.md#components)
    and tabulated in [Validation Rules (§C.1)](060-b-references-normative.md#validation_failures)

[XML Schema](004-2-1-overview-of-xml-schema.md#key-schema)
:   An **XML Schema** is a
    set of [·schema components·](004-2-1-overview-of-xml-schema.md#key-component)

