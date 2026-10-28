# G DTD for Schemas (non-normative)

Source: [XML Schema Part 1: Structures](https://www.w3.org/TR/2004/REC-xmlschema-1-20041028/) · 2nd edition, 2004-10-28.

[Document index](../README.md) · [Topic guide](../../../../README.md) · [Publisher](https://www.w3.org/TR/2004/REC-xmlschema-1-20041028/).

HTML normalized to Markdown. Complex tables and mathematical markup retain HTML; illustrations are stored locally.

[Previous](064-continuation-64.md) · [Next](066-continuation-66.md)

<a id="nonnormative-schemaDTD"></a>

## G DTD for Schemas (non-normative)

The DTD for schema documents is given below. Note there is *no*
implication here that `schema` must be the root element of a
document.

Although this DTD is non-normative, any XML document which is
not valid per this DTD, given redefinitions in its internal subset of the
'p' and 's' parameter entities below appropriate to its namespace
declaration of the XML Schema namespace, is almost certainly not
a valid schema document, with the exception of documents with multiple namespace
prefixes for the XML Schema namespace itself. Accordingly
authoring XML Schema documents using this DTD and DTD-based authoring tools, and
specifying it as the DOCTYPE of documents intended to be XML Schema
documents and validating them with a validating XML parser, are
sensible development strategies which users are encouraged to adopt
until XML Schema-based authoring tools and validators are more widely available.



