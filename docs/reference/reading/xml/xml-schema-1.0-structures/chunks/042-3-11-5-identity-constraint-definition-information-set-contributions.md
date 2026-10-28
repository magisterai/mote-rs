# 3.11.5 Identity-constraint Definition Information Set Contributions

Source: [XML Schema Part 1: Structures](https://www.w3.org/TR/2004/REC-xmlschema-1-20041028/) · 2nd edition, 2004-10-28.

[Document index](../README.md) · [Topic guide](../../../../README.md) · [Publisher](https://www.w3.org/TR/2004/REC-xmlschema-1-20041028/).

HTML normalized to Markdown. Complex tables and mathematical markup retain HTML; illustrations are stored locally.

[Previous](041-3-11-3-constraints-on-xml-representations-of-identity-constraint-definitions.md) · [Next](043-3-12-notation-declarations.md)

**Note:**Because the validation of *keyref* (see clause [4.3](041-3-11-3-constraints-on-xml-representations-of-identity-constraint-definitions.md#cl-krv)) depends on finding
appropriate entries in a element information item's [·node
table·](042-3-11-5-identity-constraint-definition-information-set-contributions.md#key-nt), and [·node tables·](042-3-11-5-identity-constraint-definition-information-set-contributions.md#key-nt) are assembled
strictly recursively from the node tables of descendants, only element
information items within the sub-tree rooted at the element information item
being [·validated·](004-2-1-overview-of-xml-schema.md#key-vn) can be referenced successfully.

**Note:**Although this specification defines a [·post-schema-validation infoset·](004-2-1-overview-of-xml-schema.md#key-psvi)
contribution which would enable schema-aware processors to implement clause [4.2.3](041-3-11-3-constraints-on-xml-representations-of-identity-constraint-definitions.md#c-nlbl) above ([Element Declaration (§3.3.5)](018-3-3-6-constraints-on-element-declaration-schema-components.md#sic-elt-decl)), processors are not required to
provide it. This clause can be read as if in the absence of this infoset contribution, the
value of the relevant [{nillable}](013-3-3-element-declarations.md#nillable) property must be available.

<a id="d0e14060"></a>

#### 3.11.5 Identity-constraint Definition Information Set Contributions

<a id="sic-key"></a>

**Schema Information Set Contribution: Identity-constraint Table**  

<a id="key-ec"></a>

[Definition:]An **eligible
identity-constraint** of an element information item is one such that clause [4.1](041-3-11-3-constraints-on-xml-representations-of-identity-constraint-definitions.md#c-u) or clause [4.2](041-3-11-3-constraints-on-xml-representations-of-identity-constraint-definitions.md#c-k) of [Identity-constraint Satisfied (§3.11.4)](041-3-11-3-constraints-on-xml-representations-of-identity-constraint-definitions.md#cvc-identity-constraint) is satisfied
with respect to that item and that constraint,
or such that any of the element information item [[children]](https://www.w3.org/TR/xml-infoset/#infoitem.element) of that item have an
[[identity-constraint table]](042-3-11-5-identity-constraint-definition-information-set-contributions.md#e-id_constraint_table) property whose value has an entry for that constraint.

<a id="key-nt"></a>

[Definition:]A **node table** is a set
of pairs each consisting of
a [·key-sequence·](041-3-11-3-constraints-on-xml-representations-of-identity-constraint-definitions.md#key-ks) and an element node.Whenever an element information item has one or more [·eligible identity-constraints·](042-3-11-5-identity-constraint-definition-information-set-contributions.md#key-ec), in the [·post-schema-validation infoset·](004-2-1-overview-of-xml-schema.md#key-psvi) that element information item has a property as follows:

PSVI Contributions
for element information items

<a id="e-id_constraint_table"></a>

[identity-constraint table]
:   one
    **Identity-constraint Binding**
    information item for each [·eligible identity-constraint·](042-3-11-5-identity-constraint-definition-information-set-contributions.md#key-ec), with
    properties as follows:

    PSVI Contributions
    for Identity-constraint Binding information items

    <a id="cb-definition"></a>

    [definition]
    :   The [·eligible identity-constraint·](042-3-11-5-identity-constraint-definition-information-set-contributions.md#key-ec).

    <a id="cb-node_table"></a>

    [node table]

    <a id="c-kc"></a>

    :   A [·node table·](042-3-11-5-identity-constraint-definition-information-set-contributions.md#key-nt) with one entry for every
        [·key-sequence·](041-3-11-3-constraints-on-xml-representations-of-identity-constraint-definitions.md#key-ks) (call it **k**) and node (call it **n**) such that
        **one** of the following must be true:

        1There is an entry in one of the [·node
        tables·](042-3-11-5-identity-constraint-definition-information-set-contributions.md#key-nt) associated with the [[definition]](042-3-11-5-identity-constraint-definition-information-set-contributions.md#cb-definition) in an
        **Identity-constraint Binding**
        information item in at least one of the [[identity-constraint table]](042-3-11-5-identity-constraint-definition-information-set-contributions.md#e-id_constraint_table)s of the element information item [[children]](https://www.w3.org/TR/xml-infoset/#infoitem.element) of the element
        information item whose [·key-sequence·](041-3-11-3-constraints-on-xml-representations-of-identity-constraint-definitions.md#key-ks) is **k** and whose node is **n**;

        2 **n** appears with
        [·key-sequence·](041-3-11-3-constraints-on-xml-representations-of-identity-constraint-definitions.md#key-ks) **k** in the [·qualified node
        set·](041-3-11-3-constraints-on-xml-representations-of-identity-constraint-definitions.md#key-qns) for the [[definition]](042-3-11-5-identity-constraint-definition-information-set-contributions.md#cb-definition).

        provided no two entries have the same [·key-sequence·](041-3-11-3-constraints-on-xml-representations-of-identity-constraint-definitions.md#key-ks) but distinct nodes. Potential conflicts are resolved by not including any conflicting entries which would have owed their inclusion to clause [1](042-3-11-5-identity-constraint-definition-information-set-contributions.md#c-kc) above. Note that if all the conflicting entries arose under clause [1](042-3-11-5-identity-constraint-definition-information-set-contributions.md#c-kc) above, this means no entry at all will appear for the offending [·key-sequence·](041-3-11-3-constraints-on-xml-representations-of-identity-constraint-definitions.md#key-ks).

**Note:**The complexity of the above arises from the fact that
*keyref* identity-constraints may be defined on domains distinct from the
embedded domain of the identity-constraint they reference, or the domains may be the
same but self-embedding at some depth. In either case the [·node
table·](042-3-11-5-identity-constraint-definition-information-set-contributions.md#key-nt) for the referenced identity-constraint needs to propagate upwards, with
conflict resolution.The **Identity-constraint Binding**
information item, unlike others in this
specification, is essentially an internal bookkeeping mechanism. It is introduced to
support the definition of [Identity-constraint Satisfied (§3.11.4)](041-3-11-3-constraints-on-xml-representations-of-identity-constraint-definitions.md#cvc-identity-constraint) above.
Accordingly, conformant processors may, but are *not* required to,
expose them via
[[identity-constraint table]](042-3-11-5-identity-constraint-definition-information-set-contributions.md#e-id_constraint_table) properties in the [·post-schema-validation infoset·](004-2-1-overview-of-xml-schema.md#key-psvi).
In other words, the above constraints may be read as saying [·validation·](004-2-1-overview-of-xml-schema.md#key-vn) of
identity-constraints proceeds *as if* such infoset items existed.

<a id="coss-identity-constraint"></a>

#### 3.11.6 Constraints on Identity-constraint Definition Schema Components

All identity-constraint definitions (see [Identity-constraint Definitions (§3.11)](040-3-11-identity-constraint-definitions.md#cIdentity-constraint_Definitions)) must satisfy the following constraint.

<a id="c-props-correct"></a>

**Schema Component Constraint: Identity-constraint Definition Properties Correct**  

**All** of the following must be true:

1 The values of the properties of an identity-constraint definition must be as described in
the property tableau in
[The Identity-constraint Definition Schema Component (§3.11.1)](040-3-11-identity-constraint-definitions.md#Identity-constraint_Definition_details), modulo the impact of [Missing Sub-components (§5.3)](058-5-3-missing-sub-components.md#conformance-missing).

2 If the [{identity-constraint category}](040-3-11-identity-constraint-definitions.md#identity-constraint_name) is *keyref*, the
cardinality of the [{fields}](040-3-11-identity-constraint-definitions.md#fields) must equal that of the [{fields}](040-3-11-identity-constraint-definitions.md#fields) of the [{referenced key}](040-3-11-identity-constraint-definitions.md#referenced_key).

<a id="c-selector-xpath"></a>

**Schema Component Constraint: Selector Value OK**  

**All** of the following must be true:

1 The [{selector}](040-3-11-identity-constraint-definitions.md#selector) must be a valid XPath
expression, as defined in [[XPath]](060-b-references-normative.md#bib-xpath).

2 **One** of the following must be true:

2.1 It must conform to the following extended BNF:

<table border="1" cellpadding="5" class="scrap" width="100%"><tbody><tr align="left"><td><strong><font color="red">Selector XPath expressions</font></strong></td></tr><tr><td><a id="Selector"></a><a id="Path"></a><a id="Step"></a><a id="NameTest"></a><table border="0"><tbody><tr valign="baseline"><td><a id="Selector" name="Selector"></a>[1]   </td><td><code>Selector</code></td><td>   ::=   </td><td><code><a href="042-3-11-5-identity-constraint-definition-information-set-contributions.md#Path">Path</a> ( '|' <a href="042-3-11-5-identity-constraint-definition-information-set-contributions.md#Path">Path</a> )*</code></td></tr></tbody><tbody><tr valign="baseline"><td><a id="Path" name="Path"></a>[2]   </td><td><code>Path</code></td><td>   ::=   </td><td><code>('.//')? <a href="042-3-11-5-identity-constraint-definition-information-set-contributions.md#Step">Step</a> ( '/' <a href="042-3-11-5-identity-constraint-definition-information-set-contributions.md#Step">Step</a> )*</code></td></tr></tbody><tbody><tr valign="baseline"><td><a id="Step" name="Step"></a>[3]   </td><td><code>Step</code></td><td>   ::=   </td><td><code>'.' | <a href="042-3-11-5-identity-constraint-definition-information-set-contributions.md#NameTest">NameTest</a></code></td></tr></tbody><tbody><tr valign="baseline"><td><a id="NameTest" name="NameTest"></a>[4]   </td><td><code>NameTest</code></td><td>   ::=   </td><td><code><a href="https://www.w3.org/TR/REC-xml-names/#NT-QName">QName</a> | '*' | <a href="https://www.w3.org/TR/REC-xml-names/#NT-NCName">NCName</a> ':' '*'</code></td></tr></tbody></table></td></tr></tbody></table>

2.2 It must be an XPath expression involving the `child` axis whose abbreviated form is
as given above.

For readability, whitespace may be used in selector XPath expressions even though not
explicitly allowed by the grammar: [whitespace](042-3-11-5-identity-constraint-definition-information-set-contributions.md#whitespace) may be freely added within patterns before or after any [token](042-3-11-5-identity-constraint-definition-information-set-contributions.md#token).

<table border="1" cellpadding="5" class="scrap" width="100%"><tbody><tr align="left"><td><strong><font color="red">Lexical productions</font></strong></td></tr><tr><td><a id="token"></a><a id="whitespace"></a><table border="0"><tbody><tr valign="baseline"><td><a id="token" name="token"></a>[5]   </td><td><code>token</code></td><td>   ::=   </td><td><code>'.' | '/' | '//' | '|' | '@' | <a href="042-3-11-5-identity-constraint-definition-information-set-contributions.md#NameTest">NameTest</a></code></td></tr></tbody><tbody><tr valign="baseline"><td><a id="whitespace" name="whitespace"></a>[6]   </td><td><code>whitespace</code></td><td>   ::=   </td><td><code><a href="https://www.w3.org/TR/REC-xml#NT-S">S</a></code></td></tr></tbody></table></td></tr></tbody></table>

When tokenizing, the longest possible token is always returned.

<a id="c-fields-xpaths"></a>

**Schema Component Constraint: Fields Value OK**  

**All** of the following must be true:

1 Each member of the [{fields}](040-3-11-identity-constraint-definitions.md#fields) must be a valid XPath
expression, as defined in [[XPath]](060-b-references-normative.md#bib-xpath).

2 **One** of the following must be true:

2.1 It must conform to the extended BNF given
above for [Selector](042-3-11-5-identity-constraint-definition-information-set-contributions.md#Selector), with the following modification:

<table border="1" cellpadding="5" class="scrap" width="100%"><tbody><tr align="left"><td><strong><font color="red">Path in Field XPath expressions</font></strong></td></tr><tr><td><a id="fPath"></a><table border="0"><tbody><tr valign="baseline"><td><a id="fPath" name="fPath"></a>[7]   </td><td><code>Path</code></td><td>   ::=   </td><td><code>('.//')? ( <a href="042-3-11-5-identity-constraint-definition-information-set-contributions.md#Step">Step</a> '/' )* ( <a href="042-3-11-5-identity-constraint-definition-information-set-contributions.md#Step">Step</a> | '@' <a href="042-3-11-5-identity-constraint-definition-information-set-contributions.md#NameTest">NameTest</a> )</code></td></tr></tbody></table></td></tr></tbody></table>

This production differs from the one above in allowing the final
step to match an attribute node.

2.2 It must be an XPath expression involving the `child` and/or `attribute` axes whose abbreviated form is
as given above.

For readability, whitespace may be used in field XPath expressions even though not
explicitly allowed by the grammar: [whitespace](042-3-11-5-identity-constraint-definition-information-set-contributions.md#whitespace) may be freely added within patterns before or after any [token](042-3-11-5-identity-constraint-definition-information-set-contributions.md#token).When tokenizing, the longest possible token is always returned.

