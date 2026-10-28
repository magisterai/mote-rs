# Uniform Resource Identifier (URI): Generic Syntax — part 5

Source: [Uniform Resource Identifier (URI): Generic Syntax](https://www.rfc-editor.org/rfc/rfc3986.txt) · RFC 3986.

[Document index](../README.md) · [Topic guide](../../../../README.md) · [Publisher](https://www.rfc-editor.org/rfc/rfc3986.txt).

Verbatim text slices in fixed-width blocks preserve grammar, examples, and diagrams.

[Previous](part-004.md) · [Next](part-006.md)

<a id="source-page-21-part-1"></a>

## Source page 21

```text

RFC 3986                   URI Generic Syntax               January 2005


   defined in Section 3.5 of [RFC1034] and Section 2.1 of [RFC1123].
   Such a name consists of a sequence of domain labels separated by ".",
   each domain label starting and ending with an alphanumeric character
   and possibly also containing "-" characters.  The rightmost domain
   label of a fully qualified domain name in DNS may be followed by a
   single "." and should be if it is necessary to distinguish between
   the complete domain name and some local domain.

      reg-name    = *( unreserved / pct-encoded / sub-delims )

   If the URI scheme defines a default for host, then that default
   applies when the host subcomponent is undefined or when the
   registered name is empty (zero length).  For example, the "file" URI
   scheme is defined so that no authority, an empty host, and
   "localhost" all mean the end-user's machine, whereas the "http"
   scheme considers a missing authority or empty host invalid.

   This specification does not mandate a particular registered name
   lookup technology and therefore does not restrict the syntax of reg-
   name beyond what is necessary for interoperability.  Instead, it
   delegates the issue of registered name syntax conformance to the
   operating system of each application performing URI resolution, and
   that operating system decides what it will allow for the purpose of
   host identification.  A URI resolution implementation might use DNS,
   host tables, yellow pages, NetInfo, WINS, or any other system for
   lookup of registered names.  However, a globally scoped naming
   system, such as DNS fully qualified domain names, is necessary for
   URIs intended to have global scope.  URI producers should use names
   that conform to the DNS syntax, even when use of DNS is not
   immediately apparent, and should limit these names to no more than
   255 characters in length.

   The reg-name syntax allows percent-encoded octets in order to
   represent non-ASCII registered names in a uniform way that is
   independent of the underlying name resolution technology.  Non-ASCII
   characters must first be encoded according to UTF-8 [STD63], and then
   each octet of the corresponding UTF-8 sequence must be percent-
   encoded to be represented as URI characters.  URI producing
   applications must not use percent-encoding in host unless it is used
   to represent a UTF-8 character sequence.  When a non-ASCII registered
   name represents an internationalized domain name intended for
   resolution via the DNS, the name must be transformed to the IDNA
   encoding [RFC3490] prior to name lookup.  URI producers should
   provide these registered names in the IDNA encoding, rather than a
   percent-encoding, if they wish to maximize interoperability with
   legacy URI resolvers.





Berners-Lee, et al.         Standards Track                    [Page 21]
```

<a id="source-page-22-part-1"></a>

## Source page 22

```text

RFC 3986                   URI Generic Syntax               January 2005


3.2.3.  Port

   The port subcomponent of authority is designated by an optional port
   number in decimal following the host and delimited from it by a
   single colon (":") character.

      port        = *DIGIT

   A scheme may define a default port.  For example, the "http" scheme
   defines a default port of "80", corresponding to its reserved TCP
   port number.  The type of port designated by the port number (e.g.,
   TCP, UDP, SCTP) is defined by the URI scheme.  URI producers and
   normalizers should omit the port component and its ":" delimiter if
   port is empty or if its value would be the same as that of the
   scheme's default.

3.3.  Path

   The path component contains data, usually organized in hierarchical
   form, that, along with data in the non-hierarchical query component
   (Section 3.4), serves to identify a resource within the scope of the
   URI's scheme and naming authority (if any).  The path is terminated
   by the first question mark ("?") or number sign ("#") character, or
   by the end of the URI.

   If a URI contains an authority component, then the path component
   must either be empty or begin with a slash ("/") character.  If a URI
   does not contain an authority component, then the path cannot begin
   with two slash characters ("//").  In addition, a URI reference
   (Section 4.1) may be a relative-path reference, in which case the
   first path segment cannot contain a colon (":") character.  The ABNF
   requires five separate rules to disambiguate these cases, only one of
   which will match the path substring within a given URI reference.  We
   use the generic term "path component" to describe the URI substring
   matched by the parser to one of these rules.

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




Berners-Lee, et al.         Standards Track                    [Page 22]
```

<a id="source-page-23-part-1"></a>

## Source page 23

```text

RFC 3986                   URI Generic Syntax               January 2005


      segment       = *pchar
      segment-nz    = 1*pchar
      segment-nz-nc = 1*( unreserved / pct-encoded / sub-delims / "@" )
                    ; non-zero-length segment without any colon ":"

      pchar         = unreserved / pct-encoded / sub-delims / ":" / "@"

   A path consists of a sequence of path segments separated by a slash
   ("/") character.  A path is always defined for a URI, though the
   defined path may be empty (zero length).  Use of the slash character
   to indicate hierarchy is only required when a URI will be used as the
   context for relative references.  For example, the URI
   <mailto:fred@example.com> has a path of "fred@example.com", whereas
   the URI <foo://info.example.com?fred> has an empty path.

   The path segments "." and "..", also known as dot-segments, are
   defined for relative reference within the path name hierarchy.  They
   are intended for use at the beginning of a relative-path reference
   (Section 4.2) to indicate relative position within the hierarchical
   tree of names.  This is similar to their role within some operating
   systems' file directory structures to indicate the current directory
   and parent directory, respectively.  However, unlike in a file
   system, these dot-segments are only interpreted within the URI path
   hierarchy and are removed as part of the resolution process (Section
   5.2).

   Aside from dot-segments in hierarchical paths, a path segment is
   considered opaque by the generic syntax.  URI producing applications
   often use the reserved characters allowed in a segment to delimit
   scheme-specific or dereference-handler-specific subcomponents.  For
   example, the semicolon (";") and equals ("=") reserved characters are
   often used to delimit parameters and parameter values applicable to
   that segment.  The comma (",") reserved character is often used for
   similar purposes.  For example, one URI producer might use a segment
   such as "name;v=1.1" to indicate a reference to version 1.1 of
   "name", whereas another might use a segment such as "name,1.1" to
   indicate the same.  Parameter types may be defined by scheme-specific
   semantics, but in most cases the syntax of a parameter is specific to
   the implementation of the URI's dereferencing algorithm.

3.4.  Query

   The query component contains non-hierarchical data that, along with
   data in the path component (Section 3.3), serves to identify a
   resource within the scope of the URI's scheme and naming authority
   (if any).  The query component is indicated by the first question
   mark ("?") character and terminated by a number sign ("#") character
   or by the end of the URI.



Berners-Lee, et al.         Standards Track                    [Page 23]
```

<a id="source-page-24-part-1"></a>

## Source page 24

```text

RFC 3986                   URI Generic Syntax               January 2005


      query       = *( pchar / "/" / "?" )

   The characters slash ("/") and question mark ("?") may represent data
   within the query component.  Beware that some older, erroneous
   implementations may not handle such data correctly when it is used as
   the base URI for relative references (Section 5.1), apparently
   because they fail to distinguish query data from path data when
   looking for hierarchical separators.  However, as query components
   are often used to carry identifying information in the form of
   "key=value" pairs and one frequently used value is a reference to
   another URI, it is sometimes better for usability to avoid percent-
   encoding those characters.

3.5.  Fragment

   The fragment identifier component of a URI allows indirect
   identification of a secondary resource by reference to a primary
   resource and additional identifying information.  The identified
   secondary resource may be some portion or subset of the primary
   resource, some view on representations of the primary resource, or
   some other resource defined or described by those representations.  A
   fragment identifier component is indicated by the presence of a
   number sign ("#") character and terminated by the end of the URI.

      fragment    = *( pchar / "/" / "?" )

   The semantics of a fragment identifier are defined by the set of
   representations that might result from a retrieval action on the
   primary resource.  The fragment's format and resolution is therefore
   dependent on the media type [RFC2046] of a potentially retrieved
   representation, even though such a retrieval is only performed if the
   URI is dereferenced.  If no such representation exists, then the
   semantics of the fragment are considered unknown and are effectively
   unconstrained.  Fragment identifier semantics are independent of the
   URI scheme and thus cannot be redefined by scheme specifications.

   Individual media types may define their own restrictions on or
   structures within the fragment identifier syntax for specifying
   different types of subsets, views, or external references that are
   identifiable as secondary resources by that media type.  If the
   primary resource has multiple representations, as is often the case
   for resources whose representation is selected based on attributes of
   the retrieval request (a.k.a., content negotiation), then whatever is
   identified by the fragment should be consistent across all of those
   representations.  Each representation should either define the
   fragment so that it corresponds to the same secondary resource,
   regardless of how it is represented, or should leave the fragment
   undefined (i.e., not found).



Berners-Lee, et al.         Standards Track                    [Page 24]
```

<a id="source-page-25-part-1"></a>

## Source page 25

```text

RFC 3986                   URI Generic Syntax               January 2005


   As with any URI, use of a fragment identifier component does not
   imply that a retrieval action will take place.  A URI with a fragment
   identifier may be used to refer to the secondary resource without any
   implication that the primary resource is accessible or will ever be
   accessed.

   Fragment identifiers have a special role in information retrieval
   systems as the primary form of client-side indirect referencing,
   allowing an author to specifically identify aspects of an existing
   resource that are only indirectly provided by the resource owner.  As
   such, the fragment identifier is not used in the scheme-specific
   processing of a URI; instead, the fragment identifier is separated
   from the rest of the URI prior to a dereference, and thus the
   identifying information within the fragment itself is dereferenced
   solely by the user agent, regardless of the URI scheme.  Although
   this separate handling is often perceived to be a loss of
   information, particularly for accurate redirection of references as
   resources move over time, it also serves to prevent information
   providers from denying reference authors the right to refer to
   information within a resource selectively.  Indirect referencing also
   provides additional flexibility and extensibility to systems that use
   URIs, as new media types are easier to define and deploy than new
   schemes of identification.

   The characters slash ("/") and question mark ("?") are allowed to
   represent data within the fragment identifier.  Beware that some
   older, erroneous implementations may not handle this data correctly
   when it is used as the base URI for relative references (Section
   5.1).

4.  Usage

   When applications make reference to a URI, they do not always use the
   full form of reference defined by the "URI" syntax rule.  To save
   space and take advantage of hierarchical locality, many Internet
   protocol elements and media type formats allow an abbreviation of a
   URI, whereas others restrict the syntax to a particular form of URI.
   We define the most common forms of reference syntax in this
   specification because they impact and depend upon the design of the
   generic syntax, requiring a uniform parsing algorithm in order to be
   interpreted consistently.

4.1.  URI Reference

   URI-reference is used to denote the most common usage of a resource
   identifier.

      URI-reference = URI / relative-ref



Berners-Lee, et al.         Standards Track                    [Page 25]
```

