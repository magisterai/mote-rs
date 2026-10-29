# Bullet lists reference

Covers Markdown and explicit lists, nesting, empty containers/items, base and item
levels, list geometry, item geometry, tight spacing and explicit spacing. Word
numbering definitions contain nine levels and are shared by equivalent geometry.
An item left indent is absolute; a list left indent is the base for level offsets.

Outside table cells, paragraph children flatten into runs in one item paragraph;
no paragraph separator is inserted. Nested lists follow the parent text, including
text authored after the nested list. This capture preserves that ordering. Child
paragraph formatting and other child blocks remain unsupported by this writer.

The empty list emits no content or numbering definition. The capture reported an
empty-list warning. The model and every package part are compared; Word rendering
and layout review remain **unreviewed**.
