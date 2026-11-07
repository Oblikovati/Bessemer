# Early text tails, facet flags and stored polygon nodes

Reference baseline: specification edition 0.250, dated 2026-10-25. Reader hashes
are recorded in the fixture manifests.

The 262 complete part and assembly roots cover versions 1645, 1646, 1673, 1674,
1834, 1835 and 1857, with 78250 independently expected traced and physical fields.
Ordinary text item kinds 7 and 15 omit `unknown_34`, `unknown_38` and `unknown_40`
through 1834 (`0x72A`). All three fields become stored together after that bound.
Both empty and populated Unicode text records are covered. Width flags remain
absent independently in this interval.

Before 1646 (`0x66E`), the native polygon reader consumes the node i32 but replaces
its logical node ID with -1. Bessemer retains the physical i32 in `node_id` for
exact writes. The reference trace derives the native logical value separately;
it does not overwrite the archive field. The oracle captures the consumed word
before the reset and checks it through independent physical-field expectations.
Owned fixtures store both signed extremes, including in empty and nonpositive
polygon records. This proves reconstruction without treating the native reset as
permission to discard bytes.

The generator observes complete ordinary-text records before removing their
verified final twenty bytes. It independently replaces polygon node words and
inline facets. These changes preserve all MFC class and object identities.
Every final root is parsed to its end at the declared version. Earlier fixture
groups remain unchanged.

Fifty-six inline models each contain two complete facets. The raw facet
`edge_ids_valid` word is absent through 1673 (`0x689`). Twenty-eight independent
facet records compare full typed contents and force exact writes at each version.
All earlier body, polygon, component, environment, material, font and bucket-list
gates remain independent.

Composite text, physical node, polygon, edge, font and facet-count edits match
independently built roots and authentically revert. A logical-node boundary
change remains byte-compatible because its physical layout is unchanged. Contexts
requiring new text or facet words reject missing fields without changing source
inputs. Truncation tests reject partial roots. Complete cached models, empty/null
models, all five old-light classes and nested 2D/3D buckets remain covered.

Four owned containers cover baseline and edited part and assembly streams at
1645. Replacement, reopen and revert preserve unrelated streams. Process
verifiers compare complete roots and whole containers in two fresh processes.

This checkpoint admits archive contexts from 1645. Earlier documented layouts
and unresolved representations remain tracked by #28. Byte fidelity does not
establish geometric validity or native application acceptance.
