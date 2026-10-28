# Continuation 7

Source: [OS/2 and Windows metrics](https://learn.microsoft.com/en-us/typography/opentype/spec/os2) · OpenType 1.9.1 website snapshot.

[Document index](../README.md) · [Topic guide](../../../../../README.md) · [Publisher](https://learn.microsoft.com/en-us/typography/opentype/spec/os2).

HTML normalized to Markdown. Complex tables and mathematical markup retain HTML; illustrations are stored locally.

[Previous](006-ystrikeoutsize.md) · [Next](008-achvendid.md)

<table>
<tbody>
<tr>
<td>Format:</td>
<td>uint32[4] (128 bits total)</td>
</tr>
<tr>
<td>Title:</td>
<td>Unicode Character Range</td>
</tr>
<tr>
<td>Description:</td>
<td>This field is used to specify the Unicode blocks or ranges encompassed by the font file in 'cmap' subtables for platform 3, encoding ID 1 (Microsoft platform, Unicode BMP) and platform 3, encoding ID 10 (Microsoft platform, Unicode full repertoire). If a bit is set (1), then the Unicode ranges assigned to that bit are considered functional. If the bit is clear (0), then the range is not considered functional. Each of the bits is treated as an independent flag and the bits can be set in any combination. The determination of “functional” is left up to the font designer, although character set selection should attempt to be functional by ranges if at all possible.
<br/><br/>
<b>All reserved fields must be zero.</b> Each uint32 is in Big-Endian form.
<table>
<tbody>
<tr>
<th>Bit</th>
<th>Unicode Range</th>
<th>Block range</th>
<th>Notes</th>
</tr>
<tr>
<td>0</td>
<td>Basic Latin</td>
<td>0000-007F</td>
<td></td>
</tr>
<tr>
<td>1</td>
<td>Latin-1 Supplement</td>
<td>0080-00FF</td>
<td></td>
</tr>
<tr>
<td>2</td>
<td>Latin Extended-A</td>
<td>0100-017F</td>
<td></td>
</tr>
<tr>
<td>3</td>
<td>Latin Extended-B</td>
<td>0180-024F</td>
<td></td>
</tr>
<tr>
<td>4</td>
<td>IPA Extensions</td>
<td>0250-02AF</td>
<td></td>
</tr>
<tr>
<td></td>
<td>Phonetic Extensions</td>
<td>1D00-1D7F</td>
<td>Added in OpenType 1.5 for OS/2 version 4.</td>
</tr>
<tr>
<td></td>
<td>Phonetic Extensions Supplement</td>
<td>1D80-1DBF</td>
<td>Added in OpenType 1.5 for OS/2 version 4.</td>
</tr>
<tr>
<td>5</td>
<td>Spacing Modifier Letters</td>
<td>02B0-02FF</td>
<td></td>
</tr>
<tr>
<td></td>
<td>Modifier Tone Letters</td>
<td>A700-A71F</td>
<td>Added in OpenType 1.5 for OS/2 version 4.</td>
</tr>
<tr>
<td>6</td>
<td>Combining Diacritical Marks</td>
<td>0300-036F</td>
<td></td>
</tr>
<tr>
<td></td>
<td>Combining Diacritical Marks Supplement</td>
<td>1DC0-1DFF</td>
<td>Added in OpenType 1.5 for OS/2 version 4.</td>
</tr>
<tr>
<td>7</td>
<td>Greek and Coptic</td>
<td>0370-03FF</td>
<td></td>
</tr>
<tr>
<td>8</td>
<td>Coptic</td>
<td>2C80-2CFF</td>
<td>Added in OpenType 1.5 for OS/2 version 4. See below for other version differences.</td>
</tr>
<tr>
<td>9</td>
<td>Cyrillic</td>
<td>0400-04FF</td>
<td></td>
</tr>
<tr>
<td></td>
<td>Cyrillic Supplement</td>
<td>0500-052F</td>
<td>Added in OpenType 1.4 for OS/2 version 3.</td>
</tr>
<tr>
<td></td>
<td>Cyrillic Extended-A</td>
<td>2DE0-2DFF</td>
<td>Added in OpenType 1.5 for OS/2 version 4.</td>
</tr>
<tr>
<td></td>
<td>Cyrillic Extended-B</td>
<td>A640-A69F</td>
<td>Added in OpenType 1.5 for OS/2 version 4.</td>
</tr>
<tr>
<td>10</td>
<td>Armenian</td>
<td>0530-058F</td>
<td></td>
</tr>
<tr>
<td>11</td>
<td>Hebrew</td>
<td>0590-05FF</td>
<td></td>
</tr>
<tr>
<td>12</td>
<td>Vai</td>
<td>A500-A63F</td>
<td>Added in OpenType 1.5 for OS/2 version 4. See below for other version differences.</td>
</tr>
<tr>
<td>13</td>
<td>Arabic</td>
<td>0600-06FF</td>
<td></td>
</tr>
<tr>
<td></td>
<td>Arabic Supplement</td>
<td>0750-077F</td>
<td>Added in OpenType 1.5 for OS/2 version 4.</td>
</tr>
<tr>
<td>14</td>
<td>NKo</td>
<td>07C0-07FF</td>
<td>Added in OpenType 1.5 for OS/2 version 4. See below for other version differences.</td>
</tr>
<tr>
<td>15</td>
<td>Devanagari</td>
<td>0900-097F</td>
<td></td>
</tr>
<tr>
<td>16</td>
<td>Bangla</td>
<td>0980-09FF</td>
<td></td>
</tr>
<tr>
<td>17</td>
<td>Gurmukhi</td>
<td>0A00-0A7F</td>
<td></td>
</tr>
<tr>
<td>18</td>
<td>Gujarati</td>
<td>0A80-0AFF</td>
<td></td>
</tr>
<tr>
<td>19</td>
<td>Odia</td>
<td>0B00-0B7F</td>
<td></td>
</tr>
<tr>
<td>20</td>
<td>Tamil</td>
<td>0B80-0BFF</td>
<td></td>
</tr>
<tr>
<td>21</td>
<td>Telugu</td>
<td>0C00-0C7F</td>
<td></td>
</tr>
<tr>
<td>22</td>
<td>Kannada</td>
<td>0C80-0CFF</td>
<td></td>
</tr>
<tr>
<td>23</td>
<td>Malayalam</td>
<td>0D00-0D7F</td>
<td></td>
</tr>
<tr>
<td>24</td>
<td>Thai</td>
<td>0E00-0E7F</td>
<td></td>
</tr>
<tr>
<td>25</td>
<td>Lao</td>
<td>0E80-0EFF</td>
<td></td>
</tr>
<tr>
<td>26</td>
<td>Georgian</td>
<td>10A0-10FF</td>
<td></td>
</tr>
<tr>
<td></td>
<td>Georgian Supplement</td>
<td>2D00-2D2F</td>
<td>Added in OpenType 1.5 for OS/2 version 4.</td>
</tr>
<tr>
<td>27</td>
<td>Balinese</td>
<td>1B00-1B7F</td>
<td>Added in OpenType 1.5 for OS/2 version 4. See below for other version differences.</td>
</tr>
<tr>
<td>28</td>
<td>Hangul Jamo</td>
<td>1100-11FF</td>
<td></td>
</tr>
<tr>
<td>29</td>
<td>Latin Extended Additional</td>
<td>1E00-1EFF</td>
<td></td>
</tr>
<tr>
<td></td>
<td>Latin Extended-C</td>
<td>2C60-2C7F</td>
<td>Added in OpenType 1.5 for OS/2 version 4.</td>
</tr>
<tr>
<td></td>
<td>Latin Extended-D</td>
<td>A720-A7FF</td>
<td>Added in OpenType 1.5 for OS/2 version 4.</td>
</tr>
<tr>
<td>30</td>
<td>Greek Extended</td>
<td>1F00-1FFF</td>
<td></td>
</tr>
<tr>
<td>31</td>
<td>General Punctuation</td>
<td>2000-206F</td>
<td></td>
</tr>
<tr>
<td></td>
<td>Supplemental Punctuation</td>
<td>2E00-2E7F</td>
<td>Added in OpenType 1.5 for OS/2 version 4.</td>
</tr>
<tr>
<td>32</td>
<td>Superscripts And Subscripts</td>
<td>2070-209F</td>
<td></td>
</tr>
<tr>
<td>33</td>
<td>Currency Symbols</td>
<td>20A0-20CF</td>
<td></td>
</tr>
<tr>
<td>34</td>
<td>Combining Diacritical Marks For Symbols</td>
<td>20D0-20FF</td>
<td></td>
</tr>
<tr>
<td>35</td>
<td>Letterlike Symbols</td>
<td>2100-214F</td>
<td></td>
</tr>
<tr>
<td>36</td>
<td>Number Forms</td>
<td>2150-218F</td>
<td></td>
</tr>
<tr>
<td>37</td>
<td>Arrows</td>
<td>2190-21FF</td>
<td></td>
</tr>
<tr>
<td></td>
<td>Supplemental Arrows-A</td>
<td>27F0-27FF</td>
<td>Added in OpenType 1.4 for OS/2 version 3.</td>
</tr>
<tr>
<td></td>
<td>Supplemental Arrows-B</td>
<td>2900-297F</td>
<td>Added in OpenType 1.4 for OS/2 version 3.</td>
</tr>
<tr>
<td></td>
<td>Miscellaneous Symbols and Arrows</td>
<td>2B00-2BFF</td>
<td>Added in OpenType 1.5 for OS/2 version 4.</td>
</tr>
<tr>
<td>38</td>
<td>Mathematical Operators</td>
<td>2200-22FF</td>
<td></td>
</tr>
<tr>
<td></td>
<td>Supplemental Mathematical Operators</td>
<td>2A00-2AFF</td>
<td>Added in OpenType 1.4 for OS/2 version 3.</td>
</tr>
<tr>
<td></td>
<td>Miscellaneous Mathematical Symbols-A</td>
<td>27C0-27EF</td>
<td>Added in OpenType 1.4 for OS/2 version 3.</td>
</tr>
<tr>
<td></td>
<td>Miscellaneous Mathematical Symbols-B</td>
<td>2980-29FF</td>
<td>Added in OpenType 1.4 for OS/2 version 3.</td>
</tr>
<tr>
<td>39</td>
<td>Miscellaneous Technical</td>
<td>2300-23FF</td>
<td></td>
</tr>
<tr>
<td>40</td>
<td>Control Pictures</td>
<td>2400-243F</td>
<td></td>
</tr>
<tr>
<td>41</td>
<td>Optical Character Recognition</td>
<td>2440-245F</td>
<td></td>
</tr>
<tr>
<td>42</td>
<td>Enclosed Alphanumerics</td>
<td>2460-24FF</td>
<td></td>
</tr>
<tr>
<td>43</td>
<td>Box Drawing</td>
<td>2500-257F</td>
<td></td>
</tr>
<tr>
<td>44</td>
<td>Block Elements</td>
<td>2580-259F</td>
<td></td>
</tr>
<tr>
<td>45</td>
<td>Geometric Shapes</td>
<td>25A0-25FF</td>
<td></td>
</tr>
<tr>
<td>46</td>
<td>Miscellaneous Symbols</td>
<td>2600-26FF</td>
<td></td>
</tr>
<tr>
<td>47</td>
<td>Dingbats</td>
<td>2700-27BF</td>
<td></td>
</tr>
<tr>
<td>48</td>
<td>CJK Symbols And Punctuation</td>
<td>3000-303F</td>
<td></td>
</tr>
<tr>
<td>49</td>
<td>Hiragana</td>
<td>3040-309F</td>
<td></td>
</tr>
<tr>
<td>50</td>
<td>Katakana</td>
<td>30A0-30FF</td>
<td></td>
</tr>
<tr>
<td></td>
<td>Katakana Phonetic Extensions</td>
<td>31F0-31FF</td>
<td>Added in OpenType 1.4 for OS/2 version 3.</td>
</tr>
<tr>
<td>51</td>
<td>Bopomofo</td>
<td>3100-312F</td>
<td></td>
</tr>
<tr>
<td></td>
<td>Bopomofo Extended</td>
<td>31A0-31BF</td>
<td>Added in OpenType 1.3, extending OS/2 version 2.</td>
</tr>
<tr>
<td>52</td>
<td>Hangul Compatibility Jamo</td>
<td>3130-318F</td>
<td></td>
</tr>
<tr>
<td>53</td>
<td>Phags-pa</td>
<td>A840-A87F</td>
<td>Added in OpenType 1.5 for OS/2 version 4. See below for other version differences.</td>
</tr>
<tr>
<td>54</td>
<td>Enclosed CJK Letters And Months</td>
<td>3200-32FF</td>
<td></td>
</tr>
<tr>
<td>55</td>
<td>CJK Compatibility</td>
<td>3300-33FF</td>
<td></td>
</tr>
<tr>
<td>56</td>
<td>Hangul Syllables</td>
<td>AC00-D7AF</td>
<td></td>
</tr>
<tr>
<td>57</td>
<td>Non-Plane 0</td>
<td>10000-10FFFF</td>
<td>Setting this bit implies there is at least one character beyond the Basic Multilingual Plane supported by this font. First assigned in OpenType 1.3 for OS/2 version 2.</td>
</tr>
<tr>
<td>58</td>
<td>Phoenician</td>
<td>10900-1091F</td>
<td>First assigned in OpenType 1.5 for OS/2 version 4.</td>
</tr>
<tr>
<td>59</td>
<td>CJK Unified Ideographs</td>
<td>4E00-9FFF</td>
<td></td>
</tr>
<tr>
<td></td>
<td>CJK Radicals Supplement</td>
<td>2E80-2EFF</td>
<td>Added in OpenType 1.3 for OS/2 version 2.</td>
</tr>
<tr>
<td></td>
<td>Kangxi Radicals</td>
<td>2F00-2FDF</td>
<td>Added in OpenType 1.3 for OS/2 version 2.</td>
</tr>
<tr>
<td></td>
<td>Ideographic Description Characters</td>
<td>2FF0-2FFF</td>
<td>Added in OpenType 1.3 for OS/2 version 2.</td>
</tr>
<tr>
<td></td>
<td>CJK Unified Ideographs Extension A</td>
<td>3400-4DBF</td>
<td>Added in OpenType 1.3 for OS/2 version 2.</td>
</tr>
<tr>
<td></td>
<td>CJK Unified Ideographs Extension B</td>
<td>20000-2A6DF</td>
<td>Added in OpenType 1.4 for OS/2 version 3.</td>
</tr>
<tr>
<td></td>
<td>Kanbun</td>
<td>3190-319F</td>
<td>Added in OpenType 1.4 for OS/2 version 3.</td>
</tr>
<tr>
<td>60</td>
<td>Private Use Area (plane 0)</td>
<td>E000-F8FF</td>
<td></td>
</tr>
<tr>
<td>61</td>
<td>CJK Strokes</td>
<td>31C0-31EF</td>
<td>Added in OpenType 1.5 for OS/2 version 4.</td>
</tr>
<tr>
<td></td>
<td>CJK Compatibility Ideographs</td>
<td>F900-FAFF</td>
<td></td>
</tr>
<tr>
<td></td>
<td>CJK Compatibility Ideographs Supplement</td>
<td>2F800-2FA1F</td>
<td>Added in OpenType 1.4 for OS/2 version 3.</td>
</tr>
<tr>
<td>62</td>
<td>Alphabetic Presentation Forms</td>
<td>FB00-FB4F</td>
<td></td>
</tr>
<tr>
<td>63</td>
<td>Arabic Presentation Forms-A</td>
<td>FB50-FDFF</td>
<td></td>
</tr>
<tr>
<td>64</td>
<td>Combining Half Marks</td>
<td>FE20-FE2F</td>
<td></td>
</tr>
<tr>
<td>65</td>
<td>Vertical Forms</td>
<td>FE10-FE1F</td>
<td>Added in OpenType 1.5 for OS/2 version 4.</td>
</tr>
<tr>
<td></td>
<td>CJK Compatibility Forms</td>
<td>FE30-FE4F</td>
<td></td>
</tr>
<tr>
<td>66</td>
<td>Small Form Variants</td>
<td>FE50-FE6F</td>
<td></td>
</tr>
<tr>
<td>67</td>
<td>Arabic Presentation Forms-B</td>
<td>FE70-FEFF</td>
<td></td>
</tr>
<tr>
<td>68</td>
<td>Halfwidth And Fullwidth Forms</td>
<td>FF00-FFEF</td>
<td></td>
</tr>
<tr>
<td>69</td>
<td>Specials</td>
<td>FFF0-FFFF</td>
<td></td>
</tr>
<tr>
<td>70</td>
<td>Tibetan</td>
<td>0F00-0FFF</td>
<td>First assigned in OpenType 1.3, extending OS/2 version 2.</td>
</tr>
<tr>
<td>71</td>
<td>Syriac</td>
<td>0700-074F</td>
<td>First assigned in OpenType 1.3, extending OS/2 version 2.</td>
</tr>
<tr>
<td>72</td>
<td>Thaana</td>
<td>0780-07BF</td>
<td>First assigned in OpenType 1.3, extending OS/2 version 2.</td>
</tr>
<tr>
<td>73</td>
<td>Sinhala</td>
<td>0D80-0DFF</td>
<td>First assigned in OpenType 1.3, extending OS/2 version 2.</td>
</tr>
<tr>
<td>74</td>
<td>Myanmar</td>
<td>1000-109F</td>
<td>First assigned in OpenType 1.3, extending OS/2 version 2.</td>
</tr>
<tr>
<td>75</td>
<td>Ethiopic</td>
<td>1200-137F</td>
<td>First assigned in OpenType 1.3, extending OS/2 version 2.</td>
</tr>
<tr>
<td></td>
<td>Ethiopic Supplement</td>
<td>1380-139F</td>
<td>Added in OpenType 1.5 for OS/2 version 4.</td>
</tr>
<tr>
<td></td>
<td>Ethiopic Extended</td>
<td>2D80-2DDF</td>
<td>Added in OpenType 1.5 for OS/2 version 4.</td>
</tr>
<tr>
<td>76</td>
<td>Cherokee</td>
<td>13A0-13FF</td>
<td>First assigned in OpenType 1.3, extending OS/2 version 2.</td>
</tr>
<tr>
<td>77</td>
<td>Unified Canadian Aboriginal Syllabics</td>
<td>1400-167F</td>
<td>First assigned in OpenType 1.3, extending OS/2 version 2.</td>
</tr>
<tr>
<td>78</td>
<td>Ogham</td>
<td>1680-169F</td>
<td>First assigned in OpenType 1.3, extending OS/2 version 2.</td>
</tr>
<tr>
<td>79</td>
<td>Runic</td>
<td>16A0-16FF</td>
<td>First assigned in OpenType 1.3, extending OS/2 version 2.</td>
</tr>
<tr>
<td>80</td>
<td>Khmer</td>
<td>1780-17FF</td>
<td>First assigned in OpenType 1.3, extending OS/2 version 2.</td>
</tr>
<tr>
<td></td>
<td>Khmer Symbols</td>
<td>19E0-19FF</td>
<td>Added in OpenType 1.5 for OS/2 version 4.</td>
</tr>
<tr>
<td>81</td>
<td>Mongolian</td>
<td>1800-18AF</td>
<td>First assigned in OpenType 1.3, extending OS/2 version 2.</td>
</tr>
<tr>
<td>82</td>
<td>Braille Patterns</td>
<td>2800-28FF</td>
<td>First assigned in OpenType 1.3, extending OS/2 version 2.</td>
</tr>
<tr>
<td>83</td>
<td>Yi Syllables</td>
<td>A000-A48F</td>
<td>First assigned in OpenType 1.3, extending OS/2 version 2.</td>
</tr>
<tr>
<td></td>
<td>Yi Radicals</td>
<td>A490-A4CF</td>
<td>Added in OpenType 1.3, extending OS/2 version 2.</td>
</tr>
<tr>
<td>84</td>
<td>Tagalog</td>
<td>1700-171F</td>
<td>First assigned in OpenType 1.4 for OS/2 version 3.</td>
</tr>
<tr>
<td></td>
<td>Hanunoo</td>
<td>1720-173F</td>
<td>Added in OpenType 1.4 for OS/2 version 3.</td>
</tr>
<tr>
<td></td>
<td>Buhid</td>
<td>1740-175F</td>
<td>Added in OpenType 1.4 for OS/2 version 3.</td>
</tr>
<tr>
<td></td>
<td>Tagbanwa</td>
<td>1760-177F</td>
<td>Added in OpenType 1.4 for OS/2 version 3.</td>
</tr>
<tr>
<td>85</td>
<td>Old Italic</td>
<td>10300-1032F</td>
<td>First assigned in OpenType 1.4 for OS/2 version 3.</td>
</tr>
<tr>
<td>86</td>
<td>Gothic</td>
<td>10330-1034F</td>
<td>First assigned in OpenType 1.4 for OS/2 version 3.</td>
</tr>
<tr>
<td>87</td>
<td>Deseret</td>
<td>10400-1044F</td>
<td>First assigned in OpenType 1.4 for OS/2 version 3.</td>
</tr>
<tr>
<td>88</td>
<td>Byzantine Musical Symbols</td>
<td>1D000-1D0FF</td>
<td>First assigned in OpenType 1.4 for OS/2 version 3.</td>
</tr>
<tr>
<td></td>
<td>Musical Symbols</td>
<td>1D100-1D1FF</td>
<td>Added in OpenType 1.4 for OS/2 version 3.</td>
</tr>
<tr>
<td></td>
<td>Ancient Greek Musical Notation</td>
<td>1D200-1D24F</td>
<td>Added in OpenType 1.5 for OS/2 version 4.</td>
</tr>
<tr>
<td>89</td>
<td>Mathematical Alphanumeric Symbols</td>
<td>1D400-1D7FF</td>
<td>First assigned in OpenType 1.4 for OS/2 version 3.</td>
</tr>
<tr>
<td>90</td>
<td>Private Use (plane 15)</td>
<td>F0000-FFFFD</td>
<td>First assigned in OpenType 1.4 for OS/2 version 3.</td>
</tr>
<tr>
<td></td>
<td>Private Use (plane 16)</td>
<td>100000-10FFFD</td>
<td>Added in OpenType 1.4 for OS/2 version 3.</td>
</tr>
<tr>
<td>91</td>
<td>Variation Selectors</td>
<td>FE00-FE0F</td>
<td>First assigned in OpenType 1.4 for OS/2 version 3.</td>
</tr>
<tr>
<td></td>
<td>Variation Selectors Supplement</td>
<td>E0100-E01EF</td>
<td>Added in OpenType 1.4 for OS/2 version 3.</td>
</tr>
<tr>
<td>92</td>
<td>Tags</td>
<td>E0000-E007F</td>
<td>First assigned in OpenType 1.4 for OS/2 version 3.</td>
</tr>
<tr>
<td>93</td>
<td>Limbu</td>
<td>1900-194F</td>
<td>First assigned in OpenType 1.5 for OS/2 version 4.</td>
</tr>
<tr>
<td>94</td>
<td>Tai Le</td>
<td>1950-197F</td>
<td>First assigned in OpenType 1.5 for OS/2 version 4.</td>
</tr>
<tr>
<td>95</td>
<td>New Tai Lue</td>
<td>1980-19DF</td>
<td>First assigned in OpenType 1.5 for OS/2 version 4.</td>
</tr>
<tr>
<td>96</td>
<td>Buginese</td>
<td>1A00-1A1F</td>
<td>First assigned in OpenType 1.5 for OS/2 version 4.</td>
</tr>
<tr>
<td>97</td>
<td>Glagolitic</td>
<td>2C00-2C5F</td>
<td>First assigned in OpenType 1.5 for OS/2 version 4.</td>
</tr>
<tr>
<td>98</td>
<td>Tifinagh</td>
<td>2D30-2D7F</td>
<td>First assigned in OpenType 1.5 for OS/2 version 4.</td>
</tr>
<tr>
<td>99</td>
<td>Yijing Hexagram Symbols</td>
<td>4DC0-4DFF</td>
<td>First assigned in OpenType 1.5 for OS/2 version 4.</td>
</tr>
<tr>
<td>100</td>
<td>Syloti Nagri</td>
<td>A800-A82F</td>
<td>First assigned in OpenType 1.5 for OS/2 version 4.</td>
</tr>
<tr>
<td>101</td>
<td>Linear B Syllabary</td>
<td>10000-1007F</td>
<td>First assigned in OpenType 1.5 for OS/2 version 4.</td>
</tr>
<tr>
<td></td>
<td>Linear B Ideograms</td>
<td>10080-100FF</td>
<td>Added in OpenType 1.5 for OS/2 version 4.</td>
</tr>
<tr>
<td></td>
<td>Aegean Numbers</td>
<td>10100-1013F</td>
<td>Added in OpenType 1.5 for OS/2 version 4.</td>
</tr>
<tr>
<td>102</td>
<td>Ancient Greek Numbers</td>
<td>10140-1018F</td>
<td>First assigned in OpenType 1.5 for OS/2 version 4.</td>
</tr>
<tr>
<td>103</td>
<td>Ugaritic</td>
<td>10380-1039F</td>
<td>First assigned in OpenType 1.5 for OS/2 version 4.</td>
</tr>
<tr>
<td>104</td>
<td>Old Persian</td>
<td>103A0-103DF</td>
<td>First assigned in OpenType 1.5 for OS/2 version 4.</td>
</tr>
<tr>
<td>105</td>
<td>Shavian</td>
<td>10450-1047F</td>
<td>First assigned in OpenType 1.5 for OS/2 version 4.</td>
</tr>
<tr>
<td>106</td>
<td>Osmanya</td>
<td>10480-104AF</td>
<td>First assigned in OpenType 1.5 for OS/2 version 4.</td>
</tr>
<tr>
<td>107</td>
<td>Cypriot Syllabary</td>
<td>10800-1083F</td>
<td>First assigned in OpenType 1.5 for OS/2 version 4.</td>
</tr>
<tr>
<td>108</td>
<td>Kharoshthi</td>
<td>10A00-10A5F</td>
<td>First assigned in OpenType 1.5 for OS/2 version 4.</td>
</tr>
<tr>
<td>109</td>
<td>Tai Xuan Jing Symbols</td>
<td>1D300-1D35F</td>
<td>First assigned in OpenType 1.5 for OS/2 version 4.</td>
</tr>
<tr>
<td>110</td>
<td>Cuneiform</td>
<td>12000-123FF</td>
<td>First assigned in OpenType 1.5 for OS/2 version 4.</td>
</tr>
<tr>
<td></td>
<td>Cuneiform Numbers and Punctuation</td>
<td>12400-1247F</td>
<td>Added in OpenType 1.5 for OS/2 version 4.</td>
</tr>
<tr>
<td>111</td>
<td>Counting Rod Numerals</td>
<td>1D360-1D37F</td>
<td>First assigned in OpenType 1.5 for OS/2 version 4.</td>
</tr>
<tr>
<td>112</td>
<td>Sundanese</td>
<td>1B80-1BBF</td>
<td>First assigned in OpenType 1.5 for OS/2 version 4.</td>
</tr>
<tr>
<td>113</td>
<td>Lepcha</td>
<td>1C00-1C4F</td>
<td>First assigned in OpenType 1.5 for OS/2 version 4.</td>
</tr>
<tr>
<td>114</td>
<td>Ol Chiki</td>
<td>1C50-1C7F</td>
<td>First assigned in OpenType 1.5 for OS/2 version 4.</td>
</tr>
<tr>
<td>115</td>
<td>Saurashtra</td>
<td>A880-A8DF</td>
<td>First assigned in OpenType 1.5 for OS/2 version 4.</td>
</tr>
<tr>
<td>116</td>
<td>Kayah Li</td>
<td>A900-A92F</td>
<td>First assigned in OpenType 1.5 for OS/2 version 4.</td>
</tr>
<tr>
<td>117</td>
<td>Rejang</td>
<td>A930-A95F</td>
<td>First assigned in OpenType 1.5 for OS/2 version 4.</td>
</tr>
<tr>
<td>118</td>
<td>Cham</td>
<td>AA00-AA5F</td>
<td>First assigned in OpenType 1.5 for OS/2 version 4.</td>
</tr>
<tr>
<td>119</td>
<td>Ancient Symbols</td>
<td>10190-101CF</td>
<td>First assigned in OpenType 1.5 for OS/2 version 4.</td>
</tr>
<tr>
<td>120</td>
<td>Phaistos Disc</td>
<td>101D0-101FF</td>
<td>First assigned in OpenType 1.5 for OS/2 version 4.</td>
</tr>
<tr>
<td>121</td>
<td>Carian</td>
<td>102A0-102DF</td>
<td>First assigned in OpenType 1.5 for OS/2 version 4.</td>
</tr>
<tr>
<td></td>
<td>Lycian</td>
<td>10280-1029F</td>
<td>Added in OpenType 1.5 for OS/2 version 4.</td>
</tr>
<tr>
<td></td>
<td>Lydian</td>
<td>10920-1093F</td>
<td>Added in OpenType 1.5 for OS/2 version 4.</td>
</tr>
<tr>
<td>122</td>
<td>Domino Tiles</td>
<td>1F030-1F09F</td>
<td>First assigned in OpenType 1.5 for OS/2 version 4.</td>
</tr>
<tr>
<td></td>
<td>Mahjong Tiles</td>
<td>1F000-1F02F</td>
<td>First assigned in OpenType 1.5 for OS/2 version 4.</td>
</tr>
<tr>
<td>123-127</td>
<td>Reserved</td>
<td></td>
<td></td>
</tr>
</tbody>
</table>
</td>
</tr>
<tr>
<td>Comments:</td>
<td>
All available bits were exhausted as of Unicode 5.1. The bit assignments were last updated for OS/2 version 4 in OpenType 1.5. There are many additional ranges supported in the current version of Unicode that are not supported by these fields in the OS/2 table. See the 'dlng' and 'slng' tags in the <a data-linktype="relative-path" href="https://learn.microsoft.com/en-us/typography/opentype/spec/meta">'meta' table</a> for an alternate mechanism to declare what scripts or languages a font can support or is designed for.
</td>
</tr>
<tr>
<td>Version differences:</td>
<td>
Different versions of the OS/2 table were created when different Unicode versions were current, and the initial specification for a given version defined fewer bit assignments than for later versions. Some applications might not support all assignments for fonts that have earlier OS/2 versions.
<br/><br/>
All of the bit assignments listed above are valid for any version of the OS/2 table, though OS/2 versions 1 and 2 were specified with some assignments that did not correspond to well-defined Unicode ranges and that conflict with later assignments — see the details below. If a font has a version 1 or version 2 OS/2 table with one of these bits set, the obsolete assignment could be the intended interpretation. Because these assignments do not correspond to well-defined ranges, however, the implied character coverage is unclear.
<br/><br/>
<b>Version 0:</b> When version 0 was first specified, no bit assignments were defined. Some applications could ignore these fields in a version 0 OS/2 table.
<br/><br/>
<b>Version 1:</b>
<br/>
Version 1 was first specified concurrent with Unicode 1.1, and bit assignments were defined for bits 0 to 69 only. With fonts that have a version 1 table, some applications might recognize only bits 0 to 69.
<br/><br/>
Also, version 1 was specified with some bit assignments that did not correspond to a well-defined Unicode range:
<ul>
<li>Bit 8: “Greek Symbols and Coptic” (bit 7 was specified as “Basic Greek”)</li>
<li>Bit 12: “Hebrew Extended” (bit 11 was specified as “Basic Hebrew”)</li>
<li>Bit 14: “Arabic Extended” (bit 13 was specified as “Basic Arabic”)</li>
<li>Bit 27: “Georgian Extended” (bit 26 was specified as “Basic Georgian”)</li>
</ul>
These assignments were discontinued as of version 2.
<br/><br/>
In addition, versions 1 and 2 were defined with bit 53 specified as “CJK Miscellaneous”, which also does not correspond to any well-defined Unicode range. This assignment was discontinued as of version 3.
<br/><br/>
<b>Version 2:</b>
<br/>
Version 2 was defined in OpenType 1.1, which was concurrent with Unicode 2.1. At that time, bit assignments were defined for bits 0 to 69 only. Bit assignments for version 2 were updated in OpenType 1.3, adding assignments for bits 70 to 83 corresponding to new blocks assigned in Unicode 2.0 and Unicode 3.0. With fonts that have a version 2 table, some applications might recognize only those bits assigned in OpenType 1.2 or OpenType 1.3.
<br/><br/>
Also, the specification for version 2 continued to use a problematic assignment for bit 53 — see details for version 1. This assignment was discontinued as of version 3.
<br/><br/>
<b>Version 3:</b> Version 3 was defined in OpenType 1.4 with assignments for bits 84 to 92 corresponding to additional ranges in Unicode 3.2. In addition, some already-assigned bits were extended to cover additional Unicode ranges for related characters; see details in the table above.
<br/><br/>
<b>Version 4:</b> Version 4 was defined in OpenType 1.5 with assignments for bit 58 and bits 93 to 122 corresponding to additional ranges in Unicode 5.1. Also, bits 8, 12, 14, 27 and 53 were re-assigned (see version 1 for previous assignments). In addition, some already-assigned bits were extended to cover additional Unicode ranges for related characters; see details in the table above.
</td>
</tr>
</tbody>
</table>

