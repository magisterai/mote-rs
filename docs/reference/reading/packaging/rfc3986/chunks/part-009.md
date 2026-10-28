# Uniform Resource Identifier (URI): Generic Syntax — part 9

Source: [Uniform Resource Identifier (URI): Generic Syntax](https://www.rfc-editor.org/rfc/rfc3986.txt) · RFC 3986.

[Document index](../README.md) · [Topic guide](../../../../README.md) · [Publisher](https://www.rfc-editor.org/rfc/rfc3986.txt).

Verbatim text slices in fixed-width blocks preserve grammar, examples, and diagrams.

[Previous](part-008.md) · [Next](part-010.md)

<a id="source-page-44-part-1"></a>

## Source page 44

```text

RFC 3986                   URI Generic Syntax               January 2005


   When a URI contains percent-encoded octets that match the delimiters
   for a given resolution or dereference protocol (for example, CR and
   LF characters for the TELNET protocol), these percent-encodings must
   not be decoded before transmission across that protocol.  Transfer of
   the percent-encoding, which might violate the protocol, is less
   harmful than allowing decoded octets to be interpreted as additional
   operations or parameters, perhaps triggering an unexpected and
   possibly harmful remote operation.

7.3.  Back-End Transcoding

   When a URI is dereferenced, the data within it is often parsed by
   both the user agent and one or more servers.  In HTTP, for example, a
   typical user agent will parse a URI into its five major components,
   access the authority's server, and send it the data within the
   authority, path, and query components.  A typical server will take
   that information, parse the path into segments and the query into
   key/value pairs, and then invoke implementation-specific handlers to
   respond to the request.  As a result, a common security concern for
   server implementations that handle a URI, either as a whole or split
   into separate components, is proper interpretation of the octet data
   represented by the characters and percent-encodings within that URI.

   Percent-encoded octets must be decoded at some point during the
   dereference process.  Applications must split the URI into its
   components and subcomponents prior to decoding the octets, as
   otherwise the decoded octets might be mistaken for delimiters.
   Security checks of the data within a URI should be applied after
   decoding the octets.  Note, however, that the "%00" percent-encoding
   (NUL) may require special handling and should be rejected if the
   application is not expecting to receive raw data within a component.

   Special care should be taken when the URI path interpretation process
   involves the use of a back-end file system or related system
   functions.  File systems typically assign an operational meaning to
   special characters, such as the "/", "\", ":", "[", and "]"
   characters, and to special device names like ".", "..", "...", "aux",
   "lpt", etc.  In some cases, merely testing for the existence of such
   a name will cause the operating system to pause or invoke unrelated
   system calls, leading to significant security concerns regarding
   denial of service and unintended data transfer.  It would be
   impossible for this specification to list all such significant
   characters and device names.  Implementers should research the
   reserved names and characters for the types of storage device that
   may be attached to their applications and restrict the use of data
   obtained from URI components accordingly.





Berners-Lee, et al.         Standards Track                    [Page 44]
```

<a id="source-page-45-part-1"></a>

## Source page 45

```text

RFC 3986                   URI Generic Syntax               January 2005


7.4.  Rare IP Address Formats

   Although the URI syntax for IPv4address only allows the common
   dotted-decimal form of IPv4 address literal, many implementations
   that process URIs make use of platform-dependent system routines,
   such as gethostbyname() and inet_aton(), to translate the string
   literal to an actual IP address.  Unfortunately, such system routines
   often allow and process a much larger set of formats than those
   described in Section 3.2.2.

   For example, many implementations allow dotted forms of three
   numbers, wherein the last part is interpreted as a 16-bit quantity
   and placed in the right-most two bytes of the network address (e.g.,
   a Class B network).  Likewise, a dotted form of two numbers means
   that the last part is interpreted as a 24-bit quantity and placed in
   the right-most three bytes of the network address (Class A), and a
   single number (without dots) is interpreted as a 32-bit quantity and
   stored directly in the network address.  Adding further to the
   confusion, some implementations allow each dotted part to be
   interpreted as decimal, octal, or hexadecimal, as specified in the C
   language (i.e., a leading 0x or 0X implies hexadecimal; a leading 0
   implies octal; otherwise, the number is interpreted as decimal).

   These additional IP address formats are not allowed in the URI syntax
   due to differences between platform implementations.  However, they
   can become a security concern if an application attempts to filter
   access to resources based on the IP address in string literal format.
   If this filtering is performed, literals should be converted to
   numeric form and filtered based on the numeric value, and not on a
   prefix or suffix of the string form.

7.5.  Sensitive Information

   URI producers should not provide a URI that contains a username or
   password that is intended to be secret.  URIs are frequently
   displayed by browsers, stored in clear text bookmarks, and logged by
   user agent history and intermediary applications (proxies).  A
   password appearing within the userinfo component is deprecated and
   should be considered an error (or simply ignored) except in those
   rare cases where the 'password' parameter is intended to be public.

7.6.  Semantic Attacks

   Because the userinfo subcomponent is rarely used and appears before
   the host in the authority component, it can be used to construct a
   URI intended to mislead a human user by appearing to identify one
   (trusted) naming authority while actually identifying a different
   authority hidden behind the noise.  For example



Berners-Lee, et al.         Standards Track                    [Page 45]
```

<a id="source-page-46-part-1"></a>

## Source page 46

```text

RFC 3986                   URI Generic Syntax               January 2005


      ftp://cnn.example.com&story=breaking_news@10.0.0.1/top_story.htm

   might lead a human user to assume that the host is 'cnn.example.com',
   whereas it is actually '10.0.0.1'.  Note that a misleading userinfo
   subcomponent could be much longer than the example above.

   A misleading URI, such as that above, is an attack on the user's
   preconceived notions about the meaning of a URI rather than an attack
   on the software itself.  User agents may be able to reduce the impact
   of such attacks by distinguishing the various components of the URI
   when they are rendered, such as by using a different color or tone to
   render userinfo if any is present, though there is no panacea.  More
   information on URI-based semantic attacks can be found in [Siedzik].

8.  IANA Considerations

   URI scheme names, as defined by <scheme> in Section 3.1, form a
   registered namespace that is managed by IANA according to the
   procedures defined in [BCP35].  No IANA actions are required by this
   document.

9.  Acknowledgements

   This specification is derived from RFC 2396 [RFC2396], RFC 1808
   [RFC1808], and RFC 1738 [RFC1738]; the acknowledgements in those
   documents still apply.  It also incorporates the update (with
   corrections) for IPv6 literals in the host syntax, as defined by
   Robert M. Hinden, Brian E. Carpenter, and Larry Masinter in
   [RFC2732].  In addition, contributions by Gisle Aas, Reese Anschultz,
   Daniel Barclay, Tim Bray, Mike Brown, Rob Cameron, Jeremy Carroll,
   Dan Connolly, Adam M. Costello, John Cowan, Jason Diamond, Martin
   Duerst, Stefan Eissing, Clive D.W. Feather, Al Gilman, Tony Hammond,
   Elliotte Harold, Pat Hayes, Henry Holtzman, Ian B. Jacobs, Michael
   Kay, John C. Klensin, Graham Klyne, Dan Kohn, Bruce Lilly, Andrew
   Main, Dave McAlpin, Ira McDonald, Michael Mealling, Ray Merkert,
   Stephen Pollei, Julian Reschke, Tomas Rokicki, Miles Sabin, Kai
   Schaetzl, Mark Thomson, Ronald Tschalaer, Norm Walsh, Marc Warne,
   Stuart Williams, and Henry Zongaro are gratefully acknowledged.

10.  References

10.1.  Normative References

   [ASCII]    American National Standards Institute, "Coded Character
              Set -- 7-bit American Standard Code for Information
              Interchange", ANSI X3.4, 1986.





Berners-Lee, et al.         Standards Track                    [Page 46]
```

<a id="source-page-47-part-1"></a>

## Source page 47

```text

RFC 3986                   URI Generic Syntax               January 2005


   [RFC2234]  Crocker, D. and P. Overell, "Augmented BNF for Syntax
              Specifications: ABNF", RFC 2234, November 1997.

   [STD63]    Yergeau, F., "UTF-8, a transformation format of
              ISO 10646", STD 63, RFC 3629, November 2003.

   [UCS]      International Organization for Standardization,
              "Information Technology - Universal Multiple-Octet Coded
              Character Set (UCS)", ISO/IEC 10646:2003, December 2003.

10.2.  Informative References

   [BCP19]    Freed, N. and J. Postel, "IANA Charset Registration
              Procedures", BCP 19, RFC 2978, October 2000.

   [BCP35]    Petke, R. and I. King, "Registration Procedures for URL
              Scheme Names", BCP 35, RFC 2717, November 1999.

   [RFC0952]  Harrenstien, K., Stahl, M., and E. Feinler, "DoD Internet
              host table specification", RFC 952, October 1985.

   [RFC1034]  Mockapetris, P., "Domain names - concepts and facilities",
              STD 13, RFC 1034, November 1987.

   [RFC1123]  Braden, R., "Requirements for Internet Hosts - Application
              and Support", STD 3, RFC 1123, October 1989.

   [RFC1535]  Gavron, E., "A Security Problem and Proposed Correction
              With Widely Deployed DNS Software", RFC 1535,
              October 1993.

   [RFC1630]  Berners-Lee, T., "Universal Resource Identifiers in WWW: A
              Unifying Syntax for the Expression of Names and Addresses
              of Objects on the Network as used in the World-Wide Web",
              RFC 1630, June 1994.

   [RFC1736]  Kunze, J., "Functional Recommendations for Internet
              Resource Locators", RFC 1736, February 1995.

   [RFC1737]  Sollins, K. and L. Masinter, "Functional Requirements for
              Uniform Resource Names", RFC 1737, December 1994.

   [RFC1738]  Berners-Lee, T., Masinter, L., and M. McCahill, "Uniform
              Resource Locators (URL)", RFC 1738, December 1994.

   [RFC1808]  Fielding, R., "Relative Uniform Resource Locators",
              RFC 1808, June 1995.




Berners-Lee, et al.         Standards Track                    [Page 47]
```

<a id="source-page-48-part-1"></a>

## Source page 48

```text

RFC 3986                   URI Generic Syntax               January 2005


   [RFC2046]  Freed, N. and N. Borenstein, "Multipurpose Internet Mail
              Extensions (MIME) Part Two: Media Types", RFC 2046,
              November 1996.

   [RFC2141]  Moats, R., "URN Syntax", RFC 2141, May 1997.

   [RFC2396]  Berners-Lee, T., Fielding, R., and L. Masinter, "Uniform
              Resource Identifiers (URI): Generic Syntax", RFC 2396,
              August 1998.

   [RFC2518]  Goland, Y., Whitehead, E., Faizi, A., Carter, S., and D.
              Jensen, "HTTP Extensions for Distributed Authoring --
              WEBDAV", RFC 2518, February 1999.

   [RFC2557]  Palme, J., Hopmann, A., and N. Shelness, "MIME
              Encapsulation of Aggregate Documents, such as HTML
              (MHTML)", RFC 2557, March 1999.

   [RFC2718]  Masinter, L., Alvestrand, H., Zigmond, D., and R. Petke,
              "Guidelines for new URL Schemes", RFC 2718, November 1999.

   [RFC2732]  Hinden, R., Carpenter, B., and L. Masinter, "Format for
              Literal IPv6 Addresses in URL's", RFC 2732, December 1999.

   [RFC3305]  Mealling, M. and R. Denenberg, "Report from the Joint
              W3C/IETF URI Planning Interest Group: Uniform Resource
              Identifiers (URIs), URLs, and Uniform Resource Names
              (URNs): Clarifications and Recommendations", RFC 3305,
              August 2002.

   [RFC3490]  Faltstrom, P., Hoffman, P., and A. Costello,
              "Internationalizing Domain Names in Applications (IDNA)",
              RFC 3490, March 2003.

   [RFC3513]  Hinden, R. and S. Deering, "Internet Protocol Version 6
              (IPv6) Addressing Architecture", RFC 3513, April 2003.

   [Siedzik]  Siedzik, R., "Semantic Attacks: What's in a URL?",
              April 2001, <http://www.giac.org/practical/gsec/
              Richard_Siedzik_GSEC.pdf>.











Berners-Lee, et al.         Standards Track                    [Page 48]
```

