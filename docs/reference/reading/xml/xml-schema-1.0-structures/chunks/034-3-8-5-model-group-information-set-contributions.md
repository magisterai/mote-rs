# 3.8.5 Model Group Information Set Contributions

Source: [XML Schema Part 1: Structures](https://www.w3.org/TR/2004/REC-xmlschema-1-20041028/) · 2nd edition, 2004-10-28.

[Document index](../README.md) · [Topic guide](../../../../README.md) · [Publisher](https://www.w3.org/TR/2004/REC-xmlschema-1-20041028/).

HTML normalized to Markdown. Complex tables and mathematical markup retain HTML; illustrations are stored locally.

[Previous](033-3-7-6-constraints-on-model-group-definition-schema-components.md) · [Next](035-3-9-particles.md)

<a id="key-partition"></a>

[Definition:]Define a
**partition** of a sequence as a sequence of sub-sequences, some or
all of which may be empty, such that concatenating all the sub-sequences yields
the original sequence.For a sequence (possibly empty) of element information items to be
locally [·valid·](004-2-1-overview-of-xml-schema.md#key-vn) with respect to
a model group
the appropriate **case** among the following
must be true:

1 **If** the [{compositor}](033-3-7-6-constraints-on-model-group-definition-schema-components.md#compositor) is *sequence*, **then** there must be a
[·partition·](034-3-8-5-model-group-information-set-contributions.md#key-partition) of the sequence into `n` sub-sequences where `n` is the length of [{particles}](033-3-7-6-constraints-on-model-group-definition-schema-components.md#particles) such that each of the sub-sequences in order is [·valid·](004-2-1-overview-of-xml-schema.md#key-vn)
with respect to the corresponding particle in the [{particles}](033-3-7-6-constraints-on-model-group-definition-schema-components.md#particles) as defined in [Element Sequence Locally Valid (Particle) (§3.9.4)](035-3-9-particles.md#cvc-particle).

2 **If** the [{compositor}](033-3-7-6-constraints-on-model-group-definition-schema-components.md#compositor) is *choice*, **then** there must be a
particle among the [{particles}](033-3-7-6-constraints-on-model-group-definition-schema-components.md#particles) such that the sequence is
[·valid·](004-2-1-overview-of-xml-schema.md#key-vn) with respect to that particle as defined in [Element Sequence Locally Valid (Particle) (§3.9.4)](035-3-9-particles.md#cvc-particle).

3 **If** the [{compositor}](033-3-7-6-constraints-on-model-group-definition-schema-components.md#compositor) is *all*, **then** there must be a
[·partition·](034-3-8-5-model-group-information-set-contributions.md#key-partition) of the sequence into `n` sub-sequences where `n` is the length of [{particles}](033-3-7-6-constraints-on-model-group-definition-schema-components.md#particles) such that there is a one-to-one mapping between the sub-sequences and the [{particles}](033-3-7-6-constraints-on-model-group-definition-schema-components.md#particles) where each sub-sequence is [·valid·](004-2-1-overview-of-xml-schema.md#key-vn) with respect to the corresponding particle as defined in [Element Sequence Locally Valid (Particle) (§3.9.4)](035-3-9-particles.md#cvc-particle).

Nothing in the above should be understood as ruling out groups whose
[{particles}](033-3-7-6-constraints-on-model-group-definition-schema-components.md#particles) is empty: although no sequence can be [·valid·](004-2-1-overview-of-xml-schema.md#key-vn)
with respect to such a group whose [{compositor}](033-3-7-6-constraints-on-model-group-definition-schema-components.md#compositor) is
*choice*, the empty sequence *is* [·valid·](004-2-1-overview-of-xml-schema.md#key-vn) with respect
to empty groups whose [{compositor}](033-3-7-6-constraints-on-model-group-definition-schema-components.md#compositor) is *sequence* or *all*.

**Note:**The above definition is implicitly non-deterministic, and should not be
taken as a recipé for implementations. Note in particular that when
[{compositor}](033-3-7-6-constraints-on-model-group-definition-schema-components.md#compositor) is *all*, particles is restricted to a list
of local and top-level element declarations (see [Constraints on Model Group Schema Components (§3.8.6)](034-3-8-5-model-group-information-set-contributions.md#coss-modelGroup)). A much simpler implementation is possible than would arise from a literal interpretation of the definition above; informally, the content is [·valid·](004-2-1-overview-of-xml-schema.md#key-vn) when each declared element occurs exactly once (or at most once, if [{min occurs}](035-3-9-particles.md#p-min_occurs) is `0`), and each is [·valid·](004-2-1-overview-of-xml-schema.md#key-vn) with respect to its corresponding declaration. The elements can occur in arbitrary order.

<a id="d0e10582"></a>

#### 3.8.5 Model Group Information Set Contributions

None as such.

<a id="coss-modelGroup"></a>

#### 3.8.6 Constraints on Model Group Schema Components

All model groups (see [Model Groups (§3.8)](033-3-7-6-constraints-on-model-group-definition-schema-components.md#Model_Groups)) must satisfy the following constraints.

<a id="mg-props-correct"></a>

**Schema Component Constraint: Model Group Correct**  

**All** of the following must be true:

1 The values of the properties of a model group must be as described in
the property tableau in
[The Model Group Schema Component (§3.8.1)](033-3-7-6-constraints-on-model-group-definition-schema-components.md#Model_Group_details), modulo the impact of [Missing Sub-components (§5.3)](058-5-3-missing-sub-components.md#conformance-missing).

2 Circular groups are disallowed. That is, within the [{particles}](033-3-7-6-constraints-on-model-group-definition-schema-components.md#particles) of a group there must not be at any
depth a particle whose [{term}](035-3-9-particles.md#term) is the
group itself.

<a id="cos-all-limited"></a>

**Schema Component Constraint: All Group Limited**  

When a model group has [{compositor}](033-3-7-6-constraints-on-model-group-definition-schema-components.md#compositor) *all*, then
**all** of the following must be true:

1 It appears only as the value of one or both of the following properties:

1.1 the [{model group}](032-3-7-model-group-definitions.md#model_group) property of a model group definition.

1.2 the
[{term}](035-3-9-particles.md#term) property of a particle with [{max occurs}](035-3-9-particles.md#p-max_occurs)`=1`which is part of a pair which constitutes the [{content type}](020-3-4-complex-type-definitions.md#content_type) of a
complex type definition.

2 The [{max occurs}](035-3-9-particles.md#p-max_occurs) of all the particles in the
[{particles}](033-3-7-6-constraints-on-model-group-definition-schema-components.md#particles) of the group must be `0` or `1`.

<a id="cos-element-consistent"></a>

**Schema Component Constraint: Element Declarations Consistent**  

If the [{particles}](033-3-7-6-constraints-on-model-group-definition-schema-components.md#particles) contains, either directly, indirectly
(that is, within the [{particles}](033-3-7-6-constraints-on-model-group-definition-schema-components.md#particles) of a contained model group,
recursively) or [·implicitly·](034-3-8-5-model-group-information-set-contributions.md#key-impl-cont) two or more
element declaration particles with the same [{name}](013-3-3-element-declarations.md#e-name) and
[{target namespace}](013-3-3-element-declarations.md#e-target_namespace), then all their type definitions must be
the same top-level definition, that is,
**all** of the following must be true:

1 all their
[{type definition}](013-3-3-element-declarations.md#type_definition)s must have a non-[·absent·](008-3-schema-component-details.md#key-null) [{name}](020-3-4-complex-type-definitions.md#ct-name).

2 all their
[{type definition}](013-3-3-element-declarations.md#type_definition)s must have the same [{name}](020-3-4-complex-type-definitions.md#ct-name).

3 all their
[{type definition}](013-3-3-element-declarations.md#type_definition)s must have the same [{target namespace}](020-3-4-complex-type-definitions.md#ct-target_namespace).

<a id="key-impl-cont"></a>

[Definition:]A list
of particles **implicitly contains** an element declaration if a
member of the list contains that
element declaration in its [·substitution group·](019-continuation-19.md#key-eq).

<a id="cos-nonambig"></a>

**Schema Component Constraint: Unique Particle Attribution**  

A content model must be formed such that
during [·validation·](004-2-1-overview-of-xml-schema.md#key-vn) of an element information
item sequence, the particle component
contained directly, indirectly or [·implicitly·](034-3-8-5-model-group-information-set-contributions.md#key-impl-cont) therein with which to attempt to [·validate·](004-2-1-overview-of-xml-schema.md#key-vn) each item in the sequence in turn can be uniquely determined without examining the content or attributes of that item, and without any information about the items in the remainder of the sequence.

**Note:**This constraint reconstructs for XML Schema the equivalent constraints of
[[XML 1.0 (Second Edition)]](060-b-references-normative.md#ref-xml) and SGML. Given the presence of element substitution groups and wildcards, the concise expression of this constraint is difficult,
see [Analysis of the Unique Particle Attribution Constraint (non-normative) (§H)](067-h-analysis-of-the-unique-particle-attribution-constraint-non-normative.md#non-ambig) for further discussion.Since this constraint is expressed at the component level, it
applies to content models whose origins (e.g. via type derivation and
references to named model groups) are no longer evident. So particles at
different points in the content model are always distinct from one another,
even if they originated from the same named model group.

**Note:**Because locally-scoped element declarations may or may not have a
[{target namespace}](013-3-3-element-declarations.md#e-target_namespace), the scope of declarations is
*not* relevant to enforcing either of the two preceding constraints.

The following constraints define relations appealed to elsewhere in this specification.

<a id="cos-seq-range"></a>

**Schema Component Constraint: Effective Total Range (all and sequence)**  

The effective total range of a particle whose [{term}](035-3-9-particles.md#term) is a group whose [{compositor}](033-3-7-6-constraints-on-model-group-definition-schema-components.md#compositor) is
*all* or *sequence* is a pair of minimum and maximum, as follows:

minimum
:   The product of the particle's [{min occurs}](035-3-9-particles.md#p-min_occurs) and the
    sum of the [{min occurs}](035-3-9-particles.md#p-min_occurs) of every wildcard or element
    declaration particle in the group's [{particles}](033-3-7-6-constraints-on-model-group-definition-schema-components.md#particles) and the minimum
    part of the effective total range of each of the group particles in the group's [{particles}](033-3-7-6-constraints-on-model-group-definition-schema-components.md#particles) (or `0` if there are no [{particles}](033-3-7-6-constraints-on-model-group-definition-schema-components.md#particles)).

maximum
:   *unbounded* if the [{max occurs}](035-3-9-particles.md#p-max_occurs) of any wildcard or element
    declaration particle in the group's [{particles}](033-3-7-6-constraints-on-model-group-definition-schema-components.md#particles) or the maximum
    part of the effective total range of any of the group particles in the group's
    [{particles}](033-3-7-6-constraints-on-model-group-definition-schema-components.md#particles) is *unbounded*, or if any of those is non-zero
    and the [{max occurs}](035-3-9-particles.md#p-max_occurs) of the particle itself is *unbounded*,
    otherwise the product of the particle's [{max occurs}](035-3-9-particles.md#p-max_occurs) and the
    sum of the [{max occurs}](035-3-9-particles.md#p-max_occurs) of every wildcard or element
    declaration particle in the group's [{particles}](033-3-7-6-constraints-on-model-group-definition-schema-components.md#particles) and the maximum
    part of the effective total range of each of the group particles in the group's [{particles}](033-3-7-6-constraints-on-model-group-definition-schema-components.md#particles) (or `0` if there are no [{particles}](033-3-7-6-constraints-on-model-group-definition-schema-components.md#particles)).

<a id="cos-choice-range"></a>

**Schema Component Constraint: Effective Total Range (choice)**  

The effective total range of a particle whose [{term}](035-3-9-particles.md#term) is a group whose [{compositor}](033-3-7-6-constraints-on-model-group-definition-schema-components.md#compositor) is
*choice* is a pair of minimum and maximum, as follows:

minimum
:   The product of the particle's [{min occurs}](035-3-9-particles.md#p-min_occurs) and the
    minimum of the [{min occurs}](035-3-9-particles.md#p-min_occurs) of every wildcard or element
    declaration particle in the group's [{particles}](033-3-7-6-constraints-on-model-group-definition-schema-components.md#particles) and the minimum
    part of the effective total range of each of the group particles in the group's [{particles}](033-3-7-6-constraints-on-model-group-definition-schema-components.md#particles) (or `0` if there are no [{particles}](033-3-7-6-constraints-on-model-group-definition-schema-components.md#particles)).

