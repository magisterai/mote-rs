# Appendices

Source: [Extensible Markup Language (XML) 1.0](https://www.w3.org/TR/2008/REC-xml-20081126/) · 5th edition, 2008-11-26.

[Document index](../README.md) · [Topic guide](../../../../README.md) · [Publisher](https://www.w3.org/TR/2008/REC-xml-20081126/).

HTML normalized to Markdown. Complex tables and mathematical markup retain HTML; illustrations are stored locally.

[Previous](001-extensible-markup-language-xml-1-0-fifth-edition.md) · [Next](003-2-documents.md)

<a id="appendices"></a>

### Appendices

A [References](015-5-2-using-xml-processors.md#sec-bibliography)  
    A.1 [Normative References](015-5-2-using-xml-processors.md#sec-existing-stds)  
    A.2 [Other References](016-a-2-other-references.md#null)  
B [Character Classes](016-a-2-other-references.md#CharClasses)  
C [XML and SGML](017-c-xml-and-sgml-non-normative.md#sec-xml-and-sgml) (Non-Normative)  
D [Expansion of Entity and Character References](017-c-xml-and-sgml-non-normative.md#sec-entexpand) (Non-Normative)  
E [Deterministic Content Models](017-c-xml-and-sgml-non-normative.md#determinism) (Non-Normative)  
F [Autodetection of Character Encodings](017-c-xml-and-sgml-non-normative.md#sec-guessing) (Non-Normative)  
    F.1 [Detection Without External Encoding Information](018-f-1-detection-without-external-encoding-information.md#sec-guessing-no-ext-info)  
    F.2 [Priorities in the Presence of External Encoding Information](018-f-1-detection-without-external-encoding-information.md#sec-guessing-with-ext-info)  
G [W3C XML Working Group](018-f-1-detection-without-external-encoding-information.md#sec-xml-wg) (Non-Normative)  
H [W3C XML Core Working Group](018-f-1-detection-without-external-encoding-information.md#sec-core-wg) (Non-Normative)  
I [Production Notes](019-i-production-notes-non-normative.md#prod-notes) (Non-Normative)  
J [Suggestions for XML Names](019-i-production-notes-non-normative.md#sec-suggested-names) (Non-Normative)

---

<a id="sec-intro"></a>

## 1 Introduction

Extensible Markup Language, abbreviated XML, describes a class of data
objects called [XML documents](003-2-documents.md#dt-xml-doc "XML Document") and partially
describes the behavior of computer programs which process them. XML is an
application profile or restricted form of SGML, the Standard Generalized Markup
Language [[ISO 8879]](016-a-2-other-references.md#ISO8879). By construction, XML documents are conforming
SGML documents.

XML documents are made up of storage units called [entities](010-3-4-conditional-sections.md#dt-entity "Entity"),
which contain either parsed or unparsed data. Parsed data is made up of [characters](003-2-documents.md#dt-character "Character"), some of which form [character
data](004-2-4-character-data-and-markup.md#dt-chardata "Character Data"), and some of which form [markup](004-2-4-character-data-and-markup.md#dt-markup "Markup").
Markup encodes a description of the document's storage layout and logical
structure. XML provides a mechanism to impose constraints on the storage layout
and logical structure.

<a id="dt-xml-proc"></a>

<a id="dt-app"></a>

[Definition: A software module called
an **XML processor** is used to read XML documents and provide access
to their content and structure.]
[Definition: It
is assumed that an XML processor is doing its work on behalf of another module,
called the **application**.] This specification describes
the required behavior of an XML processor in terms of how it must read XML
data and the information it must provide to the application.

<a id="sec-origin-goals"></a>

### 1.1 Origin and Goals

XML was developed by an XML Working Group (originally known as the SGML
Editorial Review Board) formed under the auspices of the World Wide Web Consortium
(W3C) in 1996. It was chaired by Jon Bosak of Sun Microsystems with the active
participation of an XML Special Interest Group (previously known as the SGML
Working Group) also organized by the W3C. The membership of the XML Working
Group is given in an appendix. Dan Connolly served as the Working Group's contact with
the W3C.

The design goals for XML are:

1. XML shall be straightforwardly usable over the Internet.
2. XML shall support a wide variety of applications.
3. XML shall be compatible with SGML.
4. It shall be easy to write programs which process XML documents.
5. The number of optional features in XML is to be kept to the absolute
   minimum, ideally zero.
6. XML documents should be human-legible and reasonably clear.
7. The XML design should be prepared quickly.
8. The design of XML shall be formal and concise.
9. XML documents shall be easy to create.
10. Terseness in XML markup is of minimal importance.

This specification, together with associated standards (Unicode [[Unicode]](015-5-2-using-xml-processors.md#Unicode)
and ISO/IEC 10646 [[ISO/IEC 10646]](015-5-2-using-xml-processors.md#ISO10646) for characters, Internet BCP 47
[[IETF BCP 47]](015-5-2-using-xml-processors.md#RFC1766) and the Language Subtag Registry [[IANA-LANGCODES]](016-a-2-other-references.md#IANA-LANGCODES) for language
identification tags), provides
all the information necessary to understand XML Version 1.0 and
construct computer programs to process it.

This version of the XML specification may be distributed freely, as long as
all text and legal notices remain intact.

<a id="sec-terminology"></a>

### 1.2 Terminology

The terminology used to describe XML documents is defined in the body of
this specification. The key words *MUST*, *MUST NOT*,
*REQUIRED*, *SHALL*, *SHALL NOT*,
*SHOULD*, *SHOULD NOT*, *RECOMMENDED*,
*MAY*, and *OPTIONAL*, when *EMPHASIZED*,
are to be interpreted as described in [[IETF RFC 2119]](015-5-2-using-xml-processors.md#rfc2119). In addition, the terms defined
in the following list are used in building
those definitions and in describing the actions of an XML processor:

error
:   <a id="dt-error"></a>

    [Definition: A violation of the rules of this specification;
    results are undefined. Unless otherwise specified, failure to observe a prescription of this specification indicated by one of the keywords *MUST*, *REQUIRED*, *MUST NOT*, *SHALL* and *SHALL NOT* is an error. Conforming software *MAY* detect and report an error
    and *MAY* recover from it.]

fatal error
:   <a id="dt-fatal"></a>

    [Definition: An error which a conforming [XML processor](002-appendices.md#dt-xml-proc "XML Processor")
    *MUST* detect and report to the application.
    After encountering a fatal error, the processor *MAY* continue processing the
    data to search for further errors and *MAY* report such errors to the application.
    In order to support correction of errors, the processor *MAY* make unprocessed
    data from the document (with intermingled character data and markup) available
    to the application. Once a fatal error is detected, however, the processor
    *MUST NOT* continue normal processing (i.e., it *MUST NOT* continue to pass character
    data and information about the document's logical structure to the application
    in the normal way).]

at user option
:   <a id="dt-atuseroption"></a>

    [Definition: Conforming software
    *MAY* or *MUST* (depending on the modal verb in the sentence) behave as described;
    if it does, it *MUST* provide users a means to enable or disable the behavior
    described.]

validity constraint
:   <a id="dt-vc"></a>

    [Definition: A rule which applies to
    all [valid](005-2-8-prolog-and-document-type-declaration.md#dt-valid "Validity") XML documents. Violations of validity
    constraints are errors; they *MUST*, at user option, be reported by [validating XML processors](014-4-5-construction-of-entity-replacement-text.md#dt-validating "Validating Processor").]

well-formedness constraint
:   <a id="dt-wfc"></a>

    [Definition: A rule which applies
    to all [well-formed](003-2-documents.md#dt-wellformed "Well-Formed") XML documents. Violations
    of well-formedness constraints are [fatal errors](002-appendices.md#dt-fatal "Fatal Error").]

match
:   <a id="dt-match"></a>

    [Definition: (Of strings or names:) Two strings
    or names being compared are identical. Characters with multiple possible
    representations in ISO/IEC 10646 (e.g. characters with both precomposed and
    base+diacritic forms) match only if they have the same representation in both
    strings. No
    case folding is performed. (Of strings and rules in the grammar:) A string
    matches a grammatical production if it belongs to the language generated by
    that production. (Of content and content models:) An element matches its declaration
    when it conforms in the fashion described in the constraint **[VC: [Element Valid](007-2-12-language-identification.md#elementvalid)]**.]

for compatibility
:   <a id="dt-compat"></a>

    [Definition: Marks
    a sentence describing a feature of XML included solely to ensure
    that XML remains compatible with SGML.]

for interoperability
:   <a id="dt-interop"></a>

    [Definition: Marks
    a sentence describing a non-binding recommendation included to increase
    the chances that XML documents can be processed by the existing installed
    base of SGML processors which predate the WebSGML Adaptations Annex to ISO 8879.]

