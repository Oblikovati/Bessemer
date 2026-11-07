# Copy, trim/extend and reference edge identifiers

Issue #54 adds the three exact native dispatches in clause 11.232 / C-274.
`moCopyEdgeIdRep_c` and `moTrimExtEdgeIdRep_c` reuse the complete common
entity-representation base, a direct source-entity ID and signed word +48.
`moEdgeRefIdRep_c` uses the common base and signed word +38. The direct ID
adds its tagged representation without an outer entity tag or map slot.
Native dispatch and ancestry establish these layouts; class-name similarity
and the source reader's permissive expected-class argument are not role oracles.

The supported scope is ordinary external-XML parts above version `0x27CB`.
Raw signed words retain their bits. Historical migrations, database modes,
scalar meanings and geometry semantics remain unproven.

Independent fixtures contain 90 graphs across five versions, 16,708 primitive
observations and 21 owned streams. They exercise null, fresh and cached pointers,
complete shared source representations, distinct equal roots, tagged entity
owners and three independent word edits. Rust checks every truncated prefix,
appended bytes, old context, exact map accounting and unrelated cached-pointer
rejection. Public tests verify byte-exact document reconstruction, edits and
input isolation after both successful and rejected edits.

Run `cargo test --lib model_archives::copy_edge_identifier_tests`,
`cargo test --test resolved_copy_edge_identifiers` and `cargo xtask pr`.
Independent regeneration uses `python -B -X utf8 -m unittest discover -s
docs/features/copy-trim-edge-identifiers`; source hashes must remain stable.

Two fresh native runs agree on all 906 outcomes and causes, preserving all
764 complete inputs. Cases 346 and 348 reach `moStock_c`; case 747 reaches
`moDelFace_c`. These later stops receive no completion credit. The unchanged
accepted set is bound to the preceding independent filled-surface audit of
48,978,769 feature-stream bytes; no fresh source replay is claimed here.
Geometry validity and SolidWorks application acceptance remain separate.

The humanizer skill is unavailable; prose review remains a maintainer task.
