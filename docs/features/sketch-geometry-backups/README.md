# Parabola and cylinder sketch backups

Bessemer reads, edits and writes `moParabolaBackedUpData_c`,
`mo3DCylinderBackedUpData_c`, `sgConicHandle` and `sg3DCylinderHandle` in ordinary
modern non-DB external-XML part archives above version `0x27CB`. Specification
11.342 / C-384 establishes the complete serializers and their native ancestry.
`native-ancestry-reference.json` retains all four primary bindings under a
verified installed-module hash.

Parabola inherits the segment backup body, not the line class. The existing
`line` helper implements that common wire prefix: two nullable point-backup
pointers and two signed words. The new reader calls it under `segment_base`,
then reads four more nullable point-backup pointers and two raw doubles. The
historical suffix gates precede this supported scope, so every suffix field is
present. No tagged base, extra owner word or direct geometry header is added.
The complete body with short null/cached pointers is 36 bytes.

The cylinder backup's ordinary base contributes no bytes. Its complete 56-byte
body is a three-double vector, three-double point and one raw double. All seven
numeric meanings remain unassigned beyond their native types. Neither backup
executes native geometry updates or object replacement. The two handle classes
reuse the complete existing entity-handle codec, including independently stored
short or sentinel-escaped indices, without a derived suffix.

The finite general backup and handle families include the four new classes.
Point-backup roles and the existing specific line/segment roles retain their
earlier declarations. Independent readers verify 174 complete graphs and 3,653
primitive observations across four classes and three versions. Cases include
standalone objects, shared/null/shared arrays, complete external-reference
owners, complete component-handle owners, all three external-reference modes,
and eight handle-index encodings. Parabola graphs share two distinct fresh point
backups across all six pointer occurrences. Signed extrema, signed zero,
infinity and NaN payload bits remain unchanged.

Twelve owned streams import the actual configuration map's 97 slots and finish
at 99, 101 or 105 slots. Four independent edit goldens change a raw backup double
or handle reference ID. Public tests reconstruct complete documents and check
source isolation after mutation and rejected owner-mode edits. Five private
tests compare every primitive, byte and map slot; reject every prefix, trailing
byte and old context; reject nonpoint objects at all six parabola point roles;
and prove both backup minima without extra tags or fields. Two Python tests
regenerate every observation and golden.

The shared handle fixture constructor was extracted in a separate structural
commit before these additions. Its existing 144 cases and eight owned streams
regenerated unchanged. New handle graphs call that same complete constructor.
The approved section and hashes are retained in `schema-reference.json` and
`schema-snapshot.md`; `reference.json` records reader hashes and complete
independent observations. Native geometric and index validity, native application
acceptance, special/DB and historical contexts, and full documented file-schema
parity remain unproven. No dependency, package version or exported Rust API
changed. The humanizer skill is unavailable; its prose pass remains with the
maintainer.

The full local PR gate passes 564 library tests, four xtask tests, every
integration suite and 265 doctests; three explicit external-corpus tests remain
ignored. Two fresh native processes preserve all 709 earlier complete
configurations and reach 710 of 906. Their outcomes and causes agree, with a
stable executable hash. Two other cases advance to later explicit stops: an
unsupported reference curve and the reference's already recorded unmapped
`0x4874` object tag. No map entry was guessed and neither case receives credit.

An independent immutable snapshot of 1,153 approved-source Python modules
verifies all 710 accepted streams through exact EOF: 42,734,730 feature bytes,
with every original header/configuration/manager/feature input hash checked.
Temporary source copies and audit code were removed after their processes
terminated. The remaining 196 configurations retain explicit failures; the broad
full-corpus assertion continues to fail until the entire corpus is supported.
