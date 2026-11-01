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
installed-body and independent capture proofs. Typed local plans, full
integrated replay, publication gates and remaining feature166 work are pending.

Humanizer is unavailable in this checkout; maintainer prose review remains pending.
