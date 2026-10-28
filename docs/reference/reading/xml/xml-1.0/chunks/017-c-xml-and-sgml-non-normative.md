# C XML and SGML (Non-Normative)

Source: [Extensible Markup Language (XML) 1.0](https://www.w3.org/TR/2008/REC-xml-20081126/) · 5th edition, 2008-11-26.

[Document index](../README.md) · [Topic guide](../../../../README.md) · [Publisher](https://www.w3.org/TR/2008/REC-xml-20081126/).

HTML normalized to Markdown. Complex tables and mathematical markup retain HTML; illustrations are stored locally.

[Previous](016-a-2-other-references.md) · [Next](018-f-1-detection-without-external-encoding-information.md)

<a id="sec-xml-and-sgml"></a>

## C XML and SGML (Non-Normative)

XML
is designed to be a subset of SGML, in that every XML document should also
be a conforming SGML document. For a detailed comparison of the additional
restrictions that XML places on documents beyond those of SGML, see [[Clark]](016-a-2-other-references.md#Clark).

<a id="sec-entexpand"></a>

## D Expansion of Entity and Character References (Non-Normative)

This appendix contains some examples illustrating the sequence of entity-
and character-reference recognition and expansion, as specified in [**4.4 XML Processor Treatment of Entities and References**](013-4-4-xml-processor-treatment-of-entities-and-references.md#entproc).

If the DTD contains the declaration



```
<!ENTITY example "<p>An ampersand (&#38;#38;) may be escaped
numerically (&#38;#38;#38;) or with a general entity
(&amp;amp;).</p>" >
```




then the XML processor will recognize the character references when it
parses the entity declaration, and resolve them before storing the following
string as the value of the entity "
`example`
":



```
<p>An ampersand (&#38;) may be escaped
numerically (&#38;#38;) or with a general entity
(&amp;amp;).</p>
```




A reference in the document to "
`&example;`
"
will cause the text to be reparsed, at which time the start- and end-tags
of the `p` element will be recognized and the three references will
be recognized and expanded, resulting in a `p` element with the following
content (all data, no delimiters or markup):



```
An ampersand (&) may be escaped
numerically (&#38;) or with a general entity
(&amp;).
```




A more complex example will illustrate the rules and their effects fully.
In the following example, the line numbers are solely for reference.



```
1 <?xml version='1.0'?>
2 <!DOCTYPE test [
3 <!ELEMENT test (#PCDATA) >
4 <!ENTITY % xx '&#37;zz;'>
5 <!ENTITY % zz '&#60;!ENTITY tricky "error-prone" >' >
6 %xx;
7 ]>
8 <test>This sample shows a &tricky; method.</test>
```




This produces the following:

- in line 4, the reference to character 37 is expanded immediately,
  and the parameter entity "
  `xx`
  " is stored in the symbol
  table with the value "
  `%zz;`
  ". Since the replacement
  text is not rescanned, the reference to parameter entity "
  `zz`
  "
  is not recognized. (And it would be an error if it were, since "
  `zz`
  "
  is not yet declared.)
- in line 5, the character reference "
  `&#60;`
  "
  is expanded immediately and the parameter entity "
  `zz`
  "
  is stored with the replacement text "
  `<!ENTITY tricky "error-prone"
  >`
  ", which is a well-formed entity declaration.
- in line 6, the reference to "
  `xx`
  " is recognized,
  and the replacement text of "
  `xx`
  " (namely "
  `%zz;`
  ")
  is parsed. The reference to "
  `zz`
  " is recognized in
  its turn, and its replacement text ("
  `<!ENTITY tricky "error-prone"
  >`
  ") is parsed. The general entity "
  `tricky`
  "
  has now been declared, with the replacement text "
  `error-prone`
  ".
- in line 8, the reference to the general entity "
  `tricky`
  "
  is recognized, and it is expanded, so the full content of the `test`
  element is the self-describing (and ungrammatical) string *This sample
  shows a error-prone method.*

In the following example



```
<!DOCTYPE foo [ 
<!ENTITY x "&lt;"> 
]> 
<foo attr="&x;"/>
```




the replacement text of x is the four characters "&lt;" because
references to general entities in entity values are [bypassed](013-4-4-xml-processor-treatment-of-entities-and-references.md#bypass).
The replacement text of lt is a character reference to
the less-than character, for example the five characters "&#60;"
(see [**4.6 Predefined Entities**](014-4-5-construction-of-entity-replacement-text.md#sec-predefined-ent)). Since neither of these contains a less-than character
the result is well-formed.

If the definition of x had been



```
<!ENTITY x "&#60;">
```




then the document would not have been well-formed, because the
replacement text of x would be the single character "<" which
is not permitted in attribute values (see [WFC: No < in Attribute Values](008-3-1-start-tags-end-tags-and-empty-element-tags.md#CleanAttrVals)).

<a id="determinism"></a>

## E Deterministic Content Models (Non-Normative)

As
noted in [**3.2.1 Element Content**](008-3-1-start-tags-end-tags-and-empty-element-tags.md#sec-element-content), it is required that content
models in element type declarations be deterministic. This requirement is [for compatibility](002-appendices.md#dt-compat "For Compatibility") with SGML (which calls deterministic
content models "unambiguous"); XML processors built
using SGML systems may flag non-deterministic content models as errors.

For example, the content model `((b, c) | (b, d))` is non-deterministic,
because given an initial `b` the XML processor
cannot know which `b` in the model is being matched without looking
ahead to see which element follows the `b`. In this case, the two references
to `b` can be collapsed into a single reference, making the model read `(b,
(c | d))`. An initial `b` now clearly matches only a single name
in the content model. The processor doesn't need to look ahead to see what follows; either `c` or `d`
would be accepted.

More formally: a finite state automaton may be constructed from the content
model using the standard algorithms, e.g. algorithm 3.5 in section 3.9 of
Aho, Sethi, and Ullman [[Aho/Ullman]](016-a-2-other-references.md#Aho). In many such algorithms, a follow
set is constructed for each position in the regular expression (i.e., each
leaf node in the syntax tree for the regular expression); if any position
has a follow set in which more than one following position is labeled with
the same element type name, then the content model is in error and may be
reported as an error.

Algorithms exist which allow many but not all non-deterministic content
models to be reduced automatically to equivalent deterministic models; see
Brüggemann-Klein 1991 [[Brüggemann-Klein]](016-a-2-other-references.md#ABK).

<a id="sec-guessing"></a>

## F Autodetection of Character Encodings (Non-Normative)

The XML encoding declaration functions as an internal label on each entity,
indicating which character encoding is in use. Before an XML processor can
read the internal label, however, it apparently has to know what character
encoding is in use—which is what the internal label is trying to indicate.
In the general case, this is a hopeless situation. It is not entirely hopeless
in XML, however, because XML limits the general case in two ways: each implementation
is assumed to support only a finite set of character encodings, and the XML
encoding declaration is restricted in position and content in order to make
it feasible to autodetect the character encoding in use in each entity in
normal cases. Also, in many cases other sources of information are available
in addition to the XML data stream itself. Two cases may be distinguished,
depending on whether the XML entity is presented to the processor without,
or with, any accompanying (external) information. We will consider
these cases in turn.

