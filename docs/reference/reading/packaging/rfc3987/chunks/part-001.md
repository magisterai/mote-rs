# Internationalized Resource Identifiers (IRIs) — part 1

Source: [Internationalized Resource Identifiers (IRIs)](https://www.rfc-editor.org/rfc/rfc3987.txt) · RFC 3987.

[Document index](../README.md) · [Topic guide](../../../../README.md) · [Publisher](https://www.rfc-editor.org/rfc/rfc3987.txt).

Verbatim text slices in fixed-width blocks preserve grammar, examples, and diagrams.

[Next](part-002.md)

<a id="source-page-1-part-1"></a>

## Source page 1

```text






Network Working Group                                          M. Duerst
Request for Comments: 3987                                           W3C
Category: Standards Track                                    M. Suignard
                                                   Microsoft Corporation
                                                            January 2005


             Internationalized Resource Identifiers (IRIs)

Status of This Memo

   This document specifies an Internet standards track protocol for the
   Internet community, and requests discussion and suggestions for
   improvements.  Please refer to the current edition of the "Internet
   Official Protocol Standards" (STD 1) for the standardization state
   and status of this protocol.  Distribution of this memo is unlimited.

Copyright Notice

   Copyright (C) The Internet Society (2005).

Abstract

   This document defines a new protocol element, the Internationalized
   Resource Identifier (IRI), as a complement to the Uniform Resource
   Identifier (URI).  An IRI is a sequence of characters from the
   Universal Character Set (Unicode/ISO 10646).  A mapping from IRIs to
   URIs is defined, which means that IRIs can be used instead of URIs,
   where appropriate, to identify resources.

   The approach of defining a new protocol element was chosen instead of
   extending or changing the definition of URIs.  This was done in order
   to allow a clear distinction and to avoid incompatibilities with
   existing software.  Guidelines are provided for the use and
   deployment of IRIs in various protocols, formats, and software
   components that currently deal with URIs.

Table of Contents

   1.  Introduction . . . . . . . . . . . . . . . . . . . . . . . . .  3
       1.1.  Overview and Motivation  . . . . . . . . . . . . . . . .  3
       1.2.  Applicability  . . . . . . . . . . . . . . . . . . . . .  3
       1.3.  Definitions  . . . . . . . . . . . . . . . . . . . . . .  4
       1.4.  Notation . . . . . . . . . . . . . . . . . . . . . . . .  5
   2.  IRI Syntax . . . . . . . . . . . . . . . . . . . . . . . . . .  6
       2.1.  Summary of IRI Syntax  . . . . . . . . . . . . . . . . .  6
       2.2.  ABNF for IRI References and IRIs . . . . . . . . . . . .  7




Duerst & Suignard           Standards Track                     [Page 1]
```

<a id="source-page-2-part-1"></a>

## Source page 2

```text

RFC 3987         Internationalized Resource Identifiers     January 2005


   3.  Relationship between IRIs and URIs . . . . . . . . . . . . . . 10
       3.1.  Mapping of IRIs to URIs  . . . . . . . . . . . . . . . . 10
       3.2.  Converting URIs to IRIs  . . . . . . . . . . . . . . . . 14
             3.2.1.  Examples . . . . . . . . . . . . . . . . . . . . 15
   4.  Bidirectional IRIs for Right-to-Left Languages.  . . . . . . . 16
       4.1.  Logical Storage and Visual Presentation  . . . . . . . . 17
       4.2.  Bidi IRI Structure . . . . . . . . . . . . . . . . . . . 18
       4.3.  Input of Bidi IRIs . . . . . . . . . . . . . . . . . . . 19
       4.4.  Examples . . . . . . . . . . . . . . . . . . . . . . . . 19
   5.  Normalization and Comparison . . . . . . . . . . . . . . . . . 21
       5.1.  Equivalence  . . . . . . . . . . . . . . . . . . . . . . 22
       5.2.  Preparation for Comparison . . . . . . . . . . . . . . . 22
       5.3.  Comparison Ladder  . . . . . . . . . . . . . . . . . . . 23
             5.3.1.  Simple String Comparison . . . . . . . . . . . . 23
             5.3.2.  Syntax-Based Normalization . . . . . . . . . . . 24
             5.3.3.  Scheme-Based Normalization . . . . . . . . . . . 27
             5.3.4.  Protocol-Based Normalization . . . . . . . . . . 28
   6.  Use of IRIs  . . . . . . . . . . . . . . . . . . . . . . . . . 29
       6.1.  Limitations on UCS Characters Allowed in IRIs  . . . . . 29
       6.2.  Software Interfaces and Protocols  . . . . . . . . . . . 29
       6.3.  Format of URIs and IRIs in Documents and Protocols . . . 30
       6.4.  Use of UTF-8 for Encoding Original Characters .. . . . . 30
       6.5.  Relative IRI References  . . . . . . . . . . . . . . . . 32
   7.  URI/IRI Processing Guidelines (informative)  . . . . . . . . . 32
       7.1.  URI/IRI Software Interfaces  . . . . . . . . . . . . . . 32
       7.2.  URI/IRI Entry  . . . . . . . . . . . . . . . . . . . . . 33
       7.3.  URI/IRI Transfer between Applications  . . . . . . . . . 33
       7.4.  URI/IRI Generation . . . . . . . . . . . . . . . . . . . 34
       7.5.  URI/IRI Selection  . . . . . . . . . . . . . . . . . . . 34
       7.6.  Display of URIs/IRIs . . . . . . . . . . . . . . . . . . 35
       7.7.  Interpretation of URIs and IRIs  . . . . . . . . . . . . 36
       7.8.  Upgrading Strategy . . . . . . . . . . . . . . . . . . . 36
   8.  Security Considerations  . . . . . . . . . . . . . . . . . . . 37
   9.  Acknowledgements . . . . . . . . . . . . . . . . . . . . . . . 39
   10. References . . . . . . . . . . . . . . . . . . . . . . . . . . 40
       10.1. Normative References . . . . . . . . . . . . . . . . . . 40
       10.2. Informative References . . . . . . . . . . . . . . . . . 41
   A.  Design Alternatives  . . . . . . . . . . . . . . . . . . . . . 44
       A.1.  New Scheme(s)  . . . . . . . . . . . . . . . . . . . . . 44
       A.2.  Character Encodings Other Than UTF-8 . . . . . . . . . . 44
       A.3.  New Encoding Convention  . . . . . . . . . . . . . . . . 44
       A.4.  Indicating Character Encodings in the URI/IRI  . . . . . 45
   Authors' Addresses . . . . . . . . . . . . . . . . . . . . . . . . 45
   Full Copyright Statement . . . . . . . . . . . . . . . . . . . . . 46







Duerst & Suignard           Standards Track                     [Page 2]
```

<a id="source-page-3-part-1"></a>

## Source page 3

```text

RFC 3987         Internationalized Resource Identifiers     January 2005


1.  Introduction

1.1.  Overview and Motivation

   A Uniform Resource Identifier (URI) is defined in [RFC3986] as a
   sequence of characters chosen from a limited subset of the repertoire
   of US-ASCII [ASCII] characters.

   The characters in URIs are frequently used for representing words of
   natural languages.  This usage has many advantages: Such URIs are
   easier to memorize, easier to interpret, easier to transcribe, easier
   to create, and easier to guess.  For most languages other than
   English, however, the natural script uses characters other than A -
   Z. For many people, handling Latin characters is as difficult as
   handling the characters of other scripts is for those who use only
   the Latin alphabet.  Many languages with non-Latin scripts are
   transcribed with Latin letters.  These transcriptions are now often
   used in URIs, but they introduce additional ambiguities.

   The infrastructure for the appropriate handling of characters from
   local scripts is now widely deployed in local versions of operating
   system and application software.  Software that can handle a wide
   variety of scripts and languages at the same time is increasingly
   common.  Also, increasing numbers of protocols and formats can carry
   a wide range of characters.

   This document defines a new protocol element called Internationalized
   Resource Identifier (IRI) by extending the syntax of URIs to a much
   wider repertoire of characters.  It also defines "internationalized"
   versions corresponding to other constructs from [RFC3986], such as
   URI references.  The syntax of IRIs is defined in section 2, and the
   relationship between IRIs and URIs in section 3.

   Using characters outside of A - Z in IRIs brings some difficulties.
   Section 4 discusses the special case of bidirectional IRIs, section 5
   various forms of equivalence between IRIs, and section 6 the use of
   IRIs in different situations.  Section 7 gives additional informative
   guidelines, and section 8 security considerations.

1.2.  Applicability

   IRIs are designed to be compatible with recommendations for new URI
   schemes [RFC2718].  The compatibility is provided by specifying a
   well-defined and deterministic mapping from the IRI character
   sequence to the functionally equivalent URI character sequence.
   Practical use of IRIs (or IRI references) in place of URIs (or URI
   references) depends on the following conditions being met:




Duerst & Suignard           Standards Track                     [Page 3]
```

<a id="source-page-4-part-1"></a>

## Source page 4

```text

RFC 3987         Internationalized Resource Identifiers     January 2005


   a.  A protocol or format element should be explicitly designated to
       be able to carry IRIs.  The intent is not to introduce IRIs into
       contexts that are not defined to accept them.  For example, XML
       schema [XMLSchema] has an explicit type "anyURI" that includes
       IRIs and IRI references. Therefore, IRIs and IRI references can
       be in attributes and elements of type "anyURI".  On the other
       hand, in the HTTP protocol [RFC2616], the Request URI is defined
       as a URI, which means that direct use of IRIs is not allowed in
       HTTP requests.

   b.  The protocol or format carrying the IRIs should have a mechanism
       to represent the wide range of characters used in IRIs, either
       natively or by some protocol- or format-specific escaping
       mechanism (for example, numeric character references in [XML1]).

   c.  The URI corresponding to the IRI in question has to encode
       original characters into octets using UTF-8.  For new URI
       schemes, this is recommended in [RFC2718].  It can apply to a
       whole scheme (e.g., IMAP URLs [RFC2192] and POP URLs [RFC2384],
       or the URN syntax [RFC2141]).  It can apply to a specific part of
       a URI, such as the fragment identifier (e.g., [XPointer]).  It
       can apply to a specific URI or part(s) thereof.  For details,
       please see section 6.4.

1.3.  Definitions

   The following definitions are used in this document; they follow the
   terms in [RFC2130], [RFC2277], and [ISO10646].

   character: A member of a set of elements used for the organization,
      control, or representation of data.  For example, "LATIN CAPITAL
      LETTER A" names a character.

   octet: An ordered sequence of eight bits considered as a unit.

   character repertoire: A set of characters (in the mathematical
      sense).

   sequence of characters: A sequence of characters (one after another).

   sequence of octets: A sequence of octets (one after another).

   character encoding: A method of representing a sequence of characters
      as a sequence of octets (maybe with variants).  Also, a method of
      (unambiguously) converting a sequence of octets into a sequence of
      characters.





Duerst & Suignard           Standards Track                     [Page 4]
```

<a id="source-page-5-part-1"></a>

## Source page 5

```text

RFC 3987         Internationalized Resource Identifiers     January 2005


   charset: The name of a parameter or attribute used to identify a
      character encoding.

   UCS: Universal Character Set. The coded character set defined by
      ISO/IEC 10646 [ISO10646] and the Unicode Standard [UNIV4].

   IRI reference: Denotes the common usage of an Internationalized
      Resource Identifier.  An IRI reference may be absolute or
      relative.  However, the "IRI" that results from such a reference
      only includes absolute IRIs; any relative IRI references are
      resolved to their absolute form.  Note that in [RFC2396] URIs did
      not include fragment identifiers, but in [RFC3986] fragment
      identifiers are part of URIs.

   running text: Human text (paragraphs, sentences, phrases) with syntax
      according to orthographic conventions of a natural language, as
      opposed to syntax defined for ease of processing by machines
      (e.g., markup, programming languages).

   protocol element: Any portion of a message that affects processing of
      that message by the protocol in question.

   presentation element: A presentation form corresponding to a protocol
      element; for example, using a wider range of characters.

   create (a URI or IRI): With respect to URIs and IRIs, the term is
      used for the initial creation.  This may be the initial creation
      of a resource with a certain identifier, or the initial exposition
      of a resource under a particular identifier.

   generate (a URI or IRI): With respect to URIs and IRIs, the term is
      used when the IRI is generated by derivation from other
      information.

1.4.  Notation

   RFCs and Internet Drafts currently do not allow any characters
   outside the US-ASCII repertoire.  Therefore, this document uses
   various special notations to denote such characters in examples.

   In text, characters outside US-ASCII are sometimes referenced by
   using a prefix of 'U+', followed by four to six hexadecimal digits.

   To represent characters outside US-ASCII in examples, this document
   uses two notations: 'XML Notation' and 'Bidi Notation'.






Duerst & Suignard           Standards Track                     [Page 5]
```

