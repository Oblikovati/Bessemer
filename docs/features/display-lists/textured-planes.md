# Textured-plane records

Issue #28 adds the documented textured-plane record framing for admitted part and
assembly contexts: a nonnegative signed count followed by each typed
`moSketchBitmap_c` pointer and compact transform. Each record needs at least
36 bytes (two for a null pointer and 34 for the smallest transform). Planes share
the existing entry budget and archive-wide pointer map.

The approved reference has no registered `moSketchBitmap_c` body reader. This
checkpoint supports null pointers with complete transforms; it does not claim
non-null bitmap support. A newly declared bitmap body returns `Unsupported`, and
other classes are rejected for both declarations and aliases. No opaque remainder
or guessed bitmap layout is accepted. Drawing-specific fields are outside the
currently admitted document contexts.

Fifteen complete owned roots cover both document kinds at versions 7000, 9000,
10000, 11000 and 19000, a part at 18000, an independent translation edit, a minimal
36-byte record, an empty collection and an exhaustive 1,024-plane part. That last
fixture crosses all 256 flag bytes with presence bytes 0, 1, 2 and 255. It includes
all perspective variants, raw Matrix5 discarded words, signed zeros, infinities
and both NaN kinds. The independent sibling parser consumes each root to EOF and
supplies 15,261 transform field expectations. Raw framing that the reference drops
is also preserved by forced byte reconstruction and direct typed assertions.

Each plane adds one entry and no object identities for a null pointer. The
manifest records the source budgets, count offsets, fixture hashes and reader
hashes. Exact-budget tests include the nested facets and other collections in the
complete source roots, rather than testing plane fragments in isolation.

Seven integration tests compare independent fields and exact bytes, cover all
flag/presence pairs, retain the declared pointer domain, reject new unresolved
bitmap bodies, enforce exact budgets, reject negative/impossible counts and every
truncated minimal root, and check trailing bytes. A translation edit matches
independently encoded bytes and survives container reopen/revert while other
streams remain unchanged. Source ownership and eight concurrent edits use the
same independent golden result.

The process verifier reuses the frozen-case loader and two-process runner from
the existing decal/configuration verifiers. It checks regenerated fixtures
against their frozen hashes before running Rust. The result is recorded in
`textured-plane-verification.json`; transform field oracles are checked separately
by integration tests. These are archive checks, not CAD application or geometric
acceptance. The existing 1,086-stream native corpus gate remains required.

```powershell
cargo build --example rebuild_display_archives
python -B -X utf8 docs/features/display-lists/verify_textured_plane_processes.py target/pinned-1.99.0/debug/examples/rebuild_display_archives.exe
```

Non-null bitmap object bodies, remaining display-version/caller contexts and
whole-document context/validation remain required work toward complete parity.

The maintainer-side humanizer skill is unavailable; its prose pass remains for review.
