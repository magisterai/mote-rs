# 3.4 Conditional Sections

Source: [Extensible Markup Language (XML) 1.0](https://www.w3.org/TR/2008/REC-xml-20081126/) · 5th edition, 2008-11-26.

[Document index](../README.md) · [Topic guide](../../../../README.md) · [Publisher](https://www.w3.org/TR/2008/REC-xml-20081126/).

HTML normalized to Markdown. Complex tables and mathematical markup retain HTML; illustrations are stored locally.

[Previous](009-3-3-attribute-list-declarations.md) · [Next](011-4-1-character-and-entity-references.md)

1. All line breaks *MUST* have been normalized on input to #xA as described
   in [**2.11 End-of-Line Handling**](006-2-9-standalone-document-declaration.md#sec-line-ends), so the rest of this algorithm operates
   on text normalized in this way.
2. Begin with a normalized value consisting of the empty string.
3. For each character, entity reference, or character reference in the
   unnormalized attribute value, beginning with the first and continuing to the
   last, do the following:

   - For a character reference, append the referenced character to the
     normalized value.
   - For an entity reference, recursively apply step 3 of this algorithm
     to the replacement text of the entity.
   - For a white space character (#x20, #xD, #xA, #x9), append a space
     character (#x20) to the normalized value.
   - For another character, append the character to the normalized value.

If the attribute type is not CDATA, then the XML processor *MUST* further
process the normalized attribute value by discarding any leading and trailing
space (#x20) characters, and by replacing sequences of space (#x20) characters
by a single space (#x20) character.

Note that if the unnormalized attribute value contains a character reference
to a white space character other than space (#x20), the normalized value contains
the referenced character itself (#xD, #xA or #x9). This contrasts with the
case where the unnormalized value contains a white space character (not a
reference), which is replaced with a space character (#x20) in the normalized
value and also contrasts with the case where the unnormalized value contains
an entity reference whose replacement text contains a white space character;
being recursively processed, the white space character is replaced with a
space character (#x20) in the normalized value.

All attributes for which no declaration has been read *SHOULD* be treated
by a non-validating processor as if declared **CDATA**.

It is an error if an [attribute
value](008-3-1-start-tags-end-tags-and-empty-element-tags.md#dt-attrval "Attribute Value") contains a [reference](011-4-1-character-and-entity-references.md#dt-entref "Entity Reference") to an
entity for which no declaration has been read.

Following are examples of attribute normalization. Given the following
declarations:



```
<!ENTITY d "&#xD;">
<!ENTITY a "&#xA;">
<!ENTITY da "&#xD;&#xA;">
```




the attribute specifications in the left column below would be normalized
to the character sequences of the middle column if the attribute `a`
is declared **NMTOKENS** and to those of the right columns if `a`
is declared **CDATA**.

<table border="1" frame="border"><thead><tr><th>Attribute specification</th><th>a is NMTOKENS</th><th>a is CDATA</th></tr></thead><tbody><tr><td>
<div class="exampleInner"><pre>a="

xyz"</pre></div>
</td><td>
<div class="exampleInner"><pre>x y z</pre></div>
</td><td>
<div class="exampleInner"><pre>#x20 #x20 x y z</pre></div>
</td></tr><tr><td>
<div class="exampleInner"><pre>a="&amp;d;&amp;d;A&amp;a;&amp;#x20;&amp;a;B&amp;da;"</pre></div>
</td><td>
<div class="exampleInner"><pre>A #x20 B</pre></div>
</td><td>
<div class="exampleInner"><pre>#x20 #x20 A #x20 #x20 #x20 B #x20 #x20</pre></div>
</td></tr><tr><td>
<div class="exampleInner"><pre>a=
"&amp;#xd;&amp;#xd;A&amp;#xa;&amp;#xa;B&amp;#xd;&amp;#xa;"</pre></div>
</td><td>
<div class="exampleInner"><pre>#xD #xD A #xA #xA B #xD #xA</pre></div>
</td><td>
<div class="exampleInner"><pre>#xD #xD A #xA #xA B #xD #xA</pre></div>
</td></tr></tbody></table>

Note that the last example is invalid (but well-formed) if `a`
is declared to be of type **NMTOKENS**.

<a id="sec-condition-sect"></a>

### 3.4 Conditional Sections

<a id="dt-cond-section"></a>

[Definition:
**Conditional
sections** are portions of the [document type
declaration external subset](005-2-8-prolog-and-document-type-declaration.md#dt-doctype "Document Type Declaration") or
of external parameter entities which are included in, or excluded from,
the logical structure of the DTD based on the keyword which governs them.]

<a id="d0e3658"></a>

##### Conditional Section

<a id="NT-conditionalSect"></a>

<a id="NT-includeSect"></a>

<a id="NT-ignoreSect"></a>

<a id="NT-ignoreSectContents"></a>

<a id="NT-Ignore"></a>

|  |  |  |  |
| --- | --- | --- | --- |
| [61] | `conditionalSect` | ::= | `includeSect | ignoreSect` |
| [62] | `includeSect` | ::= | `'<![' S? 'INCLUDE' S? '[' extSubsetDecl ']]>'` | [[VC: Proper Conditional Section/PE Nesting]](010-3-4-conditional-sections.md#condsec-nesting) |
| [63] | `ignoreSect` | ::= | `'<![' S? 'IGNORE' S? '[' ignoreSectContents* ']]>'` | [[VC: Proper Conditional Section/PE Nesting]](010-3-4-conditional-sections.md#condsec-nesting) |
| [64] | `ignoreSectContents` | ::= | `Ignore ('<![' ignoreSectContents ']]>' Ignore)*` |
| [65] | `Ignore` | ::= | `Char* - (Char* ('<![' | ']]>') Char*)` |

<a id="condsec-nesting"></a>

**Validity constraint: Proper Conditional Section/PE Nesting**

If any of the "`<![`",
"`[`", or "`]]>`" of a conditional section is contained
in the replacement text for a parameter-entity reference, all of them *MUST*
be contained in the same replacement text.

Like the internal and external DTD subsets, a conditional section may contain
one or more complete declarations, comments, processing instructions, or nested
conditional sections, intermingled with white space.

If the keyword of the conditional section is **INCLUDE**, then the
contents of the conditional section *MUST* be processed as part of the DTD. If the keyword of
the conditional section is **IGNORE**, then the contents of the conditional
section *MUST* *NOT* be processed as part of the DTD.
If a conditional section with a keyword of **INCLUDE** occurs within
a larger conditional section with a keyword of **IGNORE**, both the outer
and the inner conditional sections *MUST* be ignored. The contents
of an ignored conditional section *MUST* be parsed by ignoring all characters after
the "`[`" following the keyword, except conditional section starts
"`<![`" and ends "`]]>`", until the matching conditional
section end is found. Parameter entity references *MUST NOT* be recognized in this
process.

If the keyword of the conditional section is a parameter-entity reference,
the parameter entity *MUST* be replaced by its content before the processor
decides whether to include or ignore the conditional section.

An example:



```
<!ENTITY % draft 'INCLUDE' >
<!ENTITY % final 'IGNORE' >

<![%draft;[
<!ELEMENT book (comments*, title, body, supplements?)>
]]>
<![%final;[
<!ELEMENT book (title, body, supplements?)>
]]>
```




<a id="sec-physical-struct"></a>

## 4 Physical Structures

<a id="dt-entity"></a>

[Definition: An XML document may consist of one
or many storage units. These
are called **entities**; they all have **content** and are
all (except for the [document entity](014-4-5-construction-of-entity-replacement-text.md#dt-docent "Document Entity") and
the [external DTD subset](005-2-8-prolog-and-document-type-declaration.md#dt-doctype "Document Type Declaration")) identified by
entity **name**.] Each XML document has one entity
called the [document entity](014-4-5-construction-of-entity-replacement-text.md#dt-docent "Document Entity"), which serves
as the starting point for the [XML processor](002-appendices.md#dt-xml-proc "XML Processor")
and may contain the whole document.

<a id="dt-parsedent"></a>

Entities may be either parsed or unparsed. [Definition: The contents of a **parsed
entity** are referred to as its [replacement
text](014-4-5-construction-of-entity-replacement-text.md#dt-repltext "Replacement Text"); this [text](003-2-documents.md#dt-text "Text") is considered an
integral part of the document.]

<a id="dt-unparsed"></a>

[Definition: An **unparsed entity**
is a resource whose contents may or may not be [text](003-2-documents.md#dt-text "Text"),
and if text, may
be other than XML. Each unparsed entity has an associated [notation](014-4-5-construction-of-entity-replacement-text.md#dt-notation "Notation"), identified by name. Beyond a requirement
that an XML processor make the identifiers for the entity and notation available
to the application, XML places no constraints on the contents of unparsed
entities.]

Parsed entities are invoked by name using entity references; unparsed entities
by name, given in the value of **ENTITY** or **ENTITIES** attributes.

<a id="gen-entity"></a>

<a id="dt-PE"></a>

[Definition:
**General entities**
are entities for use within the document content. In this specification, general
entities are sometimes referred to with the unqualified term *entity*
when this leads to no ambiguity.]
[Definition:
**Parameter
entities** are parsed entities for use within the DTD.]
These two types of entities use different forms of reference and are recognized
in different contexts. Furthermore, they occupy different namespaces; a parameter
entity and a general entity with the same name are two distinct entities.

