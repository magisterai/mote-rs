# Internationalized Resource Identifiers (IRIs) — part 2

Source: [Internationalized Resource Identifiers (IRIs)](https://www.rfc-editor.org/rfc/rfc3987.txt) · RFC 3987.

[Document index](../README.md) · [Topic guide](../../../../README.md) · [Publisher](https://www.rfc-editor.org/rfc/rfc3987.txt).

Verbatim text slices in fixed-width blocks preserve grammar, examples, and diagrams.

[Previous](part-001.md) · [Next](part-003.md)

<a id="source-page-6-part-1"></a>

## Source page 6

```text

RFC 3987         Internationalized Resource Identifiers     January 2005


   XML Notation uses a leading '&#x', a trailing ';', and the
   hexadecimal number of the character in the UCS in between.  For
   example, &#x44F; stands for CYRILLIC CAPITAL LETTER YA.  In this
   notation, an actual '&' is denoted by '&amp;'.

   Bidi Notation is used for bidirectional examples: Lowercase letters
   stand for Latin letters or other letters that are written left to
   right, whereas uppercase letters represent Arabic or Hebrew letters
   that are written right to left.

   To denote actual octets in examples (as opposed to percent-encoded
   octets), the two hex digits denoting the octet are enclosed in "<"
   and ">".  For example, the octet often denoted as 0xc9 is denoted
   here as <c9>.

   In this document, the key words "MUST", "MUST NOT", "REQUIRED",
   "SHALL", "SHALL NOT", "SHOULD", "SHOULD NOT", "RECOMMENDED",  "MAY",
   and "OPTIONAL" are to be interpreted as described in [RFC2119].

2.  IRI Syntax

   This section defines the syntax of Internationalized Resource
   Identifiers (IRIs).

   As with URIs, an IRI is defined as a sequence of characters, not as a
   sequence of octets.  This definition accommodates the fact that IRIs
   may be written on paper or read over the radio as well as stored or
   transmitted digitally.  The same IRI may be represented as different
   sequences of octets in different protocols or documents if these
   protocols or documents use different character encodings (and/or
   transfer encodings).  Using the same character encoding as the
   containing protocol or document ensures that the characters in the
   IRI can be handled (e.g., searched, converted, displayed) in the same
   way as the rest of the protocol or document.

2.1.  Summary of IRI Syntax

   IRIs are defined similarly to URIs in [RFC3986], but the class of
   unreserved characters is extended by adding the characters of the UCS
   (Universal Character Set, [ISO10646]) beyond U+007F, subject to the
   limitations given in the syntax rules below and in section 6.1.

   Otherwise, the syntax and use of components and reserved characters
   is the same as that in [RFC3986].  All the operations defined in
   [RFC3986], such as the resolution of relative references, can be
   applied to IRIs by IRI-processing software in exactly the same way as
   they are for URIs by URI-processing software.




Duerst & Suignard           Standards Track                     [Page 6]
```

<a id="source-page-7-part-1"></a>

## Source page 7

```text

RFC 3987         Internationalized Resource Identifiers     January 2005


   Characters outside the US-ASCII repertoire are not reserved and
   therefore MUST NOT be used for syntactical purposes, such as to
   delimit components in newly defined schemes.  For example, U+00A2,
   CENT SIGN, is not allowed as a delimiter in IRIs, because it is in
   the 'iunreserved' category. This is similar to the fact that it is
   not possible to use '-' as a delimiter in URIs, because it is in the
   'unreserved' category.

2.2.  ABNF for IRI References and IRIs

   Although it might be possible to define IRI references and IRIs
   merely by their transformation to URI references and URIs, they can
   also be accepted and processed directly.  Therefore, an ABNF
   definition for IRI references (which are the most general concept and
   the start of the grammar) and IRIs is given here.  The syntax of this
   ABNF is described in [RFC2234].  Character numbers are taken from the
   UCS, without implying any actual binary encoding.  Terminals in the
   ABNF are characters, not bytes.

   The following grammar closely follows the URI grammar in [RFC3986],
   except that the range of unreserved characters is expanded to include
   UCS characters, with the restriction that private UCS characters can
   occur only in query parts.  The grammar is split into two parts:
   Rules that differ from [RFC3986] because of the above-mentioned
   expansion, and rules that are the same as those in [RFC3986].  For
   rules that are different than those in [RFC3986], the names of the
   non-terminals have been changed as follows.  If the non-terminal
   contains 'URI', this has been changed to 'IRI'.  Otherwise, an 'i'
   has been prefixed.

   The following rules are different from those in [RFC3986]:

   IRI            = scheme ":" ihier-part [ "?" iquery ]
                         [ "#" ifragment ]

   ihier-part     = "//" iauthority ipath-abempty
                  / ipath-absolute
                  / ipath-rootless
                  / ipath-empty

   IRI-reference  = IRI / irelative-ref

   absolute-IRI   = scheme ":" ihier-part [ "?" iquery ]

   irelative-ref  = irelative-part [ "?" iquery ] [ "#" ifragment ]

   irelative-part = "//" iauthority ipath-abempty
                       / ipath-absolute



Duerst & Suignard           Standards Track                     [Page 7]
```

<a id="source-page-8-part-1"></a>

## Source page 8

```text

RFC 3987         Internationalized Resource Identifiers     January 2005


                  / ipath-noscheme
                  / ipath-empty

   iauthority     = [ iuserinfo "@" ] ihost [ ":" port ]
   iuserinfo      = *( iunreserved / pct-encoded / sub-delims / ":" )
   ihost          = IP-literal / IPv4address / ireg-name

   ireg-name      = *( iunreserved / pct-encoded / sub-delims )

   ipath          = ipath-abempty   ; begins with "/" or is empty
                  / ipath-absolute  ; begins with "/" but not "//"
                  / ipath-noscheme  ; begins with a non-colon segment
                  / ipath-rootless  ; begins with a segment
                  / ipath-empty     ; zero characters

   ipath-abempty  = *( "/" isegment )
   ipath-absolute = "/" [ isegment-nz *( "/" isegment ) ]
   ipath-noscheme = isegment-nz-nc *( "/" isegment )
   ipath-rootless = isegment-nz *( "/" isegment )
   ipath-empty    = 0<ipchar>

   isegment       = *ipchar
   isegment-nz    = 1*ipchar
   isegment-nz-nc = 1*( iunreserved / pct-encoded / sub-delims
                        / "@" )
                  ; non-zero-length segment without any colon ":"

   ipchar         = iunreserved / pct-encoded / sub-delims / ":"
                  / "@"

   iquery         = *( ipchar / iprivate / "/" / "?" )

   ifragment      = *( ipchar / "/" / "?" )

   iunreserved    = ALPHA / DIGIT / "-" / "." / "_" / "~" / ucschar

   ucschar        = %xA0-D7FF / %xF900-FDCF / %xFDF0-FFEF
                  / %x10000-1FFFD / %x20000-2FFFD / %x30000-3FFFD
                  / %x40000-4FFFD / %x50000-5FFFD / %x60000-6FFFD
                  / %x70000-7FFFD / %x80000-8FFFD / %x90000-9FFFD
                  / %xA0000-AFFFD / %xB0000-BFFFD / %xC0000-CFFFD
                  / %xD0000-DFFFD / %xE1000-EFFFD

   iprivate       = %xE000-F8FF / %xF0000-FFFFD / %x100000-10FFFD

   Some productions are ambiguous.  The "first-match-wins" (a.k.a.
   "greedy") algorithm applies.  For details, see [RFC3986].




Duerst & Suignard           Standards Track                     [Page 8]
```

<a id="source-page-9-part-1"></a>

## Source page 9

```text

RFC 3987         Internationalized Resource Identifiers     January 2005


   The following rules are the same as those in [RFC3986]:

   scheme         = ALPHA *( ALPHA / DIGIT / "+" / "-" / "." )

   port           = *DIGIT

   IP-literal     = "[" ( IPv6address / IPvFuture  ) "]"

   IPvFuture      = "v" 1*HEXDIG "." 1*( unreserved / sub-delims / ":" )

   IPv6address    =                            6( h16 ":" ) ls32
                  /                       "::" 5( h16 ":" ) ls32
                  / [               h16 ] "::" 4( h16 ":" ) ls32
                  / [ *1( h16 ":" ) h16 ] "::" 3( h16 ":" ) ls32
                  / [ *2( h16 ":" ) h16 ] "::" 2( h16 ":" ) ls32
                  / [ *3( h16 ":" ) h16 ] "::"    h16 ":"   ls32
                  / [ *4( h16 ":" ) h16 ] "::"              ls32
                  / [ *5( h16 ":" ) h16 ] "::"              h16
                  / [ *6( h16 ":" ) h16 ] "::"

   h16            = 1*4HEXDIG
   ls32           = ( h16 ":" h16 ) / IPv4address

   IPv4address    = dec-octet "." dec-octet "." dec-octet "." dec-octet

   dec-octet      = DIGIT                 ; 0-9
                  / %x31-39 DIGIT         ; 10-99
                  / "1" 2DIGIT            ; 100-199
                  / "2" %x30-34 DIGIT     ; 200-249
                  / "25" %x30-35          ; 250-255

   pct-encoded    = "%" HEXDIG HEXDIG

   unreserved     = ALPHA / DIGIT / "-" / "." / "_" / "~"
   reserved       = gen-delims / sub-delims
   gen-delims     = ":" / "/" / "?" / "#" / "[" / "]" / "@"
   sub-delims     = "!" / "$" / "&" / "'" / "(" / ")"
                  / "*" / "+" / "," / ";" / "="

   This syntax does not support IPv6 scoped addressing zone identifiers.











Duerst & Suignard           Standards Track                     [Page 9]
```

<a id="source-page-10-part-1"></a>

## Source page 10

```text

RFC 3987         Internationalized Resource Identifiers     January 2005


3.  Relationship between IRIs and URIs

   IRIs are meant to replace URIs in identifying resources for
   protocols, formats, and software components that use a UCS-based
   character repertoire.  These protocols and components may never need
   to use URIs directly, especially when the resource identifier is used
   simply for identification purposes.  However, when the resource
   identifier is used for resource retrieval, it is in many cases
   necessary to determine the associated URI, because currently most
   retrieval mechanisms are only defined for URIs.  In this case, IRIs
   can serve as presentation elements for URI protocol elements.  An
   example would be an address bar in a Web user agent.  (Additional
   rationale is given in section 3.1.)

3.1.  Mapping of IRIs to URIs

   This section defines how to map an IRI to a URI.  Everything in this
   section also applies to IRI references and URI references, as well as
   to components thereof (for example, fragment identifiers).

   This mapping has two purposes:

   Syntaxical. Many URI schemes and components define additional
      syntactical restrictions not captured in section 2.2.
      Scheme-specific restrictions are applied to IRIs by converting
      IRIs to URIs and checking the URIs against the scheme-specific
      restrictions.

   Interpretational. URIs identify resources in various ways.  IRIs also
      identify resources.  When the IRI is used solely for
      identification purposes, it is not necessary to map the IRI to a
      URI (see section 5).  However, when an IRI is used for resource
      retrieval, the resource that the IRI locates is the same as the
      one located by the URI obtained after converting the IRI according
      to the procedure defined here.  This means that there is no need
      to define resolution separately on the IRI level.

   Applications MUST map IRIs to URIs by using the following two steps.

   Step 1.  Generate a UCS character sequence from the original IRI
            format.  This step has the following three variants,
            depending on the form of the input:

            a. If the IRI is written on paper, read aloud, or otherwise
               represented as a sequence of characters independent of
               any character encoding, represent the IRI as a sequence
               of characters from the UCS normalized according to
               Normalization Form C (NFC, [UTR15]).



Duerst & Suignard           Standards Track                    [Page 10]
```

<a id="source-page-11-part-1"></a>

## Source page 11

```text

RFC 3987         Internationalized Resource Identifiers     January 2005


            b. If the IRI is in some digital representation (e.g., an
               octet stream) in some known non-Unicode character
               encoding, convert the IRI to a sequence of characters
               from the UCS normalized according to NFC.

            c. If the IRI is in a Unicode-based character encoding (for
               example, UTF-8 or UTF-16), do not normalize (see section
               5.3.2.2 for details).  Apply step 2 directly to the
               encoded Unicode character sequence.

   Step 2.  For each character in 'ucschar' or 'iprivate', apply steps
            2.1 through 2.3 below.

       2.1.  Convert the character to a sequence of one or more octets
             using UTF-8 [RFC3629].

       2.2.  Convert each octet to %HH, where HH is the hexadecimal
             notation of the octet value.  Note that this is identical
             to the percent-encoding mechanism in section 2.1 of
             [RFC3986].  To reduce variability, the hexadecimal notation
             SHOULD use uppercase letters.

       2.3.  Replace the original character with the resulting character
             sequence (i.e., a sequence of %HH triplets).

   The above mapping from IRIs to URIs produces URIs fully conforming to
   [RFC3986].  The mapping is also an identity transformation for URIs
   and is idempotent;  applying the mapping a second time will not
   change anything.  Every URI is by definition an IRI.

   Systems accepting IRIs MAY convert the ireg-name component of an IRI
   as follows (before step 2 above) for schemes known to use domain
   names in ireg-name, if the scheme definition does not allow
   percent-encoding for ireg-name:

   Replace the ireg-name part of the IRI by the part converted using the
   ToASCII operation specified in section 4.1 of [RFC3490] on each
   dot-separated label, and by using U+002E (FULL STOP) as a label
   separator, with the flag UseSTD3ASCIIRules set to TRUE, and with the
   flag AllowUnassigned set to FALSE for creating IRIs and set to TRUE
   otherwise.










Duerst & Suignard           Standards Track                    [Page 11]
```

