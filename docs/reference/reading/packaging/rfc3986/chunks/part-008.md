# Uniform Resource Identifier (URI): Generic Syntax — part 8

Source: [Uniform Resource Identifier (URI): Generic Syntax](https://www.rfc-editor.org/rfc/rfc3986.txt) · RFC 3986.

[Document index](../README.md) · [Topic guide](../../../../README.md) · [Publisher](https://www.rfc-editor.org/rfc/rfc3986.txt).

Verbatim text slices in fixed-width blocks preserve grammar, examples, and diagrams.

[Previous](part-007.md) · [Next](part-009.md)

<a id="source-page-39-part-1"></a>

## Source page 39

```text

RFC 3986                   URI Generic Syntax               January 2005


6.2.  Comparison Ladder

   A variety of methods are used in practice to test URI equivalence.
   These methods fall into a range, distinguished by the amount of
   processing required and the degree to which the probability of false
   negatives is reduced.  As noted above, false negatives cannot be
   eliminated.  In practice, their probability can be reduced, but this
   reduction requires more processing and is not cost-effective for all
   applications.

   If this range of comparison practices is considered as a ladder, the
   following discussion will climb the ladder, starting with practices
   that are cheap but have a relatively higher chance of producing false
   negatives, and proceeding to those that have higher computational
   cost and lower risk of false negatives.

6.2.1.  Simple String Comparison

   If two URIs, when considered as character strings, are identical,
   then it is safe to conclude that they are equivalent.  This type of
   equivalence test has very low computational cost and is in wide use
   in a variety of applications, particularly in the domain of parsing.

   Testing strings for equivalence requires some basic precautions.
   This procedure is often referred to as "bit-for-bit" or
   "byte-for-byte" comparison, which is potentially misleading.  Testing
   strings for equality is normally based on pair comparison of the
   characters that make up the strings, starting from the first and
   proceeding until both strings are exhausted and all characters are
   found to be equal, until a pair of characters compares unequal, or
   until one of the strings is exhausted before the other.

   This character comparison requires that each pair of characters be
   put in comparable form.  For example, should one URI be stored in a
   byte array in EBCDIC encoding and the second in a Java String object
   (UTF-16), bit-for-bit comparisons applied naively will produce
   errors.  It is better to speak of equality on a character-for-
   character basis rather than on a byte-for-byte or bit-for-bit basis.
   In practical terms, character-by-character comparisons should be done
   codepoint-by-codepoint after conversion to a common character
   encoding.

   False negatives are caused by the production and use of URI aliases.
   Unnecessary aliases can be reduced, regardless of the comparison
   method, by consistently providing URI references in an already-
   normalized form (i.e., a form identical to what would be produced
   after normalization is applied, as described below).




Berners-Lee, et al.         Standards Track                    [Page 39]
```

<a id="source-page-40-part-1"></a>

## Source page 40

```text

RFC 3986                   URI Generic Syntax               January 2005


   Protocols and data formats often limit some URI comparisons to simple
   string comparison, based on the theory that people and
   implementations will, in their own best interest, be consistent in
   providing URI references, or at least consistent enough to negate any
   efficiency that might be obtained from further normalization.

6.2.2.  Syntax-Based Normalization

   Implementations may use logic based on the definitions provided by
   this specification to reduce the probability of false negatives.
   This processing is moderately higher in cost than character-for-
   character string comparison.  For example, an application using this
   approach could reasonably consider the following two URIs equivalent:

      example://a/b/c/%7Bfoo%7D
      eXAMPLE://a/./b/../b/%63/%7bfoo%7d

   Web user agents, such as browsers, typically apply this type of URI
   normalization when determining whether a cached response is
   available.  Syntax-based normalization includes such techniques as
   case normalization, percent-encoding normalization, and removal of
   dot-segments.

6.2.2.1.  Case Normalization

   For all URIs, the hexadecimal digits within a percent-encoding
   triplet (e.g., "%3a" versus "%3A") are case-insensitive and therefore
   should be normalized to use uppercase letters for the digits A-F.

   When a URI uses components of the generic syntax, the component
   syntax equivalence rules always apply; namely, that the scheme and
   host are case-insensitive and therefore should be normalized to
   lowercase.  For example, the URI <HTTP://www.EXAMPLE.com/> is
   equivalent to <http://www.example.com/>.  The other generic syntax
   components are assumed to be case-sensitive unless specifically
   defined otherwise by the scheme (see Section 6.2.3).

6.2.2.2.  Percent-Encoding Normalization

   The percent-encoding mechanism (Section 2.1) is a frequent source of
   variance among otherwise identical URIs.  In addition to the case
   normalization issue noted above, some URI producers percent-encode
   octets that do not require percent-encoding, resulting in URIs that
   are equivalent to their non-encoded counterparts.  These URIs should
   be normalized by decoding any percent-encoded octet that corresponds
   to an unreserved character, as described in Section 2.3.





Berners-Lee, et al.         Standards Track                    [Page 40]
```

<a id="source-page-41-part-1"></a>

## Source page 41

```text

RFC 3986                   URI Generic Syntax               January 2005


6.2.2.3.  Path Segment Normalization

   The complete path segments "." and ".." are intended only for use
   within relative references (Section 4.1) and are removed as part of
   the reference resolution process (Section 5.2).  However, some
   deployed implementations incorrectly assume that reference resolution
   is not necessary when the reference is already a URI and thus fail to
   remove dot-segments when they occur in non-relative paths.  URI
   normalizers should remove dot-segments by applying the
   remove_dot_segments algorithm to the path, as described in
   Section 5.2.4.

6.2.3.  Scheme-Based Normalization

   The syntax and semantics of URIs vary from scheme to scheme, as
   described by the defining specification for each scheme.
   Implementations may use scheme-specific rules, at further processing
   cost, to reduce the probability of false negatives.  For example,
   because the "http" scheme makes use of an authority component, has a
   default port of "80", and defines an empty path to be equivalent to
   "/", the following four URIs are equivalent:

      http://example.com
      http://example.com/
      http://example.com:/
      http://example.com:80/

   In general, a URI that uses the generic syntax for authority with an
   empty path should be normalized to a path of "/".  Likewise, an
   explicit ":port", for which the port is empty or the default for the
   scheme, is equivalent to one where the port and its ":" delimiter are
   elided and thus should be removed by scheme-based normalization.  For
   example, the second URI above is the normal form for the "http"
   scheme.

   Another case where normalization varies by scheme is in the handling
   of an empty authority component or empty host subcomponent.  For many
   scheme specifications, an empty authority or host is considered an
   error; for others, it is considered equivalent to "localhost" or the
   end-user's host.  When a scheme defines a default for authority and a
   URI reference to that default is desired, the reference should be
   normalized to an empty authority for the sake of uniformity, brevity,
   and internationalization.  If, however, either the userinfo or port
   subcomponents are non-empty, then the host should be given explicitly
   even if it matches the default.

   Normalization should not remove delimiters when their associated
   component is empty unless licensed to do so by the scheme



Berners-Lee, et al.         Standards Track                    [Page 41]
```

<a id="source-page-42-part-1"></a>

## Source page 42

```text

RFC 3986                   URI Generic Syntax               January 2005


   specification.  For example, the URI "http://example.com/?" cannot be
   assumed to be equivalent to any of the examples above.  Likewise, the
   presence or absence of delimiters within a userinfo subcomponent is
   usually significant to its interpretation.  The fragment component is
   not subject to any scheme-based normalization; thus, two URIs that
   differ only by the suffix "#" are considered different regardless of
   the scheme.

   Some schemes define additional subcomponents that consist of case-
   insensitive data, giving an implicit license to normalizers to
   convert this data to a common case (e.g., all lowercase).  For
   example, URI schemes that define a subcomponent of path to contain an
   Internet hostname, such as the "mailto" URI scheme, cause that
   subcomponent to be case-insensitive and thus subject to case
   normalization (e.g., "mailto:Joe@Example.COM" is equivalent to
   "mailto:Joe@example.com", even though the generic syntax considers
   the path component to be case-sensitive).

   Other scheme-specific normalizations are possible.

6.2.4.  Protocol-Based Normalization

   Substantial effort to reduce the incidence of false negatives is
   often cost-effective for web spiders.  Therefore, they implement even
   more aggressive techniques in URI comparison.  For example, if they
   observe that a URI such as

      http://example.com/data

   redirects to a URI differing only in the trailing slash

      http://example.com/data/

   they will likely regard the two as equivalent in the future.  This
   kind of technique is only appropriate when equivalence is clearly
   indicated by both the result of accessing the resources and the
   common conventions of their scheme's dereference algorithm (in this
   case, use of redirection by HTTP origin servers to avoid problems
   with relative references).












Berners-Lee, et al.         Standards Track                    [Page 42]
```

<a id="source-page-43-part-1"></a>

## Source page 43

```text

RFC 3986                   URI Generic Syntax               January 2005


7.  Security Considerations

   A URI does not in itself pose a security threat.  However, as URIs
   are often used to provide a compact set of instructions for access to
   network resources, care must be taken to properly interpret the data
   within a URI, to prevent that data from causing unintended access,
   and to avoid including data that should not be revealed in plain
   text.

7.1.  Reliability and Consistency

   There is no guarantee that once a URI has been used to retrieve
   information, the same information will be retrievable by that URI in
   the future.  Nor is there any guarantee that the information
   retrievable via that URI in the future will be observably similar to
   that retrieved in the past.  The URI syntax does not constrain how a
   given scheme or authority apportions its namespace or maintains it
   over time.  Such guarantees can only be obtained from the person(s)
   controlling that namespace and the resource in question.  A specific
   URI scheme may define additional semantics, such as name persistence,
   if those semantics are required of all naming authorities for that
   scheme.

7.2.  Malicious Construction

   It is sometimes possible to construct a URI so that an attempt to
   perform a seemingly harmless, idempotent operation, such as the
   retrieval of a representation, will in fact cause a possibly damaging
   remote operation.  The unsafe URI is typically constructed by
   specifying a port number other than that reserved for the network
   protocol in question.  The client unwittingly contacts a site running
   a different protocol service, and data within the URI contains
   instructions that, when interpreted according to this other protocol,
   cause an unexpected operation.  A frequent example of such abuse has
   been the use of a protocol-based scheme with a port component of
   "25", thereby fooling user agent software into sending an unintended
   or impersonating message via an SMTP server.

   Applications should prevent dereference of a URI that specifies a TCP
   port number within the "well-known port" range (0 - 1023) unless the
   protocol being used to dereference that URI is compatible with the
   protocol expected on that well-known port.  Although IANA maintains a
   registry of well-known ports, applications should make such
   restrictions user-configurable to avoid preventing the deployment of
   new services.






Berners-Lee, et al.         Standards Track                    [Page 43]
```

