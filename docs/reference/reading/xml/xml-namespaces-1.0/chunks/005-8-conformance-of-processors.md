# 8 Conformance of Processors

Source: [Namespaces in XML 1.0](https://www.w3.org/TR/2009/REC-xml-names-20091208/) · 3rd edition, 2009-12-08.

[Document index](../README.md) · [Topic guide](../../../../README.md) · [Publisher](https://www.w3.org/TR/2009/REC-xml-names-20091208/).

HTML normalized to Markdown. Complex tables and mathematical markup retain HTML; illustrations are stored locally.

[Previous](004-6-applying-namespaces-to-elements-and-attributes.md)

<a id="ProcessorConformance"></a>

## 8 Conformance of Processors

To conform to this specification, a processor
*MUST*
report
violations of namespace well-formedness, with the exception that it
is not
*REQUIRED*
to check that namespace names are
URI references [[RFC3986]](005-8-conformance-of-processors.md#URIRef).

<a id="dt-nvp"></a>

[Definition:
A validating XML processor that conforms to this specification
is **namespace-validating** if in addition
it reports violations of namespace validity.
]

<a id="refs"></a>

## A Normative References

<a id="keywords"></a>

Keywords
:   [RFC 2119: Key words for use in RFCs to Indicate Requirement Levels](http://www.rfc-editor.org/rfc/rfc2119.txt),
    S. Bradner, ed.
    IETF (Internet Engineering Task Force),
    March 1997.
    Available at
    http://www.rfc-editor.org/rfc/rfc2119.txt

<a id="URNs"></a>

RFC2141
:   [RFC 2141: URN Syntax](http://www.rfc-editor.org/rfc/rfc2141.txt),
    R. Moats, ed.
    IETF (Internet Engineering Task Force),
    May 1997.
    Available at
    http://www.rfc-editor.org/rfc/rfc2141.txt.

<a id="URIRef"></a>

RFC3986
:   [RFC 3986: Uniform Resource Identifier (URI): Generic Syntax](http://www.rfc-editor.org/rfc/rfc3986.txt),
    T. Berners-Lee, R. Fielding, and L. Masinter, eds.
    IETF (Internet Engineering Task Force),
    January 2005.
    Available at
    http://www.rfc-editor.org/rfc/rfc3986.txt

<a id="UTF8"></a>

RFC3629
:   [RFC 3629: UTF-8, a transformation format of ISO 10646](http://www.rfc-editor.org/rfc/rfc3629.txt),
    F. Yergeau, ed.
    IETF (Internet Engineering Task Force),
    November 2003.
    Available at http://www.rfc-editor.org/rfc/rfc3629.txt

<a id="XML"></a>

XML
:   [Extensible Markup Language
    (XML) 1.0](https://www.w3.org/TR/REC-xml/), Tim Bray, Jean
    Paoli, C. M. Sperberg-McQueen, Eve Maler, and François Yergeau eds.
    W3C (World Wide Web Consortium).
    Available at
    http://www.w3.org/TR/REC-xml/.

<a id="nrefs"></a>

## B Other references (Non-Normative)

<a id="errata10"></a>

1.0 Errata
:   [Namespaces in XML Errata](https://www.w3.org/XML/xml-names-19990114-errata).
    W3C (World Wide Web Consortium).
    Available at
    http://www.w3.org/XML/xml-names-19990114-errata.

<a id="errata10.2"></a>

1.0 2e Errata
:   [Namespaces in XML
    (Second Edition) Errata](https://www.w3.org/XML/2006/xml-names-errata).
    W3C (World Wide Web Consortium).
    Available at
    http://www.w3.org/XML/2006/xml-names-errata.

<a id="reluri"></a>

Relative URI deprecation
:   [Results of W3C XML Plenary
    Ballot on relative URI References
    In namespace declarations
    3-17 July 2000](https://www.w3.org/2000/09/xppa),
    Dave Hollander and
    C. M. Sperberg-McQueen,
    6 September 2000.
    Available at
    http://www.w3.org/2000/09/xppa.

<a id="Philosophy"></a>

## C The Internal Structure of XML Namespaces (Non-Normative)

This appendix has been deleted.

<a id="changes"></a>

## D Changes since version 1.0 (Non-Normative)

This version incorporates the errata as of 20 July 2009
[[1.0 Errata]](005-8-conformance-of-processors.md#errata10) [[1.0 2e Errata]](005-8-conformance-of-processors.md#errata10.2).

There are several editorial changes, including a number
of terminology changes and additions intended to produce greater
consistency. The non-normative appendix "The Internal Structure
of XML Namespaces" has been removed. The BNF
has been adjusted to interconnect properly with all editions of XML 1.0,
including the fifth edition.

<a id="sec-xml-and-sgml"></a>

## E Acknowledgements (Non-Normative)

This work reflects input from a very large number of people,
including especially the participants in the World Wide
Web Consortium XML Working Group and Special Interest Group
and the participants in the W3C Metadata Activity.
The contributions of Charles Frankston of Microsoft
were particularly valuable.

<a id="orphans"></a>

## F Orphaned Productions (Non-Normative)

The following two productions are modified versions of ones which were present in the first two editions of this specification. They are no longer used, but are retained here to satisfy cross-references to undated versions of this specification.

Because the `Letter` production of XML 1.0, originally used in the definition of `NCNameStartChar`, is no longer the correct basis for defining names since XML 1.0 Fifth Edition, the `NCNameStartChar` production has been modified to give the correct results against any edition of XML, by defining `NCNameStartChar` in terms of [NCName](002-1-motivation-and-summary.md#NT-NCName).

<a id="NT-NCNameChar"></a>

<a id="NT-NCNameStartChar"></a>

|  |  |  |  |
| --- | --- | --- | --- |
| [5] | `NCNameChar` | ::= | `NameChar - ':' /* An XML NameChar, minus the ":" */` |
|  |  |  |  |
| --- | --- | --- | --- |
| [6] | `NCNameStartChar` | ::= | `NCName - ( Char Char Char* ) /* The first letter of an NCName */` |

**Note:**

Production
[NC-NCNameStartChar](005-8-conformance-of-processors.md#NT-NCNameStartChar)
takes advantage of the fact that a
single-character NCName is necessarily an NCNameStartChar, and
works by subtracting from the set of NCNames of all lengths the
set of all strings of two or more characters, leaving only the
NCNames which are one character long.
