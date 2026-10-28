# 4.4 XML Processor Treatment of Entities and References

Source: [Extensible Markup Language (XML) 1.0](https://www.w3.org/TR/2008/REC-xml-20081126/) · 5th edition, 2008-11-26.

[Document index](../README.md) · [Topic guide](../../../../README.md) · [Publisher](https://www.w3.org/TR/2008/REC-xml-20081126/).

HTML normalized to Markdown. Complex tables and mathematical markup retain HTML; illustrations are stored locally.

[Previous](012-4-3-parsed-entities.md) · [Next](014-4-5-construction-of-entity-replacement-text.md)

<a id="entproc"></a>

### 4.4 XML Processor Treatment of Entities and References

The table below summarizes the contexts in which character references,
entity references, and invocations of unparsed entities might appear and the
*REQUIRED* behavior of an [XML processor](002-appendices.md#dt-xml-proc "XML Processor")
in each case. The labels in the leftmost column describe the recognition context:

Reference in Content
:   as a reference anywhere after the [start-tag](008-3-1-start-tags-end-tags-and-empty-element-tags.md#dt-stag "Start-Tag")
    and before the [end-tag](008-3-1-start-tags-end-tags-and-empty-element-tags.md#dt-etag "End Tag") of an element; corresponds
    to the nonterminal [content](008-3-1-start-tags-end-tags-and-empty-element-tags.md#NT-content).

Reference in Attribute Value
:   as a reference within either the value of an attribute in a [start-tag](008-3-1-start-tags-end-tags-and-empty-element-tags.md#dt-stag "Start-Tag"),
    or a default value in an [attribute declaration](009-3-3-attribute-list-declarations.md#dt-attdecl "Attribute-List Declaration");
    corresponds to the nonterminal [AttValue](003-2-documents.md#NT-AttValue).

Occurs as Attribute Value
:   as a [Name](003-2-documents.md#NT-Name), not a reference, appearing either as
    the value of an attribute which has been declared as type **ENTITY**,
    or as one of the space-separated tokens in the value of an attribute which
    has been declared as type **ENTITIES**.

Reference in Entity Value
:   as a reference within a parameter or internal entity's [literal
    entity value](014-4-5-construction-of-entity-replacement-text.md#dt-litentval "Literal Entity Value") in the entity's declaration; corresponds to the nonterminal [EntityValue](003-2-documents.md#NT-EntityValue).

Reference in DTD
:   as a reference within either the internal or external subsets of the [DTD](005-2-8-prolog-and-document-type-declaration.md#dt-doctype "Document Type Declaration"), but outside of an [EntityValue](003-2-documents.md#NT-EntityValue), [AttValue](003-2-documents.md#NT-AttValue), [PI](004-2-4-character-data-and-markup.md#NT-PI), [Comment](004-2-4-character-data-and-markup.md#NT-Comment), [SystemLiteral](003-2-documents.md#NT-SystemLiteral), [PubidLiteral](003-2-documents.md#NT-PubidLiteral),
    or the contents of an ignored conditional section (see [**3.4 Conditional Sections**](010-3-4-conditional-sections.md#sec-condition-sect)).

    .

<table border="1" cellpadding="7" frame="border"><tbody align="center"><tr><td rowspan="2"></td><td align="center" colspan="4" valign="bottom">Entity
Type</td><td align="center" rowspan="2">Character</td></tr><tr align="center" valign="bottom"><td>Parameter</td><td>Internal General</td><td>External Parsed
General</td><td>Unparsed</td></tr><tr align="center" valign="middle"><td align="right">Reference
in Content</td><td>
<a href="013-4-4-xml-processor-treatment-of-entities-and-references.md#not-recognized"><cite>Not recognized</cite></a>
</td><td>
<a href="013-4-4-xml-processor-treatment-of-entities-and-references.md#included"><cite>Included</cite></a>
</td><td>
<a href="013-4-4-xml-processor-treatment-of-entities-and-references.md#include-if-valid"><cite>Included
if validating</cite></a>
</td><td>
<a href="013-4-4-xml-processor-treatment-of-entities-and-references.md#forbidden"><cite>Forbidden</cite></a>
</td><td>
<a href="013-4-4-xml-processor-treatment-of-entities-and-references.md#included"><cite>Included</cite></a>
</td></tr><tr align="center" valign="middle"><td align="right">Reference in Attribute Value</td><td>
<a href="013-4-4-xml-processor-treatment-of-entities-and-references.md#not-recognized"><cite>Not recognized</cite></a>
</td><td>
<a href="013-4-4-xml-processor-treatment-of-entities-and-references.md#inliteral"><cite>Included
in literal</cite></a>
</td><td>
<a href="013-4-4-xml-processor-treatment-of-entities-and-references.md#forbidden"><cite>Forbidden</cite></a>
</td><td>
<a href="013-4-4-xml-processor-treatment-of-entities-and-references.md#forbidden"><cite>Forbidden</cite></a>
</td><td>
<a href="013-4-4-xml-processor-treatment-of-entities-and-references.md#included"><cite>Included</cite></a>
</td></tr><tr align="center" valign="middle"><td align="right">Occurs as Attribute
Value</td><td>
<a href="013-4-4-xml-processor-treatment-of-entities-and-references.md#not-recognized"><cite>Not recognized</cite></a>
</td><td>
<a href="013-4-4-xml-processor-treatment-of-entities-and-references.md#forbidden"><cite>Forbidden</cite></a>
</td><td>
<a href="013-4-4-xml-processor-treatment-of-entities-and-references.md#forbidden"><cite>Forbidden</cite></a>
</td><td>
<a href="013-4-4-xml-processor-treatment-of-entities-and-references.md#notify"><cite>Notify</cite></a>
</td><td>
<a href="013-4-4-xml-processor-treatment-of-entities-and-references.md#not-recognized"><cite>Not recognized</cite></a>
</td></tr><tr align="center" valign="middle"><td align="right">Reference in EntityValue</td><td>
<a href="013-4-4-xml-processor-treatment-of-entities-and-references.md#inliteral"><cite>Included in literal</cite></a>
</td><td>
<a href="013-4-4-xml-processor-treatment-of-entities-and-references.md#bypass"><cite>Bypassed</cite></a>
</td><td>
<a href="013-4-4-xml-processor-treatment-of-entities-and-references.md#bypass"><cite>Bypassed</cite></a>
</td><td>
<a href="013-4-4-xml-processor-treatment-of-entities-and-references.md#error"><cite>Error</cite></a>
</td><td>
<a href="013-4-4-xml-processor-treatment-of-entities-and-references.md#included"><cite>Included</cite></a>
</td></tr><tr align="center" valign="middle"><td align="right">Reference in DTD</td><td>
<a href="013-4-4-xml-processor-treatment-of-entities-and-references.md#as-PE"><cite>Included as PE</cite></a>
</td><td>
<a href="013-4-4-xml-processor-treatment-of-entities-and-references.md#forbidden"><cite>Forbidden</cite></a>
</td><td>
<a href="013-4-4-xml-processor-treatment-of-entities-and-references.md#forbidden"><cite>Forbidden</cite></a>
</td><td>
<a href="013-4-4-xml-processor-treatment-of-entities-and-references.md#forbidden"><cite>Forbidden</cite></a>
</td><td>
<a href="013-4-4-xml-processor-treatment-of-entities-and-references.md#forbidden"><cite>Forbidden</cite></a>
</td></tr></tbody></table>

<a id="not-recognized"></a>

#### 4.4.1 Not Recognized

Outside the DTD, the `%` character has no special significance;
thus, what would be parameter entity references in the DTD are not recognized
as markup in [content](008-3-1-start-tags-end-tags-and-empty-element-tags.md#NT-content). Similarly, the names of unparsed
entities are not recognized except when they appear in the value of an appropriately
declared attribute.

<a id="included"></a>

#### 4.4.2 Included

<a id="dt-include"></a>

[Definition: An entity is **included**
when its [replacement text](014-4-5-construction-of-entity-replacement-text.md#dt-repltext "Replacement Text") is retrieved
and processed, in place of the reference itself, as though it were part of
the document at the location the reference was recognized.] The replacement
text may contain both [character data](004-2-4-character-data-and-markup.md#dt-chardata "Character Data")
and (except for parameter entities) [markup](004-2-4-character-data-and-markup.md#dt-markup "Markup"),
which *MUST* be recognized in the usual way. (The string "
`AT&amp;T;`
"
expands to "
`AT&T;`
" and the remaining ampersand
is not recognized as an entity-reference delimiter.) A character reference
is **included** when the indicated character is processed in place
of the reference itself.

<a id="include-if-valid"></a>

#### 4.4.3 Included If Validating

When an XML processor recognizes a reference to a parsed entity, in order
to [validate](005-2-8-prolog-and-document-type-declaration.md#dt-valid "Validity") the document, the processor
*MUST*
[include](013-4-4-xml-processor-treatment-of-entities-and-references.md#dt-include "Include") its replacement text. If
the entity is external, and the processor is not attempting to validate the
XML document, the processor *MAY*, but need
not, include the entity's replacement text. If a non-validating processor
does not include the replacement text, it *MUST* inform the application that
it recognized, but did not read, the entity.

This rule is based on the recognition that the automatic inclusion provided
by the SGML and XML entity mechanism, primarily designed to support modularity
in authoring, is not necessarily appropriate for other applications, in particular
document browsing. Browsers, for example, when encountering an external parsed
entity reference, might choose to provide a visual indication of the entity's
presence and retrieve it for display only on demand.

<a id="forbidden"></a>

#### 4.4.4 Forbidden

The following are forbidden, and constitute [fatal
errors](002-appendices.md#dt-fatal "Fatal Error"):

- the appearance of a reference to an [unparsed
  entity](010-3-4-conditional-sections.md#dt-unparsed "Unparsed Entity"), except in the
  [EntityValue](003-2-documents.md#NT-EntityValue) in an entity declaration.
- the appearance of any character or general-entity reference in the
  DTD except within an [EntityValue](003-2-documents.md#NT-EntityValue) or [AttValue](003-2-documents.md#NT-AttValue).
- a reference to an external entity in an attribute value.

<a id="inliteral"></a>

#### 4.4.5 Included in Literal

When an [entity reference](011-4-1-character-and-entity-references.md#dt-entref "Entity Reference") appears in
an attribute value, or a parameter entity reference appears in a literal entity
value, its [replacement text](014-4-5-construction-of-entity-replacement-text.md#dt-repltext "Replacement Text")
*MUST* be processed
in place of the reference itself as though it were part of the document at
the location the reference was recognized, except that a single or double
quote character in the replacement text *MUST* always be treated as a normal data
character and *MUST NOT* terminate the literal. For example, this is well-formed:



```
<!ENTITY % YN '"Yes"' >
<!ENTITY WhatHeSaid "He said %YN;" >
```




while this is not:



```
<!ENTITY EndAttr "27'" >
<element attribute='a-&EndAttr;>
```




<a id="notify"></a>

#### 4.4.6 Notify

When the name of an [unparsed entity](010-3-4-conditional-sections.md#dt-unparsed "Unparsed Entity")
appears as a token in the value of an attribute of declared type **ENTITY**
or **ENTITIES**, a validating processor *MUST* inform the application of
the [system](011-4-1-character-and-entity-references.md#dt-sysid "System Identifier") and [public](011-4-1-character-and-entity-references.md#dt-pubid "Public identifier")
(if any) identifiers for both the entity and its associated [notation](014-4-5-construction-of-entity-replacement-text.md#dt-notation "Notation").

<a id="bypass"></a>

#### 4.4.7 Bypassed

When a general entity reference appears in the [EntityValue](003-2-documents.md#NT-EntityValue)
in an entity declaration, it *MUST* be bypassed and left as is.

<a id="as-PE"></a>

#### 4.4.8 Included as PE

Just as with external parsed entities, parameter entities need only be [included if validating](013-4-4-xml-processor-treatment-of-entities-and-references.md#include-if-valid). When a parameter-entity
reference is recognized in the DTD and included, its [replacement
text](014-4-5-construction-of-entity-replacement-text.md#dt-repltext "Replacement Text")
*MUST* be enlarged by the attachment of one leading and one following
space (#x20) character; the intent is to constrain the replacement text of
parameter entities to contain an integral number of grammatical tokens in
the DTD. This
behavior *MUST NOT* apply to parameter entity references within entity values;
these are described in [**4.4.5 Included in Literal**](013-4-4-xml-processor-treatment-of-entities-and-references.md#inliteral).

<a id="error"></a>

#### 4.4.9 Error

It is an [error](002-appendices.md#dt-error "Error") for a reference to
an unparsed entity to appear in the [EntityValue](003-2-documents.md#NT-EntityValue) in an
entity declaration.

