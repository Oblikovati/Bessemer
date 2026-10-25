# Unsigned archive version selection

The specification's clause 13.1 defines display versions as unsigned u32 values.
For example, `_DL_VERSION_-1/DLUpdateStamp` selects `4294967295`, enabling every
applicable later-version condition. Model markers use the same normalization;
drawing `VBLists` takes its version from `_MO_VERSION_`, independently of the
display marker. The library previously imposed an additional upper limit of
19000 on both complete archive readers. That limit did not follow the documented
version predicates.

Display-list archives now accept versions from 3196 through `u32::MAX`, excluding
the unresolved uncounted-window interval `0x1FAE`–`0x1FCE`. Drawing bucket archives
accept model versions from `0x27CC` through `u32::MAX`. Earlier unresolved layouts
remain unsupported. Decoding follows the documented predicates; this change does
not infer fields for future revisions of the format specification.

The [gate inventory](unsigned-gates.json) records 374 reference comparisons that
mention version variables or accessors across the six shared/drawing modules. It
binds the comparison counts and literals to source hashes. No captured version
literal exceeds 19000. Its generator fails if that changes, requiring a review
before reusing the current-layout bytes. This scan is supporting evidence, not a
claim that every possible object graph is covered.

The [owned display corpus](unsigned-versions.json) contains 85 complete roots:
17 profiles at 19000, 19001, `0x7FFFFFFF`, `0x80000000` and `0xFFFFFFFF`. Profiles
include both document kinds, populated scenes, 2D/3D buckets, shared fonts and
all twelve current-layout configuration variants, including external component
declarations. Independent reference traces provide 31,270 field expectations.
The reference's `(id, object)` part-configuration tuples receive named paths for
comparison; their objects and alias identities remain unchanged.

The [drawing corpus](../drawing-buckets/unsigned.json) adds 25 complete streams
at the same markers, with 695 independent drawing-field expectations. The
existing model, collection and backend-context tests apply unchanged. Both
corpora force complete reconstruction in two isolated Rust processes. Existing
lower-bound and unresolved-layout rejection tests remain in place.

Two complete part/assembly containers and one drawing container store signed
negative markers alongside numerically equal positive aliases. Document tests
resolve context, perform edits against independent expected bytes, reopen and
revert, preserving unrelated streams. The display file context stays 19000 while
the display version is `u32::MAX`; the drawing retains a display marker of 7000
while its model version is `u32::MAX`. Two-process document reconstruction also
checks the complete original files and resolved contexts.

Verification used Windows x64 and Rust 1.99.0 MSVC. This extends wire-level schema
coverage for #28 and #38; full model semantics, native application acceptance,
older unresolved layouts and complete documented-file parity remain separate
work. No version bump or release is included.

```text
python -B -X utf8 docs/features/display-lists/verify_unsigned_gates.py
python -B -X utf8 docs/features/display-lists/generate_unsigned_versions.py
python -B -X utf8 docs/features/drawing-buckets/generate_unsigned.py
cargo test --test display_unsigned_versions --test drawing_buckets
python -B -X utf8 docs/features/display-lists/verify_unsigned_versions.py target/pinned-1.99.0/debug/examples/rebuild_display_archives.exe
python -B -X utf8 docs/features/drawing-buckets/verify_unsigned.py target/pinned-1.99.0/debug/examples/rebuild_drawing_buckets.exe
python -B -X utf8 docs/features/display-lists/verify_earliest_component_documents.py target/pinned-1.99.0/debug/examples/rebuild_display_documents.exe unsigned
cargo xtask pr
```

The maintainer-side humanizer skill was unavailable; its prose pass remains for
review.
