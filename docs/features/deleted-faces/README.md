# Deleted-face feature interoperability

Feature #54 implements the exact `moDelFace_c` and `moDelFaceData_c` bindings in
clause 11.230 / C-272. Scope is ordinary external-XML parts above version
`0x27CB`, where every documented deleted-face field is present. Earlier reader
migration branches and DB streams remain outside this scope.

The feature reuses the complete body-feature prefix, then reads one nullable
direct object array, one signed word, a direct object array, a direct DWORD array,
four signed words and a final direct object array. The DWORDs retain unsigned
32-bit values. Constructor state at runtime offset `+860` is not serialized.
The data class has no owner pointer or ordinary object-base bytes: it contains
three independently nullable direct arrays and one signed word.

Each nullable array retains its original signed presence word. Every nonzero
value introduces the direct counted array body, including present-empty arrays.
Object arrays preserve tagged identities, nulls, duplicates and cycles; their
direct containers add no MFC map slots. Packed counts retain short/escaped
framing. Counts are checked against remaining bytes and aggregate budgets before
entries are read. The three object arrays and DWORD array keep independent counts.
No scalar meanings or class restrictions are inferred from the observed values.

Independent fixtures cover 108 complete graphs across nine versions, 15,396
primitive observations and fourteen owned streams. They include complete rich
feature bases with cached owners, standalone/nested data, shared surface/data
targets, self references, distinct roots, repeated root identities, signed
presence extrema, escaped counts, high DWORD bits and two isolated word edits.
The empty data body is sixteen bytes; the empty feature payload after its common
base is thirty bytes. Actual source readers verify complete EOF, primitive
partitions, map slots and identities under stable source hashes.

Eight private tests check all independent bytes/fields/map slots, every truncated
prefix, appended bytes, old contexts, exact minima, omitted constructor state,
independent presence words, unsigned DWORDs, shared nested data and impossible
counts. Public document tests cover complete reconstruction, independent edits
and input isolation after successful and rejected edits.

Run `cargo test --lib model_archives::deleted_face_tests`,
`cargo test --test resolved_deleted_faces` and `cargo xtask pr`.
Regeneration uses
`python -B -X utf8 -m unittest discover -s docs/features/deleted-faces` against a
stable approved source tree. The source's permissive expected-class argument is
not a role oracle; the supplied native binding establishes the feature's
body/atom ancestry and the data object's separate ordinary base.

The local PR gate passes with 788 library tests, all integrations and 274
doctests. Two fresh native processes agree on all 906 outcomes and causes. Cases
110 and 338 now reconstruct completely, raising the accepted set from 801 to 803
without losing any earlier success. Four other cases reach later explicit stops;
case 353 exposes the further `reference_78` / `moSolidRef_w` role rejection under
Bug #46. These partial advances receive no completion credit. The aggregate
corpus assertion still fails explicitly for 103 unsupported cases.

A verified immutable source snapshot independently checks all 803 accepted EOFs
and four original stream hashes, covering 52,340,481 feature bytes. Under the same
snapshot, all recorded intervals/classes in four original configurations match,
including the four deleted-face roots' exact hashes, raw words, presence values
and independent counts. Those roots total 65,197 bytes. Reports and exclusions
are retained in `frontier-checkpoint.json`, `accepted-reference.json`,
`native-sentinel-reference.json` and `local-verification.json`.

This library preserves the stored feature description; it does not execute face
deletion, healing or document regeneration. Geometry validity, application
acceptance, historical contexts and full schema parity remain open. The humanizer
skill is unavailable; maintainer prose review remains open.
