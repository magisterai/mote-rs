# 3.3.4.1 Constraining facets

Source: [XML Schema Part 2: Datatypes](https://www.w3.org/TR/2004/REC-xmlschema-2-20041028/) · 2nd edition, 2004-10-28.

[Document index](../README.md) · [Topic guide](../../../../README.md) · [Publisher](https://www.w3.org/TR/2004/REC-xmlschema-2-20041028/).

HTML normalized to Markdown. Complex tables and mathematical markup retain HTML; illustrations are stored locally.

[Previous](013-3-2-19-notation.md) · [Next](015-3-3-10-1-constraining-facets.md)

<a id="dt-NMTOKEN"></a>

[Definition:]
**NMTOKEN** represents
the [NMTOKEN attribute type](https://www.w3.org/TR/2000/WD-xml-2e-20000814#NT-TokenizedType)
from [[XML 1.0 (Second Edition)]](044-h-references.md#XML). The [·value space·](003-1-4-terminology.md#dt-value-space) of
**NMTOKEN** is the set of tokens that [·match·](003-1-4-terminology.md#dt-match)
the [Nmtoken](https://www.w3.org/TR/2000/WD-xml-2e-20000814#NT-Nmtoken) production in
[[XML 1.0 (Second Edition)]](044-h-references.md#XML). The [·lexical space·](004-2-3-lexical-space.md#dt-lexical-space) of
**NMTOKEN** is the set of strings that [·match·](003-1-4-terminology.md#dt-match)
the [Nmtoken](https://www.w3.org/TR/2000/WD-xml-2e-20000814#NT-Nmtoken) production in
[[XML 1.0 (Second Edition)]](044-h-references.md#XML). The [·base type·](005-2-5-1-3-union-datatypes.md#dt-basetype) of
**NMTOKEN** is [token](013-3-2-19-notation.md#token).

For compatibility (see [Terminology (§1.4)](003-1-4-terminology.md#terminology)) **NMTOKEN**
should be used only on attributes.

<a id="NMTOKEN-facets"></a>

##### 3.3.4.1 Constraining facets

**NMTOKEN** has the
following [·constraining facets·](004-2-3-lexical-space.md#dt-constraining-facet):

- [length](024-4-2-5-numeric.md#rf-length)
- [minLength](025-4-3-1-4-constraints-on-length-schema-components.md#rf-minLength)
- [maxLength](026-4-3-3-maxlength.md#rf-maxLength)
- [pattern](026-4-3-3-maxlength.md#rf-pattern)
- [enumeration](027-4-3-4-3-constraints-on-xml-representation-of-pattern.md#rf-enumeration)
- [whiteSpace](027-4-3-4-3-constraints-on-xml-representation-of-pattern.md#rf-whiteSpace)

<a id="NMTOKEN-derived-types"></a>

##### 3.3.4.2 Derived datatypes

The following [·built-in·](006-2-5-2-3-derived-by-union.md#dt-built-in)
datatypes are [·derived·](005-2-5-1-3-union-datatypes.md#dt-derived) from
**NMTOKEN**:

- [NMTOKENS](014-3-3-4-1-constraining-facets.md#NMTOKENS)

<a id="NMTOKENS"></a>

#### 3.3.5 NMTOKENS

<a id="dt-NMTOKENS"></a>

[Definition:]
**NMTOKENS**
represents the [NMTOKENS attribute
type](https://www.w3.org/TR/2000/WD-xml-2e-20000814#NT-TokenizedType) from [[XML 1.0 (Second Edition)]](044-h-references.md#XML). The [·value space·](003-1-4-terminology.md#dt-value-space)
of **NMTOKENS** is the set of finite, non-zero-length sequences of
[·NMTOKEN·](014-3-3-4-1-constraining-facets.md#dt-NMTOKEN)s. The [·lexical space·](004-2-3-lexical-space.md#dt-lexical-space)
of **NMTOKENS** is the set of space-separated lists of tokens,
of which each token is in the [·lexical space·](004-2-3-lexical-space.md#dt-lexical-space) of
[NMTOKEN](013-3-2-19-notation.md#NMTOKEN). The [·itemType·](004-2-3-lexical-space.md#dt-itemType) of
**NMTOKENS** is [NMTOKEN](013-3-2-19-notation.md#NMTOKEN).

For compatibility (see [Terminology (§1.4)](003-1-4-terminology.md#terminology))
**NMTOKENS** should be used only on attributes.

<a id="NMTOKENS-facets"></a>

##### 3.3.5.1 Constraining facets

**NMTOKENS** has the
following [·constraining facets·](004-2-3-lexical-space.md#dt-constraining-facet):

- [length](024-4-2-5-numeric.md#rf-length)
- [minLength](025-4-3-1-4-constraints-on-length-schema-components.md#rf-minLength)
- [maxLength](026-4-3-3-maxlength.md#rf-maxLength)
- [enumeration](027-4-3-4-3-constraints-on-xml-representation-of-pattern.md#rf-enumeration)
- [whiteSpace](027-4-3-4-3-constraints-on-xml-representation-of-pattern.md#rf-whiteSpace)
- [pattern](026-4-3-3-maxlength.md#rf-pattern)

<a id="Name"></a>

#### 3.3.6 Name

<a id="dt-Name"></a>

[Definition:]
**Name**
represents [XML Names](https://www.w3.org/TR/2000/WD-xml-2e-20000814#dt-name).
The [·value space·](003-1-4-terminology.md#dt-value-space) of **Name** is
the set of all strings which [·match·](003-1-4-terminology.md#dt-match) the
[Name](https://www.w3.org/TR/2000/WD-xml-2e-20000814#NT-Name) production of
[[XML 1.0 (Second Edition)]](044-h-references.md#XML). The [·lexical space·](004-2-3-lexical-space.md#dt-lexical-space) of
**Name** is the set of all strings which [·match·](003-1-4-terminology.md#dt-match)
the [Name](https://www.w3.org/TR/2000/WD-xml-2e-20000814#NT-Name) production of
[[XML 1.0 (Second Edition)]](044-h-references.md#XML). The [·base type·](005-2-5-1-3-union-datatypes.md#dt-basetype) of **Name**
is [token](013-3-2-19-notation.md#token).

<a id="Name-facets"></a>

##### 3.3.6.1 Constraining facets

**Name** has the
following [·constraining facets·](004-2-3-lexical-space.md#dt-constraining-facet):

- [length](024-4-2-5-numeric.md#rf-length)
- [minLength](025-4-3-1-4-constraints-on-length-schema-components.md#rf-minLength)
- [maxLength](026-4-3-3-maxlength.md#rf-maxLength)
- [pattern](026-4-3-3-maxlength.md#rf-pattern)
- [enumeration](027-4-3-4-3-constraints-on-xml-representation-of-pattern.md#rf-enumeration)
- [whiteSpace](027-4-3-4-3-constraints-on-xml-representation-of-pattern.md#rf-whiteSpace)

<a id="Name-derived-types"></a>

##### 3.3.6.2 Derived datatypes

The following [·built-in·](006-2-5-2-3-derived-by-union.md#dt-built-in)
datatypes are [·derived·](005-2-5-1-3-union-datatypes.md#dt-derived) from
**Name**:

- [NCName](014-3-3-4-1-constraining-facets.md#NCName)

<a id="NCName"></a>

#### 3.3.7 NCName

<a id="dt-NCName"></a>

[Definition:]
**NCName** represents XML
"non-colonized" Names. The [·value space·](003-1-4-terminology.md#dt-value-space) of
**NCName** is the set of all strings which [·match·](003-1-4-terminology.md#dt-match)
the [NCName](https://www.w3.org/TR/1999/REC-xml-names-19990114/#NT-NCName) production of
[[Namespaces in XML]](044-h-references.md#XMLNS). The [·lexical space·](004-2-3-lexical-space.md#dt-lexical-space) of
**NCName** is the set of all strings which [·match·](003-1-4-terminology.md#dt-match)
the [NCName](https://www.w3.org/TR/1999/REC-xml-names-19990114/#NT-NCName) production of
[[Namespaces in XML]](044-h-references.md#XMLNS). The [·base type·](005-2-5-1-3-union-datatypes.md#dt-basetype) of
**NCName** is [Name](014-3-3-4-1-constraining-facets.md#Name).

<a id="NCName-facets"></a>

##### 3.3.7.1 Constraining facets

**NCName** has the
following [·constraining facets·](004-2-3-lexical-space.md#dt-constraining-facet):

- [length](024-4-2-5-numeric.md#rf-length)
- [minLength](025-4-3-1-4-constraints-on-length-schema-components.md#rf-minLength)
- [maxLength](026-4-3-3-maxlength.md#rf-maxLength)
- [pattern](026-4-3-3-maxlength.md#rf-pattern)
- [enumeration](027-4-3-4-3-constraints-on-xml-representation-of-pattern.md#rf-enumeration)
- [whiteSpace](027-4-3-4-3-constraints-on-xml-representation-of-pattern.md#rf-whiteSpace)

<a id="NCName-derived-types"></a>

##### 3.3.7.2 Derived datatypes

The following [·built-in·](006-2-5-2-3-derived-by-union.md#dt-built-in)
datatypes are [·derived·](005-2-5-1-3-union-datatypes.md#dt-derived) from
**NCName**:

- [ID](014-3-3-4-1-constraining-facets.md#ID)
- [IDREF](014-3-3-4-1-constraining-facets.md#IDREF)
- [ENTITY](015-3-3-10-1-constraining-facets.md#ENTITY)

<a id="ID"></a>

#### 3.3.8 ID

<a id="dt-ID"></a>

[Definition:]
**ID** represents the
[ID attribute type](https://www.w3.org/TR/2000/WD-xml-2e-20000814#NT-TokenizedType) from
[[XML 1.0 (Second Edition)]](044-h-references.md#XML). The [·value space·](003-1-4-terminology.md#dt-value-space) of
**ID** is the set of all strings that [·match·](003-1-4-terminology.md#dt-match)
the [NCName](https://www.w3.org/TR/1999/REC-xml-names-19990114/#NT-NCName) production in
[[Namespaces in XML]](044-h-references.md#XMLNS). The
[·lexical space·](004-2-3-lexical-space.md#dt-lexical-space) of **ID** is the set of all
strings that [·match·](003-1-4-terminology.md#dt-match) the
[NCName](https://www.w3.org/TR/1999/REC-xml-names-19990114/#NT-NCName) production in
[[Namespaces in XML]](044-h-references.md#XMLNS).
The [·base type·](005-2-5-1-3-union-datatypes.md#dt-basetype) of **ID** is [NCName](014-3-3-4-1-constraining-facets.md#NCName).

For compatibility (see [Terminology (§1.4)](003-1-4-terminology.md#terminology))
**ID** should be used only on attributes.

<a id="ID-facets"></a>

##### 3.3.8.1 Constraining facets

**ID** has the
following [·constraining facets·](004-2-3-lexical-space.md#dt-constraining-facet):

- [length](024-4-2-5-numeric.md#rf-length)
- [minLength](025-4-3-1-4-constraints-on-length-schema-components.md#rf-minLength)
- [maxLength](026-4-3-3-maxlength.md#rf-maxLength)
- [pattern](026-4-3-3-maxlength.md#rf-pattern)
- [enumeration](027-4-3-4-3-constraints-on-xml-representation-of-pattern.md#rf-enumeration)
- [whiteSpace](027-4-3-4-3-constraints-on-xml-representation-of-pattern.md#rf-whiteSpace)

<a id="IDREF"></a>

#### 3.3.9 IDREF

<a id="dt-IDREF"></a>

[Definition:]
**IDREF** represents the
[IDREF attribute type](https://www.w3.org/TR/2000/WD-xml-2e-20000814#NT-TokenizedType) from
[[XML 1.0 (Second Edition)]](044-h-references.md#XML). The [·value space·](003-1-4-terminology.md#dt-value-space) of
**IDREF** is the set of all strings that [·match·](003-1-4-terminology.md#dt-match)
the [NCName](https://www.w3.org/TR/1999/REC-xml-names-19990114/#NT-NCName) production in
[[Namespaces in XML]](044-h-references.md#XMLNS). The
[·lexical space·](004-2-3-lexical-space.md#dt-lexical-space) of **IDREF** is the set of
strings that [·match·](003-1-4-terminology.md#dt-match) the
[NCName](https://www.w3.org/TR/1999/REC-xml-names-19990114/#NT-NCName) production in
[[Namespaces in XML]](044-h-references.md#XMLNS).
The [·base type·](005-2-5-1-3-union-datatypes.md#dt-basetype) of **IDREF** is [NCName](014-3-3-4-1-constraining-facets.md#NCName).

For compatibility (see [Terminology (§1.4)](003-1-4-terminology.md#terminology)) this datatype
should be used only on attributes.

<a id="IDREF-facets"></a>

##### 3.3.9.1 Constraining facets

**IDREF** has the
following [·constraining facets·](004-2-3-lexical-space.md#dt-constraining-facet):

- [length](024-4-2-5-numeric.md#rf-length)
- [minLength](025-4-3-1-4-constraints-on-length-schema-components.md#rf-minLength)
- [maxLength](026-4-3-3-maxlength.md#rf-maxLength)
- [pattern](026-4-3-3-maxlength.md#rf-pattern)
- [enumeration](027-4-3-4-3-constraints-on-xml-representation-of-pattern.md#rf-enumeration)
- [whiteSpace](027-4-3-4-3-constraints-on-xml-representation-of-pattern.md#rf-whiteSpace)

<a id="IDREF-derived-types"></a>

##### 3.3.9.2 Derived datatypes

The following [·built-in·](006-2-5-2-3-derived-by-union.md#dt-built-in)
datatypes are [·derived·](005-2-5-1-3-union-datatypes.md#dt-derived) from
**IDREF**:

- [IDREFS](014-3-3-4-1-constraining-facets.md#IDREFS)

<a id="IDREFS"></a>

#### 3.3.10 IDREFS

<a id="dt-IDREFS"></a>

[Definition:]
**IDREFS** represents the
[IDREFS attribute type](https://www.w3.org/TR/2000/WD-xml-2e-20000814#NT-TokenizedType) from
[[XML 1.0 (Second Edition)]](044-h-references.md#XML). The [·value space·](003-1-4-terminology.md#dt-value-space) of
**IDREFS** is the set of finite, non-zero-length sequences of
[IDREF](014-3-3-4-1-constraining-facets.md#IDREF)s.
The [·lexical space·](004-2-3-lexical-space.md#dt-lexical-space) of **IDREFS** is the
set of space-separated lists of tokens, of which each token is in the
[·lexical space·](004-2-3-lexical-space.md#dt-lexical-space) of [IDREF](014-3-3-4-1-constraining-facets.md#IDREF).
The [·itemType·](004-2-3-lexical-space.md#dt-itemType) of **IDREFS** is
[IDREF](014-3-3-4-1-constraining-facets.md#IDREF).

