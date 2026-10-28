# 4.2.5 numeric

Source: [XML Schema Part 2: Datatypes](https://www.w3.org/TR/2004/REC-xmlschema-2-20041028/) · 2nd edition, 2004-10-28.

[Document index](../README.md) · [Topic guide](../../../../README.md) · [Publisher](https://www.w3.org/TR/2004/REC-xmlschema-2-20041028/).

HTML normalized to Markdown. Complex tables and mathematical markup retain HTML; illustrations are stored locally.

[Previous](023-4-2-3-bounded.md) · [Next](025-4-3-1-4-constraints-on-length-schema-components.md)

When [{variety}](018-4-datatype-components.md#defn-variety) is [·union·](004-2-3-lexical-space.md#dt-union),
if [{value}](023-4-2-3-bounded.md#cardinality-value) is *finite*
for every member of [{member type definitions}](018-4-datatype-components.md#defn-memberTypes), then
[{value}](023-4-2-3-bounded.md#cardinality-value) is *finite*;
else [{value}](023-4-2-3-bounded.md#cardinality-value) is *countably infinite*.

<a id="rf-numeric"></a>

#### 4.2.5 numeric

<a id="dt-numeric"></a>

[Definition:]A datatype is said to be
**numeric** if its values are conceptually quantities (in some
mathematical number system).

<a id="dt-non-numeric"></a>

[Definition:]A datatype whose values
are not [·numeric·](024-4-2-5-numeric.md#dt-numeric) is said to be
**non-numeric**.

[·numeric·](024-4-2-5-numeric.md#dt-numeric) provides for:

- indicating whether a [·value space·](003-1-4-terminology.md#dt-value-space) is
  [·numeric·](024-4-2-5-numeric.md#dt-numeric)

<a id="dc-numeric"></a>

##### 4.2.5.1 The numeric Schema Component

Schema Component: [numeric](024-4-2-5-numeric.md#dt-numeric)

<a id="numeric-value"></a>

{value}
:   A [boolean](006-2-5-2-3-derived-by-union.md#boolean)

[{value}](024-4-2-5-numeric.md#numeric-value) depends on [{variety}](018-4-datatype-components.md#defn-variety),
[{facets}](018-4-datatype-components.md#defn-facets), [{base type definition}](018-4-datatype-components.md#defn-basetype) and
[{member type definitions}](018-4-datatype-components.md#defn-memberTypes) in the [Simple Type Definition](018-4-datatype-components.md#dc-defn) component
in which a [·cardinality·](023-4-2-3-bounded.md#dt-cardinality) component appears as a member of
[{fundamental facets}](018-4-datatype-components.md#defn-fund-facets).

When [{variety}](018-4-datatype-components.md#defn-variety) is [·atomic·](004-2-3-lexical-space.md#dt-atomic),
[{value}](024-4-2-5-numeric.md#numeric-value) is inherited from
[{value}](024-4-2-5-numeric.md#numeric-value) of [{base type definition}](018-4-datatype-components.md#defn-basetype).
For all [·primitive·](005-2-5-1-3-union-datatypes.md#dt-primitive) types [{value}](024-4-2-5-numeric.md#numeric-value)
is as specified in the table in [Fundamental Facets (§C.1)](035-b-dtd-for-datatype-definitions-non-normative.md#app-fundamental-facets).

When [{variety}](018-4-datatype-components.md#defn-variety) is [·list·](004-2-3-lexical-space.md#dt-list),
[{value}](024-4-2-5-numeric.md#numeric-value) is *false*.

When [{variety}](018-4-datatype-components.md#defn-variety) is [·union·](004-2-3-lexical-space.md#dt-union),
if [{value}](024-4-2-5-numeric.md#numeric-value) is *true*
for every member of [{member type definitions}](018-4-datatype-components.md#defn-memberTypes), then
[{value}](024-4-2-5-numeric.md#numeric-value) is *true*;
else [{value}](024-4-2-5-numeric.md#numeric-value) is *false*.

<a id="rf-facets"></a>

### 4.3 Constraining Facets

4.3.1 [length](024-4-2-5-numeric.md#rf-length)  
        4.3.2 [minLength](025-4-3-1-4-constraints-on-length-schema-components.md#rf-minLength)  
        4.3.3 [maxLength](026-4-3-3-maxlength.md#rf-maxLength)  
        4.3.4 [pattern](026-4-3-3-maxlength.md#rf-pattern)  
        4.3.5 [enumeration](027-4-3-4-3-constraints-on-xml-representation-of-pattern.md#rf-enumeration)  
        4.3.6 [whiteSpace](027-4-3-4-3-constraints-on-xml-representation-of-pattern.md#rf-whiteSpace)  
        4.3.7 [maxInclusive](028-4-3-6-1-the-whitespace-schema-component.md#rf-maxInclusive)  
        4.3.8 [maxExclusive](029-4-3-7-3-maxinclusive-validation-rules.md#rf-maxExclusive)  
        4.3.9 [minExclusive](030-4-3-9-minexclusive.md#rf-minExclusive)  
        4.3.10 [minInclusive](031-4-3-10-mininclusive.md#rf-minInclusive)  
        4.3.11 [totalDigits](031-4-3-10-mininclusive.md#rf-totalDigits)  
        4.3.12 [fractionDigits](032-4-3-11-3-totaldigits-validation-rules.md#rf-fractionDigits)

<a id="rf-length"></a>

#### 4.3.1 length

<a id="dt-length"></a>

[Definition:]
**length** is the number
of *units of length*, where *units of length*
varies depending on the type that is being [·derived·](005-2-5-1-3-union-datatypes.md#dt-derived) from.
The value of
**length** [·must·](003-1-4-terminology.md#dt-must) be a
[nonNegativeInteger](016-3-3-16-long.md#nonNegativeInteger).

For [string](006-2-5-2-3-derived-by-union.md#string) and datatypes [·derived·](005-2-5-1-3-union-datatypes.md#dt-derived) from [string](006-2-5-2-3-derived-by-union.md#string),
**length** is measured in units of [character](https://www.w3.org/TR/2000/WD-xml-2e-20000814#dt-character)s as defined in [[XML 1.0 (Second Edition)]](044-h-references.md#XML).
For [anyURI](012-3-2-15-hexbinary.md#anyURI), **length** is measured in units of
characters (as for [string](006-2-5-2-3-derived-by-union.md#string)).
For [hexBinary](012-3-2-15-hexbinary.md#hexBinary) and [base64Binary](012-3-2-15-hexbinary.md#base64Binary) and datatypes [·derived·](005-2-5-1-3-union-datatypes.md#dt-derived) from them,
**length** is measured in octets (8 bits) of binary data.
For datatypes [·derived·](005-2-5-1-3-union-datatypes.md#dt-derived) by [·list·](004-2-3-lexical-space.md#dt-list),
**length** is measured in number of list items.

**Note:**
For [string](006-2-5-2-3-derived-by-union.md#string) and datatypes [·derived·](005-2-5-1-3-union-datatypes.md#dt-derived) from [string](006-2-5-2-3-derived-by-union.md#string),
**length** will not always coincide with "string length" as perceived
by some users or with the number of storage units in some digital representation.
Therefore, care should be taken when specifying a value for **length**
and in attempting to infer storage requirements from a given value for
**length**.

[·length·](024-4-2-5-numeric.md#dt-length) provides for:

- Constraining a [·value space·](003-1-4-terminology.md#dt-value-space)
  to values with a specific number of *units of length*,
  where *units of length*
  varies depending on [{base type definition}](018-4-datatype-components.md#defn-basetype).

Example

The following is the definition of a [·user-derived·](006-2-5-2-3-derived-by-union.md#dt-user-derived)
datatype to represent product codes which must be
exactly 8 characters in length. By fixing the value of the
**length** facet we ensure that types derived from productCode can
change or set the values of other facets, such as **pattern**, but
cannot change the length.



```
<simpleType name='productCode'>
   <restriction base='string'>
     <length value='8' fixed='true'/>
   </restriction>
</simpleType>
```




<a id="dc-length"></a>

##### 4.3.1.1 The length Schema Component

Schema Component: [length](024-4-2-5-numeric.md#dt-length)

<a id="length-value"></a>

{value}
:   A [nonNegativeInteger](016-3-3-16-long.md#nonNegativeInteger).

<a id="length-fixed"></a>

{fixed}
:   A [boolean](006-2-5-2-3-derived-by-union.md#boolean).

<a id="length-annotation"></a>

{annotation}
:   Optional. An [annotation](https://www.w3.org/TR/2004/REC-xmlschema-1-20041028/structures.html#Annotation).

If [{fixed}](024-4-2-5-numeric.md#length-fixed) is *true*, then types for which
the current type is the [{base type definition}](018-4-datatype-components.md#defn-basetype) cannot specify a
value for [length](024-4-2-5-numeric.md#dc-length) other than [{value}](024-4-2-5-numeric.md#length-value).

<a id="xr-length"></a>

##### 4.3.1.2 XML Representation of length Schema Components

The XML representation for a [length](024-4-2-5-numeric.md#dc-length) schema
component is a [<length>](024-4-2-5-numeric.md#element-length) element information item. The
correspondences between the properties of the information item and
properties of the component are as follows:

XML Representation Summary: `length` Element Information Item

<a id="element-length"></a>

<length  
  fixed = [boolean](006-2-5-2-3-derived-by-union.md#boolean) : false  
  id = [ID](014-3-3-4-1-constraining-facets.md#ID)  
  **value** = [nonNegativeInteger](016-3-3-16-long.md#nonNegativeInteger)  
  *{any attributes with non-schema namespace . . .}*>  
*Content:* ([annotation](https://www.w3.org/TR/2004/REC-xmlschema-1-20041028/structures.html#element-annotation)?)  
</length>

<table class="reprcomp"><thead><tr><th><a href="032-4-3-11-3-totaldigits-validation-rules.md#dc-fractionDigits">length</a><strong> Schema Component</strong></th></tr></thead><tbody><tr><td><table border="0" cellpadding="3"><thead><tr><th align="left">Property</th><th align="left">Representation</th></tr></thead><tbody valign="top"><tr valign="top"><td><a class="propref" href="024-4-2-5-numeric.md#length-value">{value}</a></td><td>
The <a href="https://www.w3.org/TR/2004/REC-xmlschema-1-20041028/structures.html#key-vv">actual value</a> of the <code>value</code> <a class="xpropref" href="https://www.w3.org/TR/xml-infoset/#infoitem.attribute">[attribute]</a>
</td></tr><tr valign="top"><td><a class="propref" href="024-4-2-5-numeric.md#length-fixed">{fixed}</a></td><td>
The <a href="https://www.w3.org/TR/2004/REC-xmlschema-1-20041028/structures.html#key-vv">actual value</a> of the <code>fixed</code> <a class="xpropref" href="https://www.w3.org/TR/xml-infoset/#infoitem.attribute">[attribute]</a>, if present, otherwise false
</td></tr><tr valign="top"><td><a class="propref" href="018-4-datatype-components.md#defn-annotation">{annotation}</a></td><td>
The annotations corresponding to all the <a class="eltref" href="https://www.w3.org/TR/2004/REC-xmlschema-1-20041028/structures.html#element-annotation">&lt;annotation&gt;</a>
element information items in the <a class="xpropref" href="https://www.w3.org/TR/xml-infoset/#infoitem.element">[children]</a>, if any.
</td></tr></tbody></table></td></tr></tbody></table>

<a id="length-validation-rules"></a>

##### 4.3.1.3 length Validation Rules

<a id="cvc-length-valid"></a>

**Validation Rule: Length Valid**  

A value in a [·value space·](003-1-4-terminology.md#dt-value-space) is facet-valid with
respect to [·length·](024-4-2-5-numeric.md#dt-length), determined as follows:

1
if the [{variety}](018-4-datatype-components.md#defn-variety) is [·atomic·](004-2-3-lexical-space.md#dt-atomic) then

1.1
if [{primitive type definition}](018-4-datatype-components.md#defn-primitive) is [string](006-2-5-2-3-derived-by-union.md#string) or [anyURI](012-3-2-15-hexbinary.md#anyURI), then the
length of the value, as measured in [character](https://www.w3.org/TR/2000/WD-xml-2e-20000814#dt-character)s
[·must·](003-1-4-terminology.md#dt-must) be equal to [{value}](024-4-2-5-numeric.md#length-value);

1.2
if [{primitive type definition}](018-4-datatype-components.md#defn-primitive) is [hexBinary](012-3-2-15-hexbinary.md#hexBinary) or [base64Binary](012-3-2-15-hexbinary.md#base64Binary), then the
length of the value, as measured in octets of the binary data,
[·must·](003-1-4-terminology.md#dt-must) be equal to [{value}](024-4-2-5-numeric.md#length-value);

1.3
if [{primitive type definition}](018-4-datatype-components.md#defn-primitive) is [QName](012-3-2-15-hexbinary.md#QName) or [NOTATION](013-3-2-19-notation.md#NOTATION), then
any [{value}](024-4-2-5-numeric.md#length-value) is facet-valid.

