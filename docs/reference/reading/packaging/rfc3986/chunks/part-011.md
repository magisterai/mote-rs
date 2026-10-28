# Uniform Resource Identifier (URI): Generic Syntax — part 11

Source: [Uniform Resource Identifier (URI): Generic Syntax](https://www.rfc-editor.org/rfc/rfc3986.txt) · RFC 3986.

[Document index](../README.md) · [Topic guide](../../../../README.md) · [Publisher](https://www.rfc-editor.org/rfc/rfc3986.txt).

Verbatim text slices in fixed-width blocks preserve grammar, examples, and diagrams.

[Previous](part-010.md)

<a id="source-page-55-part-1"></a>

## Source page 55

```text

RFC 3986                   URI Generic Syntax               January 2005


   corresponding to unreserved characters.  In general, the terms
   "escaped" and "unescaped" have been replaced with "percent-encoded"
   and "decoded", respectively, to reduce confusion with other forms of
   escape mechanisms.

   The ABNF for URI and URI-reference has been redesigned to make them
   more friendly to LALR parsers and to reduce complexity.  As a result,
   the layout form of syntax description has been removed, along with
   the uric, uric_no_slash, opaque_part, net_path, abs_path, rel_path,
   path_segments, rel_segment, and mark rules.  All references to
   "opaque" URIs have been replaced with a better description of how the
   path component may be opaque to hierarchy.  The relativeURI rule has
   been replaced with relative-ref to avoid unnecessary confusion over
   whether they are a subset of URI.  The ambiguity regarding the
   parsing of URI-reference as a URI or a relative-ref with a colon in
   the first segment has been eliminated through the use of five
   separate path matching rules.

   The fragment identifier has been moved back into the section on
   generic syntax components and within the URI and relative-ref rules,
   though it remains excluded from absolute-URI.  The number sign ("#")
   character has been moved back to the reserved set as a result of
   reintegrating the fragment syntax.

   The ABNF has been corrected to allow the path component to be empty.
   This also allows an absolute-URI to consist of nothing after the
   "scheme:", as is present in practice with the "dav:" namespace
   [RFC2518] and with the "about:" scheme used internally by many WWW
   browser implementations.  The ambiguity regarding the boundary
   between authority and path has been eliminated through the use of
   five separate path matching rules.

   Registry-based naming authorities that use the generic syntax are now
   defined within the host rule.  This change allows current
   implementations, where whatever name provided is simply fed to the
   local name resolution mechanism, to be consistent with the
   specification.  It also removes the need to re-specify DNS name
   formats here.  Furthermore, it allows the host component to contain
   percent-encoded octets, which is necessary to enable
   internationalized domain names to be provided in URIs, processed in
   their native character encodings at the application layers above URI
   processing, and passed to an IDNA library as a registered name in the
   UTF-8 character encoding.  The server, hostport, hostname,
   domainlabel, toplabel, and alphanum rules have been removed.

   The resolving relative references algorithm of [RFC2396] has been
   rewritten with pseudocode for this revision to improve clarity and
   fix the following issues:



Berners-Lee, et al.         Standards Track                    [Page 55]
```

<a id="source-page-56-part-1"></a>

## Source page 56

```text

RFC 3986                   URI Generic Syntax               January 2005


   o  [RFC2396] section 5.2, step 6a, failed to account for a base URI
      with no path.

   o  Restored the behavior of [RFC1808] where, if the reference
      contains an empty path and a defined query component, the target
      URI inherits the base URI's path component.

   o  The determination of whether a URI reference is a same-document
      reference has been decoupled from the URI parser, simplifying the
      URI processing interface within applications in a way consistent
      with the internal architecture of deployed URI processing
      implementations.  The determination is now based on comparison to
      the base URI after transforming a reference to absolute form,
      rather than on the format of the reference itself.  This change
      may result in more references being considered "same-document"
      under this specification than there would be under the rules given
      in RFC 2396, especially when normalization is used to reduce
      aliases.  However, it does not change the status of existing
      same-document references.

   o  Separated the path merge routine into two routines: merge, for
      describing combination of the base URI path with a relative-path
      reference, and remove_dot_segments, for describing how to remove
      the special "." and ".." segments from a composed path.  The
      remove_dot_segments algorithm is now applied to all URI reference
      paths in order to match common implementations and to improve the
      normalization of URIs in practice.  This change only impacts the
      parsing of abnormal references and same-scheme references wherein
      the base URI has a non-hierarchical path.

Index

   A
      ABNF  11
      absolute  27
      absolute-path  26
      absolute-URI  27
      access  9
      authority  17, 18

   B
      base URI  28

   C
      character encoding  4
      character  4
      characters  8, 11
      coded character set  4



Berners-Lee, et al.         Standards Track                    [Page 56]
```

<a id="source-page-57-part-1"></a>

## Source page 57

```text

RFC 3986                   URI Generic Syntax               January 2005


   D
      dec-octet  20
      dereference  9
      dot-segments  23

   F
      fragment  16, 24

   G
      gen-delims  13
      generic syntax  6

   H
      h16  20
      hier-part  16
      hierarchical  10
      host  18

   I
      identifier  5
      IP-literal  19
      IPv4  20
      IPv4address  19, 20
      IPv6  19
      IPv6address  19, 20
      IPvFuture  19

   L
      locator  7
      ls32  20

   M
      merge  32

   N
      name  7
      network-path  26

   P
      path  16, 22, 26
         path-abempty  22
         path-absolute  22
         path-empty  22
         path-noscheme  22
         path-rootless  22
      path-abempty  16, 22, 26
      path-absolute  16, 22, 26
      path-empty  16, 22, 26



Berners-Lee, et al.         Standards Track                    [Page 57]
```

<a id="source-page-58-part-1"></a>

## Source page 58

```text

RFC 3986                   URI Generic Syntax               January 2005


      path-rootless  16, 22
      pchar  23
      pct-encoded  12
      percent-encoding  12
      port  22

   Q
      query  16, 23

   R
      reg-name  21
      registered name  20
      relative  10, 28
      relative-path  26
      relative-ref  26
      remove_dot_segments  33
      representation  9
      reserved  12
      resolution  9, 28
      resource  5
      retrieval  9

   S
      same-document  27
      sameness  9
      scheme  16, 17
      segment  22, 23
         segment-nz  23
         segment-nz-nc  23
      sub-delims  13
      suffix  27

   T
      transcription  8

   U
      uniform  4
      unreserved  13
      URI grammar
         absolute-URI  27
         ALPHA  11
         authority  18
         CR  11
         dec-octet  20
         DIGIT  11
         DQUOTE  11
         fragment  24
         gen-delims  13



Berners-Lee, et al.         Standards Track                    [Page 58]
```

<a id="source-page-59-part-1"></a>

## Source page 59

```text

RFC 3986                   URI Generic Syntax               January 2005


         h16  20
         HEXDIG  11
         hier-part  16
         host  19
         IP-literal  19
         IPv4address  20
         IPv6address  20
         IPvFuture  19
         LF  11
         ls32  20
         OCTET  11
         path  22
         path-abempty  22
         path-absolute  22
         path-empty  22
         path-noscheme  22
         path-rootless  22
         pchar  23
         pct-encoded  12
         port  22
         query  24
         reg-name  21
         relative-ref  26
         reserved  13
         scheme  17
         segment  23
         segment-nz  23
         segment-nz-nc  23
         SP  11
         sub-delims  13
         unreserved  13
         URI  16
         URI-reference  25
         userinfo  18
      URI  16
      URI-reference  25
      URL  7
      URN  7
      userinfo  18












Berners-Lee, et al.         Standards Track                    [Page 59]
```

<a id="source-page-60-part-1"></a>

## Source page 60

```text

RFC 3986                   URI Generic Syntax               January 2005


Authors' Addresses

   Tim Berners-Lee
   World Wide Web Consortium
   Massachusetts Institute of Technology
   77 Massachusetts Avenue
   Cambridge, MA  02139
   USA

   Phone: +1-617-253-5702
   Fax:   +1-617-258-5999
   EMail: timbl@w3.org
   URI:   http://www.w3.org/People/Berners-Lee/


   Roy T. Fielding
   Day Software
   5251 California Ave., Suite 110
   Irvine, CA  92617
   USA

   Phone: +1-949-679-2960
   Fax:   +1-949-679-2972
   EMail: fielding@gbiv.com
   URI:   http://roy.gbiv.com/


   Larry Masinter
   Adobe Systems Incorporated
   345 Park Ave
   San Jose, CA  95110
   USA

   Phone: +1-408-536-3024
   EMail: LMM@acm.org
   URI:   http://larry.masinter.net/















Berners-Lee, et al.         Standards Track                    [Page 60]
```

<a id="source-page-61-part-1"></a>

## Source page 61

```text

RFC 3986                   URI Generic Syntax               January 2005


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






Berners-Lee, et al.         Standards Track                    [Page 61]
```

<a id="source-page-62-part-1"></a>

## Source page 62

```text

```

