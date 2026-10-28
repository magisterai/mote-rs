# Internationalized Resource Identifiers (IRIs) — part 4

Source: [Internationalized Resource Identifiers (IRIs)](https://www.rfc-editor.org/rfc/rfc3987.txt) · RFC 3987.

[Document index](../README.md) · [Topic guide](../../../../README.md) · [Publisher](https://www.rfc-editor.org/rfc/rfc3987.txt).

Verbatim text slices in fixed-width blocks preserve grammar, examples, and diagrams.

[Previous](part-003.md) · [Next](part-005.md)

<a id="source-page-17-part-1"></a>

## Source page 17

```text

RFC 3987         Internationalized Resource Identifiers     January 2005


   relation between logical representation (used for digital
   representation and for reading/spelling) and visual representation
   (used for display/printing).

   Because of the complex interaction between the logical
   representation, the visual representation, and the syntax of a Bidi
   IRI, a balance is needed between various requirements.  The main
   requirements are

   1.  user-predictable conversion between visual and logical
       representation;

   2.  the ability to include a wide range of characters in various
       parts of the IRI; and

   3.  minor or no changes or restrictions for implementations.

4.1.  Logical Storage and Visual Presentation

   When stored or transmitted in digital representation, bidirectional
   IRIs MUST be in full logical order and MUST conform to the IRI syntax
   rules (which includes the rules relevant to their scheme). This
   ensures that bidirectional IRIs can be processed in the same way as
   other IRIs.

   Bidirectional IRIs MUST be rendered by using the Unicode
   Bidirectional Algorithm [UNIV4], [UNI9].  Bidirectional IRIs MUST be
   rendered in the same way as they would be if they were in a
   left-to-right embedding; i.e., as if they were preceded by U+202A,
   LEFT-TO-RIGHT EMBEDDING (LRE), and followed by U+202C, POP
   DIRECTIONAL FORMATTING (PDF).  Setting the embedding direction can
   also be done in a higher-level protocol (e.g., the dir='ltr'
   attribute in HTML).

   There is no requirement to use the above embedding if the display is
   still the same without the embedding.  For example, a bidirectional
   IRI in a text with left-to-right base directionality (such as used
   for English or Cyrillic) that is preceded and followed by whitespace
   and  strong left-to-right characters does not need an embedding.
   Also, a bidirectional relative IRI reference that only contains
   strong right-to-left characters and weak characters and that starts
   and ends with a strong right-to-left character and appears in a text
   with right-to-left base directionality (such as used for Arabic or
   Hebrew) and is preceded and followed by whitespace and strong
   characters does not need an embedding.






Duerst & Suignard           Standards Track                    [Page 17]
```

<a id="source-page-18-part-1"></a>

## Source page 18

```text

RFC 3987         Internationalized Resource Identifiers     January 2005


   In some other cases, using U+200E, LEFT-TO-RIGHT MARK (LRM), may be
   sufficient to force the correct display behavior.  However, the
   details of the Unicode Bidirectional algorithm are not always easy to
   understand.  Implementers are strongly advised to err on the side of
   caution and to use embedding in all cases where they are not
   completely sure that the display behavior is unaffected without the
   embedding.

   The Unicode Bidirectional Algorithm ([UNI9], section 4.3) permits
   higher-level protocols to influence bidirectional rendering.  Such
   changes by higher-level protocols MUST NOT be used if they change the
   rendering of IRIs.

   The bidirectional formatting characters that may be used before or
   after the IRI to ensure correct display are not themselves part of
   the IRI.  IRIs MUST NOT contain bidirectional formatting characters
   (LRM, RLM, LRE, RLE, LRO, RLO, and PDF).  They affect the visual
   rendering of the IRI but do not appear themselves.  It would
   therefore not be possible to input an IRI with such characters
   correctly.

4.2.  Bidi IRI Structure

   The Unicode Bidirectional Algorithm is designed mainly for running
   text.  To make sure that it does not affect the rendering of
   bidirectional IRIs too much, some restrictions on bidirectional IRIs
   are necessary.  These restrictions are given in terms of delimiters
   (structural characters, mostly punctuation such as "@", ".", ":", and
   "/") and components (usually consisting mostly of letters and
   digits).

   The following syntax rules from section 2.2 correspond to components
   for the purpose of Bidi behavior: iuserinfo, ireg-name, isegment,
   isegment-nz, isegment-nz-nc, ireg-name, iquery, and ifragment.

   Specifications that define the syntax of any of the above components
   MAY divide them further and define smaller parts to be components
   according to this document.  As an example, the restrictions of
   [RFC3490] on bidirectional domain names correspond to treating each
   label of a domain name as a component for schemes with ireg-name as a
   domain name.  Even where the components are not defined formally, it
   may be helpful to think about some syntax in terms of components and
   to apply the relevant restrictions.  For example, for the usual
   name/value syntax in query parts, it is convenient to treat each name
   and each value as a component.  As another example, the extensions in
   a resource name can be treated as separate components.





Duerst & Suignard           Standards Track                    [Page 18]
```

<a id="source-page-19-part-1"></a>

## Source page 19

```text

RFC 3987         Internationalized Resource Identifiers     January 2005


   For each component, the following restrictions apply:

   1.  A component SHOULD NOT use both right-to-left and left-to-right
       characters.

   2.  A component using right-to-left characters SHOULD start and end
       with right-to-left characters.

   The above restrictions are given as shoulds, rather than as musts.
   For IRIs that are never presented visually, they are not relevant.
   However, for IRIs in general, they are very important to ensure
   consistent conversion between visual presentation and logical
   representation, in both directions.

   Note: In some components, the above restrictions may actually be
      strictly enforced.  For example, [RFC3490] requires that these
      restrictions apply to the labels of a host name for those schemes
      where ireg-name is a host name.  In some other components (for
      example, path components) following these restrictions may not be
      too difficult.  For other components, such as parts of the query
      part, it may be very difficult to enforce the restrictions because
      the values of query parameters may be arbitrary character
      sequences.

   If the above restrictions cannot be satisfied otherwise, the affected
   component can always be mapped to URI notation as described in
   section 3.1.  Please note that the whole component has to be mapped
   (see also Example 9 below).

4.3.  Input of Bidi IRIs

   Bidi input methods MUST generate Bidi IRIs in logical order while
   rendering them according to section 4.1.  During input, rendering
   SHOULD be updated after every new character is input to avoid end-
   user confusion.

4.4.  Examples

   This section gives examples of bidirectional IRIs, in Bidi Notation.
   It shows legal IRIs with the relationship between logical and visual
   representation and explains how certain phenomena in this
   relationship may look strange to somebody not familiar with
   bidirectional behavior, but familiar to users of Arabic and Hebrew.
   It also shows what happens if the restrictions given in section 4.2
   are not followed.  The examples below can be seen at [BidiEx], in
   Arabic, Hebrew, and Bidi Notation variants.





Duerst & Suignard           Standards Track                    [Page 19]
```

<a id="source-page-20-part-1"></a>

## Source page 20

```text

RFC 3987         Internationalized Resource Identifiers     January 2005


   To read the bidi text in the examples, read the visual representation
   from left to right until you encounter a block of rtl text.  Read the
   rtl block (including slashes and other special characters) from right
   to left, then continue at the next unread ltr character.

   Example 1: A single component with rtl characters is inverted:
   Logical representation: "http://ab.CDEFGH.ij/kl/mn/op.html"
   Visual representation: "http://ab.HGFEDC.ij/kl/mn/op.html"
   Components can be read one by one, and each component can be read in
   its natural direction.

   Example 2: More than one consecutive component with rtl characters is
   inverted as a whole:
   Logical representation: "http://ab.CDE.FGH/ij/kl/mn/op.html"
   Visual representation: "http://ab.HGF.EDC/ij/kl/mn/op.html"
   A sequence of rtl components is read rtl, in the same way as a
   sequence of rtl words is read rtl in a bidi text.

   Example 3: All components of an IRI (except for the scheme) are rtl.
   All rtl components are inverted overall:
   Logical representation: "http://AB.CD.EF/GH/IJ/KL?MN=OP;QR=ST#UV"
   Visual representation: "http://VU#TS=RQ;PO=NM?LK/JI/HG/FE.DC.BA"
   The whole IRI (except the scheme) is read rtl.  Delimiters between
   rtl components stay between the respective components; delimiters
   between ltr and rtl components don't move.

   Example 4: Each of several sequences of rtl components is inverted on
   its own:
   Logical representation: "http://AB.CD.ef/gh/IJ/KL.html"
   Visual representation: "http://DC.BA.ef/gh/LK/JI.html"
   Each sequence of rtl components is read rtl, in the same way as each
   sequence of rtl words in an ltr text is read rtl.

   Example 5: Example 2, applied to components of different kinds:
   Logical representation: "http://ab.cd.EF/GH/ij/kl.html"
   Visual representation: "http://ab.cd.HG/FE/ij/kl.html"
   The inversion of the domain name label and the path component may be
   unexpected, but it is consistent with other bidi behavior.  For
   reassurance that the domain component really is "ab.cd.EF", it may be
   helpful to read aloud the visual representation following the bidi
   algorithm.  After "http://ab.cd." one reads the RTL block
   "E-F-slash-G-H", which corresponds to the logical representation.

   Example 6: Same as Example 5, with more rtl components:
   Logical representation: "http://ab.CD.EF/GH/IJ/kl.html"
   Visual representation: "http://ab.JI/HG/FE.DC/kl.html"
   The inversion of the domain name labels and the path components may
   be easier to identify because the delimiters also move.



Duerst & Suignard           Standards Track                    [Page 20]
```

<a id="source-page-21-part-1"></a>

## Source page 21

```text

RFC 3987         Internationalized Resource Identifiers     January 2005


   Example 7: A single rtl component includes digits:
   Logical representation: "http://ab.CDE123FGH.ij/kl/mn/op.html"
   Visual representation: "http://ab.HGF123EDC.ij/kl/mn/op.html"
   Numbers are written ltr in all cases but are treated as an additional
   embedding inside a run of rtl characters.  This is completely
   consistent with usual bidirectional text.

   Example 8 (not allowed): Numbers are at the start or end of an rtl
   component:
   Logical representation: "http://ab.cd.ef/GH1/2IJ/KL.html"
   Visual representation: "http://ab.cd.ef/LK/JI1/2HG.html"
   The sequence "1/2" is interpreted by the bidi algorithm as a
   fraction, fragmenting the components and leading to confusion.  There
   are other characters that are interpreted in a special way close to
   numbers; in particular, "+", "-", "#", "$", "%", ",", ".", and ":".

   Example 9 (not allowed): The numbers in the previous example are
   percent-encoded:
   Logical representation: "http://ab.cd.ef/GH%31/%32IJ/KL.html",
   Visual representation (Hebrew): "http://ab.cd.ef/%31HG/LK/JI%32.html"
   Visual representation (Arabic): "http://ab.cd.ef/31%HG/%LK/JI32.html"
   Depending on whether the uppercase letters represent Arabic or
   Hebrew, the visual representation is different.

   Example 10 (allowed but not recommended):
   Logical representation: "http://ab.CDEFGH.123/kl/mn/op.html"
   Visual representation: "http://ab.123.HGFEDC/kl/mn/op.html"
   Components consisting of only numbers are allowed (it would be rather
   difficult to prohibit them), but these may interact with adjacent RTL
   components in ways that are not easy to predict.

5.  Normalization and Comparison

      Note: The structure and much of the material for this section is
      taken from section 6 of [RFC3986]; the differences are due to the
      specifics of IRIs.

   One of the most common operations on IRIs is simple comparison:
   Determining whether two IRIs are equivalent without using the IRIs or
   the mapped URIs to access their respective resource(s).  A comparison
   is performed whenever a response cache is accessed, a browser checks
   its history to color a link, or an XML parser processes tags within a
   namespace.  Extensive normalization prior to comparison of IRIs may
   be used by spiders and indexing engines to prune a search space or
   reduce duplication of request actions and response storage.






Duerst & Suignard           Standards Track                    [Page 21]
```

