# Native block captures

`requests.json` contains 38 independent source cases; `expected.json` freezes
captured text geometry and ordered page painting operations. Comparisons use an
absolute tolerance of 1e-9 points. Core font names and variants are retained;
embedded font assets have no core font name in this projection.

Coverage includes nested bullet and ordered lists, Roman and repeating-letter
counters, continued and restarted identities, tasks that do not consume numbers,
marker-only empty items, item geometry and paragraph-property precedence, numbered
heading levels and tab advances, quote flattening, paragraph fills and spacing
bridges, borders, inline decorations, and pagination. The longer `painted-blocks`
case also has a standalone source in `../native-output/blocks.mote` for visual
review. Its declared regular face and core fallback bold face exercise both paths.

The expected data is independent of the candidate planner. The candidate capture
example reads requests only. See `capture.json` for file hashes. These fixtures
record native behavior; they do not constitute Word-rendered review evidence.
