# Continuation 3

Source: [Mathematical typesetting table](https://learn.microsoft.com/en-us/typography/opentype/spec/math) · OpenType 1.9.1 website snapshot.

[Document index](../README.md) · [Topic guide](../../../../../README.md) · [Publisher](https://learn.microsoft.com/en-us/typography/opentype/spec/math).

HTML normalized to Markdown. Complex tables and mathematical markup retain HTML; illustrations are stored locally.

[Previous](002-math-header.md) · [Next](004-mathglyphinfo-table.md)

| Type | Name | Description |
| --- | --- | --- |
| int16 | scriptPercentScaleDown | Percentage of scaling down for level 1 superscripts and subscripts. Suggested value: 80%. |
| int16 | scriptScriptPercentScaleDown | Percentage of scaling down for level 2 (scriptScript) superscripts and subscripts. Suggested value: 60%. |
| UFWORD | delimitedSubFormulaMinHeight | Minimum height required for a delimited expression (contained within parentheses, etc.) to be treated as a sub-formula. Suggested value: normal line height × 1.5. |
| UFWORD | displayOperatorMinHeight | Minimum height of n-ary operators (such as integral and summation) for formulas in display mode (that is, appearing as standalone page elements, not embedded inline within text). |
| MathValueRecord | mathLeading | White space to be left between math formulas to ensure proper line spacing. For example, for applications that treat line gap as a part of line ascender, formulas with ink going above (os2.sTypoAscender + os2.sTypoLineGap - MathLeading) or with ink going below os2.sTypoDescender will result in increasing line height. |
| MathValueRecord | axisHeight | Axis height of the font.  In math typesetting, the term axis refers to a horizontal reference line used for positioning elements in a formula. The math axis is similar to but distinct from the baseline for regular text layout. For example, in a simple equation, a minus symbol or fraction rule would be on the axis, but a string for a variable name would be set on a baseline that is offset from the axis. The axisHeight value determines the amount of that offset. |
| MathValueRecord | accentBaseHeight | Maximum (ink) height of accent base that does not require raising the accents. Suggested: x‑height of the font (os2.sxHeight) plus any possible overshots. |
| MathValueRecord | flattenedAccentBaseHeight | Maximum (ink) height of accent base that does not require flattening the accents. Suggested: cap height of the font (os2.sCapHeight). |
| MathValueRecord | subscriptShiftDown | The standard shift down applied to subscript elements. Positive for moving in the downward direction. Suggested: os2.ySubscriptYOffset. |
| MathValueRecord | subscriptTopMax | Maximum allowed height of the (ink) top of subscripts that does not require moving subscripts further down. Suggested: 4/5 x- height. |
| MathValueRecord | subscriptBaselineDropMin | Minimum allowed drop of the baseline of subscripts relative to the (ink) bottom of the base. Checked for bases that are treated as a box or extended shape. Positive for subscript baseline dropped below the base bottom. |
| MathValueRecord | superscriptShiftUp | Standard shift up applied to superscript elements. Suggested: os2.ySuperscriptYOffset. |
| MathValueRecord | superscriptShiftUpCramped | Standard shift of superscripts relative to the base, in cramped style. |
| MathValueRecord | superscriptBottomMin | Minimum allowed height of the (ink) bottom of superscripts that does not require moving subscripts further up. Suggested: ¼ x-height. |
| MathValueRecord | superscriptBaselineDropMax | Maximum allowed drop of the baseline of superscripts relative to the (ink) top of the base. Checked for bases that are treated as a box or extended shape. Positive for superscript baseline below the base top. |
| MathValueRecord | subSuperscriptGapMin | Minimum gap between the superscript and subscript ink. Suggested: 4 × default rule thickness. |
| MathValueRecord | superscriptBottomMaxWithSubscript | The maximum level to which the (ink) bottom of superscript can be pushed to increase the gap between superscript and subscript, before subscript starts being moved down. Suggested: 4/5 x-height. |
| MathValueRecord | spaceAfterScript | Extra white space to be added after each subscript and superscript that occurs after a baseline element, and before each subscript and superscript that occurs before a baseline element. Suggested: 0.5 pt for a 12 pt font.  Note that, in some math layout implementations, a constant value, such as 0.5 pt, could be used for all text sizes. Some implementations could use a constant ratio of text size, such as 1/24 of em. |
| MathValueRecord | upperLimitGapMin | Minimum gap between the (ink) bottom of the upper limit, and the (ink) top of the base operator. |
| MathValueRecord | upperLimitBaselineRiseMin | Minimum distance between baseline of upper limit and (ink) top of the base operator. |
| MathValueRecord | lowerLimitGapMin | Minimum gap between (ink) top of the lower limit, and (ink) bottom of the base operator. |
| MathValueRecord | lowerLimitBaselineDropMin | Minimum distance between baseline of the lower limit and (ink) bottom of the base operator. |
| MathValueRecord | stackTopShiftUp | Standard shift up applied to the top element of a stack. |
| MathValueRecord | stackTopDisplayStyleShiftUp | Standard shift up applied to the top element of a stack in display style. |
| MathValueRecord | stackBottomShiftDown | Standard shift down applied to the bottom element of a stack. Positive for moving in the downward direction. |
| MathValueRecord | stackBottomDisplayStyleShiftDown | Standard shift down applied to the bottom element of a stack in display style. Positive for moving in the downward direction. |
| MathValueRecord | stackGapMin | Minimum gap between (ink) bottom of the top element of a stack, and the (ink) top of the bottom element. Suggested: 3 × default rule thickness. |
| MathValueRecord | stackDisplayStyleGapMin | Minimum gap between (ink) bottom of the top element of a stack, and the (ink) top of the bottom element in display style. Suggested: 7 × default rule thickness. |
| MathValueRecord | stretchStackTopShiftUp | Standard shift up applied to the top element of the stretch stack. |
| MathValueRecord | stretchStackBottomShiftDown | Standard shift down applied to the bottom element of the stretch stack. Positive for moving in the downward direction. |
| MathValueRecord | stretchStackGapAboveMin | Minimum gap between the ink of the stretched element, and the (ink) bottom of the element above. Suggested: same value as upperLimitGapMin. |
| MathValueRecord | stretchStackGapBelowMin | Minimum gap between the ink of the stretched element, and the (ink) top of the element below. Suggested: same value as lowerLimitGapMin. |
| MathValueRecord | fractionNumeratorShiftUp | Standard shift up applied to the numerator. |
| MathValueRecord | fractionNumeratorDisplayStyleShiftUp | Standard shift up applied to the numerator in display style. Suggested: same value as stackTopDisplayStyleShiftUp. |
| MathValueRecord | fractionDenominatorShiftDown | Standard shift down applied to the denominator. Positive for moving in the downward direction. |
| MathValueRecord | fractionDenominatorDisplayStyleShiftDown | Standard shift down applied to the denominator in display style. Positive for moving in the downward direction. Suggested: same value as stackBottomDisplayStyleShiftDown. |
| MathValueRecord | fractionNumeratorGapMin | Minimum tolerated gap between the (ink) bottom of the numerator and the ink of the fraction bar. Suggested: default rule thickness. |
| MathValueRecord | fractionNumDisplayStyleGapMin | Minimum tolerated gap between the (ink) bottom of the numerator and the ink of the fraction bar in display style. Suggested: 3 × default rule thickness. |
| MathValueRecord | fractionRuleThickness | Thickness of the fraction bar. Suggested: default rule thickness. |
| MathValueRecord | fractionDenominatorGapMin | Minimum tolerated gap between the (ink) top of the denominator and the ink of the fraction bar. Suggested: default rule thickness. |
| MathValueRecord | fractionDenomDisplayStyleGapMin | Minimum tolerated gap between the (ink) top of the denominator and the ink of the fraction bar in display style. Suggested: 3 × default rule thickness. |
| MathValueRecord | skewedFractionHorizontalGap | Horizontal distance between the top and bottom elements of a skewed fraction. |
| MathValueRecord | skewedFractionVerticalGap | Vertical distance between the ink of the top and bottom elements of a skewed fraction. |
| MathValueRecord | overbarVerticalGap | Distance between the overbar and the (ink) top of the base. Suggested: 3 × default rule thickness. |
| MathValueRecord | overbarRuleThickness | Thickness of overbar. Suggested: default rule thickness. |
| MathValueRecord | overbarExtraAscender | Extra white space reserved above the overbar. Suggested: default rule thickness. |
| MathValueRecord | underbarVerticalGap | Distance between underbar and (ink) bottom of the base. Suggested: 3 × default rule thickness. |
| MathValueRecord | underbarRuleThickness | Thickness of underbar. Suggested: default rule thickness. |
| MathValueRecord | underbarExtraDescender | Extra white space reserved below the underbar. Always positive. Suggested: default rule thickness. |
| MathValueRecord | radicalVerticalGap | Space between the (ink) top of the expression and the bar over it. Suggested: 1¼ default rule thickness. |
| MathValueRecord | radicalDisplayStyleVerticalGap | Space between the (ink) top of the expression and the bar over it. Suggested: default rule thickness + ¼ x-height. |
| MathValueRecord | radicalRuleThickness | Thickness of the radical rule. This is the thickness of the rule in designed or constructed radical signs. Suggested: default rule thickness. |
| MathValueRecord | radicalExtraAscender | Extra white space reserved above the radical. Suggested: same value as radicalRuleThickness. |
| MathValueRecord | radicalKernBeforeDegree | Extra horizontal kern before the degree of a radical, if such is present. Suggested: 5/18 of em. |
| MathValueRecord | radicalKernAfterDegree | Negative kern after the degree of a radical, if such is present. Suggested: −10/18 of em. |
| int16 | radicalDegreeBottomRaisePercent | Height of the bottom of the radical degree, if such is present, in proportion to the height (ascender + descender) of the radical sign. Suggested: 60%. |

