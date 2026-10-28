# 4.3.11.3 totalDigits Validation Rules

Source: [XML Schema Part 2: Datatypes](https://www.w3.org/TR/2004/REC-xmlschema-2-20041028/) · 2nd edition, 2004-10-28.

[Document index](../README.md) · [Topic guide](../../../../README.md) · [Publisher](https://www.w3.org/TR/2004/REC-xmlschema-2-20041028/).

HTML normalized to Markdown. Complex tables and mathematical markup retain HTML; illustrations are stored locally.

[Previous](031-4-3-10-mininclusive.md) · [Next](033-5-conformance.md)

<a id="element-totalDigits"></a>

<totalDigits  
  fixed = [boolean](006-2-5-2-3-derived-by-union.md#boolean) : false  
  id = [ID](014-3-3-4-1-constraining-facets.md#ID)  
  **value** = [positiveInteger](017-3-3-21-unsignedlong.md#positiveInteger)  
  *{any attributes with non-schema namespace . . .}*>  
*Content:* ([annotation](https://www.w3.org/TR/2004/REC-xmlschema-1-20041028/structures.html#element-annotation)?)  
</totalDigits>

<table class="reprcomp"><thead><tr><th><a href="031-4-3-10-mininclusive.md#dc-totalDigits">totalDigits</a><strong> Schema Component</strong></th></tr></thead><tbody><tr><td><table border="0" cellpadding="3"><thead><tr><th align="left">Property</th><th align="left">Representation</th></tr></thead><tbody valign="top"><tr valign="top"><td><a class="propref" href="031-4-3-10-mininclusive.md#totalDigits-value">{value}</a></td><td>
The <a href="https://www.w3.org/TR/2004/REC-xmlschema-1-20041028/structures.html#key-vv">actual value</a> of the <code>value</code> <a class="xpropref" href="https://www.w3.org/TR/xml-infoset/#infoitem.attribute">[attribute]</a>
</td></tr><tr valign="top"><td><a class="propref" href="031-4-3-10-mininclusive.md#totalDigits-fixed">{fixed}</a></td><td>
The <a href="https://www.w3.org/TR/2004/REC-xmlschema-1-20041028/structures.html#key-vv">actual value</a> of the <code>fixed</code> <a class="xpropref" href="https://www.w3.org/TR/xml-infoset/#infoitem.attribute">[attribute]</a>, if present, otherwise false
</td></tr><tr valign="top"><td><a class="propref" href="018-4-datatype-components.md#defn-annotation">{annotation}</a></td><td>
The annotations corresponding to all the <a class="eltref" href="https://www.w3.org/TR/2004/REC-xmlschema-1-20041028/structures.html#element-annotation">&lt;annotation&gt;</a>
element information items in the <a class="xpropref" href="https://www.w3.org/TR/xml-infoset/#infoitem.element">[children]</a>, if any.
</td></tr></tbody></table></td></tr></tbody></table>

<a id="totalDigits-validation-rules"></a>

##### 4.3.11.3 totalDigits Validation Rules

<a id="cvc-totalDigits-valid"></a>

**Validation Rule: totalDigits Valid**  

A value in a [·value space·](003-1-4-terminology.md#dt-value-space) is facet-valid with
respect to [·totalDigits·](031-4-3-10-mininclusive.md#dt-totalDigits) if:

1
that value is expressible as *i × 10^-n* where
*i* and *n* are integers such that
*|i| < 10^[{value}](031-4-3-10-mininclusive.md#totalDigits-value)* and
*0 <= n <= [{value}](031-4-3-10-mininclusive.md#totalDigits-value)*.

<a id="totalDigits-coss"></a>

##### 4.3.11.4 Constraints on totalDigits Schema Components

<a id="totalDigits-valid-restriction"></a>

**Schema Component Constraint: totalDigits valid restriction**  

It is an [·error·](003-1-4-terminology.md#dt-error) if
[totalDigits](031-4-3-10-mininclusive.md#dc-totalDigits) is among the members of
[{facets}](018-4-datatype-components.md#defn-facets) of [{base type definition}](018-4-datatype-components.md#defn-basetype)
and [{value}](031-4-3-10-mininclusive.md#totalDigits-value) is
greater than the [{value}](031-4-3-10-mininclusive.md#totalDigits-value) of the parent
[totalDigits](031-4-3-10-mininclusive.md#dc-totalDigits)

<a id="rf-fractionDigits"></a>

#### 4.3.12 fractionDigits

<a id="dt-fractionDigits"></a>

[Definition:]
**fractionDigits**
controls the size of the minimum difference
between values
in the [·value space·](003-1-4-terminology.md#dt-value-space) of datatypes [·derived·](005-2-5-1-3-union-datatypes.md#dt-derived)
from **decimal**,
by restricting the [·value space·](003-1-4-terminology.md#dt-value-space) to numbers that are
expressible as *i × 10^-n* where
*i* and *n*
are integers and *0 <= n <= fractionDigits*.
The value of **fractionDigits** [·must·](003-1-4-terminology.md#dt-must)
be a [nonNegativeInteger](016-3-3-16-long.md#nonNegativeInteger).

The term **fractionDigits** is chosen to reflect the fact that it
restricts the [·value space·](003-1-4-terminology.md#dt-value-space) to those values that can be
represented lexically using at most *fractionDigits*
to the right of the decimal point. Note that it does not restrict the
[·lexical space·](004-2-3-lexical-space.md#dt-lexical-space) directly; a
non-[·canonical lexical representation·](004-2-3-lexical-space.md#dt-canonical-representation) that adds additional
leading zero digits or trailing fractional zero digits is still permitted.

Example

The following is the definition of a [·user-derived·](006-2-5-2-3-derived-by-union.md#dt-user-derived)
datatype which could be used to represent the magnitude
of a person's body temperature on the Celsius scale.
This definition would appear in a schema authored by an "end-user"
and shows how to define a datatype by specifying facet values which
constrain the range of the [·base type·](005-2-5-1-3-union-datatypes.md#dt-basetype).



```
<simpleType name='celsiusBodyTemp'>
  <restriction base='decimal'>
    <totalDigits value='4'/>
    <fractionDigits value='1'/>
    <minInclusive value='36.4'/>
    <maxInclusive value='40.5'/>
  </restriction>
</simpleType>
```




<a id="dc-fractionDigits"></a>

##### 4.3.12.1 The fractionDigits Schema Component

Schema Component: [fractionDigits](032-4-3-11-3-totaldigits-validation-rules.md#dt-fractionDigits)

<a id="fractionDigits-value"></a>

{value}
:   A [nonNegativeInteger](016-3-3-16-long.md#nonNegativeInteger).

<a id="fractionDigits-fixed"></a>

{fixed}
:   A [boolean](006-2-5-2-3-derived-by-union.md#boolean).

<a id="fractionDigits-annotation"></a>

{annotation}
:   Optional. An [annotation](https://www.w3.org/TR/2004/REC-xmlschema-1-20041028/structures.html#Annotation).

If [{fixed}](032-4-3-11-3-totaldigits-validation-rules.md#fractionDigits-fixed) is *true*, then types for which
the current type is the [{base type definition}](018-4-datatype-components.md#defn-basetype) cannot specify a
value for [fractionDigits](032-4-3-11-3-totaldigits-validation-rules.md#dc-fractionDigits) other than
[{value}](032-4-3-11-3-totaldigits-validation-rules.md#fractionDigits-value).

<a id="xr-fractionDigits"></a>

##### 4.3.12.2 XML Representation of fractionDigits Schema Components

The XML representation for a [fractionDigits](032-4-3-11-3-totaldigits-validation-rules.md#dc-fractionDigits) schema
component is a [<fractionDigits>](032-4-3-11-3-totaldigits-validation-rules.md#element-fractionDigits) element information item. The
correspondences between the properties of the information item and
properties of the component are as follows:

XML Representation Summary: `fractionDigits` Element Information Item

<a id="element-fractionDigits"></a>

<fractionDigits  
  fixed = [boolean](006-2-5-2-3-derived-by-union.md#boolean) : false  
  id = [ID](014-3-3-4-1-constraining-facets.md#ID)  
  **value** = [nonNegativeInteger](016-3-3-16-long.md#nonNegativeInteger)  
  *{any attributes with non-schema namespace . . .}*>  
*Content:* ([annotation](https://www.w3.org/TR/2004/REC-xmlschema-1-20041028/structures.html#element-annotation)?)  
</fractionDigits>

<table class="reprcomp"><thead><tr><th><a href="032-4-3-11-3-totaldigits-validation-rules.md#dc-fractionDigits">fractionDigits</a><strong> Schema Component</strong></th></tr></thead><tbody><tr><td><table border="0" cellpadding="3"><thead><tr><th align="left">Property</th><th align="left">Representation</th></tr></thead><tbody valign="top"><tr valign="top"><td><a class="propref" href="032-4-3-11-3-totaldigits-validation-rules.md#fractionDigits-value">{value}</a></td><td>
The <a href="https://www.w3.org/TR/2004/REC-xmlschema-1-20041028/structures.html#key-vv">actual value</a> of the <code>value</code> <a class="xpropref" href="https://www.w3.org/TR/xml-infoset/#infoitem.attribute">[attribute]</a>
</td></tr><tr valign="top"><td><a class="propref" href="032-4-3-11-3-totaldigits-validation-rules.md#fractionDigits-fixed">{fixed}</a></td><td>
The <a href="https://www.w3.org/TR/2004/REC-xmlschema-1-20041028/structures.html#key-vv">actual value</a> of the <code>fixed</code> <a class="xpropref" href="https://www.w3.org/TR/xml-infoset/#infoitem.attribute">[attribute]</a>, if present, otherwise false
</td></tr><tr valign="top"><td><a class="propref" href="018-4-datatype-components.md#defn-annotation">{annotation}</a></td><td>
The annotations corresponding to all the <a class="eltref" href="https://www.w3.org/TR/2004/REC-xmlschema-1-20041028/structures.html#element-annotation">&lt;annotation&gt;</a>
element information items in the <a class="xpropref" href="https://www.w3.org/TR/xml-infoset/#infoitem.element">[children]</a>, if any.
</td></tr></tbody></table></td></tr></tbody></table>

<a id="fractionDigits-validation-rules"></a>

##### 4.3.12.3 fractionDigits Validation Rules

<a id="cvc-fractionDigits-valid"></a>

**Validation Rule: fractionDigits Valid**  

A value in a [·value space·](003-1-4-terminology.md#dt-value-space) is facet-valid with
respect to [·fractionDigits·](032-4-3-11-3-totaldigits-validation-rules.md#dt-fractionDigits) if:

1
that value is expressible as *i × 10^-n* where
*i* and *n*
are integers and *0 <= n <= [{value}](032-4-3-11-3-totaldigits-validation-rules.md#fractionDigits-value)*.

<a id="fractionDigits-coss"></a>

##### 4.3.12.4 Constraints on fractionDigits Schema Components

<a id="fractionDigits-totalDigits"></a>

**Schema Component Constraint: fractionDigits less than or equal to totalDigits**  

It is an [·error·](003-1-4-terminology.md#dt-error) for [·fractionDigits·](032-4-3-11-3-totaldigits-validation-rules.md#dt-fractionDigits) to
be greater than [·totalDigits·](031-4-3-10-mininclusive.md#dt-totalDigits).

<a id="fractionDigits-valid-restriction"></a>

**Schema Component Constraint: fractionDigits valid restriction**  

It is an [·error·](003-1-4-terminology.md#dt-error) if [·fractionDigits·](032-4-3-11-3-totaldigits-validation-rules.md#dt-fractionDigits)
is among the members of [{facets}](018-4-datatype-components.md#defn-facets) of [{base type definition}](018-4-datatype-components.md#defn-basetype) and
[{value}](032-4-3-11-3-totaldigits-validation-rules.md#fractionDigits-value) is greater than the [{value}](032-4-3-11-3-totaldigits-validation-rules.md#fractionDigits-value) of the parent
[·fractionDigits·](032-4-3-11-3-totaldigits-validation-rules.md#dt-fractionDigits).

