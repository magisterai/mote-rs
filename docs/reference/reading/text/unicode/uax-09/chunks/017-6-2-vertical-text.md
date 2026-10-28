# 6.2 Vertical Text

Source: [UAX #9: Unicode Bidirectional Algorithm](https://www.unicode.org/reports/tr9/tr9-51.html) · Unicode 17.0.0, revision 51.

[Document index](../README.md) · [Topic guide](../../../../../README.md) · [Publisher](https://www.unicode.org/versions/Unicode17.0.0/).

HTML normalized to Markdown. Complex tables and mathematical markup retain HTML; illustrations are stored locally.

[Previous](016-4-4-bidirectional-conformance-testing.md) · [Next](018-6-4-separating-punctuation-marks.md)

<a id="Vertical_Text"></a>
<a id="bk3"></a>

### 6.2  [Vertical Text](017-6-2-vertical-text.md#Vertical_Text)

In the case of vertical line orientation, there are multiple ways
to display bidirectional text. Some
methods use the Bidirectional Algorithm, and some do not. The Unicode Standard does not specify whether
text is presented with horizontal or vertical layout, or for vertical
layout whether elements within the line are rotated. That is left up to higher-level protocols.
For example, one of the common approaches for vertical line orientation is to rotate all the glyphs
uniformly 90° clockwise. The
Bidirectional Algorithm is used with this method. While some characters end up ordered from bottom to top,
this method can represent a mixture of Arabic and Latin glyphs in the same way as occurs
for horizontal line orientation.

Another possible approach is to render the text in a uniform
single direction from top to bottom.
This method has multiple
variations to determine the orientation of characters. One variant uses the Bidirectional Algorithm to
determine the level of the text, but then the levels are not used to reorder the text. Instead, the levels are
used to determine the *rotation* of each segment of the text.
Sometimes vertical lines follow a vertical baseline in which
each character is oriented as normal (with no rotation), with characters ordered from top to bottom
whether they are Hebrew, numbers, or Latin. When setting text using the Arabic script in vertical lines, it
is more common to employ a horizontal baseline that is rotated by 90° counterclockwise so that the
characters are ordered from top to bottom. Latin text and numbers may be rotated 90° clockwise so
that those characters are also ordered from top to bottom.

<a id="Formatting"></a>

### 6.3  [Formatting](017-6-2-vertical-text.md#Formatting)

Because of the implicit character types and the heuristics for resolving neutral and numeric
directional behavior, the implicit bidirectional ordering will generally produce the correct
display without any further work. However, problematic cases may occur when a right-to-left
paragraph begins with left-to-right characters, or there are nested segments of
different-direction text, or there are weak characters on directional boundaries. In these cases,
embeddings or directional marks may be required to get the right display. Part numbers may also
require directional overrides.

The most common problematic case is that of neutrals on the boundary of an embedded language.
This can be addressed by setting the level of the embedded text correctly. For example, with all
the text at level 0 the following occurs:



```

Memory:  he said "I NEED WATER!", and expired.

Display: he said "RETAW DEEN I!", and expired.
```




If the exclamation mark is to be part of the Arabic quotation, then the user can select the
text *I NEED WATER!* and explicitly mark it as embedded Arabic, which produces the following
result:



```

Memory:  he said "RLII NEED WATER!PDI", and expired.

Display: he said "!RETAW DEEN I", and expired.
```




However, an often simpler and better method of doing this is
to place a right directional mark (RLM) after the exclamation mark.
Because the exclamation mark is now not on a directional boundary, this produces the correct result.
This is the best approach when manually editing text or programmatically generating text meant to be edited,
or dealing with an application that simply does not support explicit formatting characters.



```

Memory:  he said "I NEED WATER!RLM", and expired.

Display: he said "!RETAW DEEN I", and expired.
```




This latter approach is preferred because it does not make use of the explicit formatting characters,
which can easily get out of sync if not fully supported by editors and other string manipulation.
Nevertheless, the explicit formatting characters are
absolutely necessary in cases
where text of one direction
contains text of the opposite direction which itself contains text of the original direction.
Such cases are not as rare as one might think, because Latin-script brand names, technical terms, and abbreviations are often written in their
original Latin characters when used in non-Latin-script text, including right-to-left text, as in the following:



```

Memory:  it is called "RLIAN INTRODUCTION TO javaPDI" - $19.95 in hardcover.

Display: it is called "java OT NOITCUDORTNI NA" - $19.95 in hardcover.
```




Thus, when text is programmatically generated by inserting data into a template,
and is not intended for later manual editing,
and a particular insert happens to be of the opposite direction to the template's text,
it is easiest to wrap the insert in explicit formatting characters (or their markup equivalent) declaring its direction,
without analyzing whether it is really necessary to do so,
or if the job could be done just with stateless directional marks.

Furthermore, in this common scenario, it is highly recommended to use directional isolate formatting characters
as opposed to directional embedding formatting characters (once targeted display platforms are known to support isolates).
This is because embeddings affect the surrounding text similarly to a strong character,
whereas directional isolates have the effect of a neutral.
The embeddings' stronger effect is often difficult to anticipate and is rarely useful.
To demonstrate, here is the example above with embeddings instead of isolates:



```

Memory:  it is called "RLEAN INTRODUCTION TO javaPDF" - $19.95 in hardcover.

Display: it is called "$19.95 - "java OT NOITCUDORTNI NA in hardcover.
```




This, of course, is not the intended display, and is due to the number “sticking”
to the preceding RTL embedding (along with all the neutral characters in between),
just as it would “stick” to a preceding RTL character.

Directional isolates also offer a solution to the very common case where the direction of the text
to be programmatically inserted is not known. Instead of analyzing the characters of the text to be inserted in order
to decide whether to use an LRE or RLE (or LRI or RLI - or nothing at all), the software can take the easy way out
and *always* wrap each unknown-direction insert in an FSI and PDI.
Thus, an FSI instead of an RLI in the example above would produce the same display.
FSI's first-strong heuristic is not infallible, but it will work most of the time even on mixed-script text.

Although wrapping inserts in isolates is a useful technique,
it is best not to wrap text that is known to contain no opposite-direction characters that are not already wrapped in an isolate.
Unnecessary layers of wrapping not only add bulk and complexity;
they can also wind up exceeding the depth limit and rendering ineffective the innermost isolates,
which can make the text display incorrectly.
One very common case of an insert that does not need wrapping is one known to be localized to the context locale,
e.g. a translated message with all its inserted values either themselves localized, or wrapped in an isolate.

