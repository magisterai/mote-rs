# Uniform Resource Identifier (URI): Generic Syntax — part 1

Source: [Uniform Resource Identifier (URI): Generic Syntax](https://www.rfc-editor.org/rfc/rfc3986.txt) · RFC 3986.

[Document index](../README.md) · [Topic guide](../../../../README.md) · [Publisher](https://www.rfc-editor.org/rfc/rfc3986.txt).

Verbatim text slices in fixed-width blocks preserve grammar, examples, and diagrams.

[Next](part-002.md)

<a id="source-page-1-part-1"></a>

## Source page 1

```text






Network Working Group                                     T. Berners-Lee
Request for Comments: 3986                                       W3C/MIT
STD: 66                                                      R. Fielding
Updates: 1738                                               Day Software
Obsoletes: 2732, 2396, 1808                                  L. Masinter
Category: Standards Track                                  Adobe Systems
                                                            January 2005


           Uniform Resource Identifier (URI): Generic Syntax

Status of This Memo

   This document specifies an Internet standards track protocol for the
   Internet community, and requests discussion and suggestions for
   improvements.  Please refer to the current edition of the "Internet
   Official Protocol Standards" (STD 1) for the standardization state
   and status of this protocol.  Distribution of this memo is unlimited.

Copyright Notice

   Copyright (C) The Internet Society (2005).

Abstract

   A Uniform Resource Identifier (URI) is a compact sequence of
   characters that identifies an abstract or physical resource.  This
   specification defines the generic URI syntax and a process for
   resolving URI references that might be in relative form, along with
   guidelines and security considerations for the use of URIs on the
   Internet.  The URI syntax defines a grammar that is a superset of all
   valid URIs, allowing an implementation to parse the common components
   of a URI reference without knowing the scheme-specific requirements
   of every possible identifier.  This specification does not define a
   generative grammar for URIs; that task is performed by the individual
   specifications of each URI scheme.















Berners-Lee, et al.         Standards Track                     [Page 1]
```

<a id="source-page-2-part-1"></a>

## Source page 2

```text

RFC 3986                   URI Generic Syntax               January 2005


Table of Contents

   1.  Introduction . . . . . . . . . . . . . . . . . . . . . . . . .  4
       1.1.  Overview of URIs . . . . . . . . . . . . . . . . . . . .  4
             1.1.1.  Generic Syntax . . . . . . . . . . . . . . . . .  6
             1.1.2.  Examples . . . . . . . . . . . . . . . . . . . .  7
             1.1.3.  URI, URL, and URN  . . . . . . . . . . . . . . .  7
       1.2.  Design Considerations  . . . . . . . . . . . . . . . . .  8
             1.2.1.  Transcription  . . . . . . . . . . . . . . . . .  8
             1.2.2.  Separating Identification from Interaction . . .  9
             1.2.3.  Hierarchical Identifiers . . . . . . . . . . . . 10
       1.3.  Syntax Notation  . . . . . . . . . . . . . . . . . . . . 11
   2.  Characters . . . . . . . . . . . . . . . . . . . . . . . . . . 11
       2.1.  Percent-Encoding . . . . . . . . . . . . . . . . . . . . 12
       2.2.  Reserved Characters  . . . . . . . . . . . . . . . . . . 12
       2.3.  Unreserved Characters  . . . . . . . . . . . . . . . . . 13
       2.4.  When to Encode or Decode . . . . . . . . . . . . . . . . 14
       2.5.  Identifying Data . . . . . . . . . . . . . . . . . . . . 14
   3.  Syntax Components  . . . . . . . . . . . . . . . . . . . . . . 16
       3.1.  Scheme . . . . . . . . . . . . . . . . . . . . . . . . . 17
       3.2.  Authority  . . . . . . . . . . . . . . . . . . . . . . . 17
             3.2.1.  User Information . . . . . . . . . . . . . . . . 18
             3.2.2.  Host . . . . . . . . . . . . . . . . . . . . . . 18
             3.2.3.  Port . . . . . . . . . . . . . . . . . . . . . . 22
       3.3.  Path . . . . . . . . . . . . . . . . . . . . . . . . . . 22
       3.4.  Query  . . . . . . . . . . . . . . . . . . . . . . . . . 23
       3.5.  Fragment . . . . . . . . . . . . . . . . . . . . . . . . 24
   4.  Usage  . . . . . . . . . . . . . . . . . . . . . . . . . . . . 25
       4.1.  URI Reference  . . . . . . . . . . . . . . . . . . . . . 25
       4.2.  Relative Reference . . . . . . . . . . . . . . . . . . . 26
       4.3.  Absolute URI . . . . . . . . . . . . . . . . . . . . . . 27
       4.4.  Same-Document Reference  . . . . . . . . . . . . . . . . 27
       4.5.  Suffix Reference . . . . . . . . . . . . . . . . . . . . 27
   5.  Reference Resolution . . . . . . . . . . . . . . . . . . . . . 28
       5.1.  Establishing a Base URI  . . . . . . . . . . . . . . . . 28
             5.1.1.  Base URI Embedded in Content . . . . . . . . . . 29
             5.1.2.  Base URI from the Encapsulating Entity . . . . . 29
             5.1.3.  Base URI from the Retrieval URI  . . . . . . . . 30
             5.1.4.  Default Base URI . . . . . . . . . . . . . . . . 30
       5.2.  Relative Resolution  . . . . . . . . . . . . . . . . . . 30
             5.2.1.  Pre-parse the Base URI . . . . . . . . . . . . . 31
             5.2.2.  Transform References . . . . . . . . . . . . . . 31
             5.2.3.  Merge Paths  . . . . . . . . . . . . . . . . . . 32
             5.2.4.  Remove Dot Segments  . . . . . . . . . . . . . . 33
       5.3.  Component Recomposition  . . . . . . . . . . . . . . . . 35
       5.4.  Reference Resolution Examples  . . . . . . . . . . . . . 35
             5.4.1.  Normal Examples  . . . . . . . . . . . . . . . . 36
             5.4.2.  Abnormal Examples  . . . . . . . . . . . . . . . 36



Berners-Lee, et al.         Standards Track                     [Page 2]
```

<a id="source-page-3-part-1"></a>

## Source page 3

```text

RFC 3986                   URI Generic Syntax               January 2005


   6.  Normalization and Comparison . . . . . . . . . . . . . . . . . 38
       6.1.  Equivalence  . . . . . . . . . . . . . . . . . . . . . . 38
       6.2.  Comparison Ladder  . . . . . . . . . . . . . . . . . . . 39
             6.2.1.  Simple String Comparison . . . . . . . . . . . . 39
             6.2.2.  Syntax-Based Normalization . . . . . . . . . . . 40
             6.2.3.  Scheme-Based Normalization . . . . . . . . . . . 41
             6.2.4.  Protocol-Based Normalization . . . . . . . . . . 42
   7.  Security Considerations  . . . . . . . . . . . . . . . . . . . 43
       7.1.  Reliability and Consistency  . . . . . . . . . . . . . . 43
       7.2.  Malicious Construction . . . . . . . . . . . . . . . . . 43
       7.3.  Back-End Transcoding . . . . . . . . . . . . . . . . . . 44
       7.4.  Rare IP Address Formats  . . . . . . . . . . . . . . . . 45
       7.5.  Sensitive Information  . . . . . . . . . . . . . . . . . 45
       7.6.  Semantic Attacks . . . . . . . . . . . . . . . . . . . . 45
   8.  IANA Considerations  . . . . . . . . . . . . . . . . . . . . . 46
   9.  Acknowledgements . . . . . . . . . . . . . . . . . . . . . . . 46
   10. References . . . . . . . . . . . . . . . . . . . . . . . . . . 46
       10.1. Normative References . . . . . . . . . . . . . . . . . . 46
       10.2. Informative References . . . . . . . . . . . . . . . . . 47
   A.  Collected ABNF for URI . . . . . . . . . . . . . . . . . . . . 49
   B.  Parsing a URI Reference with a Regular Expression  . . . . . . 50
   C.  Delimiting a URI in Context  . . . . . . . . . . . . . . . . . 51
   D.  Changes from RFC 2396  . . . . . . . . . . . . . . . . . . . . 53
       D.1.  Additions  . . . . . . . . . . . . . . . . . . . . . . . 53
       D.2.  Modifications  . . . . . . . . . . . . . . . . . . . . . 53
   Index  . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . 56
   Authors' Addresses . . . . . . . . . . . . . . . . . . . . . . . . 60
   Full Copyright Statement . . . . . . . . . . . . . . . . . . . . . 61























Berners-Lee, et al.         Standards Track                     [Page 3]
```

<a id="source-page-4-part-1"></a>

## Source page 4

```text

RFC 3986                   URI Generic Syntax               January 2005


1.  Introduction

   A Uniform Resource Identifier (URI) provides a simple and extensible
   means for identifying a resource.  This specification of URI syntax
   and semantics is derived from concepts introduced by the World Wide
   Web global information initiative, whose use of these identifiers
   dates from 1990 and is described in "Universal Resource Identifiers
   in WWW" [RFC1630].  The syntax is designed to meet the
   recommendations laid out in "Functional Recommendations for Internet
   Resource Locators" [RFC1736] and "Functional Requirements for Uniform
   Resource Names" [RFC1737].

   This document obsoletes [RFC2396], which merged "Uniform Resource
   Locators" [RFC1738] and "Relative Uniform Resource Locators"
   [RFC1808] in order to define a single, generic syntax for all URIs.
   It obsoletes [RFC2732], which introduced syntax for an IPv6 address.
   It excludes portions of RFC 1738 that defined the specific syntax of
   individual URI schemes; those portions will be updated as separate
   documents.  The process for registration of new URI schemes is
   defined separately by [BCP35].  Advice for designers of new URI
   schemes can be found in [RFC2718].  All significant changes from RFC
   2396 are noted in Appendix D.

   This specification uses the terms "character" and "coded character
   set" in accordance with the definitions provided in [BCP19], and
   "character encoding" in place of what [BCP19] refers to as a
   "charset".

1.1.  Overview of URIs

   URIs are characterized as follows:

   Uniform

      Uniformity provides several benefits.  It allows different types
      of resource identifiers to be used in the same context, even when
      the mechanisms used to access those resources may differ.  It
      allows uniform semantic interpretation of common syntactic
      conventions across different types of resource identifiers.  It
      allows introduction of new types of resource identifiers without
      interfering with the way that existing identifiers are used.  It
      allows the identifiers to be reused in many different contexts,
      thus permitting new applications or protocols to leverage a pre-
      existing, large, and widely used set of resource identifiers.







Berners-Lee, et al.         Standards Track                     [Page 4]
```

<a id="source-page-5-part-1"></a>

## Source page 5

```text

RFC 3986                   URI Generic Syntax               January 2005


   Resource

      This specification does not limit the scope of what might be a
      resource; rather, the term "resource" is used in a general sense
      for whatever might be identified by a URI.  Familiar examples
      include an electronic document, an image, a source of information
      with a consistent purpose (e.g., "today's weather report for Los
      Angeles"), a service (e.g., an HTTP-to-SMS gateway), and a
      collection of other resources.  A resource is not necessarily
      accessible via the Internet; e.g., human beings, corporations, and
      bound books in a library can also be resources.  Likewise,
      abstract concepts can be resources, such as the operators and
      operands of a mathematical equation, the types of a relationship
      (e.g., "parent" or "employee"), or numeric values (e.g., zero,
      one, and infinity).

   Identifier

      An identifier embodies the information required to distinguish
      what is being identified from all other things within its scope of
      identification.  Our use of the terms "identify" and "identifying"
      refer to this purpose of distinguishing one resource from all
      other resources, regardless of how that purpose is accomplished
      (e.g., by name, address, or context).  These terms should not be
      mistaken as an assumption that an identifier defines or embodies
      the identity of what is referenced, though that may be the case
      for some identifiers.  Nor should it be assumed that a system
      using URIs will access the resource identified: in many cases,
      URIs are used to denote resources without any intention that they
      be accessed.  Likewise, the "one" resource identified might not be
      singular in nature (e.g., a resource might be a named set or a
      mapping that varies over time).

   A URI is an identifier consisting of a sequence of characters
   matching the syntax rule named <URI> in Section 3.  It enables
   uniform identification of resources via a separately defined
   extensible set of naming schemes (Section 3.1).  How that
   identification is accomplished, assigned, or enabled is delegated to
   each scheme specification.

   This specification does not place any limits on the nature of a
   resource, the reasons why an application might seek to refer to a
   resource, or the kinds of systems that might use URIs for the sake of
   identifying resources.  This specification does not require that a
   URI persists in identifying the same resource over time, though that
   is a common goal of all URI schemes.  Nevertheless, nothing in this





Berners-Lee, et al.         Standards Track                     [Page 5]
```

