# 3.2.6 Constraints on Attribute Declaration Schema Components

Source: [XML Schema Part 1: Structures](https://www.w3.org/TR/2004/REC-xmlschema-1-20041028/) · 2nd edition, 2004-10-28.

[Document index](../README.md) · [Topic guide](../../../../README.md) · [Publisher](https://www.w3.org/TR/2004/REC-xmlschema-1-20041028/).

HTML normalized to Markdown. Complex tables and mathematical markup retain HTML; illustrations are stored locally.

[Previous](011-3-2-4-attribute-declaration-validation-rules.md) · [Next](013-3-3-element-declarations.md)

<a id="a-schema_default"></a>

[schema default]
:   The [canonical lexical representation](https://www.w3.org/TR/2004/REC-xmlschema-2-20041028/datatypes.html#dt-canonical-representation) of
    the declaration's [{value constraint}](009-3-2-attribute-declarations.md#a-value_constraint) value.

If the attribute information item was not [·strictly assessed·](011-3-2-4-attribute-declaration-validation-rules.md#key-svaa), then instead of the values specified above,

1 The item's [[schema normalized value]](011-3-2-4-attribute-declaration-validation-rules.md#a-schema_normalized_value)
property has
the [·initial value·](008-3-schema-component-details.md#key-iv) of the item as its value;

2 The [[type definition]](011-3-2-4-attribute-declaration-validation-rules.md#a-type_definition) and
[[member type definition]](011-3-2-4-attribute-declaration-validation-rules.md#a-member_type_definition) properties, or their
alternatives, are based on the [·simple ur-type definition·](048-3-14-7-built-in-simple-type-definition.md#simple-ur-type-itself).

<a id="coss-attribute"></a>

#### 3.2.6 Constraints on Attribute Declaration Schema Components

All attribute declarations (see [Attribute Declarations (§3.2)](009-3-2-attribute-declarations.md#cAttribute_Declarations)) must satisfy the following constraints.

<a id="a-props-correct"></a>

**Schema Component Constraint: Attribute Declaration Properties Correct**  

**All** of the following must be true:

1 The values of the properties of an attribute declaration must be as described in
the property tableau in
[The Attribute Declaration Schema Component (§3.2.1)](009-3-2-attribute-declarations.md#Attribute_Declaration_details), modulo the impact of [Missing Sub-components (§5.3)](058-5-3-missing-sub-components.md#conformance-missing).

2 if there is a [{value constraint}](009-3-2-attribute-declarations.md#a-value_constraint), the [canonical lexical representation](https://www.w3.org/TR/2004/REC-xmlschema-2-20041028/datatypes.html#dt-canonical-representation) of its value must be
[·valid·](004-2-1-overview-of-xml-schema.md#key-vn) with respect to the [{type definition}](009-3-2-attribute-declarations.md#a-simple_type_definition) as
defined in [String Valid (§3.14.4)](047-3-14-3-constraints-on-xml-representations-of-simple-type-definitions.md#cvc-simple-type).

3 If the [{type definition}](009-3-2-attribute-declarations.md#a-simple_type_definition) is or is derived from [ID](https://www.w3.org/TR/2004/REC-xmlschema-2-20041028/datatypes.html#ID) then there must not be a [{value constraint}](009-3-2-attribute-declarations.md#a-value_constraint).

<a id="no-xmlns"></a>

**Schema Component Constraint: xmlns Not Allowed**  

The [{name}](009-3-2-attribute-declarations.md#a-name) of an attribute declaration must not match `xmlns`.

**Note:**The [{name}](009-3-2-attribute-declarations.md#a-name) of an attribute is an [·NCName·](050-3-15-2-1-references-to-schema-components.md#gloss-NCName), which implicitly
prohibits attribute declarations of the form `xmlns:*`.

<a id="no-xsi"></a>

**Schema Component Constraint: xsi: Not Allowed**  

The [{target namespace}](009-3-2-attribute-declarations.md#a-target_namespace) of an attribute declaration,
whether local or top-level, must not match `http://www.w3.org/2001/XMLSchema-instance`
(unless it is one of the four built-in declarations given in the next section).

**Note:**This reinforces the special status of these attributes, so that they not
only *need* not be declared to be allowed in instances, but
*must* not be declared. It also removes any temptation to experiment with supplying global or fixed values
for e.g. `xsi:type` or `xsi:nil`, which would be
seriously misleading, as they would have no effect.

<a id="d0e3067"></a>

#### 3.2.7 Built-in Attribute Declarations

There are four attribute declarations present in every
schema by definition:

<a id="xsi.type"></a>

<table border="1" class="schemaComp"><thead><tr><th><strong><a id="xsi.type" name="xsi.type">Attribute Declaration for the 'type' attribute</a></strong></th></tr></thead><tbody><tr><td><table border="0" cellpadding="3"><thead><tr><th align="left">Property</th><th align="left">Value</th></tr></thead><tbody valign="top"><tr><td><a class="propref" href="009-3-2-attribute-declarations.md#a-name">{name}</a></td><td><code>type</code></td></tr><tr><td><a class="propref" href="009-3-2-attribute-declarations.md#a-target_namespace">{target namespace}</a></td><td><code>http://www.w3.org/2001/XMLSchema-instance</code></td></tr><tr><td><a class="propref" href="013-3-3-element-declarations.md#type_definition">{type definition}</a></td><td>The built-in <a href="https://www.w3.org/TR/2004/REC-xmlschema-2-20041028/datatypes.html#QName">QName</a> simple
type definition</td></tr><tr><td><a class="propref" href="009-3-2-attribute-declarations.md#a-scope">{scope}</a></td><td><i>global</i></td></tr><tr><td><a class="propref" href="009-3-2-attribute-declarations.md#a-value_constraint">{value constraint}</a></td><td><a class="termref" href="008-3-schema-component-details.md#key-null"><span class="arrow">·</span>absent<span class="arrow">·</span></a></td></tr><tr><td><a class="propref" href="009-3-2-attribute-declarations.md#a-annotation">{annotation}</a></td><td><a class="termref" href="008-3-schema-component-details.md#key-null"><span class="arrow">·</span>absent<span class="arrow">·</span></a></td></tr></tbody></table></td></tr></tbody></table>

<a id="xsi.nil"></a>

<table border="1" class="schemaComp"><thead><tr><th><strong><a id="xsi.nil" name="xsi.nil">Attribute Declaration for the 'nil' attribute</a></strong></th></tr></thead><tbody><tr><td><table border="0" cellpadding="3"><thead><tr><th align="left">Property</th><th align="left">Value</th></tr></thead><tbody valign="top"><tr><td><a class="propref" href="009-3-2-attribute-declarations.md#a-name">{name}</a></td><td><code>nil</code></td></tr><tr><td><a class="propref" href="009-3-2-attribute-declarations.md#a-target_namespace">{target namespace}</a></td><td><code>http://www.w3.org/2001/XMLSchema-instance</code></td></tr><tr><td><a class="propref" href="013-3-3-element-declarations.md#type_definition">{type definition}</a></td><td>The built-in <a href="https://www.w3.org/TR/2004/REC-xmlschema-2-20041028/datatypes.html#boolean">boolean</a> simple
type definition</td></tr><tr><td><a class="propref" href="009-3-2-attribute-declarations.md#a-scope">{scope}</a></td><td><i>global</i></td></tr><tr><td><a class="propref" href="009-3-2-attribute-declarations.md#a-value_constraint">{value constraint}</a></td><td><a class="termref" href="008-3-schema-component-details.md#key-null"><span class="arrow">·</span>absent<span class="arrow">·</span></a></td></tr><tr><td><a class="propref" href="009-3-2-attribute-declarations.md#a-annotation">{annotation}</a></td><td><a class="termref" href="008-3-schema-component-details.md#key-null"><span class="arrow">·</span>absent<span class="arrow">·</span></a></td></tr></tbody></table></td></tr></tbody></table>

<a id="xsi.schemaLocation"></a>

<table border="1" class="schemaComp"><thead><tr><th><strong><a id="xsi.schemaLocation" name="xsi.schemaLocation">Attribute Declaration for the 'schemaLocation' attribute</a></strong></th></tr></thead><tbody><tr><td><table border="0" cellpadding="3"><thead><tr><th align="left">Property</th><th align="left">Value</th></tr></thead><tbody valign="top"><tr><td><a class="propref" href="009-3-2-attribute-declarations.md#a-name">{name}</a></td><td><code>schemaLocation</code></td></tr><tr><td><a class="propref" href="009-3-2-attribute-declarations.md#a-target_namespace">{target namespace}</a></td><td><code>http://www.w3.org/2001/XMLSchema-instance</code></td></tr><tr><td><a class="propref" href="013-3-3-element-declarations.md#type_definition">{type definition}</a></td><td>An anonymous simple type definition, as follows:
       <table border="0" cellpadding="3"><thead><tr><th align="left">Property</th><th align="left">Value</th></tr></thead><tbody valign="top"><tr><td><a class="propref" href="045-3-14-simple-type-definitions.md#st-name">{name}</a></td><td><a class="termref" href="008-3-schema-component-details.md#key-null"><span class="arrow">·</span>absent<span class="arrow">·</span></a></td></tr><tr><td><a class="propref" href="045-3-14-simple-type-definitions.md#st-target_namespace">{target namespace}</a></td><td><code>http://www.w3.org/2001/XMLSchema-instance</code></td></tr><tr><td><a class="propref" href="045-3-14-simple-type-definitions.md#st-base_type_definition">{base type definition}</a></td><td>The built in <a class="termref" href="048-3-14-7-built-in-simple-type-definition.md#simple-ur-type-itself"><span class="arrow">·</span>simple ur-type definition<span class="arrow">·</span></a></td></tr><tr><td><a class="propref" href="045-3-14-simple-type-definitions.md#facets">{facets}</a></td><td><a class="termref" href="008-3-schema-component-details.md#key-null"><span class="arrow">·</span>absent<span class="arrow">·</span></a></td></tr><tr><td><a class="propref" href="045-3-14-simple-type-definitions.md#variety">{variety}</a></td><td><i>list</i></td></tr><tr><td><a class="propref" href="045-3-14-simple-type-definitions.md#st-item_type_definition">{item type definition}</a></td><td>The built-in <a href="https://www.w3.org/TR/2004/REC-xmlschema-2-20041028/datatypes.html#anyURI">anyURI</a> simple
type definition</td></tr><tr><td><a class="propref" href="045-3-14-simple-type-definitions.md#st-annotation">{annotation}</a></td><td><a class="termref" href="008-3-schema-component-details.md#key-null"><span class="arrow">·</span>absent<span class="arrow">·</span></a></td></tr></tbody></table>
</td></tr><tr><td><a class="propref" href="009-3-2-attribute-declarations.md#a-scope">{scope}</a></td><td><i>global</i></td></tr><tr><td><a class="propref" href="009-3-2-attribute-declarations.md#a-value_constraint">{value constraint}</a></td><td><a class="termref" href="008-3-schema-component-details.md#key-null"><span class="arrow">·</span>absent<span class="arrow">·</span></a></td></tr><tr><td><a class="propref" href="009-3-2-attribute-declarations.md#a-annotation">{annotation}</a></td><td><a class="termref" href="008-3-schema-component-details.md#key-null"><span class="arrow">·</span>absent<span class="arrow">·</span></a></td></tr></tbody></table></td></tr></tbody></table>

<a id="xsi.noNamespaceSchemaLocation"></a>

<table border="1" class="schemaComp"><thead><tr><th><strong><a id="xsi.noNamespaceSchemaLocation" name="xsi.noNamespaceSchemaLocation">Attribute Declaration for the 'noNamespaceSchemaLocation' attribute</a></strong></th></tr></thead><tbody><tr><td><table border="0" cellpadding="3"><thead><tr><th align="left">Property</th><th align="left">Value</th></tr></thead><tbody valign="top"><tr><td><a class="propref" href="009-3-2-attribute-declarations.md#a-name">{name}</a></td><td><code>noNamespaceSchemaLocation</code></td></tr><tr><td><a class="propref" href="009-3-2-attribute-declarations.md#a-target_namespace">{target namespace}</a></td><td><code>http://www.w3.org/2001/XMLSchema-instance</code></td></tr><tr><td><a class="propref" href="013-3-3-element-declarations.md#type_definition">{type definition}</a></td><td>The built-in <a href="https://www.w3.org/TR/2004/REC-xmlschema-2-20041028/datatypes.html#anyURI">anyURI</a> simple
type definition</td></tr><tr><td><a class="propref" href="009-3-2-attribute-declarations.md#a-scope">{scope}</a></td><td><i>global</i></td></tr><tr><td><a class="propref" href="009-3-2-attribute-declarations.md#a-value_constraint">{value constraint}</a></td><td><a class="termref" href="008-3-schema-component-details.md#key-null"><span class="arrow">·</span>absent<span class="arrow">·</span></a></td></tr><tr><td><a class="propref" href="009-3-2-attribute-declarations.md#a-annotation">{annotation}</a></td><td><a class="termref" href="008-3-schema-component-details.md#key-null"><span class="arrow">·</span>absent<span class="arrow">·</span></a></td></tr></tbody></table></td></tr></tbody></table>

