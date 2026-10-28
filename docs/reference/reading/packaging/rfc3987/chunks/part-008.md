# Internationalized Resource Identifiers (IRIs) — part 8

Source: [Internationalized Resource Identifiers (IRIs)](https://www.rfc-editor.org/rfc/rfc3987.txt) · RFC 3987.

[Document index](../README.md) · [Topic guide](../../../../README.md) · [Publisher](https://www.rfc-editor.org/rfc/rfc3987.txt).

Verbatim text slices in fixed-width blocks preserve grammar, examples, and diagrams.

[Previous](part-007.md) · [Next](part-009.md)

<a id="source-page-37-part-1"></a>

## Source page 37

```text

RFC 3987         Internationalized Resource Identifiers     January 2005


   Therefore, IRIs benefit most from a broad upgrade of software to be
   able to enter and transport IRIs.  However, before an individual IRI
   is published, care should be taken to upgrade the corresponding
   interpreting software in order to cover the forms expected to be
   received by various versions of entry and transport software.

   The upgrade of generating software to generate IRIs instead of using
   a local character encoding should happen only after the service is
   upgraded to accept IRIs.  Similarly, IRIs should only be generated
   when the service accepts IRIs and the intervening infrastructure and
   protocol is known to transport them safely.

   Software converting from URIs to IRIs for display should be upgraded
   only after upgraded entry software has been widely deployed to the
   population that will see the displayed result.

   Where there is a free choice of character encodings, it is often
   possible to reduce the effort and dependencies for upgrading to IRIs
   by using UTF-8 rather than another encoding.  For example, when a new
   file-based Web server is set up, using UTF-8 as the character
   encoding for file names will make the transition to IRIs easier.
   Likewise, when a new Web form is set up using UTF-8 as the character
   encoding of the form page, the returned query URIs will use UTF-8 as
   the character encoding (unless the user, for whatever reason, changes
   the character encoding) and will therefore be compatible with IRIs.

   These recommendations, when taken together, will allow for the
   extension from URIs to IRIs in order to handle characters other than
   US-ASCII while minimizing interoperability problems.  For
   considerations regarding the upgrade of URI scheme definitions, see
   section 6.4.

8.  Security Considerations

   The security considerations discussed in [RFC3986] also apply to
   IRIs.  In addition, the following issues require particular care for
   IRIs.

   Incorrect encoding or decoding can lead to security problems.  In
   particular, some UTF-8 decoders do not check against overlong byte
   sequences.  As an example, a "/" is encoded with the byte 0x2F both
   in UTF-8 and in US-ASCII, but some UTF-8 decoders also wrongly
   interpret the sequence 0xC0 0xAF as a "/".  A sequence such as








Duerst & Suignard           Standards Track                    [Page 37]
```

<a id="source-page-38-part-1"></a>

## Source page 38

```text

RFC 3987         Internationalized Resource Identifiers     January 2005


   "%C0%AF.." may pass some security tests and then be interpreted as
   "/.." in a path if UTF-8 decoders are fault-tolerant, if conversion
   and checking are not done in the right order, and/or if reserved
   characters and unreserved characters are not clearly distinguished.

   There are various ways in which "spoofing" can occur with IRIs.
   "Spoofing" means that somebody may add a resource name that looks the
   same or similar to the user, but that points to a different resource.
   The added resource may pretend to be the real resource by looking
   very similar but may contain all kinds of changes that may be
   difficult to spot and that can cause all kinds of problems.  Most
   spoofing possibilities for IRIs are extensions of those for URIs.

   Spoofing can occur for various reasons.  First, a user's
   normalization expectations or actual normalization when entering an
   IRI or transcoding an IRI from a legacy character encoding do not
   match the normalization used on the server side.  Conceptually, this
   is no different from the problems surrounding the use of
   case-insensitive web servers.  For example, a popular web page with a
   mixed-case name ("http://big.example.com/PopularPage.html") might be
   "spoofed" by someone who is able to create
   "http://big.example.com/popularpage.html".  However, the use of
   unnormalized character sequences, and of additional mappings for user
   convenience, may increase the chance for spoofing.  Protocols and
   servers that allow the creation of resources with names that are not
   normalized are particularly vulnerable to such attacks.  This is an
   inherent security problem of the relevant protocol, server, or
   resource and is not specific to IRIs, but it is mentioned here for
   completeness.

   Spoofing can occur in various IRI components, such as the domain name
   part or a path part.  For considerations specific to the domain name
   part, see [RFC3491].  For the path part, administrators of sites that
   allow independent users to create resources in the same sub area may
   have to be careful to check for spoofing.

   Spoofing can occur because in the UCS many characters look very
   similar.  Details are discussed in Section 7.5.  Again, this is very
   similar to spoofing possibilities on US-ASCII, e.g., using "br0ken"
   or "1ame" URIs.

   Spoofing can occur when URIs with percent-encodings based on various
   character encodings are accepted to deal with older user agents.  In
   some cases, particularly for Latin-based resource names, this is
   usually easy to detect because UTF-8-encoded names, when interpreted
   and viewed as legacy character encodings, produce mostly garbage.





Duerst & Suignard           Standards Track                    [Page 38]
```

<a id="source-page-39-part-1"></a>

## Source page 39

```text

RFC 3987         Internationalized Resource Identifiers     January 2005


   When concurrently used character encodings have a similar structure
   but there are no characters that have exactly the same encoding,
   detection is more difficult.

   Spoofing can occur with bidirectional IRIs, if the restrictions in
   section 4.2 are not followed.  The same visual representation may be
   interpreted as different logical representations, and vice versa.  It
   is also very important that a correct Unicode bidirectional
   implementation be used.

9.  Acknowledgements

   We would like to thank Larry Masinter for his work as coauthor of
   many earlier versions of this document (draft-masinter-url-i18n-xx).

   The discussion on the issue addressed here started a long time ago.
   There was a thread in the HTML working group in August 1995 (under
   the topic of "Globalizing URIs") and in the www-international mailing
   list in July 1996 (under the topic of "Internationalization and
   URLs"), and there were ad-hoc meetings at the Unicode conferences in
   September 1995 and September 1997.

   Many thanks go to Francois Yergeau, Matitiahu Allouche, Roy Fielding,
   Tim Berners-Lee, Mark Davis, M.T. Carrasco Benitez, James Clark, Tim
   Bray, Chris Wendt, Yaron Goland, Andrea Vine, Misha Wolf, Leslie
   Daigle, Ted Hardie, Bill Fenner, Margaret Wasserman, Russ Housley,
   Makoto MURATA, Steven Atkin, Ryan Stansifer, Tex Texin, Graham Klyne,
   Bjoern Hoehrmann, Chris Lilley, Ian Jacobs, Adam Costello, Dan
   Oscarson, Elliotte Rusty Harold, Mike J. Brown, Roy Badami, Jonathan
   Rosenne, Asmus Freytag, Simon Josefsson, Carlos Viegas Damasio, Chris
   Haynes, Walter Underwood, and many others for help with understanding
   the issues and possible solutions, and with getting the details
   right.

   This document is a product of the Internationalization Working Group
   (I18N WG) of the World Wide Web Consortium (W3C).  Thanks to the
   members of the W3C I18N Working Group and Interest Group for their
   contributions and their work on [CharMod].  Thanks also go to the
   members of many other W3C Working Groups for adopting IRIs, and to
   the members of the Montreal IAB Workshop on Internationalization and
   Localization for their review.










Duerst & Suignard           Standards Track                    [Page 39]
```

<a id="source-page-40-part-1"></a>

## Source page 40

```text

RFC 3987         Internationalized Resource Identifiers     January 2005


10.  References

10.1.  Normative References

   [ASCII]        American National Standards Institute, "Coded
                  Character Set -- 7-bit American Standard Code for
                  Information Interchange", ANSI X3.4, 1986.

   [ISO10646]     International Organization for Standardization,
                  "ISO/IEC 10646:2003: Information Technology -
                  Universal Multiple-Octet Coded Character Set (UCS)",
                  ISO Standard 10646, December 2003.

   [RFC2119]      Bradner, S., "Key words for use in RFCs to Indicate
                  Requirement Levels", BCP 14, RFC 2119, March 1997.

   [RFC2234]      Crocker, D. and P. Overell, "Augmented BNF for Syntax
                  Specifications: ABNF", RFC 2234, November 1997.

   [RFC3490]      Faltstrom, P., Hoffman, P., and A. Costello,
                  "Internationalizing Domain Names in Applications
                  (IDNA)", RFC 3490, March 2003.

   [RFC3491]      Hoffman, P. and M. Blanchet, "Nameprep: A Stringprep
                  Profile for Internationalized Domain Names (IDN)", RFC
                  3491, March 2003.

   [RFC3629]      Yergeau, F., "UTF-8, a transformation format of ISO
                  10646", STD 63, RFC 3629, November 2003.

   [RFC3986]      Berners-Lee, T., Fielding, R., and L. Masinter,
                  "Uniform Resource Identifier (URI): Generic Syntax",
                  STD 66, RFC 3986, January 2005.

   [UNI9]         Davis, M., "The Bidirectional Algorithm", Unicode
                  Standard Annex #9, March 2004,
                  <http://www.unicode.org/reports/tr9/tr9-13.html>.

   [UNIV4]        The Unicode Consortium, "The Unicode Standard, Version
                  4.0.1, defined by: The Unicode Standard, Version 4.0
                  (Reading, MA, Addison-Wesley, 2003. ISBN
                  0-321-18578-1), as amended by Unicode 4.0.1
                  (http://www.unicode.org/versions/Unicode4.0.1/)",
                  March 2004.







Duerst & Suignard           Standards Track                    [Page 40]
```

<a id="source-page-41-part-1"></a>

## Source page 41

```text

RFC 3987         Internationalized Resource Identifiers     January 2005


   [UTR15]        Davis, M. and M. Duerst, "Unicode Normalization
                  Forms", Unicode Standard Annex #15, April 2003,
                  <http://www.unicode.org/unicode/reports/
                  tr15/tr15-23.html>.

10.2.  Informative References

   [BidiEx]       "Examples of bidirectional IRIs",
                  <http://www.w3.org/International/iri-edit/
                  BidiExamples>.

   [CharMod]      Duerst, M., Yergeau, F., Ishida, R., Wolf, M., and T.
                  Texin, "Character Model for the World Wide Web:
                  Resource Identifiers", World Wide Web Consortium
                  Candidate Recommendation, November 2004,
                  <http://www.w3.org/TR/charmod-resid>.

   [Duerst97]     Duerst, M., "The Properties and Promises of UTF-8",
                  Proc.  11th International Unicode Conference, San Jose
                  , September 1997,
                  <http://www.ifi.unizh.ch/mml/mduerst/papers/
                  PDF/IUC11-UTF-8.pdf>.

   [Gettys]       Gettys, J., "URI Model Consequences",
                  <http://www.w3.org/DesignIssues/ModelConsequences>.

   [HTML4]        Raggett, D., Le Hors, A., and I. Jacobs, "HTML 4.01
                  Specification", World Wide Web Consortium
                  Recommendation, December 1999,
                  <http://www.w3.org/TR/html401/appendix/
                  notes.html#h-B.2>.

   [RFC2045]      Freed, N. and N. Borenstein, "Multipurpose Internet
                  Mail Extensions (MIME) Part One: Format of Internet
                  Message Bodies", RFC 2045, November 1996.

   [RFC2130]      Weider, C., Preston, C., Simonsen, K., Alvestrand, H.,
                  Atkinson, R., Crispin, M., and P. Svanberg, "The
                  Report of the IAB Character Set Workshop held 29
                  February - 1 March, 1996", RFC 2130, April 1997.

   [RFC2141]      Moats, R., "URN Syntax", RFC 2141, May 1997.

   [RFC2192]      Newman, C., "IMAP URL Scheme", RFC 2192, September
                  1997.

   [RFC2277]      Alvestrand, H., "IETF Policy on Character Sets and
                  Languages", BCP 18, RFC 2277, January 1998.



Duerst & Suignard           Standards Track                    [Page 41]
```

