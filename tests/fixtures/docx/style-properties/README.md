# Declared style properties reference

This capture checks paragraph and run property ordering in declared styles,
including shading, borders, tabs, spacing, indentation, pagination flags,
decorations, fractional sizes, and nested values overriding flat values. It also
checks Unicode style names, sanitized identifier collisions, and an explicitly
empty code-style reference selecting the default Code style.

The captured flat `strike` and `double_strike` metadata produces two unsupported
property warnings even though the captured XML contains strike properties. Those
warnings remain recorded in `capture.json`. The emitted raw string values follow
scalar output coercion: only trimmed lowercase `true` enables a raw boolean.
Use the normalized fields inside `run` for intentional style formatting.

[Structural tests](../../../docx.rs) retain the exact emitted values and property
order. Real Word layout review remains **unreviewed**.
