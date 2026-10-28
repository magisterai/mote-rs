# Internationalized Resource Identifiers (IRIs) — part 7

Source: [Internationalized Resource Identifiers (IRIs)](https://www.rfc-editor.org/rfc/rfc3987.txt) · RFC 3987.

[Document index](../README.md) · [Topic guide](../../../../README.md) · [Publisher](https://www.rfc-editor.org/rfc/rfc3987.txt).

Verbatim text slices in fixed-width blocks preserve grammar, examples, and diagrams.

[Previous](part-006.md) · [Next](part-008.md)

<a id="source-page-32-part-1"></a>

## Source page 32

```text

RFC 3987         Internationalized Resource Identifiers     January 2005


   [XPointer]. The IRI corresponding to the above URI would be (in XML
   notation)
   "http://www.example.org/r%E9sum%E9.xml#r&#xE9;sum&#xE9";.

   Similar considerations apply to query parts.  The functionality of
   IRIs (namely, to be able to include non-ASCII characters) can only be
   used if the query part is encoded in UTF-8.

6.5.  Relative IRI References

   Processing of relative IRI references against a base is handled
   straightforwardly; the algorithms of [RFC3986] can be applied
   directly, treating the characters additionally allowed in IRI
   references in the same way that unreserved characters are in URI
   references.

7.  URI/IRI Processing Guidelines (Informative)

   This informative section provides guidelines for supporting IRIs in
   the same software components and operations that currently process
   URIs: Software interfaces that handle URIs, software that allows
   users to enter URIs, software that creates or generates URIs,
   software that displays URIs, formats and protocols that transport
   URIs, and software that interprets URIs.  These may all require
   modification before functioning properly with IRIs.  The
   considerations in this section also apply to URI references and IRI
   references.

7.1.  URI/IRI Software Interfaces

   Software interfaces that handle URIs, such as URI-handling APIs and
   protocols transferring URIs, need interfaces and protocol elements
   that are designed to carry IRIs.

   In case the current handling in an API or protocol is based on
   US-ASCII, UTF-8 is recommended as the character encoding for IRIs, as
   it is compatible with US-ASCII, is in accordance with the
   recommendations of [RFC2277], and makes converting to URIs easy.  In
   any case, the API or protocol definition must clearly define the
   character encoding to be used.

   The transfer from URI-only to IRI-capable components requires no
   mapping, although the conversion described in section 3.2 above may
   be performed.  It is preferable not to perform this inverse
   conversion when there is a chance that this cannot be done correctly.






Duerst & Suignard           Standards Track                    [Page 32]
```

<a id="source-page-33-part-1"></a>

## Source page 33

```text

RFC 3987         Internationalized Resource Identifiers     January 2005


7.2.  URI/IRI Entry

   Some components allow users to enter URIs into the system by typing
   or dictation, for example.  This software must be updated to allow
   for IRI entry.

   A person viewing a visual representation of an IRI (as a sequence of
   glyphs, in some order, in some visual display) or hearing an IRI will
   use an entry method for characters in the user's language to input
   the IRI.  Depending on the script and the input method used, this may
   be a more or less complicated process.

   The process of IRI entry must ensure, as much as possible, that the
   restrictions defined in section 2.2 are met.  This may be done by
   choosing appropriate input methods or variants/settings thereof, by
   appropriately converting the characters being input, by eliminating
   characters that cannot be converted, and/or by issuing a warning or
   error message to the user.

   As an example of variant settings, input method editors for East
   Asian Languages usually allow the input of Latin letters and related
   characters in full-width or half-width versions.  For IRI input, the
   input method editor should be set so that it produces half-width
   Latin letters and punctuation and full-width Katakana.

   An input field primarily or solely used for the input of URIs/IRIs
   may allow the user to view an IRI as it is mapped to a URI.  Places
   where the input of IRIs is frequent may provide the possibility for
   viewing an IRI as mapped to a URI.  This will help users when some of
   the software they use does not yet accept IRIs.

   An IRI input component interfacing to components that handle URIs,
   but not IRIs, must map the IRI to a URI before passing it to these
   components.

   For the input of IRIs with right-to-left characters, please see
   section 4.3.

7.3.  URI/IRI Transfer between Applications

   Many applications, particularly mail user agents, try to detect URIs
   appearing in plain text.  For this, they use some heuristics based on
   URI syntax.  They then allow the user to click on such URIs and
   retrieve the corresponding resource in an appropriate (usually
   scheme-dependent) application.






Duerst & Suignard           Standards Track                    [Page 33]
```

<a id="source-page-34-part-1"></a>

## Source page 34

```text

RFC 3987         Internationalized Resource Identifiers     January 2005


   Such applications have to be upgraded to use the IRI syntax as a base
   for heuristics.  In particular, a non-ASCII character should not be
   taken as the indication of the end of an IRI.  Such applications also
   have to make sure that they correctly convert the detected IRI from
   the character encoding of the document or application where the IRI
   appears to the character encoding used by the system-wide IRI
   invocation mechanism, or to a URI (according to section 3.1) if the
   system-wide invocation mechanism only accepts URIs.

   The clipboard is another frequently used way to transfer URIs and
   IRIs from one application to another.  On most platforms, the
   clipboard is able to store and transfer text in many languages and
   scripts.  Correctly used, the clipboard transfers characters, not
   bytes, which will do the right thing with IRIs.

7.4.  URI/IRI Generation

   Systems that offer resources through the Internet, where those
   resources have logical names, sometimes automatically generate URIs
   for the resources they offer.  For example, some HTTP servers can
   generate a directory listing for a file directory and then respond to
   the generated URIs with the files.

   Many legacy character encodings are in use in various file systems.
   Many currently deployed systems do not transform the local character
   representation of the underlying system before generating URIs.

   For maximum interoperability, systems that generate resource
   identifiers should make the appropriate transformations.  For
   example, if a file system contains a file named
   "r&#xE9;sum&#xE9;.html", a server should expose this as
   "r%C3%A9sum%C3%A9.html" in a URI, which allows use of
   "r&#xE9;sum&#xE9;.html" in an IRI, even if locally the file name is
   kept in a character encoding other than UTF-8.

   This recommendation particularly applies to HTTP servers.  For FTP
   servers, similar considerations apply; see [RFC2640].

7.5.  URI/IRI Selection

   In some cases, resource owners and publishers have control over the
   IRIs used to identify their resources.  This control is mostly
   executed by controlling the resource names, such as file names,
   directly.







Duerst & Suignard           Standards Track                    [Page 34]
```

<a id="source-page-35-part-1"></a>

## Source page 35

```text

RFC 3987         Internationalized Resource Identifiers     January 2005


   In these cases, it is recommended to avoid choosing IRIs that are
   easily confused.  For example, for US-ASCII, the lower-case ell ("l")
   is easily confused with the digit one ("1"), and the upper-case oh
   ("O") is easily confused with the digit zero ("0").  Publishers
   should avoid confusing users with "br0ken" or "1ame" identifiers.

   Outside the US-ASCII repertoire, there are many more opportunities
   for confusion; a complete set of guidelines is too lengthy to include
   here.  As long as names are limited to characters from a single
   script, native writers of a given script or language will know best
   when ambiguities can appear, and how they can be avoided.  What may
   look ambiguous to a stranger may be completely obvious to the average
   native user.  On the other hand, in some cases, the UCS contains
   variants for compatibility reasons; for example, for typographic
   purposes.  These should be avoided wherever possible.  Although there
   may be exceptions, newly created resource names should generally be
   in NFKC [UTR15] (which means that they are also in NFC).

   As an example, the UCS contains the "fi" ligature at U+FB01 for
   compatibility reasons.  Wherever possible, IRIs should use the two
   letters "f" and "i" rather than the "fi" ligature.  An example where
   the latter may be used is in the query part of an IRI for an explicit
   search for a word written containing the "fi" ligature.

   In certain cases, there is a chance that characters from different
   scripts look the same.  The best known example is the similarity of
   the Latin "A", the Greek "Alpha", and the Cyrillic "A".  To avoid
   such cases, only IRIs should be created where all the characters in a
   single component are used together in a given language.  This usually
   means that all of these characters will be from the same script, but
   there are languages that mix characters from different scripts (such
   as Japanese).  This is similar to the heuristics used to distinguish
   between letters and numbers in the examples above.  Also, for Latin,
   Greek, and Cyrillic, using lowercase letters results in fewer
   ambiguities than using uppercase letters would.

7.6.  Display of URIs/IRIs

   In situations where the rendering software is not expected to display
   non-ASCII parts of the IRI correctly using the available layout and
   font resources, these parts should be percent-encoded before being
   displayed.

   For display of Bidi IRIs, please see section 4.1.







Duerst & Suignard           Standards Track                    [Page 35]
```

<a id="source-page-36-part-1"></a>

## Source page 36

```text

RFC 3987         Internationalized Resource Identifiers     January 2005


7.7.  Interpretation of URIs and IRIs

   Software that interprets IRIs as the names of local resources should
   accept IRIs in multiple forms and convert and match them with the
   appropriate local resource names.

   First, multiple representations include both IRIs in the native
   character encoding of the protocol and also their URI counterparts.

   Second, it may include URIs constructed based on character encodings
   other than UTF-8.  These URIs may be produced by user agents that do
   not conform to this specification and that use legacy character
   encodings to convert non-ASCII characters to URIs.  Whether this is
   necessary, and what character encodings to cover, depends on a number
   of factors, such as the legacy character encodings used locally and
   the distribution of various versions of user agents.  For example,
   software for Japanese may accept URIs in Shift_JIS and/or EUC-JP in
   addition to UTF-8.

   Third, it may include additional mappings to be more user-friendly
   and robust against transmission errors.  These would be similar to
   how some servers currently treat URIs as case insensitive or perform
   additional matching to account for spelling errors.  For characters
   beyond the US-ASCII repertoire, this may, for example, include
   ignoring the accents on received IRIs or resource names.  Please note
   that such mappings, including case mappings, are language dependent.

   It can be difficult to identify a resource unambiguously if too many
   mappings are taken into consideration.  However, percent-encoded and
   not percent-encoded parts of IRIs can always be clearly
   distinguished.  Also, the regularity of UTF-8 (see [Duerst97]) makes
   the potential for collisions lower than it may seem at first.

7.8.  Upgrading Strategy

   Where this recommendation places further constraints on software for
   which many instances are already deployed, it is important to
   introduce upgrades carefully and to be aware of the various
   interdependencies.

   If IRIs cannot be interpreted correctly, they should not be created,
   generated, or transported.  This suggests that upgrading URI
   interpreting software to accept IRIs should have highest priority.

   On the other hand, a single IRI is interpreted only by a single or
   very few interpreters that are known in advance, although it may be
   entered and transported very widely.




Duerst & Suignard           Standards Track                    [Page 36]
```

