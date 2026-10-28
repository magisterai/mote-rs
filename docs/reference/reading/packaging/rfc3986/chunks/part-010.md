# Uniform Resource Identifier (URI): Generic Syntax — part 10

Source: [Uniform Resource Identifier (URI): Generic Syntax](https://www.rfc-editor.org/rfc/rfc3986.txt) · RFC 3986.

[Document index](../README.md) · [Topic guide](../../../../README.md) · [Publisher](https://www.rfc-editor.org/rfc/rfc3986.txt).

Verbatim text slices in fixed-width blocks preserve grammar, examples, and diagrams.

[Previous](part-009.md) · [Next](part-011.md)

<a id="source-page-49-part-1"></a>

## Source page 49

```text

RFC 3986                   URI Generic Syntax               January 2005


Appendix A.  Collected ABNF for URI

   URI           = scheme ":" hier-part [ "?" query ] [ "#" fragment ]

   hier-part     = "//" authority path-abempty
                 / path-absolute
                 / path-rootless
                 / path-empty

   URI-reference = URI / relative-ref

   absolute-URI  = scheme ":" hier-part [ "?" query ]

   relative-ref  = relative-part [ "?" query ] [ "#" fragment ]

   relative-part = "//" authority path-abempty
                 / path-absolute
                 / path-noscheme
                 / path-empty

   scheme        = ALPHA *( ALPHA / DIGIT / "+" / "-" / "." )

   authority     = [ userinfo "@" ] host [ ":" port ]
   userinfo      = *( unreserved / pct-encoded / sub-delims / ":" )
   host          = IP-literal / IPv4address / reg-name
   port          = *DIGIT

   IP-literal    = "[" ( IPv6address / IPvFuture  ) "]"

   IPvFuture     = "v" 1*HEXDIG "." 1*( unreserved / sub-delims / ":" )

   IPv6address   =                            6( h16 ":" ) ls32
                 /                       "::" 5( h16 ":" ) ls32
                 / [               h16 ] "::" 4( h16 ":" ) ls32
                 / [ *1( h16 ":" ) h16 ] "::" 3( h16 ":" ) ls32
                 / [ *2( h16 ":" ) h16 ] "::" 2( h16 ":" ) ls32
                 / [ *3( h16 ":" ) h16 ] "::"    h16 ":"   ls32
                 / [ *4( h16 ":" ) h16 ] "::"              ls32
                 / [ *5( h16 ":" ) h16 ] "::"              h16
                 / [ *6( h16 ":" ) h16 ] "::"

   h16           = 1*4HEXDIG
   ls32          = ( h16 ":" h16 ) / IPv4address
   IPv4address   = dec-octet "." dec-octet "." dec-octet "." dec-octet







Berners-Lee, et al.         Standards Track                    [Page 49]
```

<a id="source-page-50-part-1"></a>

## Source page 50

```text

RFC 3986                   URI Generic Syntax               January 2005


   dec-octet     = DIGIT                 ; 0-9
                 / %x31-39 DIGIT         ; 10-99
                 / "1" 2DIGIT            ; 100-199
                 / "2" %x30-34 DIGIT     ; 200-249
                 / "25" %x30-35          ; 250-255

   reg-name      = *( unreserved / pct-encoded / sub-delims )

   path          = path-abempty    ; begins with "/" or is empty
                 / path-absolute   ; begins with "/" but not "//"
                 / path-noscheme   ; begins with a non-colon segment
                 / path-rootless   ; begins with a segment
                 / path-empty      ; zero characters

   path-abempty  = *( "/" segment )
   path-absolute = "/" [ segment-nz *( "/" segment ) ]
   path-noscheme = segment-nz-nc *( "/" segment )
   path-rootless = segment-nz *( "/" segment )
   path-empty    = 0<pchar>

   segment       = *pchar
   segment-nz    = 1*pchar
   segment-nz-nc = 1*( unreserved / pct-encoded / sub-delims / "@" )
                 ; non-zero-length segment without any colon ":"

   pchar         = unreserved / pct-encoded / sub-delims / ":" / "@"

   query         = *( pchar / "/" / "?" )

   fragment      = *( pchar / "/" / "?" )

   pct-encoded   = "%" HEXDIG HEXDIG

   unreserved    = ALPHA / DIGIT / "-" / "." / "_" / "~"
   reserved      = gen-delims / sub-delims
   gen-delims    = ":" / "/" / "?" / "#" / "[" / "]" / "@"
   sub-delims    = "!" / "$" / "&" / "'" / "(" / ")"
                 / "*" / "+" / "," / ";" / "="

Appendix B.  Parsing a URI Reference with a Regular Expression

   As the "first-match-wins" algorithm is identical to the "greedy"
   disambiguation method used by POSIX regular expressions, it is
   natural and commonplace to use a regular expression for parsing the
   potential five components of a URI reference.

   The following line is the regular expression for breaking-down a
   well-formed URI reference into its components.



Berners-Lee, et al.         Standards Track                    [Page 50]
```

<a id="source-page-51-part-1"></a>

## Source page 51

```text

RFC 3986                   URI Generic Syntax               January 2005


      ^(([^:/?#]+):)?(//([^/?#]*))?([^?#]*)(\?([^#]*))?(#(.*))?
       12            3  4          5       6  7        8 9

   The numbers in the second line above are only to assist readability;
   they indicate the reference points for each subexpression (i.e., each
   paired parenthesis).  We refer to the value matched for subexpression
   <n> as $<n>.  For example, matching the above expression to

      http://www.ics.uci.edu/pub/ietf/uri/#Related

   results in the following subexpression matches:

      $1 = http:
      $2 = http
      $3 = //www.ics.uci.edu
      $4 = www.ics.uci.edu
      $5 = /pub/ietf/uri/
      $6 = <undefined>
      $7 = <undefined>
      $8 = #Related
      $9 = Related

   where <undefined> indicates that the component is not present, as is
   the case for the query component in the above example.  Therefore, we
   can determine the value of the five components as

      scheme    = $2
      authority = $4
      path      = $5
      query     = $7
      fragment  = $9

   Going in the opposite direction, we can recreate a URI reference from
   its components by using the algorithm of Section 5.3.

Appendix C.  Delimiting a URI in Context

   URIs are often transmitted through formats that do not provide a
   clear context for their interpretation.  For example, there are many
   occasions when a URI is included in plain text; examples include text
   sent in email, USENET news, and on printed paper.  In such cases, it
   is important to be able to delimit the URI from the rest of the text,
   and in particular from punctuation marks that might be mistaken for
   part of the URI.

   In practice, URIs are delimited in a variety of ways, but usually
   within double-quotes "http://example.com/", angle brackets
   <http://example.com/>, or just by using whitespace:



Berners-Lee, et al.         Standards Track                    [Page 51]
```

<a id="source-page-52-part-1"></a>

## Source page 52

```text

RFC 3986                   URI Generic Syntax               January 2005


      http://example.com/

   These wrappers do not form part of the URI.

   In some cases, extra whitespace (spaces, line-breaks, tabs, etc.) may
   have to be added to break a long URI across lines.  The whitespace
   should be ignored when the URI is extracted.

   No whitespace should be introduced after a hyphen ("-") character.
   Because some typesetters and printers may (erroneously) introduce a
   hyphen at the end of line when breaking it, the interpreter of a URI
   containing a line break immediately after a hyphen should ignore all
   whitespace around the line break and should be aware that the hyphen
   may or may not actually be part of the URI.

   Using <> angle brackets around each URI is especially recommended as
   a delimiting style for a reference that contains embedded whitespace.

   The prefix "URL:" (with or without a trailing space) was formerly
   recommended as a way to help distinguish a URI from other bracketed
   designators, though it is not commonly used in practice and is no
   longer recommended.

   For robustness, software that accepts user-typed URI should attempt
   to recognize and strip both delimiters and embedded whitespace.

   For example, the text

      Yes, Jim, I found it under "http://www.w3.org/Addressing/",
      but you can probably pick it up from <ftp://foo.example.
      com/rfc/>.  Note the warning in <http://www.ics.uci.edu/pub/
      ietf/uri/historical.html#WARNING>.

   contains the URI references

      http://www.w3.org/Addressing/
      ftp://foo.example.com/rfc/
      http://www.ics.uci.edu/pub/ietf/uri/historical.html#WARNING













Berners-Lee, et al.         Standards Track                    [Page 52]
```

<a id="source-page-53-part-1"></a>

## Source page 53

```text

RFC 3986                   URI Generic Syntax               January 2005


Appendix D.  Changes from RFC 2396

D.1.  Additions

   An ABNF rule for URI has been introduced to correspond to one common
   usage of the term: an absolute URI with optional fragment.

   IPv6 (and later) literals have been added to the list of possible
   identifiers for the host portion of an authority component, as
   described by [RFC2732], with the addition of "[" and "]" to the
   reserved set and a version flag to anticipate future versions of IP
   literals.  Square brackets are now specified as reserved within the
   authority component and are not allowed outside their use as
   delimiters for an IP literal within host.  In order to make this
   change without changing the technical definition of the path, query,
   and fragment components, those rules were redefined to directly
   specify the characters allowed.

   As [RFC2732] defers to [RFC3513] for definition of an IPv6 literal
   address, which, unfortunately, lacks an ABNF description of
   IPv6address, we created a new ABNF rule for IPv6address that matches
   the text representations defined by Section 2.2 of [RFC3513].
   Likewise, the definition of IPv4address has been improved in order to
   limit each decimal octet to the range 0-255.

   Section 6, on URI normalization and comparison, has been completely
   rewritten and extended by using input from Tim Bray and discussion
   within the W3C Technical Architecture Group.

D.2.  Modifications

   The ad-hoc BNF syntax of RFC 2396 has been replaced with the ABNF of
   [RFC2234].  This change required all rule names that formerly
   included underscore characters to be renamed with a dash instead.  In
   addition, a number of syntax rules have been eliminated or simplified
   to make the overall grammar more comprehensible.  Specifications that
   refer to the obsolete grammar rules may be understood by replacing
   those rules according to the following table:













Berners-Lee, et al.         Standards Track                    [Page 53]
```

<a id="source-page-54-part-1"></a>

## Source page 54

```text

RFC 3986                   URI Generic Syntax               January 2005


   +----------------+--------------------------------------------------+
   | obsolete rule  | translation                                      |
   +----------------+--------------------------------------------------+
   | absoluteURI    | absolute-URI                                     |
   | relativeURI    | relative-part [ "?" query ]                      |
   | hier_part      | ( "//" authority path-abempty /                  |
   |                | path-absolute ) [ "?" query ]                    |
   |                |                                                  |
   | opaque_part    | path-rootless [ "?" query ]                      |
   | net_path       | "//" authority path-abempty                      |
   | abs_path       | path-absolute                                    |
   | rel_path       | path-rootless                                    |
   | rel_segment    | segment-nz-nc                                    |
   | reg_name       | reg-name                                         |
   | server         | authority                                        |
   | hostport       | host [ ":" port ]                                |
   | hostname       | reg-name                                         |
   | path_segments  | path-abempty                                     |
   | param          | *<pchar excluding ";">                           |
   |                |                                                  |
   | uric           | unreserved / pct-encoded / ";" / "?" / ":"       |
   |                |  / "@" / "&" / "=" / "+" / "$" / "," / "/"       |
   |                |                                                  |
   | uric_no_slash  | unreserved / pct-encoded / ";" / "?" / ":"       |
   |                |  / "@" / "&" / "=" / "+" / "$" / ","             |
   |                |                                                  |
   | mark           | "-" / "_" / "." / "!" / "~" / "*" / "'"          |
   |                |  / "(" / ")"                                     |
   |                |                                                  |
   | escaped        | pct-encoded                                      |
   | hex            | HEXDIG                                           |
   | alphanum       | ALPHA / DIGIT                                    |
   +----------------+--------------------------------------------------+

   Use of the above obsolete rules for the definition of scheme-specific
   syntax is deprecated.

   Section 2, on characters, has been rewritten to explain what
   characters are reserved, when they are reserved, and why they are
   reserved, even when they are not used as delimiters by the generic
   syntax.  The mark characters that are typically unsafe to decode,
   including the exclamation mark ("!"), asterisk ("*"), single-quote
   ("'"), and open and close parentheses ("(" and ")"), have been moved
   to the reserved set in order to clarify the distinction between
   reserved and unreserved and, hopefully, to answer the most common
   question of scheme designers.  Likewise, the section on
   percent-encoded characters has been rewritten, and URI normalizers
   are now given license to decode any percent-encoded octets



Berners-Lee, et al.         Standards Track                    [Page 54]
```

