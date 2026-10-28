# 4.3.4.3 Constraints on XML Representation of pattern

Source: [XML Schema Part 2: Datatypes](https://www.w3.org/TR/2004/REC-xmlschema-2-20041028/) · 2nd edition, 2004-10-28.

[Document index](../README.md) · [Topic guide](../../../../README.md) · [Publisher](https://www.w3.org/TR/2004/REC-xmlschema-2-20041028/).

HTML normalized to Markdown. Complex tables and mathematical markup retain HTML; illustrations are stored locally.

[Previous](026-4-3-3-maxlength.md) · [Next](028-4-3-6-1-the-whitespace-schema-component.md)

XML Representation Summary: `pattern` Element Information Item

<a id="element-pattern"></a>

<pattern  
  id = [ID](014-3-3-4-1-constraining-facets.md#ID)  
  **value** = [string](006-2-5-2-3-derived-by-union.md#string)  
  *{any attributes with non-schema namespace . . .}*>  
*Content:* ([annotation](https://www.w3.org/TR/2004/REC-xmlschema-1-20041028/structures.html#element-annotation)?)  
</pattern>

[{value}](026-4-3-3-maxlength.md#pattern-value) [·must·](003-1-4-terminology.md#dt-must) be a valid
[·regular expression·](039-e-1-algorithm.md#dt-regex).

<table class="reprcomp"><thead><tr><th><a href="032-4-3-11-3-totaldigits-validation-rules.md#dc-fractionDigits">pattern</a><strong> Schema Component</strong></th></tr></thead><tbody><tr><td><table border="0" cellpadding="3"><thead><tr><th align="left">Property</th><th align="left">Representation</th></tr></thead><tbody valign="top"><tr valign="top"><td><a class="propref" href="026-4-3-3-maxlength.md#pattern-value">{value}</a></td><td>
The <a href="https://www.w3.org/TR/2004/REC-xmlschema-1-20041028/structures.html#key-vv">actual value</a> of the <code>value</code> <a class="xpropref" href="https://www.w3.org/TR/xml-infoset/#infoitem.attribute">[attribute]</a>
</td></tr><tr valign="top"><td><a class="propref" href="018-4-datatype-components.md#defn-annotation">{annotation}</a></td><td>
The annotations corresponding to all the <a class="eltref" href="https://www.w3.org/TR/2004/REC-xmlschema-1-20041028/structures.html#element-annotation">&lt;annotation&gt;</a>
element information items in the <a class="xpropref" href="https://www.w3.org/TR/xml-infoset/#infoitem.element">[children]</a>, if any.
</td></tr></tbody></table></td></tr></tbody></table>

<a id="pattern-rep-constr"></a>

##### 4.3.4.3 Constraints on XML Representation of pattern

<a id="src-multiple-patterns"></a>

**Schema Representation Constraint: Multiple patterns**  

If multiple [<pattern>](027-4-3-4-3-constraints-on-xml-representation-of-pattern.md#element-pattern) element information items appear as
[[children]](https://www.w3.org/TR/xml-infoset/#infoitem.element) of a [<simpleType>](018-4-datatype-components.md#element-simpleType), the [[value]](https://www.w3.org/TR/xml-infoset/#infoitem.attribute)s should
be combined as if they appeared in a single
[·regular expression·](039-e-1-algorithm.md#dt-regex) as separate
[·branch·](039-e-1-algorithm.md#dt-branch)es.

**Note:**
It is a consequence of the schema representation constraint
[Multiple patterns (§4.3.4.3)](027-4-3-4-3-constraints-on-xml-representation-of-pattern.md#src-multiple-patterns) and of the rules for
[·restriction·](005-2-5-1-3-union-datatypes.md#dt-restriction) that [·pattern·](026-4-3-3-maxlength.md#dt-pattern)
facets specified on the *same* step in a type
derivation are **OR**ed together, while [·pattern·](026-4-3-3-maxlength.md#dt-pattern)
facets specified on *different* steps of a type derivation
are **AND**ed together.Thus, to impose two [·pattern·](026-4-3-3-maxlength.md#dt-pattern) constraints simultaneously,
schema authors may either write a single [·pattern·](026-4-3-3-maxlength.md#dt-pattern) which
expresses the intersection of the two [·pattern·](026-4-3-3-maxlength.md#dt-pattern)s they wish to
impose, or define each [·pattern·](026-4-3-3-maxlength.md#dt-pattern) on a separate type derivation
step.

<a id="pattern-validation-rules"></a>

##### 4.3.4.4 pattern Validation Rules

<a id="cvc-pattern-valid"></a>

**Validation Rule: pattern valid**  

A literal in a [·lexical space·](004-2-3-lexical-space.md#dt-lexical-space) is facet-valid with
respect to [·pattern·](026-4-3-3-maxlength.md#dt-pattern) if:

1
the literal is among the set of character sequences denoted by
the [·regular expression·](039-e-1-algorithm.md#dt-regex) specified in [{value}](026-4-3-3-maxlength.md#pattern-value).

<a id="rf-enumeration"></a>

#### 4.3.5 enumeration

<a id="dt-enumeration"></a>

[Definition:]
**enumeration** constrains the [·value space·](003-1-4-terminology.md#dt-value-space)
to a specified set of values.

**enumeration** does not impose an order relation on the
[·value space·](003-1-4-terminology.md#dt-value-space) it creates; the value of the
[·ordered·](022-4-1-6-simple-type-definition-for-anysimpletype.md#dt-ordered) property of the [·derived·](005-2-5-1-3-union-datatypes.md#dt-derived)
datatype remains that of the datatype from which it is
[·derived·](005-2-5-1-3-union-datatypes.md#dt-derived).

[·enumeration·](027-4-3-4-3-constraints-on-xml-representation-of-pattern.md#dt-enumeration) provides for:

- Constraining a [·value space·](003-1-4-terminology.md#dt-value-space)
  to a specified set of values.

Example

The following example is a datatype definition for a
[·user-derived·](006-2-5-2-3-derived-by-union.md#dt-user-derived) datatype which limits the values
of dates to the three US holidays enumerated. This datatype
definition would appear in a schema authored by an "end-user" and
shows how to define a datatype by enumerating the values in its
[·value space·](003-1-4-terminology.md#dt-value-space). The enumerated values must be
type-valid literals for the [·base type·](005-2-5-1-3-union-datatypes.md#dt-basetype).



```
<simpleType name='holidays'>
    <annotation>
        <documentation>some US holidays</documentation>
    </annotation>
    <restriction base='gMonthDay'>
      <enumeration value='--01-01'>
        <annotation>
            <documentation>New Year's day</documentation>
        </annotation>
      </enumeration>
      <enumeration value='--07-04'>
        <annotation>
            <documentation>4th of July</documentation>
        </annotation>
      </enumeration>
      <enumeration value='--12-25'>
        <annotation>
            <documentation>Christmas</documentation>
        </annotation>
      </enumeration>
    </restriction>
</simpleType>
```




<a id="dc-enumeration"></a>

##### 4.3.5.1 The enumeration Schema Component

Schema Component: [enumeration](027-4-3-4-3-constraints-on-xml-representation-of-pattern.md#dt-enumeration)

<a id="enumeration-value"></a>

{value}
:   A set of values from the [·value space·](003-1-4-terminology.md#dt-value-space) of the
    [{base type definition}](018-4-datatype-components.md#defn-basetype).

<a id="enumeration-annotation"></a>

{annotation}
:   Optional. An [annotation](https://www.w3.org/TR/2004/REC-xmlschema-1-20041028/structures.html#Annotation).

<a id="xr-enumeration"></a>

##### 4.3.5.2 XML Representation of enumeration Schema Components

The XML representation for an [enumeration](027-4-3-4-3-constraints-on-xml-representation-of-pattern.md#dc-enumeration) schema
component is an [<enumeration>](027-4-3-4-3-constraints-on-xml-representation-of-pattern.md#element-enumeration) element information item. The
correspondences between the properties of the information item and
properties of the component are as follows:

XML Representation Summary: `enumeration` Element Information Item

<a id="element-enumeration"></a>

<enumeration  
  id = [ID](014-3-3-4-1-constraining-facets.md#ID)  
  **value** = [anySimpleType](005-2-5-1-3-union-datatypes.md#dt-anySimpleType)  
  *{any attributes with non-schema namespace . . .}*>  
*Content:* ([annotation](https://www.w3.org/TR/2004/REC-xmlschema-1-20041028/structures.html#element-annotation)?)  
</enumeration>

[{value}](027-4-3-4-3-constraints-on-xml-representation-of-pattern.md#enumeration-value) [·must·](003-1-4-terminology.md#dt-must) be
in the [·value space·](003-1-4-terminology.md#dt-value-space) of [{base type definition}](018-4-datatype-components.md#defn-basetype).

<table class="reprcomp"><thead><tr><th><a href="027-4-3-4-3-constraints-on-xml-representation-of-pattern.md#dc-enumeration">enumeration</a><strong> Schema Component</strong></th></tr></thead><tbody><tr><td><table border="0" cellpadding="3"><thead><tr><th align="left">Property</th><th align="left">Representation</th></tr></thead><tbody valign="top"><tr valign="top"><td><a class="propref" href="027-4-3-4-3-constraints-on-xml-representation-of-pattern.md#enumeration-value">{value}</a></td><td>
The <a href="https://www.w3.org/TR/2004/REC-xmlschema-1-20041028/structures.html#key-vv">actual value</a> of the <code>value</code> <a class="xpropref" href="https://www.w3.org/TR/xml-infoset/#infoitem.attribute">[attribute]</a>
</td></tr><tr valign="top"><td><a class="propref" href="018-4-datatype-components.md#defn-annotation">{annotation}</a></td><td>
The annotations corresponding to all the <a class="eltref" href="https://www.w3.org/TR/2004/REC-xmlschema-1-20041028/structures.html#element-annotation">&lt;annotation&gt;</a>
element information items in the <a class="xpropref" href="https://www.w3.org/TR/xml-infoset/#infoitem.element">[children]</a>, if any.
</td></tr></tbody></table></td></tr></tbody></table>

<a id="enumeration-rep-constr"></a>

##### 4.3.5.3 Constraints on XML Representation of enumeration

<a id="src-multiple-enumerations"></a>

**Schema Representation Constraint: Multiple enumerations**  

If multiple [<enumeration>](027-4-3-4-3-constraints-on-xml-representation-of-pattern.md#element-enumeration) element information items appear
as [[children]](https://www.w3.org/TR/xml-infoset/#infoitem.element) of a [<simpleType>](018-4-datatype-components.md#element-simpleType) the
[{value}](027-4-3-4-3-constraints-on-xml-representation-of-pattern.md#enumeration-value) of the [enumeration](027-4-3-4-3-constraints-on-xml-representation-of-pattern.md#dc-enumeration)
component should be the set of all such [[value]](https://www.w3.org/TR/xml-infoset/#infoitem.attribute)s.

<a id="enumeration-validation-rules"></a>

##### 4.3.5.4 enumeration Validation Rules

<a id="cvc-enumeration-valid"></a>

**Validation Rule: enumeration valid**  

A value in a [·value space·](003-1-4-terminology.md#dt-value-space) is facet-valid with
respect to [·enumeration·](027-4-3-4-3-constraints-on-xml-representation-of-pattern.md#dt-enumeration) if
the value is one of the values specified in
[{value}](027-4-3-4-3-constraints-on-xml-representation-of-pattern.md#enumeration-value)

<a id="enumeration-coss"></a>

##### 4.3.5.5 Constraints on enumeration Schema Components

<a id="enumeration-valid-restriction"></a>

**Schema Component Constraint: enumeration valid restriction**  

It is an [·error·](003-1-4-terminology.md#dt-error) if any member of [{value}](027-4-3-4-3-constraints-on-xml-representation-of-pattern.md#enumeration-value) is not in the [·value space·](003-1-4-terminology.md#dt-value-space)
of [{base type definition}](018-4-datatype-components.md#defn-basetype).

<a id="rf-whiteSpace"></a>

#### 4.3.6 whiteSpace

<a id="dt-whiteSpace"></a>

[Definition:]
**whiteSpace** constrains the [·value space·](003-1-4-terminology.md#dt-value-space)
of types [·derived·](005-2-5-1-3-union-datatypes.md#dt-derived) from [string](006-2-5-2-3-derived-by-union.md#string) such that
the various behaviors
specified in [Attribute Value Normalization](https://www.w3.org/TR/2000/WD-xml-2e-20000814#AVNormalize)
in [[XML 1.0 (Second Edition)]](044-h-references.md#XML) are realized. The value of
**whiteSpace** must be one of {preserve, replace, collapse}.

preserve
:   No normalization is done, the value is not changed (this is the
    behavior required by [[XML 1.0 (Second Edition)]](044-h-references.md#XML) for element content)

replace
:   All occurrences of #x9 (tab), #xA (line feed) and #xD (carriage return)
    are replaced with #x20 (space)

collapse
:   After the processing implied by **replace**, contiguous
    sequences of #x20's are collapsed to a single #x20, and leading and
    trailing #x20's are removed.

**Note:**
The notation #xA used here (and elsewhere in this specification) represents
the Universal Character Set (UCS) code point `hexadecimal A` (line feed), which is denoted by
U+000A. This notation is to be distinguished from `&#xA;`,
which is the XML [character reference](https://www.w3.org/TR/2000/WD-xml-2e-20000814#NT-CharRef)
to that same UCS code point.

