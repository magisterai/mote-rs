# Continuation 37

Source: [XML Schema Part 1: Structures](https://www.w3.org/TR/2004/REC-xmlschema-1-20041028/) · 2nd edition, 2004-10-28.

[Document index](../README.md) · [Topic guide](../../../../README.md) · [Publisher](https://www.w3.org/TR/2004/REC-xmlschema-1-20041028/).

HTML normalized to Markdown. Complex tables and mathematical markup retain HTML; illustrations are stored locally.

[Previous](036-3-9-5-particle-information-set-contributions.md) · [Next](038-3-10-wildcards.md)

**Note:**The above constraint on [{type definition}](013-3-3-element-declarations.md#type_definition) means that in
deriving a type by restriction, any contained type definitions must themselves be
explicitly derived by restriction from the corresponding type definitions in the
base definition, or be one of the member types of a
corresponding union..

<a id="rcase-NSCompat"></a>

**Schema Component Constraint: Particle Derivation OK (Elt:Any -- NSCompat)**  

For an element declaration particle to be a [·valid restriction·](036-3-9-5-particle-information-set-contributions.md#cd-model-restriction) of a wildcard particle
**all** of the following must be true:

1 The element declaration's [{target namespace}](013-3-3-element-declarations.md#e-target_namespace) is
[·valid·](004-2-1-overview-of-xml-schema.md#key-vn) with respect to the wildcard's [{namespace constraint}](038-3-10-wildcards.md#namespace_constraint) as
defined by [Wildcard allows Namespace Name (§3.10.4)](039-3-10-4-wildcard-validation-rules.md#cvc-wildcard-namespace).

2 **R**'s occurrence range is a valid
restriction of **B**'s occurrence range as defined by [Occurrence Range OK (§3.9.6)](036-3-9-5-particle-information-set-contributions.md#range-ok).

<a id="rcase-RecurseAsIfGroup"></a>

**Schema Component Constraint: Particle Derivation OK (Elt:All/Choice/Sequence -- RecurseAsIfGroup)**  

For an element declaration particle to be a [·valid restriction·](036-3-9-5-particle-information-set-contributions.md#cd-model-restriction) of a group particle (*all*, *choice* or *sequence*)
a group particle of the variety corresponding to **B**'s, with
[{min occurs}](035-3-9-particles.md#p-min_occurs) and [{max occurs}](035-3-9-particles.md#p-max_occurs) of `1` and with [{particles}](033-3-7-6-constraints-on-model-group-definition-schema-components.md#particles) consisting of a single particle
the same as the element declaration must be a [·valid restriction·](036-3-9-5-particle-information-set-contributions.md#cd-model-restriction) of the group as defined by [Particle Derivation OK (All:All,Sequence:Sequence -- Recurse) (§3.9.6)](037-continuation-37.md#rcase-Recurse), [Particle Derivation OK (Choice:Choice -- RecurseLax) (§3.9.6)](037-continuation-37.md#rcase-RecurseLax) or [Particle Derivation OK (All:All,Sequence:Sequence -- Recurse) (§3.9.6)](037-continuation-37.md#rcase-Recurse), depending on whether the group is *all*, *choice* or *sequence*.

<a id="rcase-NSSubset"></a>

**Schema Component Constraint: Particle Derivation OK (Any:Any -- NSSubset)**  

For a wildcard particle to be a [·valid restriction·](036-3-9-5-particle-information-set-contributions.md#cd-model-restriction) of another wildcard particle
**all** of the following must be true:

1 **R**'s occurrence range must be a valid
restriction of **B**'s occurrence range as defined by [Occurrence Range OK (§3.9.6)](036-3-9-5-particle-information-set-contributions.md#range-ok).

2 **R**'s [{namespace constraint}](038-3-10-wildcards.md#namespace_constraint) must be an intensional
subset of **B**'s [{namespace constraint}](038-3-10-wildcards.md#namespace_constraint) as defined by [Wildcard Subset (§3.10.6)](039-3-10-4-wildcard-validation-rules.md#cos-ns-subset).

3 Unless **B** is the content model wildcard of
the [·ur-type definition·](029-3-4-7-built-in-complex-type-definition.md#ur-type-itself), **R**'s [{process contents}](038-3-10-wildcards.md#process_contents) must be identical
to or stronger than **B**'s [{process contents}](038-3-10-wildcards.md#process_contents), where
*strict* is stronger than *lax* is stronger than *skip*.

<a id="procContNote"></a>

**Note:**

The exception to the third clause above for derivations from
the [·ur-type definition·](029-3-4-7-built-in-complex-type-definition.md#ur-type-itself) is necessary as
its wildcards have a [{process contents}](038-3-10-wildcards.md#process_contents) of *lax*, so
without this exception, no use of wildcards with [{process contents}](038-3-10-wildcards.md#process_contents) of *skip* would be possible.

<a id="rcase-NSRecurseCheckCardinality"></a>

**Schema Component Constraint: Particle Derivation OK (All/Choice/Sequence:Any -- NSRecurseCheckCardinality)**  

For a group particle to be a [·valid restriction·](036-3-9-5-particle-information-set-contributions.md#cd-model-restriction) of a wildcard particle
**all** of the following must be true:

1 Every member of the [{particles}](033-3-7-6-constraints-on-model-group-definition-schema-components.md#particles) of the group is a
[·valid restriction·](036-3-9-5-particle-information-set-contributions.md#cd-model-restriction) of the wildcard
as defined by [Particle Valid (Restriction) (§3.9.6)](036-3-9-5-particle-information-set-contributions.md#cos-particle-restrict).

2 The effective total range of the group, as defined by [Effective Total Range (all and sequence) (§3.8.6)](034-3-8-5-model-group-information-set-contributions.md#cos-seq-range) (if
the group is *all* or *sequence*) or
[Effective Total Range (choice) (§3.8.6)](034-3-8-5-model-group-information-set-contributions.md#cos-choice-range) (if it is *choice*) is a valid
restriction of **B**'s occurrence range as defined by [Occurrence Range OK (§3.9.6)](036-3-9-5-particle-information-set-contributions.md#range-ok).

<a id="rcase-Recurse"></a>

**Schema Component Constraint: Particle Derivation OK (All:All,Sequence:Sequence -- Recurse)**  

For an *all* or *sequence* group particle to be a [·valid restriction·](036-3-9-5-particle-information-set-contributions.md#cd-model-restriction) of another group particle with the same [{compositor}](033-3-7-6-constraints-on-model-group-definition-schema-components.md#compositor)
**all** of the following must be true:

1 **R**'s occurrence range is a valid
restriction of **B**'s occurrence range as defined by [Occurrence Range OK (§3.9.6)](036-3-9-5-particle-information-set-contributions.md#range-ok).

2 There is a complete [·order-preserving·](037-continuation-37.md#key-op) functional mapping from the particles in the
[{particles}](033-3-7-6-constraints-on-model-group-definition-schema-components.md#particles) of **R** to the particles in the [{particles}](033-3-7-6-constraints-on-model-group-definition-schema-components.md#particles) of **B** such that
**all** of the following must be true:

2.1 Each particle in the [{particles}](033-3-7-6-constraints-on-model-group-definition-schema-components.md#particles) of **R** is a
[·valid restriction·](036-3-9-5-particle-information-set-contributions.md#cd-model-restriction) of the
particle in the [{particles}](033-3-7-6-constraints-on-model-group-definition-schema-components.md#particles) of **B** it maps to as defined
by [Particle Valid (Restriction) (§3.9.6)](036-3-9-5-particle-information-set-contributions.md#cos-particle-restrict).

2.2 All particles in the [{particles}](033-3-7-6-constraints-on-model-group-definition-schema-components.md#particles) of **B** which
are not mapped to by any particle in the [{particles}](033-3-7-6-constraints-on-model-group-definition-schema-components.md#particles) of **R**
are [·emptiable·](038-3-10-wildcards.md#cd-emptiable) as defined by [Particle Emptiable (§3.9.6)](038-3-10-wildcards.md#cos-group-emptiable).

**Note:**Although the [·validation·](004-2-1-overview-of-xml-schema.md#key-vn) semantics of an *all* group does not
depend on the order of its particles, derived *all* groups are required to
match the order of their base in order to simplify checking that the derivation is OK.

<a id="key-op"></a>

[Definition:]A complete functional mapping is
**order-preserving** if each particle **r** in the domain **R** maps to a
particle **b** in the range **B** which follows (not necessarily
immediately) the particle in the range
**B** mapped to by the predecessor of **r**, if any, where
"predecessor" and "follows" are defined with respect
to the order of the lists which constitute **R** and **B**.

<a id="rcase-RecurseLax"></a>

**Schema Component Constraint: Particle Derivation OK (Choice:Choice -- RecurseLax)**  

For a *choice* group particle to be a [·valid restriction·](036-3-9-5-particle-information-set-contributions.md#cd-model-restriction) of another *choice* group particle
**all** of the following must be true:

1 **R**'s occurrence range is a valid
restriction of **B**'s occurrence range as defined by [Occurrence Range OK (§3.9.6)](036-3-9-5-particle-information-set-contributions.md#range-ok);

2 There is a complete [·order-preserving·](037-continuation-37.md#key-op) functional mapping from the particles in the
[{particles}](033-3-7-6-constraints-on-model-group-definition-schema-components.md#particles) of **R** to the particles in the [{particles}](033-3-7-6-constraints-on-model-group-definition-schema-components.md#particles) of **B** such that each particle in the [{particles}](033-3-7-6-constraints-on-model-group-definition-schema-components.md#particles) of **R** is a
[·valid restriction·](036-3-9-5-particle-information-set-contributions.md#cd-model-restriction) of the
particle in the [{particles}](033-3-7-6-constraints-on-model-group-definition-schema-components.md#particles) of **B** it maps to as defined
by [Particle Valid (Restriction) (§3.9.6)](036-3-9-5-particle-information-set-contributions.md#cos-particle-restrict).

**Note:**Although the [·validation·](004-2-1-overview-of-xml-schema.md#key-vn) semantics of a *choice* group does not
depend on the order of its particles, derived *choice* groups are
required to
match the order of their base in order to simplify checking that the derivation is OK.

<a id="rcase-RecurseUnordered"></a>

**Schema Component Constraint: Particle Derivation OK (Sequence:All -- RecurseUnordered)**  

For a *sequence* group particle to be a [·valid restriction·](036-3-9-5-particle-information-set-contributions.md#cd-model-restriction) of an *all* group particle
**all** of the following must be true:

1 **R**'s occurrence range is a valid
restriction of **B**'s occurrence range as defined by [Occurrence Range OK (§3.9.6)](036-3-9-5-particle-information-set-contributions.md#range-ok).

2 There is a complete functional mapping from the particles in the
[{particles}](033-3-7-6-constraints-on-model-group-definition-schema-components.md#particles) of **R** to the particles in the [{particles}](033-3-7-6-constraints-on-model-group-definition-schema-components.md#particles) of **B** such that
**all** of the following must be true:

2.1 No particle in the [{particles}](033-3-7-6-constraints-on-model-group-definition-schema-components.md#particles) of **B** is mapped
to by more than one of the particles in the
[{particles}](033-3-7-6-constraints-on-model-group-definition-schema-components.md#particles) of **R**;

2.2 Each particle in the [{particles}](033-3-7-6-constraints-on-model-group-definition-schema-components.md#particles) of **R** is a
[·valid restriction·](036-3-9-5-particle-information-set-contributions.md#cd-model-restriction) of the
particle in the [{particles}](033-3-7-6-constraints-on-model-group-definition-schema-components.md#particles) of **B** it maps to as defined
by [Particle Valid (Restriction) (§3.9.6)](036-3-9-5-particle-information-set-contributions.md#cos-particle-restrict);

2.3 All particles in the [{particles}](033-3-7-6-constraints-on-model-group-definition-schema-components.md#particles) of **B** which
are not mapped to by any particle in the [{particles}](033-3-7-6-constraints-on-model-group-definition-schema-components.md#particles) of **R**
are [·emptiable·](038-3-10-wildcards.md#cd-emptiable) as defined by [Particle Emptiable (§3.9.6)](038-3-10-wildcards.md#cos-group-emptiable).

**Note:**Although this clause allows reordering, because of the limits on the
contents of *all* groups the checking process can still be deterministic.

<a id="rcase-MapAndSum"></a>

**Schema Component Constraint: Particle Derivation OK (Sequence:Choice -- MapAndSum)**  

For a *sequence* group particle to be a [·valid restriction·](036-3-9-5-particle-information-set-contributions.md#cd-model-restriction) of a *choice* group particle
**all** of the following must be true:

1 There is a complete functional mapping from the particles in the
[{particles}](033-3-7-6-constraints-on-model-group-definition-schema-components.md#particles) of **R** to the particles in the [{particles}](033-3-7-6-constraints-on-model-group-definition-schema-components.md#particles) of **B** such that each particle in the [{particles}](033-3-7-6-constraints-on-model-group-definition-schema-components.md#particles) of **R** is a
[·valid restriction·](036-3-9-5-particle-information-set-contributions.md#cd-model-restriction) of the
particle in the [{particles}](033-3-7-6-constraints-on-model-group-definition-schema-components.md#particles) of **B** it maps to as defined
by [Particle Valid (Restriction) (§3.9.6)](036-3-9-5-particle-information-set-contributions.md#cos-particle-restrict).

2 The pair consisting of the product of the [{min occurs}](035-3-9-particles.md#p-min_occurs) of **R** and the length of its [{particles}](033-3-7-6-constraints-on-model-group-definition-schema-components.md#particles) and *unbounded* if [{max occurs}](035-3-9-particles.md#p-max_occurs) is *unbounded* otherwise the product of the [{max occurs}](035-3-9-particles.md#p-max_occurs) of **R** and the length of its [{particles}](033-3-7-6-constraints-on-model-group-definition-schema-components.md#particles) is a valid
restriction of **B**'s occurrence range as defined by [Occurrence Range OK (§3.9.6)](036-3-9-5-particle-information-set-contributions.md#range-ok).

