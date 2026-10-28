# Continuation 22

Source: [XML Schema Part 1: Structures](https://www.w3.org/TR/2004/REC-xmlschema-1-20041028/) · 2nd edition, 2004-10-28.

[Document index](../README.md) · [Topic guide](../../../../README.md) · [Publisher](https://www.w3.org/TR/2004/REC-xmlschema-1-20041028/).

HTML normalized to Markdown. Complex tables and mathematical markup retain HTML; illustrations are stored locally.

[Previous](021-3-4-2-xml-representation-of-complex-type-definitions.md) · [Next](023-continuation-23.md)

<a id="element-simpleContent..extension"></a>

<extension  
  **base** = [QName](https://www.w3.org/TR/2004/REC-xmlschema-2-20041028/datatypes.html#QName)  
  id = [ID](https://www.w3.org/TR/2004/REC-xmlschema-2-20041028/datatypes.html#ID)  
  *{any attributes with non-schema namespace . . .}*>  
*Content:* ([annotation](044-3-13-annotations.md#element-annotation)?, (([attribute](009-3-2-attribute-declarations.md#element-attribute) | [attributeGroup](030-3-5-attributeuses.md#element-attributeGroup))\*, [anyAttribute](022-continuation-22.md#element-anyAttribute)?))  
</extension>

<a id="element-simpleContent..attributeGroup"></a>

<attributeGroup  
  id = [ID](https://www.w3.org/TR/2004/REC-xmlschema-2-20041028/datatypes.html#ID)  
  **ref** = [QName](https://www.w3.org/TR/2004/REC-xmlschema-2-20041028/datatypes.html#QName)  
  *{any attributes with non-schema namespace . . .}*>  
*Content:* ([annotation](044-3-13-annotations.md#element-annotation)?)  
</attributeGroup>

<a id="element-anyAttribute"></a>

<anyAttribute  
  id = [ID](https://www.w3.org/TR/2004/REC-xmlschema-2-20041028/datatypes.html#ID)  
  namespace =
((##any | ##other) | List of
([anyURI](https://www.w3.org/TR/2004/REC-xmlschema-2-20041028/datatypes.html#anyURI) | (##targetNamespace | ##local))
)
 : ##any  
  processContents = (lax | skip | strict) : strict  
  *{any attributes with non-schema namespace . . .}*>  
*Content:* ([annotation](044-3-13-annotations.md#element-annotation)?)  
</anyAttribute>

