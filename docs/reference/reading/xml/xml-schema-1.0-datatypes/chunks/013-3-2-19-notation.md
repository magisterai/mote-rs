# 3.2.19 NOTATION

Source: [XML Schema Part 2: Datatypes](https://www.w3.org/TR/2004/REC-xmlschema-2-20041028/) · 2nd edition, 2004-10-28.

[Document index](../README.md) · [Topic guide](../../../../README.md) · [Publisher](https://www.w3.org/TR/2004/REC-xmlschema-2-20041028/).

HTML normalized to Markdown. Complex tables and mathematical markup retain HTML; illustrations are stored locally.

[Previous](012-3-2-15-hexbinary.md) · [Next](014-3-3-4-1-constraining-facets.md)

- [length](024-4-2-5-numeric.md#rf-length)
- [minLength](025-4-3-1-4-constraints-on-length-schema-components.md#rf-minLength)
- [maxLength](026-4-3-3-maxlength.md#rf-maxLength)
- [pattern](026-4-3-3-maxlength.md#rf-pattern)
- [enumeration](027-4-3-4-3-constraints-on-xml-representation-of-pattern.md#rf-enumeration)
- [whiteSpace](027-4-3-4-3-constraints-on-xml-representation-of-pattern.md#rf-whiteSpace)

The use of [·length·](024-4-2-5-numeric.md#dt-length), [·minLength·](025-4-3-1-4-constraints-on-length-schema-components.md#dt-minLength) and
[·maxLength·](026-4-3-3-maxlength.md#dt-maxLength)
on datatypes [·derived·](005-2-5-1-3-union-datatypes.md#dt-derived) from [QName](012-3-2-15-hexbinary.md#QName) is
deprecated. Future versions of this specification may
remove these facets for this datatype.

<a id="NOTATION"></a>

#### 3.2.19 NOTATION

<a id="dt-NOTATION"></a>

[Definition:]
**NOTATION**
represents the [NOTATION](https://www.w3.org/TR/2000/WD-xml-2e-20000814#NT-NotationType) attribute
type from [[XML 1.0 (Second Edition)]](044-h-references.md#XML). The [·value space·](003-1-4-terminology.md#dt-value-space)
of **NOTATION** is the set of [QName](012-3-2-15-hexbinary.md#QName)s
of notations declared in the current schema.
The [·lexical space·](004-2-3-lexical-space.md#dt-lexical-space) of **NOTATION** is the set
of all names of [notations](https://www.w3.org/TR/2004/REC-xmlschema-1-20041028/structures.html#declare-notation)
declared in the current schema (in the form of [QName](012-3-2-15-hexbinary.md#QName)s).

<a id="enumeration-required-notation"></a>

**Schema Component Constraint: enumeration facet value required for NOTATION**  

It is an [·error·](003-1-4-terminology.md#dt-error) for **NOTATION**
to be used directly in a schema. Only datatypes that are
[·derived·](005-2-5-1-3-union-datatypes.md#dt-derived) from **NOTATION** by
specifying a value for [·enumeration·](027-4-3-4-3-constraints-on-xml-representation-of-pattern.md#dt-enumeration) can be used
in a schema.

For compatibility (see [Terminology (§1.4)](003-1-4-terminology.md#terminology)) **NOTATION**
should be used only on attributes
and should only be used in schemas with no
target namespace.

<a id="NOTATION-facets"></a>

##### 3.2.19.1 Constraining facets

**NOTATION** has the
following [·constraining facets·](004-2-3-lexical-space.md#dt-constraining-facet):

- [length](024-4-2-5-numeric.md#rf-length)
- [minLength](025-4-3-1-4-constraints-on-length-schema-components.md#rf-minLength)
- [maxLength](026-4-3-3-maxlength.md#rf-maxLength)
- [pattern](026-4-3-3-maxlength.md#rf-pattern)
- [enumeration](027-4-3-4-3-constraints-on-xml-representation-of-pattern.md#rf-enumeration)
- [whiteSpace](027-4-3-4-3-constraints-on-xml-representation-of-pattern.md#rf-whiteSpace)

The use of [·length·](024-4-2-5-numeric.md#dt-length), [·minLength·](025-4-3-1-4-constraints-on-length-schema-components.md#dt-minLength) and [·maxLength·](026-4-3-3-maxlength.md#dt-maxLength)
on datatypes [·derived·](005-2-5-1-3-union-datatypes.md#dt-derived) from [NOTATION](013-3-2-19-notation.md#NOTATION) is
deprecated. Future versions of this specification may
remove these facets for this datatype.

<a id="built-in-derived"></a>

### 3.3 Derived datatypes

3.3.1 [normalizedString](013-3-2-19-notation.md#normalizedString)  
        3.3.2 [token](013-3-2-19-notation.md#token)  
        3.3.3 [language](013-3-2-19-notation.md#language)  
        3.3.4 [NMTOKEN](013-3-2-19-notation.md#NMTOKEN)  
        3.3.5 [NMTOKENS](014-3-3-4-1-constraining-facets.md#NMTOKENS)  
        3.3.6 [Name](014-3-3-4-1-constraining-facets.md#Name)  
        3.3.7 [NCName](014-3-3-4-1-constraining-facets.md#NCName)  
        3.3.8 [ID](014-3-3-4-1-constraining-facets.md#ID)  
        3.3.9 [IDREF](014-3-3-4-1-constraining-facets.md#IDREF)  
        3.3.10 [IDREFS](014-3-3-4-1-constraining-facets.md#IDREFS)  
        3.3.11 [ENTITY](015-3-3-10-1-constraining-facets.md#ENTITY)  
        3.3.12 [ENTITIES](015-3-3-10-1-constraining-facets.md#ENTITIES)  
        3.3.13 [integer](015-3-3-10-1-constraining-facets.md#integer)  
        3.3.14 [nonPositiveInteger](015-3-3-10-1-constraining-facets.md#nonPositiveInteger)  
        3.3.15 [negativeInteger](015-3-3-10-1-constraining-facets.md#negativeInteger)  
        3.3.16 [long](016-3-3-16-long.md#long)  
        3.3.17 [int](016-3-3-16-long.md#int)  
        3.3.18 [short](016-3-3-16-long.md#short)  
        3.3.19 [byte](016-3-3-16-long.md#byte)  
        3.3.20 [nonNegativeInteger](016-3-3-16-long.md#nonNegativeInteger)  
        3.3.21 [unsignedLong](017-3-3-21-unsignedlong.md#unsignedLong)  
        3.3.22 [unsignedInt](017-3-3-21-unsignedlong.md#unsignedInt)  
        3.3.23 [unsignedShort](017-3-3-21-unsignedlong.md#unsignedShort)  
        3.3.24 [unsignedByte](017-3-3-21-unsignedlong.md#unsignedByte)  
        3.3.25 [positiveInteger](017-3-3-21-unsignedlong.md#positiveInteger)

This section gives conceptual definitions for all
[·built-in·](006-2-5-2-3-derived-by-union.md#dt-built-in) [·derived·](005-2-5-1-3-union-datatypes.md#dt-derived) datatypes
defined by this specification. The XML representation used to define
[·derived·](005-2-5-1-3-union-datatypes.md#dt-derived) datatypes (whether
[·built-in·](006-2-5-2-3-derived-by-union.md#dt-built-in) or [·user-derived·](006-2-5-2-3-derived-by-union.md#dt-user-derived)) is
given in section [XML Representation of Simple Type Definition Schema Components (§4.1.2)](018-4-datatype-components.md#xr-defn) and the complete
definitions of the [·built-in·](006-2-5-2-3-derived-by-union.md#dt-built-in) 
[·derived·](005-2-5-1-3-union-datatypes.md#dt-derived) datatypes are provided in Appendix A
[Schema for Datatype Definitions (normative) (§A)](033-5-conformance.md#schema).

<a id="normalizedString"></a>

#### 3.3.1 normalizedString

<a id="dt-normalizedString"></a>

[Definition:]
**normalizedString**
represents white space normalized strings.
The [·value space·](003-1-4-terminology.md#dt-value-space) of **normalizedString** is the
set of strings that do not
contain the carriage return (#xD), line feed (#xA) nor tab (#x9) characters.
The [·lexical space·](004-2-3-lexical-space.md#dt-lexical-space) of **normalizedString** is the
set of strings that do not
contain the carriage return (#xD),
line feed (#xA)
nor tab (#x9) characters.
The [·base type·](005-2-5-1-3-union-datatypes.md#dt-basetype) of **normalizedString** is [string](006-2-5-2-3-derived-by-union.md#string).

<a id="normalizedString-facets"></a>

##### 3.3.1.1 Constraining facets

**normalizedString** has the
following [·constraining facets·](004-2-3-lexical-space.md#dt-constraining-facet):

- [length](024-4-2-5-numeric.md#rf-length)
- [minLength](025-4-3-1-4-constraints-on-length-schema-components.md#rf-minLength)
- [maxLength](026-4-3-3-maxlength.md#rf-maxLength)
- [pattern](026-4-3-3-maxlength.md#rf-pattern)
- [enumeration](027-4-3-4-3-constraints-on-xml-representation-of-pattern.md#rf-enumeration)
- [whiteSpace](027-4-3-4-3-constraints-on-xml-representation-of-pattern.md#rf-whiteSpace)

<a id="normalizedString-derived-types"></a>

##### 3.3.1.2 Derived datatypes

The following [·built-in·](006-2-5-2-3-derived-by-union.md#dt-built-in)
datatypes are [·derived·](005-2-5-1-3-union-datatypes.md#dt-derived) from
**normalizedString**:

- [token](013-3-2-19-notation.md#token)

<a id="token"></a>

#### 3.3.2 token

<a id="dt-token"></a>

[Definition:]
**token**
represents tokenized strings.
The [·value space·](003-1-4-terminology.md#dt-value-space) of **token** is the
set of strings that do not
contain the
carriage return (#xD),
line feed (#xA) nor tab (#x9) characters, that have no
leading or trailing spaces (#x20) and that have no internal sequences
of two or more spaces.
The [·lexical space·](004-2-3-lexical-space.md#dt-lexical-space) of **token** is the
set of strings that do not contain the
carriage return (#xD),
line feed (#xA) nor tab (#x9) characters, that have no
leading or trailing spaces (#x20) and that have no internal sequences
of two or more spaces.
The [·base type·](005-2-5-1-3-union-datatypes.md#dt-basetype) of **token** is [normalizedString](013-3-2-19-notation.md#normalizedString).

<a id="token-facets"></a>

##### 3.3.2.1 Constraining facets

**token** has the
following [·constraining facets·](004-2-3-lexical-space.md#dt-constraining-facet):

- [length](024-4-2-5-numeric.md#rf-length)
- [minLength](025-4-3-1-4-constraints-on-length-schema-components.md#rf-minLength)
- [maxLength](026-4-3-3-maxlength.md#rf-maxLength)
- [pattern](026-4-3-3-maxlength.md#rf-pattern)
- [enumeration](027-4-3-4-3-constraints-on-xml-representation-of-pattern.md#rf-enumeration)
- [whiteSpace](027-4-3-4-3-constraints-on-xml-representation-of-pattern.md#rf-whiteSpace)

<a id="token-derived-types"></a>

##### 3.3.2.2 Derived datatypes

The following [·built-in·](006-2-5-2-3-derived-by-union.md#dt-built-in)
datatypes are [·derived·](005-2-5-1-3-union-datatypes.md#dt-derived) from
**token**:

- [language](013-3-2-19-notation.md#language)
- [NMTOKEN](013-3-2-19-notation.md#NMTOKEN)
- [Name](014-3-3-4-1-constraining-facets.md#Name)

<a id="language"></a>

#### 3.3.3 language

<a id="dt-language"></a>

[Definition:]
**language**
represents natural language identifiers as defined by
by [[RFC 3066]](044-h-references.md#RFC3066)
.
The [·value space·](003-1-4-terminology.md#dt-value-space) of **language** is the
set of all strings that are valid language identifiers as defined
[[RFC 3066]](044-h-references.md#RFC3066)
.
The [·lexical space·](004-2-3-lexical-space.md#dt-lexical-space) of
**language** is the set of all strings that
conform to the pattern `[a-zA-Z]{1,8}(-[a-zA-Z0-9]{1,8})*`
.
The [·base type·](005-2-5-1-3-union-datatypes.md#dt-basetype) of **language** is [token](013-3-2-19-notation.md#token).

<a id="language-facets"></a>

##### 3.3.3.1 Constraining facets

**language** has the
following [·constraining facets·](004-2-3-lexical-space.md#dt-constraining-facet):

- [length](024-4-2-5-numeric.md#rf-length)
- [minLength](025-4-3-1-4-constraints-on-length-schema-components.md#rf-minLength)
- [maxLength](026-4-3-3-maxlength.md#rf-maxLength)
- [pattern](026-4-3-3-maxlength.md#rf-pattern)
- [enumeration](027-4-3-4-3-constraints-on-xml-representation-of-pattern.md#rf-enumeration)
- [whiteSpace](027-4-3-4-3-constraints-on-xml-representation-of-pattern.md#rf-whiteSpace)

<a id="NMTOKEN"></a>

#### 3.3.4 NMTOKEN

