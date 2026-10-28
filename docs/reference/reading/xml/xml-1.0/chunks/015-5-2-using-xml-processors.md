# 5.2 Using XML Processors

Source: [Extensible Markup Language (XML) 1.0](https://www.w3.org/TR/2008/REC-xml-20081126/) · 5th edition, 2008-11-26.

[Document index](../README.md) · [Topic guide](../../../../README.md) · [Publisher](https://www.w3.org/TR/2008/REC-xml-20081126/).

HTML normalized to Markdown. Complex tables and mathematical markup retain HTML; illustrations are stored locally.

[Previous](014-4-5-construction-of-entity-replacement-text.md) · [Next](016-a-2-other-references.md)

<a id="safe-behavior"></a>

### 5.2 Using XML Processors

The behavior of a validating XML processor is highly predictable; it must
read every piece of a document and report all well-formedness and validity
violations. Less is required of a non-validating processor; it need not read
any part of the document other than the document entity. This has two effects
that may be important to users of XML processors:

- Certain well-formedness errors, specifically those that require reading
  external entities, may fail to be detected by a non-validating processor. Examples
  include the constraints entitled [Entity Declared](011-4-1-character-and-entity-references.md#wf-entdeclared), [Parsed Entity](011-4-1-character-and-entity-references.md#textent), and [No
  Recursion](011-4-1-character-and-entity-references.md#norecursion), as well as some of the cases described as [forbidden](013-4-4-xml-processor-treatment-of-entities-and-references.md#forbidden) in [**4.4 XML Processor Treatment of Entities and References**](013-4-4-xml-processor-treatment-of-entities-and-references.md#entproc).
- The information passed from the processor to the application may
  vary, depending on whether the processor reads parameter and external entities.
  For example, a non-validating processor may fail to [normalize](009-3-3-attribute-list-declarations.md#AVNormalize)
  attribute values, [include](013-4-4-xml-processor-treatment-of-entities-and-references.md#included) the replacement
  text of internal entities, or supply [default
  attribute values](009-3-3-attribute-list-declarations.md#sec-attr-defaults), where doing so depends on having read declarations
  in external or parameter entities, or in the internal subset after an unread
  parameter entity reference.

For maximum reliability in interoperating between different XML processors,
applications which use non-validating processors *SHOULD NOT* rely on any behaviors
not required of such processors. Applications which require DTD facilities not related to validation (such
as the declaration of default attributes and internal entities that are or may be specified in
external entities) *SHOULD* use validating XML processors.

<a id="sec-notation"></a>

## 6 Notation

The formal grammar of XML is given in this specification using a simple
Extended Backus-Naur Form (EBNF) notation. Each rule in the grammar defines
one symbol, in the form



```
symbol ::= expression
```




Symbols are written with an initial capital letter if they are the
start symbol of a regular language, otherwise with an initial lowercase letter.
Literal strings are quoted.

Within the expression on the right-hand side of a rule, the following expressions
are used to match strings of one or more characters:

`#xN`
:   where `N` is a hexadecimal integer, the expression matches the character
    whose number
    (code point) in ISO/IEC 10646 is `N`. The number of leading zeros in the `#xN`
    form is insignificant.

`[a-zA-Z]`, `[#xN-#xN]`
:   matches any [Char](003-2-documents.md#NT-Char) with a value in the range(s) indicated (inclusive).

`[abc]`, `[#xN#xN#xN]`
:   matches any [Char](003-2-documents.md#NT-Char) with a value among the characters
    enumerated. Enumerations and ranges can be mixed in one set of brackets.

`[^a-z]`, `[^#xN-#xN]`
:   matches any [Char](003-2-documents.md#NT-Char) with a value *outside* the range
    indicated.

`[^abc]`, `[^#xN#xN#xN]`
:   matches any [Char](003-2-documents.md#NT-Char) with a value not among the characters given. Enumerations
    and ranges of forbidden values can be mixed in one set of brackets.

`"string"`
:   matches a literal string [matching](002-appendices.md#dt-match "match") that
    given inside the double quotes.

`'string'`
:   matches a literal string [matching](002-appendices.md#dt-match "match") that
    given inside the single quotes.

These symbols may be combined to match more complex patterns as follows,
where `A` and `B` represent simple expressions:

(`expression`)
:   `expression` is treated as a unit and may be combined as described
    in this list.

`A?`
:   matches `A` or nothing; optional `A`.

`A B`
:   matches `A` followed by `B`. This
    operator has higher precedence than alternation; thus `A B | C D`
    is identical to `(A B) | (C D)`.

`A | B`
:   matches `A` or `B`.

`A - B`
:   matches any string that matches `A` but does not match `B`.

`A+`
:   matches one or more occurrences of `A`. Concatenation
    has higher precedence than alternation; thus `A+ | B+` is identical
    to `(A+) | (B+)`.

`A*`
:   matches zero or more occurrences of `A`. Concatenation
    has higher precedence than alternation; thus `A* | B*` is identical
    to `(A*) | (B*)`.

Other notations used in the productions are:

`/* ... */`
:   comment.

`[ wfc: ... ]`
:   well-formedness constraint; this identifies by name a constraint on [well-formed](003-2-documents.md#dt-wellformed "Well-Formed") documents associated with a production.

`[ vc: ... ]`
:   validity constraint; this identifies by name a constraint on [valid](005-2-8-prolog-and-document-type-declaration.md#dt-valid "Validity")
    documents associated with a production.

<a id="sec-bibliography"></a>

## A References

<a id="sec-existing-stds"></a>

### A.1 Normative References

<a id="IANA"></a>

IANA-CHARSETS
:   (Internet
    Assigned Numbers Authority) [Official Names for Character Sets](http://www.iana.org/assignments/character-sets),
    ed. Keld Simonsen et al. (See http://www.iana.org/assignments/character-sets.)

<a id="rfc2119"></a>

IETF RFC 2119
:   IETF
    (Internet Engineering Task Force). [RFC 2119: Key words for use in RFCs to Indicate Requirement Levels](http://www.ietf.org/rfc/rfc2119.txt).
    Scott Bradner, 1997. (See http://www.ietf.org/rfc/rfc2119.txt.)

<a id="RFC1766"></a>

IETF BCP 47
:   IETF
    (Internet Engineering Task Force). BCP 47, consisting of [RFC 4646: Tags for Identifying Languages](http://tools.ietf.org/html/rfc4646), and [RFC 4647: Matching of Language Tags](http://tools.ietf.org/html/rfc4647),
    A. Phillips, M. Davis. 2006.

<a id="rfc3986"></a>

IETF RFC 3986
:   IETF (Internet Engineering Task Force). [RFC 3986: Uniform Resource Identifier (URI): Generic Syntax](http://www.ietf.org/rfc/rfc3986.txt). T. Berners-Lee, R. Fielding, L. Masinter. 2005. (See http://www.ietf.org/rfc/rfc3986.txt.)

<a id="ISO10646"></a>

ISO/IEC 10646
:   ISO (International
    Organization for Standardization). ISO/IEC 10646-1:2000. Information
    technology — Universal Multiple-Octet Coded Character Set (UCS) —
    Part 1: Architecture and Basic Multilingual Plane and ISO/IEC 10646-2:2001.
    Information technology — Universal Multiple-Octet Coded Character Set (UCS) — Part 2:
    Supplementary Planes, as, from time to time, amended, replaced by a new edition or
    expanded by the addition of new parts. [Geneva]: International Organization for Standardization.
    (See [http://www.iso.org/iso/home.htm](http://www.iso.org/iso/home.htm) for the latest version.)

<a id="ISO10646-2000"></a>

ISO/IEC 10646:2000
:   ISO (International
    Organization for Standardization). ISO/IEC 10646-1:2000. Information
    technology — Universal Multiple-Octet Coded Character Set (UCS) —
    Part 1: Architecture and Basic Multilingual Plane. [Geneva]: International
    Organization for Standardization, 2000.

<a id="Unicode"></a>

Unicode
:   The Unicode Consortium. *The Unicode
    Standard, Version 5.0.0,* defined by: The Unicode Standard, Version 5.0 (Boston, MA,
    Addison-Wesley, 2007. ISBN 0-321-48091-0).

<a id="unipart15"></a>

UnicodeNormal
:   The Unicode
    Consortium. [Unicode normalization forms](http://unicode.org/reports/tr15/). Mark Davis and
    Martin Durst. 2008. (See http://unicode.org/reports/tr15/.)

