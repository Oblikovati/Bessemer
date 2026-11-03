# Stored entity visual state

[Feature #181](https://github.com/Oblikovati/Bessemer/issues/181) covers named
stored visual roles over Bessemer's existing complete visibility codec. The
foundation retains eleven complete schema clauses, fifteen historical reports
and nineteen current reader/test pins. The first strict run passed 117 checks:
all 114 tests in eleven unchanged original modules, plus three separate checks
for readers, clauses and reports. That foundation changed no runtime API or
parser behavior.

The first [runtime API route](api.md) now projects revision-bound stored roles
and edits them through complete resolved part feature archives. The
[operation checks](resolved-operation-verification.json) cover all 11,493 owned
model/sketch/display cases, including 22,986 typed RGB changes and no-ops, width
pairs, foreign/stale handles and shared-source mixed success/failure. The full
API routes now cover configuration, display and drawing archives as described
below; wider corpus and operation/publication gates remain open.

The [display and drawing API checks](display-operation-verification.json) now
cover complete display enumeration and revision-bound edits, including real
float absence, complete empty results and shared-source isolation. The full
7,662-case word matrix uses the typed RGB route alongside the existing raw edit
gates. Drawing wrappers retain their layout and expose the shared complete edit
report. The configuration routes are verified below; remaining corpus and
operation gates still need completion.

The [configuration operation checks](configuration-operation-verification.json)
now bind five real visuals in each ordinary owned part and assembly configuration
to independently observed Source spans. Projection, literal RGB edits, original
history, no-ops, document reopening/checkpoint reversal and cross-configuration
rejection pass. The model archive families share the same typed implementation;
the [owned routing checkpoint](routing-operation-verification.json) also verifies
five records through both Source and the typed API. Retained native-corpus
acceptance and the wider operation gates remain pending.

The [native typed projection check](native-typed-projection-verification.json)
passed all 906 retained part cases. Public enumeration and every stored word
match 9,586 configuration visuals and 127,773 resolved-feature visuals. All
per-stream count rows match the earlier physical gate exactly. This establishes
complete native reads/projections; native edit and wider operation acceptance
remain separate work.

The [native Source span capture](native-source-span-verification.json) independently
located all 137,359 physical records in two fresh original-reader processes.
Their offset-and-word packets match byte for byte and are bound to complete
configuration/feature stream hashes. The exhaustive native typed RGB edit test
uses those offsets for literal full-stream expectations. Its complete906-case
run is still pending; passing projection and capture does not establish edits.
The [native assembly capture](native-assembly-source-span-verification.json)
also covers all 180 retained ordinary/routing configurations and 2,516 physical
visuals in two identical Source packets. The [native assembly typed edit check](native-assembly-rust-verification.json)
subsequently passed all 180 configurations and 2,516 changed visual requests in
two fresh processes. Every result matches its complete literal Source-offset
patch, retains its original field parent and leaves the source unchanged. The
unit target was compiled separately; Cargo/Clippy and publication remain open.

The ignored test requires `target/resolved-native-probe.bin`,
`target/entity-visual-native-ordered-spans.bin` and `BESSEMER_KERNEL_SCHEMA`:

```powershell
cargo test --lib native_visual_rgb_edits_match_source_offsets_for_every_physical_record -- --include-ignored --nocapture
```

The [native graphics Source check](native-graphics-source-span-verification.json)
parsed all 910 part displays, 176 assembly displays and 20 drawing streams in two
accepted processes. None contained a physical entity visual record. Each process
also detected the one expected record in both owned positive controls. The full
native identity manifests match the earlier complete corpora. The
[Rust native graphics check](native-graphics-rust-verification.json) also passed
all 1,106 documents in two fresh processes. It forces complete stream encoders,
checks typed empty results and exact whole-document reopening, and binds all
194,267,137 reconstructed bytes per process to the Source hashes. Both exports
match byte for byte. The test was compiled against the existing validated
library without another Cargo command; Cargo/Clippy and publication gates remain
separate. These native empty results complement the positive owned edit matrices.

The existing physical fields remain the compatibility surface:

| Field suffix | Stored role | Type |
| --- | --- | --- |
| `unknown_0` | Layer number | u16 |
| `unknown_2` | Full ByLayer word | u16 |
| `unknown_4` | Layer-attribute cache byte | u8 |
| `unknown_8` | Complete color word | u32 |
| `layer` | Line style, distinct from layer number | u16 |
| `line_width.standard_width` | Stored width selector | i16 |
| `line_width.custom_width` | Version-dependent custom width bits | raw f32 |

The typed operation must preserve these names and every unselected field. Color
channels are the low, next and third bytes, in red/green/blue order. The high
byte stays unresolved; it is not alpha. Only the exact `0xFFFFFFFF` word means
missing color. RGB white alone does not. ByLayer roles are the five named low
bits; cache bits 0 and 1 are stored visibility and printability. Effective
layer/default/renderer choices, runtime overrides and setter cascades remain
outside stored-file edits. Widths retain raw values without unit conversion or
positivity/finiteness restrictions.

The source harness reuses the immutable-copy/import framework and the existing
dimension test asset staging. Every original test runs without patches. Native
research dependencies are checked against their original hashes. Complete
historical reader/tool provenance remains in the copied reports; it is not
silently treated as the current reader revision. The C-1959 packet preserves
all 906 feature-stream identities, 127,773 physical visual reads and their
894,411-byte unknown charge. Its historical accounting totals are preserved,
without adopting them as current Source accounting or claiming new byte credit.

The [installed-image check](installed-verification.json) subsequently passed all
118 checks, including every original test. Three pinned DLLs match all 93
reported intervals totaling 13,962 bytes, 45 explicit named bindings, seven
import call sites, the raw RGB normalization constant and both renderer/provider
links. All declared export names were read in full, avoiding the PE reader's
default count limit and name preview. Overlapping report captures remain
separate; the byte total is the sum of reported intervals. These are ordinary
static observations with the existing PE reader, not fresh REA Evidence. CAD
code was not loaded or executed, and no semantic byte credit was added.

The first complete owned caller contains 3,831 resolved-feature streams with
7,662 physical folder definitions and 3,831 instance aliases. Independent
literals cover every color high byte, cache byte and known ByLayer combination,
plus signed width limits, raw floating-point edge patterns and layer/style
boundaries. The 386,931 original primitive reads partition every stream byte;
53,634 literal visual spans also retain their original reader attribution.
The [owned proof](owned-verification.json) records the strict replay and hashes
of the complete carriers, word expectations and primitive observations.

The [complete sketch proof](owned-sketch-verification.json) extends the same
3,831-case matrix to direct points inside real `sgSketch` owners. Its 7,662
physical visual records and 3,831 aliases span 6,903,462 wire bytes and
2,041,923 original primitive reads. Initialization wrote only the new sketch
artifacts. The subsequent no-write replay passed all 120 checks: 114 unchanged
original tests and six separate evidence checks. All four earlier folder
artifacts remained byte-identical after the shared corpus verifier was
extracted. No Source reader registration, runtime API or native byte credit
was added.

The [nested display proof](owned-display-verification.json) covers 3,831 complete
revolution streams, each with two display dimensions reached through their real
owners. It distinguishes 7,662 focused display visuals from 7,662 unchanged
model-feature visuals. The complete streams retain 3,831 outer aliases and
108 map slots per case. All 3,053,307 original primitive reads partition the
13,216,950 wire bytes. Initialization writes only the display artifacts; the
strict replay checks the new corpus and all eight earlier folder/sketch
artifacts without rewriting them. Component checks remain separate.

The [Rust carrier check](rust-carrier-verification.json) decoded all 11,493
folder/sketch/display cases through the existing complete feature-archive API.
Both focused records matched the literal offsets and stored bits. Full
field-based reconstruction reproduced every byte, retained the expected object
slots, reported `Unchanged` and returned empty changed-field history. All three
integration tests and Clippy passed. This check adds no runtime visual API.

The [native carrier check](native-carrier-verification.json) decoded all 906
retained configurations and feature streams. It found 9,586 configuration
visuals and matched all 127,773 feature visuals to the original C-1959
per-stream identities, lengths, hashes and physical counts. Every selected
record retained its complete field shape, and the feature streams reconstructed
exactly. The configuration count is a separate observation; historical byte
accounting and semantic credit remain unchanged.

The [changed RGB write check](rgb-write-verification.json) exercises both
physical targets in every owned case: 22,986 changed writes through the existing
validated archive edit operation. Each complete result matched independently
patched literal bytes, preserved the high byte and every unselected field, and
retained its original source bytes. Every request returned one changed-field
history entry with the original revision-bound parent. These raw-codec checks
predate the typed API checkpoints above; broader acceptance remains pending.

The [complete word write check](word-write-verification.json) replaces all seven
physical values in both targets of every owned model, sketch and nested display
case: 22,986 changed requests and 160,902 changed fields. Each result matched a
literal 17-byte replacement, preserved every unselected field and retained its
source bytes. History matched the existing source-path order, including all seven
original revision-bound parents. These checks use the existing archive codec;
the typed API checkpoints above provide the current operation status.

The [component context probes](component-context-probe-verification.json) accepted
all 88 retained complete roots in two fresh processes across ten declared display
versions from 5000 through 19000. Both runs agreed on every case, including 86
physical visuals and 172,216 primitive reads. A named fake document supplied each
matching version marker; the original readers checked complete bytes and holder
identity without overrides. Older native document markers, versions below 5000
and the full modern component word matrix remain separate acceptance work.

The [Rust component context checks](component-context-rust-verification.json)
reconstruct all 88 baseline roots and edit all 86 physical visuals. All 11,696
single-bit writes matched independent literal byte changes and preserved every
unselected field and source byte. Each change retained its exact revision-bound
parent. RGB edits also reparse and reverse exactly; 14 modern cases retain
whole-document checks. The early layout checks below cover retained pre-5000
baselines.

The [early Source checks](early-component-source-verification.json) accepted all
220 retained bucket-owned component roots across 55 exact versions in two fresh
processes with identical Source snapshots. They checked 224,302 primitive reads,
132 visuals without a custom-width float and 88 with one. The matching
[Rust checks](early-component-rust-verification.json) passed explicit field-presence
checks and 25,696 single-bit writes, including the 3904/3905 boundary. Versions
below 3196 and continuous version coverage remain unproven. The full operation gates remain pending; the full modern matrix initialization is
recorded below.

The [changed-output Source checks](component-changed-source-verification.json)
accepted all 37,698 independently expected complete bit/RGB outputs asserted by
the Rust tests. Two fresh processes matched every case and ordered output hash
over 244,882,785 bytes. Original readers checked exact EOF, named values, custom
width presence and holder identity without primitive or float overrides. The
inputs were literal expected bytes, rather than exported Rust-produced files;
the Rust checks establish equality to the actual edit results. Rust process and
concurrency exports remain part of the full operation acceptance.

The [drawing caller probe](drawing-owner-source-verification.json) retains a
complete 554-byte `VBLists` candidate at model version 10200. Original readers
accepted it twice with 193 primitive reads and five map slots. Its real bucket
object keys share one physical component property; top-level aliases, a null
pointer and a bucket self-reference retain identity. The
[Rust drawing checks](drawing-owner-rust-verification.json) now reconstruct it
through the public wrapper and verify all 136 bit edits and exact reversals. Eight
workers match serial bytes, and two fresh Rust exports match exactly. Original
readers accepted every actual export with 26,248 primitive checks and unchanged
unselected reads. A [direct-buffer rerun](drawing-direct-export-verification.json)
exports the encoder buffer itself after each independent byte assertion; both
fresh exports retain the exact Source-accepted packet hash. Wider drawing
profiles and native document acceptance remain pending.

The [full component initialization](component-initialization-verification.json)
passed 7,662 complete modern cases, with 16,791,273 fresh primitive reads over
81,129,087 wire bytes and 53,634 selected scalar spans. Each case preserved every
unselected primitive and the real holder cycle/shared-property identity. Storage
factors out the two unchanged complete templates; parsing still covers every
case. All three focused checks passed in 5,066.655 seconds. The initial Source
snapshot is retained separately. The [strict replay](component-strict-replay-verification.json)
also passed all three checks in 5,162.718 seconds, with unchanged fixtures, helpers
and every Source file shared with initialization.

The [full Rust component matrix](component-matrix-rust-verification.json) passed
all 7,662 cases in two fresh processes. Each input reconstructs through
`DisplayArchive::new`, each RGB edit preserves its unselected fields and exact
history parent, and every case reopens and reverses inside a complete owned
document. Both direct encoder exports match at 162,319,474 bytes. Original Source
accepted all 15,324 exported roots, checked each input against its retained hash
and each RGB result against a literal patch, and retained holder identity. The
strict primitive replay has since passed; wider operation gates remain open.

The [actual typed export checks](typed-export-verification.json) passed in two
fresh component processes and two fresh drawing processes. Component exports
come directly from the typed RGB edit result; drawing exports exercise all 136
stored bits through typed requests and compare eight workers with serial output.
Every exported byte matches the retained packets already accepted by complete
original Source readers. This reuses that acceptance by byte identity and does
not claim a fresh Source execution for those typed packets.

The [complete component concurrency check](component-concurrency-verification.json)
passed all 7,662 modern word cases in two fresh processes. Eight workers share
each immutable snapshot and perform a changed RGB edit, a rejected width request
and a no-op. All 206,874 requests per process match serial bytes, every field,
history, diagnostics, coverage, fidelity and typed errors. Both wire exports
retain the Source-accepted packet bytes. Complete operation traces also match
byte for byte, covering 2,790,230,192 uncompressed bytes per process with raw
stored float bits. Current reader pins and all eleven schema clauses still
match; Cargo/Clippy, remaining native parts and publication gates stay open.

The [full model concurrency check](model-concurrency-verification.json) passed
all 11,493 owned model, sketch and nested-display cases in two fresh processes.
Each process checked 22,986 focused visuals through 620,622 changed, rejected and
no-op requests. Both complete wire exports and 2,668,426,898-byte operation
projections match byte for byte. An independent checker binds every input and
both focused edits to the owned Source packets and literal scalar offsets.
The [document controls](model-document-controls-verification.json) also passed
all three complete callers and six exact document edit/reopen/reversals. The
[full document matrix](model-document-matrix-verification.json) subsequently
passed all 11,493 cases twice, with 22,986 changed/reopened documents and typed
exact reversals per run. The stronger repeat also verifies each changed field
vector and original history parent. Every unrelated stream, name and order
remains exact, and reversal restores every original container byte. Native part
and repository/publication gates remain open.

The initial 117-check foundation and subsequent 118-check installed-image record
bind their snapshots and support hashes to their original Git revisions.
Retained native edit acceptance and wider operation/publication gates remain
pending.
The [design notes](design.md) retain that complete caller and verification scope.

Run the source checks from the repository root:

```powershell
python -B -X utf8 docs/features/entity-visual-state/freeze_visual_state.py
```

The humanizer skill is unavailable; its prose pass remains for maintainer review.
