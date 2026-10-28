# 2.12 Language Identification

Source: [Extensible Markup Language (XML) 1.0](https://www.w3.org/TR/2008/REC-xml-20081126/) · 5th edition, 2008-11-26.

[Document index](../README.md) · [Topic guide](../../../../README.md) · [Publisher](https://www.w3.org/TR/2008/REC-xml-20081126/).

HTML normalized to Markdown. Complex tables and mathematical markup retain HTML; illustrations are stored locally.

[Previous](006-2-9-standalone-document-declaration.md) · [Next](008-3-1-start-tags-end-tags-and-empty-element-tags.md)

<a id="sec-lang-tag"></a>

### 2.12 Language Identification

In document processing, it is often useful to identify the natural or formal
language in which the content is written. A special [attribute](008-3-1-start-tags-end-tags-and-empty-element-tags.md#dt-attr "Attribute")
named `xml:lang` may be inserted in documents to specify the language
used in the contents and attribute values of any element in an XML document.
In valid documents, this attribute, like any other, *MUST* be [declared](009-3-3-attribute-list-declarations.md#dt-attdecl "Attribute-List Declaration")
if it is used. The
values of the attribute are language identifiers as defined by [[IETF BCP 47]](015-5-2-using-xml-processors.md#RFC1766), Tags
for the Identification of Languages; in addition, the empty string may be specified.

(Productions 33 through 38 have been removed.)

For example:



```
<p xml:lang="en">The quick brown fox jumps over the lazy dog.</p>
<p xml:lang="en-GB">What colour is it?</p>
<p xml:lang="en-US">What color is it?</p>
<sp who="Faust" desc='leise' xml:lang="de">
  <l>Habe nun, ach! Philosophie,</l>
  <l>Juristerei, und Medizin</l>
  <l>und leider auch Theologie</l>
  <l>durchaus studiert mit heißem Bemüh'n.</l>
</sp>
```




The language specified by `xml:lang` applies to the element where it is specified
(including the values of its attributes), and to all elements in its content unless
overridden with another instance of `xml:lang`. In particular, the empty value of `xml:lang` is used on an element B to override
a specification of `xml:lang` on an enclosing element A, without specifying another language. Within B,
it is considered that there is no language information available, just as if `xml:lang` had not been specified
on B or any of its ancestors. Applications determine which of an element's attribute values
and which parts of its character content, if any, are treated as language-dependent values described by `xml:lang`.

**Note:**

Language information may also be provided by external transport protocols (e.g. HTTP or
MIME). When available, this information may be used by XML applications, but the more local
information provided by `xml:lang` should be considered to override it.

A simple declaration for `xml:lang` might take the form



```
xml:lang CDATA #IMPLIED
```




but specific default values may also be given, if appropriate. In a collection
of French poems for English students, with glosses and notes in English, the `xml:lang`
attribute might be declared this way:



```
<!ATTLIST poem   xml:lang CDATA 'fr'>
<!ATTLIST gloss  xml:lang CDATA 'en'>
<!ATTLIST note   xml:lang CDATA 'en'>
```




<a id="sec-logical-struct"></a>

## 3 Logical Structures

<a id="dt-element"></a>

[Definition: Each [XML
document](003-2-documents.md#dt-xml-doc "XML Document") contains one or more **elements**, the boundaries
of which are either delimited by [start-tags](008-3-1-start-tags-end-tags-and-empty-element-tags.md#dt-stag "Start-Tag")
and [end-tags](008-3-1-start-tags-end-tags-and-empty-element-tags.md#dt-etag "End Tag"), or, for [empty](008-3-1-start-tags-end-tags-and-empty-element-tags.md#dt-empty "Empty")
elements, by an [empty-element tag](008-3-1-start-tags-end-tags-and-empty-element-tags.md#dt-eetag "empty-element tag"). Each
element has a type, identified by name, sometimes called its "generic
identifier" (GI), and may have a set of attribute specifications.]
Each attribute specification has a [name](008-3-1-start-tags-end-tags-and-empty-element-tags.md#dt-attrname "Attribute Name")
and a [value](008-3-1-start-tags-end-tags-and-empty-element-tags.md#dt-attrval "Attribute Value").

<a id="d0e2130"></a>

##### Element

<a id="NT-element"></a>

|  |  |  |  |
| --- | --- | --- | --- |
| [39] | `element` | ::= | `EmptyElemTag` |
|  |  |  | `| STag content ETag` | [[WFC: Element Type Match]](007-2-12-language-identification.md#GIMatch) |
|  |  |  |  | [[VC: Element Valid]](007-2-12-language-identification.md#elementvalid) |

This specification does not constrain the
application semantics, use, or (beyond syntax)
names of the element types and attributes, except that names beginning with
a match to `(('X'|'x')('M'|'m')('L'|'l'))` are reserved for standardization
in this or future versions of this specification.

<a id="GIMatch"></a>

**Well-formedness constraint: Element Type Match**

The [Name](003-2-documents.md#NT-Name)
in an element's end-tag *MUST* match the element type in the start-tag.

<a id="elementvalid"></a>

**Validity constraint: Element Valid**

An element is valid
if there is a declaration matching [elementdecl](008-3-1-start-tags-end-tags-and-empty-element-tags.md#NT-elementdecl)
where the [Name](003-2-documents.md#NT-Name) matches the element type, and one of
the following holds:

1. The declaration matches **EMPTY** and the element has no [content](008-3-1-start-tags-end-tags-and-empty-element-tags.md#dt-content "Content") (not even entity
   references, comments, PIs or white space).
2. The declaration matches [children](008-3-1-start-tags-end-tags-and-empty-element-tags.md#NT-children) and the
   sequence of [child elements](003-2-documents.md#dt-parentchild "Parent/Child") belongs
   to the language generated by the regular expression in the content model,
   with optional white space, comments and
   PIs (i.e. markup matching production [27] [Misc](005-2-8-prolog-and-document-type-declaration.md#NT-Misc)) between the
   start-tag and the first child element, between child elements, or between
   the last child element and the end-tag. Note that a CDATA section containing
   only white space or a reference
   to an entity whose replacement text is character references expanding to white
   space do not
   match the nonterminal [S](003-2-documents.md#NT-S), and
   hence cannot appear in these positions; however, a
   reference to an internal entity with a literal value consisting of character
   references expanding to white space does match [S](003-2-documents.md#NT-S), since its
   replacement text is the white space resulting from expansion of the character
   references.
3. The declaration matches [Mixed](008-3-1-start-tags-end-tags-and-empty-element-tags.md#NT-Mixed), and the content
   (after replacing
   any entity references with their replacement text) consists of
   [character data](004-2-4-character-data-and-markup.md#dt-chardata "Character Data")
   (including [CDATA sections](004-2-4-character-data-and-markup.md#dt-cdsection "CDATA Section")),
   [comments](004-2-4-character-data-and-markup.md#dt-comment "Comment"), [PIs](004-2-4-character-data-and-markup.md#dt-pi "Processing instruction") and [child elements](003-2-documents.md#dt-parentchild "Parent/Child") whose types match names in the
   content model.
4. The declaration matches **ANY**, and the content (after replacing
   any entity references with their replacement text)
   consists of character data, [CDATA
   sections](004-2-4-character-data-and-markup.md#dt-cdsection "CDATA Section"), [comments](004-2-4-character-data-and-markup.md#dt-comment "Comment"), [PIs](004-2-4-character-data-and-markup.md#dt-pi "Processing instruction")
   and [child elements](003-2-documents.md#dt-parentchild "Parent/Child")
   whose types have been declared.

