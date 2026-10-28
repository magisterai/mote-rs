# Internationalized Resource Identifiers (IRIs) — part 3

Source: [Internationalized Resource Identifiers (IRIs)](https://www.rfc-editor.org/rfc/rfc3987.txt) · RFC 3987.

[Document index](../README.md) · [Topic guide](../../../../README.md) · [Publisher](https://www.rfc-editor.org/rfc/rfc3987.txt).

Verbatim text slices in fixed-width blocks preserve grammar, examples, and diagrams.

[Previous](part-002.md) · [Next](part-004.md)

<a id="source-page-12-part-1"></a>

## Source page 12

```text

RFC 3987         Internationalized Resource Identifiers     January 2005


   The ToASCII operation may fail, but this would mean that the IRI
   cannot be resolved.  This conversion SHOULD be used when the goal is
   to maximize interoperability with legacy URI resolvers.  For example,
   the IRI

   "http://r&#xE9;sum&#xE9;.example.org"

   may be converted to

   "http://xn--rsum-bpad.example.org"

   instead of

   "http://r%C3%A9sum%C3%A9.example.org".

   An IRI with a scheme that is known to use domain names in ireg-name,
   but where the scheme definition does not allow percent-encoding for
   ireg-name, meets scheme-specific restrictions if either the
   straightforward conversion or the conversion using the ToASCII
   operation on ireg-name result in an URI that meets the scheme-
   specific restrictions.

   Such an IRI resolves to the URI obtained after converting the IRI and
   uses the ToASCII operation on ireg-name.  Implementations do not have
   to do this conversion as long as they produce the same result.

   Note: The difference between variants b and c in step 1 (using
      normalization with NFC, versus not using any normalization)
      accounts for the fact that in many non-Unicode character
      encodings, some text cannot be represented directly. For example,
      the word "Vietnam" is natively written "Vi&#x1EC7;t Nam"
      (containing a LATIN SMALL LETTER E WITH CIRCUMFLEX AND DOT BELOW)
      in NFC, but a direct transcoding from the windows-1258 character
      encoding leads to "Vi&#xEA;&#x323;t Nam" (containing a LATIN SMALL
      LETTER E WITH CIRCUMFLEX followed by a COMBINING DOT BELOW).
      Direct transcoding of other 8-bit encodings of Vietnamese may lead
      to other representations.

   Note: The uniform treatment of the whole IRI in step 2 is important
      to make processing independent of URI scheme.  See [Gettys] for an
      in-depth discussion.

   Note: In practice, whether the general mapping (steps 1 and 2) or the
      ToASCII operation of [RFC3490] is used for ireg-name will not be
      noticed if mapping from IRI to URI and resolution is tightly
      integrated (e.g., carried out in the same user agent).  But





Duerst & Suignard           Standards Track                    [Page 12]
```

<a id="source-page-13-part-1"></a>

## Source page 13

```text

RFC 3987         Internationalized Resource Identifiers     January 2005


      conversion using [RFC3490] may be able to better deal with
      backwards compatibility issues in case mapping and resolution are
      separated, as in the case of using an HTTP proxy.

   Note: Internationalized Domain Names may be contained in parts of an
      IRI other than the ireg-name part.  It is the responsibility of
      scheme-specific implementations (if the Internationalized Domain
      Name is part of the scheme syntax) or of server-side
      implementations (if the Internationalized Domain Name is part of
      'iquery') to apply the necessary conversions at the appropriate
      point.  Example: Trying to validate the Web page at
      http://r&#xE9;sum&#xE9;.example.org would lead to an IRI of
      http://validator.w3.org/check?uri=http%3A%2F%2Fr&#xE9;sum&#xE9;.
      example.org, which would convert to a URI of
      http://validator.w3.org/check?uri=http%3A%2F%2Fr%C3%A9sum%C3%A9.
      example.org.  The server side implementation would be responsible
      for making the necessary conversions to be able to retrieve the
      Web page.

   Systems accepting IRIs MAY also deal with the printable characters in
   US-ASCII that are not allowed in URIs, namely "<", ">", '"', space,
   "{", "}", "|", "\", "^", and "`", in step 2 above.  If these
   characters are found but are not converted, then the conversion
   SHOULD fail.  Please note that the number sign ("#"), the percent
   sign ("%"), and the square bracket characters ("[", "]") are not part
   of the above list and MUST NOT be converted.  Protocols and formats
   that have used earlier definitions of IRIs including these characters
   MAY require percent-encoding of these characters as a preprocessing
   step to extract the actual IRI from a given field.  This
   preprocessing MAY also be used by applications allowing the user to
   enter an IRI.

   Note: In this process (in step 2.3), characters allowed in URI
      references and existing percent-encoded sequences are not encoded
      further.  (This mapping is similar to, but different from, the
      encoding applied when arbitrary content is included in some part
      of a URI.)  For example, an IRI of
      "http://www.example.org/red%09ros&#xE9;#red" (in XML notation) is
      converted to
      "http://www.example.org/red%09ros%C3%A9#red", not to something
      like
      "http%3A%2F%2Fwww.example.org%2Fred%2509ros%C3%A9%23red".

   Note: Some older software transcoding to UTF-8 may produce illegal
      output for some input, in particular for characters outside the
      BMP (Basic Multilingual Plane).  As an example, for the IRI with
      non-BMP characters (in XML Notation):
      "http://example.com/&#x10300;&#x10301;&#x10302";



Duerst & Suignard           Standards Track                    [Page 13]
```

<a id="source-page-14-part-1"></a>

## Source page 14

```text

RFC 3987         Internationalized Resource Identifiers     January 2005


      which contains the first three letters of the Old Italic alphabet,
      the correct conversion to a URI is
      "http://example.com/%F0%90%8C%80%F0%90%8C%81%F0%90%8C%82"

3.2.  Converting URIs to IRIs

   In some situations, converting a URI into an equivalent IRI may be
   desirable.  This section gives a procedure for this conversion.  The
   conversion described in this section will always result in an IRI
   that maps back to the URI used as an input for the conversion (except
   for potential case differences in percent-encoding and for potential
   percent-encoded unreserved characters).  However, the IRI resulting
   from this conversion may not be exactly the same as the original IRI
   (if there ever was one).

   URI-to-IRI conversion removes percent-encodings, but not all
   percent-encodings can be eliminated.  There are several reasons for
   this:

   1.  Some percent-encodings are necessary to distinguish percent-
       encoded and unencoded uses of reserved characters.

   2.  Some percent-encodings cannot be interpreted as sequences of
       UTF-8 octets.

       (Note: The octet patterns of UTF-8 are highly regular.
       Therefore, there is a very high probability, but no guarantee,
       that percent-encodings that can be interpreted as sequences of
       UTF-8 octets actually originated from UTF-8.  For a detailed
       discussion, see [Duerst97].)

   3.  The conversion may result in a character that is not appropriate
       in an IRI.  See sections 2.2, 4.1, and 6.1 for further details.

   Conversion from a URI to an IRI is done by using the following steps
   (or any other algorithm that produces the same result):

   1.  Represent the URI as a sequence of octets in US-ASCII.

   2.  Convert all percent-encodings ("%" followed by two hexadecimal
       digits) to the corresponding octets, except those corresponding
       to "%", characters in "reserved", and characters in US-ASCII not
       allowed in URIs.

   3.  Re-percent-encode any octet produced in step 2 that is not part
       of a strictly legal UTF-8 octet sequence.





Duerst & Suignard           Standards Track                    [Page 14]
```

<a id="source-page-15-part-1"></a>

## Source page 15

```text

RFC 3987         Internationalized Resource Identifiers     January 2005


   4. Re-percent-encode all octets produced in step 3 that in UTF-8
      represent characters that are not appropriate according to
      sections 2.2, 4.1, and 6.1.

   5. Interpret the resulting octet sequence as a sequence of characters
      encoded in UTF-8.

   This procedure will convert as many percent-encoded characters as
   possible to characters in an IRI.  Because there are some choices
   when step 4 is applied (see section 6.1), results may vary.

   Conversions from URIs to IRIs MUST NOT use any character encoding
   other than UTF-8 in steps 3 and 4, even if it might be possible to
   guess from the context that another character encoding than UTF-8 was
   used in the URI.  For example, the URI
   "http://www.example.org/r%E9sum%E9.html" might with some guessing be
   interpreted to contain two e-acute characters encoded as iso-8859-1.
   It must not be converted to an IRI containing these e-acute
   characters.  Otherwise, in the future the IRI will be mapped to
   "http://www.example.org/r%C3%A9sum%C3%A9.html", which is a different
   URI from "http://www.example.org/r%E9sum%E9.html".

3.2.1.  Examples

   This section shows various examples of converting URIs to IRIs.  Each
   example shows the result after each of the steps 1 through 5 is
   applied.  XML Notation is used for the final result.  Octets are
   denoted by "<" followed by two hexadecimal digits followed by ">".

   The following example contains the sequence "%C3%BC", which is a
   strictly legal UTF-8 sequence, and which is converted into the actual
   character U+00FC, LATIN SMALL LETTER U WITH DIAERESIS (also known as
   u-umlaut).

   1.  http://www.example.org/D%C3%BCrst

   2.  http://www.example.org/D<c3><bc>rst

   3.  http://www.example.org/D<c3><bc>rst

   4.  http://www.example.org/D<c3><bc>rst

   5.  http://www.example.org/D&#xFC;rst

   The following example contains the sequence "%FC", which might
   represent U+00FC, LATIN SMALL LETTER U WITH DIAERESIS, in the
   iso-8859-1 character encoding.  (It might represent other characters
   in other character encodings.  For example, the octet <fc> in



Duerst & Suignard           Standards Track                    [Page 15]
```

<a id="source-page-16-part-1"></a>

## Source page 16

```text

RFC 3987         Internationalized Resource Identifiers     January 2005


   iso-8859-5 represents U+045C, CYRILLIC SMALL LETTER KJE.)  Because
   <fc> is not part of a strictly legal UTF-8 sequence, it is
   re-percent-encoded in step 3.

   1.  http://www.example.org/D%FCrst

   2.  http://www.example.org/D<fc>rst

   3.  http://www.example.org/D%FCrst

   4.  http://www.example.org/D%FCrst

   5.  http://www.example.org/D%FCrst

   The following example contains "%e2%80%ae", which is the percent-
   encoded UTF-8 character encoding of U+202E, RIGHT-TO-LEFT OVERRIDE.
   Section 4.1 forbids the direct use of this character in an IRI.
   Therefore, the corresponding octets are re-percent-encoded in step 4.
   This example shows that the case (upper- or lowercase) of letters
   used in percent-encodings may not be preserved.  The example also
   contains a punycode-encoded domain name label (xn--99zt52a), which is
   not converted.

   1.  http://xn--99zt52a.example.org/%e2%80%ae

   2.  http://xn--99zt52a.example.org/<e2><80><ae>

   3.  http://xn--99zt52a.example.org/<e2><80><ae>

   4.  http://xn--99zt52a.example.org/%E2%80%AE

   5.  http://xn--99zt52a.example.org/%E2%80%AE

   Implementations with scheme-specific knowledge MAY convert
   punycode-encoded domain name labels to the corresponding characters
   by using the ToUnicode procedure.  Thus, for the example above, the
   label "xn--99zt52a" may be converted to U+7D0D U+8C46 (Japanese
   Natto), leading to the overall IRI of
   "http://&#x7D0D;&#x8C46;.example.org/%E2%80%AE".

4.  Bidirectional IRIs for Right-to-Left Languages

   Some UCS characters, such as those used in the Arabic and Hebrew
   scripts, have an inherent right-to-left (rtl) writing direction.
   IRIs containing these characters (called bidirectional IRIs or Bidi
   IRIs) require additional attention because of the non-trivial





Duerst & Suignard           Standards Track                    [Page 16]
```

