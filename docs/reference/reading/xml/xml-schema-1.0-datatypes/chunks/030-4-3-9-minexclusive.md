# 4.3.9 minExclusive

Source: [XML Schema Part 2: Datatypes](https://www.w3.org/TR/2004/REC-xmlschema-2-20041028/) · 2nd edition, 2004-10-28.

[Document index](../README.md) · [Topic guide](../../../../README.md) · [Publisher](https://www.w3.org/TR/2004/REC-xmlschema-2-20041028/).

HTML normalized to Markdown. Complex tables and mathematical markup retain HTML; illustrations are stored locally.

[Previous](029-4-3-7-3-maxinclusive-validation-rules.md) · [Next](031-4-3-10-mininclusive.md)

It is an [·error·](003-1-4-terminology.md#dt-error) for both
[·maxInclusive·](028-4-3-6-1-the-whitespace-schema-component.md#dt-maxInclusive) and [·maxExclusive·](029-4-3-7-3-maxinclusive-validation-rules.md#dt-maxExclusive)
to be specified in the same derivation step of a datatype definition.

<a id="minExclusive-less-than-equal-to-maxExclusive"></a>

**Schema Component Constraint: minExclusive <= maxExclusive**  

It is an [·error·](003-1-4-terminology.md#dt-error) for the value specified for
[·minExclusive·](030-4-3-9-minexclusive.md#dt-minExclusive) to be greater than the value
specified for [·maxExclusive·](029-4-3-7-3-maxinclusive-validation-rules.md#dt-maxExclusive) for the same datatype.

<a id="maxExclusive-valid-restriction"></a>

**Schema Component Constraint: maxExclusive valid restriction**  

It is an [·error·](003-1-4-terminology.md#dt-error) if any of the following conditions
is true:

1
[maxExclusive](029-4-3-7-3-maxinclusive-validation-rules.md#dc-maxExclusive) is among the members of
[{facets}](018-4-datatype-components.md#defn-facets) of [{base type definition}](018-4-datatype-components.md#defn-basetype)
and [{value}](029-4-3-7-3-maxinclusive-validation-rules.md#maxExclusive-value) is
greater than the [{value}](029-4-3-7-3-maxinclusive-validation-rules.md#maxExclusive-value) of the parent
[maxExclusive](029-4-3-7-3-maxinclusive-validation-rules.md#dc-maxExclusive)

2
[maxInclusive](028-4-3-6-1-the-whitespace-schema-component.md#dc-maxInclusive) is among the members of
[{facets}](018-4-datatype-components.md#defn-facets) of [{base type definition}](018-4-datatype-components.md#defn-basetype)
and [{value}](029-4-3-7-3-maxinclusive-validation-rules.md#maxExclusive-value) is
greater than the [{value}](028-4-3-6-1-the-whitespace-schema-component.md#maxInclusive-value) of the parent
[maxInclusive](028-4-3-6-1-the-whitespace-schema-component.md#dc-maxInclusive)

3
[minInclusive](031-4-3-10-mininclusive.md#dc-minInclusive) is among the members of
[{facets}](018-4-datatype-components.md#defn-facets) of [{base type definition}](018-4-datatype-components.md#defn-basetype)
and [{value}](029-4-3-7-3-maxinclusive-validation-rules.md#maxExclusive-value) is
less than or equal to the [{value}](031-4-3-10-mininclusive.md#minInclusive-value) of the parent
[minInclusive](031-4-3-10-mininclusive.md#dc-minInclusive)

4
[minExclusive](030-4-3-9-minexclusive.md#dc-minExclusive) is among the members of
[{facets}](018-4-datatype-components.md#defn-facets) of [{base type definition}](018-4-datatype-components.md#defn-basetype)
and [{value}](029-4-3-7-3-maxinclusive-validation-rules.md#maxExclusive-value) is
less than or equal to the [{value}](030-4-3-9-minexclusive.md#minExclusive-value) of the parent
[minExclusive](030-4-3-9-minexclusive.md#dc-minExclusive)

<a id="rf-minExclusive"></a>

#### 4.3.9 minExclusive

<a id="dt-minExclusive"></a>

[Definition:]
**minExclusive** is the [·exclusive lower bound·](023-4-2-3-bounded.md#dt-exclusive-lower-bound)
of the [·value space·](003-1-4-terminology.md#dt-value-space) for a datatype with the
[·ordered·](022-4-1-6-simple-type-definition-for-anysimpletype.md#dt-ordered) property.
The value of **minExclusive** [·must·](003-1-4-terminology.md#dt-must)
be in the [·value space·](003-1-4-terminology.md#dt-value-space) of the
[·base type·](005-2-5-1-3-union-datatypes.md#dt-basetype)
or be equal to [{value}](030-4-3-9-minexclusive.md#minExclusive-value) in
[{base type definition}](018-4-datatype-components.md#defn-basetype).

[·minExclusive·](030-4-3-9-minexclusive.md#dt-minExclusive) provides for:

- Constraining a [·value space·](003-1-4-terminology.md#dt-value-space) to values
  with a specific [·exclusive lower bound·](023-4-2-3-bounded.md#dt-exclusive-lower-bound).

Example

The following is the definition of a [·user-derived·](006-2-5-2-3-derived-by-union.md#dt-user-derived)
datatype which limits values to integers greater than or equal to
100, using [·minExclusive·](030-4-3-9-minexclusive.md#dt-minExclusive).



```
<simpleType name='more-than-ninety-nine'>
  <restriction base='integer'>
    <minExclusive value='99'/>
  </restriction>
</simpleType>
```




Note that the
[·value space·](003-1-4-terminology.md#dt-value-space) of this datatype is identical to
the previous one (named 'one-hundred-or-more').

<a id="dc-minExclusive"></a>

##### 4.3.9.1 The minExclusive Schema Component

Schema Component: [minExclusive](030-4-3-9-minexclusive.md#dt-minExclusive)

<a id="minExclusive-value"></a>

{value}
:   A value from the [·value space·](003-1-4-terminology.md#dt-value-space) of the
    [{base type definition}](018-4-datatype-components.md#defn-basetype).

<a id="minExclusive-fixed"></a>

{fixed}
:   A [boolean](006-2-5-2-3-derived-by-union.md#boolean).

<a id="minExclusive-annotation"></a>

{annotation}
:   Optional. An [annotation](https://www.w3.org/TR/2004/REC-xmlschema-1-20041028/structures.html#Annotation).

If [{fixed}](030-4-3-9-minexclusive.md#minExclusive-fixed) is *true*, then types for which
the current type is the [{base type definition}](018-4-datatype-components.md#defn-basetype) cannot specify a
value for [minExclusive](030-4-3-9-minexclusive.md#dc-minExclusive) other than [{value}](030-4-3-9-minexclusive.md#minExclusive-value).

<a id="xr-minExclusive"></a>

##### 4.3.9.2 XML Representation of minExclusive Schema Components

The XML representation for a [minExclusive](030-4-3-9-minexclusive.md#dc-minExclusive) schema
component is a [<minExclusive>](030-4-3-9-minexclusive.md#element-minExclusive) element information item. The
correspondences between the properties of the information item and
properties of the component are as follows:

XML Representation Summary: `minExclusive` Element Information Item

<a id="element-minExclusive"></a>

<minExclusive  
  fixed = [boolean](006-2-5-2-3-derived-by-union.md#boolean) : false  
  id = [ID](014-3-3-4-1-constraining-facets.md#ID)  
  **value** = [anySimpleType](005-2-5-1-3-union-datatypes.md#dt-anySimpleType)  
  *{any attributes with non-schema namespace . . .}*>  
*Content:* ([annotation](https://www.w3.org/TR/2004/REC-xmlschema-1-20041028/structures.html#element-annotation)?)  
</minExclusive>

[{value}](030-4-3-9-minexclusive.md#minExclusive-value) [·must·](003-1-4-terminology.md#dt-must) be
in the [·value space·](003-1-4-terminology.md#dt-value-space) of [{base type definition}](018-4-datatype-components.md#defn-basetype).

<table class="reprcomp"><thead><tr><th><a href="030-4-3-9-minexclusive.md#dt-minExclusive">minExclusive</a><strong> Schema Component</strong></th></tr></thead><tbody><tr><td><table border="0" cellpadding="3"><thead><tr><th align="left">Property</th><th align="left">Representation</th></tr></thead><tbody valign="top"><tr valign="top"><td><a class="propref" href="030-4-3-9-minexclusive.md#minExclusive-value">{value}</a></td><td>
The <a href="https://www.w3.org/TR/2004/REC-xmlschema-1-20041028/structures.html#key-vv">actual value</a> of the <code>value</code> <a class="xpropref" href="https://www.w3.org/TR/xml-infoset/#infoitem.attribute">[attribute]</a>
</td></tr><tr valign="top"><td><a class="propref" href="030-4-3-9-minexclusive.md#minExclusive-fixed">{fixed}</a></td><td>
The <a href="https://www.w3.org/TR/2004/REC-xmlschema-1-20041028/structures.html#key-vv">actual value</a> of the <code>fixed</code> <a class="xpropref" href="https://www.w3.org/TR/xml-infoset/#infoitem.attribute">[attribute]</a>, if present, otherwise false
</td></tr><tr valign="top"><td><a class="propref" href="018-4-datatype-components.md#defn-annotation">{annotation}</a></td><td>
The annotations corresponding to all the <a class="eltref" href="https://www.w3.org/TR/2004/REC-xmlschema-1-20041028/structures.html#element-annotation">&lt;annotation&gt;</a>
element information items in the <a class="xpropref" href="https://www.w3.org/TR/xml-infoset/#infoitem.element">[children]</a>, if any.
</td></tr></tbody></table></td></tr></tbody></table>

<a id="minExclusive-validation-rules"></a>

##### 4.3.9.3 minExclusive Validation Rules

<a id="cvc-minExclusive-valid"></a>

**Validation Rule: minExclusive Valid**  

A value in an [·ordered·](022-4-1-6-simple-type-definition-for-anysimpletype.md#dt-ordered) [·value space·](003-1-4-terminology.md#dt-value-space)
is facet-valid with respect to [·minExclusive·](030-4-3-9-minexclusive.md#dt-minExclusive) if:

1
if the [·numeric·](024-4-2-5-numeric.md#dt-numeric) property in
[{fundamental facets}](018-4-datatype-components.md#defn-fund-facets) is *true*, then the
value [·must·](003-1-4-terminology.md#dt-must) be numerically greater than
[{value}](030-4-3-9-minexclusive.md#minExclusive-value);

2
if the [·numeric·](024-4-2-5-numeric.md#dt-numeric) property in
[{fundamental facets}](018-4-datatype-components.md#defn-fund-facets) is *false* (i.e.,
[{base type definition}](018-4-datatype-components.md#defn-basetype) is one of the date and time related
datatypes), then the value [·must·](003-1-4-terminology.md#dt-must) be chronologically
greater than [{value}](030-4-3-9-minexclusive.md#minExclusive-value);

<a id="minExclusive-coss"></a>

##### 4.3.9.4 Constraints on minExclusive Schema Components

<a id="minInclusive-minExclusive"></a>

**Schema Component Constraint: minInclusive and minExclusive**  

It is an [·error·](003-1-4-terminology.md#dt-error) for both
[·minInclusive·](031-4-3-10-mininclusive.md#dt-minInclusive) and [·minExclusive·](030-4-3-9-minexclusive.md#dt-minExclusive)
to be specified for the same datatype.

<a id="minExclusive-less-than-maxInclusive"></a>

**Schema Component Constraint: minExclusive < maxInclusive**  

It is an [·error·](003-1-4-terminology.md#dt-error) for the value specified for
[·minExclusive·](030-4-3-9-minexclusive.md#dt-minExclusive) to be greater than or equal to the value
specified for [·maxInclusive·](028-4-3-6-1-the-whitespace-schema-component.md#dt-maxInclusive) for the same datatype.

<a id="minExclusive-valid-restriction"></a>

**Schema Component Constraint: minExclusive valid restriction**  

It is an [·error·](003-1-4-terminology.md#dt-error) if any of the following conditions
is true:

1
[minExclusive](030-4-3-9-minexclusive.md#dc-minExclusive) is among the members of
[{facets}](018-4-datatype-components.md#defn-facets) of [{base type definition}](018-4-datatype-components.md#defn-basetype)
and [{value}](030-4-3-9-minexclusive.md#minExclusive-value) is
less than the [{value}](030-4-3-9-minexclusive.md#minExclusive-value) of the parent
[minExclusive](030-4-3-9-minexclusive.md#dc-minExclusive)

2
[maxInclusive](028-4-3-6-1-the-whitespace-schema-component.md#dc-maxInclusive) is among the members of
[{facets}](018-4-datatype-components.md#defn-facets) of [{base type definition}](018-4-datatype-components.md#defn-basetype)
and [{value}](030-4-3-9-minexclusive.md#minExclusive-value) is
greater the [{value}](028-4-3-6-1-the-whitespace-schema-component.md#maxInclusive-value) of the parent
[maxInclusive](028-4-3-6-1-the-whitespace-schema-component.md#dc-maxInclusive)

3
[minInclusive](031-4-3-10-mininclusive.md#dc-minInclusive) is among the members of
[{facets}](018-4-datatype-components.md#defn-facets) of [{base type definition}](018-4-datatype-components.md#defn-basetype)
and [{value}](030-4-3-9-minexclusive.md#minExclusive-value) is
less than the [{value}](031-4-3-10-mininclusive.md#minInclusive-value) of the parent
[minInclusive](031-4-3-10-mininclusive.md#dc-minInclusive)

