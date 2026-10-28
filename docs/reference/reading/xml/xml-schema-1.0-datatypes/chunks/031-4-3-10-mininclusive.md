# 4.3.10 minInclusive

Source: [XML Schema Part 2: Datatypes](https://www.w3.org/TR/2004/REC-xmlschema-2-20041028/) · 2nd edition, 2004-10-28.

[Document index](../README.md) · [Topic guide](../../../../README.md) · [Publisher](https://www.w3.org/TR/2004/REC-xmlschema-2-20041028/).

HTML normalized to Markdown. Complex tables and mathematical markup retain HTML; illustrations are stored locally.

[Previous](030-4-3-9-minexclusive.md) · [Next](032-4-3-11-3-totaldigits-validation-rules.md)

4
[maxExclusive](029-4-3-7-3-maxinclusive-validation-rules.md#dc-maxExclusive) is among the members of
[{facets}](018-4-datatype-components.md#defn-facets) of [{base type definition}](018-4-datatype-components.md#defn-basetype)
and [{value}](029-4-3-7-3-maxinclusive-validation-rules.md#maxExclusive-value) is
greater than or equal to the [{value}](029-4-3-7-3-maxinclusive-validation-rules.md#maxExclusive-value) of the parent
[maxExclusive](029-4-3-7-3-maxinclusive-validation-rules.md#dc-maxExclusive)

<a id="rf-minInclusive"></a>

#### 4.3.10 minInclusive

<a id="dt-minInclusive"></a>

[Definition:]
**minInclusive** is the [·inclusive lower bound·](023-4-2-3-bounded.md#dt-inclusive-lower-bound)
of the [·value space·](003-1-4-terminology.md#dt-value-space) for a datatype with the
[·ordered·](022-4-1-6-simple-type-definition-for-anysimpletype.md#dt-ordered) property. The value of
**minInclusive**
 [·must·](003-1-4-terminology.md#dt-must) be in the [·value space·](003-1-4-terminology.md#dt-value-space) of the
[·base type·](005-2-5-1-3-union-datatypes.md#dt-basetype).

[·minInclusive·](031-4-3-10-mininclusive.md#dt-minInclusive) provides for:

- Constraining a [·value space·](003-1-4-terminology.md#dt-value-space) to values
  with a specific [·inclusive lower bound·](023-4-2-3-bounded.md#dt-inclusive-lower-bound).

Example

The following is the definition of a [·user-derived·](006-2-5-2-3-derived-by-union.md#dt-user-derived)
datatype which limits values to integers greater than or equal to
100, using [·minInclusive·](031-4-3-10-mininclusive.md#dt-minInclusive).



```
<simpleType name='one-hundred-or-more'>
  <restriction base='integer'>
    <minInclusive value='100'/>
  </restriction>
</simpleType>
```




<a id="dc-minInclusive"></a>

##### 4.3.10.1 The minInclusive Schema Component

Schema Component: [minInclusive](031-4-3-10-mininclusive.md#dt-minInclusive)

<a id="minInclusive-value"></a>

{value}
:   A value from the [·value space·](003-1-4-terminology.md#dt-value-space) of the
    [{base type definition}](018-4-datatype-components.md#defn-basetype).

<a id="minInclusive-fixed"></a>

{fixed}
:   A [boolean](006-2-5-2-3-derived-by-union.md#boolean).

<a id="minInclusive-annotation"></a>

{annotation}
:   Optional. An [annotation](https://www.w3.org/TR/2004/REC-xmlschema-1-20041028/structures.html#Annotation).

If [{fixed}](031-4-3-10-mininclusive.md#minInclusive-fixed) is *true*, then types for which
the current type is the [{base type definition}](018-4-datatype-components.md#defn-basetype) cannot specify a
value for [minInclusive](031-4-3-10-mininclusive.md#dc-minInclusive) other than [{value}](031-4-3-10-mininclusive.md#minInclusive-value).

<a id="xr-minInclusive"></a>

##### 4.3.10.2 XML Representation of minInclusive Schema Components

The XML representation for a [minInclusive](031-4-3-10-mininclusive.md#dc-minInclusive) schema
component is a [<minInclusive>](031-4-3-10-mininclusive.md#element-minInclusive) element information item. The
correspondences between the properties of the information item and
properties of the component are as follows:

XML Representation Summary: `minInclusive` Element Information Item

<a id="element-minInclusive"></a>

<minInclusive  
  fixed = [boolean](006-2-5-2-3-derived-by-union.md#boolean) : false  
  id = [ID](014-3-3-4-1-constraining-facets.md#ID)  
  **value** = [anySimpleType](005-2-5-1-3-union-datatypes.md#dt-anySimpleType)  
  *{any attributes with non-schema namespace . . .}*>  
*Content:* ([annotation](https://www.w3.org/TR/2004/REC-xmlschema-1-20041028/structures.html#element-annotation)?)  
</minInclusive>

[{value}](031-4-3-10-mininclusive.md#minInclusive-value) [·must·](003-1-4-terminology.md#dt-must) be
in the [·value space·](003-1-4-terminology.md#dt-value-space) of [{base type definition}](018-4-datatype-components.md#defn-basetype).

<table class="reprcomp"><thead><tr><th><a href="031-4-3-10-mininclusive.md#dt-minInclusive">minInclusive</a><strong> Schema Component</strong></th></tr></thead><tbody><tr><td><table border="0" cellpadding="3"><thead><tr><th align="left">Property</th><th align="left">Representation</th></tr></thead><tbody valign="top"><tr valign="top"><td><a class="propref" href="031-4-3-10-mininclusive.md#minInclusive-value">{value}</a></td><td>
The <a href="https://www.w3.org/TR/2004/REC-xmlschema-1-20041028/structures.html#key-vv">actual value</a> of the <code>value</code> <a class="xpropref" href="https://www.w3.org/TR/xml-infoset/#infoitem.attribute">[attribute]</a>
</td></tr><tr valign="top"><td><a class="propref" href="031-4-3-10-mininclusive.md#minInclusive-fixed">{fixed}</a></td><td>
The <a href="https://www.w3.org/TR/2004/REC-xmlschema-1-20041028/structures.html#key-vv">actual value</a> of the <code>fixed</code> <a class="xpropref" href="https://www.w3.org/TR/xml-infoset/#infoitem.attribute">[attribute]</a>, if present, otherwise false
</td></tr><tr valign="top"><td><a class="propref" href="018-4-datatype-components.md#defn-annotation">{annotation}</a></td><td>
The annotations corresponding to all the <a class="eltref" href="https://www.w3.org/TR/2004/REC-xmlschema-1-20041028/structures.html#element-annotation">&lt;annotation&gt;</a>
element information items in the <a class="xpropref" href="https://www.w3.org/TR/xml-infoset/#infoitem.element">[children]</a>, if any.
</td></tr></tbody></table></td></tr></tbody></table>

<a id="minInclusive-validation-rules"></a>

##### 4.3.10.3 minInclusive Validation Rules

<a id="cvc-minInclusive-valid"></a>

**Validation Rule: minInclusive Valid**  

A value in an [·ordered·](022-4-1-6-simple-type-definition-for-anysimpletype.md#dt-ordered) [·value space·](003-1-4-terminology.md#dt-value-space)
is facet-valid with respect to [·minInclusive·](031-4-3-10-mininclusive.md#dt-minInclusive) if:

1
if the [·numeric·](024-4-2-5-numeric.md#dt-numeric) property in
[{fundamental facets}](018-4-datatype-components.md#defn-fund-facets) is *true*, then the
value [·must·](003-1-4-terminology.md#dt-must) be numerically greater than or equal to
[{value}](031-4-3-10-mininclusive.md#minInclusive-value);

2
if the [·numeric·](024-4-2-5-numeric.md#dt-numeric) property in
[{fundamental facets}](018-4-datatype-components.md#defn-fund-facets) is *false* (i.e.,
[{base type definition}](018-4-datatype-components.md#defn-basetype) is one of the date and time related
datatypes), then the value [·must·](003-1-4-terminology.md#dt-must) be chronologically
greater than or equal to [{value}](031-4-3-10-mininclusive.md#minInclusive-value);

<a id="minInclusive-coss"></a>

##### 4.3.10.4 Constraints on minInclusive Schema Components

<a id="minInclusive-less-than-maxExclusive"></a>

**Schema Component Constraint: minInclusive < maxExclusive**  

It is an [·error·](003-1-4-terminology.md#dt-error) for the value specified for
[·minInclusive·](031-4-3-10-mininclusive.md#dt-minInclusive) to be greater than or equal to the value
specified for [·maxExclusive·](029-4-3-7-3-maxinclusive-validation-rules.md#dt-maxExclusive) for the same datatype.

<a id="minInclusive-valid-restriction"></a>

**Schema Component Constraint: minInclusive valid restriction**  

It is an [·error·](003-1-4-terminology.md#dt-error) if any of the following conditions
is true:

1
[minInclusive](031-4-3-10-mininclusive.md#dc-minInclusive) is among the members of
[{facets}](018-4-datatype-components.md#defn-facets) of [{base type definition}](018-4-datatype-components.md#defn-basetype)
and [{value}](031-4-3-10-mininclusive.md#minInclusive-value) is
less than the [{value}](031-4-3-10-mininclusive.md#minInclusive-value) of the parent
[minInclusive](031-4-3-10-mininclusive.md#dc-minInclusive)

2
[maxInclusive](028-4-3-6-1-the-whitespace-schema-component.md#dc-maxInclusive) is among the members of
[{facets}](018-4-datatype-components.md#defn-facets) of [{base type definition}](018-4-datatype-components.md#defn-basetype)
and [{value}](031-4-3-10-mininclusive.md#minInclusive-value) is
greater the [{value}](028-4-3-6-1-the-whitespace-schema-component.md#maxInclusive-value) of the parent
[maxInclusive](028-4-3-6-1-the-whitespace-schema-component.md#dc-maxInclusive)

3
[minExclusive](030-4-3-9-minexclusive.md#dc-minExclusive) is among the members of
[{facets}](018-4-datatype-components.md#defn-facets) of [{base type definition}](018-4-datatype-components.md#defn-basetype)
and [{value}](031-4-3-10-mininclusive.md#minInclusive-value) is
less than or equal to the [{value}](030-4-3-9-minexclusive.md#minExclusive-value) of the parent
[minExclusive](030-4-3-9-minexclusive.md#dc-minExclusive)

4
[maxExclusive](029-4-3-7-3-maxinclusive-validation-rules.md#dc-maxExclusive) is among the members of
[{facets}](018-4-datatype-components.md#defn-facets) of [{base type definition}](018-4-datatype-components.md#defn-basetype)
and [{value}](031-4-3-10-mininclusive.md#minInclusive-value) is
greater than or equal to the [{value}](029-4-3-7-3-maxinclusive-validation-rules.md#maxExclusive-value) of the parent
[maxExclusive](029-4-3-7-3-maxinclusive-validation-rules.md#dc-maxExclusive)

<a id="rf-totalDigits"></a>

#### 4.3.11 totalDigits

<a id="dt-totalDigits"></a>

[Definition:]
**totalDigits**
controls the maximum number of values in
the [·value space·](003-1-4-terminology.md#dt-value-space)
of datatypes [·derived·](005-2-5-1-3-union-datatypes.md#dt-derived) from [decimal](006-2-5-2-3-derived-by-union.md#decimal),
by restricting it to numbers that are expressible as
*i × 10^-n* where *i*
and *n* are integers such that
*|i| < 10^totalDigits* and
*0 <= n <= totalDigits*.
The value of
**totalDigits** [·must·](003-1-4-terminology.md#dt-must) be a
[positiveInteger](017-3-3-21-unsignedlong.md#positiveInteger).

The term **totalDigits** is chosen to reflect the fact that it
restricts the [·value space·](003-1-4-terminology.md#dt-value-space) to those values that can
be represented lexically using at most *totalDigits*
digits. Note that it does not restrict the [·lexical space·](004-2-3-lexical-space.md#dt-lexical-space)
directly; a lexical representation that adds
additional leading zero digits or trailing fractional zero digits is
still permitted.

<a id="dc-totalDigits"></a>

##### 4.3.11.1 The totalDigits Schema Component

Schema Component: [totalDigits](031-4-3-10-mininclusive.md#dt-totalDigits)

<a id="totalDigits-value"></a>

{value}
:   A [positiveInteger](017-3-3-21-unsignedlong.md#positiveInteger).

<a id="totalDigits-fixed"></a>

{fixed}
:   A [boolean](006-2-5-2-3-derived-by-union.md#boolean).

<a id="totalDigits-annotation"></a>

{annotation}
:   Optional. An [annotation](https://www.w3.org/TR/2004/REC-xmlschema-1-20041028/structures.html#Annotation).

If [{fixed}](031-4-3-10-mininclusive.md#totalDigits-fixed) is *true*, then types for which
the current type is the [{base type definition}](018-4-datatype-components.md#defn-basetype) cannot specify a
value for [totalDigits](031-4-3-10-mininclusive.md#dc-totalDigits) other than
[{value}](031-4-3-10-mininclusive.md#totalDigits-value).

<a id="xr-totalDigits"></a>

##### 4.3.11.2 XML Representation of totalDigits Schema Components

The XML representation for a [totalDigits](031-4-3-10-mininclusive.md#dc-totalDigits) schema
component is a [<totalDigits>](032-4-3-11-3-totaldigits-validation-rules.md#element-totalDigits) element information item. The
correspondences between the properties of the information item and
properties of the component are as follows:

XML Representation Summary: `totalDigits` Element Information Item

