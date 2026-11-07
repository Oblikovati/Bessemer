# Trim-wire and pattern-body edge identifiers

Issue #54 adds `moTrimWireExtEdgeIdRep_c` and `moPttnBodyEdgeIdRep_c` in
ordinary external-XML part archives above version `0x27CB`. Both use existing
complete entity-representation readers. Clause 11.350 / C-392 binds the exact
native serializers and ancestry; the other five classes in that clause remain
separate work.

The trim-wire body retains feature and external-reference pointers followed by
two signed words. The pattern-body body also contains a direct entity ID before
its two signed words. That direct member adds no outer object tag or identity-map
slot. Signed values remain unknown fields and are preserved without normalization.

The independent fixtures cover 60 complete graphs across five versions, 11,142
primitive reads and 14 owned streams. They include null and shared references,
complete trim-feature parents, distinct equal roots, repeated aliases, fresh and
cached entity-ID owners, signed extremes and isolated word edits. The source
reader verifies actual target identities and complete primitive partitions.
Rust checks byte preservation, exact object slots, every truncated prefix,
appended bytes, old context and unrelated cached pointer targets. Public tests
check document storage preservation and input isolation after successful and
rejected edits.

Regenerate with `python -B -X utf8 docs/features/frontier-edge-identifiers/freeze_frontier_edges.py`.
Run the Python checks with `python -B -X utf8 -m unittest discover -s docs/features/frontier-edge-identifiers`.
Run Rust checks with `cargo test frontier_edge` and
`cargo test --test resolved_frontier_edge_identifiers`; `cargo xtask pr` runs the
repository gate. Source hashes must remain stable throughout regeneration.

`schema-reference.json` and `native-binding-reference.json` bind the specification
and installed module evidence. `reference.json` records independent fixture
observations. Native corpus results are recorded separately; synthetic success
does not establish whole-file or geometry validity. Earlier trim-edge fixtures
remain unchanged; shared fixture setup is described in `fixture-reuse.md`.

Two fresh native corpus processes produced identical outcomes and failure causes
for all 906 inputs. Complete reconstructions remain 751, with 155 explicit stops.
Case 291 advances to `moLPatternItemId_c`; case 353 advances to `moNetBlend_c`.
Neither receives completion credit. The unchanged accepted index set is bound to
the prior independent source audit of 47,006,007 feature-stream bytes.

The humanizer skill is unavailable in this session; prose review remains a
maintainer task.
