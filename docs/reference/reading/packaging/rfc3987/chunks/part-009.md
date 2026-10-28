# Internationalized Resource Identifiers (IRIs) — part 9

Source: [Internationalized Resource Identifiers (IRIs)](https://www.rfc-editor.org/rfc/rfc3987.txt) · RFC 3987.

[Document index](../README.md) · [Topic guide](../../../../README.md) · [Publisher](https://www.rfc-editor.org/rfc/rfc3987.txt).

Verbatim text slices in fixed-width blocks preserve grammar, examples, and diagrams.

[Previous](part-008.md)

<a id="source-page-42-part-1"></a>

## Source page 42

```text

RFC 3987         Internationalized Resource Identifiers     January 2005


   [RFC2368]      Hoffman, P., Masinter, L., and J. Zawinski, "The
                  mailto URL scheme", RFC 2368, July 1998.

   [RFC2384]      Gellens, R., "POP URL Scheme", RFC 2384, August 1998.

   [RFC2396]      Berners-Lee, T., Fielding, R., and L. Masinter,
                  "Uniform Resource Identifiers (URI): Generic Syntax",
                  RFC 2396, August 1998.

   [RFC2397]      Masinter, L., "The "data" URL scheme", RFC 2397,
                  August 1998.

   [RFC2616]      Fielding,  R., Gettys, J., Mogul, J., Frystyk, H.,
                  Masinter, L., Leach, P., and T. Berners-Lee,
                  "Hypertext Transfer Protocol -- HTTP/1.1", RFC 2616,
                  June 1999.

   [RFC2640]      Curtin, B., "Internationalization of the File Transfer
                  Protocol", RFC 2640, July 1999.

   [RFC2718]      Masinter, L., Alvestrand, H., Zigmond, D., and R.
                  Petke, "Guidelines for new URL Schemes", RFC 2718,
                  November 1999.

   [UNIXML]       Duerst, M. and A. Freytag, "Unicode in XML and other
                  Markup Languages", Unicode Technical Report #20, World
                  Wide Web Consortium Note, June 2003,
                  <http://www.w3.org/TR/unicode-xml/>.

   [XLink]        DeRose, S., Maler, E., and D. Orchard, "XML Linking
                  Language (XLink) Version 1.0", World Wide Web
                  Consortium Recommendation, June 2001,
                  <http://www.w3.org/TR/xlink/#link-locators>.

   [XML1]         Bray, T., Paoli, J., Sperberg-McQueen, C., Maler, E.,
                  and F. Yergeau, "Extensible Markup Language (XML) 1.0
                  (Third Edition)", World Wide Web Consortium
                  Recommendation, February 2004,
                  <http://www.w3.org/TR/REC-xml#sec-external-ent>.

   [XMLNamespace] Bray, T., Hollander, D., and A. Layman, "Namespaces in
                  XML", World Wide Web Consortium Recommendation,
                  January 1999, <http://www.w3.org/TR/REC-xml-names>.

   [XMLSchema]    Biron, P. and A. Malhotra, "XML Schema Part 2:
                  Datatypes", World Wide Web Consortium Recommendation,
                  May 2001, <http://www.w3.org/TR/xmlschema-2/#anyURI>.




Duerst & Suignard           Standards Track                    [Page 42]
```

<a id="source-page-43-part-1"></a>

## Source page 43

```text

RFC 3987         Internationalized Resource Identifiers     January 2005


   [XPointer]     Grosso, P., Maler, E., Marsh, J. and N. Walsh,
                  "XPointer Framework", World Wide Web Consortium
                  Recommendation, March 2003,
                  <http://www.w3.org/TR/xptr-framework/#escaping>.















































Duerst & Suignard           Standards Track                    [Page 43]
```

<a id="source-page-44-part-1"></a>

## Source page 44

```text

RFC 3987         Internationalized Resource Identifiers     January 2005


Appendix A.  Design Alternatives

   This section shortly summarizes major design alternatives and the
   reasons for why they were not chosen.

Appendix A.1.  New Scheme(s)

   Introducing new schemes (for example, httpi:, ftpi:,...) or a new
   metascheme (e.g., i:, leading to URI/IRI prefixes such as i:http:,
   i:ftp:,...) was proposed to make IRI-to-URI conversion scheme
   dependent or to distinguish between percent-encodings resulting from
   IRI-to-URI conversion and percent-encodings from legacy character
   encodings.

   New schemes are not needed to distinguish URIs from true IRIs (i.e.,
   IRIs that contain non-ASCII characters).  The benefit of being able
   to detect the origin of percent-encodings is marginal, as UTF-8 can
   be detected with very high reliability.  Deploying new schemes is
   extremely hard, so not requiring new schemes for IRIs makes
   deployment of IRIs vastly easier.  Making conversion scheme dependent
   is highly inadvisable and would be encouraged by separate schemes for
   IRIs.  Using a uniform convention for conversion from IRIs to URIs
   makes IRI implementation orthogonal to the introduction of actual new
   schemes.

Appendix A.2.  Character Encodings Other Than UTF-8

   At an early stage, UTF-7 was considered as an alternative to UTF-8
   when IRIs are converted to URIs.  UTF-7 would not have needed
   percent-encoding and in most cases would have been shorter than
   percent-encoded UTF-8.

   Using UTF-8 avoids a double layering and overloading of the use of
   the "+" character.  UTF-8 is fully compatible with US-ASCII and has
   therefore been recommended by the IETF, and is being used widely.

   UTF-7 has never been used much and is now clearly being discouraged.
   Requiring implementations to convert from UTF-8 to UTF-7 and back
   would be an additional implementation burden.

Appendix A.3.  New Encoding Convention

   Instead of using the existing percent-encoding convention of URIs,
   which is based on octets, the idea was to create a new encoding
   convention; for example, to use "%u" to introduce UCS code points.






Duerst & Suignard           Standards Track                    [Page 44]
```

<a id="source-page-45-part-1"></a>

## Source page 45

```text

RFC 3987         Internationalized Resource Identifiers     January 2005


   Using the existing octet-based percent-encoding mechanism does not
   need an upgrade of the URI syntax and does not need corresponding
   server upgrades.

Appendix A.4.  Indicating Character Encodings in the URI/IRI

   Some proposals suggested indicating the character encodings used in
   an URI or IRI with some new syntactic convention in the URI itself,
   similar to the "charset" parameter for e-mails and Web pages.  As an
   example, the label in square brackets in
   "http://www.example.org/ros[iso-8859-1]&#xE9"; indicated that the
   following "&#xE9"; had to be interpreted as iso-8859-1.

   If UTF-8 is used exclusively, an upgrade to the URI syntax is not
   needed.  It avoids potentially multiple labels that have to be copied
   correctly in all cases, even on the side of a bus or on a napkin,
   leading to usability problems (and being prohibitively annoying).
   Exclusively using UTF-8 also reduces transcoding errors and
   confusion.

Authors' Addresses

   Martin Duerst  (Note: Please write "Duerst" with u-umlaut wherever
                  possible, for example as "D&#252;rst" in XML and
                  HTML.)
   World Wide Web Consortium
   5322 Endo
   Fujisawa, Kanagawa  252-8520
   Japan

   Phone: +81 466 49 1170
   Fax:   +81 466 49 1171
   EMail: duerst@w3.org
   URI:   http://www.w3.org/People/D%C3%BCrst/
   (Note: This is the percent-encoded form of an IRI.)


   Michel Suignard
   Microsoft Corporation
   One Microsoft Way
   Redmond, WA  98052
   U.S.A.

   Phone: +1 425 882-8080
   EMail: michelsu@microsoft.com
   URI:   http://www.suignard.com





Duerst & Suignard           Standards Track                    [Page 45]
```

<a id="source-page-46-part-1"></a>

## Source page 46

```text

RFC 3987         Internationalized Resource Identifiers     January 2005


Full Copyright Statement

   Copyright (C) The Internet Society (2005).

   This document is subject to the rights, licenses and restrictions
   contained in BCP 78, and except as set forth therein, the authors
   retain all their rights.

   This document and the information contained herein are provided on an
   "AS IS" basis and THE CONTRIBUTOR, THE ORGANIZATION HE/SHE REPRESENTS
   OR IS SPONSORED BY (IF ANY), THE INTERNET SOCIETY AND THE INTERNET
   ENGINEERING TASK FORCE DISCLAIM ALL WARRANTIES, EXPRESS OR IMPLIED,
   INCLUDING BUT NOT LIMITED TO ANY WARRANTY THAT THE USE OF THE
   INFORMATION HEREIN WILL NOT INFRINGE ANY RIGHTS OR ANY IMPLIED
   WARRANTIES OF MERCHANTABILITY OR FITNESS FOR A PARTICULAR PURPOSE.

Intellectual Property

   The IETF takes no position regarding the validity or scope of any
   Intellectual Property Rights or other rights that might be claimed to
   pertain to the implementation or use of the technology described in
   this document or the extent to which any license under such rights
   might or might not be available; nor does it represent that it has
   made any independent effort to identify any such rights.  Information
   on the IETF's procedures with respect to rights in IETF Documents can
   be found in BCP 78 and BCP 79.

   Copies of IPR disclosures made to the IETF Secretariat and any
   assurances of licenses to be made available, or the result of an
   attempt made to obtain a general license or permission for the use of
   such proprietary rights by implementers or users of this
   specification can be obtained from the IETF on-line IPR repository at
   http://www.ietf.org/ipr.

   The IETF invites any interested party to bring to its attention any
   copyrights, patents or patent applications, or other proprietary
   rights that may cover technology that may be required to implement
   this standard.  Please address the information to the IETF at ietf-
   ipr@ietf.org.


Acknowledgement

   Funding for the RFC Editor function is currently provided by the
   Internet Society.






Duerst & Suignard           Standards Track                    [Page 46]
```

<a id="source-page-47-part-1"></a>

## Source page 47

```text

```

