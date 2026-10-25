# Display versions 11000–11999

This checkpoint fills the remaining 11000–11999 interval in the complete part and
assembly display-list codec, using explicit native file context 19000. It advances
#28 and resolves the implementation gap tracked by #36. Whole-schema parity is
still incomplete; no CAD application or geometric-validity claim follows from
archive reconstruction.

Part configurations are absent through `0x2B5E`. Above that value and before
`0x2B67`, each configuration contains inline `uoTempPartTessData`, without an MFC
object tag. From `0x2B67`, it contains a pointer. The decoder now scopes inline
fields without allocating a map slot or inventing pointer metadata. A completed
empty inline tessellation remains distinct from a null pointer. Version 11000
itself skips part configurations, so #36 was a gap in the interval being added,
not a demonstrated regression in that existing fixture.

Configuration view matrices appear after `0x2B61` for parts and `0x2B6F` for
assemblies. Annotation visibility appears after `0x2B7F` for parts and `0x2B77`
for assemblies. Assembly versions `0x2B78` and `0x2B79` also store a leading word,
which Bessemer retains as `unknown_legacy`. The independent reader discards that
word; exact reconstruction checks its preservation. Later configuration tails
remain independently gated at `0x2FA5`.

Part tessellation adds configuration ID/name after `0x2B96`, the graphics-manager
presence word after `0x2B9D`, and bounds after `0x2BA3`. Viewer components add the
graphics-manager presence word after `0x2B88` and seed-reference text after
`0x2B9B`. Face records and inline facets add face strings after `0x2BCF`; full
facets add edge-point records after `0x2BD0`. Other collection layouts are reused.
The owned pair of one-entry facet edge-point collections costs two entries only
when present. Scalar omissions do not consume collection entries.

The new evidence contains 68 complete shaded roots across the version boundaries,
27 complete roots with populated configurations, and 68 unshaded bucket roots in
both coordinate scopes. The independent reader consumes every stream to EOF and
supplies 29,147 shaded/configuration expectations plus 14,421 bucket expectations:
43,568 new field comparisons. Manifests record raw hashes, reader hashes, exact
aggregate-entry budgets and loaded-object counts. The inline configuration fixture
has an empty body collection with nonzero state fields; its independent edit
changes the exploded-view flag to 23 without changing the wire layout.

The shared earlier-range suites now cover 155 shaded roots and 122 bucket roots,
plus nine seven-kind reference-geometry roots. They compare forced bytes and field
oracles, enforce exact resource limits and failing lower bounds, and pin the new
presence boundaries. Additional tests reject every prefix of a populated inline
configuration root, check malformed counts and source isolation, compare eight
concurrent edits with an independent fixture, reopen that edit in a complete
container, and restore the authentic original container bytes.

The process verifier regenerates all 163 new roots, checks their frozen hashes
and compares forced bytes and field traces from two Rust processes. The earlier
123-fixture and 134-fixture process gates retain the same input, output and trace
hashes. Shared fixture orchestration was extracted separately; existing fixtures
were not regenerated to accommodate the new layouts.

```powershell
cargo build --example rebuild_display_archives
python -B -X utf8 docs/features/display-lists/verify_precollector_processes.py target/pinned-1.99.0/debug/examples/rebuild_display_archives.exe
cargo xtask pr
```

The native corpus gate passed all 1,086 streams and 1,849,563 independent field
comparisons, with identical reconstruction in two processes. `cargo xtask pr`
passed on Windows x64 with Rust 1.99.0 MSVC, including all 176 doctests. The final
expanded earlier-root suite and all-target Clippy check also passed.
Earlier intervals, other file-version contexts, unresolved object bodies and
whole-document context/validation remain open. The maintainer-side humanizer
skill is unavailable; its prose pass remains for review.
