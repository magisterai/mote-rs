# 3.9 Particles

Source: [XML Schema Part 1: Structures](https://www.w3.org/TR/2004/REC-xmlschema-1-20041028/) · 2nd edition, 2004-10-28.

[Document index](../README.md) · [Topic guide](../../../../README.md) · [Publisher](https://www.w3.org/TR/2004/REC-xmlschema-1-20041028/).

HTML normalized to Markdown. Complex tables and mathematical markup retain HTML; illustrations are stored locally.

[Previous](034-3-8-5-model-group-information-set-contributions.md) · [Next](036-3-9-5-particle-information-set-contributions.md)

maximum
:   *unbounded* if the [{max occurs}](035-3-9-particles.md#p-max_occurs) of any wildcard or element
    declaration particle in the group's [{particles}](033-3-7-6-constraints-on-model-group-definition-schema-components.md#particles) or the maximum
    part of the effective total range of any of the group particles in the group's
    [{particles}](033-3-7-6-constraints-on-model-group-definition-schema-components.md#particles) is *unbounded*, or if any of those is non-zero
    and the [{max occurs}](035-3-9-particles.md#p-max_occurs) of the particle itself is *unbounded*,
    otherwise the product of the particle's [{max occurs}](035-3-9-particles.md#p-max_occurs) and the
    maximum of the [{max occurs}](035-3-9-particles.md#p-max_occurs) of every wildcard or element
    declaration particle in the group's [{particles}](033-3-7-6-constraints-on-model-group-definition-schema-components.md#particles) and the maximum
    part of the effective total range of each of the group particles in the group's [{particles}](033-3-7-6-constraints-on-model-group-definition-schema-components.md#particles) (or `0` if there are no [{particles}](033-3-7-6-constraints-on-model-group-definition-schema-components.md#particles)).

<a id="cParticles"></a>

### 3.9 Particles

3.9.1 [The Particle Schema Component](035-3-9-particles.md#Particle_details)  
        3.9.2 [XML Representation of Particle Components](035-3-9-particles.md#d0e11005)  
        3.9.3 [Constraints on XML Representations of Particles](035-3-9-particles.md#d0e11037)  
        3.9.4 [Particle Validation Rules](035-3-9-particles.md#d0e11042)  
        3.9.5 [Particle Information Set Contributions](036-3-9-5-particle-information-set-contributions.md#d0e11288)  
        3.9.6 [Constraints on Particle Schema Components](036-3-9-5-particle-information-set-contributions.md#coss-particle)

As described in [Model Groups (§3.8)](033-3-7-6-constraints-on-model-group-definition-schema-components.md#Model_Groups), particles contribute to the definition
of content models.

Example



```
<xs:element ref="egg" minOccurs="12" maxOccurs="12"/>

<xs:group ref="omelette" minOccurs="0"/>

<xs:any maxOccurs="unbounded"/>
     
```




XML representations which all involve particles, illustrating some of
the possibilities for controlling occurrence.

<a id="Particle_details"></a>

#### 3.9.1 The Particle Schema Component

The particle schema component has the following properties:

Schema Component: [Particle](005-2-2-1-3-complex-type-definition.md#Particle)

<a id="p-min_occurs"></a>

{min occurs}
:   A non-negative
    integer.

<a id="p-max_occurs"></a>

{max occurs}
:   Either a non-negative integer
    or *unbounded*.

<a id="term"></a>

{term}
:   One of a model group, a wildcard, or an element declaration.

In general, multiple element
information item [[children]](https://www.w3.org/TR/xml-infoset/#infoitem.element), possibly with intervening character [[children]](https://www.w3.org/TR/xml-infoset/#infoitem.element) if the content type
is *mixed*, can be [·validated·](004-2-1-overview-of-xml-schema.md#key-vn) with
respect to a single particle. When the [{term}](035-3-9-particles.md#term) is an element
declaration or wildcard, [{min occurs}](035-3-9-particles.md#p-min_occurs) determines the minimum number of such element [[children]](https://www.w3.org/TR/xml-infoset/#infoitem.element) that can occur. The number of such children must be greater than or equal to [{min occurs}](035-3-9-particles.md#p-min_occurs). If [{min occurs}](035-3-9-particles.md#p-min_occurs) is *0*, then occurrence of such children is optional.

Again, when the [{term}](035-3-9-particles.md#term) is an element
declaration or wildcard, the number of such element [[children]](https://www.w3.org/TR/xml-infoset/#infoitem.element) must be less than or equal to any numeric specification of
[{max occurs}](035-3-9-particles.md#p-max_occurs); if [{max occurs}](035-3-9-particles.md#p-max_occurs) is *unbounded*, then there is no
upper bound on the number of such children.

When the [{term}](035-3-9-particles.md#term) is a model group, the permitted
occurrence range is determined by a combination of [{min occurs}](035-3-9-particles.md#p-min_occurs) and [{max occurs}](035-3-9-particles.md#p-max_occurs) and the occurrence ranges of the [{term}](035-3-9-particles.md#term)'s [{particles}](033-3-7-6-constraints-on-model-group-definition-schema-components.md#particles).

<a id="d0e11005"></a>

#### 3.9.2 XML Representation of Particle Components

Particles correspond to all three elements ([<element>](013-3-3-element-declarations.md#element-element) not immediately within [<schema>](049-3-15-schemas-as-a-whole.md#element-schema), [<group>](032-3-7-model-group-definitions.md#element-group) not immediately within [<schema>](049-3-15-schemas-as-a-whole.md#element-schema) and [<any>](038-3-10-wildcards.md#element-any)) which allow `minOccurs` and `maxOccurs` attributes. These in turn correspond to
*two* components in each case, a particle and its [{term}](035-3-9-particles.md#term). The appropriate mapping is described in [XML Representation of Element Declaration Schema Components (§3.3.2)](013-3-3-element-declarations.md#declare-element), [XML Representation of Model Group Schema Components (§3.8.2)](033-3-7-6-constraints-on-model-group-definition-schema-components.md#declare-contentModel) and [XML Representation of Wildcard Schema Components (§3.10.2)](038-3-10-wildcards.md#declare-openness) respectively.

<a id="d0e11037"></a>

#### 3.9.3 Constraints on XML Representations of Particles

None as such.

<a id="d0e11042"></a>

#### 3.9.4 Particle Validation Rules

<a id="cvc-particle"></a>

**Validation Rule: Element Sequence Locally Valid (Particle)**  

For a sequence (possibly empty) of element information items to be
locally [·valid·](004-2-1-overview-of-xml-schema.md#key-vn)
with respect to a particle
the appropriate **case** among the following
must be true:

1

<a id="c-pw"></a>

**If** the [{term}](035-3-9-particles.md#term) is a wildcard, **then** 
**all** of the following must be true:

1.1 The length of the sequence must be greater than or equal to the [{min occurs}](035-3-9-particles.md#p-min_occurs).

1.2 If [{max occurs}](035-3-9-particles.md#p-max_occurs) is a number, the length of the sequence must be less than or equal to the [{max occurs}](035-3-9-particles.md#p-max_occurs).

1.3 Each element
information item in the sequence must be [·valid·](004-2-1-overview-of-xml-schema.md#key-vn) with respect to the wildcard as defined by [Item Valid (Wildcard) (§3.10.4)](039-3-10-4-wildcard-validation-rules.md#cvc-wildcard).

2

<a id="c-cdde"></a>

**If** the [{term}](035-3-9-particles.md#term) is an element declaration, **then** 
**all** of the following must be true:

2.1 The length of the sequence must be greater than or equal to the [{min occurs}](035-3-9-particles.md#p-min_occurs).

2.2 If [{max occurs}](035-3-9-particles.md#p-max_occurs) is a number, the length of the sequence must be less than or equal to the [{max occurs}](035-3-9-particles.md#p-max_occurs).

2.3 For each element
information item in the sequence
**one** of the following must be true:

2.3.1 The element declaration is local (i.e. its
[{scope}](013-3-3-element-declarations.md#e-scope) must not be *global*), its [{abstract}](013-3-3-element-declarations.md#e-abstract) is *false*, the element information
item's [[namespace name]](https://www.w3.org/TR/xml-infoset/#infoitem.element) is identical to the element declaration's [{target namespace}](013-3-3-element-declarations.md#e-target_namespace) (where an [·absent·](008-3-schema-component-details.md#key-null) [{target namespace}](013-3-3-element-declarations.md#e-target_namespace) is taken to be identical to a [[namespace name]](https://www.w3.org/TR/xml-infoset/#infoitem.element) with no value) and the element information
item's [[local
name]](https://www.w3.org/TR/xml-infoset/#infoitem.element) matches the element declaration's [{name}](013-3-3-element-declarations.md#e-name).In this case the element declaration is the [·context-determined declaration·](011-3-2-4-attribute-declaration-validation-rules.md#key-dd) for the element information item with respect to [Schema-Validity Assessment (Element) (§3.3.4)](017-3-3-5-element-declaration-information-set-contributions.md#cvc-assess-elt) and [Assessment Outcome (Element) (§3.3.5)](017-3-3-5-element-declaration-information-set-contributions.md#sic-e-outcome).

2.3.2 The element declaration is top-level (i.e. its
[{scope}](013-3-3-element-declarations.md#e-scope) is *global*), [{abstract}](013-3-3-element-declarations.md#e-abstract) is *false*, the element information
item's [[namespace name]](https://www.w3.org/TR/xml-infoset/#infoitem.element) is identical to the element declaration's [{target namespace}](013-3-3-element-declarations.md#e-target_namespace) (where an [·absent·](008-3-schema-component-details.md#key-null) [{target namespace}](013-3-3-element-declarations.md#e-target_namespace) is taken to be identical to a [[namespace name]](https://www.w3.org/TR/xml-infoset/#infoitem.element) with no value) and the element information
item's [[local
name]](https://www.w3.org/TR/xml-infoset/#infoitem.element) matches the element declaration's [{name}](013-3-3-element-declarations.md#e-name).In this case the element declaration is the [·context-determined declaration·](011-3-2-4-attribute-declaration-validation-rules.md#key-dd) for the element information item with respect to [Schema-Validity Assessment (Element) (§3.3.4)](017-3-3-5-element-declaration-information-set-contributions.md#cvc-assess-elt) and [Assessment Outcome (Element) (§3.3.5)](017-3-3-5-element-declaration-information-set-contributions.md#sic-e-outcome).

2.3.3

<a id="c-psg"></a>

The element declaration is top-level (i.e. its
[{scope}](013-3-3-element-declarations.md#e-scope) is *global*), its [{disallowed substitutions}](013-3-3-element-declarations.md#e-exact) does not contain *substitution*,
the [[local
]](https://www.w3.org/TR/xml-infoset/#infoitem.element) and [[namespace name]](https://www.w3.org/TR/xml-infoset/#infoitem.element) of the element information item resolve to an element declaration, as
defined in [QName resolution (Instance) (§3.15.4)](050-3-15-2-1-references-to-schema-components.md#cvc-resolve-instance) -- 

<a id="key-eqd"></a>

[Definition:]call this declaration the **substituting declaration** and the [·substituting declaration·](035-3-9-particles.md#key-eqd) together with the particle's element declaration's [{disallowed substitutions}](013-3-3-element-declarations.md#e-exact) is validly substitutable for the particle's element declaration as defined in [Substitution Group OK (Transitive) (§3.3.6)](019-continuation-19.md#cos-equiv-derived-ok-rec).In this case the [·substituting declaration·](035-3-9-particles.md#key-eqd) is the [·context-determined declaration·](011-3-2-4-attribute-declaration-validation-rules.md#key-dd) for the element information item with respect to [Schema-Validity Assessment (Element) (§3.3.4)](017-3-3-5-element-declaration-information-set-contributions.md#cvc-assess-elt) and [Assessment Outcome (Element) (§3.3.5)](017-3-3-5-element-declaration-information-set-contributions.md#sic-e-outcome).

3 **If** the [{term}](035-3-9-particles.md#term) is a model group, **then** 
**all** of the following must be true:

3.1 There is a [·partition·](034-3-8-5-model-group-information-set-contributions.md#key-partition) of the sequence into `n` sub-sequences such that `n` is greater than or equal to [{min occurs}](035-3-9-particles.md#p-min_occurs).

3.2 If [{max occurs}](035-3-9-particles.md#p-max_occurs) is a number, `n`
must be less than or equal to [{max occurs}](035-3-9-particles.md#p-max_occurs).

3.3 Each sub-sequence in the [·partition·](034-3-8-5-model-group-information-set-contributions.md#key-partition) is [·valid·](004-2-1-overview-of-xml-schema.md#key-vn) with respect to that model group as defined in [Element Sequence Valid (§3.8.4)](033-3-7-6-constraints-on-model-group-definition-schema-components.md#cvc-model-group).

