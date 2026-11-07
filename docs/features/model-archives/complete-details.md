# Complete detail-definition records

This historical checkpoint is followed by the
[complete owned part operation](owned-part-operation.md), which reaches full
configuration EOF and verifies the name edit, document reopen and exact revert.

The private codec now composes the complete supported ordinary `moDetailDefs_c`
body. It reads the prefix, dimension, annotation, table and view-label defaults,
then the surface-finish/note, weld/tolerance and center/datum/SWIFT helpers,
followed by print widths and the tail. Only this complete body is registered for
tagged `moDetailDefs_c` objects; its constructor-owned direct instances add no
MFC slots. The current schema snapshot records edition 0.313, clauses
11.103–11.109, alongside the earlier prefix/dimension snapshots.

The owned body occupies bytes 3003..20501 of the configuration: 17,498 bytes
reconstruct exactly from typed fields, and all 2,910 independently captured
scalar values match. The native body needs the inherited object map. Four
schema-built direct/tagged cases also reach exact EOF, reconstruct exactly and
match 10,980 scalar values. Their oldest supported and modern versions exercise
composition across different grids, optional fields and nested object identities.
Every shortened case prefix and every trailing-byte mutation fails complete
publication. Owned fields survive source overwrite and reconstruct identically
on four concurrent workers.

| Additional record suite | Cases | Expected scalar values |
|---|---:|---:|
| Annotation, table and label defaults | 33 | 10,188 |
| Surface-finish and note helpers | 21 | 1,827 |
| Weld, geometric tolerance, datum and tolerance helpers | 81 | 6,093 |
| Center/datum/SWIFT helpers and detail tail | 30 | 4,484 |

Each suite checks exact reconstruction, all truncated prefixes and trailing
bytes. Version gates, noncanonical presence values, signed words, duplicate map
keys, object cycles, tolerance aliases, datum variants and compatibility fields
retain their stored representations. Note maps use a checked native 64-bit
count, rather than ReadCount or i32. Negative/impossible nested counts and
oversized note counts fail before unbounded allocation. No native font, style,
tolerance or numeric normalization replaces the archived values.

The SWIFT table keeps all 159 entries, including three distinct writes to native
member +220. Typed field indices preserve their order without collapsing equal
member labels. The reference's retained load and store traces were compared
separately with the Rust table; both match every entry. Their binary/evidence
hashes and the table extraction are recorded in `swift-layout-provenance.json`.
The ordinal-valued SWIFT cases use that reference layout, so they are not an
independent field-order oracle. Native byte reconstruction and the separate
load/store evidence provide the additional checks. SWIFT scalar meanings remain
unknown; no semantic or geometry-validation credit is inferred.

Bug #45 corrected the new composite-oracle projection. Copying each dictionary
had turned a tolerance alias into a second apparent serialized body. The
projection now uses an operation-local identity memo, preserving aliases and
cycles while removing constructor-only labels. Six independent Python tests
cover identity, cycles, metadata projection and isolation. Correcting this new
oracle removed 84 falsely duplicated scalar expectations; all original wire
records and the native 2,910-value oracle remained unchanged. No existing frozen
baseline was updated. `oracle-alias-regression.json` records the reason, hashes
and before/after counts; the issue remains open until integration with #44.

Fresh reference processes reproduced the five new suites and complete native
body. Their manifests bind reference and generator hashes; the complete-body
manifest also binds the projection code. The verification report records the
executed gates and exact Rust sources.

The composed configuration reader now reaches byte 20,501 with one builder and
one MFC map. It still has 3,527 unread bytes and rejects full-root publication.
Remaining shared parameters, environment/view/array/layer/reference/model/part
tails, complete derived-length construction and edit history, document isolation,
percent-unit assignability, corpus acceptance, native application checks and the
public model API remain open under #44. This checkpoint does not complete model
or whole-document interoperability.

No version, release, extra branch or PR is included. The humanizer skill is
unavailable; the prose pass remains for maintainer review.
