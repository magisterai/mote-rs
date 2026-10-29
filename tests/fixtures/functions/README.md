# Function captures

These sources exercise positional/named arguments, defaults expressed in Lua,
content returns with formatting and block boundaries, lexical snapshots,
retained aliases, scalar/typed/array returns, and nested Lua syntax.

`captured/<case>/model.json` and the complete extracted DOCX package are frozen
comparison inputs. The source and native PNG hashes are recorded in each
`capture.json`. Package tests ignore only generation timestamps and XML prefix
or attribute order differences; content and package membership must agree.

Native PDF comparisons were pixel-exact across seven pages at 144 DPI. Native
PNG tests allow a maximum channel difference of 56 and a mean foreground
difference of 12 for rasterizer antialiasing. The `scanner` case has a separate
104/15 bound: eleven pixels around its final `7` exceed the usual maximum, with
no PDF difference or displaced glyph. Every generated PDF page was
visually inspected. Microsoft Word review remains unreviewed.

`empty` intentionally retains the empty-body `{}` conversion. `lexical` exposes
the current nested-call global environment behavior. These cases must not be
rewritten to conceal those limits.
