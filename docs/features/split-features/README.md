# Split features, items and native integer maps

Issue #54 implements clause 11.234 / C-276 for ordinary external-XML parts above
version `0x27CB`. The parent writes selected trim-data members directly,
interleaved with its body chooser, arrays, DWORD array, scalar words and integer
map. Constructed but unwritten storage is omitted. The entity-reference keeper
is a direct array, without a tag, presence word or archive-map slot. The tagged
`moSplitData_c` and `moSplitBodyIdKeeper_c` serializers are separately complete
empty bodies; they do not stand in for the parent's owned data.

The integer map retains its signed i32 count and every consumed pair in wire
order. Nonpositive counts consume nothing. Positive counts are bounded before
reading pairs. `PartResolvedFeaturesArchive::split_integer_maps()` returns owned,
immutable `SplitIntegerMap` views with the raw count and pairs plus a separate
ordered native result. Only nonnegative keys and values enter that result; the
last accepted duplicate replaces its earlier value. Rejected later duplicates
do not erase accepted values. Derivation does not change physical fields or
serialization. Types are `Send` and `Sync`; derivation uses operation-local
collections, without shared mutable state or locks.

Split items preserve counted names, typed visual pointers and full-width body
presence. Present bodies require an explicit kernel context and complete
compressed single-body decoding, followed by their direct attribute list.
Suppressed-body early-return variants stop explicitly. Two fixed transforms
retain 30 u32 words each. Their word-28 special kinds stop explicitly; the
compact restore grammar is not substituted. The C-247 fixed-transform reader
is reused with its earlier behavior preserved.

Independent fixtures cover 312 complete graphs across thirteen versions,
48,522 primitive observations and 27 owned streams. They exercise complete
profiles and directions, self-owned direct choosers, shared visuals, items and nonnull inherited empty keepers,
positive compressed bodies, empty and populated attributes, CString encodings,
ReadCount escapes, signed count extremes, duplicates and negative map pairs,
raw transform bits and three independently frozen edits. Source identities,
primitive partitions and derived maps are checked against stable source hashes.
The source's permissive expected-class argument is not a pointer-role oracle.

Private tests check every truncated prefix, appended bytes, old context,
minimum bodies, all 256 flag bytes in both fixed transforms, missing kernel
context, suppressed bodies, impossible counts, typed-pointer rejection,
malformed derived-map inputs and result isolation. Public tests check complete
document reconstruction, unchanged storage, parent and transform edits,
rejected edits and immutable map views after source mutation.

Run `cargo test --lib model_archives::split_feature_tests`,
`cargo test --test resolved_split_features` and `cargo xtask pr`. Independent
regeneration uses `python -B -X utf8 -m unittest discover -s docs/features/split-features`.

Two fresh native processes agree on all 906 outcomes and causes. The corpus
remains at 765 complete inputs and 141 explicit stops, with no lost success;
earlier unsupported classes still prevent reaching these split roots. No new
whole-file completion or native-prefix credit is claimed. The independent
pinned-source audit verifies all 765 accepted streams and four stream hashes
per input, covering 49,094,182 feature-stream bytes. Under the same snapshot,
the four original split-root spans reproduce 131,828 bytes, including seven
items' compressed and inflated hashes, six empty attribute lists, one populated
list and paired fixed-transform widths. These source-only native sentinels are
separate from Bessemer corpus completion. Results are retained in
`accepted-reference.json` and `native-sentinel-reference.json`.

Source/native bindings and executed checks are recorded beside this document.
Historical migrations, alternate body-handle contexts, suppressed-body return
boundaries, special transform kinds, raw option meanings, geometry validity,
SolidWorks application acceptance and broader all-schema parity remain open.
The humanizer skill is unavailable; prose review remains a maintainer task.
