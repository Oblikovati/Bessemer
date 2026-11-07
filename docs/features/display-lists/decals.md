# Decal records

Issue #28 implements complete decal bodies in body/component lists and part data
for the currently admitted display contexts. Part data uses a direct list before
`0x239D` and a keyed list map afterward. Both collection counts are unsigned
32-bit words. Duplicate keys, empty lists and record order remain intact.

A decal contains its raw byte, direction/center/width floats, mask filename,
mask type and inversion word, unsigned color mask, texture and material bodies,
face associations and versioned trailing settings. The new reader reuses the
existing texture and material codecs. It does not resolve mask paths or load
external images. Float bits, Unicode, embedded NULs, raw flags and signed states
are preserved. Mirror offsets are stored only above version `0x1BBE`.
The [earliest light layouts](earliest-lights.md) also cover the older underlying
appearance and original decal-ID word gates.

Unsigned list, map and color counts are checked against physical bytes before
conversion and budget charging. Face counts must be nonnegative and physically
bounded. Every map entry, decal, color-mask element, face association and nested
material collection consumes the existing shared entry budget. Direct decal
records do not allocate MFC object identities or reset the global object map.

Fourteen complete owned roots cover versions 7000, 9000, 10000, 11000 and 19000
for both document kinds, plus an 18000 part and modern edit/empty variants. They
populate body lists, assembly component lists, old part lists and keyed part
maps, including duplicate map keys with an empty second list. The approved
sibling reader consumes every root to EOF and supplies 7,292 independent scalar
expectations. Material bytes come from the existing reviewed owned root for each
version; the manifest records their hashes separately. The decal framing and
primitive values are independently assembled in `decal_wire.py`.

Each populated list has two decals; each populated map has two keys and a
single populated two-decal value. Each populated decal adds two mask entries and
two face associations. The ledger therefore adds ten entries per populated
collection and two more for each keyed map. Empty mask/face variants add only
the two decals and map keys; empty-list/map variants keep the established source
budget. The manifest and exact-budget integration test pin these totals.

Six integration tests compare all independent fields and force exact bytes,
check raw flags, palette/face IDs, version omission and duplicate keys, exercise
an independently encoded mask edit with container reopen/revert and other
streams preserved, and enforce exact byte/field/object/entry budgets. They also
reject impossible unsigned counts, negative face counts, every truncated empty-
collection root and trailing bytes, and verify owned source isolation plus eight
concurrent edits against the independent golden result.

`verify_decal_processes.py` regenerates the frozen cases and uses the existing
process verifier to force all 14 roots through two isolated Rust processes.
The resulting byte and trace hashes are recorded in `decal-verification.json`.
The 1,086-stream native corpus gate remains separate and mandatory. These are
archive checks; CAD application and geometric validity are not claimed.

```powershell
cargo build --example rebuild_display_archives
python -B -X utf8 docs/features/display-lists/verify_decal_processes.py target/pinned-1.99.0/debug/examples/rebuild_display_archives.exe
```

Textured-plane bitmap object bodies remain unresolved in the approved reference.
Earlier display contexts, whole-document context and cross-stream validation
remain required work toward complete schema parity.

The maintainer-side humanizer skill is unavailable; its prose pass remains for review.
