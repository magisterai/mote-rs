# Uniform Resource Identifier (URI): Generic Syntax — part 6

Source: [Uniform Resource Identifier (URI): Generic Syntax](https://www.rfc-editor.org/rfc/rfc3986.txt) · RFC 3986.

[Document index](../README.md) · [Topic guide](../../../../README.md) · [Publisher](https://www.rfc-editor.org/rfc/rfc3986.txt).

Verbatim text slices in fixed-width blocks preserve grammar, examples, and diagrams.

[Previous](part-005.md) · [Next](part-007.md)

<a id="source-page-26-part-1"></a>

## Source page 26

```text

RFC 3986                   URI Generic Syntax               January 2005


   A URI-reference is either a URI or a relative reference.  If the
   URI-reference's prefix does not match the syntax of a scheme followed
   by its colon separator, then the URI-reference is a relative
   reference.

   A URI-reference is typically parsed first into the five URI
   components, in order to determine what components are present and
   whether the reference is relative.  Then, each component is parsed
   for its subparts and their validation.  The ABNF of URI-reference,
   along with the "first-match-wins" disambiguation rule, is sufficient
   to define a validating parser for the generic syntax.  Readers
   familiar with regular expressions should see Appendix B for an
   example of a non-validating URI-reference parser that will take any
   given string and extract the URI components.

4.2.  Relative Reference

   A relative reference takes advantage of the hierarchical syntax
   (Section 1.2.3) to express a URI reference relative to the name space
   of another hierarchical URI.

      relative-ref  = relative-part [ "?" query ] [ "#" fragment ]

      relative-part = "//" authority path-abempty
                    / path-absolute
                    / path-noscheme
                    / path-empty

   The URI referred to by a relative reference, also known as the target
   URI, is obtained by applying the reference resolution algorithm of
   Section 5.

   A relative reference that begins with two slash characters is termed
   a network-path reference; such references are rarely used.  A
   relative reference that begins with a single slash character is
   termed an absolute-path reference.  A relative reference that does
   not begin with a slash character is termed a relative-path reference.

   A path segment that contains a colon character (e.g., "this:that")
   cannot be used as the first segment of a relative-path reference, as
   it would be mistaken for a scheme name.  Such a segment must be
   preceded by a dot-segment (e.g., "./this:that") to make a relative-
   path reference.








Berners-Lee, et al.         Standards Track                    [Page 26]
```

<a id="source-page-27-part-1"></a>

## Source page 27

```text

RFC 3986                   URI Generic Syntax               January 2005


4.3.  Absolute URI

   Some protocol elements allow only the absolute form of a URI without
   a fragment identifier.  For example, defining a base URI for later
   use by relative references calls for an absolute-URI syntax rule that
   does not allow a fragment.

      absolute-URI  = scheme ":" hier-part [ "?" query ]

   URI scheme specifications must define their own syntax so that all
   strings matching their scheme-specific syntax will also match the
   <absolute-URI> grammar.  Scheme specifications will not define
   fragment identifier syntax or usage, regardless of its applicability
   to resources identifiable via that scheme, as fragment identification
   is orthogonal to scheme definition.  However, scheme specifications
   are encouraged to include a wide range of examples, including
   examples that show use of the scheme's URIs with fragment identifiers
   when such usage is appropriate.

4.4.  Same-Document Reference

   When a URI reference refers to a URI that is, aside from its fragment
   component (if any), identical to the base URI (Section 5.1), that
   reference is called a "same-document" reference.  The most frequent
   examples of same-document references are relative references that are
   empty or include only the number sign ("#") separator followed by a
   fragment identifier.

   When a same-document reference is dereferenced for a retrieval
   action, the target of that reference is defined to be within the same
   entity (representation, document, or message) as the reference;
   therefore, a dereference should not result in a new retrieval action.

   Normalization of the base and target URIs prior to their comparison,
   as described in Sections 6.2.2 and 6.2.3, is allowed but rarely
   performed in practice.  Normalization may increase the set of same-
   document references, which may be of benefit to some caching
   applications.  As such, reference authors should not assume that a
   slightly different, though equivalent, reference URI will (or will
   not) be interpreted as a same-document reference by any given
   application.

4.5.  Suffix Reference

   The URI syntax is designed for unambiguous reference to resources and
   extensibility via the URI scheme.  However, as URI identification and
   usage have become commonplace, traditional media (television, radio,
   newspapers, billboards, etc.) have increasingly used a suffix of the



Berners-Lee, et al.         Standards Track                    [Page 27]
```

<a id="source-page-28-part-1"></a>

## Source page 28

```text

RFC 3986                   URI Generic Syntax               January 2005


   URI as a reference, consisting of only the authority and path
   portions of the URI, such as

      www.w3.org/Addressing/

   or simply a DNS registered name on its own.  Such references are
   primarily intended for human interpretation rather than for machines,
   with the assumption that context-based heuristics are sufficient to
   complete the URI (e.g., most registered names beginning with "www"
   are likely to have a URI prefix of "http://").  Although there is no
   standard set of heuristics for disambiguating a URI suffix, many
   client implementations allow them to be entered by the user and
   heuristically resolved.

   Although this practice of using suffix references is common, it
   should be avoided whenever possible and should never be used in
   situations where long-term references are expected.  The heuristics
   noted above will change over time, particularly when a new URI scheme
   becomes popular, and are often incorrect when used out of context.
   Furthermore, they can lead to security issues along the lines of
   those described in [RFC1535].

   As a URI suffix has the same syntax as a relative-path reference, a
   suffix reference cannot be used in contexts where a relative
   reference is expected.  As a result, suffix references are limited to
   places where there is no defined base URI, such as dialog boxes and
   off-line advertisements.

5.  Reference Resolution

   This section defines the process of resolving a URI reference within
   a context that allows relative references so that the result is a
   string matching the <URI> syntax rule of Section 3.

5.1.  Establishing a Base URI

   The term "relative" implies that a "base URI" exists against which
   the relative reference is applied.  Aside from fragment-only
   references (Section 4.4), relative references are only usable when a
   base URI is known.  A base URI must be established by the parser
   prior to parsing URI references that might be relative.  A base URI
   must conform to the <absolute-URI> syntax rule (Section 4.3).  If the
   base URI is obtained from a URI reference, then that reference must
   be converted to absolute form and stripped of any fragment component
   prior to its use as a base URI.






Berners-Lee, et al.         Standards Track                    [Page 28]
```

<a id="source-page-29-part-1"></a>

## Source page 29

```text

RFC 3986                   URI Generic Syntax               January 2005


   The base URI of a reference can be established in one of four ways,
   discussed below in order of precedence.  The order of precedence can
   be thought of in terms of layers, where the innermost defined base
   URI has the highest precedence.  This can be visualized graphically
   as follows:

         .----------------------------------------------------------.
         |  .----------------------------------------------------.  |
         |  |  .----------------------------------------------.  |  |
         |  |  |  .----------------------------------------.  |  |  |
         |  |  |  |  .----------------------------------.  |  |  |  |
         |  |  |  |  |       <relative-reference>       |  |  |  |  |
         |  |  |  |  `----------------------------------'  |  |  |  |
         |  |  |  | (5.1.1) Base URI embedded in content   |  |  |  |
         |  |  |  `----------------------------------------'  |  |  |
         |  |  | (5.1.2) Base URI of the encapsulating entity |  |  |
         |  |  |         (message, representation, or none)   |  |  |
         |  |  `----------------------------------------------'  |  |
         |  | (5.1.3) URI used to retrieve the entity            |  |
         |  `----------------------------------------------------'  |
         | (5.1.4) Default Base URI (application-dependent)         |
         `----------------------------------------------------------'

5.1.1.  Base URI Embedded in Content

   Within certain media types, a base URI for relative references can be
   embedded within the content itself so that it can be readily obtained
   by a parser.  This can be useful for descriptive documents, such as
   tables of contents, which may be transmitted to others through
   protocols other than their usual retrieval context (e.g., email or
   USENET news).

   It is beyond the scope of this specification to specify how, for each
   media type, a base URI can be embedded.  The appropriate syntax, when
   available, is described by the data format specification associated
   with each media type.

5.1.2.  Base URI from the Encapsulating Entity

   If no base URI is embedded, the base URI is defined by the
   representation's retrieval context.  For a document that is enclosed
   within another entity, such as a message or archive, the retrieval
   context is that entity.  Thus, the default base URI of a
   representation is the base URI of the entity in which the
   representation is encapsulated.






Berners-Lee, et al.         Standards Track                    [Page 29]
```

<a id="source-page-30-part-1"></a>

## Source page 30

```text

RFC 3986                   URI Generic Syntax               January 2005


   A mechanism for embedding a base URI within MIME container types
   (e.g., the message and multipart types) is defined by MHTML
   [RFC2557].  Protocols that do not use the MIME message header syntax,
   but that do allow some form of tagged metadata to be included within
   messages, may define their own syntax for defining a base URI as part
   of a message.

5.1.3.  Base URI from the Retrieval URI

   If no base URI is embedded and the representation is not encapsulated
   within some other entity, then, if a URI was used to retrieve the
   representation, that URI shall be considered the base URI.  Note that
   if the retrieval was the result of a redirected request, the last URI
   used (i.e., the URI that resulted in the actual retrieval of the
   representation) is the base URI.

5.1.4.  Default Base URI

   If none of the conditions described above apply, then the base URI is
   defined by the context of the application.  As this definition is
   necessarily application-dependent, failing to define a base URI by
   using one of the other methods may result in the same content being
   interpreted differently by different types of applications.

   A sender of a representation containing relative references is
   responsible for ensuring that a base URI for those references can be
   established.  Aside from fragment-only references, relative
   references can only be used reliably in situations where the base URI
   is well defined.

5.2.  Relative Resolution

   This section describes an algorithm for converting a URI reference
   that might be relative to a given base URI into the parsed components
   of the reference's target.  The components can then be recomposed, as
   described in Section 5.3, to form the target URI.  This algorithm
   provides definitive results that can be used to test the output of
   other implementations.  Applications may implement relative reference
   resolution by using some other algorithm, provided that the results
   match what would be given by this one.











Berners-Lee, et al.         Standards Track                    [Page 30]
```

<a id="source-page-31-part-1"></a>

## Source page 31

```text

RFC 3986                   URI Generic Syntax               January 2005


5.2.1.  Pre-parse the Base URI

   The base URI (Base) is established according to the procedure of
   Section 5.1 and parsed into the five main components described in
   Section 3.  Note that only the scheme component is required to be
   present in a base URI; the other components may be empty or
   undefined.  A component is undefined if its associated delimiter does
   not appear in the URI reference; the path component is never
   undefined, though it may be empty.

   Normalization of the base URI, as described in Sections 6.2.2 and
   6.2.3, is optional.  A URI reference must be transformed to its
   target URI before it can be normalized.

5.2.2.  Transform References

   For each URI reference (R), the following pseudocode describes an
   algorithm for transforming R into its target URI (T):

      -- The URI reference is parsed into the five URI components
      --
      (R.scheme, R.authority, R.path, R.query, R.fragment) = parse(R);

      -- A non-strict parser may ignore a scheme in the reference
      -- if it is identical to the base URI's scheme.
      --
      if ((not strict) and (R.scheme == Base.scheme)) then
         undefine(R.scheme);
      endif;






















Berners-Lee, et al.         Standards Track                    [Page 31]
```

