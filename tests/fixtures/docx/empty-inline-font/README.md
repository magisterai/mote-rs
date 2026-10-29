# Empty inline font capture

An explicit empty inline font is retained in the model but supplies no typeface
override in DOCX. It must not erase an enclosing direct font, including when a
character style is also applied. Script mappings and following sibling formatting
remain intact. Source, evaluated model and all package parts are frozen.

Real Microsoft Word visual review remains unreviewed.
