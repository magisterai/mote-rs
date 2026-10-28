# UTF-8, a transformation format of ISO 10646 — part 2

Source: [UTF-8, a transformation format of ISO 10646](https://www.rfc-editor.org/rfc/rfc3629.txt) · RFC 3629.

[Document index](../README.md) · [Topic guide](../../../../../README.md) · [Publisher](https://www.rfc-editor.org/rfc/rfc3629.txt).

Verbatim text slices in fixed-width blocks preserve grammar, examples, and diagrams.

[Previous](part-001.md) · [Next](part-003.md)

<a id="source-page-6-part-1"></a>

## Source page 6

```text

RFC 3629                         UTF-8                     November 2003


   UTF8-3      = %xE0 %xA0-BF UTF8-tail / %xE1-EC 2( UTF8-tail ) /
                 %xED %x80-9F UTF8-tail / %xEE-EF 2( UTF8-tail )
   UTF8-4      = %xF0 %x90-BF 2( UTF8-tail ) / %xF1-F3 3( UTF8-tail ) /
                 %xF4 %x80-8F 2( UTF8-tail )
   UTF8-tail   = %x80-BF

   NOTE -- The authoritative definition of UTF-8 is in [UNICODE].  This
   grammar is believed to describe the same thing Unicode describes, but
   does not claim to be authoritative.  Implementors are urged to rely
   on the authoritative source, rather than on this ABNF.

5.  Versions of the standards

   ISO/IEC 10646 is updated from time to time by publication of
   amendments and additional parts; similarly, new versions of the
   Unicode standard are published over time.  Each new version obsoletes
   and replaces the previous one, but implementations, and more
   significantly data, are not updated instantly.

   In general, the changes amount to adding new characters, which does
   not pose particular problems with old data.  In 1996, Amendment 5 to
   the 1993 edition of ISO/IEC 10646 and Unicode 2.0 moved and expanded
   the Korean Hangul block, thereby making any previous data containing
   Hangul characters invalid under the new version.  Unicode 2.0 has the
   same difference from Unicode 1.1.  The justification for allowing
   such an incompatible change was that there were no major
   implementations and no significant amounts of data containing Hangul.
   The incident has been dubbed the "Korean mess", and the relevant
   committees have pledged to never, ever again make such an
   incompatible change (see Unicode Consortium Policies [1]).

   New versions, and in particular any incompatible changes, have
   consequences regarding MIME charset labels, to be discussed in MIME
   registration (Section 8).

6.  Byte order mark (BOM)

   The UCS character U+FEFF "ZERO WIDTH NO-BREAK SPACE" is also known
   informally as "BYTE ORDER MARK" (abbreviated "BOM").  This character
   can be used as a genuine "ZERO WIDTH NO-BREAK SPACE" within text, but
   the BOM name hints at a second possible usage of the character:  to
   prepend a U+FEFF character to a stream of UCS characters as a
   "signature".  A receiver of such a serialized stream may then use the
   initial character as a hint that the stream consists of UCS
   characters and also to recognize which UCS encoding is involved and,
   with encodings having a multi-octet encoding unit, as a way to





Yergeau                     Standards Track                     [Page 6]
```

<a id="source-page-7-part-1"></a>

## Source page 7

```text

RFC 3629                         UTF-8                     November 2003


   recognize the serialization order of the octets.  UTF-8 having a
   single-octet encoding unit, this last function is useless and the BOM
   will always appear as the octet sequence EF BB BF.

   It is important to understand that the character U+FEFF appearing at
   any position other than the beginning of a stream MUST be interpreted
   with the semantics for the zero-width non-breaking space, and MUST
   NOT be interpreted as a signature.  When interpreted as a signature,
   the Unicode standard suggests than an initial U+FEFF character may be
   stripped before processing the text.  Such stripping is necessary in
   some cases (e.g., when concatenating two strings, because otherwise
   the resulting string may contain an unintended "ZERO WIDTH NO-BREAK
   SPACE" at the connection point), but might affect an external process
   at a different layer (such as a digital signature or a count of the
   characters) that is relying on the presence of all characters in the
   stream.  It is therefore RECOMMENDED to avoid stripping an initial
   U+FEFF interpreted as a signature without a good reason, to ignore it
   instead of stripping it when appropriate (such as for display) and to
   strip it only when really necessary.

   U+FEFF in the first position of a stream MAY be interpreted as a
   zero-width non-breaking space, and is not always a signature.  In an
   attempt at diminishing this uncertainty, Unicode 3.2 adds a new
   character, U+2060 "WORD JOINER", with exactly the same semantics and
   usage as U+FEFF except for the signature function, and strongly
   recommends its exclusive use for expressing word-joining semantics.
   Eventually, following this recommendation will make it all but
   certain that any initial U+FEFF is a signature, not an intended "ZERO
   WIDTH NO-BREAK SPACE".

   In the meantime, the uncertainty unfortunately remains and may affect
   Internet protocols.  Protocol specifications MAY restrict usage of
   U+FEFF as a signature in order to reduce or eliminate the potential
   ill effects of this uncertainty.  In the interest of striking a
   balance between the advantages (reduction of uncertainty) and
   drawbacks (loss of the signature function) of such restrictions, it
   is useful to distinguish a few cases:

   o  A protocol SHOULD forbid use of U+FEFF as a signature for those
      textual protocol elements that the protocol mandates to be always
      UTF-8, the signature function being totally useless in those
      cases.

   o  A protocol SHOULD also forbid use of U+FEFF as a signature for
      those textual protocol elements for which the protocol provides
      character encoding identification mechanisms, when it is expected
      that implementations of the protocol will be in a position to
      always use the mechanisms properly.  This will be the case when



Yergeau                     Standards Track                     [Page 7]
```

<a id="source-page-8-part-1"></a>

## Source page 8

```text

RFC 3629                         UTF-8                     November 2003


      the protocol elements are maintained tightly under the control of
      the implementation from the time of their creation to the time of
      their (properly labeled) transmission.

   o  A protocol SHOULD NOT forbid use of U+FEFF as a signature for
      those textual protocol elements for which the protocol does not
      provide character encoding identification mechanisms, when a ban
      would be unenforceable, or when it is expected that
      implementations of the protocol will not be in a position to
      always use the mechanisms properly.  The latter two cases are
      likely to occur with larger protocol elements such as MIME
      entities, especially when implementations of the protocol will
      obtain such entities from file systems, from protocols that do not
      have encoding identification mechanisms for payloads (such as FTP)
      or from other protocols that do not guarantee proper
      identification of character encoding (such as HTTP).

   When a protocol forbids use of U+FEFF as a signature for a certain
   protocol element, then any initial U+FEFF in that protocol element
   MUST be interpreted as a "ZERO WIDTH NO-BREAK SPACE".  When a
   protocol does NOT forbid use of U+FEFF as a signature for a certain
   protocol element, then implementations SHOULD be prepared to handle a
   signature in that element and react appropriately: using the
   signature to identify the character encoding as necessary and
   stripping or ignoring the signature as appropriate.

7.  Examples

   The character sequence U+0041 U+2262 U+0391 U+002E "A<NOT IDENTICAL
   TO><ALPHA>." is encoded in UTF-8 as follows:

       --+--------+-----+--
       41 E2 89 A2 CE 91 2E
       --+--------+-----+--

   The character sequence U+D55C U+AD6D U+C5B4 (Korean "hangugeo",
   meaning "the Korean language") is encoded in UTF-8 as follows:

       --------+--------+--------
       ED 95 9C EA B5 AD EC 96 B4
       --------+--------+--------

   The character sequence U+65E5 U+672C U+8A9E (Japanese "nihongo",
   meaning "the Japanese language") is encoded in UTF-8 as follows:

       --------+--------+--------
       E6 97 A5 E6 9C AC E8 AA 9E
       --------+--------+--------



Yergeau                     Standards Track                     [Page 8]
```

<a id="source-page-9-part-1"></a>

## Source page 9

```text

RFC 3629                         UTF-8                     November 2003


   The character U+233B4 (a Chinese character meaning 'stump of tree'),
   prepended with a UTF-8 BOM, is encoded in UTF-8 as follows:

       --------+-----------
       EF BB BF F0 A3 8E B4
       --------+-----------

8.  MIME registration

   This memo serves as the basis for registration of the MIME charset
   parameter for UTF-8, according to [RFC2978].  The charset parameter
   value is "UTF-8".  This string labels media types containing text
   consisting of characters from the repertoire of ISO/IEC 10646
   including all amendments at least up to amendment 5 of the 1993
   edition (Korean block), encoded to a sequence of octets using the
   encoding scheme outlined above.  UTF-8 is suitable for use in MIME
   content types under the "text" top-level type.

   It is noteworthy that the label "UTF-8" does not contain a version
   identification, referring generically to ISO/IEC 10646.  This is
   intentional, the rationale being as follows:

   A MIME charset label is designed to give just the information needed
   to interpret a sequence of bytes received on the wire into a sequence
   of characters, nothing more (see [RFC2045], section 2.2).  As long as
   a character set standard does not change incompatibly, version
   numbers serve no purpose, because one gains nothing by learning from
   the tag that newly assigned characters may be received that one
   doesn't know about.  The tag itself doesn't teach anything about the
   new characters, which are going to be received anyway.

   Hence, as long as the standards evolve compatibly, the apparent
   advantage of having labels that identify the versions is only that,
   apparent.  But there is a disadvantage to such version-dependent
   labels: when an older application receives data accompanied by a
   newer, unknown label, it may fail to recognize the label and be
   completely unable to deal with the data, whereas a generic, known
   label would have triggered mostly correct processing of the data,
   which may well not contain any new characters.

   Now the "Korean mess" (ISO/IEC 10646 amendment 5) is an incompatible
   change, in principle contradicting the appropriateness of a version
   independent MIME charset label as described above.  But the
   compatibility problem can only appear with data containing Korean
   Hangul characters encoded according to Unicode 1.1 (or equivalently
   ISO/IEC 10646 before amendment 5), and there is arguably no such data
   to worry about, this being the very reason the incompatible change
   was deemed acceptable.



Yergeau                     Standards Track                     [Page 9]
```

<a id="source-page-10-part-1"></a>

## Source page 10

```text

RFC 3629                         UTF-8                     November 2003


   In practice, then, a version-independent label is warranted, provided
   the label is understood to refer to all versions after Amendment 5,
   and provided no incompatible change actually occurs.  Should
   incompatible changes occur in a later version of ISO/IEC 10646, the
   MIME charset label defined here will stay aligned with the previous
   version until and unless the IETF specifically decides otherwise.

9.  IANA Considerations

   The entry for UTF-8 in the IANA charset registry has been updated to
   point to this memo.

10.  Security Considerations

   Implementers of UTF-8 need to consider the security aspects of how
   they handle illegal UTF-8 sequences.  It is conceivable that in some
   circumstances an attacker would be able to exploit an incautious
   UTF-8 parser by sending it an octet sequence that is not permitted by
   the UTF-8 syntax.

   A particularly subtle form of this attack can be carried out against
   a parser which performs security-critical validity checks against the
   UTF-8 encoded form of its input, but interprets certain illegal octet
   sequences as characters.  For example, a parser might prohibit the
   NUL character when encoded as the single-octet sequence 00, but
   erroneously allow the illegal two-octet sequence C0 80 and interpret
   it as a NUL character.  Another example might be a parser which
   prohibits the octet sequence 2F 2E 2E 2F ("/../"), yet permits the
   illegal octet sequence 2F C0 AE 2E 2F.  This last exploit has
   actually been used in a widespread virus attacking Web servers in
   2001; thus, the security threat is very real.

   Another security issue occurs when encoding to UTF-8: the ISO/IEC
   10646 description of UTF-8 allows encoding character numbers up to
   U+7FFFFFFF, yielding sequences of up to 6 bytes.  There is therefore
   a risk of buffer overflow if the range of character numbers is not
   explicitly limited to U+10FFFF or if buffer sizing doesn't take into
   account the possibility of 5- and 6-byte sequences.

   Security may also be impacted by a characteristic of several
   character encodings, including UTF-8: the "same thing" (as far as a
   user can tell) can be represented by several distinct character
   sequences.  For instance, an e with acute accent can be represented
   by the precomposed U+00E9 E ACUTE character or by the canonically
   equivalent sequence U+0065 U+0301 (E + COMBINING ACUTE).  Even though
   UTF-8 provides a single byte sequence for each character sequence,
   the existence of multiple character sequences for "the same thing"
   may have security consequences whenever string matching, indexing,



Yergeau                     Standards Track                    [Page 10]
```

