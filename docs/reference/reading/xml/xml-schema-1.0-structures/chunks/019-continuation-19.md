# Continuation 19

Source: [XML Schema Part 1: Structures](https://www.w3.org/TR/2004/REC-xmlschema-1-20041028/) · 2nd edition, 2004-10-28.

[Document index](../README.md) · [Topic guide](../../../../README.md) · [Publisher](https://www.w3.org/TR/2004/REC-xmlschema-1-20041028/).

HTML normalized to Markdown. Complex tables and mathematical markup retain HTML; illustrations are stored locally.

[Previous](018-3-3-6-constraints-on-element-declaration-schema-components.md) · [Next](020-3-4-complex-type-definitions.md)

1 The values of the properties of an element declaration must be as described in
the property tableau in
[The Element Declaration Schema Component (§3.3.1)](013-3-3-element-declarations.md#Element_Declaration_details), modulo the impact of [Missing Sub-components (§5.3)](058-5-3-missing-sub-components.md#conformance-missing).

2 If there is a [{value constraint}](013-3-3-element-declarations.md#e-value_constraint), the [canonical lexical representation](https://www.w3.org/TR/2004/REC-xmlschema-2-20041028/datatypes.html#dt-canonical-representation) of its value must be
[·valid·](004-2-1-overview-of-xml-schema.md#key-vn) with respect to the [{type definition}](013-3-3-element-declarations.md#type_definition) as defined in [Element Default Valid (Immediate) (§3.3.6)](019-continuation-19.md#cos-valid-default).

3 If there is a non-[·absent·](008-3-schema-component-details.md#key-null)
[{substitution group affiliation}](013-3-3-element-declarations.md#class_exemplar), then [{scope}](013-3-3-element-declarations.md#e-scope) must be *global*.

4 If there is a [{substitution group affiliation}](013-3-3-element-declarations.md#class_exemplar), the [{type definition}](013-3-3-element-declarations.md#type_definition) of the element declaration must
be validly derived from the [{type definition}](013-3-3-element-declarations.md#type_definition) of the [{substitution group affiliation}](013-3-3-element-declarations.md#class_exemplar), given the value of the [{substitution group exclusions}](013-3-3-element-declarations.md#e-final) of the [{substitution group affiliation}](013-3-3-element-declarations.md#class_exemplar), as defined in [Type Derivation OK (Complex) (§3.4.6)](028-continuation-28.md#cos-ct-derived-ok) (if the [{type definition}](013-3-3-element-declarations.md#type_definition) is complex) or as defined in [Type Derivation OK (Simple) (§3.14.6)](048-3-14-7-built-in-simple-type-definition.md#cos-st-derived-ok) (if the [{type definition}](013-3-3-element-declarations.md#type_definition) is simple).

5 If the [{type definition}](013-3-3-element-declarations.md#type_definition) or [{type definition}](013-3-3-element-declarations.md#type_definition)'s [{content type}](020-3-4-complex-type-definitions.md#content_type) is or is derived from [ID](https://www.w3.org/TR/2004/REC-xmlschema-2-20041028/datatypes.html#ID) then there must not be a [{value constraint}](013-3-3-element-declarations.md#e-value_constraint).

**Note:**The use of [ID](https://www.w3.org/TR/2004/REC-xmlschema-2-20041028/datatypes.html#ID) as a type
definition for elements goes beyond XML 1.0, and should be avoided if backwards
compatibility is desired.

6 Circular substitution groups are disallowed. That is, it
must not be possible to return to an element declaration by repeatedly following
the [{substitution group affiliation}](013-3-3-element-declarations.md#class_exemplar) property.

The following constraints define relations appealed to elsewhere in this specification.

<a id="cos-valid-default"></a>

**Schema Component Constraint: Element Default Valid (Immediate)**  

For a string to be a valid default with respect to a type definition
the appropriate **case** among the following
must be true:

1 **If** the type definition is a simple type definition, **then** the string must be
[·valid·](004-2-1-overview-of-xml-schema.md#key-vn) with respect to that definition as defined by [String Valid (§3.14.4)](047-3-14-3-constraints-on-xml-representations-of-simple-type-definitions.md#cvc-simple-type).

2 **If** the type definition is a complex type definition, **then** 
**all** of the following must be true:

2.1 its [{content type}](020-3-4-complex-type-definitions.md#content_type) must be a simple type definition
or *mixed*.

2.2 The appropriate **case** among the following
must be true:

2.2.1 **If** the [{content type}](020-3-4-complex-type-definitions.md#content_type) is a simple type definition, **then** the string must be
[·valid·](004-2-1-overview-of-xml-schema.md#key-vn) with respect to that simple type definition as defined by [String Valid (§3.14.4)](047-3-14-3-constraints-on-xml-representations-of-simple-type-definitions.md#cvc-simple-type).

2.2.2 **If** the [{content type}](020-3-4-complex-type-definitions.md#content_type) is *mixed*, **then** the [{content type}](020-3-4-complex-type-definitions.md#content_type)'s particle must be [·emptiable·](038-3-10-wildcards.md#cd-emptiable) as defined by [Particle Emptiable (§3.9.6)](038-3-10-wildcards.md#cos-group-emptiable).

<a id="cos-equiv-derived-ok-rec"></a>

**Schema Component Constraint: Substitution Group OK (Transitive)**  

For an element declaration (call it **D**) to be validly
substitutable for another element declaration (call it **C**)
subject to a blocking constraint (a subset of
{*substitution*, *extension*, *restriction*}, the value of
a [{disallowed substitutions}](013-3-3-element-declarations.md#e-exact))
**one** of the following must be true:

1 **D** and **C** are the same element declaration.

2 **All** of the following must be true:

2.1 The blocking constraint does not contain *substitution*.

2.2 There is a chain of [{substitution group affiliation}](013-3-3-element-declarations.md#class_exemplar)s from **D** to
**C**, that is, either **D**'s [{substitution group affiliation}](013-3-3-element-declarations.md#class_exemplar) is
**C**, or **D**'s [{substitution group affiliation}](013-3-3-element-declarations.md#class_exemplar)'s [{substitution group affiliation}](013-3-3-element-declarations.md#class_exemplar) is **C**, or . . .

2.3 The set of all [{derivation method}](020-3-4-complex-type-definitions.md#derivation_method)s
involved in the derivation of **D**'s [{type definition}](013-3-3-element-declarations.md#type_definition) from
**C**'s [{type definition}](013-3-3-element-declarations.md#type_definition) does not intersect with the union
of the blocking constraint, **C**'s [{prohibited substitutions}](020-3-4-complex-type-definitions.md#ct-exact) (if **C**
is complex, otherwise the empty set) and the
[{prohibited substitutions}](020-3-4-complex-type-definitions.md#ct-exact) (respectively the empty set) of any intermediate [{type definition}](013-3-3-element-declarations.md#type_definition)s
in the derivation of **D**'s [{type definition}](013-3-3-element-declarations.md#type_definition) from
**C**'s [{type definition}](013-3-3-element-declarations.md#type_definition).

<a id="cos-equiv-class"></a>

**Schema Component Constraint: Substitution Group**  

<a id="key-eq"></a>

[Definition:]Every element
declaration (call this **HEAD**)
in the [{element declarations}](049-3-15-schemas-as-a-whole.md#element_declarations) of a schema defines a
**substitution group**, a subset of those [{element declarations}](049-3-15-schemas-as-a-whole.md#element_declarations), as follows:Define **P**, the potential substitution group for **HEAD**, as follows:

1 The element declaration itself is in **P**;

2 **P** is closed with respect to [{substitution group affiliation}](013-3-3-element-declarations.md#class_exemplar), that
is, if any element declaration in the [{element declarations}](049-3-15-schemas-as-a-whole.md#element_declarations)
has a [{substitution group affiliation}](013-3-3-element-declarations.md#class_exemplar) in **P**, then that element is also in **P** itself.

**HEAD**'s actual [·substitution
group·](019-continuation-19.md#key-eq) is then the set consisting of each member of **P**
such that **all** of the following must be true:

1 Its [{abstract}](013-3-3-element-declarations.md#e-abstract) is *false*.

2 It is validly substitutable for **HEAD** subject to
**HEAD**'s [{disallowed substitutions}](013-3-3-element-declarations.md#e-exact) as the
blocking constraint, as defined in [Substitution Group OK (Transitive) (§3.3.6)](019-continuation-19.md#cos-equiv-derived-ok-rec).

