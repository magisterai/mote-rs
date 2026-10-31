# Math value definitions

[Inventory guide](README.md)

These are complete schema value domains for the selected properties.
Property rows record Mote support and supported subsets separately.
Identical Strict and Transitional definitions are combined.
See also [prose-defined extensions](prose-extensions.md).

## Types

- [m:ST_BreakBin](#m-st_breakbin)
- [m:ST_BreakBinSub](#m-st_breakbinsub)
- [m:ST_Char](#m-st_char)
- [m:ST_FType](#m-st_ftype)
- [m:ST_Integer2](#m-st_integer2)
- [m:ST_Integer255](#m-st_integer255)
- [m:ST_Jc](#m-st_jc)
- [m:ST_LimLoc](#m-st_limloc)
- [m:ST_Script](#m-st_script)
- [m:ST_Shp](#m-st_shp)
- [m:ST_SpacingRule](#m-st_spacingrule)
- [m:ST_Style](#m-st_style)
- [m:ST_TopBot](#m-st_topbot)
- [m:ST_UnSignedInteger](#m-st_unsignedinteger)

<a id="m-st_breakbin"></a>

## m:ST_BreakBin

[Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/shared-math.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/shared-math.xsd):

- restriction: base: `xsd:string`
- enumeration: value: `before`
- enumeration: value: `after`
- enumeration: value: `repeat`

<a id="m-st_breakbinsub"></a>

## m:ST_BreakBinSub

[Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/shared-math.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/shared-math.xsd):

- restriction: base: `xsd:string`
- enumeration: value: `--`
- enumeration: value: `-+`
- enumeration: value: `+-`

<a id="m-st_char"></a>

## m:ST_Char

[Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/shared-math.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/shared-math.xsd):

- restriction: base: `xsd:string`
- maxLength: value: `1`

<a id="m-st_ftype"></a>

## m:ST_FType

[Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/shared-math.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/shared-math.xsd):

- restriction: base: `xsd:string`
- enumeration: value: `bar`
- enumeration: value: `skw`
- enumeration: value: `lin`
- enumeration: value: `noBar`

<a id="m-st_integer2"></a>

## m:ST_Integer2

[Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/shared-math.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/shared-math.xsd):

- restriction: base: `xsd:integer`
- minInclusive: value: `-2`
- maxInclusive: value: `2`

<a id="m-st_integer255"></a>

## m:ST_Integer255

[Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/shared-math.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/shared-math.xsd):

- restriction: base: `xsd:integer`
- minInclusive: value: `1`
- maxInclusive: value: `255`

<a id="m-st_jc"></a>

## m:ST_Jc

[Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/shared-math.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/shared-math.xsd):

- restriction: base: `xsd:string`
- enumeration: value: `left`
- enumeration: value: `right`
- enumeration: value: `center`
- enumeration: value: `centerGroup`

<a id="m-st_limloc"></a>

## m:ST_LimLoc

[Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/shared-math.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/shared-math.xsd):

- restriction: base: `xsd:string`
- enumeration: value: `undOvr`
- enumeration: value: `subSup`

<a id="m-st_script"></a>

## m:ST_Script

[Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/shared-math.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/shared-math.xsd):

- restriction: base: `xsd:string`
- enumeration: value: `roman`
- enumeration: value: `script`
- enumeration: value: `fraktur`
- enumeration: value: `double-struck`
- enumeration: value: `sans-serif`
- enumeration: value: `monospace`

<a id="m-st_shp"></a>

## m:ST_Shp

[Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/shared-math.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/shared-math.xsd):

- restriction: base: `xsd:string`
- enumeration: value: `centered`
- enumeration: value: `match`

<a id="m-st_spacingrule"></a>

## m:ST_SpacingRule

[Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/shared-math.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/shared-math.xsd):

- restriction: base: `xsd:integer`
- minInclusive: value: `0`
- maxInclusive: value: `4`

<a id="m-st_style"></a>

## m:ST_Style

[Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/shared-math.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/shared-math.xsd):

- restriction: base: `xsd:string`
- enumeration: value: `p`
- enumeration: value: `b`
- enumeration: value: `i`
- enumeration: value: `bi`

<a id="m-st_topbot"></a>

## m:ST_TopBot

[Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/shared-math.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/shared-math.xsd):

- restriction: base: `xsd:string`
- enumeration: value: `top`
- enumeration: value: `bot`

<a id="m-st_unsignedinteger"></a>

## m:ST_UnSignedInteger

[Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/shared-math.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/shared-math.xsd):

- restriction: base: `xsd:unsignedInt`
