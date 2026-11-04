# Legend A8 preference conversion

Feature166, specification14.480, evidenceC-860.

`CDspLegend::GetPreferences_Adv` converts object wordA8 to the first word of
`plotAdvPreference` with an integer zero test. The result is1 for source0 and0
for every other signed32 value. Original words remain authoritative: applying
this relation twice to2 yields1, so it cannot reconstruct arbitrary inputs.

`SetDefaultOptionByPreferences` applies the same zero test to preference
source1528 on its reached assignment paths. Its existing plot-object gate
returns early for type22,57 or73. Type70 reaches a separate palette setup that
also writes A8. Neither the file nor the scalar relation establishes which
runtime plot object or preference source was present.

The getter body at`0x1800dbbc0..0x1800dbc2a` has31instructions and106bytes.
The setter at`0x1800e3a90..0x1800e3c0c` has91instructions and380bytes. Full
instruction and byte checks retain the exact exports, both original
decompilation hashes, both zero-test assignments and the bounded bit-test
mask for the early plot-type returns. The getter's other color exports and
the setter's other assignments remain separate from this scalar relation.

All276 original native interpretations retain their four-byte A8 word and
absolute offset. Of these,252 store0 and24 store1. Derived preference views
remain separate from stored values. All eight original rejected identities,
candidate versions and modern prefix-only boundaries remain explicit.
Owned replay retains425 complete parents, including20 raw-class contexts
without a canonical A8 field. Absence supplies no preference value or default.

This work does not execute native code, recover a UI option name, emulate
the entire preference methods or authorize universal A8 rewrites. The later
layer-zero renderer gate is a separate consumer binding. No additional wire
bytes or global coverage credit arise from the conversion.

Targeted freeze passed15 checks in6.021seconds. Strict replay passed the same15
checks in5.489seconds; both processes exited successfully after snapshot cleanup.
The [verification record](legend-inverse-verification.json) retains exact source,
installed-body and independent capture proofs. The full foundation replay passed
329checks in500.588seconds and exited successfully after snapshot cleanup.

The typed operation behavior is `C860-local-preference-zero-tests-v1`.
Both complete parent types expose optional canonical A8 through
`legend_preference_inputs`. `project_preference_word` requires that word and
returns its derived preference0 value. An absent raw-class field is a typed
missing-input error, never a default value.

`SimulationLegendPreferenceRuntime::project_local_a8_assignment` requires an
explicit plot context. `None` is unknown; `Some(NullPlot)` is a known null plot.
Present types22,57 and73 complete with `NoAssignment` before consuming the
optional preference1528 word. Other contexts require that source word; type70
retains its distinct branch identity. The known-null case describes only the
reached local store and establishes no safety for later native pointer uses.

Plans preserve every supplied input, including unused source words on an early
return, with a derived history relation, completion, diagnostics and exact local
read/store counts. They change no archive bytes, normalize no original words
and infer no previous A8 value from an empty assignment. Full getter/setter
behavior, palette operations and native pointer validity remain separate.

Operation fixtures cover12,678 fixed/generated cases,565 complete owned parents
(including140 A8 mutations) and276 native parents. Independent setter decisions
use the native bounded unsigned subtraction/bit mask rather than the Rust match
arms. Both lower and upper mask aliases, missing-input order, noninvertibility,
original unused values and completed early returns are checked.

The operation's targeted freeze passed17 checks in6.785seconds. Strict replay
passed17 in5.973seconds; both processes exited successfully after cleanup.
All11 Rust tests and all-target/all-feature Clippy passed. Eight workers matched
serial replay; two fresh processes matched202,785 original-history/result words,
811,140bytes, against independent expected fixtures. The
[operation record](legend-inverse-operation-verification.json) and
[process proof](legend-inverse-process-verification.json) retain the exact gates.
Full operation Source replay passed331 checks in541.752seconds and exited
successfully after snapshot cleanup. Publication acceptance and remaining
feature166 work are pending.

Humanizer is unavailable in this checkout; maintainer prose review remains pending.
