# 4.1.2.1 Derivation by restriction

Source: [XML Schema Part 2: Datatypes](https://www.w3.org/TR/2004/REC-xmlschema-2-20041028/) · 2nd edition, 2004-10-28.

[Document index](../README.md) · [Topic guide](../../../../README.md) · [Publisher](https://www.w3.org/TR/2004/REC-xmlschema-2-20041028/).

HTML normalized to Markdown. Complex tables and mathematical markup retain HTML; illustrations are stored locally.

[Previous](018-4-datatype-components.md) · [Next](020-4-1-2-3-derivation-by-union.md)

<table class="reprcomp"><thead><tr><th><a href="018-4-datatype-components.md#dc-defn">Datatype Definition</a><strong> Schema Component</strong></th></tr></thead><tbody><tr><td><table border="0" cellpadding="3"><thead><tr><th align="left">Property</th><th align="left">Representation</th></tr></thead><tbody valign="top"><tr valign="top"><td><a class="propref" href="018-4-datatype-components.md#defn-name">{name}</a></td><td>
The <a href="https://www.w3.org/TR/2004/REC-xmlschema-1-20041028/structures.html#key-vv">actual value</a> of the <code>name</code> <a class="xpropref" href="https://www.w3.org/TR/xml-infoset/#infoitem.attribute">[attribute]</a>, if present,
otherwise <a href="https://www.w3.org/TR/2004/REC-xmlschema-1-20041028/structures.html#key-null">null</a>
</td></tr><tr valign="top"><td><a class="propref" href="018-4-datatype-components.md#defn-final">{final}</a></td><td>
A set corresponding to the <a href="https://www.w3.org/TR/2004/REC-xmlschema-1-20041028/structures.html#key-vv">actual value</a> of the
<code>final</code> <a class="xpropref" href="https://www.w3.org/TR/xml-infoset/#infoitem.attribute">[attribute]</a>, if present, otherwise
 the <a href="https://www.w3.org/TR/2004/REC-xmlschema-1-20041028/structures.html#key-vv">actual value</a> of the
<code>finalDefault</code> <a class="xpropref" href="https://www.w3.org/TR/xml-infoset/#infoitem.attribute">[attribute]</a> of the ancestor
<a href="https://www.w3.org/TR/2004/REC-xmlschema-1-20041028/structures.html#element-schema">schema</a>
element information item, if present, otherwise the empty string, as follows:
   <dl><dt class="label">the empty string</dt><dd>the empty set;</dd><dt class="label">
<code>#all</code>
</dt><dd><em>{restriction, list, union}</em>;</dd><dt class="label"><em>otherwise</em></dt><dd>a set with members drawn from the set above, each being present or
absent depending on whether the string contains an equivalently named
space-delimited substring.<div class="note"><b>Note: </b>Although the <code>finalDefault</code> <a class="xpropref" href="https://www.w3.org/TR/xml-infoset/#infoitem.attribute">[attribute]</a> of
       <a href="https://www.w3.org/TR/2004/REC-xmlschema-1-20041028/structures.html#element-schema">schema</a> may include
       values other than
       <i>restriction</i>, <i>list</i> or <i>union</i>, those values
       are ignored in the determination of <a class="propref" href="018-4-datatype-components.md#defn-final">{final}</a>
</div></dd></dl>
</td></tr><tr valign="top"><td><a class="propref" href="018-4-datatype-components.md#defn-target-namespace">{target namespace}</a></td><td>
The <a href="https://www.w3.org/TR/2004/REC-xmlschema-1-20041028/structures.html#key-vv">actual value</a> of the <code>targetNamespace</code> <a class="xpropref" href="https://www.w3.org/TR/xml-infoset/#infoitem.attribute">[attribute]</a>
of the parent <code>schema</code> element information item.
</td></tr><tr valign="top"><td><a class="propref" href="018-4-datatype-components.md#defn-annotation">{annotation}</a></td><td>
The annotation corresponding to the <a class="eltref" href="https://www.w3.org/TR/2004/REC-xmlschema-1-20041028/structures.html#element-annotation">&lt;annotation&gt;</a>
element information item in the <a class="xpropref" href="https://www.w3.org/TR/xml-infoset/#infoitem.element">[children]</a>, if present, otherwise
<a href="https://www.w3.org/TR/2004/REC-xmlschema-1-20041028/structures.html#key-null">null</a>
</td></tr></tbody></table></td></tr></tbody></table>

A [·derived·](005-2-5-1-3-union-datatypes.md#dt-derived) datatype can be [·derived·](005-2-5-1-3-union-datatypes.md#dt-derived)
from a [·primitive·](005-2-5-1-3-union-datatypes.md#dt-primitive) datatype or another
[·derived·](005-2-5-1-3-union-datatypes.md#dt-derived) datatype by one of three means:
by *restriction*, by *list* or by *union*.

<a id="derivation-by-restriction"></a>

##### 4.1.2.1 Derivation by restriction

XML Representation Summary: `restriction` Element Information Item

<a id="element-restriction"></a>

<restriction  
  base = [QName](012-3-2-15-hexbinary.md#QName)  
  id = [ID](014-3-3-4-1-constraining-facets.md#ID)  
  *{any attributes with non-schema namespace . . .}*>  
*Content:* ([annotation](https://www.w3.org/TR/2004/REC-xmlschema-1-20041028/structures.html#element-annotation)?, ([simpleType](018-4-datatype-components.md#element-simpleType)?, ([minExclusive](030-4-3-9-minexclusive.md#element-minExclusive) | [minInclusive](031-4-3-10-mininclusive.md#element-minInclusive) | [maxExclusive](029-4-3-7-3-maxinclusive-validation-rules.md#element-maxExclusive) | [maxInclusive](028-4-3-6-1-the-whitespace-schema-component.md#element-maxInclusive) | [totalDigits](032-4-3-11-3-totaldigits-validation-rules.md#element-totalDigits) | [fractionDigits](032-4-3-11-3-totaldigits-validation-rules.md#element-fractionDigits) | [length](024-4-2-5-numeric.md#element-length) | [minLength](025-4-3-1-4-constraints-on-length-schema-components.md#element-minLength) | [maxLength](026-4-3-3-maxlength.md#element-maxLength) | [enumeration](027-4-3-4-3-constraints-on-xml-representation-of-pattern.md#element-enumeration) | [whiteSpace](028-4-3-6-1-the-whitespace-schema-component.md#element-whiteSpace) | [pattern](027-4-3-4-3-constraints-on-xml-representation-of-pattern.md#element-pattern))\*))  
</restriction>

<table class="reprcomp"><thead><tr><th><a href="018-4-datatype-components.md#dc-defn">Simple Type Definition</a><strong> Schema Component</strong></th></tr></thead><tbody><tr><td><table border="0" cellpadding="3"><thead><tr><th align="left">Property</th><th align="left">Representation</th></tr></thead><tbody valign="top"><tr valign="top"><td><a class="propref" href="018-4-datatype-components.md#defn-variety">{variety}</a></td><td>
The <a href="https://www.w3.org/TR/2004/REC-xmlschema-1-20041028/structures.html#key-vv">actual value</a> of <a class="propref" href="018-4-datatype-components.md#defn-variety">{variety}</a> of <a class="propref" href="018-4-datatype-components.md#defn-basetype">{base type definition}</a>
</td></tr><tr valign="top"><td><a class="propref" href="018-4-datatype-components.md#defn-facets">{facets}</a></td><td>
The union of the set of <a href="004-2-3-lexical-space.md#facets">Facets (§2.4)</a> components
resolved to by the facet <a class="xpropref" href="https://www.w3.org/TR/xml-infoset/#infoitem.element">[children]</a> merged with <a class="propref" href="018-4-datatype-components.md#defn-facets">{facets}</a>
from <a class="propref" href="018-4-datatype-components.md#defn-basetype">{base type definition}</a>, subject to the Facet Restriction Valid
constraints specified in <a href="004-2-3-lexical-space.md#facets">Facets (§2.4)</a>.
</td></tr><tr valign="top"><td><a class="propref" href="018-4-datatype-components.md#defn-basetype">{base type definition}</a></td><td>
The <a href="018-4-datatype-components.md#dc-defn">Simple Type Definition</a> component resolved to by the <a href="https://www.w3.org/TR/2004/REC-xmlschema-1-20041028/structures.html#key-vv">actual value</a> of the
<code>base</code> <a class="xpropref" href="https://www.w3.org/TR/xml-infoset/#infoitem.attribute">[attribute]</a> or the <a class="eltref" href="018-4-datatype-components.md#element-simpleType">&lt;simpleType&gt;</a> <a class="xpropref" href="https://www.w3.org/TR/xml-infoset/#infoitem.element">[children]</a>,
whichever is present.
</td></tr></tbody></table></td></tr></tbody></table>

Example

An electronic commerce schema might define a datatype called
*Sku* (the barcode number that appears on products) from the
[·built-in·](006-2-5-2-3-derived-by-union.md#dt-built-in) datatype [string](006-2-5-2-3-derived-by-union.md#string) by
supplying a value for the [·pattern·](026-4-3-3-maxlength.md#dt-pattern) facet.



```
<simpleType name='Sku'>
    <restriction base='string'>
      <pattern value='\d{3}-[A-Z]{2}'/>
    </restriction>
</simpleType>
```




In this case, *Sku* is the name of the new
[·user-derived·](006-2-5-2-3-derived-by-union.md#dt-user-derived) datatype, [string](006-2-5-2-3-derived-by-union.md#string) is
its [·base type·](005-2-5-1-3-union-datatypes.md#dt-basetype) and [·pattern·](026-4-3-3-maxlength.md#dt-pattern)
is the facet.

<a id="derivation-by-list"></a>

##### 4.1.2.2 Derivation by list

XML Representation Summary: `list` Element Information Item

<a id="element-list"></a>

<list  
  id = [ID](014-3-3-4-1-constraining-facets.md#ID)  
  itemType = [QName](012-3-2-15-hexbinary.md#QName)  
  *{any attributes with non-schema namespace . . .}*>  
*Content:* ([annotation](https://www.w3.org/TR/2004/REC-xmlschema-1-20041028/structures.html#element-annotation)?, [simpleType](018-4-datatype-components.md#element-simpleType)?)  
</list>

<table class="reprcomp"><thead><tr><th><a href="018-4-datatype-components.md#dc-defn">Simple Type Definition</a><strong> Schema Component</strong></th></tr></thead><tbody><tr><td><table border="0" cellpadding="3"><thead><tr><th align="left">Property</th><th align="left">Representation</th></tr></thead><tbody valign="top"><tr valign="top"><td><a class="propref" href="018-4-datatype-components.md#defn-variety">{variety}</a></td><td>
list
</td></tr><tr valign="top"><td><a class="propref" href="018-4-datatype-components.md#defn-itemType">{item type definition}</a></td><td>
The <a href="018-4-datatype-components.md#dc-defn">Simple Type Definition</a> component resolved to by the <a href="https://www.w3.org/TR/2004/REC-xmlschema-1-20041028/structures.html#key-vv">actual value</a> of the
<code>itemType</code> <a class="xpropref" href="https://www.w3.org/TR/xml-infoset/#infoitem.attribute">[attribute]</a>
or the <a class="eltref" href="018-4-datatype-components.md#element-simpleType">&lt;simpleType&gt;</a> <a class="xpropref" href="https://www.w3.org/TR/xml-infoset/#infoitem.element">[children]</a>,
whichever is present.
</td></tr></tbody></table></td></tr></tbody></table>

A [·list·](004-2-3-lexical-space.md#dt-list) datatype must be [·derived·](005-2-5-1-3-union-datatypes.md#dt-derived)
from an [·atomic·](004-2-3-lexical-space.md#dt-atomic) or a [·union·](004-2-3-lexical-space.md#dt-union) datatype,
known as the
[·itemType·](004-2-3-lexical-space.md#dt-itemType) of the [·list·](004-2-3-lexical-space.md#dt-list) datatype.
This yields a datatype whose [·value space·](003-1-4-terminology.md#dt-value-space) is composed of
finite-length sequences of values from the [·value space·](003-1-4-terminology.md#dt-value-space) of the
[·itemType·](004-2-3-lexical-space.md#dt-itemType) and whose [·lexical space·](004-2-3-lexical-space.md#dt-lexical-space) is
composed of space-separated lists of literals of the
[·itemType·](004-2-3-lexical-space.md#dt-itemType).

Example

A system might want to store lists of floating point values.



```
<simpleType name='listOfFloat'>
  <list itemType='float'/>
</simpleType>
```




In this case, *listOfFloat* is the name of the new
[·user-derived·](006-2-5-2-3-derived-by-union.md#dt-user-derived) datatype, [float](007-3-2-3-1-lexical-representation.md#float) is its
[·itemType·](004-2-3-lexical-space.md#dt-itemType) and [·list·](004-2-3-lexical-space.md#dt-list) is the
derivation method.

As mentioned in [List datatypes (§2.5.1.2)](004-2-3-lexical-space.md#list-datatypes),
when a datatype is [·derived·](005-2-5-1-3-union-datatypes.md#dt-derived) from a
[·list·](004-2-3-lexical-space.md#dt-list) datatype, the following
[·constraining facet·](004-2-3-lexical-space.md#dt-constraining-facet)s can be used:

- [·length·](024-4-2-5-numeric.md#dt-length)
- [·maxLength·](026-4-3-3-maxlength.md#dt-maxLength)
- [·minLength·](025-4-3-1-4-constraints-on-length-schema-components.md#dt-minLength)
- [·enumeration·](027-4-3-4-3-constraints-on-xml-representation-of-pattern.md#dt-enumeration)
- [·pattern·](026-4-3-3-maxlength.md#dt-pattern)
- [·whiteSpace·](027-4-3-4-3-constraints-on-xml-representation-of-pattern.md#dt-whiteSpace)

