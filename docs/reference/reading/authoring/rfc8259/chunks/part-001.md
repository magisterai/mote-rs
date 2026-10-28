# The JavaScript Object Notation (JSON) Data Interchange Format — part 1

Source: [The JavaScript Object Notation (JSON) Data Interchange Format](https://www.rfc-editor.org/rfc/rfc8259.txt) · RFC 8259.

[Document index](../README.md) · [Topic guide](../../../../README.md) · [Publisher](https://www.rfc-editor.org/rfc/rfc8259.txt).

Verbatim text slices in fixed-width blocks preserve grammar, examples, and diagrams.

[Next](part-002.md)

<a id="source-page-1-part-1"></a>

## Source page 1

```text






Internet Engineering Task Force (IETF)                      T. Bray, Ed.
Request for Comments: 8259                                    Textuality
Obsoletes: 7159                                            December 2017
Category: Standards Track
ISSN: 2070-1721


     The JavaScript Object Notation (JSON) Data Interchange Format

Abstract

   JavaScript Object Notation (JSON) is a lightweight, text-based,
   language-independent data interchange format.  It was derived from
   the ECMAScript Programming Language Standard.  JSON defines a small
   set of formatting rules for the portable representation of structured
   data.

   This document removes inconsistencies with other specifications of
   JSON, repairs specification errors, and offers experience-based
   interoperability guidance.

Status of This Memo

   This is an Internet Standards Track document.

   This document is a product of the Internet Engineering Task Force
   (IETF).  It represents the consensus of the IETF community.  It has
   received public review and has been approved for publication by the
   Internet Engineering Steering Group (IESG).  Further information on
   Internet Standards is available in Section 2 of RFC 7841.

   Information about the current status of this document, any errata,
   and how to provide feedback on it may be obtained at
   https://www.rfc-editor.org/info/rfc8259.

















Bray                         Standards Track                    [Page 1]
```

<a id="source-page-2-part-1"></a>

## Source page 2

```text

RFC 8259                          JSON                     December 2017


Copyright Notice

   Copyright (c) 2017 IETF Trust and the persons identified as the
   document authors.  All rights reserved.

   This document is subject to BCP 78 and the IETF Trust's Legal
   Provisions Relating to IETF Documents
   (https://trustee.ietf.org/license-info) in effect on the date of
   publication of this document.  Please review these documents
   carefully, as they describe your rights and restrictions with respect
   to this document.  Code Components extracted from this document must
   include Simplified BSD License text as described in Section 4.e of
   the Trust Legal Provisions and are provided without warranty as
   described in the Simplified BSD License.

   This document may contain material from IETF Documents or IETF
   Contributions published or made publicly available before November
   10, 2008.  The person(s) controlling the copyright in some of this
   material may not have granted the IETF Trust the right to allow
   modifications of such material outside the IETF Standards Process.
   Without obtaining an adequate license from the person(s) controlling
   the copyright in such materials, this document may not be modified
   outside the IETF Standards Process, and derivative works of it may
   not be created outside the IETF Standards Process, except to format
   it for publication as an RFC or to translate it into languages other
   than English.

























Bray                         Standards Track                    [Page 2]
```

<a id="source-page-3-part-1"></a>

## Source page 3

```text

RFC 8259                          JSON                     December 2017


Table of Contents

   1.  Introduction  . . . . . . . . . . . . . . . . . . . . . . . .   3
     1.1.  Conventions Used in This Document . . . . . . . . . . . .   4
     1.2.  Specifications of JSON  . . . . . . . . . . . . . . . . .   4
     1.3.  Introduction to This Revision . . . . . . . . . . . . . .   5
   2.  JSON Grammar  . . . . . . . . . . . . . . . . . . . . . . . .   5
   3.  Values  . . . . . . . . . . . . . . . . . . . . . . . . . . .   6
   4.  Objects . . . . . . . . . . . . . . . . . . . . . . . . . . .   6
   5.  Arrays  . . . . . . . . . . . . . . . . . . . . . . . . . . .   7
   6.  Numbers . . . . . . . . . . . . . . . . . . . . . . . . . . .   7
   7.  Strings . . . . . . . . . . . . . . . . . . . . . . . . . . .   8
   8.  String and Character Issues . . . . . . . . . . . . . . . . .   9
     8.1.  Character Encoding  . . . . . . . . . . . . . . . . . . .   9
     8.2.  Unicode Characters  . . . . . . . . . . . . . . . . . . .  10
     8.3.  String Comparison . . . . . . . . . . . . . . . . . . . .  10
   9.  Parsers . . . . . . . . . . . . . . . . . . . . . . . . . . .  10
   10. Generators  . . . . . . . . . . . . . . . . . . . . . . . . .  10
   11. IANA Considerations . . . . . . . . . . . . . . . . . . . . .  11
   12. Security Considerations . . . . . . . . . . . . . . . . . . .  12
   13. Examples  . . . . . . . . . . . . . . . . . . . . . . . . . .  12
   14. References  . . . . . . . . . . . . . . . . . . . . . . . . .  14
     14.1.  Normative References . . . . . . . . . . . . . . . . . .  14
     14.2.  Informative References . . . . . . . . . . . . . . . . .  14
   Appendix A.  Changes from RFC 7159  . . . . . . . . . . . . . . .  16
   Contributors  . . . . . . . . . . . . . . . . . . . . . . . . . .  16
   Author's Address  . . . . . . . . . . . . . . . . . . . . . . . .  16

1.  Introduction

   JavaScript Object Notation (JSON) is a text format for the
   serialization of structured data.  It is derived from the object
   literals of JavaScript, as defined in the ECMAScript Programming
   Language Standard, Third Edition [ECMA-262].

   JSON can represent four primitive types (strings, numbers, booleans,
   and null) and two structured types (objects and arrays).

   A string is a sequence of zero or more Unicode characters [UNICODE].
   Note that this citation references the latest version of Unicode
   rather than a specific release.  It is not expected that future
   changes in the Unicode specification will impact the syntax of JSON.

   An object is an unordered collection of zero or more name/value
   pairs, where a name is a string and a value is a string, number,
   boolean, null, object, or array.

   An array is an ordered sequence of zero or more values.



Bray                         Standards Track                    [Page 3]
```

<a id="source-page-4-part-1"></a>

## Source page 4

```text

RFC 8259                          JSON                     December 2017


   The terms "object" and "array" come from the conventions of
   JavaScript.

   JSON's design goals were for it to be minimal, portable, textual, and
   a subset of JavaScript.

1.1.  Conventions Used in This Document

   The key words "MUST", "MUST NOT", "REQUIRED", "SHALL", "SHALL NOT",
   "SHOULD", "SHOULD NOT", "RECOMMENDED", "NOT RECOMMENDED", "MAY", and
   "OPTIONAL" in this document are to be interpreted as described in BCP
   14 [RFC2119] [RFC8174] when, and only when, they appear in all
   capitals, as shown here.

   The grammatical rules in this document are to be interpreted as
   described in [RFC5234].

1.2.  Specifications of JSON

   This document replaces [RFC7159].  [RFC7159] obsoleted [RFC4627],
   which originally described JSON and registered the media type
   "application/json".

   JSON is also described in [ECMA-404].

   The reference to ECMA-404 in the previous sentence is normative, not
   with the usual meaning that implementors need to consult it in order
   to understand this document, but to emphasize that there are no
   inconsistencies in the definition of the term "JSON text" in any of
   its specifications.  Note, however, that ECMA-404 allows several
   practices that this specification recommends avoiding in the
   interests of maximal interoperability.

   The intent is that the grammar is the same between the two documents,
   although different descriptions are used.  If there is a difference
   found between them, ECMA and the IETF will work together to update
   both documents.

   If an error is found with either document, the other should be
   examined to see if it has a similar error; if it does, it should be
   fixed, if possible.

   If either document is changed in the future, ECMA and the IETF will
   work together to ensure that the two documents stay aligned through
   the change.






Bray                         Standards Track                    [Page 4]
```

<a id="source-page-5-part-1"></a>

## Source page 5

```text

RFC 8259                          JSON                     December 2017


1.3.  Introduction to This Revision

   In the years since the publication of RFC 4627, JSON has found very
   wide use.  This experience has revealed certain patterns that, while
   allowed by its specifications, have caused interoperability problems.

   Also, a small number of errata have been reported regarding RFC 4627
   (see RFC Errata IDs 607 [Err607] and 3607 [Err3607]) and regarding
   RFC 7159 (see RFC Errata IDs 3915 [Err3915], 4264 [Err4264], 4336
   [Err4336], and 4388 [Err4388]).

   This document's goal is to apply the errata, remove inconsistencies
   with other specifications of JSON, and highlight practices that can
   lead to interoperability problems.

2.  JSON Grammar

   A JSON text is a sequence of tokens.  The set of tokens includes six
   structural characters, strings, numbers, and three literal names.

   A JSON text is a serialized value.  Note that certain previous
   specifications of JSON constrained a JSON text to be an object or an
   array.  Implementations that generate only objects or arrays where a
   JSON text is called for will be interoperable in the sense that all
   implementations will accept these as conforming JSON texts.

      JSON-text = ws value ws

   These are the six structural characters:

      begin-array     = ws %x5B ws  ; [ left square bracket

      begin-object    = ws %x7B ws  ; { left curly bracket

      end-array       = ws %x5D ws  ; ] right square bracket

      end-object      = ws %x7D ws  ; } right curly bracket

      name-separator  = ws %x3A ws  ; : colon

      value-separator = ws %x2C ws  ; , comma










Bray                         Standards Track                    [Page 5]
```

<a id="source-page-6-part-1"></a>

## Source page 6

```text

RFC 8259                          JSON                     December 2017


   Insignificant whitespace is allowed before or after any of the six
   structural characters.

      ws = *(
              %x20 /              ; Space
              %x09 /              ; Horizontal tab
              %x0A /              ; Line feed or New line
              %x0D )              ; Carriage return

3.  Values

   A JSON value MUST be an object, array, number, or string, or one of
   the following three literal names:

      false
      null
      true

   The literal names MUST be lowercase.  No other literal names are
   allowed.

      value = false / null / true / object / array / number / string

      false = %x66.61.6c.73.65   ; false

      null  = %x6e.75.6c.6c      ; null

      true  = %x74.72.75.65      ; true

4.  Objects

   An object structure is represented as a pair of curly brackets
   surrounding zero or more name/value pairs (or members).  A name is a
   string.  A single colon comes after each name, separating the name
   from the value.  A single comma separates a value from a following
   name.  The names within an object SHOULD be unique.

      object = begin-object [ member *( value-separator member ) ]
               end-object

      member = string name-separator value

   An object whose names are all unique is interoperable in the sense
   that all software implementations receiving that object will agree on
   the name-value mappings.  When the names within an object are not
   unique, the behavior of software that receives such an object is
   unpredictable.  Many implementations report the last name/value pair
   only.  Other implementations report an error or fail to parse the



Bray                         Standards Track                    [Page 6]
```

<a id="source-page-7-part-1"></a>

## Source page 7

```text

RFC 8259                          JSON                     December 2017


   object, and some implementations report all of the name/value pairs,
   including duplicates.

   JSON parsing libraries have been observed to differ as to whether or
   not they make the ordering of object members visible to calling
   software.  Implementations whose behavior does not depend on member
   ordering will be interoperable in the sense that they will not be
   affected by these differences.

5.  Arrays

   An array structure is represented as square brackets surrounding zero
   or more values (or elements).  Elements are separated by commas.

   array = begin-array [ value *( value-separator value ) ] end-array

   There is no requirement that the values in an array be of the same
   type.

6.  Numbers

   The representation of numbers is similar to that used in most
   programming languages.  A number is represented in base 10 using
   decimal digits.  It contains an integer component that may be
   prefixed with an optional minus sign, which may be followed by a
   fraction part and/or an exponent part.  Leading zeros are not
   allowed.

   A fraction part is a decimal point followed by one or more digits.

   An exponent part begins with the letter E in uppercase or lowercase,
   which may be followed by a plus or minus sign.  The E and optional
   sign are followed by one or more digits.

   Numeric values that cannot be represented in the grammar below (such
   as Infinity and NaN) are not permitted.

      number = [ minus ] int [ frac ] [ exp ]

      decimal-point = %x2E       ; .

      digit1-9 = %x31-39         ; 1-9

      e = %x65 / %x45            ; e E

      exp = e [ minus / plus ] 1*DIGIT

      frac = decimal-point 1*DIGIT



Bray                         Standards Track                    [Page 7]
```

