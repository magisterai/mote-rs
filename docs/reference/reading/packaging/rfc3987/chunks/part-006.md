# Internationalized Resource Identifiers (IRIs) — part 6

Source: [Internationalized Resource Identifiers (IRIs)](https://www.rfc-editor.org/rfc/rfc3987.txt) · RFC 3987.

[Document index](../README.md) · [Topic guide](../../../../README.md) · [Publisher](https://www.rfc-editor.org/rfc/rfc3987.txt).

Verbatim text slices in fixed-width blocks preserve grammar, examples, and diagrams.

[Previous](part-005.md) · [Next](part-007.md)

<a id="source-page-27-part-1"></a>

## Source page 27

```text

RFC 3987         Internationalized Resource Identifiers     January 2005


   application or used further in some other way, its original form MUST
   be preserved.  The conversion described here should be performed only
   for local comparison.

5.3.2.4.  Path Segment Normalization

   The complete path segments "." and ".." are intended only for use
   within relative references (section 4.1 of [RFC3986]) and are removed
   as part of the reference resolution process (section 5.2 of
   [RFC3986]).  However, some implementations may incorrectly assume
   that reference resolution is not necessary when the reference is
   already an IRI, and thus fail to remove dot-segments when they occur
   in non-relative paths.  IRI normalizers should remove dot-segments by
   applying the remove_dot_segments algorithm to the path, as described
   in section 5.2.4 of [RFC3986].

5.3.3.  Scheme-Based Normalization

   The syntax and semantics of IRIs vary from scheme to scheme, as
   described by the defining specification for each scheme.
   Implementations may use scheme-specific rules, at further processing
   cost, to reduce the probability of false negatives.  For example,
   because the "http" scheme makes use of an authority component, has a
   default port of "80", and defines an empty path to be equivalent to
   "/", the following four IRIs are equivalent:

      http://example.com
      http://example.com/
      http://example.com:/
      http://example.com:80/

   In general, an IRI that uses the generic syntax for authority with an
   empty path should be normalized to a path of "/".  Likewise, an
   explicit ":port", for which the port is empty or the default for the
   scheme, is equivalent to one where the port and its ":" delimiter are
   elided and thus should be removed by scheme-based normalization.  For
   example, the second IRI above is the normal form for the "http"
   scheme.

   Another case where normalization varies by scheme is in the handling
   of an empty authority component or empty host subcomponent.  For many
   scheme specifications, an empty authority or host is considered an
   error; for others, it is considered equivalent to "localhost" or the
   end-user's host.  When a scheme defines a default for authority and
   an IRI reference to that default is desired, the reference should be
   normalized to an empty authority for the sake of uniformity, brevity,





Duerst & Suignard           Standards Track                    [Page 27]
```

<a id="source-page-28-part-1"></a>

## Source page 28

```text

RFC 3987         Internationalized Resource Identifiers     January 2005


   and internationalization.  If, however, either the userinfo or port
   subcomponents are non-empty, then the host should be given explicitly
   even if it matches the default.

   Normalization should not remove delimiters when their associated
   component is empty unless it is licensed to do so by the scheme
   specification.  For example, the IRI "http://example.com/?" cannot be
   assumed to be equivalent to any of the examples above.  Likewise, the
   presence or absence of delimiters within a userinfo subcomponent is
   usually significant to its interpretation.  The fragment component is
   not subject to any scheme-based normalization; thus, two IRIs that
   differ only by the suffix "#" are considered different regardless of
   the scheme.

   Some IRI schemes may allow the usage of Internationalized Domain
   Names (IDN) [RFC3490] either in their ireg-name part or elsewhere.
   When in use in IRIs, those names SHOULD be validated by using the
   ToASCII operation defined in [RFC3490], with the flags
   "UseSTD3ASCIIRules" and "AllowUnassigned".  An IRI containing an
   invalid IDN cannot successfully be resolved.  Validated IDN
   components of IRIs SHOULD be character normalized by using the
   Nameprep process [RFC3491]; however, for legibility purposes, they
   SHOULD NOT be converted into ASCII Compatible Encoding (ACE).

   Scheme-based normalization may also consider IDN components and their
   conversions to punycode as equivalent.  As an example,
   "http://r&#xE9;sum&#xE9;.example.org" may be considered equivalent to
   "http://xn--rsum-bpad.example.org".

   Other scheme-specific normalizations are possible.

5.3.4.  Protocol-Based Normalization

   Substantial effort to reduce the incidence of false negatives is
   often cost-effective for web spiders. Consequently, they implement
   even more aggressive techniques in IRI comparison.  For example, if
   they observe that an IRI such as

      http://example.com/data

   redirects to an IRI differing only in the trailing slash

      http://example.com/data/

   they will likely regard the two as equivalent in the future.  This
   kind of technique is only appropriate when equivalence is clearly
   indicated by both the result of accessing the resources and the




Duerst & Suignard           Standards Track                    [Page 28]
```

<a id="source-page-29-part-1"></a>

## Source page 29

```text

RFC 3987         Internationalized Resource Identifiers     January 2005


   common conventions of their scheme's dereference algorithm (in this
   case, use of redirection by HTTP origin servers to avoid problems
   with relative references).

6.  Use of IRIs

6.1.  Limitations on UCS Characters Allowed in IRIs

   This section discusses limitations on characters and character
   sequences usable for IRIs beyond those given in section 2.2 and
   section 4.1.  The considerations in this section are relevant when
   IRIs are created and when URIs are converted to IRIs.

   a.  The repertoire of characters allowed in each IRI component is
       limited by the definition of that component.  For example, the
       definition of the scheme component does not allow characters
       beyond US-ASCII.

       (Note: In accordance with URI practice, generic IRI software
       cannot and should not check for such limitations.)

   b.  The UCS contains many areas of characters for which there are
       strong visual look-alikes.  Because of the likelihood of
       transcription errors, these also should be avoided.  This
       includes the full-width equivalents of Latin characters,
       half-width Katakana characters for Japanese, and many others.  It
       also includes many look-alikes of "space", "delims", and
       "unwise", characters excluded in [RFC3491].

   Additional information is available from [UNIXML].  [UNIXML] is
   written in the context of running text rather than in that of
   identifiers.  Nevertheless, it discusses many of the categories of
   characters not appropriate for IRIs.

6.2.  Software Interfaces and Protocols

   Although an IRI is defined as a sequence of characters, software
   interfaces for URIs typically function on sequences of octets or
   other kinds of code units.  Thus, software interfaces and protocols
   MUST define which character encoding is used.

   Intermediate software interfaces between IRI-capable components and
   URI-only components MUST map the IRIs per section 3.1, when
   transferring from IRI-capable to URI-only components.  This mapping
   SHOULD be applied as late as possible.  It SHOULD NOT be applied
   between components that are known to be able to handle IRIs.





Duerst & Suignard           Standards Track                    [Page 29]
```

<a id="source-page-30-part-1"></a>

## Source page 30

```text

RFC 3987         Internationalized Resource Identifiers     January 2005


6.3.  Format of URIs and IRIs in Documents and Protocols

   Document formats that transport URIs may have to be upgraded to allow
   the transport of IRIs.  In cases where the document as a whole has a
   native character encoding, IRIs MUST also be encoded in this
   character encoding and converted accordingly by a parser or
   interpreter.  IRI characters not expressible in the native character
   encoding SHOULD be escaped by using the escaping conventions of the
   document format if such conventions are available. Alternatively,
   they MAY be percent-encoded according to section 3.1. For example, in
   HTML or XML, numeric character references SHOULD be used.  If a
   document as a whole has a native character encoding and that
   character encoding is not UTF-8, then IRIs MUST NOT be placed into
   the document in the UTF-8 character encoding.

   Note: Some formats already accommodate IRIs, although they use
   different terminology.  HTML 4.0 [HTML4] defines the conversion from
   IRIs to URIs as error-avoiding behavior.  XML 1.0 [XML1], XLink
   [XLink], XML Schema [XMLSchema], and specifications based upon them
   allow IRIs.  Also, it is expected that all relevant new W3C formats
   and protocols will be required to handle IRIs [CharMod].

6.4.  Use of UTF-8 for Encoding Original Characters

   This section discusses details and gives examples for point c) in
   section 1.2.  To be able to use IRIs, the URI corresponding to the
   IRI in question has to encode original characters into octets by
   using UTF-8.  This can be specified for all URIs of a URI scheme or
   can apply to individual URIs for schemes that do not specify how to
   encode original characters.  It can apply to the whole URI, or only
   to some part.  For background information on encoding characters into
   URIs, see also section 2.5 of [RFC3986].

   For new URI schemes, using UTF-8 is recommended in [RFC2718].
   Examples where UTF-8 is already used are the URN syntax [RFC2141],
   IMAP URLs [RFC2192], and POP URLs [RFC2384].  On the other hand,
   because the HTTP URL scheme does not specify how to encode original
   characters, only some HTTP URLs can have corresponding but different
   IRIs.

   For example, for a document with a URI of
   "http://www.example.org/r%C3%A9sum%C3%A9.html", it is possible to
   construct a corresponding IRI (in XML notation, see, section 1.4):
   "http://www.example.org/r&#xE9;sum&#xE9;.html" ("&#xE9"; stands for
   the e-acute character, and "%C3%A9" is the UTF-8 encoded and
   percent-encoded representation of that character).  On the other
   hand, for a document with a URI of




Duerst & Suignard           Standards Track                    [Page 30]
```

<a id="source-page-31-part-1"></a>

## Source page 31

```text

RFC 3987         Internationalized Resource Identifiers     January 2005


   "http://www.example.org/r%E9sum%E9.html", the percent-encoding octets
   cannot be converted to actual characters in an IRI, as the
   percent-encoding is not based on UTF-8.

   This means that for most URI schemes, there is no need to upgrade
   their scheme definition in order for them to work with IRIs.  The
   main case where upgrading makes sense is when a scheme definition, or
   a particular component of a scheme, is strictly limited to the use of
   US-ASCII characters with no provision to include non-ASCII
   characters/octets via percent-encoding, or if a scheme definition
   currently uses highly scheme-specific provisions for the encoding of
   non-ASCII characters.  An example of this is the mailto: scheme
   [RFC2368].

   This specification does not upgrade any scheme specifications in any
   way; this has to be done separately.  Also, note that there is no
   such thing as an "IRI scheme"; all IRIs use URI schemes, and all URI
   schemes can be used with IRIs, even though in some cases only by
   using URIs directly as IRIs, without any conversion.

   URI schemes can impose restrictions on the syntax of scheme-specific
   URIs; i.e., URIs that are admissible under the generic URI syntax
   [RFC3986] may not be admissible due to narrower syntactic constraints
   imposed by a URI scheme specification.  URI scheme definitions cannot
   broaden the syntactic restrictions of the generic URI syntax;
   otherwise, it would be possible to generate URIs that satisfied the
   scheme-specific syntactic constraints without satisfying the
   syntactic constraints of the generic URI syntax.  However, additional
   syntactic constraints imposed by URI scheme specifications are
   applicable to IRI, as the corresponding URI resulting from the
   mapping defined in section 3.1 MUST be a valid URI under the
   syntactic restrictions of generic URI syntax and any narrower
   restrictions imposed by the corresponding URI scheme specification.

   The requirement for the use of UTF-8 applies to all parts of a URI
   (with the potential exception of the ireg-name part; see section
   3.1).  However, it is possible that the capability of IRIs to
   represent a wide range of characters directly is used just in some
   parts of the IRI (or IRI reference).  The other parts of the IRI may
   only contain US-ASCII characters, or they may not be based on UTF-8.
   They may be based on another character encoding, or they may directly
   encode raw binary data (see also [RFC2397]).

   For example, it is possible to have a URI reference of
   "http://www.example.org/r%E9sum%E9.xml#r%C3%A9sum%C3%A9", where the
   document name is encoded in iso-8859-1 based on server settings, but
   where the fragment identifier is encoded in UTF-8 according to




Duerst & Suignard           Standards Track                    [Page 31]
```

