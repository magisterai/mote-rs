# Controls, fields, comments and metadata

`checkbox` is standalone inline content with no body or positional arguments.
Its optional `checked` boolean defaults to false. `kind` selects
`content_control` (the default) or `legacy_form_field`. Optional `name` and
`title` accept strings or numbers and normalize to strings. The shared
`properties` object supplies overrides.

`field` is standalone inline content with one required positional or named
string `name`. Optional string `code` supplies the field instruction; when it is
empty or absent, the uppercase name supplies the instruction. The name supplies
cached display text, falling back to the code when the name is empty. A field
has no body. Field evaluation and live values depend on the output application.

`comment` requires one content body, which can contain inline or block content.
It accepts optional string `author` and `initials`, string or number `id`, and
an ISO date string or typed `date`. An absent or empty author defaults to `Mote`
in the annotation story. The authored `id` remains metadata; output allocates
its own comment identities. Its body remains visible in the document and also
supplies the annotation text. Shared `properties` overrides apply.

`metadata` declares document-wide named values without a body or positional
arguments. Repeated declarations replace matching names and preserve other
values. `title` and `author` identify the document; additional values such as
`subject`, `keywords`, `description`, `category`, and `company` remain available
in the model. Retention does not guarantee an output-format mapping.

```mote
#metadata(title="Review",author="Ada")
#paragraph[
Approved #checkbox(checked=true,name="approval",title="Approval").
Page #field("PAGE").
#comment(author="Ada",initials="AL",date=date("2024-02-29"))[Ready for review.]
]
```

Lua `document.checkbox`, `document.field`, and `document.comment` use the same
contracts and return elements; `document.metadata` declares values and returns
no content. As with other constructors, a leading property table supplies named
attributes.

DOCX preserves editable checkboxes, field instructions and comments. Comments in
running or annotation stories flatten with a warning; a legacy checkbox title
has no output effect. Native document output uses `[x]` and `[ ]` for checkboxes,
omits general fields, and retains comment body text. Presentation text retains
comment bodies and omits standalone controls and general fields. These fallbacks
warn; they do not imply interactive controls or annotation placement.

DOCX maps metadata `title` and `author`; PPTX maps `title`. Other metadata is
retained without package or native rendering effects.
