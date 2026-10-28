# 4.3.3 maxLength

Source: [XML Schema Part 2: Datatypes](https://www.w3.org/TR/2004/REC-xmlschema-2-20041028/) · 2nd edition, 2004-10-28.

[Document index](../README.md) · [Topic guide](../../../../README.md) · [Publisher](https://www.w3.org/TR/2004/REC-xmlschema-2-20041028/).

HTML normalized to Markdown. Complex tables and mathematical markup retain HTML; illustrations are stored locally.

[Previous](025-4-3-1-4-constraints-on-length-schema-components.md) · [Next](027-4-3-4-3-constraints-on-xml-representation-of-pattern.md)

If both [minLength](025-4-3-1-4-constraints-on-length-schema-components.md#dc-minLength) and [maxLength](026-4-3-3-maxlength.md#dc-maxLength)
are members of [{facets}](018-4-datatype-components.md#defn-facets), then the
[{value}](025-4-3-1-4-constraints-on-length-schema-components.md#minLength-value) of [minLength](025-4-3-1-4-constraints-on-length-schema-components.md#dc-minLength)
 [·must·](003-1-4-terminology.md#dt-must) be less than or equal to the
[{value}](026-4-3-3-maxlength.md#maxLength-value) of [maxLength](026-4-3-3-maxlength.md#dc-maxLength).

<a id="minLength-valid-restriction"></a>

**Schema Component Constraint: minLength valid restriction**  

It is an [·error·](003-1-4-terminology.md#dt-error) if [minLength](025-4-3-1-4-constraints-on-length-schema-components.md#dc-minLength)
is among the members of [{facets}](018-4-datatype-components.md#defn-facets) of
[{base type definition}](018-4-datatype-components.md#defn-basetype) and [{value}](025-4-3-1-4-constraints-on-length-schema-components.md#minLength-value) is
less than the [{value}](025-4-3-1-4-constraints-on-length-schema-components.md#minLength-value) of the parent
[minLength](025-4-3-1-4-constraints-on-length-schema-components.md#dc-minLength).

<a id="rf-maxLength"></a>

#### 4.3.3 maxLength

<a id="dt-maxLength"></a>

[Definition:]
**maxLength** is
the maximum number of *units of length*, where
*units of length* varies
depending on the type that is being [·derived·](005-2-5-1-3-union-datatypes.md#dt-derived) from.
The value of **maxLength**
 [·must·](003-1-4-terminology.md#dt-must) be a [nonNegativeInteger](016-3-3-16-long.md#nonNegativeInteger).

For [string](006-2-5-2-3-derived-by-union.md#string) and datatypes [·derived·](005-2-5-1-3-union-datatypes.md#dt-derived) from [string](006-2-5-2-3-derived-by-union.md#string),
**maxLength** is measured in units of [character](https://www.w3.org/TR/2000/WD-xml-2e-20000814#dt-character)s as defined in [[XML 1.0 (Second Edition)]](044-h-references.md#XML).
For [hexBinary](012-3-2-15-hexbinary.md#hexBinary) and [base64Binary](012-3-2-15-hexbinary.md#base64Binary) and datatypes [·derived·](005-2-5-1-3-union-datatypes.md#dt-derived) from them,
**maxLength** is measured in octets (8 bits) of binary data.
For datatypes [·derived·](005-2-5-1-3-union-datatypes.md#dt-derived) by [·list·](004-2-3-lexical-space.md#dt-list),
**maxLength** is measured in number of list items.

**Note:**
For [string](006-2-5-2-3-derived-by-union.md#string) and datatypes [·derived·](005-2-5-1-3-union-datatypes.md#dt-derived) from [string](006-2-5-2-3-derived-by-union.md#string),
**maxLength** will not always coincide with "string length" as perceived
by some users or with the number of storage units in some digital representation.
Therefore, care should be taken when specifying a value for **maxLength**
and in attempting to infer storage requirements from a given value for
**maxLength**.

[·maxLength·](026-4-3-3-maxlength.md#dt-maxLength) provides for:

- Constraining a [·value space·](003-1-4-terminology.md#dt-value-space)
  to values with at most a specific number of *units of length*,
  where *units of length*
  varies depending on [{base type definition}](018-4-datatype-components.md#defn-basetype).

Example

The following is the definition of a [·user-derived·](006-2-5-2-3-derived-by-union.md#dt-user-derived)
datatype which might be used to accept form input with an upper limit
to the number of characters that are acceptable.



```
<simpleType name='form-input'>
  <restriction base='string'>
    <maxLength value='50'/>
  </restriction>
</simpleType>
```




<a id="dc-maxLength"></a>

##### 4.3.3.1 The maxLength Schema Component

Schema Component: [maxLength](026-4-3-3-maxlength.md#dt-maxLength)

<a id="maxLength-value"></a>

{value}
:   A [nonNegativeInteger](016-3-3-16-long.md#nonNegativeInteger).

<a id="maxLength-fixed"></a>

{fixed}
:   A [boolean](006-2-5-2-3-derived-by-union.md#boolean).

<a id="maxLength-annotation"></a>

{annotation}
:   Optional. An [annotation](https://www.w3.org/TR/2004/REC-xmlschema-1-20041028/structures.html#Annotation).

If [{fixed}](026-4-3-3-maxlength.md#maxLength-fixed) is *true*, then types for which
the current type is the [{base type definition}](018-4-datatype-components.md#defn-basetype) cannot specify a
value for [maxLength](026-4-3-3-maxlength.md#dc-maxLength) other than [{value}](026-4-3-3-maxlength.md#maxLength-value).

<a id="xr-maxLength"></a>

##### 4.3.3.2 XML Representation of maxLength Schema Components

The XML representation for a [maxLength](026-4-3-3-maxlength.md#dc-maxLength) schema
component is a [<maxLength>](026-4-3-3-maxlength.md#element-maxLength) element information item. The
correspondences between the properties of the information item and
properties of the component are as follows:

XML Representation Summary: `maxLength` Element Information Item

<a id="element-maxLength"></a>

<maxLength  
  fixed = [boolean](006-2-5-2-3-derived-by-union.md#boolean) : false  
  id = [ID](014-3-3-4-1-constraining-facets.md#ID)  
  **value** = [nonNegativeInteger](016-3-3-16-long.md#nonNegativeInteger)  
  *{any attributes with non-schema namespace . . .}*>  
*Content:* ([annotation](https://www.w3.org/TR/2004/REC-xmlschema-1-20041028/structures.html#element-annotation)?)  
</maxLength>

<table class="reprcomp"><thead><tr><th><a href="032-4-3-11-3-totaldigits-validation-rules.md#dc-fractionDigits">maxLength</a><strong> Schema Component</strong></th></tr></thead><tbody><tr><td><table border="0" cellpadding="3"><thead><tr><th align="left">Property</th><th align="left">Representation</th></tr></thead><tbody valign="top"><tr valign="top"><td><a class="propref" href="026-4-3-3-maxlength.md#maxLength-value">{value}</a></td><td>
The <a href="https://www.w3.org/TR/2004/REC-xmlschema-1-20041028/structures.html#key-vv">actual value</a> of the <code>value</code> <a class="xpropref" href="https://www.w3.org/TR/xml-infoset/#infoitem.attribute">[attribute]</a>
</td></tr><tr valign="top"><td><a class="propref" href="026-4-3-3-maxlength.md#maxLength-fixed">{fixed}</a></td><td>
The <a href="https://www.w3.org/TR/2004/REC-xmlschema-1-20041028/structures.html#key-vv">actual value</a> of the <code>fixed</code> <a class="xpropref" href="https://www.w3.org/TR/xml-infoset/#infoitem.attribute">[attribute]</a>, if present, otherwise false
</td></tr><tr valign="top"><td><a class="propref" href="018-4-datatype-components.md#defn-annotation">{annotation}</a></td><td>
The annotations corresponding to all the <a class="eltref" href="https://www.w3.org/TR/2004/REC-xmlschema-1-20041028/structures.html#element-annotation">&lt;annotation&gt;</a>
element information items in the <a class="xpropref" href="https://www.w3.org/TR/xml-infoset/#infoitem.element">[children]</a>, if any.
</td></tr></tbody></table></td></tr></tbody></table>

<a id="maxLength-validation-rules"></a>

##### 4.3.3.3 maxLength Validation Rules

<a id="cvc-maxLength-valid"></a>

**Validation Rule: maxLength Valid**  

A value in a [·value space·](003-1-4-terminology.md#dt-value-space) is facet-valid with
respect to [·maxLength·](026-4-3-3-maxlength.md#dt-maxLength), determined as follows:

1
if the [{variety}](018-4-datatype-components.md#defn-variety) is [·atomic·](004-2-3-lexical-space.md#dt-atomic) then

1.1
if [{primitive type definition}](018-4-datatype-components.md#defn-primitive) is [string](006-2-5-2-3-derived-by-union.md#string) or
[anyURI](012-3-2-15-hexbinary.md#anyURI), then the
length of the value, as measured in [character](https://www.w3.org/TR/2000/WD-xml-2e-20000814#dt-character)s
[·must·](003-1-4-terminology.md#dt-must) be less than or equal to
[{value}](026-4-3-3-maxlength.md#maxLength-value);

1.2
if [{primitive type definition}](018-4-datatype-components.md#defn-primitive) is [hexBinary](012-3-2-15-hexbinary.md#hexBinary) or [base64Binary](012-3-2-15-hexbinary.md#base64Binary), then the
length of the value, as measured in octets of the binary data,
[·must·](003-1-4-terminology.md#dt-must) be less than or equal to [{value}](026-4-3-3-maxlength.md#maxLength-value);

1.3
if [{primitive type definition}](018-4-datatype-components.md#defn-primitive) is [QName](012-3-2-15-hexbinary.md#QName) or [NOTATION](013-3-2-19-notation.md#NOTATION), then
any [{value}](026-4-3-3-maxlength.md#maxLength-value) is facet-valid.

2
if the [{variety}](018-4-datatype-components.md#defn-variety) is [·list·](004-2-3-lexical-space.md#dt-list),
then the length of the value, as measured
in list items, [·must·](003-1-4-terminology.md#dt-must) be less than or equal to
[{value}](026-4-3-3-maxlength.md#maxLength-value)

The use of [·maxLength·](026-4-3-3-maxlength.md#dt-maxLength)
on datatypes [·derived·](005-2-5-1-3-union-datatypes.md#dt-derived) from [QName](012-3-2-15-hexbinary.md#QName) and [NOTATION](013-3-2-19-notation.md#NOTATION)
is deprecated. Future versions of this
specification may remove this facet for these datatypes.

<a id="maxLength-coss"></a>

##### 4.3.3.4 Constraints on maxLength Schema Components

<a id="maxLength-valid-restriction"></a>

**Schema Component Constraint: maxLength valid restriction**  

It is an [·error·](003-1-4-terminology.md#dt-error) if [maxLength](026-4-3-3-maxlength.md#dc-maxLength)
is among the members of [{facets}](018-4-datatype-components.md#defn-facets) of
[{base type definition}](018-4-datatype-components.md#defn-basetype) and [{value}](026-4-3-3-maxlength.md#maxLength-value) is
greater than the [{value}](026-4-3-3-maxlength.md#maxLength-value) of the parent
[maxLength](026-4-3-3-maxlength.md#dc-maxLength).

<a id="rf-pattern"></a>

#### 4.3.4 pattern

<a id="dt-pattern"></a>

[Definition:]
**pattern** is a constraint on the
[·value space·](003-1-4-terminology.md#dt-value-space) of a datatype which is achieved by
constraining the [·lexical space·](004-2-3-lexical-space.md#dt-lexical-space) to literals
which match a specific pattern. The value of **pattern**
 [·must·](003-1-4-terminology.md#dt-must) be a [·regular expression·](039-e-1-algorithm.md#dt-regex).

[·pattern·](026-4-3-3-maxlength.md#dt-pattern) provides for:

- Constraining a [·value space·](003-1-4-terminology.md#dt-value-space)
  to values that are denoted by literals which match a specific
  [·regular expression·](039-e-1-algorithm.md#dt-regex).

Example

The following is the definition of a [·user-derived·](006-2-5-2-3-derived-by-union.md#dt-user-derived)
datatype which is a better representation of postal codes in the
United States, by limiting strings to those which are matched by
a specific [·regular expression·](039-e-1-algorithm.md#dt-regex).



```
<simpleType name='better-us-zipcode'>
  <restriction base='string'>
    <pattern value='[0-9]{5}(-[0-9]{4})?'/>
  </restriction>
</simpleType>
```




<a id="dc-pattern"></a>

##### 4.3.4.1 The pattern Schema Component

Schema Component: [pattern](026-4-3-3-maxlength.md#dt-pattern)

<a id="pattern-value"></a>

{value}
:   A [·regular expression·](039-e-1-algorithm.md#dt-regex).

<a id="pattern-annotation"></a>

{annotation}
:   Optional. An [annotation](https://www.w3.org/TR/2004/REC-xmlschema-1-20041028/structures.html#Annotation).

<a id="xr-pattern"></a>

##### 4.3.4.2 XML Representation of pattern Schema Components

The XML representation for a [pattern](026-4-3-3-maxlength.md#dc-pattern) schema
component is a [<pattern>](027-4-3-4-3-constraints-on-xml-representation-of-pattern.md#element-pattern) element information item. The
correspondences between the properties of the information item and
properties of the component are as follows:

