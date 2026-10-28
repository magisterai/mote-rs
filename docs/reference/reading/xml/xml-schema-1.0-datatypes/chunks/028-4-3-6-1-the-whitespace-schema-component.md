# 4.3.6.1 The whiteSpace Schema Component

Source: [XML Schema Part 2: Datatypes](https://www.w3.org/TR/2004/REC-xmlschema-2-20041028/) · 2nd edition, 2004-10-28.

[Document index](../README.md) · [Topic guide](../../../../README.md) · [Publisher](https://www.w3.org/TR/2004/REC-xmlschema-2-20041028/).

HTML normalized to Markdown. Complex tables and mathematical markup retain HTML; illustrations are stored locally.

[Previous](027-4-3-4-3-constraints-on-xml-representation-of-pattern.md) · [Next](029-4-3-7-3-maxinclusive-validation-rules.md)

**whiteSpace** is applicable to all [·atomic·](004-2-3-lexical-space.md#dt-atomic) and
[·list·](004-2-3-lexical-space.md#dt-list) datatypes. For all [·atomic·](004-2-3-lexical-space.md#dt-atomic)
datatypes other than [string](006-2-5-2-3-derived-by-union.md#string) (and types [·derived·](005-2-5-1-3-union-datatypes.md#dt-derived)
by [·restriction·](005-2-5-1-3-union-datatypes.md#dt-restriction) from it) the value of **whiteSpace** is
`collapse` and cannot be changed by a schema author; for
[string](006-2-5-2-3-derived-by-union.md#string) the value of **whiteSpace** is
`preserve`; for any type [·derived·](005-2-5-1-3-union-datatypes.md#dt-derived) by
[·restriction·](005-2-5-1-3-union-datatypes.md#dt-restriction) from
[string](006-2-5-2-3-derived-by-union.md#string) the value of **whiteSpace** can
be any of the three legal values. For all datatypes
[·derived·](005-2-5-1-3-union-datatypes.md#dt-derived) by [·list·](004-2-3-lexical-space.md#dt-list) the
value of **whiteSpace** is `collapse` and cannot
be changed by a schema author. For all datatypes
[·derived·](005-2-5-1-3-union-datatypes.md#dt-derived) by [·union·](004-2-3-lexical-space.md#dt-union)
 **whiteSpace** does not apply directly; however, the
normalization behavior of [·union·](004-2-3-lexical-space.md#dt-union) types is controlled by
the value of **whiteSpace** on that one of the
[·memberTypes·](005-2-5-1-3-union-datatypes.md#dt-memberTypes) against which the [·union·](004-2-3-lexical-space.md#dt-union)
is successfully validated.

**Note:**
For more information on **whiteSpace**, see the
discussion on white space normalization in
[Schema Component Details](https://www.w3.org/TR/2004/REC-xmlschema-1-20041028/structures.html#components)
in [[XML Schema Part 1: Structures]](044-h-references.md#structural-schemas).

[·whiteSpace·](027-4-3-4-3-constraints-on-xml-representation-of-pattern.md#dt-whiteSpace) provides for:

- Constraining a [·value space·](003-1-4-terminology.md#dt-value-space) according to
  the white space normalization rules.

Example

The following example is the datatype definition for
the [token](013-3-2-19-notation.md#token) [·built-in·](006-2-5-2-3-derived-by-union.md#dt-built-in) 
[·derived·](005-2-5-1-3-union-datatypes.md#dt-derived)
datatype.



```
<simpleType name='token'>
    <restriction base='normalizedString'>
      <whiteSpace value='collapse'/>
    </restriction>
</simpleType>
```




<a id="dc-whiteSpace"></a>

##### 4.3.6.1 The whiteSpace Schema Component

Schema Component: [whiteSpace](027-4-3-4-3-constraints-on-xml-representation-of-pattern.md#dt-whiteSpace)

<a id="whiteSpace-value"></a>

{value}
:   One of `{preserve, replace, collapse}`.

<a id="whiteSpace-fixed"></a>

{fixed}
:   A [boolean](006-2-5-2-3-derived-by-union.md#boolean).

<a id="whiteSpace-annotation"></a>

{annotation}
:   Optional. An [annotation](https://www.w3.org/TR/2004/REC-xmlschema-1-20041028/structures.html#Annotation).

If [{fixed}](028-4-3-6-1-the-whitespace-schema-component.md#whiteSpace-fixed) is *true*, then types for which
the current type is the [{base type definition}](018-4-datatype-components.md#defn-basetype) cannot specify a
value for [whiteSpace](028-4-3-6-1-the-whitespace-schema-component.md#dc-whiteSpace) other than [{value}](028-4-3-6-1-the-whitespace-schema-component.md#whiteSpace-value).

<a id="xr-whiteSpace"></a>

##### 4.3.6.2 XML Representation of whiteSpace Schema Components

The XML representation for a [whiteSpace](028-4-3-6-1-the-whitespace-schema-component.md#dc-whiteSpace) schema
component is a [<whiteSpace>](028-4-3-6-1-the-whitespace-schema-component.md#element-whiteSpace) element information item. The
correspondences between the properties of the information item and
properties of the component are as follows:

XML Representation Summary: `whiteSpace` Element Information Item

<a id="element-whiteSpace"></a>

<whiteSpace  
  fixed = [boolean](006-2-5-2-3-derived-by-union.md#boolean) : false  
  id = [ID](014-3-3-4-1-constraining-facets.md#ID)  
  **value** = (collapse | preserve | replace)  
  *{any attributes with non-schema namespace . . .}*>  
*Content:* ([annotation](https://www.w3.org/TR/2004/REC-xmlschema-1-20041028/structures.html#element-annotation)?)  
</whiteSpace>

<table class="reprcomp"><thead><tr><th><a href="028-4-3-6-1-the-whitespace-schema-component.md#dc-whiteSpace">whiteSpace</a><strong> Schema Component</strong></th></tr></thead><tbody><tr><td><table border="0" cellpadding="3"><thead><tr><th align="left">Property</th><th align="left">Representation</th></tr></thead><tbody valign="top"><tr valign="top"><td><a class="propref" href="028-4-3-6-1-the-whitespace-schema-component.md#whiteSpace-value">{value}</a></td><td>
The <a href="https://www.w3.org/TR/2004/REC-xmlschema-1-20041028/structures.html#key-vv">actual value</a> of the <code>value</code> <a class="xpropref" href="https://www.w3.org/TR/xml-infoset/#infoitem.attribute">[attribute]</a>
</td></tr><tr valign="top"><td><a class="propref" href="028-4-3-6-1-the-whitespace-schema-component.md#whiteSpace-fixed">{fixed}</a></td><td>
The <a href="https://www.w3.org/TR/2004/REC-xmlschema-1-20041028/structures.html#key-vv">actual value</a> of the <code>fixed</code> <a class="xpropref" href="https://www.w3.org/TR/xml-infoset/#infoitem.attribute">[attribute]</a>, if present, otherwise false
</td></tr><tr valign="top"><td><a class="propref" href="018-4-datatype-components.md#defn-annotation">{annotation}</a></td><td>
The annotations corresponding to all the <a class="eltref" href="https://www.w3.org/TR/2004/REC-xmlschema-1-20041028/structures.html#element-annotation">&lt;annotation&gt;</a>
element information items in the <a class="xpropref" href="https://www.w3.org/TR/xml-infoset/#infoitem.element">[children]</a>, if any.
</td></tr></tbody></table></td></tr></tbody></table>

<a id="whiteSpace-validation-rules"></a>

##### 4.3.6.3 whiteSpace Validation Rules

**Note:**
There are no [·Validation Rule·](003-1-4-terminology.md#dt-cvc)s associated [·whiteSpace·](027-4-3-4-3-constraints-on-xml-representation-of-pattern.md#dt-whiteSpace).
For more information, see the
discussion on white space normalization in
[Schema Component Details](https://www.w3.org/TR/2004/REC-xmlschema-1-20041028/structures.html#components)
in [[XML Schema Part 1: Structures]](044-h-references.md#structural-schemas).

<a id="whiteSpace-coss"></a>

##### 4.3.6.4 Constraints on whiteSpace Schema Components

<a id="whiteSpace-valid-restriction"></a>

**Schema Component Constraint: whiteSpace valid restriction**  

It is an [·error·](003-1-4-terminology.md#dt-error) if [whiteSpace](028-4-3-6-1-the-whitespace-schema-component.md#dc-whiteSpace)
is among the members of [{facets}](018-4-datatype-components.md#defn-facets) of
[{base type definition}](018-4-datatype-components.md#defn-basetype) and any of the following conditions is
true:

1
[{value}](028-4-3-6-1-the-whitespace-schema-component.md#whiteSpace-value) is *replace* or *preserve*
and the [{value}](028-4-3-6-1-the-whitespace-schema-component.md#whiteSpace-value) of the parent
[whiteSpace](028-4-3-6-1-the-whitespace-schema-component.md#dc-whiteSpace) is *collapse*

2
[{value}](028-4-3-6-1-the-whitespace-schema-component.md#whiteSpace-value) is *preserve*
and the [{value}](028-4-3-6-1-the-whitespace-schema-component.md#whiteSpace-value) of the parent
[whiteSpace](028-4-3-6-1-the-whitespace-schema-component.md#dc-whiteSpace) is *replace*

<a id="rf-maxInclusive"></a>

#### 4.3.7 maxInclusive

<a id="dt-maxInclusive"></a>

[Definition:]
**maxInclusive** is the [·inclusive upper bound·](023-4-2-3-bounded.md#dt-inclusive-upper-bound)
of the [·value space·](003-1-4-terminology.md#dt-value-space) for a datatype with the
[·ordered·](022-4-1-6-simple-type-definition-for-anysimpletype.md#dt-ordered) property. The value of
**maxInclusive** [·must·](003-1-4-terminology.md#dt-must) be
in the [·value space·](003-1-4-terminology.md#dt-value-space) of the
[·base type·](005-2-5-1-3-union-datatypes.md#dt-basetype).

[·maxInclusive·](028-4-3-6-1-the-whitespace-schema-component.md#dt-maxInclusive) provides for:

- Constraining a [·value space·](003-1-4-terminology.md#dt-value-space) to values
  with a specific [·inclusive upper bound·](023-4-2-3-bounded.md#dt-inclusive-upper-bound).

Example

The following is the definition of a [·user-derived·](006-2-5-2-3-derived-by-union.md#dt-user-derived)
datatype which limits values to integers less than or equal to
100, using [·maxInclusive·](028-4-3-6-1-the-whitespace-schema-component.md#dt-maxInclusive).



```
<simpleType name='one-hundred-or-less'>
  <restriction base='integer'>
    <maxInclusive value='100'/>
  </restriction>
</simpleType>
```




<a id="dc-maxInclusive"></a>

##### 4.3.7.1 The maxInclusive Schema Component

Schema Component: [maxInclusive](028-4-3-6-1-the-whitespace-schema-component.md#dt-maxInclusive)

<a id="maxInclusive-value"></a>

{value}
:   A value from the [·value space·](003-1-4-terminology.md#dt-value-space) of the
    [{base type definition}](018-4-datatype-components.md#defn-basetype).

<a id="maxInclusive-fixed"></a>

{fixed}
:   A [boolean](006-2-5-2-3-derived-by-union.md#boolean).

<a id="maxInclusive-annotation"></a>

{annotation}
:   Optional. An [annotation](https://www.w3.org/TR/2004/REC-xmlschema-1-20041028/structures.html#Annotation).

If [{fixed}](028-4-3-6-1-the-whitespace-schema-component.md#maxInclusive-fixed) is *true*, then types for which
the current type is the [{base type definition}](018-4-datatype-components.md#defn-basetype) cannot specify a
value for [maxInclusive](028-4-3-6-1-the-whitespace-schema-component.md#dc-maxInclusive) other than [{value}](028-4-3-6-1-the-whitespace-schema-component.md#maxInclusive-value).

<a id="xr-maxInclusive"></a>

##### 4.3.7.2 XML Representation of maxInclusive Schema Components

The XML representation for a [maxInclusive](028-4-3-6-1-the-whitespace-schema-component.md#dc-maxInclusive) schema
component is a [<maxInclusive>](028-4-3-6-1-the-whitespace-schema-component.md#element-maxInclusive) element information item. The
correspondences between the properties of the information item and
properties of the component are as follows:

XML Representation Summary: `maxInclusive` Element Information Item

<a id="element-maxInclusive"></a>

<maxInclusive  
  fixed = [boolean](006-2-5-2-3-derived-by-union.md#boolean) : false  
  id = [ID](014-3-3-4-1-constraining-facets.md#ID)  
  **value** = [anySimpleType](005-2-5-1-3-union-datatypes.md#dt-anySimpleType)  
  *{any attributes with non-schema namespace . . .}*>  
*Content:* ([annotation](https://www.w3.org/TR/2004/REC-xmlschema-1-20041028/structures.html#element-annotation)?)  
</maxInclusive>

[{value}](028-4-3-6-1-the-whitespace-schema-component.md#maxInclusive-value) [·must·](003-1-4-terminology.md#dt-must) be
in the [·value space·](003-1-4-terminology.md#dt-value-space) of [{base type definition}](018-4-datatype-components.md#defn-basetype).

