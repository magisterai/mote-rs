# Uniform Resource Identifier (URI): Generic Syntax — part 3

Source: [Uniform Resource Identifier (URI): Generic Syntax](https://www.rfc-editor.org/rfc/rfc3986.txt) · RFC 3986.

[Document index](../README.md) · [Topic guide](../../../../README.md) · [Publisher](https://www.rfc-editor.org/rfc/rfc3986.txt).

Verbatim text slices in fixed-width blocks preserve grammar, examples, and diagrams.

[Previous](part-002.md) · [Next](part-004.md)

<a id="source-page-11-part-1"></a>

## Source page 11

```text

RFC 3986                   URI Generic Syntax               January 2005


   presented in Section 5, defines how such a reference is transformed
   to the target URI.  As relative references can only be used within
   the context of a hierarchical URI, designers of new URI schemes
   should use a syntax consistent with the generic syntax's hierarchical
   components unless there are compelling reasons to forbid relative
   referencing within that scheme.

      NOTE: Previous specifications used the terms "partial URI" and
      "relative URI" to denote a relative reference to a URI.  As some
      readers misunderstood those terms to mean that relative URIs are a
      subset of URIs rather than a method of referencing URIs, this
      specification simply refers to them as relative references.

   All URI references are parsed by generic syntax parsers when used.
   However, because hierarchical processing has no effect on an absolute
   URI used in a reference unless it contains one or more dot-segments
   (complete path segments of "." or "..", as described in Section 3.3),
   URI scheme specifications can define opaque identifiers by
   disallowing use of slash characters, question mark characters, and
   the URIs "scheme:." and "scheme:..".

1.3.  Syntax Notation

   This specification uses the Augmented Backus-Naur Form (ABNF)
   notation of [RFC2234], including the following core ABNF syntax rules
   defined by that specification: ALPHA (letters), CR (carriage return),
   DIGIT (decimal digits), DQUOTE (double quote), HEXDIG (hexadecimal
   digits), LF (line feed), and SP (space).  The complete URI syntax is
   collected in Appendix A.

2.  Characters

   The URI syntax provides a method of encoding data, presumably for the
   sake of identifying a resource, as a sequence of characters.  The URI
   characters are, in turn, frequently encoded as octets for transport
   or presentation.  This specification does not mandate any particular
   character encoding for mapping between URI characters and the octets
   used to store or transmit those characters.  When a URI appears in a
   protocol element, the character encoding is defined by that protocol;
   without such a definition, a URI is assumed to be in the same
   character encoding as the surrounding text.

   The ABNF notation defines its terminal values to be non-negative
   integers (codepoints) based on the US-ASCII coded character set
   [ASCII].  Because a URI is a sequence of characters, we must invert
   that relation in order to understand the URI syntax.  Therefore, the





Berners-Lee, et al.         Standards Track                    [Page 11]
```

<a id="source-page-12-part-1"></a>

## Source page 12

```text

RFC 3986                   URI Generic Syntax               January 2005


   integer values used by the ABNF must be mapped back to their
   corresponding characters via US-ASCII in order to complete the syntax
   rules.

   A URI is composed from a limited set of characters consisting of
   digits, letters, and a few graphic symbols.  A reserved subset of
   those characters may be used to delimit syntax components within a
   URI while the remaining characters, including both the unreserved set
   and those reserved characters not acting as delimiters, define each
   component's identifying data.

2.1.  Percent-Encoding

   A percent-encoding mechanism is used to represent a data octet in a
   component when that octet's corresponding character is outside the
   allowed set or is being used as a delimiter of, or within, the
   component.  A percent-encoded octet is encoded as a character
   triplet, consisting of the percent character "%" followed by the two
   hexadecimal digits representing that octet's numeric value.  For
   example, "%20" is the percent-encoding for the binary octet
   "00100000" (ABNF: %x20), which in US-ASCII corresponds to the space
   character (SP).  Section 2.4 describes when percent-encoding and
   decoding is applied.

      pct-encoded = "%" HEXDIG HEXDIG

   The uppercase hexadecimal digits 'A' through 'F' are equivalent to
   the lowercase digits 'a' through 'f', respectively.  If two URIs
   differ only in the case of hexadecimal digits used in percent-encoded
   octets, they are equivalent.  For consistency, URI producers and
   normalizers should use uppercase hexadecimal digits for all percent-
   encodings.

2.2.  Reserved Characters

   URIs include components and subcomponents that are delimited by
   characters in the "reserved" set.  These characters are called
   "reserved" because they may (or may not) be defined as delimiters by
   the generic syntax, by each scheme-specific syntax, or by the
   implementation-specific syntax of a URI's dereferencing algorithm.
   If data for a URI component would conflict with a reserved
   character's purpose as a delimiter, then the conflicting data must be
   percent-encoded before the URI is formed.








Berners-Lee, et al.         Standards Track                    [Page 12]
```

<a id="source-page-13-part-1"></a>

## Source page 13

```text

RFC 3986                   URI Generic Syntax               January 2005


      reserved    = gen-delims / sub-delims

      gen-delims  = ":" / "/" / "?" / "#" / "[" / "]" / "@"

      sub-delims  = "!" / "$" / "&" / "'" / "(" / ")"
                  / "*" / "+" / "," / ";" / "="

   The purpose of reserved characters is to provide a set of delimiting
   characters that are distinguishable from other data within a URI.
   URIs that differ in the replacement of a reserved character with its
   corresponding percent-encoded octet are not equivalent.  Percent-
   encoding a reserved character, or decoding a percent-encoded octet
   that corresponds to a reserved character, will change how the URI is
   interpreted by most applications.  Thus, characters in the reserved
   set are protected from normalization and are therefore safe to be
   used by scheme-specific and producer-specific algorithms for
   delimiting data subcomponents within a URI.

   A subset of the reserved characters (gen-delims) is used as
   delimiters of the generic URI components described in Section 3.  A
   component's ABNF syntax rule will not use the reserved or gen-delims
   rule names directly; instead, each syntax rule lists the characters
   allowed within that component (i.e., not delimiting it), and any of
   those characters that are also in the reserved set are "reserved" for
   use as subcomponent delimiters within the component.  Only the most
   common subcomponents are defined by this specification; other
   subcomponents may be defined by a URI scheme's specification, or by
   the implementation-specific syntax of a URI's dereferencing
   algorithm, provided that such subcomponents are delimited by
   characters in the reserved set allowed within that component.

   URI producing applications should percent-encode data octets that
   correspond to characters in the reserved set unless these characters
   are specifically allowed by the URI scheme to represent data in that
   component.  If a reserved character is found in a URI component and
   no delimiting role is known for that character, then it must be
   interpreted as representing the data octet corresponding to that
   character's encoding in US-ASCII.

2.3.  Unreserved Characters

   Characters that are allowed in a URI but do not have a reserved
   purpose are called unreserved.  These include uppercase and lowercase
   letters, decimal digits, hyphen, period, underscore, and tilde.

      unreserved  = ALPHA / DIGIT / "-" / "." / "_" / "~"





Berners-Lee, et al.         Standards Track                    [Page 13]
```

<a id="source-page-14-part-1"></a>

## Source page 14

```text

RFC 3986                   URI Generic Syntax               January 2005


   URIs that differ in the replacement of an unreserved character with
   its corresponding percent-encoded US-ASCII octet are equivalent: they
   identify the same resource.  However, URI comparison implementations
   do not always perform normalization prior to comparison (see Section
   6).  For consistency, percent-encoded octets in the ranges of ALPHA
   (%41-%5A and %61-%7A), DIGIT (%30-%39), hyphen (%2D), period (%2E),
   underscore (%5F), or tilde (%7E) should not be created by URI
   producers and, when found in a URI, should be decoded to their
   corresponding unreserved characters by URI normalizers.

2.4.  When to Encode or Decode

   Under normal circumstances, the only time when octets within a URI
   are percent-encoded is during the process of producing the URI from
   its component parts.  This is when an implementation determines which
   of the reserved characters are to be used as subcomponent delimiters
   and which can be safely used as data.  Once produced, a URI is always
   in its percent-encoded form.

   When a URI is dereferenced, the components and subcomponents
   significant to the scheme-specific dereferencing process (if any)
   must be parsed and separated before the percent-encoded octets within
   those components can be safely decoded, as otherwise the data may be
   mistaken for component delimiters.  The only exception is for
   percent-encoded octets corresponding to characters in the unreserved
   set, which can be decoded at any time.  For example, the octet
   corresponding to the tilde ("~") character is often encoded as "%7E"
   by older URI processing implementations; the "%7E" can be replaced by
   "~" without changing its interpretation.

   Because the percent ("%") character serves as the indicator for
   percent-encoded octets, it must be percent-encoded as "%25" for that
   octet to be used as data within a URI.  Implementations must not
   percent-encode or decode the same string more than once, as decoding
   an already decoded string might lead to misinterpreting a percent
   data octet as the beginning of a percent-encoding, or vice versa in
   the case of percent-encoding an already percent-encoded string.

2.5.  Identifying Data

   URI characters provide identifying data for each of the URI
   components, serving as an external interface for identification
   between systems.  Although the presence and nature of the URI
   production interface is hidden from clients that use its URIs (and is
   thus beyond the scope of the interoperability requirements defined by
   this specification), it is a frequent source of confusion and errors
   in the interpretation of URI character issues.  Implementers have to
   be aware that there are multiple character encodings involved in the



Berners-Lee, et al.         Standards Track                    [Page 14]
```

<a id="source-page-15-part-1"></a>

## Source page 15

```text

RFC 3986                   URI Generic Syntax               January 2005


   production and transmission of URIs: local name and data encoding,
   public interface encoding, URI character encoding, data format
   encoding, and protocol encoding.

   Local names, such as file system names, are stored with a local
   character encoding.  URI producing applications (e.g., origin
   servers) will typically use the local encoding as the basis for
   producing meaningful names.  The URI producer will transform the
   local encoding to one that is suitable for a public interface and
   then transform the public interface encoding into the restricted set
   of URI characters (reserved, unreserved, and percent-encodings).
   Those characters are, in turn, encoded as octets to be used as a
   reference within a data format (e.g., a document charset), and such
   data formats are often subsequently encoded for transmission over
   Internet protocols.

   For most systems, an unreserved character appearing within a URI
   component is interpreted as representing the data octet corresponding
   to that character's encoding in US-ASCII.  Consumers of URIs assume
   that the letter "X" corresponds to the octet "01011000", and even
   when that assumption is incorrect, there is no harm in making it.  A
   system that internally provides identifiers in the form of a
   different character encoding, such as EBCDIC, will generally perform
   character translation of textual identifiers to UTF-8 [STD63] (or
   some other superset of the US-ASCII character encoding) at an
   internal interface, thereby providing more meaningful identifiers
   than those resulting from simply percent-encoding the original
   octets.

   For example, consider an information service that provides data,
   stored locally using an EBCDIC-based file system, to clients on the
   Internet through an HTTP server.  When an author creates a file with
   the name "Laguna Beach" on that file system, the "http" URI
   corresponding to that resource is expected to contain the meaningful
   string "Laguna%20Beach".  If, however, that server produces URIs by
   using an overly simplistic raw octet mapping, then the result would
   be a URI containing "%D3%81%87%A4%95%81@%C2%85%81%83%88".  An
   internal transcoding interface fixes this problem by transcoding the
   local name to a superset of US-ASCII prior to producing the URI.
   Naturally, proper interpretation of an incoming URI on such an
   interface requires that percent-encoded octets be decoded (e.g.,
   "%20" to SP) before the reverse transcoding is applied to obtain the
   local name.

   In some cases, the internal interface between a URI component and the
   identifying data that it has been crafted to represent is much less
   direct than a character encoding translation.  For example, portions
   of a URI might reflect a query on non-ASCII data, or numeric



Berners-Lee, et al.         Standards Track                    [Page 15]
```

