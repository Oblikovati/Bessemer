# Drawing view-bucket archives

Work for #38 implements complete `Contents/VBLists` framing from edition 0.206,
clauses 13.9–13.11. The supported model archive interval is `0x27CC` through 19000.
Earlier layouts and versions above the verified shared-codec range return typed
unsupported errors. Drawing `Contents/DisplayLists` remains unresolved and is a
separate stream.

`DrawingBucketArchive::from_document` resolves normalized `_MO_VERSION_` markers
and reads the complete stream with a byte limit. It rejects missing or conflicting
model markers and does not substitute `_DL_VERSION_` markers. The byte API takes
an explicit `ArchiveVersion`. The container's version 4 is not the model version.

The codec retains one MFC map across top-level buckets, nested buckets, lights,
appearances and all shaded models. It preserves null pointers, aliases, direct
signed counts, unsigned model indices, both direct string-list groups and all
three trailing word-list groups. These direct list wrappers add no object slots.
Every nonzero shaded presence word selects the complete continuation; no opaque
tail is published. Part and assembly records use the existing shared serializers.

The drawing backend omits facet edge points at versions `0x2BE5` and later.
Before that boundary, the shared facet version rule still applies. This selector
is archive context, not a stored flag. Reading and writing use the same context;
owned construction rejects a facet with the other layout's shape. Standalone
part/assembly archives retain their full facet layout.

Published archives are immutable, owned, `Send` and `Sync`. Construction forces
encoding and complete reparsing, including path, type, ordering and object-map
checks. Edits are validated before publication; equivalent edits and authentic
reverts retain their original bytes. Failed construction does not change the
source. Typed failures identify `Contents/VBLists` and retain their failure class
and cause. Unresolved field meanings remain raw typed values. Successful framing
does not certify model geometry, reference resolution or native application
acceptance.

## Evidence

The [native report](native-verification.json) covers all 20 installed drawings,
including all 33 shaded models: 23 parts and 10 assemblies. Two isolated Rust
processes force exact reconstruction of all 8,215,597 decoded stream bytes and
match 1,383 independent drawing-field expectations. The document API also forces
replacement and reopening of all 20 native containers in two processes, retaining
every complete file byte. These expectations cover the
new drawing envelope, model headers and direct collections. Shared bucket,
appearance and tessellation fields retain their existing independent codec tests;
the drawing oracle does not claim additional semantic coverage for those fields.
Native sample bytes are read in place and are not redistributed. The manifest
hash is `080a7f20236e69e68c392329688c338d081afb7df907fb6264a4a8b72ef8a264`.

The [owned report](owned-verification.json) covers five populated/empty streams
and 34 complete roots across shared layout boundaries, with 683 independent
drawing-field expectations. The fixtures include populated narrow and UTF-16
lists, unsigned extremes, null and aliased buckets, nested self references,
negative nonzero presence words, both model kinds and drawing facets. The
independent edit changes a model reference and three trailing words.

Integration tests check forced bytes and fields, every truncation of the five
base streams, trailing bytes, malformed kinds/counts, reordered fields, limits,
incompatible backend context, source isolation, eight concurrent edits and
whole-document replace/reopen/revert with unrelated streams preserved. Verification
uses Windows x64, Rust 1.99.0 MSVC and the sibling Python reference. ThreadSanitizer
was not run on this platform; cross-platform CI and native application acceptance
remain integration work. No geometric or application acceptance is claimed.

```text
python -B -X utf8 docs/features/drawing-buckets/generate_vectors.py
python -B -X utf8 docs/features/drawing-buckets/range_wire.py
cargo test --test drawing_buckets
cargo build --example rebuild_drawing_buckets
python -B -X utf8 docs/features/drawing-buckets/verify_owned.py target/pinned-1.99.0/debug/examples/rebuild_drawing_buckets.exe
python -B -X utf8 docs/features/drawing-buckets/verify_corpus.py target/pinned-1.99.0/debug/examples/rebuild_drawing_buckets.exe
cargo xtask pr
```

This work remains on the existing `codex/display-lists` branch. It will join the
single complete feature PR after the remaining part/assembly schema work is
ready, with all required CI checks green before merging. No release or version
bump is included. The maintainer-side humanizer skill was unavailable; the prose
pass remains for review.
