# JSON resource captures

Three frozen cases cover source-relative paths, repeated reads, duplicate keys,
nested properties, typed formatting, arrays, empty collections, large integers,
and null content. Complete model and editable package captures record the output;
source and resource fingerprints keep the inputs identifiable.

These tests are not real-Word visual review. Resource lookup, failures, snapshots,
custom logical names, and CLI standard input are exercised in `tests/json_data.rs`.
