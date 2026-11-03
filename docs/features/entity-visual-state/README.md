# Stored entity visual state

[Feature #181](https://github.com/Oblikovati/Bessemer/issues/181) covers named
stored visual roles over Bessemer's existing complete visibility codec. The
foundation retains eleven complete schema clauses, fifteen historical reports
and nineteen current reader/test pins. The first strict run passed 117 checks:
all 114 tests in eleven unchanged original modules, plus three separate checks
for readers, clauses and reports. No runtime API or parser behavior changed.

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

The initial 117-check foundation and subsequent 118-check installed-image record
bind their snapshots and support hashes to their original Git revisions. Other
complete caller/edit oracles, the public typed operation and retained-corpus
acceptance remain pending.
The [design notes](design.md) retain that complete caller and verification scope.

Run the source checks from the repository root:

```powershell
python -B -X utf8 docs/features/entity-visual-state/freeze_visual_state.py
```

The humanizer skill is unavailable; its prose pass remains for maintainer review.
