# 4.3.7.3 maxInclusive Validation Rules

Source: [XML Schema Part 2: Datatypes](https://www.w3.org/TR/2004/REC-xmlschema-2-20041028/) · 2nd edition, 2004-10-28.

[Document index](../README.md) · [Topic guide](../../../../README.md) · [Publisher](https://www.w3.org/TR/2004/REC-xmlschema-2-20041028/).

HTML normalized to Markdown. Complex tables and mathematical markup retain HTML; illustrations are stored locally.

[Previous](028-4-3-6-1-the-whitespace-schema-component.md) · [Next](030-4-3-9-minexclusive.md)

<table class="reprcomp"><thead><tr><th><a href="028-4-3-6-1-the-whitespace-schema-component.md#dt-maxInclusive">maxInclusive</a><strong> Schema Component</strong></th></tr></thead><tbody><tr><td><table border="0" cellpadding="3"><thead><tr><th align="left">Property</th><th align="left">Representation</th></tr></thead><tbody valign="top"><tr valign="top"><td><a class="propref" href="028-4-3-6-1-the-whitespace-schema-component.md#maxInclusive-value">{value}</a></td><td>
The <a href="https://www.w3.org/TR/2004/REC-xmlschema-1-20041028/structures.html#key-vv">actual value</a> of the <code>value</code> <a class="xpropref" href="https://www.w3.org/TR/xml-infoset/#infoitem.attribute">[attribute]</a>
</td></tr><tr valign="top"><td><a class="propref" href="028-4-3-6-1-the-whitespace-schema-component.md#maxInclusive-fixed">{fixed}</a></td><td>
The <a href="https://www.w3.org/TR/2004/REC-xmlschema-1-20041028/structures.html#key-vv">actual value</a> of the <code>fixed</code> <a class="xpropref" href="https://www.w3.org/TR/xml-infoset/#infoitem.attribute">[attribute]</a>, if present, otherwise false, if present, otherwise false
</td></tr><tr valign="top"><td><a class="propref" href="018-4-datatype-components.md#defn-annotation">{annotation}</a></td><td>
The annotations corresponding to all the <a class="eltref" href="https://www.w3.org/TR/2004/REC-xmlschema-1-20041028/structures.html#element-annotation">&lt;annotation&gt;</a>
element information items in the <a class="xpropref" href="https://www.w3.org/TR/xml-infoset/#infoitem.element">[children]</a>, if any.
</td></tr></tbody></table></td></tr></tbody></table>

<a id="maxInclusive-validation-rules"></a>

##### 4.3.7.3 maxInclusive Validation Rules

<a id="cvc-maxInclusive-valid"></a>

**Validation Rule: maxInclusive Valid**  

A value in an [·ordered·](022-4-1-6-simple-type-definition-for-anysimpletype.md#dt-ordered) [·value space·](003-1-4-terminology.md#dt-value-space)
is facet-valid with respect to [·maxInclusive·](028-4-3-6-1-the-whitespace-schema-component.md#dt-maxInclusive), determined as
follows:

1
if the [·numeric·](024-4-2-5-numeric.md#dt-numeric) property in
[{fundamental facets}](018-4-datatype-components.md#defn-fund-facets) is *true*, then the value
[·must·](003-1-4-terminology.md#dt-must) be numerically less than or
equal to [{value}](028-4-3-6-1-the-whitespace-schema-component.md#maxInclusive-value);

2
if the [·numeric·](024-4-2-5-numeric.md#dt-numeric) property in
[{fundamental facets}](018-4-datatype-components.md#defn-fund-facets) is *false* (i.e.,
[{base type definition}](018-4-datatype-components.md#defn-basetype) is one of the date and time related
datatypes), then the value [·must·](003-1-4-terminology.md#dt-must) be chronologically
less than or equal to [{value}](028-4-3-6-1-the-whitespace-schema-component.md#maxInclusive-value);

<a id="maxInclusive-coss"></a>

##### 4.3.7.4 Constraints on maxInclusive Schema Components

<a id="minInclusive-less-than-equal-to-maxInclusive"></a>

**Schema Component Constraint: minInclusive <= maxInclusive**  

It is an [·error·](003-1-4-terminology.md#dt-error) for the value specified for
[·minInclusive·](031-4-3-10-mininclusive.md#dt-minInclusive) to be greater than the value
specified for [·maxInclusive·](028-4-3-6-1-the-whitespace-schema-component.md#dt-maxInclusive) for the same datatype.

<a id="maxInclusive-valid-restriction"></a>

**Schema Component Constraint: maxInclusive valid restriction**  

It is an [·error·](003-1-4-terminology.md#dt-error) if any of the following conditions
is true:

1
[maxInclusive](028-4-3-6-1-the-whitespace-schema-component.md#dc-maxInclusive) is among the members of
[{facets}](018-4-datatype-components.md#defn-facets) of [{base type definition}](018-4-datatype-components.md#defn-basetype)
and [{value}](028-4-3-6-1-the-whitespace-schema-component.md#maxInclusive-value) is
greater than the [{value}](028-4-3-6-1-the-whitespace-schema-component.md#maxInclusive-value) of the parent
[maxInclusive](028-4-3-6-1-the-whitespace-schema-component.md#dc-maxInclusive)

2
[maxExclusive](029-4-3-7-3-maxinclusive-validation-rules.md#dc-maxExclusive) is among the members of
[{facets}](018-4-datatype-components.md#defn-facets) of [{base type definition}](018-4-datatype-components.md#defn-basetype)
and [{value}](028-4-3-6-1-the-whitespace-schema-component.md#maxInclusive-value) is
greater than or equal to the [{value}](029-4-3-7-3-maxinclusive-validation-rules.md#maxExclusive-value) of the parent
[maxExclusive](029-4-3-7-3-maxinclusive-validation-rules.md#dc-maxExclusive)

3
[minInclusive](031-4-3-10-mininclusive.md#dc-minInclusive) is among the members of
[{facets}](018-4-datatype-components.md#defn-facets) of [{base type definition}](018-4-datatype-components.md#defn-basetype)
and [{value}](028-4-3-6-1-the-whitespace-schema-component.md#maxInclusive-value) is
less than the [{value}](031-4-3-10-mininclusive.md#minInclusive-value) of the parent
[minInclusive](031-4-3-10-mininclusive.md#dc-minInclusive)

4
[minExclusive](030-4-3-9-minexclusive.md#dc-minExclusive) is among the members of
[{facets}](018-4-datatype-components.md#defn-facets) of [{base type definition}](018-4-datatype-components.md#defn-basetype)
and [{value}](028-4-3-6-1-the-whitespace-schema-component.md#maxInclusive-value) is
less than or equal to the [{value}](030-4-3-9-minexclusive.md#minExclusive-value) of the parent
[minExclusive](030-4-3-9-minexclusive.md#dc-minExclusive)

<a id="rf-maxExclusive"></a>

#### 4.3.8 maxExclusive

<a id="dt-maxExclusive"></a>

[Definition:]
**maxExclusive** is the [·exclusive upper bound·](023-4-2-3-bounded.md#dt-exclusive-upper-bound)
of the [·value space·](003-1-4-terminology.md#dt-value-space) for a datatype with the
[·ordered·](022-4-1-6-simple-type-definition-for-anysimpletype.md#dt-ordered) property. The value of **maxExclusive**
 [·must·](003-1-4-terminology.md#dt-must) be in the [·value space·](003-1-4-terminology.md#dt-value-space) of the
[·base type·](005-2-5-1-3-union-datatypes.md#dt-basetype) or be equal to [{value}](029-4-3-7-3-maxinclusive-validation-rules.md#maxExclusive-value) in
[{base type definition}](018-4-datatype-components.md#defn-basetype).

[·maxExclusive·](029-4-3-7-3-maxinclusive-validation-rules.md#dt-maxExclusive) provides for:

- Constraining a [·value space·](003-1-4-terminology.md#dt-value-space) to values
  with a specific [·exclusive upper bound·](023-4-2-3-bounded.md#dt-exclusive-upper-bound).

Example

The following is the definition of a [·user-derived·](006-2-5-2-3-derived-by-union.md#dt-user-derived)
datatype which limits values to integers less than or equal to
100, using [·maxExclusive·](029-4-3-7-3-maxinclusive-validation-rules.md#dt-maxExclusive).



```
<simpleType name='less-than-one-hundred-and-one'>
  <restriction base='integer'>
    <maxExclusive value='101'/>
  </restriction>
</simpleType>
```




Note that the
[·value space·](003-1-4-terminology.md#dt-value-space) of this datatype is identical to
the previous one (named 'one-hundred-or-less').

<a id="dc-maxExclusive"></a>

##### 4.3.8.1 The maxExclusive Schema Component

Schema Component: [maxExclusive](029-4-3-7-3-maxinclusive-validation-rules.md#dt-maxExclusive)

<a id="maxExclusive-value"></a>

{value}
:   A value from the [·value space·](003-1-4-terminology.md#dt-value-space) of the
    [{base type definition}](018-4-datatype-components.md#defn-basetype).

<a id="maxExclusive-fixed"></a>

{fixed}
:   A [boolean](006-2-5-2-3-derived-by-union.md#boolean).

<a id="maxExclusive-annotation"></a>

{annotation}
:   Optional. An [annotation](https://www.w3.org/TR/2004/REC-xmlschema-1-20041028/structures.html#Annotation).

If [{fixed}](029-4-3-7-3-maxinclusive-validation-rules.md#maxExclusive-fixed) is *true*, then types for which
the current type is the [{base type definition}](018-4-datatype-components.md#defn-basetype) cannot specify a
value for [maxExclusive](029-4-3-7-3-maxinclusive-validation-rules.md#dc-maxExclusive) other than [{value}](029-4-3-7-3-maxinclusive-validation-rules.md#maxExclusive-value).

<a id="xr-maxExclusive"></a>

##### 4.3.8.2 XML Representation of maxExclusive Schema Components

The XML representation for a [maxExclusive](029-4-3-7-3-maxinclusive-validation-rules.md#dc-maxExclusive) schema
component is a [<maxExclusive>](029-4-3-7-3-maxinclusive-validation-rules.md#element-maxExclusive) element information item. The
correspondences between the properties of the information item and
properties of the component are as follows:

XML Representation Summary: `maxExclusive` Element Information Item

<a id="element-maxExclusive"></a>

<maxExclusive  
  fixed = [boolean](006-2-5-2-3-derived-by-union.md#boolean) : false  
  id = [ID](014-3-3-4-1-constraining-facets.md#ID)  
  **value** = [anySimpleType](005-2-5-1-3-union-datatypes.md#dt-anySimpleType)  
  *{any attributes with non-schema namespace . . .}*>  
*Content:* ([annotation](https://www.w3.org/TR/2004/REC-xmlschema-1-20041028/structures.html#element-annotation)?)  
</maxExclusive>

[{value}](029-4-3-7-3-maxinclusive-validation-rules.md#maxExclusive-value) [·must·](003-1-4-terminology.md#dt-must) be
in the [·value space·](003-1-4-terminology.md#dt-value-space) of [{base type definition}](018-4-datatype-components.md#defn-basetype).

<table class="reprcomp"><thead><tr><th><a href="029-4-3-7-3-maxinclusive-validation-rules.md#dt-maxExclusive">maxExclusive</a><strong> Schema Component</strong></th></tr></thead><tbody><tr><td><table border="0" cellpadding="3"><thead><tr><th align="left">Property</th><th align="left">Representation</th></tr></thead><tbody valign="top"><tr valign="top"><td><a class="propref" href="029-4-3-7-3-maxinclusive-validation-rules.md#maxExclusive-value">{value}</a></td><td>
The <a href="https://www.w3.org/TR/2004/REC-xmlschema-1-20041028/structures.html#key-vv">actual value</a> of the <code>value</code> <a class="xpropref" href="https://www.w3.org/TR/xml-infoset/#infoitem.attribute">[attribute]</a>
</td></tr><tr valign="top"><td><a class="propref" href="029-4-3-7-3-maxinclusive-validation-rules.md#maxExclusive-fixed">{fixed}</a></td><td>
The <a href="https://www.w3.org/TR/2004/REC-xmlschema-1-20041028/structures.html#key-vv">actual value</a> of the <code>fixed</code> <a class="xpropref" href="https://www.w3.org/TR/xml-infoset/#infoitem.attribute">[attribute]</a>, if present, otherwise false
</td></tr><tr valign="top"><td><a class="propref" href="018-4-datatype-components.md#defn-annotation">{annotation}</a></td><td>
The annotations corresponding to all the <a class="eltref" href="https://www.w3.org/TR/2004/REC-xmlschema-1-20041028/structures.html#element-annotation">&lt;annotation&gt;</a>
element information items in the <a class="xpropref" href="https://www.w3.org/TR/xml-infoset/#infoitem.element">[children]</a>, if any.
</td></tr></tbody></table></td></tr></tbody></table>

<a id="maxExclusive-validation-rules"></a>

##### 4.3.8.3 maxExclusive Validation Rules

<a id="cvc-maxExclusive-valid"></a>

**Validation Rule: maxExclusive Valid**  

A value in an [·ordered·](022-4-1-6-simple-type-definition-for-anysimpletype.md#dt-ordered) [·value space·](003-1-4-terminology.md#dt-value-space)
is facet-valid with respect to [·maxExclusive·](029-4-3-7-3-maxinclusive-validation-rules.md#dt-maxExclusive), determined
as follows:

1
if the [·numeric·](024-4-2-5-numeric.md#dt-numeric) property in
[{fundamental facets}](018-4-datatype-components.md#defn-fund-facets) is *true*, then the
value [·must·](003-1-4-terminology.md#dt-must) be numerically less than
[{value}](029-4-3-7-3-maxinclusive-validation-rules.md#maxExclusive-value);

2
if the [·numeric·](024-4-2-5-numeric.md#dt-numeric) property in
[{fundamental facets}](018-4-datatype-components.md#defn-fund-facets) is *false* (i.e.,
[{base type definition}](018-4-datatype-components.md#defn-basetype) is one of the date and time related
datatypes), then the value [·must·](003-1-4-terminology.md#dt-must) be chronologically
less than [{value}](029-4-3-7-3-maxinclusive-validation-rules.md#maxExclusive-value);

<a id="maxExclusive-coss"></a>

##### 4.3.8.4 Constraints on maxExclusive Schema Components

<a id="maxInclusive-maxExclusive"></a>

**Schema Component Constraint: maxInclusive and maxExclusive**  

