# Stored visual-role operation

The planned profile is `entity-visual-stored-roles-v1`. It adds named stored
projections and isolated edits without changing the existing codec, raw field
paths, byte layout or accepted caller/version boundaries. No version bump or
release is part of this work.

The shared codec has four source callers: model features, model display objects,
sketch entities and display component properties. The operation must cover the
complete part configuration, resolved-feature and assembly configuration APIs,
plus complete part/assembly display archives and applicable drawing-bucket
wrappers. Drawing-bucket model-version restrictions stay explicit. Each caller
needs an independently valid complete carrier; a standalone record passing the
Source reader does not establish reachability or whole-archive acceptance.

Project the complete layer/style, ByLayer, cache, color and width values from
revision-owned physical fields. RGB edits preserve the high byte. Single-bit
ByLayer/cache edits preserve every other bit. Raw word requests preserve all
supplied values, including sentinels. Width edits must respect the actual
presence of a stored custom-float field; do not invent one or convert a raw bit
pattern into a numeric normalization. Retain unrelated fonts, strings, geometry
payloads and object aliases. Missing records and wrong types/scopes/versions
fail explicitly; they do not become zero-filled visual records.

Keep source handles bound to the archive revision. Delegate complete replacement
fields to the existing operation result, preserving validation coverage,
diagnostics, completion and ordered copy/derived history. Width requests can
affect two physical fields; history must reflect the actual changed fields in
wire order. No-ops retain identity. Source and caller buffers remain isolated
after success, failure and later source changes.

Acceptance needs all high bytes, asymmetric/primary/white RGB values and exact
missing-sentinel controls; all cache byte combinations and ByLayer roles with
unresolved bits; layer/style signedness and sentinel cases; raw width limits,
negative zero, NaN/infinity patterns and real version-presence boundaries.
Use literal complete images and full Source primitive/field oracles, then check
all unselected fields, object identity and every ordered history parent. Whole
documents must reopen, preserve unrelated streams and reverse exactly.

Bind all retained part feature counts to the original C-1959 stream identities
and hashes, without changing its unknown accounting. Other model/display/drawing
corpora need separate declared denominators. Published types need compile-time
Send/Sync checks. Shared snapshots must survive mixed successful and failed
operations across workers; full serial, multithread and two-process exports must
match in bytes, fields and history. Finish local and exact-head platform gates
before publication. This remains file-operation acceptance, separate from
native application opening, rendering and geometry validity.

An audit lead needs a complete caller before any bug claim: Source's component
font count has earlier 9/10/11/13/14-font branches, while the current Rust helper
starts with 15. Source also has early component-property/holder gates. Bucket
CObject-key maps start after `0xAD2`, and modern model callers use later
versions, so a difference between standalone bodies is insufficient evidence of
a public decoder defect. Construct a reachable Source-valid complete archive
before classifying a mismatch as a Bug. Keep any proven fix separate from this
additive operation and land it through the patch workflow.

The first owned carrier uses complete `moFolder_c` definitions in a resolved
feature stream. Each case contains a new class, a loaded-class definition with
different words, an alias to the first instance and an empty boundary list.
The inherited 97-slot context grows to 100 slots. All original primitive reads
partition the complete stream, including the fourteen literal visual scalars.
Integer wrapper attribution stays with `mfcobjects`; direct byte/color reads
belong to `dl_light` and the custom width float belongs to `dl_ui`.

A sketch point cannot serve as a top-level feature: the unchanged Source reader
rejects that class at that location. Sketch coverage must use its real sketch
owner. No reader registration or Source override was added to accept the
discarded fixture attempt. Other visual callers still need complete carriers.

Publication is serial behind the dialog publication and the existing accepted queue. This
foundation creates no publication branch or PR. The humanizer skill is
unavailable; its prose pass remains for maintainer review.
