# UTF-8, a transformation format of ISO 10646 — part 1

Source: [UTF-8, a transformation format of ISO 10646](https://www.rfc-editor.org/rfc/rfc3629.txt) · RFC 3629.

[Document index](../README.md) · [Topic guide](../../../../../README.md) · [Publisher](https://www.rfc-editor.org/rfc/rfc3629.txt).

Verbatim text slices in fixed-width blocks preserve grammar, examples, and diagrams.

[Next](part-002.md)

<a id="source-page-1-part-1"></a>

## Source page 1

```text






Network Working Group                                         F. Yergeau
Request for Comments: 3629                             Alis Technologies
STD: 63                                                    November 2003
Obsoletes: 2279
Category: Standards Track


              UTF-8, a transformation format of ISO 10646

Status of this Memo

   This document specifies an Internet standards track protocol for the
   Internet community, and requests discussion and suggestions for
   improvements.  Please refer to the current edition of the "Internet
   Official Protocol Standards" (STD 1) for the standardization state
   and status of this protocol.  Distribution of this memo is unlimited.

Copyright Notice

   Copyright (C) The Internet Society (2003).  All Rights Reserved.

Abstract

   ISO/IEC 10646-1 defines a large character set called the Universal
   Character Set (UCS) which encompasses most of the world's writing
   systems.  The originally proposed encodings of the UCS, however, were
   not compatible with many current applications and protocols, and this
   has led to the development of UTF-8, the object of this memo.  UTF-8
   has the characteristic of preserving the full US-ASCII range,
   providing compatibility with file systems, parsers and other software
   that rely on US-ASCII values but are transparent to other values.
   This memo obsoletes and replaces RFC 2279.

Table of Contents

   1.  Introduction . . . . . . . . . . . . . . . . . . . . . . . . .  2
   2.  Notational conventions . . . . . . . . . . . . . . . . . . . .  3
   3.  UTF-8 definition . . . . . . . . . . . . . . . . . . . . . . .  4
   4.  Syntax of UTF-8 Byte Sequences . . . . . . . . . . . . . . . .  5
   5.  Versions of the standards  . . . . . . . . . . . . . . . . . .  6
   6.  Byte order mark (BOM)  . . . . . . . . . . . . . . . . . . . .  6
   7.  Examples . . . . . . . . . . . . . . . . . . . . . . . . . . .  8
   8.  MIME registration  . . . . . . . . . . . . . . . . . . . . . .  9
   9.  IANA Considerations  . . . . . . . . . . . . . . . . . . . . . 10
   10. Security Considerations  . . . . . . . . . . . . . . . . . . . 10
   11. Acknowledgements . . . . . . . . . . . . . . . . . . . . . . . 11
   12. Changes from RFC 2279  . . . . . . . . . . . . . . . . . . . . 11
   13. Normative References . . . . . . . . . . . . . . . . . . . . . 12



Yergeau                     Standards Track                     [Page 1]
```

<a id="source-page-2-part-1"></a>

## Source page 2

```text

RFC 3629                         UTF-8                     November 2003


   14. Informative References . . . . . . . . . . . . . . . . . . . . 12
   15. URI's  . . . . . . . . . . . . . . . . . . . . . . . . . . . . 13
   16. Intellectual Property Statement  . . . . . . . . . . . . . . . 13
   17. Author's Address . . . . . . . . . . . . . . . . . . . . . . . 13
   18. Full Copyright Statement . . . . . . . . . . . . . . . . . . . 14

1. Introduction

   ISO/IEC 10646 [ISO.10646] defines a large character set called the
   Universal Character Set (UCS), which encompasses most of the world's
   writing systems.  The same set of characters is defined by the
   Unicode standard [UNICODE], which further defines additional
   character properties and other application details of great interest
   to implementers.  Up to the present time, changes in Unicode and
   amendments and additions to ISO/IEC 10646 have tracked each other, so
   that the character repertoires and code point assignments have
   remained in sync.  The relevant standardization committees have
   committed to maintain this very useful synchronism.

   ISO/IEC 10646 and Unicode define several encoding forms of their
   common repertoire: UTF-8, UCS-2, UTF-16, UCS-4 and UTF-32.  In an
   encoding form, each character is represented as one or more encoding
   units.  All standard UCS encoding forms except UTF-8 have an encoding
   unit larger than one octet, making them hard to use in many current
   applications and protocols that assume 8 or even 7 bit characters.

   UTF-8, the object of this memo, has a one-octet encoding unit.  It
   uses all bits of an octet, but has the quality of preserving the full
   US-ASCII [US-ASCII] range: US-ASCII characters are encoded in one
   octet having the normal US-ASCII value, and any octet with such a
   value can only stand for a US-ASCII character, and nothing else.

   UTF-8 encodes UCS characters as a varying number of octets, where the
   number of octets, and the value of each, depend on the integer value
   assigned to the character in ISO/IEC 10646 (the character number,
   a.k.a. code position, code point or Unicode scalar value).  This
   encoding form has the following characteristics (all values are in
   hexadecimal):

   o  Character numbers from U+0000 to U+007F (US-ASCII repertoire)
      correspond to octets 00 to 7F (7 bit US-ASCII values).  A direct
      consequence is that a plain ASCII string is also a valid UTF-8
      string.








Yergeau                     Standards Track                     [Page 2]
```

<a id="source-page-3-part-1"></a>

## Source page 3

```text

RFC 3629                         UTF-8                     November 2003


   o  US-ASCII octet values do not appear otherwise in a UTF-8 encoded
      character stream.  This provides compatibility with file systems
      or other software (e.g., the printf() function in C libraries)
      that parse based on US-ASCII values but are transparent to other
      values.

   o  Round-trip conversion is easy between UTF-8 and other encoding
      forms.

   o  The first octet of a multi-octet sequence indicates the number of
      octets in the sequence.

   o  The octet values C0, C1, F5 to FF never appear.

   o  Character boundaries are easily found from anywhere in an octet
      stream.

   o  The byte-value lexicographic sorting order of UTF-8 strings is the
      same as if ordered by character numbers.  Of course this is of
      limited interest since a sort order based on character numbers is
      almost never culturally valid.

   o  The Boyer-Moore fast search algorithm can be used with UTF-8 data.

   o  UTF-8 strings can be fairly reliably recognized as such by a
      simple algorithm, i.e., the probability that a string of
      characters in any other encoding appears as valid UTF-8 is low,
      diminishing with increasing string length.

   UTF-8 was devised in September 1992 by Ken Thompson, guided by design
   criteria specified by Rob Pike, with the objective of defining a UCS
   transformation format usable in the Plan9 operating system in a non-
   disruptive manner.  Thompson's design was stewarded through
   standardization by the X/Open Joint Internationalization Group XOJIG
   (see [FSS_UTF]), bearing the names FSS-UTF (variant FSS/UTF), UTF-2
   and finally UTF-8 along the way.

2.  Notational conventions

   The key words "MUST", "MUST NOT", "REQUIRED", "SHALL", "SHALL NOT",
   "SHOULD", "SHOULD NOT", "RECOMMENDED", "MAY", and "OPTIONAL" in this
   document are to be interpreted as described in [RFC2119].

   UCS characters are designated by the U+HHHH notation, where HHHH is a
   string of from 4 to 6 hexadecimal digits representing the character
   number in ISO/IEC 10646.





Yergeau                     Standards Track                     [Page 3]
```

<a id="source-page-4-part-1"></a>

## Source page 4

```text

RFC 3629                         UTF-8                     November 2003


3.  UTF-8 definition

   UTF-8 is defined by the Unicode Standard [UNICODE].  Descriptions and
   formulae can also be found in Annex D of ISO/IEC 10646-1 [ISO.10646]

   In UTF-8, characters from the U+0000..U+10FFFF range (the UTF-16
   accessible range) are encoded using sequences of 1 to 4 octets.  The
   only octet of a "sequence" of one has the higher-order bit set to 0,
   the remaining 7 bits being used to encode the character number.  In a
   sequence of n octets, n>1, the initial octet has the n higher-order
   bits set to 1, followed by a bit set to 0.  The remaining bit(s) of
   that octet contain bits from the number of the character to be
   encoded.  The following octet(s) all have the higher-order bit set to
   1 and the following bit set to 0, leaving 6 bits in each to contain
   bits from the character to be encoded.

   The table below summarizes the format of these different octet types.
   The letter x indicates bits available for encoding bits of the
   character number.

   Char. number range  |        UTF-8 octet sequence
      (hexadecimal)    |              (binary)
   --------------------+---------------------------------------------
   0000 0000-0000 007F | 0xxxxxxx
   0000 0080-0000 07FF | 110xxxxx 10xxxxxx
   0000 0800-0000 FFFF | 1110xxxx 10xxxxxx 10xxxxxx
   0001 0000-0010 FFFF | 11110xxx 10xxxxxx 10xxxxxx 10xxxxxx

   Encoding a character to UTF-8 proceeds as follows:

   1.  Determine the number of octets required from the character number
       and the first column of the table above.  It is important to note
       that the rows of the table are mutually exclusive, i.e., there is
       only one valid way to encode a given character.

   2.  Prepare the high-order bits of the octets as per the second
       column of the table.

   3.  Fill in the bits marked x from the bits of the character number,
       expressed in binary.  Start by putting the lowest-order bit of
       the character number in the lowest-order position of the last
       octet of the sequence, then put the next higher-order bit of the
       character number in the next higher-order position of that octet,
       etc.  When the x bits of the last octet are filled in, move on to
       the next to last octet, then to the preceding one, etc. until all
       x bits are filled in.





Yergeau                     Standards Track                     [Page 4]
```

<a id="source-page-5-part-1"></a>

## Source page 5

```text

RFC 3629                         UTF-8                     November 2003


   The definition of UTF-8 prohibits encoding character numbers between
   U+D800 and U+DFFF, which are reserved for use with the UTF-16
   encoding form (as surrogate pairs) and do not directly represent
   characters.  When encoding in UTF-8 from UTF-16 data, it is necessary
   to first decode the UTF-16 data to obtain character numbers, which
   are then encoded in UTF-8 as described above.  This contrasts with
   CESU-8 [CESU-8], which is a UTF-8-like encoding that is not meant for
   use on the Internet.  CESU-8 operates similarly to UTF-8 but encodes
   the UTF-16 code values (16-bit quantities) instead of the character
   number (code point).  This leads to different results for character
   numbers above 0xFFFF; the CESU-8 encoding of those characters is NOT
   valid UTF-8.

   Decoding a UTF-8 character proceeds as follows:

   1.  Initialize a binary number with all bits set to 0.  Up to 21 bits
       may be needed.

   2.  Determine which bits encode the character number from the number
       of octets in the sequence and the second column of the table
       above (the bits marked x).

   3.  Distribute the bits from the sequence to the binary number, first
       the lower-order bits from the last octet of the sequence and
       proceeding to the left until no x bits are left.  The binary
       number is now equal to the character number.

   Implementations of the decoding algorithm above MUST protect against
   decoding invalid sequences.  For instance, a naive implementation may
   decode the overlong UTF-8 sequence C0 80 into the character U+0000,
   or the surrogate pair ED A1 8C ED BE B4 into U+233B4.  Decoding
   invalid sequences may have security consequences or cause other
   problems.  See Security Considerations (Section 10) below.

4.  Syntax of UTF-8 Byte Sequences

   For the convenience of implementors using ABNF, a definition of UTF-8
   in ABNF syntax is given here.

   A UTF-8 string is a sequence of octets representing a sequence of UCS
   characters.  An octet sequence is valid UTF-8 only if it matches the
   following syntax, which is derived from the rules for encoding UTF-8
   and is expressed in the ABNF of [RFC2234].

   UTF8-octets = *( UTF8-char )
   UTF8-char   = UTF8-1 / UTF8-2 / UTF8-3 / UTF8-4
   UTF8-1      = %x00-7F
   UTF8-2      = %xC2-DF UTF8-tail



Yergeau                     Standards Track                     [Page 5]
```

