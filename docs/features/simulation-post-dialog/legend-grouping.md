# Legend C0 grouping control

Feature166, specification14.478, evidenceC-858.

The stored signed32 word `legend.word_c0` enables conditional locale thousands
grouping when nonzero. It does not choose the separator or guarantee insertion.
Decimal localization remains independent of this control. Existing numeric
format construction determines the preceding e/f/g conversion.

The installed `CDspApp::ConvertNumberFormat` body spans
`0x1800c8320..0x1800c8592` (exclusive end):626bytes and176instructions. Its
`CDspLegend::RenderLegend2EDrawings` caller spans
`0x1800e0100..0x1800e1bd9`:6,873bytes and1,638instructions. Three call sites
pass C0 in EDX across the renderer's branches. These sites are retained
individually rather than treating the two rendering paths as a count of calls.
GNU comparison qualifies exactly five signed-immediate presentation differences
in the converter. Each qualification binds the original instruction, address,
three opcode bytes and GNU spelling. The shared normalizer remains unchanged;
all1,814 locations and the complete body bytes remain required.

The converter obtains decimal and thousands characters from named runtime
providers. It splits at the first literal period, localizes periods in the
suffix, trims trailing whitespace and rejoins the portions. The grouping path
also checks the integer portion and original period position, skips source
strings containing either `e` or `E`, and accounts for leading spaces and a
leading minus in its inspected branch. Reached insertions advance three
positions from the right. A nonzero control alone establishes none of the
other required facts.

The original corpus contains276 accepted interpretations. It supplies160
stored C0 controls:84 have value1 and76 have value0. The other116 retain an
absent C0 wire field; the separate explicit-reader reset policy supplies a
derived runtime zero. Raw-class conversion, constructor defaults and reused
object state remain separate. Native prefix/version qualifications and all
eight original rejected candidates remain part of replay.

This evidence does not establish arbitrary CString rendering, locale character
values, digit validation, actual native execution or additional file coverage.
Owned replay retains arbitrary signed patterns without narrowing them to0/1.
Structural absence is not replaced by a serialized zero.

`legend_grouping_inputs` on both complete parent types returns
`SimulationLegendGroupingInputs` with the original optional
`locale_thousands_grouping_enabled_raw`. It performs no CString conversion or
wire edit. Its behavior version is `C858-stored-grouping-view-v1`.

The owned checks reuse557 complete C857 fixture parents, including132 C0/C4/C8
mutations, and independently compare their C0 presence and bits against C858
Source observations. They consume no reset-event expectations. Native checks
retain all276 complete fixture parents. Eight workers compare complete views
against serial replay; fresh processes compare all1,666 presence/value words.

Targeted freeze passed13 checks in6.935seconds; strict replay passed the same13
checks in5.336seconds. Both processes exited successfully after snapshot cleanup.
All four Rust tests, including native replay, and Clippy passed. Both fresh
processes matched the6,664 expected bytes. The [verification record](legend-grouping-verification.json)
retains the exact gates and [process proof](legend-grouping-process-verification.json).
Full integrated replay, publication acceptance and remaining feature166 work
remain separate obligations.

Humanizer is unavailable in this checkout; maintainer prose review remains pending.
