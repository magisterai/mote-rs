# Continuation 24

Source: [XML Schema Part 1: Structures](https://www.w3.org/TR/2004/REC-xmlschema-1-20041028/) · 2nd edition, 2004-10-28.

[Document index](../README.md) · [Topic guide](../../../../README.md) · [Publisher](https://www.w3.org/TR/2004/REC-xmlschema-1-20041028/).

HTML normalized to Markdown. Complex tables and mathematical markup retain HTML; illustrations are stored locally.

[Previous](023-continuation-23.md) · [Next](025-continuation-25.md)

When the [<complexContent>](024-continuation-24.md#element-complexContent) alternative is chosen, the following
elements are relevant (as are the [<attributeGroup>](022-continuation-22.md#element-simpleContent..attributeGroup) and [<anyAttribute>](022-continuation-22.md#element-anyAttribute) elements, not repeated here), and the additional property mappings are as below. Note that either
[<restriction>](024-continuation-24.md#element-complexContent..restriction) or [<extension>](024-continuation-24.md#element-complexContent..extension) must be chosen as the
content of [<complexContent>](024-continuation-24.md#element-complexContent), but their content models are
different in this case from the case above when they occur as children of
[<simpleContent>](021-3-4-2-xml-representation-of-complex-type-definitions.md#element-simpleContent).

The property mappings below are *also* used in the case where
the third alternative (neither [<simpleContent>](021-3-4-2-xml-representation-of-complex-type-definitions.md#element-simpleContent) nor [<complexContent>](024-continuation-24.md#element-complexContent)) is chosen. This case is understood as shorthand for complex content restricting the [·ur-type definition·](004-2-1-overview-of-xml-schema.md#key-urType), and the details of the mappings should be modified as necessary.

<a id="element-complexContent"></a>

<complexContent  
  id = [ID](https://www.w3.org/TR/2004/REC-xmlschema-2-20041028/datatypes.html#ID)  
  mixed = [boolean](https://www.w3.org/TR/2004/REC-xmlschema-2-20041028/datatypes.html#boolean)  
  *{any attributes with non-schema namespace . . .}*>  
*Content:* ([annotation](044-3-13-annotations.md#element-annotation)?, ([restriction](024-continuation-24.md#element-complexContent..restriction) | [extension](024-continuation-24.md#element-complexContent..extension)))  
</complexContent>

<a id="element-complexContent..restriction"></a>

<restriction  
  **base** = [QName](https://www.w3.org/TR/2004/REC-xmlschema-2-20041028/datatypes.html#QName)  
  id = [ID](https://www.w3.org/TR/2004/REC-xmlschema-2-20041028/datatypes.html#ID)  
  *{any attributes with non-schema namespace . . .}*>  
*Content:* ([annotation](044-3-13-annotations.md#element-annotation)?, ([group](032-3-7-model-group-definitions.md#element-group) | [all](033-3-7-6-constraints-on-model-group-definition-schema-components.md#element-all) | [choice](033-3-7-6-constraints-on-model-group-definition-schema-components.md#element-choice) | [sequence](033-3-7-6-constraints-on-model-group-definition-schema-components.md#element-sequence))?, (([attribute](009-3-2-attribute-declarations.md#element-attribute) | [attributeGroup](030-3-5-attributeuses.md#element-attributeGroup))\*, [anyAttribute](022-continuation-22.md#element-anyAttribute)?))  
</restriction>

<a id="element-complexContent..extension"></a>

<extension  
  **base** = [QName](https://www.w3.org/TR/2004/REC-xmlschema-2-20041028/datatypes.html#QName)  
  id = [ID](https://www.w3.org/TR/2004/REC-xmlschema-2-20041028/datatypes.html#ID)  
  *{any attributes with non-schema namespace . . .}*>  
*Content:* ([annotation](044-3-13-annotations.md#element-annotation)?, (([group](032-3-7-model-group-definitions.md#element-group) | [all](033-3-7-6-constraints-on-model-group-definition-schema-components.md#element-all) | [choice](033-3-7-6-constraints-on-model-group-definition-schema-components.md#element-choice) | [sequence](033-3-7-6-constraints-on-model-group-definition-schema-components.md#element-sequence))?, (([attribute](009-3-2-attribute-declarations.md#element-attribute) | [attributeGroup](030-3-5-attributeuses.md#element-attributeGroup))\*, [anyAttribute](022-continuation-22.md#element-anyAttribute)?)))  
</extension>

