# Continuation 28

Source: [XML Schema Part 1: Structures](https://www.w3.org/TR/2004/REC-xmlschema-1-20041028/) · 2nd edition, 2004-10-28.

[Document index](../README.md) · [Topic guide](../../../../README.md) · [Publisher](https://www.w3.org/TR/2004/REC-xmlschema-1-20041028/).

HTML normalized to Markdown. Complex tables and mathematical markup retain HTML; illustrations are stored locally.

[Previous](027-3-4-5-complex-type-definition-information-set-contributions.md) · [Next](029-3-4-7-built-in-complex-type-definition.md)

1.5 It must in principle be possible to derive the complex type
definition in two steps, the first an extension and the
second a restriction (possibly vacuous), from that type definition among its
ancestors whose [{base type definition}](020-3-4-complex-type-definitions.md#ct-base_type_definition) is the [·ur-type definition·](029-3-4-7-built-in-complex-type-definition.md#ur-type-itself).

**Note:**This requirement ensures that nothing removed by a restriction
is subsequently added back by an extension. It is trivial to check if the
extension in question is the only extension in its derivation, or if there are
no restrictions bar the first from the [·ur-type
definition·](029-3-4-7-built-in-complex-type-definition.md#ur-type-itself).Constructing the intermediate type definition to check this
constraint is straightforward: simply re-order the derivation to put all the
extension steps first, then collapse them into a single extension. If the
resulting definition can be the basis for a valid restriction to the desired
definition, the constraint is satisfied.

2 **If** the [{base type definition}](020-3-4-complex-type-definitions.md#ct-base_type_definition) is a simple type
definition, **then** 
**all** of the following must be true:

2.1 The [{content type}](020-3-4-complex-type-definitions.md#content_type) must be the same simple type
definition.

2.2 The [{final}](045-3-14-simple-type-definitions.md#st-final) of the [{base type definition}](020-3-4-complex-type-definitions.md#ct-base_type_definition) must not contain *extension*.

<a id="cd-ct-extension"></a>

[Definition:]If this
constraint [Derivation Valid (Extension) (§3.4.6)](027-3-4-5-complex-type-definition-information-set-contributions.md#cos-ct-extends) holds of a complex type definition, it is a **valid
extension** of its [{base type definition}](020-3-4-complex-type-definitions.md#ct-base_type_definition).

<a id="derivation-ok-restriction"></a>

**Schema Component Constraint: Derivation Valid (Restriction, Complex)**  

If the [{derivation method}](020-3-4-complex-type-definitions.md#derivation_method) is *restriction*
**all** of the following must be true:

1 The [{base type definition}](020-3-4-complex-type-definitions.md#ct-base_type_definition) must be a complex type
definition whose [{final}](020-3-4-complex-type-definitions.md#ct-final) does not contain *restriction*.

2

<a id="c-rad"></a>

For each attribute use (call this **R**) in the [{attribute uses}](020-3-4-complex-type-definitions.md#ct-attribute_declarations)
the appropriate **case** among the following
must be true:

2.1 **If** there is an attribute use in the
[{attribute uses}](020-3-4-complex-type-definitions.md#ct-attribute_declarations) of the [{base type definition}](020-3-4-complex-type-definitions.md#ct-base_type_definition) (call this **B**) whose [{attribute declaration}](030-3-5-attributeuses.md#attribute) has the same [{name}](009-3-2-attribute-declarations.md#a-name) and [{target namespace}](009-3-2-attribute-declarations.md#a-target_namespace), **then** 
**all** of the following must be true:

2.1.1 **one** of the following must be true:

2.1.1.1 **B**'s [{required}](030-3-5-attributeuses.md#required) is *false*.

2.1.1.2 **R**'s [{required}](030-3-5-attributeuses.md#required) is *true*.

2.1.2 **R**'s [{attribute declaration}](030-3-5-attributeuses.md#attribute)'s [{type definition}](009-3-2-attribute-declarations.md#a-simple_type_definition) must be validly derived from **B**'s [{type definition}](009-3-2-attribute-declarations.md#a-simple_type_definition) given the
empty set as defined in
[Type Derivation OK (Simple) (§3.14.6)](048-3-14-7-built-in-simple-type-definition.md#cos-st-derived-ok).

2.1.3 

<a id="key-evc"></a>

[Definition:]Let the
**effective value constraint** of an attribute use be
its [{value constraint}](030-3-5-attributeuses.md#au-value_constraint), if present, otherwise
its [{attribute declaration}](030-3-5-attributeuses.md#attribute)'s [{value constraint}](009-3-2-attribute-declarations.md#a-value_constraint)
. Then **one** of the following must be true:

2.1.3.1 **B**'s [·effective value
constraint·](028-continuation-28.md#key-evc) is [·absent·](008-3-schema-component-details.md#key-null) or *default*.

2.1.3.2 **R**'s [·effective value
constraint·](028-continuation-28.md#key-evc) is
*fixed* with the same string as **B**'s.

2.2 **otherwise** the [{base type definition}](020-3-4-complex-type-definitions.md#ct-base_type_definition) must
have an [{attribute wildcard}](020-3-4-complex-type-definitions.md#ct-attribute_wildcard) and the [{target namespace}](009-3-2-attribute-declarations.md#a-target_namespace) of the **R**'s [{attribute declaration}](030-3-5-attributeuses.md#attribute) must be [·valid·](004-2-1-overview-of-xml-schema.md#key-vn) with respect to that wildcard, as defined in [Wildcard allows Namespace Name (§3.10.4)](039-3-10-4-wildcard-validation-rules.md#cvc-wildcard-namespace).

3

<a id="c-rad2"></a>

For each attribute use in the [{attribute uses}](020-3-4-complex-type-definitions.md#ct-attribute_declarations) of
the [{base type definition}](020-3-4-complex-type-definitions.md#ct-base_type_definition) whose [{required}](030-3-5-attributeuses.md#required) is
*true*, there must be an attribute use with an [{attribute declaration}](030-3-5-attributeuses.md#attribute) with the same [{name}](009-3-2-attribute-declarations.md#a-name) and [{target namespace}](009-3-2-attribute-declarations.md#a-target_namespace) as its [{attribute declaration}](030-3-5-attributeuses.md#attribute) in the
[{attribute uses}](020-3-4-complex-type-definitions.md#ct-attribute_declarations) of the complex type definition
itself whose [{required}](030-3-5-attributeuses.md#required) is
*true*.

4

<a id="c-raw"></a>

If there is an [{attribute wildcard}](020-3-4-complex-type-definitions.md#ct-attribute_wildcard),
**all** of the following must be true:

4.1 The
[{base type definition}](020-3-4-complex-type-definitions.md#ct-base_type_definition) must also have one.

4.2 The complex
type definition's [{attribute wildcard}](020-3-4-complex-type-definitions.md#ct-attribute_wildcard)'s [{namespace constraint}](038-3-10-wildcards.md#namespace_constraint) must be a subset of the [{base type definition}](020-3-4-complex-type-definitions.md#ct-base_type_definition)'s [{attribute wildcard}](020-3-4-complex-type-definitions.md#ct-attribute_wildcard)'s [{namespace constraint}](038-3-10-wildcards.md#namespace_constraint), as defined by [Wildcard Subset (§3.10.6)](039-3-10-4-wildcard-validation-rules.md#cos-ns-subset).

4.3 Unless the [{base type definition}](020-3-4-complex-type-definitions.md#ct-base_type_definition) is the [·ur-type definition·](029-3-4-7-built-in-complex-type-definition.md#ur-type-itself), the complex
type definition's [{attribute wildcard}](020-3-4-complex-type-definitions.md#ct-attribute_wildcard)'s [{process contents}](038-3-10-wildcards.md#process_contents) must be identical
to or stronger than the [{base type definition}](020-3-4-complex-type-definitions.md#ct-base_type_definition)'s [{attribute wildcard}](020-3-4-complex-type-definitions.md#ct-attribute_wildcard)'s [{process contents}](038-3-10-wildcards.md#process_contents), where
*strict* is stronger than *lax* is stronger than *skip*.

5 **One** of the following must be true:

5.1 The [{base type definition}](020-3-4-complex-type-definitions.md#ct-base_type_definition) must be the [·ur-type definition·](029-3-4-7-built-in-complex-type-definition.md#ur-type-itself).

5.2 **All** of the following must be true:

5.2.1 The [{content type}](020-3-4-complex-type-definitions.md#content_type) of the complex type definition
must be a simple type definition

5.2.2 **One** of the following must be true:

5.2.2.1 The [{content type}](020-3-4-complex-type-definitions.md#content_type) of the [{base type definition}](020-3-4-complex-type-definitions.md#ct-base_type_definition) must be a simple type
definition from which
the [{content type}](020-3-4-complex-type-definitions.md#content_type) is validly derived given the empty set as defined in
[Type Derivation OK (Simple) (§3.14.6)](048-3-14-7-built-in-simple-type-definition.md#cos-st-derived-ok).

5.2.2.2 The [{base type definition}](020-3-4-complex-type-definitions.md#ct-base_type_definition) must be *mixed*
and have a particle which is [·emptiable·](038-3-10-wildcards.md#cd-emptiable) as defined in [Particle Emptiable (§3.9.6)](038-3-10-wildcards.md#cos-group-emptiable).

5.3 **All** of the following must be true:

5.3.1 The [{content type}](020-3-4-complex-type-definitions.md#content_type) of the complex type itself
must be *empty*

5.3.2 **One** of the following must be true:

5.3.2.1 The [{content type}](020-3-4-complex-type-definitions.md#content_type) of the [{base type definition}](020-3-4-complex-type-definitions.md#ct-base_type_definition) must also be *empty*.

5.3.2.2 The [{content type}](020-3-4-complex-type-definitions.md#content_type) of the [{base type definition}](020-3-4-complex-type-definitions.md#ct-base_type_definition) must be *elementOnly* or *mixed* and have a particle which is [·emptiable·](038-3-10-wildcards.md#cd-emptiable) as defined in [Particle Emptiable (§3.9.6)](038-3-10-wildcards.md#cos-group-emptiable).

5.4 **All** of the following must be true:

5.4.1 **One** of the following must be true:

5.4.1.1 The [{content type}](020-3-4-complex-type-definitions.md#content_type) of the complex type
definition itself must be *element-only*

5.4.1.2 The [{content type}](020-3-4-complex-type-definitions.md#content_type) of the complex
type definition itself and of the
[{base type definition}](020-3-4-complex-type-definitions.md#ct-base_type_definition) must be *mixed*

5.4.2 The particle of the complex type definition itself
must be a [·valid restriction·](036-3-9-5-particle-information-set-contributions.md#cd-model-restriction) of the particle of the [{content type}](020-3-4-complex-type-definitions.md#content_type) of the [{base type definition}](020-3-4-complex-type-definitions.md#ct-base_type_definition) as defined in [Particle Valid (Restriction) (§3.9.6)](036-3-9-5-particle-information-set-contributions.md#cos-particle-restrict).

**Note:**Attempts to derive complex type definitions whose [{content type}](020-3-4-complex-type-definitions.md#content_type) is *element-only* by restricting
a [{base type definition}](020-3-4-complex-type-definitions.md#ct-base_type_definition) whose [{content type}](020-3-4-complex-type-definitions.md#content_type)
is *empty* are not ruled out by this clause. However if the complex
type definition itself has a non-pointless particle it will fail to satisfy
[Particle Valid (Restriction) (§3.9.6)](036-3-9-5-particle-information-set-contributions.md#cos-particle-restrict). On the other hand some type
definitions with pointless *element-only* content, for example an empty
[<sequence>](033-3-7-6-constraints-on-model-group-definition-schema-components.md#element-sequence), will satisfy [Particle Valid (Restriction) (§3.9.6)](036-3-9-5-particle-information-set-contributions.md#cos-particle-restrict)
with respect to an *empty* [{base type definition}](020-3-4-complex-type-definitions.md#ct-base_type_definition), and
so be valid restrictions.

<a id="cd-ct-restriction"></a>

[Definition:]If this
constraint [Derivation Valid (Restriction, Complex) (§3.4.6)](028-continuation-28.md#derivation-ok-restriction) holds of a complex type definition, it is a **valid
restriction** of its [{base type definition}](020-3-4-complex-type-definitions.md#ct-base_type_definition).

**Note:**To restrict a complex type definition with a simple base type definition
to *empty*, use a simple type definition with a *fixed* value of
the empty string: this preserves the type information.

The following constraint defines a relation appealed to elsewhere in this specification.

<a id="cos-ct-derived-ok"></a>

**Schema Component Constraint: Type Derivation OK (Complex)**  

For a complex type definition (call it **D**, for derived) to be validly
derived from a type definition (call this **B**, for base) given
a subset of {*extension*, *restriction*}
**all** of the following must be true:

