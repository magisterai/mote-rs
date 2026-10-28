# Uniform Resource Identifier (URI): Generic Syntax — part 4

Source: [Uniform Resource Identifier (URI): Generic Syntax](https://www.rfc-editor.org/rfc/rfc3986.txt) · RFC 3986.

[Document index](../README.md) · [Topic guide](../../../../README.md) · [Publisher](https://www.rfc-editor.org/rfc/rfc3986.txt).

Verbatim text slices in fixed-width blocks preserve grammar, examples, and diagrams.

[Previous](part-003.md) · [Next](part-005.md)

<a id="source-page-16-part-1"></a>

## Source page 16

```text

RFC 3986                   URI Generic Syntax               January 2005


   coordinates on a map.  Likewise, a URI scheme may define components
   with additional encoding requirements that are applied prior to
   forming the component and producing the URI.

   When a new URI scheme defines a component that represents textual
   data consisting of characters from the Universal Character Set [UCS],
   the data should first be encoded as octets according to the UTF-8
   character encoding [STD63]; then only those octets that do not
   correspond to characters in the unreserved set should be percent-
   encoded.  For example, the character A would be represented as "A",
   the character LATIN CAPITAL LETTER A WITH GRAVE would be represented
   as "%C3%80", and the character KATAKANA LETTER A would be represented
   as "%E3%82%A2".

3.  Syntax Components

   The generic URI syntax consists of a hierarchical sequence of
   components referred to as the scheme, authority, path, query, and
   fragment.

      URI         = scheme ":" hier-part [ "?" query ] [ "#" fragment ]

      hier-part   = "//" authority path-abempty
                  / path-absolute
                  / path-rootless
                  / path-empty

   The scheme and path components are required, though the path may be
   empty (no characters).  When authority is present, the path must
   either be empty or begin with a slash ("/") character.  When
   authority is not present, the path cannot begin with two slash
   characters ("//").  These restrictions result in five different ABNF
   rules for a path (Section 3.3), only one of which will match any
   given URI reference.

   The following are two example URIs and their component parts:

         foo://example.com:8042/over/there?name=ferret#nose
         \_/   \______________/\_________/ \_________/ \__/
          |           |            |            |        |
       scheme     authority       path        query   fragment
          |   _____________________|__
         / \ /                        \
         urn:example:animal:ferret:nose







Berners-Lee, et al.         Standards Track                    [Page 16]
```

<a id="source-page-17-part-1"></a>

## Source page 17

```text

RFC 3986                   URI Generic Syntax               January 2005


3.1.  Scheme

   Each URI begins with a scheme name that refers to a specification for
   assigning identifiers within that scheme.  As such, the URI syntax is
   a federated and extensible naming system wherein each scheme's
   specification may further restrict the syntax and semantics of
   identifiers using that scheme.

   Scheme names consist of a sequence of characters beginning with a
   letter and followed by any combination of letters, digits, plus
   ("+"), period ("."), or hyphen ("-").  Although schemes are case-
   insensitive, the canonical form is lowercase and documents that
   specify schemes must do so with lowercase letters.  An implementation
   should accept uppercase letters as equivalent to lowercase in scheme
   names (e.g., allow "HTTP" as well as "http") for the sake of
   robustness but should only produce lowercase scheme names for
   consistency.

      scheme      = ALPHA *( ALPHA / DIGIT / "+" / "-" / "." )

   Individual schemes are not specified by this document.  The process
   for registration of new URI schemes is defined separately by [BCP35].
   The scheme registry maintains the mapping between scheme names and
   their specifications.  Advice for designers of new URI schemes can be
   found in [RFC2718].  URI scheme specifications must define their own
   syntax so that all strings matching their scheme-specific syntax will
   also match the <absolute-URI> grammar, as described in Section 4.3.

   When presented with a URI that violates one or more scheme-specific
   restrictions, the scheme-specific resolution process should flag the
   reference as an error rather than ignore the unused parts; doing so
   reduces the number of equivalent URIs and helps detect abuses of the
   generic syntax, which might indicate that the URI has been
   constructed to mislead the user (Section 7.6).

3.2.  Authority

   Many URI schemes include a hierarchical element for a naming
   authority so that governance of the name space defined by the
   remainder of the URI is delegated to that authority (which may, in
   turn, delegate it further).  The generic syntax provides a common
   means for distinguishing an authority based on a registered name or
   server address, along with optional port and user information.

   The authority component is preceded by a double slash ("//") and is
   terminated by the next slash ("/"), question mark ("?"), or number
   sign ("#") character, or by the end of the URI.




Berners-Lee, et al.         Standards Track                    [Page 17]
```

<a id="source-page-18-part-1"></a>

## Source page 18

```text

RFC 3986                   URI Generic Syntax               January 2005


      authority   = [ userinfo "@" ] host [ ":" port ]

   URI producers and normalizers should omit the ":" delimiter that
   separates host from port if the port component is empty.  Some
   schemes do not allow the userinfo and/or port subcomponents.

   If a URI contains an authority component, then the path component
   must either be empty or begin with a slash ("/") character.  Non-
   validating parsers (those that merely separate a URI reference into
   its major components) will often ignore the subcomponent structure of
   authority, treating it as an opaque string from the double-slash to
   the first terminating delimiter, until such time as the URI is
   dereferenced.

3.2.1.  User Information

   The userinfo subcomponent may consist of a user name and, optionally,
   scheme-specific information about how to gain authorization to access
   the resource.  The user information, if present, is followed by a
   commercial at-sign ("@") that delimits it from the host.

      userinfo    = *( unreserved / pct-encoded / sub-delims / ":" )

   Use of the format "user:password" in the userinfo field is
   deprecated.  Applications should not render as clear text any data
   after the first colon (":") character found within a userinfo
   subcomponent unless the data after the colon is the empty string
   (indicating no password).  Applications may choose to ignore or
   reject such data when it is received as part of a reference and
   should reject the storage of such data in unencrypted form.  The
   passing of authentication information in clear text has proven to be
   a security risk in almost every case where it has been used.

   Applications that render a URI for the sake of user feedback, such as
   in graphical hypertext browsing, should render userinfo in a way that
   is distinguished from the rest of a URI, when feasible.  Such
   rendering will assist the user in cases where the userinfo has been
   misleadingly crafted to look like a trusted domain name
   (Section 7.6).

3.2.2.  Host

   The host subcomponent of authority is identified by an IP literal
   encapsulated within square brackets, an IPv4 address in dotted-
   decimal form, or a registered name.  The host subcomponent is case-
   insensitive.  The presence of a host subcomponent within a URI does
   not imply that the scheme requires access to the given host on the
   Internet.  In many cases, the host syntax is used only for the sake



Berners-Lee, et al.         Standards Track                    [Page 18]
```

<a id="source-page-19-part-1"></a>

## Source page 19

```text

RFC 3986                   URI Generic Syntax               January 2005


   of reusing the existing registration process created and deployed for
   DNS, thus obtaining a globally unique name without the cost of
   deploying another registry.  However, such use comes with its own
   costs: domain name ownership may change over time for reasons not
   anticipated by the URI producer.  In other cases, the data within the
   host component identifies a registered name that has nothing to do
   with an Internet host.  We use the name "host" for the ABNF rule
   because that is its most common purpose, not its only purpose.

      host        = IP-literal / IPv4address / reg-name

   The syntax rule for host is ambiguous because it does not completely
   distinguish between an IPv4address and a reg-name.  In order to
   disambiguate the syntax, we apply the "first-match-wins" algorithm:
   If host matches the rule for IPv4address, then it should be
   considered an IPv4 address literal and not a reg-name.  Although host
   is case-insensitive, producers and normalizers should use lowercase
   for registered names and hexadecimal addresses for the sake of
   uniformity, while only using uppercase letters for percent-encodings.

   A host identified by an Internet Protocol literal address, version 6
   [RFC3513] or later, is distinguished by enclosing the IP literal
   within square brackets ("[" and "]").  This is the only place where
   square bracket characters are allowed in the URI syntax.  In
   anticipation of future, as-yet-undefined IP literal address formats,
   an implementation may use an optional version flag to indicate such a
   format explicitly rather than rely on heuristic determination.

      IP-literal = "[" ( IPv6address / IPvFuture  ) "]"

      IPvFuture  = "v" 1*HEXDIG "." 1*( unreserved / sub-delims / ":" )

   The version flag does not indicate the IP version; rather, it
   indicates future versions of the literal format.  As such,
   implementations must not provide the version flag for the existing
   IPv4 and IPv6 literal address forms described below.  If a URI
   containing an IP-literal that starts with "v" (case-insensitive),
   indicating that the version flag is present, is dereferenced by an
   application that does not know the meaning of that version flag, then
   the application should return an appropriate error for "address
   mechanism not supported".

   A host identified by an IPv6 literal address is represented inside
   the square brackets without a preceding version flag.  The ABNF
   provided here is a translation of the text definition of an IPv6
   literal address provided in [RFC3513].  This syntax does not support
   IPv6 scoped addressing zone identifiers.




Berners-Lee, et al.         Standards Track                    [Page 19]
```

<a id="source-page-20-part-1"></a>

## Source page 20

```text

RFC 3986                   URI Generic Syntax               January 2005


   A 128-bit IPv6 address is divided into eight 16-bit pieces.  Each
   piece is represented numerically in case-insensitive hexadecimal,
   using one to four hexadecimal digits (leading zeroes are permitted).
   The eight encoded pieces are given most-significant first, separated
   by colon characters.  Optionally, the least-significant two pieces
   may instead be represented in IPv4 address textual format.  A
   sequence of one or more consecutive zero-valued 16-bit pieces within
   the address may be elided, omitting all their digits and leaving
   exactly two consecutive colons in their place to mark the elision.

      IPv6address =                            6( h16 ":" ) ls32
                  /                       "::" 5( h16 ":" ) ls32
                  / [               h16 ] "::" 4( h16 ":" ) ls32
                  / [ *1( h16 ":" ) h16 ] "::" 3( h16 ":" ) ls32
                  / [ *2( h16 ":" ) h16 ] "::" 2( h16 ":" ) ls32
                  / [ *3( h16 ":" ) h16 ] "::"    h16 ":"   ls32
                  / [ *4( h16 ":" ) h16 ] "::"              ls32
                  / [ *5( h16 ":" ) h16 ] "::"              h16
                  / [ *6( h16 ":" ) h16 ] "::"

      ls32        = ( h16 ":" h16 ) / IPv4address
                  ; least-significant 32 bits of address

      h16         = 1*4HEXDIG
                  ; 16 bits of address represented in hexadecimal

   A host identified by an IPv4 literal address is represented in
   dotted-decimal notation (a sequence of four decimal numbers in the
   range 0 to 255, separated by "."), as described in [RFC1123] by
   reference to [RFC0952].  Note that other forms of dotted notation may
   be interpreted on some platforms, as described in Section 7.4, but
   only the dotted-decimal form of four octets is allowed by this
   grammar.

      IPv4address = dec-octet "." dec-octet "." dec-octet "." dec-octet

      dec-octet   = DIGIT                 ; 0-9
                  / %x31-39 DIGIT         ; 10-99
                  / "1" 2DIGIT            ; 100-199
                  / "2" %x30-34 DIGIT     ; 200-249
                  / "25" %x30-35          ; 250-255

   A host identified by a registered name is a sequence of characters
   usually intended for lookup within a locally defined host or service
   name registry, though the URI's scheme-specific semantics may require
   that a specific registry (or fixed name table) be used instead.  The
   most common name registry mechanism is the Domain Name System (DNS).
   A registered name intended for lookup in the DNS uses the syntax



Berners-Lee, et al.         Standards Track                    [Page 20]
```

