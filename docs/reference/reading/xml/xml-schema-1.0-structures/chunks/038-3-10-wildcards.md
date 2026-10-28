# 3.10 Wildcards

Source: [XML Schema Part 1: Structures](https://www.w3.org/TR/2004/REC-xmlschema-1-20041028/) · 2nd edition, 2004-10-28.

[Document index](../README.md) · [Topic guide](../../../../README.md) · [Publisher](https://www.w3.org/TR/2004/REC-xmlschema-1-20041028/).

HTML normalized to Markdown. Complex tables and mathematical markup retain HTML; illustrations are stored locally.

[Previous](037-continuation-37.md) · [Next](039-3-10-4-wildcard-validation-rules.md)

**Note:**This clause is in principle more restrictive than absolutely
necessary, but in practice will cover all the likely cases, and is much easier
to specify than the fully general version.

**Note:**This case allows the "unfolding" of iterated disjunctions
into sequences. It may be particularly useful when the disjunction is an
implicit one arising from the use of substitution groups.

<a id="cos-group-emptiable"></a>

**Schema Component Constraint: Particle Emptiable**  

<a id="cd-emptiable"></a>

[Definition:]For a particle to be
**emptiable**
**one** of the following must be true:

1 Its [{min occurs}](035-3-9-particles.md#p-min_occurs) is `0`.

2 Its [{term}](035-3-9-particles.md#term) is a group and the minimum part of the
effective total range of that group, as defined by [Effective Total Range (all and sequence) (§3.8.6)](034-3-8-5-model-group-information-set-contributions.md#cos-seq-range) (if
the group is *all* or *sequence*) or
[Effective Total Range (choice) (§3.8.6)](034-3-8-5-model-group-information-set-contributions.md#cos-choice-range) (if it is *choice*), is `0`.

<a id="Wildcards"></a>

### 3.10 Wildcards

3.10.1 [The Wildcard Schema Component](038-3-10-wildcards.md#Wildcard_details)  
        3.10.2 [XML Representation of Wildcard Schema Components](038-3-10-wildcards.md#declare-openness)  
        3.10.3 [Constraints on XML Representations of Wildcards](038-3-10-wildcards.md#d0e12788)  
        3.10.4 [Wildcard Validation Rules](039-3-10-4-wildcard-validation-rules.md#d0e12802)  
        3.10.5 [Wildcard Information Set Contributions](039-3-10-4-wildcard-validation-rules.md#d0e12923)  
        3.10.6 [Constraints on Wildcard Schema Components](039-3-10-4-wildcard-validation-rules.md#coss-wildcard)

In order to exploit the full potential for extensibility offered by XML
plus namespaces, more provision is needed than DTDs allow for targeted flexibility in content
models and attribute declarations. A wildcard provides for [·validation·](004-2-1-overview-of-xml-schema.md#key-vn) of
attribute and element information items dependent on their namespace
name, but independently of their local name.

Example



```
<xs:any processContents="skip"/>

<xs:any namespace="##other" processContents="lax"/>

<xs:any namespace="http://www.w3.org/1999/XSL/Transform"/>

<xs:any namespace="##targetNamespace"/>

<xs:anyAttribute namespace="http://www.w3.org/XML/1998/namespace"/>
```




XML representations of the four basic types of wildcard, plus one attribute wildcard.

<a id="Wildcard_details"></a>

#### 3.10.1 The Wildcard Schema Component

The wildcard schema component has the following properties:

Schema Component: [Wildcard](005-2-2-1-3-complex-type-definition.md#Wildcard)

<a id="namespace_constraint"></a>

{namespace constraint}
:   One of *any*; a pair of *not* and a namespace name
    or [·absent·](008-3-schema-component-details.md#key-null); or a set whose
    members are either namespace names or [·absent·](008-3-schema-component-details.md#key-null).

<a id="process_contents"></a>

{process contents}
:   One of *skip*, *lax* or *strict*.

<a id="w-annotation"></a>

{annotation}
:   Optional. An annotation.

[{namespace constraint}](038-3-10-wildcards.md#namespace_constraint) provides for [·validation·](004-2-1-overview-of-xml-schema.md#key-vn) of attribute and element items that:

1. (*any*) have any namespace or are not namespace-qualified;
2. (*not* and a namespace name) are namespace-qualified with a namespace
   other than the specified namespace name;
3. (*not* and [·absent·](008-3-schema-component-details.md#key-null)) are namespace-qualified;
4. (a set whose
   members are either namespace names or [·absent·](008-3-schema-component-details.md#key-null)) have any of the
   specified namespaces and/or, if [·absent·](008-3-schema-component-details.md#key-null) is included in the set, are unqualified.

[{process contents}](038-3-10-wildcards.md#process_contents) controls the impact on [·assessment·](004-2-1-overview-of-xml-schema.md#key-va)
of the information items allowed by wildcards, as follows:

strict
:   There must be a top-level declaration for the item available, or the item
    must have an `xsi:type`, and the item
    must be [·valid·](004-2-1-overview-of-xml-schema.md#key-vn) as appropriate.

skip
:   No constraints at all: the item must simply be well-formed XML.

lax
:   If the item has a uniquely
    determined declaration available, it must be [·valid·](004-2-1-overview-of-xml-schema.md#key-vn) with respect to
    that definition, that is, [·validate·](004-2-1-overview-of-xml-schema.md#key-vn)
    if you
    can, don't worry if you can't.

See [Annotations (§3.13)](044-3-13-annotations.md#cAnnotations) for information on the role of the
[{annotation}](038-3-10-wildcards.md#w-annotation) property.

<a id="declare-openness"></a>

#### 3.10.2 XML Representation of Wildcard Schema Components

The XML representation for a wildcard schema component is an
[<any>](038-3-10-wildcards.md#element-any) or [<anyAttribute>](022-continuation-22.md#element-anyAttribute) element information item. The correspondences between the
properties of an [<any>](038-3-10-wildcards.md#element-any) information item and
properties of the components it corresponds to are as follows (see [<complexType>](021-3-4-2-xml-representation-of-complex-type-definitions.md#element-complexType) and [<attributeGroup>](030-3-5-attributeuses.md#element-attributeGroup) for the correspondences for [<anyAttribute>](022-continuation-22.md#element-anyAttribute)):

XML Representation Summary: `any` Element Information Item

<a id="element-any"></a>

<any  
  id = [ID](https://www.w3.org/TR/2004/REC-xmlschema-2-20041028/datatypes.html#ID)  
  maxOccurs =
([nonNegativeInteger](https://www.w3.org/TR/2004/REC-xmlschema-2-20041028/datatypes.html#nonNegativeInteger) | unbounded)
 : 1  
  minOccurs = [nonNegativeInteger](https://www.w3.org/TR/2004/REC-xmlschema-2-20041028/datatypes.html#nonNegativeInteger) : 1  
  namespace =
((##any | ##other) | List of
([anyURI](https://www.w3.org/TR/2004/REC-xmlschema-2-20041028/datatypes.html#anyURI) | (##targetNamespace | ##local))
)
 : ##any  
  processContents = (lax | skip | strict) : strict  
  *{any attributes with non-schema namespace . . .}*>  
*Content:* ([annotation](044-3-13-annotations.md#element-annotation)?)  
</any>

A particle containing a wildcard, with properties as follows (unless `minOccurs=maxOccurs=0`, in which case the item
corresponds to no component at all):

<table class="reprcomp"><thead><tr><th><a href="035-3-9-particles.md#Particle_details">Particle</a><strong> Schema Component</strong></th></tr></thead><tbody><tr><td><table border="0" cellpadding="3"><thead><tr><th align="left">Property</th><th align="left">Representation</th></tr></thead><tbody valign="top"><tr valign="top"><td><a class="propref" href="035-3-9-particles.md#p-min_occurs">{min occurs}</a></td><td>The <a class="termref" href="008-3-schema-component-details.md#key-vv"><span class="arrow">·</span>actual value<span class="arrow">·</span></a> of the <code>minOccurs</code>
<a class="xpropref" href="https://www.w3.org/TR/xml-infoset/#infoitem.element">[attribute]</a>, if present, otherwise <code>1</code>.</td></tr><tr valign="top"><td><a class="propref" href="035-3-9-particles.md#p-max_occurs">{max occurs}</a></td><td><i>unbounded</i>, if the <code>maxOccurs</code>
<a class="xpropref" href="https://www.w3.org/TR/xml-infoset/#infoitem.element">[attribute]</a> equals <i>unbounded</i>, otherwise the <a class="termref" href="008-3-schema-component-details.md#key-vv"><span class="arrow">·</span>actual value<span class="arrow">·</span></a> of the <code>maxOccurs</code>
<a class="xpropref" href="https://www.w3.org/TR/xml-infoset/#infoitem.element">[attribute]</a>, if present, otherwise <code>1</code>.</td></tr><tr valign="top"><td><a class="propref" href="035-3-9-particles.md#term">{term}</a></td><td>A wildcard as given below:</td></tr></tbody></table></td></tr></tbody></table>

<table class="reprcomp"><thead><tr><th><a href="038-3-10-wildcards.md#Wildcard_details">Wildcard</a><strong> Schema Component</strong></th></tr></thead><tbody><tr><td><table border="0" cellpadding="3"><thead><tr><th align="left">Property</th><th align="left">Representation</th></tr></thead><tbody valign="top"><tr valign="top"><td><a class="propref" href="038-3-10-wildcards.md#namespace_constraint">{namespace constraint}</a></td><td>Dependent on the <a class="termref" href="008-3-schema-component-details.md#key-vv"><span class="arrow">·</span>actual value<span class="arrow">·</span></a> of the
<code>namespace</code> <a class="xpropref" href="https://www.w3.org/TR/xml-infoset/#infoitem.element">[attribute]</a>: if absent, then <i>any</i>, otherwise as follows:<dl><dt class="label">##any</dt><dd><i>any</i></dd><dt class="label">##other</dt><dd>a pair of <i>not</i> and the <a class="termref" href="008-3-schema-component-details.md#key-vv"><span class="arrow">·</span>actual value<span class="arrow">·</span></a> of the <code>targetNamespace</code> <a class="xpropref" href="https://www.w3.org/TR/xml-infoset/#infoitem.element">[attribute]</a> of the <a class="eltref" href="049-3-15-schemas-as-a-whole.md#element-schema">&lt;schema&gt;</a> ancestor
element information item if present, otherwise <a class="termref" href="008-3-schema-component-details.md#key-null"><span class="arrow">·</span>absent<span class="arrow">·</span></a>.</dd><dt class="label"><em>otherwise</em></dt><dd>a set whose members are namespace names corresponding to the
space-delimited substrings of the string, except
     <div class="constraintlist"><div class="clnumber">1 if one such
substring is <code>##targetNamespace</code>, the corresponding member is the <a class="termref" href="008-3-schema-component-details.md#key-vv"><span class="arrow">·</span>actual value<span class="arrow">·</span></a> of the <code>targetNamespace</code> <a class="xpropref" href="https://www.w3.org/TR/xml-infoset/#infoitem.element">[attribute]</a> of the <a class="eltref" href="049-3-15-schemas-as-a-whole.md#element-schema">&lt;schema&gt;</a> ancestor
element information item if present, otherwise <a class="termref" href="008-3-schema-component-details.md#key-null"><span class="arrow">·</span>absent<span class="arrow">·</span></a>.</div><div class="clnumber">2 if one such
substring is <code>##local</code>, the corresponding member is <a class="termref" href="008-3-schema-component-details.md#key-null"><span class="arrow">·</span>absent<span class="arrow">·</span></a>.</div></div>
</dd></dl>
</td></tr><tr valign="top"><td><a class="propref" href="038-3-10-wildcards.md#process_contents">{process contents}</a></td><td>The <a class="termref" href="008-3-schema-component-details.md#key-vv"><span class="arrow">·</span>actual value<span class="arrow">·</span></a> of the
<code>processContents</code> <a class="xpropref" href="https://www.w3.org/TR/xml-infoset/#infoitem.element">[attribute]</a>, if present, otherwise <i>strict</i>.</td></tr><tr valign="top"><td><a class="propref" href="038-3-10-wildcards.md#w-annotation">{annotation}</a></td><td>The annotation corresponding to the <a class="eltref" href="044-3-13-annotations.md#element-annotation">&lt;annotation&gt;</a> element information item in the
<a class="xpropref" href="https://www.w3.org/TR/xml-infoset/#infoitem.element">[children]</a>, if present, otherwise <a class="termref" href="008-3-schema-component-details.md#key-null"><span class="arrow">·</span>absent<span class="arrow">·</span></a>.</td></tr></tbody></table></td></tr></tbody></table>

Wildcards are subject to the same ambiguity constraints
([Unique Particle Attribution (§3.8.6)](034-3-8-5-model-group-information-set-contributions.md#cos-nonambig)) as other
content model particles: If an instance element could match either an explicit
particle and a wildcard, or one of two wildcards, within the content model of a
type, that model is in error.

<a id="d0e12788"></a>

#### 3.10.3 Constraints on XML Representations of Wildcards

<a id="src-wildcard"></a>

**Schema Representation Constraint: Wildcard Representation OK**  

