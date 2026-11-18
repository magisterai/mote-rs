# JSON data

[Specification](README.md) · [Checklist](../checklists/json.md)

The `json(path)` helper loads a JSON resource and returns a typed value. It takes
one positional string path, with no named arguments or body. It works in static
expressions without enabling scripting.

JSON objects become property objects, arrays become arrays, strings remain
strings, booleans remain booleans, numbers become finite numeric values, and
`null` becomes nil. Empty JSON objects and arrays stay distinct. Duplicate object
keys keep the last value. Numbers use the same double-precision representation as
other numeric values; sufficiently large integers can lose precision.

Suppose `invoice.json` beside the source contains:

```json
{"client":{"name":"Acme"},"total":1234.5,"items":[{"name":"Support"}]}
```

The following source loads it, selects a nested property and an array member, and
uses those values as ordinary content:

```mote
#let(invoice=json("invoice.json"), first=invoice.items.0.name)
#heading[#invoice.client.name]
#paragraph[#first: #invoice.total]
```

Relative paths are resolved against the source containing the request. A source
read from standard input uses the command's working directory. Absolute paths
remain absolute. Resource lookup goes through the selected resolver, so library
callers can supply files, in-memory data, or resources with logical URI names.
The filesystem resolver does not fetch network resources. A custom resolver's
failure is final; evaluation does not retry it through the filesystem.

The filesystem resolver of the library, `FileAssetResolver::new`, is confined
to a root directory. A path is accepted only if, after its `..` segments and
symbolic links are resolved, it lies within the root; an absolute path is
accepted on the same condition. A confined resolver reads regular files alone
and refuses a file larger than 64 MiB, a limit that
`FileAssetResolver::with_maximum_size` replaces. A refused path is an error at
the expression that requested it, or, for an image, the warning of a missing
asset. The rule holds for every resource a source names: JSON and CSV data,
imported sources, images, and font files. An import path is sought through the
same resolver, and so lies within the root.

The command line trusts its source and is not confined: a path is read wherever
it leads, as `FileAssetResolver::unconfined` provides. The functions
`docx::emit` and `pptx::emit`, which take no resolver, read through a resolver
confined to the working directory of the process.

Repeated requests use a snapshot within one compilation. Two requests that
resolve to the same logical name must provide identical bytes. Invalid JSON,
missing resources, and conflicting bytes fail evaluation with the source
expression's location; JSON decoding errors also identify the data resource.

Loaded properties retain their ordinary types. A JSON string such as `"12pt"`
remains a string until an accepting directive normalizes it. Loading JSON does
not bypass attribute validation or imply a rendering effect for arbitrary keys.
Direct `#json("invoice.json")` content uses the value's text representation;
selecting individual fields is generally clearer. A top-level JSON `null` inserts
no content.
