# 4.1.2.3 Derivation by union

Source: [XML Schema Part 2: Datatypes](https://www.w3.org/TR/2004/REC-xmlschema-2-20041028/) · 2nd edition, 2004-10-28.

[Document index](../README.md) · [Topic guide](../../../../README.md) · [Publisher](https://www.w3.org/TR/2004/REC-xmlschema-2-20041028/).

HTML normalized to Markdown. Complex tables and mathematical markup retain HTML; illustrations are stored locally.

[Previous](019-4-1-2-1-derivation-by-restriction.md) · [Next](021-continuation-21.md)

regardless of the [·constraining facet·](004-2-3-lexical-space.md#dt-constraining-facet)s that are applicable
to the [·atomic·](004-2-3-lexical-space.md#dt-atomic) datatype that serves as the
[·itemType·](004-2-3-lexical-space.md#dt-itemType) of the [·list·](004-2-3-lexical-space.md#dt-list).

For each of [·length·](024-4-2-5-numeric.md#dt-length), [·maxLength·](026-4-3-3-maxlength.md#dt-maxLength)
and [·minLength·](025-4-3-1-4-constraints-on-length-schema-components.md#dt-minLength), the
*unit of length* is measured in number of list items.
The value of [·whiteSpace·](027-4-3-4-3-constraints-on-xml-representation-of-pattern.md#dt-whiteSpace)
is fixed to the value *collapse*.

<a id="derivation-by-union"></a>

##### 4.1.2.3 Derivation by union

XML Representation Summary: `union` Element Information Item

<a id="element-union"></a>

<union  
  id = [ID](014-3-3-4-1-constraining-facets.md#ID)  
  memberTypes = List of [QName](012-3-2-15-hexbinary.md#QName)  
  *{any attributes with non-schema namespace . . .}*>  
*Content:* ([annotation](https://www.w3.org/TR/2004/REC-xmlschema-1-20041028/structures.html#element-annotation)?, [simpleType](018-4-datatype-components.md#element-simpleType)\*)  
</union>

<table class="reprcomp"><thead><tr><th><a href="018-4-datatype-components.md#dc-defn">Simple Type Definition</a><strong> Schema Component</strong></th></tr></thead><tbody><tr><td><table border="0" cellpadding="3"><thead><tr><th align="left">Property</th><th align="left">Representation</th></tr></thead><tbody valign="top"><tr valign="top"><td><a class="propref" href="018-4-datatype-components.md#defn-variety">{variety}</a></td><td>
union
</td></tr><tr valign="top"><td><a class="propref" href="018-4-datatype-components.md#defn-memberTypes">{member type definitions}</a></td><td>
The sequence of <a href="018-4-datatype-components.md#dc-defn">Simple Type Definition</a> components resolved to by the
items in the <a href="https://www.w3.org/TR/2004/REC-xmlschema-1-20041028/structures.html#key-vv">actual value</a> of the
<code>memberTypes</code> <a class="xpropref" href="https://www.w3.org/TR/xml-infoset/#infoitem.attribute">[attribute]</a>, if any,
in order, followed by the <a href="018-4-datatype-components.md#dc-defn">Simple Type Definition</a> components resolved to by the
<a class="eltref" href="018-4-datatype-components.md#element-simpleType">&lt;simpleType&gt;</a> <a class="xpropref" href="https://www.w3.org/TR/xml-infoset/#infoitem.element">[children]</a>, if any, in order.
If <a class="propref" href="018-4-datatype-components.md#defn-variety">{variety}</a> is <em>union</em> for
any <a href="018-4-datatype-components.md#dc-defn">Simple Type Definition</a> components resolved to above, then
the <a href="018-4-datatype-components.md#dc-defn">Simple Type Definition</a> is replaced by its
<a class="propref" href="018-4-datatype-components.md#defn-memberTypes">{member type definitions}</a>.
</td></tr></tbody></table></td></tr></tbody></table>

A [·union·](004-2-3-lexical-space.md#dt-union) datatype can be [·derived·](005-2-5-1-3-union-datatypes.md#dt-derived)
from one or more [·atomic·](004-2-3-lexical-space.md#dt-atomic), [·list·](004-2-3-lexical-space.md#dt-list) or
other [·union·](004-2-3-lexical-space.md#dt-union) datatypes, known as the [·memberTypes·](005-2-5-1-3-union-datatypes.md#dt-memberTypes)
of that [·union·](004-2-3-lexical-space.md#dt-union) datatype.

Example

As an example, taken from a typical display oriented text markup language,
one might want to express font sizes as an integer between 8 and 72, or with
one of the tokens "small", "medium" or "large". The [·union·](004-2-3-lexical-space.md#dt-union)
type definition below would accomplish that.



```

<xsd:attribute name="size">
  <xsd:simpleType>
    <xsd:union>
      <xsd:simpleType>
        <xsd:restriction base="xsd:positiveInteger">
          <xsd:minInclusive value="8"/>
          <xsd:maxInclusive value="72"/>
        </xsd:restriction>
      </xsd:simpleType>
      <xsd:simpleType>
        <xsd:restriction base="xsd:NMTOKEN">
          <xsd:enumeration value="small"/>
          <xsd:enumeration value="medium"/>
          <xsd:enumeration value="large"/>
        </xsd:restriction>
      </xsd:simpleType>
    </xsd:union>
  </xsd:simpleType>
</xsd:attribute>
```






```

<p>
<font size='large'>A header</font>
</p>
<p>
<font size='12'>this is a test</font>
</p>
```




As mentioned in [Union datatypes (§2.5.1.3)](005-2-5-1-3-union-datatypes.md#union-datatypes),
when a datatype is [·derived·](005-2-5-1-3-union-datatypes.md#dt-derived) from a
[·union·](004-2-3-lexical-space.md#dt-union) datatype, the only following
[·constraining facet·](004-2-3-lexical-space.md#dt-constraining-facet)s can be used:

- [·pattern·](026-4-3-3-maxlength.md#dt-pattern)
- [·enumeration·](027-4-3-4-3-constraints-on-xml-representation-of-pattern.md#dt-enumeration)

regardless of the [·constraining facet·](004-2-3-lexical-space.md#dt-constraining-facet)s that are
applicable to the datatypes that participate in the [·union·](004-2-3-lexical-space.md#dt-union)

<a id="defn-rep-constr"></a>

#### 4.1.3 Constraints on XML Representation of Simple Type Definition

<a id="src-single-facet-value"></a>

**Schema Representation Constraint: Single Facet Value**  

Unless otherwise specifically allowed by this specification
([Multiple patterns (§4.3.4.3)](027-4-3-4-3-constraints-on-xml-representation-of-pattern.md#src-multiple-patterns) and
[Multiple enumerations (§4.3.5.3)](027-4-3-4-3-constraints-on-xml-representation-of-pattern.md#src-multiple-enumerations)) any given
[·constraining facet·](004-2-3-lexical-space.md#dt-constraining-facet) can only be specifed once within
a single derivation step.

<a id="src-list-itemType-or-simpleType"></a>

**Schema Representation Constraint: itemType attribute or simpleType child**  

Either the `itemType` [[attribute]](https://www.w3.org/TR/xml-infoset/#infoitem.attribute) or the
[<simpleType>](018-4-datatype-components.md#element-simpleType) [[child]](https://www.w3.org/TR/xml-infoset/#infoitem.element) of the [<list>](019-4-1-2-1-derivation-by-restriction.md#element-list) element
must be present, but not both.

<a id="src-restriction-base-or-simpleType"></a>

**Schema Representation Constraint: base attribute or simpleType child**  

Either the `base` [[attribute]](https://www.w3.org/TR/xml-infoset/#infoitem.attribute) or the
`simpleType` [[child]](https://www.w3.org/TR/xml-infoset/#infoitem.element) of the [<restriction>](019-4-1-2-1-derivation-by-restriction.md#element-restriction)
element must be present, but not both.

<a id="src-union-memberTypes-or-simpleTypes"></a>

**Schema Representation Constraint: memberTypes attribute or simpleType children**  

Either the `memberTypes` [[attribute]](https://www.w3.org/TR/xml-infoset/#infoitem.attribute) of the [<union>](020-4-1-2-3-derivation-by-union.md#element-union)
element must be non-empty or
there must be at least one `simpleType` [[child]](https://www.w3.org/TR/xml-infoset/#infoitem.element).

<a id="defn-validation-rules"></a>

#### 4.1.4 Simple Type Definition Validation Rules

<a id="cvc-facet-valid"></a>

**Validation Rule: Facet Valid**  

A value in a [·value space·](003-1-4-terminology.md#dt-value-space) is facet-valid with
respect to a [·constraining facet·](004-2-3-lexical-space.md#dt-constraining-facet) component if:

1
the value is facet-valid with respect to the particular
[·constraining facet·](004-2-3-lexical-space.md#dt-constraining-facet) as specified below.

<a id="cvc-datatype-valid"></a>

**Validation Rule: Datatype Valid**  

A string is datatype-valid with respect to a datatype definition if:

1
it [·match·](003-1-4-terminology.md#dt-match)es a literal in the
[·lexical space·](004-2-3-lexical-space.md#dt-lexical-space) of the datatype, determined as follows:

1.1
if [·pattern·](026-4-3-3-maxlength.md#dt-pattern) is a member of [{facets}](018-4-datatype-components.md#defn-facets),
then the string must be [pattern valid (§4.3.4.4)](027-4-3-4-3-constraints-on-xml-representation-of-pattern.md#cvc-pattern-valid);

1.2
if [·pattern·](026-4-3-3-maxlength.md#dt-pattern) is not a member of [{facets}](018-4-datatype-components.md#defn-facets),
then

1.2.1
if [{variety}](018-4-datatype-components.md#defn-variety) is [·atomic·](004-2-3-lexical-space.md#dt-atomic) then
the string must [·match·](003-1-4-terminology.md#dt-match) a literal in the
[·lexical space·](004-2-3-lexical-space.md#dt-lexical-space) of [{base type definition}](018-4-datatype-components.md#defn-basetype)

1.2.2
if [{variety}](018-4-datatype-components.md#defn-variety) is [·list·](004-2-3-lexical-space.md#dt-list) then the string must
be a sequence of space-separated tokens, each of which [·match·](003-1-4-terminology.md#dt-match)es a literal in the [·lexical space·](004-2-3-lexical-space.md#dt-lexical-space)
of [{item type definition}](018-4-datatype-components.md#defn-itemType)

1.2.3
if [{variety}](018-4-datatype-components.md#defn-variety) is [·union·](004-2-3-lexical-space.md#dt-union) then
the string must [·match·](003-1-4-terminology.md#dt-match) a literal in the
[·lexical space·](004-2-3-lexical-space.md#dt-lexical-space) of at least one member of
[{member type definitions}](018-4-datatype-components.md#defn-memberTypes)

2
the value denoted by the literal [·match·](003-1-4-terminology.md#dt-match)ed in the previous step
is a member of the [·value space·](003-1-4-terminology.md#dt-value-space) of the datatype, as determined
by it being [Facet Valid (§4.1.4)](020-4-1-2-3-derivation-by-union.md#cvc-facet-valid)
with respect to each member of [{facets}](018-4-datatype-components.md#defn-facets) (except
for [·pattern·](026-4-3-3-maxlength.md#dt-pattern)).

<a id="defn-coss"></a>

#### 4.1.5 Constraints on Simple Type Definition Schema Components

<a id="cos-applicable-facets"></a>

**Schema Component Constraint: applicable facets**  

The [·constraining facet·](004-2-3-lexical-space.md#dt-constraining-facet)s which are allowed
to be members of [{facets}](018-4-datatype-components.md#defn-facets) are dependent on
[{base type definition}](018-4-datatype-components.md#defn-basetype) as specified in the following table:

