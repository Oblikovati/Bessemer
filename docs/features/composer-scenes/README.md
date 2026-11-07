# Composer geometry streams

Feature [#103](https://github.com/Oblikovati/Bessemer/issues/103) continues the
whole-file interoperability work after the complete carrier and packed-array
operations. `composer::geometry::ComposerGeometryArchive` reads, constructs,
edits and writes complete version-7.9 geometry streams.

`from_gzip` validates strict header/deflate termination, decoded size and CRC.
`from_decoded` accepts the entire decoded grammar. Both require exact typed
reconstruction before publication. `from_fields` constructs a new stream and
requires canonical paths, storage types and array caller contexts after reparse.
`to_decoded` serializes every stored field. `to_gzip` retains original compressed
bytes for no-op reads and creates a checked-size stream for new constructions.

`fields`, `field_id` and `field` expose immutable, named typed storage. Text
retains raw code units and its terminator. Matrices/colors retain fixed pattern
arrays; vectors and indices use the existing complete array types. Revision-bound
field IDs reject foreign or stale revisions. Counts omitted by the wire remain
absent; runtime defaults are not fabricated as extra stored fields.

`edit_field` changes a copy, derives enclosing logical/physical sizes, reparses
all typed fields and validates the complete new gzip before publication. Bounded
size fields cannot be edited directly. Completed outcomes return completion,
exact stored-field fidelity, full coverage, parent/result history and explicit
project/geometry/application exclusions. All published types are `Send`/`Sync`.
Carrier composition uses `ComposerArchive::edit_member` with `scene.to_gzip()`;
the complete scene is reopened while unrelated carrier contents stay unchanged.

The pinned contract is document version 7.9 with explicit texture-import
context. Clauses 14.7, 14.87 and 14.88 define the information prefix, sized user
data, geodes and LODs, common material attributes and all six drawable kinds.
The existing packed-vector and index codecs supply the array wire operations.
Numeric reconstruction remains an explicit separate operation.

Information logical size excludes the two info-string terminators. User-data
and material sizes include their complete string terminators. Short bounded
bodies can omit later complete fields; partial scalars, matrices and strings
fail. Only explicitly sized documented extension bytes may be retained. An
unexplained scene suffix must fail the complete operation.

The 45 owned cases include every drawable kind with normal/texture scenarios,
both texture-import contexts, absent/zero/short/unsupported material bodies,
multiple LODs, noncanonical matrix presence, raw floating patterns, unpaired
UTF-16 surrogates, both string terminators and bounded future bytes. Independent
geode-name, info-text, user-data-text and material-text edit goldens vary UTF-16
lengths. Short metadata cases distinguish absent fields, stored empty strings,
default LOD 1 and explicit LOD 0. Python's standard gzip
writer supplies owned compressed inputs, and its independent decoder must agree
with the actual reference gzip/CRC reader on all owned and native contents.

All 38 installed streams must reach exact EOF: 45,280,926 decoded bytes, 1,787
geodes and 2,056 drawables. The drawable counts are 138 triangle sets, two indexed
triangle sets, three indexed triangle-strip sets, 1,903 mono-indexed triangle-strip
sets and ten line-strip sets. Point sets have synthetic coverage. Every original
archive/member identity, complete scene projection and record count is retained
by hash. Large native payloads stay in the local probe packet.

Rejection checks cover every compressed prefix of the owned inputs, decoded
extensions and unsupported versions. A decoded stream ending at a complete
top-level record boundary is a valid shorter stream; it must not be mislabeled
as an invalid truncation. Gzip integrity and exact compressed termination supply
the complete encoded-file boundary.

Run `python docs/features/composer-scenes/freeze_scenes.py` to replay the
baselines. Add `--export` to write the local native probe packet. `--write` is
reserved for reviewed fixture updates. All reader and fixture-builder imports
come from an immutable copied source tree whose hashes and origins are checked.
The schema snapshot compares scoped clauses independently of unrelated changes
elsewhere in the upstream specification.

The native Rust gate reconstructs and constructs every installed stream, edits
its first geode name and requires all other stored fields to stay equal. The
`verify_native_edits.py` audit reopens all 38 edits through actual frozen readers
and independent gzip decoding. It compares every source projection value after
the requested name change, excluding only byte positions and derived scene byte
count, and checks original gzip headers and member identities.

Behavior 1's owned baseline was extended by appending bounded-text edit goldens,
short metadata cases and constant-point context checks. All earlier 35 complete
inputs/projections stayed unchanged. The added cases pin documented branches;
they do not change accepted native behavior or widen wire validation rules.

Native/project relationships, geometry validity, transforms/rendering and
application acceptance remain separate requirements. Full scene framing does
not complete the parity goal.

The maintainer-side humanizer skill was unavailable. A maintainer still needs
to review this prose.
