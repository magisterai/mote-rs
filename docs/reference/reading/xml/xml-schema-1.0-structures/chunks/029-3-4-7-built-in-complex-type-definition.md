# 3.4.7 Built-in Complex Type Definition

Source: [XML Schema Part 1: Structures](https://www.w3.org/TR/2004/REC-xmlschema-1-20041028/) · 2nd edition, 2004-10-28.

[Document index](../README.md) · [Topic guide](../../../../README.md) · [Publisher](https://www.w3.org/TR/2004/REC-xmlschema-1-20041028/).

HTML normalized to Markdown. Complex tables and mathematical markup retain HTML; illustrations are stored locally.

[Previous](028-continuation-28.md) · [Next](030-3-5-attributeuses.md)

1 If **B** and **D** are not the same type definition, then
the [{derivation method}](020-3-4-complex-type-definitions.md#derivation_method) of **D** must not be in the subset.

2
**One** of the following must be true:

2.1

<a id="c-tid"></a>

**B** and **D** must be the same type
definition.

2.2 **B** must be **D**'s [{base type definition}](020-3-4-complex-type-definitions.md#ct-base_type_definition).

2.3
**All** of the following must be true:

2.3.1 **D**'s [{base type definition}](020-3-4-complex-type-definitions.md#ct-base_type_definition) must not be the
[·ur-type definition·](029-3-4-7-built-in-complex-type-definition.md#ur-type-itself).

2.3.2
The appropriate **case** among the following
must be true:

2.3.2.1 **If** **D**'s [{base type definition}](020-3-4-complex-type-definitions.md#ct-base_type_definition) is complex, **then** it must be validly derived
from **B** given the subset as defined by this constraint.

2.3.2.2 **If** **D**'s [{base type definition}](020-3-4-complex-type-definitions.md#ct-base_type_definition) is simple, **then** it must be validly derived
from **B** given the subset as defined in [Type Derivation OK (Simple) (§3.14.6)](048-3-14-7-built-in-simple-type-definition.md#cos-st-derived-ok).

**Note:**This constraint is used to check that when someone uses a type in a
context where another type was expected (either via `xsi:type` or
substitution groups), that the type used is actually derived from the expected
type, and that that derivation does not involve a form of derivation which was
ruled out by the expected type.

<a id="no-identity"></a>

**Note:**

The wording of clause [2.1](029-3-4-7-built-in-complex-type-definition.md#c-tid) above appeals to a notion of component identity which
is only incompletely defined by this version of this specification.
In some cases, the wording of this specification does make clear the
rules for component identity. These cases include:

- When they are both top-level components with the same component type,
  namespace name, and local name;
- When they are necessarily the same type definition (for example, when
  the two types definitions in question are the type definitions associated with
  two attribute or element declarations, which are discovered to be the same
  declaration);
- When they are the same by construction (for example, when an element's
  type definition defaults to being the same type definition as that of its
  substitution-group head or when a complex type definition inherits an attribute
  declaration from its base type definition).

In other cases two conforming implementations may disagree as to whether
components are identical.

<a id="d0e9252"></a>

#### 3.4.7 Built-in Complex Type Definition

There is a complex type definition nearly equivalent to the [·ur-type definition·](004-2-1-overview-of-xml-schema.md#key-urType) present in every
schema by definition. It has the following properties:

<a id="ur-type-itself"></a>

<table border="1" class="schemaComp"><thead><tr><th><strong><a id="ur-type-itself" name="ur-type-itself">Complex Type Definition of the Ur-Type</a></strong></th></tr></thead><tbody><tr><td><table border="0" cellpadding="3"><thead><tr><th align="left">Property</th><th align="left">Value</th></tr></thead><tbody valign="top"><tr><td><a class="propref" href="020-3-4-complex-type-definitions.md#ct-name">{name}</a></td><td>anyType</td></tr><tr><td><a class="propref" href="020-3-4-complex-type-definitions.md#ct-target_namespace">{target namespace}</a></td><td>http://www.w3.org/2001/XMLSchema</td></tr><tr><td><a class="propref" href="020-3-4-complex-type-definitions.md#ct-base_type_definition">{base type definition}</a></td><td>Itself</td></tr><tr><td><a class="propref" href="020-3-4-complex-type-definitions.md#derivation_method">{derivation method}</a></td><td><i>restriction</i></td></tr><tr><td><a class="propref" href="020-3-4-complex-type-definitions.md#content_type">{content type}</a></td><td>A pair consisting of <i>mixed</i> and a
particle with the following properties: 
       <table border="0" cellpadding="3"><thead><tr><th align="left">Property</th><th align="left">Value</th></tr></thead><tbody valign="top"><tr><td><a class="propref" href="035-3-9-particles.md#p-min_occurs">{min occurs}</a></td><td>1</td></tr><tr><td><a class="propref" href="035-3-9-particles.md#p-max_occurs">{max occurs}</a></td><td>1</td></tr><tr><td><a class="propref" href="035-3-9-particles.md#term">{term}</a></td><td>a model group with
the following properties:
         <table border="0" cellpadding="3"><thead><tr><th align="left">Property</th><th align="left">Value</th></tr></thead><tbody valign="top"><tr><td><a class="propref" href="033-3-7-6-constraints-on-model-group-definition-schema-components.md#compositor">{compositor}</a></td><td><i>sequence</i></td></tr><tr><td><a class="propref" href="033-3-7-6-constraints-on-model-group-definition-schema-components.md#particles">{particles}</a></td><td>
           a list containing one particle with the following properties:
           <table border="0" cellpadding="3"><thead><tr><th align="left">Property</th><th align="left">Value</th></tr></thead><tbody valign="top"><tr><td><a class="propref" href="035-3-9-particles.md#p-min_occurs">{min occurs}</a></td><td>0</td></tr><tr><td><a class="propref" href="035-3-9-particles.md#p-max_occurs">{max occurs}</a></td><td><i>unbounded</i></td></tr><tr><td><a class="propref" href="035-3-9-particles.md#term">{term}</a></td><td>a wildcard with the following properties:
             <table border="0" cellpadding="3"><thead><tr><th align="left">Property</th><th align="left">Value</th></tr></thead><tbody valign="top"><tr><td><a class="propref" href="038-3-10-wildcards.md#namespace_constraint">{namespace constraint}</a></td><td><i>any</i></td></tr><tr><td><a class="propref" href="038-3-10-wildcards.md#process_contents">{process contents}</a></td><td><i>lax</i></td></tr></tbody></table>
</td></tr></tbody></table>
</td></tr></tbody></table>
</td></tr></tbody></table>
</td></tr><tr><td><a class="propref" href="020-3-4-complex-type-definitions.md#ct-attribute_declarations">{attribute uses}</a></td><td>The empty set</td></tr><tr><td><a class="propref" href="020-3-4-complex-type-definitions.md#ct-attribute_wildcard">{attribute wildcard}</a></td><td>
        a wildcard with the following properties::
             <table border="0" cellpadding="3"><thead><tr><th align="left">Property</th><th align="left">Value</th></tr></thead><tbody valign="top"><tr><td><a class="propref" href="038-3-10-wildcards.md#namespace_constraint">{namespace constraint}</a></td><td><i>any</i></td></tr><tr><td><a class="propref" href="038-3-10-wildcards.md#process_contents">{process contents}</a></td><td><i>lax</i></td></tr></tbody></table></td></tr><tr><td><a class="propref" href="020-3-4-complex-type-definitions.md#ct-final">{final}</a></td><td>The empty set</td></tr><tr><td><a class="propref" href="020-3-4-complex-type-definitions.md#ct-exact">{prohibited substitutions}</a></td><td>The empty set</td></tr><tr><td><a class="propref" href="020-3-4-complex-type-definitions.md#ct-abstract">{abstract}</a></td><td><i>false</i></td></tr></tbody></table></td></tr></tbody></table>

The `mixed` content specification together with the
*lax* wildcard and attribute specification produce the defining property for the
[·ur-type definition·](004-2-1-overview-of-xml-schema.md#key-urType), namely that *every* type
definition is (eventually) a restriction
of the [·ur-type definition·](004-2-1-overview-of-xml-schema.md#key-urType): its permissions and requirements are
(nearly) the least restrictive possible.

**Note:**This specification does not provide an inventory of built-in complex
type definitions for use in user schemas. A preliminary library of complex type
definitions is available which includes both mathematical (e.g.
`rational`) and utility (e.g. `array`) type definitions.
In particular, there is a `text` type definition which is recommended for use
as the type definition in element declarations intended for general text
content, as it makes sensible provision for various aspects of
internationalization. For more details, see the schema document for the type
library at its namespace name: [http://www.w3.org/2001/03/XMLSchema/TypeLibrary.xsd](https://www.w3.org/2001/03/XMLSchema/TypeLibrary.xsd).

