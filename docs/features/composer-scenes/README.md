# Composer geometry streams

Feature [#103](https://github.com/Oblikovati/Bessemer/issues/103) continues the
whole-file interoperability work after the complete carrier and packed-array
operations. This directory currently retains reference evidence. It does not
expose a partial Bessemer scene API.

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

The owned packet includes every drawable kind with normal/texture scenarios,
both texture-import contexts, absent/zero/short/unsupported material bodies,
multiple LODs, noncanonical matrix presence, raw floating patterns, unpaired
UTF-16 surrogates, both string terminators and bounded future bytes. Independent
name and material-text edit goldens vary UTF-16 lengths. Python's standard gzip
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

The implementation gate requires complete read/construct/edit/write operations,
forced typed reconstruction of every decoded byte, gzip integrity, explicit
resource limits, revision-bound fields, isolated edits and derived bounded sizes.
Owned edit goldens and every installed stream must pass the Bessemer codec and
reopen through the actual reference reader. Native/project relationships,
geometry validity, transforms/rendering and application acceptance remain
separate requirements; full scene framing does not complete the parity goal.

The maintainer-side humanizer skill was unavailable. A maintainer still needs
to review this prose.
