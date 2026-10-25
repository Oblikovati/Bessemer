# Display versions 9000–9999

This checkpoint fills the 9000–9999 interval in the complete part and assembly
display-list codec, with explicit file context 19000. It advances #28. The
production readers already contained the individual layouts; the context gate
now admits the interval after owned complete-root and boundary verification.
Full documented-schema parity remains incomplete.

Table display items add `always_2d` after `0x238C`. Part decals use a list before
`0x239D` and a keyed map from that value. Bucket polygons add the raw
`section_cut_new` word from `0x23B4`. Scene concepts add tooltip text after
`0x23CD`. Bucket component maps use unsigned 16-bit keys through `0x23D7` and
signed 32-bit keys afterward. The owned duplicate key is 65529 in the earlier
layout and -7 in the later layout; the codec retains their distinct wire types.
Viewer components add grouped-folder fields after `0x23E4`, and EDW annotation
records add `unknown_s3` after `0x23FE`. Appearances add the word `unknown_7c`
after `0x2419`; this does not change the earlier default-appearance field.

The assembly configuration footer is absent before `0x2404`. A populated record
starts with its configuration ID and shaded-data array; its visual-property
pointer appears only after `0x240E`. Earlier records do not contain a null tag in
that position. Part configuration footers remain absent throughout this interval.
Seven complete roots pin the early assembly configuration layout and retain its
empty shaded-data array without inventing missing pointer fields.

The new evidence contains 50 complete shaded boundary/edit roots, seven roots
with populated assembly configurations, 50 unshaded bucket roots in both
coordinate scopes, and four populated decal roots. Independent expectations total
17,034 shaded/configuration fields, 8,992 bucket fields and 2,144 decal fields:
28,170 comparisons across 111 fixtures. Each manifest records wire hashes,
reference-reader hashes, exact collection budgets and loaded-object counts.

The decal roots exercise the list/map boundary for both document kinds. Both
sides contain populated body/part/component lists; the mapped part layout also
retains duplicate keys and an empty second value. Per-decal palette and face
collections reuse the existing tested codec. The additional map entries cost two
aggregate entries, while a changed key width or omitted scalar costs none.
Configurations add two entries for their name and record; their null pointers
and empty arrays add no loaded objects.

The shared suites now compare 274 shaded roots and 230 bucket roots, plus the
nine seven-kind geometry roots. The decal suite checks 18 complete roots.
Assertions pin field omission, duplicate keys, unsigned/signed key types and the
early configuration pointer boundary. Changing only the context to reinterpret
a word-key bucket as a 32-bit-key layout fails construction. Exact budgets and
failing lower bounds remain enforced. Every prefix of the 9000 part and assembly
roots is rejected; independent ambient edits reconstruct and revert exactly.
Existing document, source-isolation and concurrent-edit checks remain in place.

The process verifier regenerates all 111 new fixtures, verifies frozen hashes,
and compares forced bytes and traces from two Rust processes. The earlier
120-, 163-, 123-, 134- and 14-fixture process gates retain the same input, output
and trace hashes. These checks establish archive wire behavior, not native CAD
application acceptance or geometric validity.

```powershell
cargo build --example rebuild_display_archives
python -B -X utf8 docs/features/display-lists/verify_grouped_components.py target/pinned-1.99.0/debug/examples/rebuild_display_archives.exe
cargo xtask pr
```

The native corpus gate passed all 1,086 streams and 1,849,563 independent field
comparisons with identical reconstruction in two processes. `cargo xtask pr`
passed on Windows x64 with Rust 1.99.0 MSVC, including all 176 doctests.
Earlier intervals, other file-version contexts, unresolved object bodies,
whole-document context/validation and remaining schemas are still open. The
maintainer-side humanizer skill is unavailable; its prose pass remains for review.
