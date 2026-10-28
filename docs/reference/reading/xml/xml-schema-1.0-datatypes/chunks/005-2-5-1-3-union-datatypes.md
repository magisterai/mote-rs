# 2.5.1.3 Union datatypes

Source: [XML Schema Part 2: Datatypes](https://www.w3.org/TR/2004/REC-xmlschema-2-20041028/) · 2nd edition, 2004-10-28.

[Document index](../README.md) · [Topic guide](../../../../README.md) · [Publisher](https://www.w3.org/TR/2004/REC-xmlschema-2-20041028/).

HTML normalized to Markdown. Complex tables and mathematical markup retain HTML; illustrations are stored locally.

[Previous](004-2-3-lexical-space.md) · [Next](006-2-5-2-3-derived-by-union.md)

For each of [·length·](024-4-2-5-numeric.md#dt-length), [·maxLength·](026-4-3-3-maxlength.md#dt-maxLength)
and [·minLength·](025-4-3-1-4-constraints-on-length-schema-components.md#dt-minLength), the *unit of length* is
measured in number of list items. The value of [·whiteSpace·](027-4-3-4-3-constraints-on-xml-representation-of-pattern.md#dt-whiteSpace)
is fixed to the value *collapse*.

For [·list·](004-2-3-lexical-space.md#dt-list) datatypes the [·lexical space·](004-2-3-lexical-space.md#dt-lexical-space)
is composed of space-separated
literals of its [·itemType·](004-2-3-lexical-space.md#dt-itemType). Hence, any
[·pattern·](026-4-3-3-maxlength.md#dt-pattern) specified when a new datatype is
[·derived·](005-2-5-1-3-union-datatypes.md#dt-derived) from a [·list·](004-2-3-lexical-space.md#dt-list) datatype is matched against
each literal of the [·list·](004-2-3-lexical-space.md#dt-list) datatype and
not against the literals of the datatype that serves as its
[·itemType·](004-2-3-lexical-space.md#dt-itemType).

Example



```

<xs:simpleType name='myList'>
	<xs:list itemType='xs:integer'/>
</xs:simpleType>
<xs:simpleType name='myRestrictedList'>
	<xs:restriction base='myList'>
		<xs:pattern value='123 (\d+\s)*456'/>
	</xs:restriction>
</xs:simpleType>
<someElement xsi:type='myRestrictedList'>123 456</someElement>
<someElement xsi:type='myRestrictedList'>123 987 456</someElement>
<someElement xsi:type='myRestrictedList'>123 987 567 456</someElement>

```




The [canonical-lexical-representation](004-2-3-lexical-space.md#canonical-lexical-representation) for the
[·list·](004-2-3-lexical-space.md#dt-list) datatype is defined as the lexical form in which
each item in the [·list·](004-2-3-lexical-space.md#dt-list) has the canonical lexical
representation of its [·itemType·](004-2-3-lexical-space.md#dt-itemType).

<a id="union-datatypes"></a>

##### 2.5.1.3 Union datatypes

The [·value space·](003-1-4-terminology.md#dt-value-space) and [·lexical space·](004-2-3-lexical-space.md#dt-lexical-space)
of a [·union·](004-2-3-lexical-space.md#dt-union) datatype are the union of the
[·value space·](003-1-4-terminology.md#dt-value-space)s and [·lexical space·](004-2-3-lexical-space.md#dt-lexical-space)s of
its [·memberTypes·](005-2-5-1-3-union-datatypes.md#dt-memberTypes).
[·union·](004-2-3-lexical-space.md#dt-union) datatypes are always [·derived·](005-2-5-1-3-union-datatypes.md#dt-derived).
Currently, there are no [·built-in·](006-2-5-2-3-derived-by-union.md#dt-built-in) [·union·](004-2-3-lexical-space.md#dt-union)
datatypes.

Example

A prototypical example of a [·union·](004-2-3-lexical-space.md#dt-union) type is the
[maxOccurs attribute](https://www.w3.org/TR/2004/REC-xmlschema-1-20041028/structures.html#p-max_occurs) on the
[element element](https://www.w3.org/TR/2004/REC-xmlschema-1-20041028/structures.html#element-element)
in XML Schema itself: it is a union of nonNegativeInteger
and an enumeration with the single member, the string "unbounded", as shown below.



```

  <attributeGroup name="occurs">
    <attribute name="minOccurs" type="nonNegativeInteger"
    	use="optional" default="1"/>
    <attribute name="maxOccurs"use="optional" default="1">
      <simpleType>
        <union>
          <simpleType>
            <restriction base='nonNegativeInteger'/>
          </simpleType>
          <simpleType>
            <restriction base='string'>
              <enumeration value='unbounded'/>
            </restriction>
          </simpleType>
        </union>
      </simpleType>
    </attribute>
  </attributeGroup>
```




Any number (greater than 1) of [·atomic·](004-2-3-lexical-space.md#dt-atomic) or [·list·](004-2-3-lexical-space.md#dt-list)
[·datatype·](003-1-4-terminology.md#dt-datatype)s can participate in a [·union·](004-2-3-lexical-space.md#dt-union) type.

<a id="dt-memberTypes"></a>

[Definition:]
The datatypes that participate in the
definition of a [·union·](004-2-3-lexical-space.md#dt-union) datatype are known as the
**memberTypes** of that [·union·](004-2-3-lexical-space.md#dt-union) datatype.

The order in which the [·memberTypes·](005-2-5-1-3-union-datatypes.md#dt-memberTypes) are specified in the
definition (that is, the order of the <simpleType> children of the <union>
element, or the order of the [QName](012-3-2-15-hexbinary.md#QName)s in the *memberTypes*
attribute) is significant.
During validation, an element or attribute's value is validated against the
[·memberTypes·](005-2-5-1-3-union-datatypes.md#dt-memberTypes) in the order in which they appear in the
definition until a match is found. The evaluation order can be overridden
with the use of [xsi:type](https://www.w3.org/TR/2004/REC-xmlschema-1-20041028/structures.html#xsi_type).

Example

For example, given the definition below, the first instance of the <size> element
validates correctly as an [integer (§3.3.13)](015-3-3-10-1-constraining-facets.md#integer), the second and third as
[string (§3.2.1)](006-2-5-2-3-derived-by-union.md#string).



```

  <xsd:element name='size'>
    <xsd:simpleType>
      <xsd:union>
        <xsd:simpleType>
          <xsd:restriction base='integer'/>
        </xsd:simpleType>
        <xsd:simpleType>
          <xsd:restriction base='string'/>
        </xsd:simpleType>
      </xsd:union>
    </xsd:simpleType>
  </xsd:element>
```






```

  <size>1</size>
  <size>large</size>
  <size xsi:type='xsd:string'>1</size>
```




The [canonical-lexical-representation](004-2-3-lexical-space.md#canonical-lexical-representation) for a
[·union·](004-2-3-lexical-space.md#dt-union) datatype is defined as the lexical form in which
the values have the canonical lexical representation
of the appropriate [·memberTypes·](005-2-5-1-3-union-datatypes.md#dt-memberTypes).

**Note:**
A datatype which is [·atomic·](004-2-3-lexical-space.md#dt-atomic) in this specification
need not be an "atomic" datatype in any programming language used to
implement this specification. Likewise, a datatype which is a
[·list·](004-2-3-lexical-space.md#dt-list) in this specification need not be a "list"
datatype in any programming language used to implement this specification.
Furthermore, a datatype which is a [·union·](004-2-3-lexical-space.md#dt-union) in this
specification need not be a "union" datatype in any programming
language used to implement this specification.

<a id="primitive-vs-derived"></a>

#### 2.5.2 Primitive vs. derived datatypes

Next, we distinguish between [·primitive·](005-2-5-1-3-union-datatypes.md#dt-primitive) and
[·derived·](005-2-5-1-3-union-datatypes.md#dt-derived) datatypes.

<a id="dt-primitive"></a>

- [Definition:]**Primitive**
  datatypes are those that are not defined in terms of other datatypes;
  they exist *ab initio*.

<a id="dt-derived"></a>

- [Definition:]**Derived**
  datatypes are those that are defined in terms of other datatypes.

For example, in this specification, [float](007-3-2-3-1-lexical-representation.md#float) is a well-defined
mathematical
concept that cannot be defined in terms of other datatypes, while
a [integer](015-3-3-10-1-constraining-facets.md#integer) is a special case of the more general datatype
[decimal](006-2-5-2-3-derived-by-union.md#decimal).

<a id="dt-anySimpleType"></a>

[Definition:]
The simple ur-type definition is a special restriction of the
[ur-type definition](https://www.w3.org/TR/2004/REC-xmlschema-1-20041028/structures.html#key-urType)
whose name is **anySimpleType** in the XML Schema namespace.
**anySimpleType** can be
considered as the [·base type·](005-2-5-1-3-union-datatypes.md#dt-basetype) of all [·primitive·](005-2-5-1-3-union-datatypes.md#dt-primitive)
datatypes.
**anySimpleType** is considered to have an unconstrained lexical space and a
[·value space·](003-1-4-terminology.md#dt-value-space) consisting of the union of the
[·value space·](003-1-4-terminology.md#dt-value-space)s of all the
[·primitive·](005-2-5-1-3-union-datatypes.md#dt-primitive)
datatypes and the set of all lists of all members of the
[·value space·](003-1-4-terminology.md#dt-value-space)s of all the
[·primitive·](005-2-5-1-3-union-datatypes.md#dt-primitive) datatypes.

The datatypes defined by this specification fall into both
the [·primitive·](005-2-5-1-3-union-datatypes.md#dt-primitive) and [·derived·](005-2-5-1-3-union-datatypes.md#dt-derived)
categories. It is felt that a judiciously chosen set of
[·primitive·](005-2-5-1-3-union-datatypes.md#dt-primitive) datatypes will serve the widest
possible audience by providing a set of convenient datatypes that
can be used as is, as well as providing a rich enough base from
which the variety of datatypes needed by schema designers can be
[·derived·](005-2-5-1-3-union-datatypes.md#dt-derived).

In the example above, [integer](015-3-3-10-1-constraining-facets.md#integer) is [·derived·](005-2-5-1-3-union-datatypes.md#dt-derived)
from [decimal](006-2-5-2-3-derived-by-union.md#decimal).

**Note:**
A datatype which is [·primitive·](005-2-5-1-3-union-datatypes.md#dt-primitive) in this specification
need not be a "primitive" datatype in any programming language used to
implement this specification. Likewise, a datatype which is
[·derived·](005-2-5-1-3-union-datatypes.md#dt-derived) in this specification need not be a
"derived" datatype in any programming language used to implement
this specification.

As described in more detail in [XML Representation of Simple Type Definition Schema Components (§4.1.2)](018-4-datatype-components.md#xr-defn),
each [·user-derived·](006-2-5-2-3-derived-by-union.md#dt-user-derived) datatype [·must·](003-1-4-terminology.md#dt-must)
be defined in terms of another datatype in one of three ways: 1) by assigning
[·constraining facet·](004-2-3-lexical-space.md#dt-constraining-facet)s which serve to *restrict* the
[·value space·](003-1-4-terminology.md#dt-value-space) of the [·user-derived·](006-2-5-2-3-derived-by-union.md#dt-user-derived)
datatype to a subset of that of the [·base type·](005-2-5-1-3-union-datatypes.md#dt-basetype); 2) by creating
a [·list·](004-2-3-lexical-space.md#dt-list) datatype whose [·value space·](003-1-4-terminology.md#dt-value-space)
consists of finite-length sequences of values of its
[·itemType·](004-2-3-lexical-space.md#dt-itemType); or 3) by creating a [·union·](004-2-3-lexical-space.md#dt-union)
datatype whose [·value space·](003-1-4-terminology.md#dt-value-space) consists of the union of the
[·value space·](003-1-4-terminology.md#dt-value-space)s of its [·memberTypes·](005-2-5-1-3-union-datatypes.md#dt-memberTypes).

<a id="restriction"></a>

##### 2.5.2.1 Derived by restriction

<a id="dt-restriction"></a>

[Definition:]A datatype is said to be
[·derived·](005-2-5-1-3-union-datatypes.md#dt-derived) by **restriction** from another datatype
when values for zero or more [·constraining facet·](004-2-3-lexical-space.md#dt-constraining-facet)s are specified
that serve to constrain its [·value space·](003-1-4-terminology.md#dt-value-space) and/or its
[·lexical space·](004-2-3-lexical-space.md#dt-lexical-space) to a subset of those of its
[·base type·](005-2-5-1-3-union-datatypes.md#dt-basetype).

<a id="dt-basetype"></a>

[Definition:]Every
datatype that is [·derived·](005-2-5-1-3-union-datatypes.md#dt-derived) by **restriction**
is defined in terms of an existing datatype, referred to as its
**base type**. **base type**s can be either
[·primitive·](005-2-5-1-3-union-datatypes.md#dt-primitive) or [·derived·](005-2-5-1-3-union-datatypes.md#dt-derived).

<a id="list"></a>

##### 2.5.2.2 Derived by list

