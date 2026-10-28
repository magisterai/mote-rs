# 2 Definitions

Source: [UAX #14: Unicode Line Breaking Algorithm](https://www.unicode.org/reports/tr14/tr14-55.html) · Unicode 17.0.0, revision 55.

[Document index](../README.md) · [Topic guide](../../../../../README.md) · [Publisher](https://www.unicode.org/versions/Unicode17.0.0/).

HTML normalized to Markdown. Complex tables and mathematical markup retain HTML; illustrations are stored locally.

[Previous](001-unicode-standard-annex-14.md) · [Next](003-continuation-3.md)

<a id="Definitions"></a>

## 2  [Definitions](002-2-definitions.md#Definitions)

The notation defined in this annex differs somewhat from the
notation defined elsewhere in the Unicode Standard.

*All other notation used here without an
explicit definition shall be as defined elsewhere in the Unicode
Standard [[Unicode](https://www.unicode.org/reports/tr41/tr41-36.html#Unicode)].*

<a id="LD1"></a>

***[LD1](002-2-definitions.md#LD1)**. **Line Fitting:***
The process of determining how much text will fit
on a line of text, given the available space between the margins and the
actual display width of the text.

<a id="LD2"></a>

***[LD2](002-2-definitions.md#LD2)**. **Line Break:***
The position in the text where one line ends and the
next one starts.

<a id="LD3"></a>

***[LD3](002-2-definitions.md#LD3)**. **Line Break Opportunity:***
A place where a line is allowed to end.

- Whether a given position in the text is a valid line break opportunity depends
  on the context as well as the line breaking rules in force.

<a id="LD4"></a>

***[LD4](002-2-definitions.md#LD4)**. **Line Breaking:***
The process of selecting one among several line
break opportunities such that the resulting line is optimal or ends at a
user-requested explicit line break.

<a id="LD5"></a>

***[LD5](002-2-definitions.md#LD5)**. **Line Breaking Property:***
A character property with enumerated
values, as listed in *[Table 1](002-2-definitions.md#Table1)*, and separated into normative and informative
values.

- Line breaking property values are used to classify characters and, taken in context,
  determine the type of line break opportunity.

<a id="LD6"></a>

***[LD6](002-2-definitions.md#LD6)**. **Line Breaking Class:***
A class of characters with the same line breaking property value.

- The line breaking classes are described in
  *Section 5.1, [Description of Line Breaking Properties](006-5-1-description-of-line-breaking-properties.md#DescriptionOfProperties)*.

<a id="LD7"></a>

***[LD7](002-2-definitions.md#LD7)**. **Mandatory Break:***
A line must break following a character that has the mandatory break property.

- Such a break is also known as a *forced* break and is
  indicated in the rules as **B !**, where **B** is the character with the
  mandatory break property.

<a id="LD8"></a>

***[LD8](002-2-definitions.md#LD8)**. **Direct Break:***
A line break opportunity exists between two
adjacent characters of the given line breaking classes.

- A direct break is indicated in
  the rules below as **B** ÷ **A**, where **B** is the character class
  of the character *before* and **A** is the character class of the
  character *after* the break. If they are separated by one or more space
  characters, a break opportunity exists instead after the last space.

<a id="LD9"></a>

***[LD9](002-2-definitions.md#LD9)**. **Indirect Break:***
A line break opportunity exists between two
characters of the given line breaking classes *only* if they are
separated by one or more spaces.

- For an indirect break, a break opportunity exists
  after the last space. No break opportunity exists if the characters are
  immediately adjacent.
- In the notation of the rules in
  *Section 6, [Line Breaking Algorithm](017-5-5-use-of-double-hyphen.md#Algorithm)*,
  an indirect break is represented as two
  rules: **B** × **A** *and* **B** [SP](014-qu-quotation.md#SP)**+ ÷ A**
  where the “+” sign means one or more occurrences.

<a id="LD10"></a>

***[LD10](002-2-definitions.md#LD10)**. **Prohibited Break:***
No line break opportunity exists between two
characters of the given line breaking classes, even if they are separated
by one or more space characters.

- In the notation of the rules in
  *Section 6, [Line Breaking Algorithm](017-5-5-use-of-double-hyphen.md#Algorithm)*,
  a prohibited break is expressed as a rule of the form:
  **B** [SP](014-qu-quotation.md#SP)**\* × A**.

<a id="LD11"></a>

***[LD11](002-2-definitions.md#LD11)**. **Hyphenation:***
Hyphenation uses language-specific rules to provide
additional line break opportunities *within* a word.

- Hyphenation improves
  the layout of narrow columns, especially for languages with many longer
  words, such as German or Finnish. For the purpose of this annex, it is
  assumed that hyphenation is equivalent to inserting *soft hyphen*
  characters. All other aspects of hyphenation are outside the scope of this annex.

*Table 1* lists all of line breaking classes by name, also
giving their class abbreviation and their status as
tailorable or not. The examples and brief indication of line breaking
behavior in this table are merely typical, not exhaustive.
*Section 5.1, [Description of Line Breaking Properties](006-5-1-description-of-line-breaking-properties.md#DescriptionOfProperties)*,
provides a detailed description of each line breaking class, including
detailed overview of the line breaking behavior for characters of that
class.

<a id="Table1"></a>

Table 1. [Line Breaking Classes](002-2-definitions.md#Table1)

