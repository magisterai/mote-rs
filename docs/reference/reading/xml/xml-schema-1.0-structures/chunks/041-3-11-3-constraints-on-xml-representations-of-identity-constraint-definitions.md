# 3.11.3 Constraints on XML Representations of Identity-constraint Definitions

Source: [XML Schema Part 1: Structures](https://www.w3.org/TR/2004/REC-xmlschema-1-20041028/) · 2nd edition, 2004-10-28.

[Document index](../README.md) · [Topic guide](../../../../README.md) · [Publisher](https://www.w3.org/TR/2004/REC-xmlschema-1-20041028/).

HTML normalized to Markdown. Complex tables and mathematical markup retain HTML; illustrations are stored locally.

[Previous](040-3-11-identity-constraint-definitions.md) · [Next](042-3-11-5-identity-constraint-definition-information-set-contributions.md)

<table class="reprcomp"><thead><tr><th><a href="040-3-11-identity-constraint-definitions.md#Identity-constraint_Definition_details">Identity-constraint Definition</a><strong> Schema Component</strong></th></tr></thead><tbody><tr><td><table border="0" cellpadding="3"><thead><tr><th align="left">Property</th><th align="left">Representation</th></tr></thead><tbody valign="top"><tr valign="top"><td><a class="propref" href="040-3-11-identity-constraint-definitions.md#c-name">{name}</a></td><td>The <a class="termref" href="008-3-schema-component-details.md#key-vv"><span class="arrow">·</span>actual value<span class="arrow">·</span></a> of the <code>name</code> <a class="xpropref" href="https://www.w3.org/TR/xml-infoset/#infoitem.element">[attribute]</a></td></tr><tr valign="top"><td><a class="propref" href="040-3-11-identity-constraint-definitions.md#c-target_namespace">{target namespace}</a></td><td>The <a class="termref" href="008-3-schema-component-details.md#key-vv"><span class="arrow">·</span>actual value<span class="arrow">·</span></a> of the
<code>targetNamespace</code> <a class="xpropref" href="https://www.w3.org/TR/xml-infoset/#infoitem.element">[attribute]</a> of the parent <code>schema</code>
element information item.</td></tr><tr valign="top"><td><a class="propref" href="040-3-11-identity-constraint-definitions.md#identity-constraint_name">{identity-constraint category}</a></td><td>One of <i>key</i>, <i>keyref</i> or
<i>unique</i>, depending on the item.</td></tr><tr valign="top"><td><a class="propref" href="040-3-11-identity-constraint-definitions.md#selector">{selector}</a></td><td>A restricted XPath expression corresponding to the <a class="termref" href="008-3-schema-component-details.md#key-vv"><span class="arrow">·</span>actual value<span class="arrow">·</span></a> of
the <code>xpath</code> <a class="xpropref" href="https://www.w3.org/TR/xml-infoset/#infoitem.element">[attribute]</a> of the <a class="eltref" href="040-3-11-identity-constraint-definitions.md#element-selector">&lt;selector&gt;</a> element information item among the <a class="xpropref" href="https://www.w3.org/TR/xml-infoset/#infoitem.element">[children]</a></td></tr><tr valign="top"><td><a class="propref" href="040-3-11-identity-constraint-definitions.md#fields">{fields}</a></td><td>A sequence of XPath expressions, corresponding to the
<a class="termref" href="008-3-schema-component-details.md#key-vv"><span class="arrow">·</span>actual value<span class="arrow">·</span></a>s of the <code>xpath</code> <a class="xpropref" href="https://www.w3.org/TR/xml-infoset/#infoitem.element">[attribute]</a>s of the <a class="eltref" href="040-3-11-identity-constraint-definitions.md#element-field">&lt;field&gt;</a> element information item <a class="xpropref" href="https://www.w3.org/TR/xml-infoset/#infoitem.element">[children]</a>, in order.</td></tr><tr valign="top"><td><a class="propref" href="040-3-11-identity-constraint-definitions.md#referenced_key">{referenced key}</a></td><td>If the item is a <a class="eltref" href="040-3-11-identity-constraint-definitions.md#element-keyref">&lt;keyref&gt;</a>, the
identity-constraint definition <a class="termref" href="050-3-15-2-1-references-to-schema-components.md#src-resolve"><span class="arrow">·</span>resolved<span class="arrow">·</span></a> to by the
<a class="termref" href="008-3-schema-component-details.md#key-vv"><span class="arrow">·</span>actual value<span class="arrow">·</span></a> of the <code>refer</code> <a class="xpropref" href="https://www.w3.org/TR/xml-infoset/#infoitem.element">[attribute]</a>, otherwise <a class="termref" href="008-3-schema-component-details.md#key-null"><span class="arrow">·</span>absent<span class="arrow">·</span></a>.</td></tr><tr valign="top"><td><a class="propref" href="040-3-11-identity-constraint-definitions.md#rc-annotation">{annotation}</a></td><td>The annotations corresponding to the <a class="eltref" href="044-3-13-annotations.md#element-annotation">&lt;annotation&gt;</a> element information item in the
<a class="xpropref" href="https://www.w3.org/TR/xml-infoset/#infoitem.element">[children]</a>, if present, and in the <a class="eltref" href="040-3-11-identity-constraint-definitions.md#element-selector">&lt;selector&gt;</a> and <a class="eltref" href="040-3-11-identity-constraint-definitions.md#element-field">&lt;field&gt;</a> <a class="xpropref" href="https://www.w3.org/TR/xml-infoset/#infoitem.element">[children]</a>, if present, otherwise <a class="termref" href="008-3-schema-component-details.md#key-null"><span class="arrow">·</span>absent<span class="arrow">·</span></a>.</td></tr></tbody></table></td></tr></tbody></table>

Example



```
<xs:element name="vehicle">
 <xs:complexType>
  . . .
  <xs:attribute name="plateNumber" type="xs:integer"/>
  <xs:attribute name="state" type="twoLetterCode"/>
 </xs:complexType>
</xs:element>

<xs:element name="state">
 <xs:complexType>
  <xs:sequence>
   <xs:element name="code" type="twoLetterCode"/>
   <xs:element ref="vehicle" maxOccurs="unbounded"/>
   <xs:element ref="person" maxOccurs="unbounded"/>
  </xs:sequence>
 </xs:complexType>

 <xs:key name="reg"> <!-- vehicles are keyed by their plate within states -->
  <xs:selector xpath=".//vehicle"/>
  <xs:field xpath="@plateNumber"/>
 </xs:key>
</xs:element>

<xs:element name="root">
 <xs:complexType>
  <xs:sequence>
   . . .
   <xs:element ref="state" maxOccurs="unbounded"/>
   . . .
  </xs:sequence>
 </xs:complexType>

 <xs:key name="state"> <!-- states are keyed by their code -->
  <xs:selector xpath=".//state"/>
  <xs:field xpath="code"/>
 </xs:key>

 <xs:keyref name="vehicleState" refer="state">
  <!-- every vehicle refers to its state -->
  <xs:selector xpath=".//vehicle"/>
  <xs:field xpath="@state"/>
 </xs:keyref>

 <xs:key name="regKey"> <!-- vehicles are keyed by a pair of state and plate -->
  <xs:selector xpath=".//vehicle"/>
  <xs:field xpath="@state"/>
  <xs:field xpath="@plateNumber"/>
 </xs:key>

 <xs:keyref name="carRef" refer="regKey"> <!-- people's cars are a reference -->
  <xs:selector xpath=".//car"/>
  <xs:field xpath="@regState"/>
  <xs:field xpath="@regPlate"/>
 </xs:keyref>

</xs:element>

<xs:element name="person">
 <xs:complexType>
  <xs:sequence>
   . . .
   <xs:element name="car">
    <xs:complexType>
     <xs:attribute name="regState" type="twoLetterCode"/>
     <xs:attribute name="regPlate" type="xs:integer"/>
    </xs:complexType>
   </xs:element>
  </xs:sequence>
 </xs:complexType>
</xs:element>
```




A `state` element is defined, which
contains a `code` child and some `vehicle` and `person`
children. A `vehicle` in turn has a `plateNumber` attribute,
which is an integer, and a `state` attribute. State's
`code`s are a key for them within the document. Vehicle's
`plateNumber`s are a key for them within states, and
`state` and
`plateNumber` is asserted to be a *key* for
`vehicle` within the document as a whole. Furthermore, a `person` element has
an empty `car` child, with `regState` and
`regPlate` attributes, which are then asserted together to refer to
`vehicle`s via the `carRef` constraint. The requirement
that a `vehicle`'s `state` match its containing
`state`'s `code` is not expressed here.

<a id="d0e13803"></a>

#### 3.11.3 Constraints on XML Representations of Identity-constraint Definitions

<a id="src-identity-constraint"></a>

**Schema Representation Constraint: Identity-constraint Definition Representation OK**  

In addition to the conditions imposed on [<key>](040-3-11-identity-constraint-definitions.md#element-key), [<keyref>](040-3-11-identity-constraint-definitions.md#element-keyref) and [<unique>](040-3-11-identity-constraint-definitions.md#element-unique) element
information items by the schema for schemas, the corresponding identity-constraint definition must satisfy the conditions set
out in [Constraints on Identity-constraint Definition Schema Components (§3.11.6)](042-3-11-5-identity-constraint-definition-information-set-contributions.md#coss-identity-constraint).

<a id="d0e13819"></a>

#### 3.11.4 Identity-constraint Definition Validation Rules

<a id="cvc-identity-constraint"></a>

**Validation Rule: Identity-constraint Satisfied**  

For an element information item to be locally [·valid·](004-2-1-overview-of-xml-schema.md#key-vn) with respect to an identity-constraint
**all** of the following must be true:

1 The [{selector}](040-3-11-identity-constraint-definitions.md#selector), with the element information item as the
context node, evaluates to a node-set (as defined in
[[XPath]](060-b-references-normative.md#bib-xpath)). 

<a id="key-tns"></a>

[Definition:]Call this the **target node set**.

2 Each node in the [·target node set·](041-3-11-3-constraints-on-xml-representations-of-identity-constraint-definitions.md#key-tns) is
either the context node oran
element node among its descendants.

3 For each node in the [·target node set·](041-3-11-3-constraints-on-xml-representations-of-identity-constraint-definitions.md#key-tns) all of the [{fields}](040-3-11-identity-constraint-definitions.md#fields), with that node as the context
node, evaluate to either an empty node-set or a node-set with exactly one
member, which must have a simple type. 

<a id="key-ks"></a>

[Definition:]Call the sequence of the
type-determined values (as defined in [[XML Schemas: Datatypes]](060-b-references-normative.md#ref-xsp2)) of the
[schema normalized value] of the element and/or attribute information items in those node-sets in order the **key-sequence** of the node.

4

<a id="key-qns"></a>

[Definition:]Call the subset of the [·target node set·](041-3-11-3-constraints-on-xml-representations-of-identity-constraint-definitions.md#key-tns) for
which all the [{fields}](040-3-11-identity-constraint-definitions.md#fields) evaluate to a node-set with exactly one
member which is an element or attribute node with a simple type the **qualified node set**.
The appropriate **case** among the following
must be true:

4.1

<a id="c-u"></a>

**If** the [{identity-constraint category}](040-3-11-identity-constraint-definitions.md#identity-constraint_name) is *unique*, **then** no two members of the [·qualified node
set·](041-3-11-3-constraints-on-xml-representations-of-identity-constraint-definitions.md#key-qns) have [·key-sequences·](041-3-11-3-constraints-on-xml-representations-of-identity-constraint-definitions.md#key-ks) whose members
are pairwise equal, as defined by [Equal](https://www.w3.org/TR/2004/REC-xmlschema-2-20041028/datatypes.html#equal) in [[XML Schemas: Datatypes]](060-b-references-normative.md#ref-xsp2).

4.2

<a id="c-k"></a>

**If** the [{identity-constraint category}](040-3-11-identity-constraint-definitions.md#identity-constraint_name) is *key*, **then** 
**all** of the following must be true:

4.2.1 The [·target node set·](041-3-11-3-constraints-on-xml-representations-of-identity-constraint-definitions.md#key-tns) and the [·qualified node
set·](041-3-11-3-constraints-on-xml-representations-of-identity-constraint-definitions.md#key-qns) are equal, that is, every member of the [·target node set·](041-3-11-3-constraints-on-xml-representations-of-identity-constraint-definitions.md#key-tns) is also a member of the [·qualified node
set·](041-3-11-3-constraints-on-xml-representations-of-identity-constraint-definitions.md#key-qns) and *vice versa*.

4.2.2 No two members of the [·qualified node
set·](041-3-11-3-constraints-on-xml-representations-of-identity-constraint-definitions.md#key-qns) have [·key-sequences·](041-3-11-3-constraints-on-xml-representations-of-identity-constraint-definitions.md#key-ks) whose members
are pairwise equal, as defined by [Equal](https://www.w3.org/TR/2004/REC-xmlschema-2-20041028/datatypes.html#equal) in [[XML Schemas: Datatypes]](060-b-references-normative.md#ref-xsp2).

4.2.3

<a id="c-nlbl"></a>

No element member of the [·key-sequence·](041-3-11-3-constraints-on-xml-representations-of-identity-constraint-definitions.md#key-ks) of any
member of the [·qualified node
set·](041-3-11-3-constraints-on-xml-representations-of-identity-constraint-definitions.md#key-qns) was assessed as [·valid·](004-2-1-overview-of-xml-schema.md#key-vn) by reference to an element
declaration whose [{nillable}](013-3-3-element-declarations.md#nillable) is *true*.

4.3

<a id="cl-krv"></a>

**If** the [{identity-constraint category}](040-3-11-identity-constraint-definitions.md#identity-constraint_name) is *keyref*, **then** for each member of the [·qualified node
set·](041-3-11-3-constraints-on-xml-representations-of-identity-constraint-definitions.md#key-qns) (call this the **keyref member**), there must be a [·node table·](042-3-11-5-identity-constraint-definition-information-set-contributions.md#key-nt) associated with the
[{referenced key}](040-3-11-identity-constraint-definitions.md#referenced_key) in the [[identity-constraint table]](042-3-11-5-identity-constraint-definition-information-set-contributions.md#e-id_constraint_table)
of the element information item (see [Identity-constraint Table (§3.11.5)](042-3-11-5-identity-constraint-definition-information-set-contributions.md#sic-key), which must be
understood as logically prior to this clause of this constraint, below) and
there must be an entry in that table whose
[·key-sequence·](041-3-11-3-constraints-on-xml-representations-of-identity-constraint-definitions.md#key-ks) is equal to the
**keyref member's** [·key-sequence·](041-3-11-3-constraints-on-xml-representations-of-identity-constraint-definitions.md#key-ks) member for
member, as defined by [Equal](https://www.w3.org/TR/2004/REC-xmlschema-2-20041028/datatypes.html#equal) in [[XML Schemas: Datatypes]](060-b-references-normative.md#ref-xsp2).

**Note:**The use of [schema normalized value] in the definition
of [·key sequence·](041-3-11-3-constraints-on-xml-representations-of-identity-constraint-definitions.md#key-ks) above means that
*default* or *fixed* value constraints may play a part in [·key sequence·](041-3-11-3-constraints-on-xml-representations-of-identity-constraint-definitions.md#key-ks)s.

