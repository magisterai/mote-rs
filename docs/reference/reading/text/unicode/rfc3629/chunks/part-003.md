# UTF-8, a transformation format of ISO 10646 — part 3

Source: [UTF-8, a transformation format of ISO 10646](https://www.rfc-editor.org/rfc/rfc3629.txt) · RFC 3629.

[Document index](../README.md) · [Topic guide](../../../../../README.md) · [Publisher](https://www.rfc-editor.org/rfc/rfc3629.txt).

Verbatim text slices in fixed-width blocks preserve grammar, examples, and diagrams.

[Previous](part-002.md)

<a id="source-page-11-part-1"></a>

## Source page 11

```text

RFC 3629                         UTF-8                     November 2003


   searching, sorting, regular expression matching and selection are
   involved.  An example would be string matching of an identifier
   appearing in a credential and in access control list entries.  This
   issue is amenable to solutions based on Unicode Normalization Forms,
   see [UAX15].

11.  Acknowledgements

   The following have participated in the drafting and discussion of
   this memo: James E. Agenbroad, Harald Alvestrand, Andries Brouwer,
   Mark Davis, Martin J. Duerst, Patrick Faltstrom, Ned Freed, David
   Goldsmith, Tony Hansen, Edwin F. Hart, Paul Hoffman, David Hopwood,
   Simon Josefsson, Kent Karlsson, Dan Kohn, Markus Kuhn, Michael Kung,
   Alain LaBonte, Ira McDonald, Alexey Melnikov, MURATA Makoto, John
   Gardiner Myers, Chris Newman, Dan Oscarsson, Roozbeh Pournader,
   Murray Sargent, Markus Scherer, Keld Simonsen, Arnold Winkler,
   Kenneth Whistler and Misha Wolf.

12.  Changes from RFC 2279

   o  Restricted the range of characters to 0000-10FFFF (the UTF-16
      accessible range).

   o  Made Unicode the source of the normative definition of UTF-8,
      keeping ISO/IEC 10646 as the reference for characters.

   o  Straightened out terminology.  UTF-8 now described in terms of an
      encoding form of the character number.  UCS-2 and UCS-4 almost
      disappeared.

   o  Turned the note warning against decoding of invalid sequences into
      a normative MUST NOT.

   o  Added a new section about the UTF-8 BOM, with advice for
      protocols.

   o  Removed suggested UNICODE-1-1-UTF-8 MIME charset registration.

   o  Added an ABNF syntax for valid UTF-8 octet sequences

   o  Expanded Security Considerations section, in particular impact of
      Unicode normalization









Yergeau                     Standards Track                    [Page 11]
```

<a id="source-page-12-part-1"></a>

## Source page 12

```text

RFC 3629                         UTF-8                     November 2003


13.  Normative References

   [RFC2119]   Bradner, S., "Key words for use in RFCs to Indicate
               Requirement Levels", BCP 14, RFC 2119, March 1997.

   [ISO.10646] International Organization for Standardization,
               "Information Technology - Universal Multiple-octet coded
               Character Set (UCS)", ISO/IEC Standard 10646,  comprised
               of ISO/IEC 10646-1:2000, "Information technology --
               Universal Multiple-Octet Coded Character Set (UCS) --
               Part 1: Architecture and Basic Multilingual Plane",
               ISO/IEC 10646-2:2001, "Information technology --
               Universal Multiple-Octet Coded Character Set (UCS) --
               Part 2:  Supplementary Planes" and ISO/IEC 10646-
               1:2000/Amd 1:2002, "Mathematical symbols and other
               characters".

   [UNICODE]   The Unicode Consortium, "The Unicode Standard -- Version
               4.0",  defined by The Unicode Standard, Version 4.0
               (Boston, MA, Addison-Wesley, 2003.  ISBN 0-321-18578-1),
               April 2003, <http://www.unicode.org/unicode/standard/
               versions/enumeratedversions.html#Unicode_4_0_0>.

14.  Informative References

   [CESU-8]    Phipps, T., "Unicode Technical Report #26: Compatibility
               Encoding Scheme for UTF-16: 8-Bit (CESU-8)", UTR 26,
               April 2002,
               <http://www.unicode.org/unicode/reports/tr26/>.

   [FSS_UTF]   X/Open Company Ltd., "X/Open Preliminary Specification --
               File System Safe UCS Transformation Format (FSS-UTF)",
               May 1993, <http://wwwold.dkuug.dk/jtc1/sc22/wg20/docs/
               N193-FSS-UTF.pdf>.

   [RFC2045]   Freed, N. and N. Borenstein, "Multipurpose Internet Mail
               Extensions (MIME) Part One: Format of Internet Message
               Bodies", RFC 2045, November 1996.

   [RFC2234]   Crocker, D. and P. Overell, "Augmented BNF for Syntax
               Specifications: ABNF", RFC 2234, November 1997.

   [RFC2978]   Freed, N. and J. Postel, "IANA Charset Registration
               Procedures", BCP 19, RFC 2978, October 2000.







Yergeau                     Standards Track                    [Page 12]
```

<a id="source-page-13-part-1"></a>

## Source page 13

```text

RFC 3629                         UTF-8                     November 2003


   [UAX15]     Davis, M. and M. Duerst, "Unicode Standard Annex #15:
               Unicode Normalization Forms",  An integral part of The
               Unicode Standard, Version 4.0.0, April 2003, <http://
               www.unicode.org/unicode/reports/tr15>.

   [US-ASCII]  American National Standards Institute, "Coded Character
               Set - 7-bit American Standard Code for Information
               Interchange", ANSI X3.4, 1986.

15.  URIs

   [1]  <http://www.unicode.org/unicode/standard/policies.html>

16.  Intellectual Property Statement

   The IETF takes no position regarding the validity or scope of any
   intellectual property or other rights that might be claimed to
   pertain to the implementation or use of the technology described in
   this document or the extent to which any license under such rights
   might or might not be available; neither does it represent that it
   has made any effort to identify any such rights.  Information on the
   IETF's procedures with respect to rights in standards-track and
   standards-related documentation can be found in BCP-11.  Copies of
   claims of rights made available for publication and any assurances of
   licenses to be made available, or the result of an attempt made to
   obtain a general license or permission for the use of such
   proprietary rights by implementors or users of this specification can
   be obtained from the IETF Secretariat.

   The IETF invites any interested party to bring to its attention any
   copyrights, patents or patent applications, or other proprietary
   rights which may cover technology that may be required to practice
   this standard.  Please address the information to the IETF Executive
   Director.

17.  Author's Address

   Francois Yergeau
   Alis Technologies
   100, boul. Alexis-Nihon, bureau 600
   Montreal, QC  H4M 2P2
   Canada

   Phone: +1 514 747 2547
   Fax:   +1 514 747 2561
   EMail: fyergeau@alis.com





Yergeau                     Standards Track                    [Page 13]
```

<a id="source-page-14-part-1"></a>

## Source page 14

```text

RFC 3629                         UTF-8                     November 2003


18.  Full Copyright Statement

   Copyright (C) The Internet Society (2003).  All Rights Reserved.

   This document and translations of it may be copied and furnished to
   others, and derivative works that comment on or otherwise explain it
   or assist in its implementation may be prepared, copied, published
   and distributed, in whole or in part, without restriction of any
   kind, provided that the above copyright notice and this paragraph are
   included on all such copies and derivative works.  However, this
   document itself may not be modified in any way, such as by removing
   the copyright notice or references to the Internet Society or other
   Internet organizations, except as needed for the purpose of
   developing Internet standards in which case the procedures for
   copyrights defined in the Internet Standards process must be
   followed, or as required to translate it into languages other than
   English.

   The limited permissions granted above are perpetual and will not be
   revoked by the Internet Society or its successors or assignees.

   This document and the information contained herein is provided on an
   "AS IS" basis and THE INTERNET SOCIETY AND THE INTERNET ENGINEERING
   TASK FORCE DISCLAIMS ALL WARRANTIES, EXPRESS OR IMPLIED, INCLUDING
   BUT NOT LIMITED TO ANY WARRANTY THAT THE USE OF THE INFORMATION
   HEREIN WILL NOT INFRINGE ANY RIGHTS OR ANY IMPLIED WARRANTIES OF
   MERCHANTABILITY OR FITNESS FOR A PARTICULAR PURPOSE.

Acknowledgement

   Funding for the RFC Editor function is currently provided by the
   Internet Society.



















Yergeau                     Standards Track                    [Page 14]
```

<a id="source-page-15-part-1"></a>

## Source page 15

```text

```

