# 7 Design Goals

Source: [UAX #15: Unicode Normalization Forms](https://www.unicode.org/reports/tr15/tr15-57.html) · Unicode 17.0.0, revision 57.

[Document index](../README.md) · [Topic guide](../../../../../README.md) · [Publisher](https://www.unicode.org/versions/Unicode17.0.0/).

HTML normalized to Markdown. Complex tables and mathematical markup retain HTML; illustrations are stored locally.

[Previous](007-5-2-composition-exclusion-data-files.md) · [Next](009-9-1-stable-code-points.md)

<a id="Design_Goals"></a>

## 7  [Design Goals](008-7-design-goals.md#Design_Goals)

The following are the design goals for the specification of the
Normalization Forms and are
presented here for reference. The first goal is a fundamental conformance feature of the
design.

#### Goal 1: Uniqueness

The first, and by far the most important, design goal for the Normalization
Forms is
uniqueness. Two equivalent strings will have *precisely* the same normalized form. More
explicitly,

1. If two strings x and y are canonical equivalents, then
   - toNFC(x) = toNFC(y)
   - toNFD(x) = toNFD(y)
2. If two strings are compatibility equivalents, then
   - toNFKC(x) = toNFKC(y)
   - toNFKD(x) = toNFKD(y)
3. All of the transformations are idempotent: that is,
   - toNFC(toNFC(x)) = toNFC(x)
   - toNFD(toNFD(x)) = toNFD(x)
   - toNFKC(toNFKC(x)) = toNFKC(x)
   - toNFKD(toNFKD(x)) = toNFKD(x)

Goal 1.3 is a consequence of Goals 1.2 and 1.1, but is stated here for clarity.

Another consequence of the definitions is that any chain of normalizations is equivalent to a single normalization, which is:

1. a compatibility normalization, if *any* normalization is a compatibility normalization
2. a composition normalization, if the *final* normalization is a composition normalization

For example,
the following table lists equivalent chains of two transformations:

| toNFC(x) | toNFD(x) | toNFKC(x) | toNFKD(x) |
| --- | --- | --- | --- |
| =toNFC(toNFC(x))  =toNFC(toNFD(x)) | =toNFD(toNFC(x))  =toNFD(toNFD(x)) | =toNFC(toNFKC(x))  =toNFC(toNFKD(x))  =toNFKC(toNFC(x))  =toNFKC(toNFD(x))  =toNFKC(toNFKC(x))  =toNFKC(toNFKD(x)) | =toNFD(toNFKC(x))  =toNFD(toNFKD(x))  =toNFKD(toNFC(x))  =toNFKD(toNFD(x))  =toNFKD(toNFKC(x))  =toNFKD(toNFKD(x)) |

#### Goal 2: Stability

The second major design goal for the Normalization Forms is stability of characters that are
not involved in the composition or decomposition process.

1. If a string x contains a character with a compatibility decomposition,
   then toNFD(x) and toNFC(x) still contain that character.
2. As much as possible, if there are no combining characters in x, then toNFC(x) = x.
   - The only characters for which this is not true are those in the
     *Section 5, [Composition Exclusion Table](006-5-composition-exclusion.md#Primary_Exclusion_List_Table)*.
3. Irrelevant combining marks should not affect the results of composition. See example **f**
   in *Section 6, [Examples and Charts](007-5-2-composition-exclusion-data-files.md#Examples),* where the *horn*
   character does not affect the results of composition.

#### Goal 3: Efficiency

The third major design goal for the Normalization Forms is to allow efficient
implementations.

1. It is possible to implement efficient code for producing the Normalization Forms. In
   particular, it should be possible to produce Normalization Form C very quickly from strings that
   are already in Normalization Form C or are in Normalization Form D.
2. Normalization Forms that compose do not have to produce the shortest possible results, because that can be
   computationally expensive.

<a id="Legacy_Encodings"></a>

## 8  [Legacy Encodings](008-7-design-goals.md#Legacy_Encodings)

While the Normalization Forms are specified for Unicode text, they can also be extended to
non-Unicode (legacy) character encodings. This is based on mapping the legacy character set
strings to and from Unicode using definitions UAX15-D1
and UAX15-D2.

<a id="UAX15-D1"></a>

***[UAX15-D1.](008-7-design-goals.md#UAX15-D1)*** An *invertible transcoding* T for a legacy character set L is
a one-to-one mapping from characters encoded in L to characters in Unicode with an associated
mapping T<sup>-1</sup> such that for any string S in L, T<sup>-1</sup>(T(S)) = S.

Most legacy character sets have a single invertible transcoding
in common use. In a few cases there may be multiple invertible transcodings. For example,
Shift-JIS may have two different mappings used in different circumstances: one to preserve the
'\'
semantics of 5C<sub>16</sub>, and one to preserve the '¥' semantics.

The character indexes in the legacy character set string may be different from
character indexes in the Unicode equivalent. For example, if a legacy string uses visual encoding
for Hebrew, then its first character might be the last character in the Unicode string.

If transcoders are implemented for legacy character sets, it is recommended that the result be
in Normalization Form C where possible. See Unicode Technical
Standard #22, “Unicode Character Mapping Markup Language” [[UTS22](https://www.unicode.org/reports/tr41/tr41-36.html#UTS22)] for more information.

<a id="UAX15-D2"></a>

***[UAX15-D2.](008-7-design-goals.md#UAX15-D2)*** Given a string S encoded in L and an invertible transcoding T for
L, the *Normalization Form X of S under T* is defined to be the result of mapping to Unicode,
normalizing to Unicode Normalization Form X, and mapping back to the legacy character encoding—for example, T<sup>-1</sup>(toNFx(T(S))). Where there is a single invertible transcoding for that character set in common use, one can simply speak of the
Normalization Form X of S.

Legacy character sets are classified into three categories based on their normalization behavior with
accepted transcoders.

1. *Prenormalized.* Any string in the character set is already in Normalization Form X.
   - For example, ISO 8859-1 is prenormalized in NFC.
2. *Normalizable.* Although the set is not prenormalized, any string in the set
   *can*
   be normalized to Normalization Form X.
   - For example, ISO 2022 (with a mixture of ISO 5426 and ISO 8859-1) is normalizable.
3. *Unnormalizable.* Some strings in the character set cannot be normalized into
   Normalization Form X.
   - For example, ISO 5426 is unnormalizable in NFC under common transcoders, because it
     contains combining marks but not composites.

<a id="Detecting_Normalization_Forms"></a>

## 9  [Detecting Normalization Forms](008-7-design-goals.md#Detecting_Normalization_Forms)

The Unicode Character Database supplies properties that allow implementations to quickly
determine whether a string x is in a particular Normalization Form—for example, isNFC(x). This is, in general, many times faster than normalizing and then comparing.

For each Normalization Form, the properties provide three possible values for each Unicode code
point, as shown in *Table 9*.

<a id="Quick_Check_Table"></a>

Table 9. [Description of Quick\_Check Values](008-7-design-goals.md#Quick_Check_Table)

| Values | Abbr | Description |
| --- | --- | --- |
| NO | N | The code point cannot occur in that Normalization Form. |
| YES | Y | The code point is a starter and can occur in the Normalization Form. In addition, for NFKC and NFC, the character may compose with a following character, but it *never* composes with a previous character. Furthermore, if the Decomposition\_Mapping of the character is more than one code point in length, the *first* code point in that Decomposition\_Mapping *must* also have the corresponding Quick\_Check value YES. |
| MAYBE | M | The code point can occur, subject to canonical ordering, but with constraints. In particular, the text might not be in the specified Normalization Form depending on the context in which the character occurs. |

Code that uses this property can do a *very* fast first pass over a string to determine
the Normalization Form. The result is also either NO, YES, or MAYBE. For NO or YES, the answer is
definite. In the MAYBE case, a more thorough check must be made, typically by putting a copy of
the string into the Normalization Form and checking for equality with the original.

- Even the slow case can be optimized, with a function that does not perform a complete
  normalization of the entire string, but instead works incrementally, only normalizing a limited
  area around the MAYBE character. See *Section 9.1,
  [Stable Code Points](009-9-1-stable-code-points.md#Stable_Code_Points)*.

This check is much faster than simply running the normalization algorithm, because it avoids
any memory allocation and copying. The vast majority of strings will return a definitive YES or NO
answer, leaving only a small percentage that require more work. The sample below is written in
Java, although for accessibility it avoids the use of object-oriented techniques.



```
public int quickCheck(String source) {
    short lastCanonicalClass = 0;
    int result = YES;
    for (int i = 0; i < source.length(); ++i) {
        int ch = source.codepointAt(i);
        if (Character.isSupplementaryCodePoint(ch)) ++i;
        short canonicalClass = getCanonicalClass(ch);
        if (lastCanonicalClass > canonicalClass && canonicalClass != 0) {
            return NO;        }
        int check = isAllowed(ch);
        if (check == NO) return NO;
        if (check == MAYBE) result = MAYBE;
        lastCanonicalClass = canonicalClass;
    }
    return result;
}
public static final int NO = 0, YES = 1, MAYBE = -1;
```




The `isAllowed()` call should access the data from Derived Normalization Properties
file [[NormProps](https://www.unicode.org/reports/tr41/tr41-36.html#NormProps)] for the
Normalization Form in question.  (For more
information, see Unicode Standard Annex #44, "Unicode Character Database"
[[UAX44](https://www.unicode.org/reports/tr41/tr41-36.html#UAX44)].) For example, here is a
segment of the data for NFC:



```
...
0338 ; NFC_QC; M # Mn COMBINING LONG SOLIDUS OVERLAY
...

F900..FA0D ; NFD_QC; N # Lo [270] CJK COMPATIBILITY IDEOGRAPH-F900..CJK COMPATIBILITY IDEOGRAPH-FA0D
...
```




These lines assign the value NFC\_QC==MAYBE to the code point U+0338, and the value NFC\_QC==NO to the
code points in the range U+F900..U+FA0D. There are no MAYBE values for NFD and NFKD:
the `quickCheck` function will always produce a definite result for these
Normalization Forms. All characters that are not specifically mentioned in the file have the values YES.

The data for the implementation of the `isAllowed()` call can be accessed in memory
with a hash table or a trie (see *Section 14,
[Implementation Notes](013-14-implementation-notes.md#Implementation_Notes)*); the latter will be the fastest.

There is also a Unicode
Consortium stability policy that canonical mappings are always limited in
all versions of Unicode, so that no string when decomposed with NFC expands
to more than 3× in length (measured in code units). This is true whether the
text is in UTF-8, UTF-16, or UTF-32. This guarantee also allows for certain
optimizations in processing, especially in determining buffer sizes. See
also *Section 13,
[Stream-Safe Text Format](012-13-stream-safe-text-format.md#Stream_Safe_Text_Format)*.

