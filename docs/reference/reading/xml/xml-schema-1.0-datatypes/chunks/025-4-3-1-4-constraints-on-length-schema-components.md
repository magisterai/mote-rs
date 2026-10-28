# 4.3.1.4 Constraints on length Schema Components

Source: [XML Schema Part 2: Datatypes](https://www.w3.org/TR/2004/REC-xmlschema-2-20041028/) · 2nd edition, 2004-10-28.

[Document index](../README.md) · [Topic guide](../../../../README.md) · [Publisher](https://www.w3.org/TR/2004/REC-xmlschema-2-20041028/).

HTML normalized to Markdown. Complex tables and mathematical markup retain HTML; illustrations are stored locally.

[Previous](024-4-2-5-numeric.md) · [Next](026-4-3-3-maxlength.md)

2
if the [{variety}](018-4-datatype-components.md#defn-variety) is [·list·](004-2-3-lexical-space.md#dt-list),
then the length of the value, as measured
in list items, [·must·](003-1-4-terminology.md#dt-must) be equal to [{value}](024-4-2-5-numeric.md#length-value)

The use of [·length·](024-4-2-5-numeric.md#dt-length)
on datatypes [·derived·](005-2-5-1-3-union-datatypes.md#dt-derived) from [QName](012-3-2-15-hexbinary.md#QName) and [NOTATION](013-3-2-19-notation.md#NOTATION)
is deprecated. Future versions of this
specification may remove this facet for these datatypes.

<a id="length-coss"></a>

##### 4.3.1.4 Constraints on length Schema Components

<a id="length-minLength-maxLength"></a>

**Schema Component Constraint: length and minLength or maxLength**  

If [length](024-4-2-5-numeric.md#dc-length) is a member of [{facets}](018-4-datatype-components.md#defn-facets) then

1 It is an error for [minLength](025-4-3-1-4-constraints-on-length-schema-components.md#dc-minLength) to be a member of
[{facets}](018-4-datatype-components.md#defn-facets) unless

1.1 the [{value}](025-4-3-1-4-constraints-on-length-schema-components.md#minLength-value) of [minLength](025-4-3-1-4-constraints-on-length-schema-components.md#dc-minLength) <= the [{value}](024-4-2-5-numeric.md#length-value) of [length](024-4-2-5-numeric.md#dc-length) and

1.2 there is type definition from which this one is derived by
one or more restriction steps in which [minLength](025-4-3-1-4-constraints-on-length-schema-components.md#dc-minLength) has the same
[{value}](025-4-3-1-4-constraints-on-length-schema-components.md#minLength-value) and [length](024-4-2-5-numeric.md#dc-length) is not specified.

2 It is an error for [maxLength](026-4-3-3-maxlength.md#dc-maxLength) to be a member of
[{facets}](018-4-datatype-components.md#defn-facets) unless

2.1 the [{value}](024-4-2-5-numeric.md#length-value) of [length](024-4-2-5-numeric.md#dc-length) <= the [{value}](026-4-3-3-maxlength.md#maxLength-value) of [maxLength](026-4-3-3-maxlength.md#dc-maxLength) and

2.2 there is type definition from which this one is derived by
one or more restriction steps in which [maxLength](026-4-3-3-maxlength.md#dc-maxLength) has the same
[{value}](026-4-3-3-maxlength.md#maxLength-value) and [length](024-4-2-5-numeric.md#dc-length) is not specified.

<a id="length-valid-restriction"></a>

**Schema Component Constraint: length valid restriction**  

It is an [·error·](003-1-4-terminology.md#dt-error) if [length](024-4-2-5-numeric.md#dc-length)
is among the members of [{facets}](018-4-datatype-components.md#defn-facets) of
[{base type definition}](018-4-datatype-components.md#defn-basetype) and [{value}](024-4-2-5-numeric.md#length-value) is
not equal to the [{value}](024-4-2-5-numeric.md#length-value) of the parent
[length](024-4-2-5-numeric.md#dc-length).

<a id="rf-minLength"></a>

#### 4.3.2 minLength

<a id="dt-minLength"></a>

[Definition:]
**minLength** is
the minimum number of *units of length*, where
*units of length* varies depending on the type that is being
[·derived·](005-2-5-1-3-union-datatypes.md#dt-derived) from.
The value of **minLength**
 [·must·](003-1-4-terminology.md#dt-must) be a [nonNegativeInteger](016-3-3-16-long.md#nonNegativeInteger).

For [string](006-2-5-2-3-derived-by-union.md#string) and datatypes [·derived·](005-2-5-1-3-union-datatypes.md#dt-derived) from [string](006-2-5-2-3-derived-by-union.md#string),
**minLength** is measured in units of [character](https://www.w3.org/TR/2000/WD-xml-2e-20000814#dt-character)s as defined in [[XML 1.0 (Second Edition)]](044-h-references.md#XML).
For [hexBinary](012-3-2-15-hexbinary.md#hexBinary) and [base64Binary](012-3-2-15-hexbinary.md#base64Binary) and datatypes [·derived·](005-2-5-1-3-union-datatypes.md#dt-derived) from them,
**minLength** is measured in octets (8 bits) of binary data.
For datatypes [·derived·](005-2-5-1-3-union-datatypes.md#dt-derived) by [·list·](004-2-3-lexical-space.md#dt-list),
**minLength** is measured in number of list items.

**Note:**
For [string](006-2-5-2-3-derived-by-union.md#string) and datatypes [·derived·](005-2-5-1-3-union-datatypes.md#dt-derived) from [string](006-2-5-2-3-derived-by-union.md#string),
**minLength** will not always coincide with "string length" as perceived
by some users or with the number of storage units in some digital representation.
Therefore, care should be taken when specifying a value for **minLength**
and in attempting to infer storage requirements from a given value for
**minLength**.

[·minLength·](025-4-3-1-4-constraints-on-length-schema-components.md#dt-minLength) provides for:

- Constraining a [·value space·](003-1-4-terminology.md#dt-value-space)
  to values with at least a specific number of *units of length*,
  where *units of length*
  varies depending on [{base type definition}](018-4-datatype-components.md#defn-basetype).

Example

The following is the definition of a [·user-derived·](006-2-5-2-3-derived-by-union.md#dt-user-derived)
datatype which requires strings to have at least one character (i.e.,
the empty string is not in the [·value space·](003-1-4-terminology.md#dt-value-space)
of this datatype).



```
<simpleType name='non-empty-string'>
  <restriction base='string'>
    <minLength value='1'/>
  </restriction>
</simpleType>
```




<a id="dc-minLength"></a>

##### 4.3.2.1 The minLength Schema Component

Schema Component: [minLength](025-4-3-1-4-constraints-on-length-schema-components.md#dt-minLength)

<a id="minLength-value"></a>

{value}
:   A [nonNegativeInteger](016-3-3-16-long.md#nonNegativeInteger).

<a id="minLength-fixed"></a>

{fixed}
:   A [boolean](006-2-5-2-3-derived-by-union.md#boolean).

<a id="minLength-annotation"></a>

{annotation}
:   Optional. An [annotation](https://www.w3.org/TR/2004/REC-xmlschema-1-20041028/structures.html#Annotation).

If [{fixed}](025-4-3-1-4-constraints-on-length-schema-components.md#minLength-fixed) is *true*, then types for which
the current type is the [{base type definition}](018-4-datatype-components.md#defn-basetype) cannot specify a
value for [minLength](025-4-3-1-4-constraints-on-length-schema-components.md#dc-minLength) other than [{value}](025-4-3-1-4-constraints-on-length-schema-components.md#minLength-value).

<a id="xr-minLength"></a>

##### 4.3.2.2 XML Representation of minLength Schema Component

The XML representation for a [minLength](025-4-3-1-4-constraints-on-length-schema-components.md#dc-minLength) schema
component is a [<minLength>](025-4-3-1-4-constraints-on-length-schema-components.md#element-minLength) element information item. The
correspondences between the properties of the information item and
properties of the component are as follows:

XML Representation Summary: `minLength` Element Information Item

<a id="element-minLength"></a>

<minLength  
  fixed = [boolean](006-2-5-2-3-derived-by-union.md#boolean) : false  
  id = [ID](014-3-3-4-1-constraining-facets.md#ID)  
  **value** = [nonNegativeInteger](016-3-3-16-long.md#nonNegativeInteger)  
  *{any attributes with non-schema namespace . . .}*>  
*Content:* ([annotation](https://www.w3.org/TR/2004/REC-xmlschema-1-20041028/structures.html#element-annotation)?)  
</minLength>

<table class="reprcomp"><thead><tr><th><a href="032-4-3-11-3-totaldigits-validation-rules.md#dc-fractionDigits">minLength</a><strong> Schema Component</strong></th></tr></thead><tbody><tr><td><table border="0" cellpadding="3"><thead><tr><th align="left">Property</th><th align="left">Representation</th></tr></thead><tbody valign="top"><tr valign="top"><td><a class="propref" href="025-4-3-1-4-constraints-on-length-schema-components.md#minLength-value">{value}</a></td><td>
The <a href="https://www.w3.org/TR/2004/REC-xmlschema-1-20041028/structures.html#key-vv">actual value</a> of the <code>value</code> <a class="xpropref" href="https://www.w3.org/TR/xml-infoset/#infoitem.attribute">[attribute]</a>
</td></tr><tr valign="top"><td><a class="propref" href="025-4-3-1-4-constraints-on-length-schema-components.md#minLength-fixed">{fixed}</a></td><td>
The <a href="https://www.w3.org/TR/2004/REC-xmlschema-1-20041028/structures.html#key-vv">actual value</a> of the <code>fixed</code> <a class="xpropref" href="https://www.w3.org/TR/xml-infoset/#infoitem.attribute">[attribute]</a>, if present, otherwise false
</td></tr><tr valign="top"><td><a class="propref" href="018-4-datatype-components.md#defn-annotation">{annotation}</a></td><td>
The annotations corresponding to all the <a class="eltref" href="https://www.w3.org/TR/2004/REC-xmlschema-1-20041028/structures.html#element-annotation">&lt;annotation&gt;</a>
element information items in the <a class="xpropref" href="https://www.w3.org/TR/xml-infoset/#infoitem.element">[children]</a>, if any.
</td></tr></tbody></table></td></tr></tbody></table>

<a id="minLength-validation-rules"></a>

##### 4.3.2.3 minLength Validation Rules

<a id="cvc-minLength-valid"></a>

**Validation Rule: minLength Valid**  

A value in a [·value space·](003-1-4-terminology.md#dt-value-space) is facet-valid with
respect to [·minLength·](025-4-3-1-4-constraints-on-length-schema-components.md#dt-minLength), determined as follows:

1
if the [{variety}](018-4-datatype-components.md#defn-variety) is [·atomic·](004-2-3-lexical-space.md#dt-atomic) then

1.1
if [{primitive type definition}](018-4-datatype-components.md#defn-primitive) is [string](006-2-5-2-3-derived-by-union.md#string) or
[anyURI](012-3-2-15-hexbinary.md#anyURI), then the
length of the value, as measured in[character](https://www.w3.org/TR/2000/WD-xml-2e-20000814#dt-character)s
[·must·](003-1-4-terminology.md#dt-must) be greater than or equal to
[{value}](025-4-3-1-4-constraints-on-length-schema-components.md#minLength-value);

1.2
if [{primitive type definition}](018-4-datatype-components.md#defn-primitive) is [hexBinary](012-3-2-15-hexbinary.md#hexBinary) or [base64Binary](012-3-2-15-hexbinary.md#base64Binary), then the
length of the value, as measured in octets of the binary data,
[·must·](003-1-4-terminology.md#dt-must) be greater than or equal to
[{value}](025-4-3-1-4-constraints-on-length-schema-components.md#minLength-value);

1.3
if [{primitive type definition}](018-4-datatype-components.md#defn-primitive) is [QName](012-3-2-15-hexbinary.md#QName) or [NOTATION](013-3-2-19-notation.md#NOTATION), then
any [{value}](025-4-3-1-4-constraints-on-length-schema-components.md#minLength-value) is facet-valid.

2
if the [{variety}](018-4-datatype-components.md#defn-variety) is [·list·](004-2-3-lexical-space.md#dt-list),
then the length of the value, as measured
in list items, [·must·](003-1-4-terminology.md#dt-must) be greater than or equal
to [{value}](025-4-3-1-4-constraints-on-length-schema-components.md#minLength-value)

The use of [·minLength·](025-4-3-1-4-constraints-on-length-schema-components.md#dt-minLength)
on datatypes [·derived·](005-2-5-1-3-union-datatypes.md#dt-derived) from [QName](012-3-2-15-hexbinary.md#QName) and [NOTATION](013-3-2-19-notation.md#NOTATION)
is deprecated. Future versions of this
specification may remove this facet for these datatypes.

<a id="minLength-coss"></a>

##### 4.3.2.4 Constraints on minLength Schema Components

<a id="minLength-less-than-equal-to-maxLength"></a>

**Schema Component Constraint: minLength <= maxLength**  

