# Clause 13 acceptance review

This review covers #28's part and assembly display archives and the #38 drawing
VBLists implementation carried on the same branch. It does not declare complete
SolidWorks file-schema parity. Part, assembly and drawing model archives and other
documented stream families remain work under #2.

The local implementation and corpus requirements below have evidence. Integration
requires all five CI checks on the final PR revision. Native application opening
and geometric validity are separate acceptance scopes and are not claimed here.

## Schema and caller scope

The current captured specification is
[edition 0.273](specification-13-edition-0.273.md). Its part/assembly layout text
in 13.2–13.8 is unchanged from the retained 0.128 snapshot; the newer snapshot adds
drawing VBLists clauses 13.9–13.11 and updates the overview. The
[snapshot record](current-schema-snapshot.json) binds the captured files to hashes.

All six display-list reference modules are bound in
[the executed audit](version-predicate-audit.json), along with archive primitives,
object-map support, runtime sources, adapters and the executable. The audit fails
if its source or executable inputs differ at the beginning and end of replay.

The audit checks all 27 registered reference classes against the Rust dispatcher
and requires owned full-root witnesses for each. This includes aliases such as
`moLightFeature_c` and `uoTempSubAssemblySHDData_c`, plus generic component-property,
graphics-reference and PW scene objects. A missing decoder, unknown class or absent
witness fails the check.

