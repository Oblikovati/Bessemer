# PLine silhouette edge identifiers

Issue #54 implements clause 11.192 / C-220 with the exact native
`moPLineSilhIdRep_c` dispatch. In ordinary external-XML part archives above
version `0x27CB`, its body reuses the common entity representation, a direct
source-entity ID and two signed words at +48 and +4C. The direct ID contains its
tagged representation without an outer entity tag or archive-map slot. Native
serializer selection and ancestry establish this reuse; names and the source
reader's permissive expected-class argument are not role oracles.

Independent fixtures cover 30 complete graphs across five versions, 5,686
primitive observations and seven owned streams. They retain null, fresh and
cached pointers, distinct equal roots, shared complete references, nested source
representations, tagged entity owners and signed extrema in both suffix words.
An independently frozen edit changes only the first word of the first equal root.

Private tests check every truncated prefix, appended bytes, old context,
14-byte minimum bodies, exact map accounting, exact class dispatch and rejection
of unrelated cached arrays through feature, external-reference, source-ID and
tagged entity-owner roles. Public tests verify complete document reconstruction,
unchanged storage and source isolation after successful and rejected word edits.

Run `cargo test --lib model_archives::silhouette_edge_tests`,
`cargo test --test resolved_silhouette_edges` and `cargo xtask pr`.
Independent regeneration uses `python -B -X utf8 -m unittest discover -s docs/features/silhouette-edge-identifiers`.
Source hashes must remain stable across regeneration.

Two fresh native processes agree on all 906 outcomes and causes. Cases 302 and
303 complete, increasing reconstruction from 765 to 767 inputs without losing
any prior success. Four cases reach the documented projected-line draft role
rejection, catalogued under Bug #46; they receive no completion credit. The
independent pinned-source audit verifies all 767 accepted streams and four
stream hashes per input, covering 49,339,288 feature-stream bytes. The same
snapshot reproduces four original containing-root spans totaling 4,578 bytes,
including all six silhouette records' raw words and source classes. Results
are in `accepted-reference.json` and `native-sentinel-reference.json`.

Source/native bindings and executed checks are recorded beside this document.
Historical migrations, alternate contexts, index/direction meanings, geometry
validity, SolidWorks application acceptance and broader all-schema parity remain
open. Stored values of one in six native records do not narrow the signed domains.
The humanizer skill is unavailable; prose review remains a maintainer task.
