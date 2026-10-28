# B References (normative)

Source: [XML Schema Part 1: Structures](https://www.w3.org/TR/2004/REC-xmlschema-1-20041028/) · 2nd edition, 2004-10-28.

[Document index](../README.md) · [Topic guide](../../../../README.md) · [Publisher](https://www.w3.org/TR/2004/REC-xmlschema-1-20041028/).

HTML normalized to Markdown. Complex tables and mathematical markup retain HTML; illustrations are stored locally.

[Previous](059-continuation-59.md) · [Next](061-c-3-schema-representation-constraints.md)




**Note:**And that is the end of the schema for schema documents.

<a id="normative-references"></a>

## B References (normative)

<a id="ref-xml"></a>

XML 1.0 (Second Edition)
:   *Extensible
    Markup Language (XML) 1.0, Second Edition*, Tim Bray et al., eds., W3C,
    6 October 2000. See
    [http://www.w3.org/TR/2000/REC-xml-20001006](https://www.w3.org/TR/2000/REC-xml-20001006)

<a id="ref-xsreq"></a>

XML Schema Requirements
:   *XML Schema Requirements* , Ashok Malhotra and Murray Maloney, eds.,
    W3C, 15 February 1999. See [http://www.w3.org/TR/1999/NOTE-xml-schema-req-19990215](https://www.w3.org/TR/1999/NOTE-xml-schema-req-19990215)

<a id="ref-xsp2"></a>

XML Schemas: Datatypes
:   *XML Schema Part 2: Datatypes*, Paul V. Biron and Ashok
    Malhotra, eds., W3C, 2 May 2001. See [http://www.w3.org/TR/2004/REC-xmlschema-2-20041028/datatypes.html](https://www.w3.org/TR/2004/REC-xmlschema-2-20041028/datatypes.html)

<a id="ref-xmlinfo"></a>

XML-Infoset
:   *XML Information Set*,
    John Cowan and Richard Tobin, eds., W3C, 16 March 2001. See
    [http://www.w3.org/TR/2001/WD-xml-infoset-20010316/](https://www.w3.org/TR/2001/WD-xml-infoset-20010316/)

<a id="ref-xml-namespaces"></a>

XML-Namespaces
:   *Namespaces in XML*, Tim Bray et al., eds., W3C, 14 January 1999. See
    [http://www.w3.org/TR/1999/REC-xml-names-19990114/](https://www.w3.org/TR/1999/REC-xml-names-19990114/)

<a id="bib-xpath"></a>

XPath
:   *XML Path Language*, James Clark
    and Steve DeRose, eds., W3C, 16 November 1999. See [http://www.w3.org/TR/1999/REC-xpath-19991116](https://www.w3.org/TR/1999/REC-xpath-19991116)

<a id="ref-xpointer"></a>

XPointer
:   *XML
    Pointer Language (XPointer)*, Eve Maler and Steve DeRose, eds., W3C, 8 January 2001. See [http://www.w3.org/TR/2001/WD-xptr-20010108/](https://www.w3.org/TR/2001/WD-xptr-20010108/)

<a id="outcomes"></a>

## C Outcome Tabulations (normative)

To facilitate consistent reporting of schema errors and [·validation·](004-2-1-overview-of-xml-schema.md#key-vn)
failures, this section tabulates and provides unique names for all the
constraints listed in this document. Wherever such constraints have numbered
parts, reports should use the name given below plus the part number, separated
by a period ('.'). Thus for example `cos-ct-extends.1.2` should be
used to report a violation of the clause [1.2](027-3-4-5-complex-type-definition-information-set-contributions.md#c-cte) of
[Derivation Valid (Extension) (§3.4.6)](027-3-4-5-complex-type-definition-information-set-contributions.md#cos-ct-extends).

<a id="validation_failures"></a>

### C.1 Validation Rules

cvc-assess-attr
:   [Schema-Validity Assessment (Attribute)](011-3-2-4-attribute-declaration-validation-rules.md#cvc-assess-attr)

cvc-assess-elt
:   [Schema-Validity Assessment (Element)](017-3-3-5-element-declaration-information-set-contributions.md#cvc-assess-elt)

cvc-attribute
:   [Attribute Locally Valid](011-3-2-4-attribute-declaration-validation-rules.md#cvc-attribute)

cvc-au
:   [Attribute Locally Valid (Use)](030-3-5-attributeuses.md#cvc-au)

cvc-complex-type
:   [Element Locally Valid (Complex Type)](026-3-4-3-constraints-on-xml-representations-of-complex-type-definitions.md#cvc-complex-type)

cvc-datatype-valid
:   [Datatype Valid](https://www.w3.org/TR/2004/REC-xmlschema-2-20041028/datatypes.html#cvc-datatype-valid)

cvc-elt
:   [Element Locally Valid (Element)](016-3-3-3-constraints-on-xml-representations-of-element-declarations.md#cvc-elt)

cvc-enumeration-valid
:   [enumeration valid](https://www.w3.org/TR/2004/REC-xmlschema-2-20041028/datatypes.html#cvc-enumeration-valid)

cvc-facet-valid
:   [Facet Valid](https://www.w3.org/TR/2004/REC-xmlschema-2-20041028/datatypes.html#cvc-facet-valid)

cvc-fractionDigits-valid
:   [fractionDigits Valid](https://www.w3.org/TR/2004/REC-xmlschema-2-20041028/datatypes.html#cvc-fractionDigits-valid)

cvc-id
:   [Validation Root Valid (ID/IDREF)](017-3-3-5-element-declaration-information-set-contributions.md#cvc-id)

cvc-identity-constraint
:   [Identity-constraint Satisfied](041-3-11-3-constraints-on-xml-representations-of-identity-constraint-definitions.md#cvc-identity-constraint)

cvc-length-valid
:   [Length Valid](https://www.w3.org/TR/2004/REC-xmlschema-2-20041028/datatypes.html#cvc-length-valid)

cvc-maxExclusive-valid
:   [maxExclusive Valid](https://www.w3.org/TR/2004/REC-xmlschema-2-20041028/datatypes.html#cvc-maxExclusive-valid)

cvc-maxInclusive-valid
:   [maxInclusive Valid](https://www.w3.org/TR/2004/REC-xmlschema-2-20041028/datatypes.html#cvc-maxInclusive-valid)

cvc-maxLength-valid
:   [maxLength Valid](https://www.w3.org/TR/2004/REC-xmlschema-2-20041028/datatypes.html#cvc-maxLength-valid)

cvc-minExclusive-valid
:   [minExclusive Valid](https://www.w3.org/TR/2004/REC-xmlschema-2-20041028/datatypes.html#cvc-minExclusive-valid)

cvc-minInclusive-valid
:   [minInclusive Valid](https://www.w3.org/TR/2004/REC-xmlschema-2-20041028/datatypes.html#cvc-minInclusive-valid)

cvc-minLength-valid
:   [minLength Valid](https://www.w3.org/TR/2004/REC-xmlschema-2-20041028/datatypes.html#cvc-minLength-valid)

cvc-model-group
:   [Element Sequence Valid](033-3-7-6-constraints-on-model-group-definition-schema-components.md#cvc-model-group)

cvc-particle
:   [Element Sequence Locally Valid (Particle)](035-3-9-particles.md#cvc-particle)

cvc-pattern-valid
:   [pattern valid](https://www.w3.org/TR/2004/REC-xmlschema-2-20041028/datatypes.html#cvc-pattern-valid)

cvc-resolve-instance
:   [QName resolution (Instance)](050-3-15-2-1-references-to-schema-components.md#cvc-resolve-instance)

cvc-simple-type
:   [String Valid](047-3-14-3-constraints-on-xml-representations-of-simple-type-definitions.md#cvc-simple-type)

cvc-totalDigits-valid
:   [totalDigits Valid](https://www.w3.org/TR/2004/REC-xmlschema-2-20041028/datatypes.html#cvc-totalDigits-valid)

cvc-type
:   [Element Locally Valid (Type)](016-3-3-3-constraints-on-xml-representations-of-element-declarations.md#cvc-type)

cvc-wildcard
:   [Item Valid (Wildcard)](039-3-10-4-wildcard-validation-rules.md#cvc-wildcard)

cvc-wildcard-namespace
:   [Wildcard allows Namespace Name](039-3-10-4-wildcard-validation-rules.md#cvc-wildcard-namespace)

<a id="PSVI_contributions"></a>

### C.2 Contributions to the post-schema-validation infoset

attribute information item properties
:   [[attribute declaration]](011-3-2-4-attribute-declaration-validation-rules.md#a-declaration)
    ([Attribute Declaration](011-3-2-4-attribute-declaration-validation-rules.md#sic-attr-decl))
      
    [[member type definition]](011-3-2-4-attribute-declaration-validation-rules.md#a-member_type_definition)
    ([Attribute Validated by Type](011-3-2-4-attribute-declaration-validation-rules.md#sic-attrType))
      
    [[member type definition anonymous]](011-3-2-4-attribute-declaration-validation-rules.md#a-member_type_definition_anonymous)
    ([Attribute Validated by Type](011-3-2-4-attribute-declaration-validation-rules.md#sic-attrType))
      
    [[member type definition name]](011-3-2-4-attribute-declaration-validation-rules.md#a-member_type_definition_name)
    ([Attribute Validated by Type](011-3-2-4-attribute-declaration-validation-rules.md#sic-attrType))
      
    [[member type definition namespace]](011-3-2-4-attribute-declaration-validation-rules.md#a-member_type_definition_namespace)
    ([Attribute Validated by Type](011-3-2-4-attribute-declaration-validation-rules.md#sic-attrType))
      
    [[schema default]](012-3-2-6-constraints-on-attribute-declaration-schema-components.md#a-schema_default)
    ([Attribute Validated by Type](011-3-2-4-attribute-declaration-validation-rules.md#sic-attrType))
      
    [[schema error code]](011-3-2-4-attribute-declaration-validation-rules.md#a-schema_error_code)
    ([Validation Failure (Attribute)](011-3-2-4-attribute-declaration-validation-rules.md#sic-attr-error-code))
      
    [[schema normalized value]](011-3-2-4-attribute-declaration-validation-rules.md#a-schema_normalized_value)
    ([Attribute Validated by Type](011-3-2-4-attribute-declaration-validation-rules.md#sic-attrType))
      
    [[schema specified]](011-3-2-4-attribute-declaration-validation-rules.md#a-schema_specified)
    ([Assessment Outcome (Attribute)](011-3-2-4-attribute-declaration-validation-rules.md#sic-a-outcome))
      
    [[type definition]](011-3-2-4-attribute-declaration-validation-rules.md#a-type_definition)
    ([Attribute Validated by Type](011-3-2-4-attribute-declaration-validation-rules.md#sic-attrType))
      
    [[type definition anonymous]](011-3-2-4-attribute-declaration-validation-rules.md#a-type_definition_anonymous)
    ([Attribute Validated by Type](011-3-2-4-attribute-declaration-validation-rules.md#sic-attrType))
      
    [[type definition name]](011-3-2-4-attribute-declaration-validation-rules.md#a-type_definition_name)
    ([Attribute Validated by Type](011-3-2-4-attribute-declaration-validation-rules.md#sic-attrType))
      
    [[type definition namespace]](011-3-2-4-attribute-declaration-validation-rules.md#a-type_definition_namespace)
    ([Attribute Validated by Type](011-3-2-4-attribute-declaration-validation-rules.md#sic-attrType))
      
    [[type definition type]](011-3-2-4-attribute-declaration-validation-rules.md#a-type_definition_type)
    ([Attribute Validated by Type](011-3-2-4-attribute-declaration-validation-rules.md#sic-attrType))
      
    [[validation attempted]](011-3-2-4-attribute-declaration-validation-rules.md#a-validation_attempted)
    ([Assessment Outcome (Attribute)](011-3-2-4-attribute-declaration-validation-rules.md#sic-a-outcome))
      
    [[validation context]](011-3-2-4-attribute-declaration-validation-rules.md#a-validation_context)
    ([Assessment Outcome (Attribute)](011-3-2-4-attribute-declaration-validation-rules.md#sic-a-outcome))
      
    [[validity]](011-3-2-4-attribute-declaration-validation-rules.md#a-validity)
    ([Assessment Outcome (Attribute)](011-3-2-4-attribute-declaration-validation-rules.md#sic-a-outcome))

element information item properties
:   [[element declaration]](018-3-3-6-constraints-on-element-declaration-schema-components.md#e-declaration)
    ([Element Declaration](018-3-3-6-constraints-on-element-declaration-schema-components.md#sic-elt-decl))
      
    [[ID/IDREF table]](051-3-15-5-schema-information-set-contributions.md#e-ii_table)
    ([ID/IDREF Table](051-3-15-5-schema-information-set-contributions.md#sic-id))
      
    [[identity-constraint table]](042-3-11-5-identity-constraint-definition-information-set-contributions.md#e-id_constraint_table)
    ([Identity-constraint Table](042-3-11-5-identity-constraint-definition-information-set-contributions.md#sic-key))
      
    [[member type definition]](018-3-3-6-constraints-on-element-declaration-schema-components.md#e-member_type_definition)
    ([Element Validated by Type](018-3-3-6-constraints-on-element-declaration-schema-components.md#sic-eltType))
      
    [[member type definition anonymous]](018-3-3-6-constraints-on-element-declaration-schema-components.md#e-member_type_definition_anonymous)
    ([Element Validated by Type](018-3-3-6-constraints-on-element-declaration-schema-components.md#sic-eltType))
      
    [[member type definition name]](018-3-3-6-constraints-on-element-declaration-schema-components.md#e-member_type_definition_name)
    ([Element Validated by Type](018-3-3-6-constraints-on-element-declaration-schema-components.md#sic-eltType))
      
    [[member type definition namespace]](018-3-3-6-constraints-on-element-declaration-schema-components.md#e-member_type_definition_namespace)
    ([Element Validated by Type](018-3-3-6-constraints-on-element-declaration-schema-components.md#sic-eltType))
      
    [[nil]](018-3-3-6-constraints-on-element-declaration-schema-components.md#e-nil)
    ([Element Declaration](018-3-3-6-constraints-on-element-declaration-schema-components.md#sic-elt-decl))
      
    [[notation]](043-3-12-notation-declarations.md#e-notation)
    ([Validated with Notation](043-3-12-notation-declarations.md#sic-notation-used))
      
    [[notation public]](043-3-12-notation-declarations.md#e-notation_public)
    ([Validated with Notation](043-3-12-notation-declarations.md#sic-notation-used))
      
    [[notation system]](043-3-12-notation-declarations.md#e-notation_system)
    ([Validated with Notation](043-3-12-notation-declarations.md#sic-notation-used))
      
    [[schema default]](018-3-3-6-constraints-on-element-declaration-schema-components.md#e-schema_default)
    ([Element Validated by Type](018-3-3-6-constraints-on-element-declaration-schema-components.md#sic-eltType))
      
    [[schema error code]](018-3-3-6-constraints-on-element-declaration-schema-components.md#e-schema_error_code)
    ([Validation Failure (Element)](018-3-3-6-constraints-on-element-declaration-schema-components.md#sic-elt-error-code))
      
    [[schema information]](051-3-15-5-schema-information-set-contributions.md#e-schema_information)
    ([Schema Information](051-3-15-5-schema-information-set-contributions.md#sic-schema))
      
    [[schema normalized value]](018-3-3-6-constraints-on-element-declaration-schema-components.md#e-schema_normalized_value)
    ([Element Validated by Type](018-3-3-6-constraints-on-element-declaration-schema-components.md#sic-eltType))
      
    [[schema specified]](018-3-3-6-constraints-on-element-declaration-schema-components.md#e-schema_specified)
    ([Element Default Value](018-3-3-6-constraints-on-element-declaration-schema-components.md#sic-eltDefault))
      
