# Numeric plot54 callback color

Section14.509 and C-891 bind the complete four-channel2AC read in the numeric
plot54 callback path. The raw2BC copy remains a separate forwarded argument;
forwarding does not establish its purpose or prove global nonuse. Both original
RGBA groups remain authoritative.

Caller180159560 first loads runtime2C68 and admits only unsigned(mode-3) below2,
then tests parent plot code170 against54. It copies2BC first and2AC second,
passing2AC inR8 and2BC inR9. No UI names are inferred for these numbers.

Helper18015A820 savesR8 as its color source. It rejects a missing core2520 object
before the key10 callback lookup, and rejects a null lookup result before the
GetColor4fv read. Renderer virtual state calls precede that read and remain
outside a local color-only contract. The read targets all four renderer units
at+20; later graphics, reflection and callback geometry paths remain unbound.

The installed verifier preserves both complete bodies,29 and294 instructions,
the copy and GetColor4fv import slots, both source argument chains, helper
guards, literal bytes and alpha comparison. Independent Ghidra12.1.4 captures
retain all323 instruction locations and the four bytes `0000803f` at1802D7AEC.
The original binary was not executed or edited, and the temporary session was
closed. Provider limitations and original evidence records remain attached.

The alpha decision at18015A904 is `COMISS(1, alpha)`, followed by `JAE` over the
write of1. Under masked exceptions, an unordered comparison sets carry, so it
also reaches that write. This is an inference from the verified instructions
and the [Intel COMISS/Jcc reference](https://cdrdv2-public.intel.com/812383/253666-sdm-vol-2a.pdf),
not observed native NaN behavior. The local model makes its masked-exception
and DAZ/FTZ-off assumptions explicit and tests the flag decision independently.
A decompiled ordered `alpha > 1` expression alone loses the unordered branch.

Every original208 stream identity, parent plot code, conditional stored-code
view and both complete raw RGBA groups is rematched to freshly parsed complete
archive bytes. Frequencies preserve stream multiplicity even for identical
payload hashes. There are three2AC encodings and one stored plot54 code. Actual
runtime mode and actual helper invocation remain unknown in every native row.
Those counts impose no field-validity or UI-meaning restrictions.

All425 owned parent/style contexts also parse through complete EOF. The pair's
wire gate is version above820005;285 owned contexts contain it and140 do not.
A standalone style has no supplied parent plot code in this evidence. Neither
that code nor a missing color pair is replaced with a guessed zero/default.
The exact historical color-report hash transition is retained separately;
every other original audit/probe/decompilation dependency must match.

The strict targeted suite passed15 checks in4.565seconds and its process exited
successfully after snapshot recording. The [evidence proof](plot54-evidence-verification.json)
records exact revision, hashes, scope, preserved unknowns and remaining gates.

## Local operation

`plot54_color_inputs()` projects optional canonical colors and parent plot-code
availability from dialog and standalone style snapshots. The immutable
`plan_masked_daz_ftz_off(runtime)` evaluates mode, plot code, copied colors,
core presence and callback availability in their reached order. Unknown facts
return typed dependency errors; known bypasses and null rejections return
completed empty local plans. Helper rejection retains both caller copies.

An admitted read retains exact original2AC RGBA and a separate alpha projection.
Values above1 and unordered values become1; negative values, signed zeros and
subnormals remain unchanged under the declared floating-point assumptions.
Original2BC remains forwarded with unknown consumer purpose. Results carry
completion, diagnostics, original inputs, runtime facts and exact local coverage.
They model neither renderer state nor callback geometry or a completed draw.

The final strict Source suite passed17 checks in5.810seconds. Seven integration
checks and a partial-pair regression passed, as did Clippy and allseven C889
regressions after sharing the verified alpha projection. Independent expectations
cover17,236 operation cases plus425 owned and276 native archive contexts.
Two fresh processes matched all1,553,884 expected bytes; eight workers matched
serial operation results. Native runtime scenarios are hypothetical inputs,
with no claim of observed native callback invocation.

The [operation proof](plot54-operation-verification.json) and
[process proof](plot54-process-verification.json) record hashes and exclusions.
The full Source gate and exact publication-cut checks remain required before
feature166 integration, alongside the remaining named schema models.
