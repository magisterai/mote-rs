# 3.2.15 hexBinary

Source: [XML Schema Part 2: Datatypes](https://www.w3.org/TR/2004/REC-xmlschema-2-20041028/) · 2nd edition, 2004-10-28.

[Document index](../README.md) · [Topic guide](../../../../README.md) · [Publisher](https://www.w3.org/TR/2004/REC-xmlschema-2-20041028/).

HTML normalized to Markdown. Complex tables and mathematical markup retain HTML; illustrations are stored locally.

[Previous](011-3-2-10-1-lexical-representation.md) · [Next](013-3-2-19-notation.md)

**gMonth** has the
following [·constraining facets·](004-2-3-lexical-space.md#dt-constraining-facet):

- [pattern](026-4-3-3-maxlength.md#rf-pattern)
- [enumeration](027-4-3-4-3-constraints-on-xml-representation-of-pattern.md#rf-enumeration)
- [whiteSpace](027-4-3-4-3-constraints-on-xml-representation-of-pattern.md#rf-whiteSpace)
- [maxInclusive](028-4-3-6-1-the-whitespace-schema-component.md#rf-maxInclusive)
- [maxExclusive](029-4-3-7-3-maxinclusive-validation-rules.md#rf-maxExclusive)
- [minInclusive](031-4-3-10-mininclusive.md#rf-minInclusive)
- [minExclusive](030-4-3-9-minexclusive.md#rf-minExclusive)

<a id="hexBinary"></a>

#### 3.2.15 hexBinary

<a id="dt-hexBinary"></a>

[Definition:]
**hexBinary** represents
arbitrary hex-encoded binary data. The [·value space·](003-1-4-terminology.md#dt-value-space) of
**hexBinary** is the set of finite-length sequences of binary
octets.

<a id="hexBinary-lexical-representation"></a>

##### 3.2.15.1 Lexical Representation

**hexBinary** has a lexical representation where
each binary octet is encoded as a character tuple, consisting of two
hexadecimal digits ([0-9a-fA-F]) representing the octet code. For example,
"0FB7" is a *hex* encoding for the 16-bit integer 4023
(whose binary representation is 111110110111).

<a id="hexBinary-canonical-repr"></a>

##### 3.2.15.2 Canonical Representation

The canonical representation for **hexBinary** is defined
by prohibiting certain options from the
[Lexical Representation (§3.2.15.1)](012-3-2-15-hexbinary.md#hexBinary-lexical-representation). Specifically, the lower case
hexadecimal digits ([a-f]) are not allowed.

<a id="hexBinary-facets"></a>

##### 3.2.15.3 Constraining facets

**hexBinary** has the
following [·constraining facets·](004-2-3-lexical-space.md#dt-constraining-facet):

- [length](024-4-2-5-numeric.md#rf-length)
- [minLength](025-4-3-1-4-constraints-on-length-schema-components.md#rf-minLength)
- [maxLength](026-4-3-3-maxlength.md#rf-maxLength)
- [pattern](026-4-3-3-maxlength.md#rf-pattern)
- [enumeration](027-4-3-4-3-constraints-on-xml-representation-of-pattern.md#rf-enumeration)
- [whiteSpace](027-4-3-4-3-constraints-on-xml-representation-of-pattern.md#rf-whiteSpace)

<a id="base64Binary"></a>

#### 3.2.16 base64Binary

<a id="dt-base64Binary"></a>

[Definition:]
**base64Binary**
represents Base64-encoded arbitrary binary data. The [·value space·](003-1-4-terminology.md#dt-value-space) of
**base64Binary** is the set of finite-length sequences of binary
octets. For **base64Binary** data the
entire binary stream is encoded using the Base64
Alphabet in
[[RFC 2045]](044-h-references.md#RFC2045).

The lexical forms of **base64Binary** values are limited to the 65 characters
of the Base64 Alphabet defined in [[RFC 2045]](044-h-references.md#RFC2045), i.e., `a-z`,
`A-Z`, `0-9`, the plus sign (+), the forward slash (/) and the
equal sign (=), together with the characters defined in [[XML 1.0 (Second Edition)]](044-h-references.md#XML) as white space.
No other characters are allowed.

For compatibility with older mail gateways, [[RFC 2045]](044-h-references.md#RFC2045) suggests that
base64 data should have lines limited to at most 76 characters in length. This
line-length limitation is not mandated in the lexical forms of **base64Binary**
data and must not be enforced by XML Schema processors.

The lexical space of **base64Binary** is given by the following grammar
(the notation is that used in [[XML 1.0 (Second Edition)]](044-h-references.md#XML)); legal lexical forms must match
the **Base64Binary** production.

`Base64Binary  ::=  ((B64S B64S B64S B64S)*  
                     ((B64S B64S B64S B64) |  
                      (B64S B64S B16S '=') |  
                      (B64S B04S '=' #x20? '=')))?  
  
B64S         ::= B64 #x20?  
  
B16S         ::= B16 #x20?  
  
B04S         ::= B04 #x20?`
`B04         ::=  [AQgw]  
B16         ::=  [AEIMQUYcgkosw048]  
B64         ::=  [A-Za-z0-9+/]`

Note that this grammar requires the number of non-whitespace characters in the lexical
form to be a multiple of four, and for equals signs to appear only at the end of the
lexical form; strings which do not meet these constraints are not legal lexical forms
of **base64Binary** because they cannot successfully be decoded by base64
decoders.

**Note:**The above definition of the lexical space is more restrictive than that
given in [[RFC 2045]](044-h-references.md#RFC2045) as regards whitespace -- this is not an issue
in practice. Any string compatible with the RFC can occur in
an element or attribute validated by this type, because the [·whiteSpace·](027-4-3-4-3-constraints-on-xml-representation-of-pattern.md#dt-whiteSpace) facet of this type is fixed
to *collapse*, which means that all leading and trailing whitespace
will be stripped, and all internal whitespace collapsed to single space
characters, *before* the above grammar is enforced.

The canonical lexical form of a **base64Binary** data value is the base64
encoding of the value which matches the Canonical-base64Binary production in the following
grammar:

`Canonical-base64Binary  ::=  (B64
B64 B64 B64)*  
                               ((B64 B64 B16 '=') | (B64 B04 '=='))?`

**Note:**For some values the canonical form defined above does not conform to
[[RFC 2045]](044-h-references.md#RFC2045), which requires
breaking with linefeeds at appropriate intervals.

The length of a **base64Binary** value is the number of octets it contains.
This may be calculated from the lexical form by removing whitespace and padding characters
and performing the calculation shown in the pseudo-code below:

`lex2    := killwhitespace(lexform)    -- remove whitespace characters  
lex3    := strip_equals(lex2)         -- strip padding characters at end  
length  := floor (length(lex3) * 3 / 4)         -- calculate length`

Note on encoding: [[RFC 2045]](044-h-references.md#RFC2045) explicitly references US-ASCII encoding. However,
decoding of **base64Binary** data in an XML entity is to be performed on the
Unicode characters obtained after character encoding processing as specified by
[[XML 1.0 (Second Edition)]](044-h-references.md#XML)

<a id="base64Binary-facets"></a>

##### 3.2.16.1 Constraining facets

**base64Binary** has the
following [·constraining facets·](004-2-3-lexical-space.md#dt-constraining-facet):

- [length](024-4-2-5-numeric.md#rf-length)
- [minLength](025-4-3-1-4-constraints-on-length-schema-components.md#rf-minLength)
- [maxLength](026-4-3-3-maxlength.md#rf-maxLength)
- [pattern](026-4-3-3-maxlength.md#rf-pattern)
- [enumeration](027-4-3-4-3-constraints-on-xml-representation-of-pattern.md#rf-enumeration)
- [whiteSpace](027-4-3-4-3-constraints-on-xml-representation-of-pattern.md#rf-whiteSpace)

<a id="anyURI"></a>

#### 3.2.17 anyURI

<a id="dt-anyURI"></a>

[Definition:]
**anyURI** represents a Uniform Resource Identifier Reference
(URI). An **anyURI** value can be absolute or relative, and may
have an optional fragment identifier (i.e., it may be a URI Reference). This
type should be used to specify the intention that the value fulfills
the role of a URI as defined by [[RFC 2396]](044-h-references.md#RFC2396), as amended by
[[RFC 2732]](044-h-references.md#RFC2732).

The mapping from **anyURI** values to URIs is as
defined by the URI reference escaping procedure
defined in
Section 5.4 [Locator Attribute](https://www.w3.org/TR/2001/REC-xlink-20010627/#link-locators)
of [[XML Linking Language]](044-h-references.md#XLink) (see also Section 8
[Character Encoding in URI References](https://www.w3.org/TR/2001/WD-charmod-20010126/#sec-URIs)
of [[Character Model]](044-h-references.md#CharMod)). This means
that a wide range of internationalized resource identifiers can be specified
when an **anyURI** is called for, and still be understood as
URIs per [[RFC 2396]](044-h-references.md#RFC2396), as amended by [[RFC 2732]](044-h-references.md#RFC2732),
where appropriate to identify resources.

**Note:**
Section 5.4 [Locator Attribute](https://www.w3.org/TR/2001/REC-xlink-20010627/#link-locators)
of [[XML Linking Language]](044-h-references.md#XLink) requires that relative URI references be absolutized
as defined in [[XML Base]](044-h-references.md#XBase) before use. This is an XLink-specific
requirement and is not appropriate for XML Schema, since neither the
[·lexical space·](004-2-3-lexical-space.md#dt-lexical-space) nor the [·value space·](003-1-4-terminology.md#dt-value-space)
of the [anyURI](012-3-2-15-hexbinary.md#anyURI) type are restricted to absolute URIs. Accordingly
absolutization must not be performed by schema processors as part of schema
validation.

**Note:**
Each URI scheme imposes specialized syntax rules for URIs in
that scheme, including restrictions on the syntax of allowed
fragment
identifiers. Because it is
impractical for processors to check that a value is a
context-appropriate URI reference, this specification follows the
lead of [[RFC 2396]](044-h-references.md#RFC2396) (as amended by [[RFC 2732]](044-h-references.md#RFC2732))
in this matter: such rules and restrictions are not part of type validity
and are not checked by [·minimally conforming·](033-5-conformance.md#dt-minimally-conforming) processors.
Thus in practice the above definition imposes only very modest obligations
on [·minimally conforming·](033-5-conformance.md#dt-minimally-conforming) processors.

<a id="anyURI-lexical-representation"></a>

##### 3.2.17.1 Lexical representation

The [·lexical space·](004-2-3-lexical-space.md#dt-lexical-space) of **anyURI** is
finite-length character sequences which, when the algorithm defined in
Section 5.4 of [[XML Linking Language]](044-h-references.md#XLink) is applied to them, result in strings
which are legal URIs according to [[RFC 2396]](044-h-references.md#RFC2396), as amended by
[[RFC 2732]](044-h-references.md#RFC2732).

**Note:**
Spaces are, in principle, allowed in the [·lexical space·](004-2-3-lexical-space.md#dt-lexical-space)
of **anyURI**, however, their use is highly discouraged
(unless they are encoded by %20).

<a id="anyURI-facets"></a>

##### 3.2.17.2 Constraining facets

**anyURI** has the
following [·constraining facets·](004-2-3-lexical-space.md#dt-constraining-facet):

- [length](024-4-2-5-numeric.md#rf-length)
- [minLength](025-4-3-1-4-constraints-on-length-schema-components.md#rf-minLength)
- [maxLength](026-4-3-3-maxlength.md#rf-maxLength)
- [pattern](026-4-3-3-maxlength.md#rf-pattern)
- [enumeration](027-4-3-4-3-constraints-on-xml-representation-of-pattern.md#rf-enumeration)
- [whiteSpace](027-4-3-4-3-constraints-on-xml-representation-of-pattern.md#rf-whiteSpace)

<a id="QName"></a>

#### 3.2.18 QName

<a id="dt-QName"></a>

[Definition:]
**QName** represents
[XML qualified names](https://www.w3.org/TR/1999/REC-xml-names-19990114/#dt-qname).
The [·value space·](003-1-4-terminology.md#dt-value-space) of **QName** is the set of
tuples {[namespace name](https://www.w3.org/TR/1999/REC-xml-names-19990114/#dt-NSName),
[local part](https://www.w3.org/TR/1999/REC-xml-names-19990114/#dt-localname)},
where [namespace name](https://www.w3.org/TR/1999/REC-xml-names-19990114/#dt-NSName)
is an [anyURI](012-3-2-15-hexbinary.md#anyURI)
and [local part](https://www.w3.org/TR/1999/REC-xml-names-19990114/#dt-localname) is
an [NCName](014-3-3-4-1-constraining-facets.md#NCName).
The [·lexical space·](004-2-3-lexical-space.md#dt-lexical-space) of **QName** is the set
of strings that [·match·](003-1-4-terminology.md#dt-match) the [QName](https://www.w3.org/TR/1999/REC-xml-names-19990114/#NT-QName) production of [[Namespaces in XML]](044-h-references.md#XMLNS).

**Note:**
The mapping between literals in the [·lexical space·](004-2-3-lexical-space.md#dt-lexical-space) and
values in the [·value space·](003-1-4-terminology.md#dt-value-space) of **QName** requires
a namespace declaration to be in scope for the context in which **QName**
is used.

<a id="QName-facets"></a>

##### 3.2.18.1 Constraining facets

**QName** has the
following [·constraining facets·](004-2-3-lexical-space.md#dt-constraining-facet):