There are 391 recognized version comparisons: 362 observed both outcomes, and
29 have explicit caller or negative-case reviews. The
[review verification](predicate-review-verification.json) binds each remaining
comparison to current evidence. It rejects stale hashes, changed outcomes,
duplicate reviews, missing reviews and delegated coverage that lacks either
outcome. The classifications are reviewed call-path arguments, not a substitute
for field tests or a proof of geometric correctness. The generic PW caller found
during this review was fixed and given independent full-root fixtures (#41).

## Requirement evidence

| Requirement from #28 | Current evidence |
|---|---|
| Whole part/assembly operation, complete EOF and forced construction | `tests/display_archive.rs`; `DisplayArchive::new` encodes and completely reparses supplied fields; all native streams pass the forced-byte gate |
| Unsigned display version, document kind and independent file context | `tests/display_unsigned_versions.rs`, `tests/display_file_contexts.rs`, `tests/display_document_context.rs`; signed aliases, zero, maximum unsigned values, conflicting markers and independent tangent-list boundaries |
| Compact/fixed transforms and exact raw values | `tests/display_transforms.rs`; 1,025 independent vectors and 3,467 distinct native encodings; stored NaNs, signed zero, flags and matrix words retained |
| Shared MFC classes, object identities, aliases and cycles | `tests/display_object_identity.rs`, `tests/display_assembly.rs`, `tests/display_ui_objects.rs`; all 27 registered classes have owned witnesses; compact/extended tag relocation remains covered |
| Environments, splitters, fonts and all earlier root layouts | `tests/display_pre_facet_roots.rs` and its owned version families down to zero; omitted members do not create synthetic stored fields |
| Named, 3D and annotation views | `tests/display_named_views.rs`, `tests/display_spatial_views.rs`, `tests/display_annotation_views.rs`; native collections and independent field traces; later named suffixes integrated in complete roots |
| Display-item selectors and conditional payloads | `tests/display_items.rs`; 32 payload types, 14 known payload-free types and two unknown default selectors in a complete owned root; version families cover earlier payload layouts |
| Buckets, line styles, layers and component-property graphs | `tests/display_buckets.rs`, `tests/display_legacy_buckets.rs`, `tests/display_ui_objects.rs`, `tests/display_pre_facet_roots.rs`; coordinate, version, count, role and alias boundaries |
| Lights, appearances and scenes | `tests/display_lights.rs`, `tests/display_scenes.rs`, `tests/display_earliest_visuals.rs`, `tests/display_earliest_walls.rs`; five light classes, both scene shapes, three wall shapes and early generic PW keys |
| Shaded part/assembly data and explicit external-facet context | `tests/display_shaded.rs`, `tests/display_assembly.rs`, `tests/display_earliest_faces.rs`, `tests/display_earliest_bodies.rs`; inline, cached and absent faces remain distinct |
| Decals, graphics references and all seven reference-geometry kinds | `tests/display_decals.rs`, `tests/display_graphics_references.rs`, `tests/display_reference_geometry.rs`; complete typed bodies, presence flags and isolated edits |
| Ink grids and configuration tails | `tests/display_markup_ink.rs`, `tests/display_configurations.rs`, `tests/display_earlier_range.rs`; counted dimensions, object roles, direct early part bodies, visibility and high-word-first tails |
| Complete cross-stream checks | `tests/display_external_links.rs`; 170 native documents, 193 blocks, 1,258 declarations and 35,242 requested facets checked independently, with no partial links after failure |
| Whole-document edits and unrelated-stream preservation | `tests/display_document_context.rs` and owned containers in the version families; all 1,086 native documents reconstructed in two processes with resolved contexts |
| Immutable ownership and revision scope | `tests/display_object_identity.rs`, `tests/display_edit_history.rs`; source mutation, failed edits, clones, foreign revisions, authentic reverts and Send/Sync assertions |
| Completion, diagnostics, coverage, fidelity and history together | `DisplayArchive::edit_with_history`; exact stored-archive mode and revision-bound structural parents/results; `tests/display_edit_history.rs` and independent named/PW edits |
| Invalid counts, malformed references, limits and incomplete streams | Focused integration tests across each codec and whole-root family; exact byte/field/object/entry budgets, malformed MFC roles and sampled or exhaustive truncation checks as declared by each test |
| Independent owned version witnesses and deterministic execution | 11,313 complete roots in 135 groups reconstruct identically in two processes; version, scene, item, light, material and bucket fields have independent oracles; concurrent edit tests compare serial expected results |
| All 1,086 native streams, exact bytes and field oracles | [Native archive report](archive-audit.json): 1,849,563 independent fields; unchanged manifest `94047d85dc01ed0f391378374f2c15f5464301bb26990d4214eecdd600723a87` |
| Documentation and changelog | Public README, dedicated feature evidence, current and historical schema snapshots, and #28/#38 entries in the unreleased changelog; no package-version change or release |

## Explicit rejection boundary

The contract requires rejection of unresolved bodies rather than an opaque
remainder. These are retained limits of the reference schema, not positive passes:

| Layout | Evidence of explicit handling |
|---|---|
| Open-window transforms at `0x1FAE`–`0x1FCE` | `legacy_zero::uncounted_window_versions_remain_explicitly_unsupported_after_the_lower_guard_is_removed` |
| Inline object-backed facets at `0xA25`–`0xC7B` | `unresolved_facet_objects_fail_at_the_payload_instead_of_rejecting_every_old_root`; empty and external-faced roots remain usable |
| Non-null legacy wall materials | `unresolved_non_null_wall_materials_fail_explicitly_without_publishing_partial_results`; null pointers have complete owned witnesses |
| Non-null `moSketchBitmap_c` textured-plane bodies | `new_bitmap_bodies_remain_unsupported_and_unrelated_classes_cannot_fill_the_pointer`; null-bitmap records retain complete transforms |
| Unresolved `moFacetedData_c` graphics bodies | `unresolved_faceted_object_bodies_fail_explicitly_without_opaque_success` |
| Drawing `Contents/DisplayLists` | Not represented by the part/assembly document-kind API; drawing `Contents/VBLists` has a distinct complete codec and context |

## Drawing VBLists and integration

The [drawing feature evidence](../drawing-buckets/README.md) establishes complete
reading, construction, validated edits and writing for modern unsigned model
versions from `0x27CC`. Its mode distinction is external archive context, not an
invented wire flag. All 20 native streams and containers reconstruct in two
processes, including 33 shaded models and 1,383 independent drawing fields.
Direct lists and raw words retain their framing and unknown meanings. No public
prefix-only archive or opaque shaded continuation is accepted.

The final integration gate is `cargo xtask pr` plus the workflow's Linux, Windows
and two macOS full gates. Linux additionally runs the ThreadSanitizer tests.
All five checks must pass for the final PR head before merging into the standing
interop candidate branch. The completed feature branch is then removed. This
integration does not authorize a version bump, tag, release, or a claim that
the broader #2 interoperability goal is finished.

Historical checkpoint counts remain in their original reports. The humanizer
skill was unavailable; the new prose remains subject to maintainer review.
