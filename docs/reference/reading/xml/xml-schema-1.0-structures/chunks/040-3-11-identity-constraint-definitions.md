# 3.11 Identity-constraint Definitions

Source: [XML Schema Part 1: Structures](https://www.w3.org/TR/2004/REC-xmlschema-1-20041028/) · 2nd edition, 2004-10-28.

[Document index](../README.md) · [Topic guide](../../../../README.md) · [Publisher](https://www.w3.org/TR/2004/REC-xmlschema-1-20041028/).

HTML normalized to Markdown. Complex tables and mathematical markup retain HTML; illustrations are stored locally.

[Previous](039-3-10-4-wildcard-validation-rules.md) · [Next](041-3-11-3-constraints-on-xml-representations-of-identity-constraint-definitions.md)

<a id="cIdentity-constraint_Definitions"></a>

### 3.11 Identity-constraint Definitions

3.11.1 [The Identity-constraint Definition Schema Component](040-3-11-identity-constraint-definitions.md#Identity-constraint_Definition_details)  
        3.11.2 [XML Representation of Identity-constraint Definition Schema Components](040-3-11-identity-constraint-definitions.md#declare-key)  
        3.11.3 [Constraints on XML Representations of Identity-constraint Definitions](041-3-11-3-constraints-on-xml-representations-of-identity-constraint-definitions.md#d0e13803)  
        3.11.4 [Identity-constraint Definition Validation Rules](041-3-11-3-constraints-on-xml-representations-of-identity-constraint-definitions.md#d0e13819)  
        3.11.5 [Identity-constraint Definition Information Set Contributions](042-3-11-5-identity-constraint-definition-information-set-contributions.md#d0e14060)  
        3.11.6 [Constraints on Identity-constraint Definition Schema Components](042-3-11-5-identity-constraint-definition-information-set-contributions.md#coss-identity-constraint)

Identity-constraint definition components provide for uniqueness and
reference constraints with respect to the contents of multiple elements and attributes.

Example



```
<xs:key name="fullName">
 <xs:selector xpath=".//person"/>
 <xs:field xpath="forename"/>
 <xs:field xpath="surname"/>
</xs:key>

<xs:keyref name="personRef" refer="fullName">
 <xs:selector xpath=".//personPointer"/>
 <xs:field xpath="@first"/>
 <xs:field xpath="@last"/>
</xs:keyref>

<xs:unique name="nearlyID">
 <xs:selector xpath=".//*"/>
 <xs:field xpath="@id"/>
</xs:unique>
```




XML representations for the three kinds of identity-constraint definitions.

<a id="Identity-constraint_Definition_details"></a>

#### 3.11.1 The Identity-constraint Definition Schema Component

The identity-constraint definition schema component has the following
properties:

Schema Component: [Identity-constraint Definition](005-2-2-1-3-complex-type-definition.md#Identity-constraint_Definition)

<a id="c-name"></a>

{name}
:   An NCName as defined by [[XML-Namespaces]](060-b-references-normative.md#ref-xml-namespaces).

<a id="c-target_namespace"></a>

{target namespace}
:   Either [·absent·](008-3-schema-component-details.md#key-null) or a namespace name, as defined in [[XML-Namespaces]](060-b-references-normative.md#ref-xml-namespaces).

<a id="identity-constraint_name"></a>

{identity-constraint category}
:   One of *key*, *keyref* or *unique*.

<a id="selector"></a>

{selector}
:   A restricted XPath ([[XPath]](060-b-references-normative.md#bib-xpath)) expression.

<a id="fields"></a>

{fields}
:   A non-empty list of restricted XPath ([[XPath]](060-b-references-normative.md#bib-xpath)) expressions.

<a id="referenced_key"></a>

{referenced key}
:   Required if [{identity-constraint category}](040-3-11-identity-constraint-definitions.md#identity-constraint_name) is *keyref*, forbidden
    otherwise. An identity-constraint definition with [{identity-constraint category}](040-3-11-identity-constraint-definitions.md#identity-constraint_name)
    equal to *key* or *unique*.

<a id="rc-annotation"></a>

{annotation}
:   Optional. A set of annotations.

Identity-constraint definitions are identified by their [{name}](040-3-11-identity-constraint-definitions.md#c-name) and [{target namespace}](040-3-11-identity-constraint-definitions.md#c-target_namespace); Identity-constraint definition identities must be unique within an [·XML Schema·](004-2-1-overview-of-xml-schema.md#key-schema). See [References to schema components across namespaces (§4.2.3)](054-4-2-3-references-to-schema-components-across-namespaces.md#composition-schemaImport) for the use of component
identifiers when importing one schema into another.

Informally, [{identity-constraint category}](040-3-11-identity-constraint-definitions.md#identity-constraint_name) identifies the Identity-constraint definition as playing one of
three roles:

- (*unique*) the Identity-constraint definition asserts uniqueness, with respect to the content
  identified by [{selector}](040-3-11-identity-constraint-definitions.md#selector), of the tuples resulting from
  evaluation of the [{fields}](040-3-11-identity-constraint-definitions.md#fields) XPath expression(s).
- (*key*) the Identity-constraint definition asserts uniqueness as for
  *unique*. *key* further asserts that all selected content
  actually has such tuples.
- (*keyref*) the Identity-constraint definition asserts a correspondence, with respect to the content
  identified by [{selector}](040-3-11-identity-constraint-definitions.md#selector), of the tuples resulting from
  evaluation of the [{fields}](040-3-11-identity-constraint-definitions.md#fields) XPath expression(s), with those of the [{referenced key}](040-3-11-identity-constraint-definitions.md#referenced_key).

These constraints are specified along side the specification of types for the
attributes and elements involved, i.e. something declared as of type integer
may also serve as a key. Each constraint declaration has a name, which exists in a
single symbol space for constraints. The equality and inequality conditions
appealed to in checking these constraints apply to the *value* of
the fields selected, so that for example `3.0` and `3`
would be conflicting keys if they were both number, but non-conflicting if
they were both strings, or one was a string and one a number. Values of
differing type can only be equal if one type is derived from the other, and the
value is in the value space of both.

Overall the augmentations to XML's `ID/IDREF` mechanism are:

- Functioning as a part of an identity-constraint is in addition to, not instead of,
  having a type;
- Not just attribute values, but also element content and combinations
  of values and content can be declared to be unique;
- Identity-constraints are specified to hold within the scope of particular elements;
- (Combinations of) attribute values and/or element content can be
  declared to be keys, that is, not only unique, but always present and non-nillable;
- The comparison between *keyref* [{fields}](040-3-11-identity-constraint-definitions.md#fields) and
  *key* or *unique* [{fields}](040-3-11-identity-constraint-definitions.md#fields) is by value equality,
  not by string equality.

[{selector}](040-3-11-identity-constraint-definitions.md#selector) specifies a restricted XPath ([[XPath]](060-b-references-normative.md#bib-xpath)) expression relative to
instances of the element being declared. This must identify a node set of
subordinate elements (i.e. contained within the declared element) to which the constraint applies.

[{fields}](040-3-11-identity-constraint-definitions.md#fields) specifies XPath expressions relative to each
element selected by a [{selector}](040-3-11-identity-constraint-definitions.md#selector). This must identify
a single node (element or attribute) whose content or value, which must be
of a simple type, is used in the constraint. It is possible to specify an
ordered list of [{fields}](040-3-11-identity-constraint-definitions.md#fields)s, to cater to multi-field keys,
keyrefs, and uniqueness constraints.

In order to reduce the burden on implementers, in particular
implementers of streaming processors, only restricted subsets of XPath
expressions are allowed in [{selector}](040-3-11-identity-constraint-definitions.md#selector) and [{fields}](040-3-11-identity-constraint-definitions.md#fields). The details are given in [Constraints on Identity-constraint Definition Schema Components (§3.11.6)](042-3-11-5-identity-constraint-definition-information-set-contributions.md#coss-identity-constraint).

**Note:**Provision for multi-field keys etc. goes beyond what is supported by `xsl:key`.

See [Annotations (§3.13)](044-3-13-annotations.md#cAnnotations) for information on the role of the
[{annotation}](040-3-11-identity-constraint-definitions.md#rc-annotation) property.

<a id="declare-key"></a>

#### 3.11.2 XML Representation of Identity-constraint Definition Schema Components

The XML representation for an identity-constraint definition schema component is
either a
[<key>](040-3-11-identity-constraint-definitions.md#element-key), a [<keyref>](040-3-11-identity-constraint-definitions.md#element-keyref) or a [<unique>](040-3-11-identity-constraint-definitions.md#element-unique)
element information item. The correspondences between the
properties of those information items and
properties of the component they correspond to are as follows:

XML Representation Summary: `unique` Element Information Item

<a id="element-unique"></a>

<unique  
  id = [ID](https://www.w3.org/TR/2004/REC-xmlschema-2-20041028/datatypes.html#ID)  
  **name** = [NCName](https://www.w3.org/TR/2004/REC-xmlschema-2-20041028/datatypes.html#NCName)  
  *{any attributes with non-schema namespace . . .}*>  
*Content:* ([annotation](044-3-13-annotations.md#element-annotation)?, ([selector](040-3-11-identity-constraint-definitions.md#element-selector), [field](040-3-11-identity-constraint-definitions.md#element-field)+))  
</unique>

<a id="element-key"></a>

<key  
  id = [ID](https://www.w3.org/TR/2004/REC-xmlschema-2-20041028/datatypes.html#ID)  
  **name** = [NCName](https://www.w3.org/TR/2004/REC-xmlschema-2-20041028/datatypes.html#NCName)  
  *{any attributes with non-schema namespace . . .}*>  
*Content:* ([annotation](044-3-13-annotations.md#element-annotation)?, ([selector](040-3-11-identity-constraint-definitions.md#element-selector), [field](040-3-11-identity-constraint-definitions.md#element-field)+))  
</key>

<a id="element-keyref"></a>

<keyref  
  id = [ID](https://www.w3.org/TR/2004/REC-xmlschema-2-20041028/datatypes.html#ID)  
  **name** = [NCName](https://www.w3.org/TR/2004/REC-xmlschema-2-20041028/datatypes.html#NCName)  
  **refer** = [QName](https://www.w3.org/TR/2004/REC-xmlschema-2-20041028/datatypes.html#QName)  
  *{any attributes with non-schema namespace . . .}*>  
*Content:* ([annotation](044-3-13-annotations.md#element-annotation)?, ([selector](040-3-11-identity-constraint-definitions.md#element-selector), [field](040-3-11-identity-constraint-definitions.md#element-field)+))  
</keyref>

<a id="element-selector"></a>

<selector  
  id = [ID](https://www.w3.org/TR/2004/REC-xmlschema-2-20041028/datatypes.html#ID)  
  **xpath** = *a subset of XPath expression, see below*  
  *{any attributes with non-schema namespace . . .}*>  
*Content:* ([annotation](044-3-13-annotations.md#element-annotation)?)  
</selector>

<a id="element-field"></a>

<field  
  id = [ID](https://www.w3.org/TR/2004/REC-xmlschema-2-20041028/datatypes.html#ID)  
  **xpath** = *a subset of XPath expression, see below*  
  *{any attributes with non-schema namespace . . .}*>  
*Content:* ([annotation](044-3-13-annotations.md#element-annotation)?)  
</field>

