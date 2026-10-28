# Uniform Resource Identifier (URI): Generic Syntax — part 2

Source: [Uniform Resource Identifier (URI): Generic Syntax](https://www.rfc-editor.org/rfc/rfc3986.txt) · RFC 3986.

[Document index](../README.md) · [Topic guide](../../../../README.md) · [Publisher](https://www.rfc-editor.org/rfc/rfc3986.txt).

Verbatim text slices in fixed-width blocks preserve grammar, examples, and diagrams.

[Previous](part-001.md) · [Next](part-003.md)

<a id="source-page-6-part-1"></a>

## Source page 6

```text

RFC 3986                   URI Generic Syntax               January 2005


   specification prevents an application from limiting itself to
   particular types of resources, or to a subset of URIs that maintains
   characteristics desired by that application.

   URIs have a global scope and are interpreted consistently regardless
   of context, though the result of that interpretation may be in
   relation to the end-user's context.  For example, "http://localhost/"
   has the same interpretation for every user of that reference, even
   though the network interface corresponding to "localhost" may be
   different for each end-user: interpretation is independent of access.
   However, an action made on the basis of that reference will take
   place in relation to the end-user's context, which implies that an
   action intended to refer to a globally unique thing must use a URI
   that distinguishes that resource from all other things.  URIs that
   identify in relation to the end-user's local context should only be
   used when the context itself is a defining aspect of the resource,
   such as when an on-line help manual refers to a file on the end-
   user's file system (e.g., "file:///etc/hosts").

1.1.1.  Generic Syntax

   Each URI begins with a scheme name, as defined in Section 3.1, that
   refers to a specification for assigning identifiers within that
   scheme.  As such, the URI syntax is a federated and extensible naming
   system wherein each scheme's specification may further restrict the
   syntax and semantics of identifiers using that scheme.

   This specification defines those elements of the URI syntax that are
   required of all URI schemes or are common to many URI schemes.  It
   thus defines the syntax and semantics needed to implement a scheme-
   independent parsing mechanism for URI references, by which the
   scheme-dependent handling of a URI can be postponed until the
   scheme-dependent semantics are needed.  Likewise, protocols and data
   formats that make use of URI references can refer to this
   specification as a definition for the range of syntax allowed for all
   URIs, including those schemes that have yet to be defined.  This
   decouples the evolution of identification schemes from the evolution
   of protocols, data formats, and implementations that make use of
   URIs.

   A parser of the generic URI syntax can parse any URI reference into
   its major components.  Once the scheme is determined, further
   scheme-specific parsing can be performed on the components.  In other
   words, the URI generic syntax is a superset of the syntax of all URI
   schemes.






Berners-Lee, et al.         Standards Track                     [Page 6]
```

<a id="source-page-7-part-1"></a>

## Source page 7

```text

RFC 3986                   URI Generic Syntax               January 2005


1.1.2.  Examples

   The following example URIs illustrate several URI schemes and
   variations in their common syntax components:

      ftp://ftp.is.co.za/rfc/rfc1808.txt

      http://www.ietf.org/rfc/rfc2396.txt

      ldap://[2001:db8::7]/c=GB?objectClass?one

      mailto:John.Doe@example.com

      news:comp.infosystems.www.servers.unix

      tel:+1-816-555-1212

      telnet://192.0.2.16:80/

      urn:oasis:names:specification:docbook:dtd:xml:4.1.2


1.1.3.  URI, URL, and URN

   A URI can be further classified as a locator, a name, or both.  The
   term "Uniform Resource Locator" (URL) refers to the subset of URIs
   that, in addition to identifying a resource, provide a means of
   locating the resource by describing its primary access mechanism
   (e.g., its network "location").  The term "Uniform Resource Name"
   (URN) has been used historically to refer to both URIs under the
   "urn" scheme [RFC2141], which are required to remain globally unique
   and persistent even when the resource ceases to exist or becomes
   unavailable, and to any other URI with the properties of a name.

   An individual scheme does not have to be classified as being just one
   of "name" or "locator".  Instances of URIs from any given scheme may
   have the characteristics of names or locators or both, often
   depending on the persistence and care in the assignment of
   identifiers by the naming authority, rather than on any quality of
   the scheme.  Future specifications and related documentation should
   use the general term "URI" rather than the more restrictive terms
   "URL" and "URN" [RFC3305].









Berners-Lee, et al.         Standards Track                     [Page 7]
```

<a id="source-page-8-part-1"></a>

## Source page 8

```text

RFC 3986                   URI Generic Syntax               January 2005


1.2.  Design Considerations

1.2.1.  Transcription

   The URI syntax has been designed with global transcription as one of
   its main considerations.  A URI is a sequence of characters from a
   very limited set: the letters of the basic Latin alphabet, digits,
   and a few special characters.  A URI may be represented in a variety
   of ways; e.g., ink on paper, pixels on a screen, or a sequence of
   character encoding octets.  The interpretation of a URI depends only
   on the characters used and not on how those characters are
   represented in a network protocol.

   The goal of transcription can be described by a simple scenario.
   Imagine two colleagues, Sam and Kim, sitting in a pub at an
   international conference and exchanging research ideas.  Sam asks Kim
   for a location to get more information, so Kim writes the URI for the
   research site on a napkin.  Upon returning home, Sam takes out the
   napkin and types the URI into a computer, which then retrieves the
   information to which Kim referred.

   There are several design considerations revealed by the scenario:

   o  A URI is a sequence of characters that is not always represented
      as a sequence of octets.

   o  A URI might be transcribed from a non-network source and thus
      should consist of characters that are most likely able to be
      entered into a computer, within the constraints imposed by
      keyboards (and related input devices) across languages and
      locales.

   o  A URI often has to be remembered by people, and it is easier for
      people to remember a URI when it consists of meaningful or
      familiar components.

   These design considerations are not always in alignment.  For
   example, it is often the case that the most meaningful name for a URI
   component would require characters that cannot be typed into some
   systems.  The ability to transcribe a resource identifier from one
   medium to another has been considered more important than having a
   URI consist of the most meaningful of components.

   In local or regional contexts and with improving technology, users
   might benefit from being able to use a wider range of characters;
   such use is not defined by this specification.  Percent-encoded
   octets (Section 2.1) may be used within a URI to represent characters
   outside the range of the US-ASCII coded character set if this



Berners-Lee, et al.         Standards Track                     [Page 8]
```

<a id="source-page-9-part-1"></a>

## Source page 9

```text

RFC 3986                   URI Generic Syntax               January 2005


   representation is allowed by the scheme or by the protocol element in
   which the URI is referenced.  Such a definition should specify the
   character encoding used to map those characters to octets prior to
   being percent-encoded for the URI.

1.2.2.  Separating Identification from Interaction

   A common misunderstanding of URIs is that they are only used to refer
   to accessible resources.  The URI itself only provides
   identification; access to the resource is neither guaranteed nor
   implied by the presence of a URI.  Instead, any operation associated
   with a URI reference is defined by the protocol element, data format
   attribute, or natural language text in which it appears.

   Given a URI, a system may attempt to perform a variety of operations
   on the resource, as might be characterized by words such as "access",
   "update", "replace", or "find attributes".  Such operations are
   defined by the protocols that make use of URIs, not by this
   specification.  However, we do use a few general terms for describing
   common operations on URIs.  URI "resolution" is the process of
   determining an access mechanism and the appropriate parameters
   necessary to dereference a URI; this resolution may require several
   iterations.  To use that access mechanism to perform an action on the
   URI's resource is to "dereference" the URI.

   When URIs are used within information retrieval systems to identify
   sources of information, the most common form of URI dereference is
   "retrieval": making use of a URI in order to retrieve a
   representation of its associated resource.  A "representation" is a
   sequence of octets, along with representation metadata describing
   those octets, that constitutes a record of the state of the resource
   at the time when the representation is generated.  Retrieval is
   achieved by a process that might include using the URI as a cache key
   to check for a locally cached representation, resolution of the URI
   to determine an appropriate access mechanism (if any), and
   dereference of the URI for the sake of applying a retrieval
   operation.  Depending on the protocols used to perform the retrieval,
   additional information might be supplied about the resource (resource
   metadata) and its relation to other resources.

   URI references in information retrieval systems are designed to be
   late-binding: the result of an access is generally determined when it
   is accessed and may vary over time or due to other aspects of the
   interaction.  These references are created in order to be used in the
   future: what is being identified is not some specific result that was
   obtained in the past, but rather some characteristic that is expected
   to be true for future results.  In such cases, the resource referred
   to by the URI is actually a sameness of characteristics as observed



Berners-Lee, et al.         Standards Track                     [Page 9]
```

<a id="source-page-10-part-1"></a>

## Source page 10

```text

RFC 3986                   URI Generic Syntax               January 2005


   over time, perhaps elucidated by additional comments or assertions
   made by the resource provider.

   Although many URI schemes are named after protocols, this does not
   imply that use of these URIs will result in access to the resource
   via the named protocol.  URIs are often used simply for the sake of
   identification.  Even when a URI is used to retrieve a representation
   of a resource, that access might be through gateways, proxies,
   caches, and name resolution services that are independent of the
   protocol associated with the scheme name.  The resolution of some
   URIs may require the use of more than one protocol (e.g., both DNS
   and HTTP are typically used to access an "http" URI's origin server
   when a representation isn't found in a local cache).

1.2.3.  Hierarchical Identifiers

   The URI syntax is organized hierarchically, with components listed in
   order of decreasing significance from left to right.  For some URI
   schemes, the visible hierarchy is limited to the scheme itself:
   everything after the scheme component delimiter (":") is considered
   opaque to URI processing.  Other URI schemes make the hierarchy
   explicit and visible to generic parsing algorithms.

   The generic syntax uses the slash ("/"), question mark ("?"), and
   number sign ("#") characters to delimit components that are
   significant to the generic parser's hierarchical interpretation of an
   identifier.  In addition to aiding the readability of such
   identifiers through the consistent use of familiar syntax, this
   uniform representation of hierarchy across naming schemes allows
   scheme-independent references to be made relative to that hierarchy.

   It is often the case that a group or "tree" of documents has been
   constructed to serve a common purpose, wherein the vast majority of
   URI references in these documents point to resources within the tree
   rather than outside it.  Similarly, documents located at a particular
   site are much more likely to refer to other resources at that site
   than to resources at remote sites.  Relative referencing of URIs
   allows document trees to be partially independent of their location
   and access scheme.  For instance, it is possible for a single set of
   hypertext documents to be simultaneously accessible and traversable
   via each of the "file", "http", and "ftp" schemes if the documents
   refer to each other with relative references.  Furthermore, such
   document trees can be moved, as a whole, without changing any of the
   relative references.

   A relative reference (Section 4.2) refers to a resource by describing
   the difference within a hierarchical name space between the reference
   context and the target URI.  The reference resolution algorithm,



Berners-Lee, et al.         Standards Track                    [Page 10]
```

