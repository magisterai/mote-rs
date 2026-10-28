# 3.9.5 Particle Information Set Contributions

Source: [XML Schema Part 1: Structures](https://www.w3.org/TR/2004/REC-xmlschema-1-20041028/) · 2nd edition, 2004-10-28.

[Document index](../README.md) · [Topic guide](../../../../README.md) · [Publisher](https://www.w3.org/TR/2004/REC-xmlschema-1-20041028/).

HTML normalized to Markdown. Complex tables and mathematical markup retain HTML; illustrations are stored locally.

[Previous](035-3-9-particles.md) · [Next](037-continuation-37.md)

**Note:**Clauses clause [1](035-3-9-particles.md#c-pw) and clause [2.3.3](035-3-9-particles.md#c-psg) do not
interact: an element information item validatable by a declaration with a substitution group head in a
different namespace is *not* validatable by a wildcard which accepts
the head's namespace but not its own.

<a id="d0e11288"></a>

#### 3.9.5 Particle Information Set Contributions

None as such.

<a id="coss-particle"></a>

#### 3.9.6 Constraints on Particle Schema Components

All particles (see [Particles (§3.9)](035-3-9-particles.md#cParticles)) must satisfy the following constraints.

<a id="p-props-correct"></a>

**Schema Component Constraint: Particle Correct**  

**All** of the following must be true:

1 The values of the properties of a particle must be as described in
the property tableau in
[The Particle Schema Component (§3.9.1)](035-3-9-particles.md#Particle_details), modulo the impact of [Missing Sub-components (§5.3)](058-5-3-missing-sub-components.md#conformance-missing).

2 If [{max occurs}](035-3-9-particles.md#p-max_occurs) is not *unbounded*, that is, it has a
numeric value, then
**all** of the following must be true:

2.1 [{min occurs}](035-3-9-particles.md#p-min_occurs) must not be greater than [{max occurs}](035-3-9-particles.md#p-max_occurs).

2.2 [{max occurs}](035-3-9-particles.md#p-max_occurs) must be greater than or equal to 1.

The following constraints define relations appealed to elsewhere in this specification.

<a id="cos-particle-extend"></a>

**Schema Component Constraint: Particle Valid (Extension)**  

<a id="cd-model-extension"></a>

[Definition:]For a particle
(call it **E**, for extension) to be a **valid extension** of
another particle (call it **B**, for base)
**one** of the following must be true:

1 They are the same particle.

2 **E**'s [{min occurs}](035-3-9-particles.md#p-min_occurs)=[{max occurs}](035-3-9-particles.md#p-max_occurs)`=1` and its [{term}](035-3-9-particles.md#term) is a *sequence* group whose [{particles}](033-3-7-6-constraints-on-model-group-definition-schema-components.md#particles)' first member is a particle all of whose properties, recursively, are identical to those of **B**, with the exception of {annotation} properties.

The approach to defining a type by restricting another type definition
set out here is designed to ensure that types defined in this way are
guaranteed to be a subset of the type they restrict. This is accomplished by
requiring a clear mapping between the components of the base type definition and the
restricting type definition. Permissible mappings are set out below via a set
of recursive definitions, bottoming out in the obvious cases, e.g. where an
(restricted) element declaration corresponds to another (base) element
declaration with the same name and type but the same or wider range of occurrence.

**Note:**The structural correspondence approach to guaranteeing the subset
relation set out here is necessarily verbose, but has the advantage of being
checkable in a straightforward way. The working group solicits feedback on how
difficult this is in practice, and on whether other approaches are found to be viable.

<a id="cos-particle-restrict"></a>

**Schema Component Constraint: Particle Valid (Restriction)**  

<a id="cd-model-restriction"></a>

[Definition:]For a particle (call it **R**, for restriction) to be a **valid restriction** of
another particle (call it **B**, for base)
**one** of the following must be true:

1 They are the same particle.

2 depending on the kind of particle, per the table below, with the
qualifications that
**all** of the following must be true:

2.1 Any top-level element declaration particle (in **R** or
**B**) which is the
[{substitution group affiliation}](013-3-3-element-declarations.md#class_exemplar) of one or more other element declarations
and whose [·substitution group·](019-continuation-19.md#key-eq)
contains at least one element declaration other than itself is
treated as if it were a *choice* group whose [{min occurs}](035-3-9-particles.md#p-min_occurs) and [{max occurs}](035-3-9-particles.md#p-max_occurs) are those of the particle, and whose [{particles}](033-3-7-6-constraints-on-model-group-definition-schema-components.md#particles) consists of
one particle with [{min occurs}](035-3-9-particles.md#p-min_occurs) and [{max occurs}](035-3-9-particles.md#p-max_occurs) of `1` for each of the declarations in its [·substitution group·](019-continuation-19.md#key-eq).

2.2 Any pointless occurrences of [<sequence>](033-3-7-6-constraints-on-model-group-definition-schema-components.md#element-sequence), [<choice>](033-3-7-6-constraints-on-model-group-definition-schema-components.md#element-choice) or [<all>](033-3-7-6-constraints-on-model-group-definition-schema-components.md#element-all) are ignored, where pointlessness is understood as follows:

[<sequence>](033-3-7-6-constraints-on-model-group-definition-schema-components.md#element-sequence)
:   **One** of the following must be true:

    2.2.1 [{particles}](033-3-7-6-constraints-on-model-group-definition-schema-components.md#particles) is empty.

    2.2.2 **All** of the following must be true:

    2.2.2.1 The particle within which this [<sequence>](033-3-7-6-constraints-on-model-group-definition-schema-components.md#element-sequence)
    appears has [{max occurs}](035-3-9-particles.md#p-max_occurs) and [{min occurs}](035-3-9-particles.md#p-min_occurs) of
    `1`.

    2.2.2.2 **One** of the following must be true:

    2.2.2.2.1 The [<sequence>](033-3-7-6-constraints-on-model-group-definition-schema-components.md#element-sequence)'s [{particles}](033-3-7-6-constraints-on-model-group-definition-schema-components.md#particles)
    has only one member.

    2.2.2.2.2 The particle within which this [<sequence>](033-3-7-6-constraints-on-model-group-definition-schema-components.md#element-sequence)
    appears is itself among the [{particles}](033-3-7-6-constraints-on-model-group-definition-schema-components.md#particles) of a [<sequence>](033-3-7-6-constraints-on-model-group-definition-schema-components.md#element-sequence).

[<all>](033-3-7-6-constraints-on-model-group-definition-schema-components.md#element-all)
:   **One** of the following must be true:

    2.2.1 [{particles}](033-3-7-6-constraints-on-model-group-definition-schema-components.md#particles) is empty.

    2.2.2 [{particles}](033-3-7-6-constraints-on-model-group-definition-schema-components.md#particles) has only one member.

[<choice>](033-3-7-6-constraints-on-model-group-definition-schema-components.md#element-choice)
:   **One** of the following must be true:

    2.2.1 [{particles}](033-3-7-6-constraints-on-model-group-definition-schema-components.md#particles) is empty and the
    particle within which this [<choice>](033-3-7-6-constraints-on-model-group-definition-schema-components.md#element-choice) appears has [{min occurs}](035-3-9-particles.md#p-min_occurs) of `0`.

    2.2.2 **All** of the following must be true:

    2.2.2.1 The particle within which this [<choice>](033-3-7-6-constraints-on-model-group-definition-schema-components.md#element-choice)
    appears has [{max occurs}](035-3-9-particles.md#p-max_occurs) and [{min occurs}](035-3-9-particles.md#p-min_occurs) of
    `1`.

    2.2.2.2 **One** of the following must be true:

    2.2.2.2.1 The [<choice>](033-3-7-6-constraints-on-model-group-definition-schema-components.md#element-choice)'s [{particles}](033-3-7-6-constraints-on-model-group-definition-schema-components.md#particles)
    has only one member.

    2.2.2.2.2 The particle within which this [<choice>](033-3-7-6-constraints-on-model-group-definition-schema-components.md#element-choice)
    appears is itself among the [{particles}](033-3-7-6-constraints-on-model-group-definition-schema-components.md#particles) of a [<choice>](033-3-7-6-constraints-on-model-group-definition-schema-components.md#element-choice).

<table border="2" cellpadding="3" class="restricts"><thead><tr><th colspan="2"></th><th colspan="6"><strong>Base Particle</strong></th></tr></thead><tbody><tr><th colspan="2"></th><th>elt</th><th>any</th><th>all</th><th>choice</th><th>sequence</th></tr><tr><th rowspan="5">Derived Particle</th><th>elt</th><td><a href="036-3-9-5-particle-information-set-contributions.md#rcase-NameAndTypeOK">NameAnd- TypeOK</a></td><td><a href="037-continuation-37.md#rcase-NSCompat">NSCompat</a></td><td><a href="037-continuation-37.md#rcase-RecurseAsIfGroup">Recurse- AsIfGroup</a></td><td><a href="037-continuation-37.md#rcase-RecurseAsIfGroup">Recurse- AsIfGroup</a></td><td><a href="037-continuation-37.md#rcase-RecurseAsIfGroup">RecurseAs- IfGroup</a></td></tr><tr><th>any</th><td>Forbidden</td><td><a href="037-continuation-37.md#rcase-NSSubset">NSSubset</a></td><td>Forbidden</td><td>Forbidden</td><td>Forbidden</td></tr><tr><th>all</th><td>Forbidden</td><td><a href="037-continuation-37.md#rcase-NSRecurseCheckCardinality">NSRecurse- CheckCardinality</a></td><td><a href="037-continuation-37.md#rcase-Recurse">Recurse</a></td><td>Forbidden</td><td>Forbidden</td></tr><tr><th>choice</th><td>Forbidden</td><td><a href="037-continuation-37.md#rcase-NSRecurseCheckCardinality">NSRecurse- CheckCardinality</a></td><td>Forbidden</td><td><a href="037-continuation-37.md#rcase-RecurseLax">RecurseLax</a></td><td>Forbidden</td></tr><tr><th>seq- uence</th><td>Forbidden</td><td><a href="037-continuation-37.md#rcase-NSRecurseCheckCardinality">NSRecurse- CheckCardinality</a></td><td><a href="037-continuation-37.md#rcase-RecurseUnordered">Recurse- Unordered</a></td><td><a href="037-continuation-37.md#rcase-MapAndSum">MapAndSum</a></td><td><a href="037-continuation-37.md#rcase-Recurse">Recurse</a></td></tr></tbody></table>

<a id="range-ok"></a>

**Schema Component Constraint: Occurrence Range OK**  

For a particle's occurrence range to be a valid restriction of another's
occurrence range
**all** of the following must be true:

1 Its [{min occurs}](035-3-9-particles.md#p-min_occurs) is greater than or equal to the
other's [{min occurs}](035-3-9-particles.md#p-min_occurs).

2 **one** of the following must be true:

2.1 The other's [{max occurs}](035-3-9-particles.md#p-max_occurs) is *unbounded*.

2.2 Both [{max occurs}](035-3-9-particles.md#p-max_occurs) are numbers, and the particle's is less than or equal to the
other's.

<a id="rcase-NameAndTypeOK"></a>

**Schema Component Constraint: Particle Restriction OK (Elt:Elt -- NameAndTypeOK)**  

For an element declaration particle to be a [·valid restriction·](036-3-9-5-particle-information-set-contributions.md#cd-model-restriction) of another element declaration particle
**all** of the following must be true:

1 The declarations' [{name}](013-3-3-element-declarations.md#e-name)s and [{target namespace}](013-3-3-element-declarations.md#e-target_namespace)s are the same.

2 **R**'s occurrence range is a valid
restriction of **B**'s occurrence range as defined by [Occurrence Range OK (§3.9.6)](036-3-9-5-particle-information-set-contributions.md#range-ok).

3 **One** of the following must be true:

3.1 Both **B**'s declaration's [{scope}](013-3-3-element-declarations.md#e-scope) and
**R**'s declaration's [{scope}](013-3-3-element-declarations.md#e-scope) are *global*.

3.2 **All** of the following must be true:

3.2.1 Either **B**'s [{nillable}](013-3-3-element-declarations.md#nillable) is *true* or **R**'s [{nillable}](013-3-3-element-declarations.md#nillable) is *false*.

3.2.2 either **B**'s declaration's [{value constraint}](013-3-3-element-declarations.md#e-value_constraint) is
absent, or is
not *fixed*, or **R**'s declaration's [{value constraint}](013-3-3-element-declarations.md#e-value_constraint) is
*fixed* with the same value.

3.2.3 **R**'s declaration's [{identity-constraint definitions}](013-3-3-element-declarations.md#identity-constraint_definitions) is
a subset of **B**'s declaration's [{identity-constraint definitions}](013-3-3-element-declarations.md#identity-constraint_definitions),
if any.

3.2.4 **R**'s declaration's [{disallowed substitutions}](013-3-3-element-declarations.md#e-exact) is
a superset of **B**'s declaration's [{disallowed substitutions}](013-3-3-element-declarations.md#e-exact).

3.2.5 **R**'s [{type definition}](013-3-3-element-declarations.md#type_definition) is validly derived given
{*extension*, *list*, *union*} from **B**'s [{type definition}](013-3-3-element-declarations.md#type_definition) as defined by
[Type Derivation OK (Complex) (§3.4.6)](028-continuation-28.md#cos-ct-derived-ok) or [Type Derivation OK (Simple) (§3.14.6)](048-3-14-7-built-in-simple-type-definition.md#cos-st-derived-ok), as appropriate.

