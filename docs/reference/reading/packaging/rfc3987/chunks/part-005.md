# Internationalized Resource Identifiers (IRIs) — part 5

Source: [Internationalized Resource Identifiers (IRIs)](https://www.rfc-editor.org/rfc/rfc3987.txt) · RFC 3987.

[Document index](../README.md) · [Topic guide](../../../../README.md) · [Publisher](https://www.rfc-editor.org/rfc/rfc3987.txt).

Verbatim text slices in fixed-width blocks preserve grammar, examples, and diagrams.

[Previous](part-004.md) · [Next](part-006.md)

<a id="source-page-22-part-1"></a>

## Source page 22

```text

RFC 3987         Internationalized Resource Identifiers     January 2005


   IRI comparison is performed for some particular purpose.  Protocols
   or implementations that compare IRIs for different purposes will
   often be subject to differing design trade-offs in regards to how
   much effort should be spent in reducing aliased identifiers.  This
   section describes various methods that may be used to compare IRIs,
   the trade-offs between them, and the types of applications that might
   use them.

5.1.  Equivalence

   Because IRIs exist to identify resources, presumably they should be
   considered equivalent when they identify the same resource.  However,
   this definition of equivalence is not of much practical use, as there
   is no way for an implementation to compare two resources unless it
   has full knowledge or control of them. For this reason, determination
   of equivalence or difference of IRIs is based on string comparison,
   perhaps augmented by reference to additional rules provided by URI
   scheme definitions.  We use the terms "different" and "equivalent" to
   describe the possible outcomes of such comparisons, but there are
   many application-dependent versions of equivalence.

   Even though it is possible to determine that two IRIs are equivalent,
   IRI comparison is not sufficient to determine whether two IRIs
   identify different resources.  For example, an owner of two different
   domain names could decide to serve the same resource from both,
   resulting in two different IRIs.  Therefore, comparison methods are
   designed to minimize false negatives while strictly avoiding false
   positives.

   In testing for equivalence, applications should not directly compare
   relative references; the references should be converted to their
   respective target IRIs before comparison.  When IRIs are compared to
   select (or avoid) a network action, such as retrieval of a
   representation, fragment components (if any) should be excluded from
   the comparison.

   Applications using IRIs as identity tokens with no relationship to a
   protocol MUST use the Simple String Comparison (see section 5.3.1).
   All other applications MUST select one of the comparison practices
   from the Comparison Ladder (see section 5.3 or, after IRI-to-URI
   conversion, select one of the comparison practices from the URI
   comparison ladder in [RFC3986], section 6.2)

5.2.  Preparation for Comparison

   Any kind of IRI comparison REQUIRES that all escapings or encodings
   in the protocol or format that carries an IRI are resolved.  This is
   usually done when the protocol or format is parsed.  Examples of such



Duerst & Suignard           Standards Track                    [Page 22]
```

<a id="source-page-23-part-1"></a>

## Source page 23

```text

RFC 3987         Internationalized Resource Identifiers     January 2005


   escapings or encodings are entities and numeric character references
   in [HTML4] and [XML1].  As an example,
   "http://example.org/ros&eacute;" (in HTML),
   "http://example.org/ros&#233"; (in HTML or XML), and
   "http://example.org/ros&#xE9"; (in HTML or XML) are all resolved into
   what is denoted in this document (see section 1.4) as
   "http://example.org/ros&#xE9"; (the "&#xE9;" here standing for the
   actual e-acute character, to compensate for the fact that this
   document cannot contain non-ASCII characters).

   Similar considerations apply to encodings such as Transfer Codings in
   HTTP (see [RFC2616]) and Content Transfer Encodings in MIME
   ([RFC2045]), although in these cases, the encoding is based not on
   characters but on octets, and additional care is required to make
   sure that characters, and not just arbitrary octets, are compared
   (see section 5.3.1).

5.3.  Comparison Ladder

   In practice, a variety of methods are used, to test IRI equivalence.
   These methods fall into a range distinguished by the amount of
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

5.3.1.  Simple String Comparison

   If two IRIs, when considered as character strings, are identical,
   then it is safe to conclude that they are equivalent.  This type of
   equivalence test has very low computational cost and is in wide use
   in a variety of applications, particularly in the domain of parsing.
   It is also used when a definitive answer to the question of IRI
   equivalence is needed that is independent of the scheme used and that
   can be calculated quickly and without accessing a network.  An
   example of such a case is XML Namespaces ([XMLNamespace]).

   Testing strings for equivalence requires some basic precautions. This
   procedure is often referred to as "bit-for-bit" or "byte-for-byte"
   comparison, which is potentially misleading.  Testing strings for
   equality is normally based on pair comparison of the characters that



Duerst & Suignard           Standards Track                    [Page 23]
```

<a id="source-page-24-part-1"></a>

## Source page 24

```text

RFC 3987         Internationalized Resource Identifiers     January 2005


   make up the strings, starting from the first and proceeding until
   both strings are exhausted and all characters are found to be equal,
   until a pair of characters compares unequal, or until one of the
   strings is exhausted before the other.

   This character comparison requires that each pair of characters be
   put in comparable encoding form.  For example, should one IRI be
   stored in a byte array in UTF-8 encoding form and the second in a
   UTF-16 encoding form, bit-for-bit comparisons applied naively will
   produce errors.  It is better to speak of equality on a
   character-for-character rather than on a byte-for-byte or bit-for-bit
   basis.  In practical terms, character-by-character comparisons should
   be done codepoint by codepoint after conversion to a common character
   encoding form.  When comparing character by character, the comparison
   function MUST NOT map IRIs to URIs, because such a mapping would
   create additional spurious equivalences.  It follows that an IRI
   SHOULD NOT be modified when being transported if there is any chance
   that this IRI might be used as an identifier.

   False negatives are caused by the production and use of IRI aliases.
   Unnecessary aliases can be reduced, regardless of the comparison
   method, by consistently providing IRI references in an already
   normalized form (i.e., a form identical to what would be produced
   after normalization is applied, as described below). Protocols and
   data formats often limit some IRI comparisons to simple string
   comparison, based on the theory that people and implementations will,
   in their own best interest, be consistent in providing IRI
   references, or at least be consistent enough to negate any efficiency
   that might be obtained from further normalization.

5.3.2.  Syntax-Based Normalization

   Implementations may use logic based on the definitions provided by
   this specification to reduce the probability of false negatives. This
   processing is moderately higher in cost than character-for-character
   string comparison.  For example, an application using this approach
   could reasonably consider the following two IRIs equivalent:

      example://a/b/c/%7Bfoo%7D/ros&#xE9;
      eXAMPLE://a/./b/../b/%63/%7bfoo%7d/ros%C3%A9

   Web user agents, such as browsers, typically apply this type of IRI
   normalization when determining whether a cached response is
   available.  Syntax-based normalization includes such techniques as
   case normalization, character normalization, percent-encoding
   normalization, and removal of dot-segments.





Duerst & Suignard           Standards Track                    [Page 24]
```

<a id="source-page-25-part-1"></a>

## Source page 25

```text

RFC 3987         Internationalized Resource Identifiers     January 2005


5.3.2.1.  Case Normalization

   For all IRIs, the hexadecimal digits within a percent-encoding
   triplet (e.g., "%3a" versus "%3A") are case-insensitive and therefore
   should be normalized to use uppercase letters for the digits A - F.

   When an IRI uses components of the generic syntax, the component
   syntax equivalence rules always apply; namely, that the scheme and
   US-ASCII only host are case insensitive and therefore should be
   normalized to lowercase.  For example, the URI
   "HTTP://www.EXAMPLE.com/" is equivalent to "http://www.example.com/".
   Case equivalence for non-ASCII characters in IRI components that are
   IDNs are discussed in section 5.3.3.  The other generic syntax
   components are assumed to be case sensitive unless specifically
   defined otherwise by the scheme.

   Creating schemes that allow case-insensitive syntax components
   containing non-ASCII characters should be avoided. Case normalization
   of non-ASCII characters can be culturally dependent and is always a
   complex operation.  The only exception concerns non-ASCII host names
   for which the character normalization includes a mapping step derived
   from case folding.

5.3.2.2.  Character Normalization

   The Unicode Standard [UNIV4] defines various equivalences between
   sequences of characters for various purposes.  Unicode Standard Annex
   #15 [UTR15] defines various Normalization Forms for these
   equivalences, in particular Normalization Form C (NFC, Canonical
   Decomposition, followed by Canonical Composition) and Normalization
   Form KC (NFKC, Compatibility Decomposition, followed by Canonical
   Composition).

   Equivalence of IRIs MUST rely on the assumption that IRIs are
   appropriately pre-character-normalized rather than apply character
   normalization when comparing two IRIs.  The exceptions are conversion
   from a non-digital form, and conversion from a non-UCS-based
   character encoding to a UCS-based character encoding. In these cases,
   NFC or a normalizing transcoder using NFC MUST be used for
   interoperability.  To avoid false negatives and problems with
   transcoding, IRIs SHOULD be created by using NFC.  Using NFKC may
   avoid even more problems; for example, by choosing half-width Latin
   letters instead of full-width ones, and full-width instead of
   half-width Katakana.

   As an example, "http://www.example.org/r&#xE9;sum&#xE9;.html" (in XML
   Notation) is in NFC.  On the other hand,
   "http://www.example.org/re&#x301;sume&#x301;.html" is not in NFC.



Duerst & Suignard           Standards Track                    [Page 25]
```

<a id="source-page-26-part-1"></a>

## Source page 26

```text

RFC 3987         Internationalized Resource Identifiers     January 2005


   The former uses precombined e-acute characters, and the latter uses
   "e" characters followed by combining acute accents.  Both usages are
   defined as canonically equivalent in [UNIV4].

   Note: Because it is unknown how a particular sequence of characters
      is being treated with respect to character normalization, it would
      be inappropriate to allow third parties to normalize an IRI
      arbitrarily.  This does not contradict the recommendation that
      when a resource is created, its IRI should be as character
      normalized as possible (i.e., NFC or even NFKC).  This is similar
      to the uppercase/lowercase problems.  Some parts of a URI are case
      insensitive (domain name).  For others, it is unclear whether they
      are case sensitive, case insensitive, or something in between
      (e.g., case sensitive, but with a multiple choice selection if the
      wrong case is used, instead of a direct negative result).  The
      best recipe is that the creator use a reasonable capitalization
      and, when transferring the URI, capitalization never be changed.

   Various IRI schemes may allow the usage of Internationalized Domain
   Names (IDN) [RFC3490] either in the ireg-name part or elsewhere.
   Character Normalization also applies to IDNs, as discussed in section
   5.3.3.

5.3.2.3.  Percent-Encoding Normalization

   The percent-encoding mechanism (section 2.1 of [RFC3986]) is a
   frequent source of variance among otherwise identical IRIs.  In
   addition to the case normalization issue noted above, some IRI
   producers percent-encode octets that do not require percent-encoding,
   resulting in IRIs that are equivalent to their non encoded
   counterparts.  These IRIs should be normalized by decoding any
   percent-encoded octet sequence that corresponds to an unreserved
   character, as described in section 2.3 of [RFC3986].

   For actual resolution, differences in percent-encoding (except for
   the percent-encoding of reserved characters) MUST always result in
   the same resource.  For example, "http://example.org/~user",
   "http://example.org/%7euser", and "http://example.org/%7Euser", must
   resolve to the same resource.

   If this kind of equivalence is to be tested, the percent-encoding of
   both IRIs to be compared has to be aligned; for example, by
   converting both IRIs to URIs (see section 3.1), eliminating escape
   differences in the resulting URIs, and making sure that the case of
   the hexadecimal characters in the percent-encoding is always the same
   (preferably uppercase).  If the IRI is to be passed to another





Duerst & Suignard           Standards Track                    [Page 26]
```

