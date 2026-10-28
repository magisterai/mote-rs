# I Production Notes (Non-Normative)

Source: [Extensible Markup Language (XML) 1.0](https://www.w3.org/TR/2008/REC-xml-20081126/) · 5th edition, 2008-11-26.

[Document index](../README.md) · [Topic guide](../../../../README.md) · [Publisher](https://www.w3.org/TR/2008/REC-xml-20081126/).

HTML normalized to Markdown. Complex tables and mathematical markup retain HTML; illustrations are stored locally.

[Previous](018-f-1-detection-without-external-encoding-information.md)

<a id="prod-notes"></a>

## I Production Notes (Non-Normative)

This edition was encoded in a
slightly modified version of the
[XMLspec DTD, v2.10](https://www.w3.org/2002/xmlspec/dtd/2.10/xmlspec.dtd).
The XHTML versions were produced with a combination of the
[xmlspec.xsl](https://www.w3.org/2002/xmlspec/xhtml/1.13/xmlspec.xsl),
[diffspec.xsl](https://www.w3.org/2002/xmlspec/xhtml/1.13/diffspec.xsl),
and [REC-xml.xsl](https://www.w3.org/TR/2008/REC-xml-20081126/REC-xml.xsl)
XSLT stylesheets.

<a id="sec-suggested-names"></a>

## J Suggestions for XML Names (Non-Normative)

The following suggestions define what is believed to be best
practice in the construction of XML names used as element names,
attribute names, processing instruction targets, entity names,
notation names, and the values of attributes of type ID, and are
intended as guidance for document authors and schema designers.
All references to Unicode are understood with respect to
a particular version of the Unicode Standard greater than or equal
to 5.0; which version should be used is left to the discretion of
the document author or schema designer.

The first two suggestions are directly derived from the rules
given for identifiers in Standard Annex #31 (UAX #31) of the Unicode Standard, version 5.0 [[Unicode]](015-5-2-using-xml-processors.md#Unicode), and
exclude all control characters, enclosing nonspacing marks,
non-decimal numbers, private-use characters, punctuation characters
(with the noted exceptions), symbol characters, unassigned
codepoints, and white space characters. The other suggestions
are mostly derived from Appendix B in previous editions of this specification.

1. The first character of any name should have a Unicode property
   of ID\_Start, or else be '\_' #x5F.
2. Characters other than the first should have a Unicode property
   of ID\_Continue, or be one of the characters listed in the table
   entitled "Characters for Natural Language Identifiers" in UAX
   #31, with the exception of "'" #x27 and "’" #x2019.
3. Characters in names should be expressed using
   Normalization Form C as defined in [[UnicodeNormal]](015-5-2-using-xml-processors.md#unipart15).
4. Ideographic characters which have a canonical decomposition
   (including those in the ranges [#xF900-#xFAFF] and
   [#x2F800-#x2FFFD], with 12 exceptions) should not be used in names.
5. Characters which have a compatibility decomposition (those with
   a "compatibility formatting tag" in field 5 of the Unicode
   Character Database -- marked by field 5 beginning with a "<")
   should not be used in names. This suggestion does not apply
   to characters which
   despite their compatibility decompositions are in regular use in
   their scripts, for
   example #x0E33 THAI CHARACTER SARA AM or #x0EB3 LAO CHARACTER AM.
6. Combining characters meant for use with symbols only (including
   those in the ranges [#x20D0-#x20EF] and [#x1D165-#x1D1AD]) should
   not be used in names.
7. The interlinear annotation characters ([#xFFF9-#xFFFB]) should
   not be used in names.
8. Variation selector characters should not be used in names.
9. Names which are nonsensical, unpronounceable, hard to read, or
   easily confusable with other names should not be employed.
