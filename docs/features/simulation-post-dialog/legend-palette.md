# Legend palette-control evidence

Specification14.468 and C-848 bind four existing signed control words:

| Object member | Stored role |
| --- | --- |
| 8 | Raw color-mapping mode |
| C | Raw base-palette reversal flag |
| 10 | Base-palette color count |
| D4 | Raw gray-above-yield enable flag |

These object offsets are separate from the version-dependent wire spans.
The legend always stores nine fixed COGLColor slots. The base count does not
resize that wire area. The Source model reports whether the signed count is
within0..9 as a comparison with available serialized slots. That diagnostic
does not establish native runtime validity for zero, negative or excessive
counts. Raw values are never clamped or restricted to observed samples.

The regular mapper reads count10 and the color area beginning at18. Its
reversal test copies forward for zero and backward for every nonzero raw C
value, beginning at18+(count-1)*16. The requested output-level count remains a
separate argument. The complete584-instruction body is retained for provenance;
the documented control roles do not provide a complete interpolation model.

The complete36-instruction dispatcher checks D4 first. A nonzero flag calls
the gray-above-yield helper; a nonzero helper return terminates dispatch, while
zero continues. Ordinary dispatch selects the regular mapper for mode values
other than4. Mode4 dereferences the runtime plot pointer at1A8, selecting the
shell-material tail target for plot word18 equal3 and shell-thickness otherwise.
The flag alone does not establish gray output, and stored mode4 does not
establish a runtime plot object or its word18. The shell branches are direct
tail jumps; their larger bodies are outside the dispatcher extent.

The installed verifier retains both complete bodies and hashes, all620 probe
instructions, named exports, helper targets, provider constructor/copy imports,
raw-control argument chains and both decompilation pins. Independent Ghidra
captures retain the same620 locations and complete contiguous body extents.
The temporary analysis session closed; no target code executed.

[Bug #172](https://github.com/Oblikovati/Bessemer/issues/172) records one
equivalent signed-immediate spelling: Source uses `-0xc` and GNU uses
`0xfffffffffffffff4` at1800DA9CC. Qualification requires the exact original and
canonical text plus bytes `4883c1f4`. All four single-byte mutations, changed
register/immediate/operand width and an unknown address are checked. Global
normalization and the original Source probe remain unchanged.

The corpus retains all284 original rows,276 accepted contexts and eight
rejected streams with their selected-version and prefix/EOF qualifications.
Its1040 raw-word observations include276 modes, reversal flags and counts,
plus212 present D4 words. All observed modes are0 and counts5; reversal is0
in275 contexts and1 in one. All212 present D4 values are0. Missing earlier
words remain absent, and no gray or mode4 invocation is inferred. The owned
gate also retains all425 complete contexts, including twenty opaque styles.

The [evidence verification record](legend-palette-evidence-verification.json)
records the complete bindings and corpus gates. Its full Source suite passed
238checks in467.785seconds and exited successfully after snapshot recording.
Full interpolation, actual runtime color selection and the gray/shell algorithms
remain unfinished in the reference specification. No new stored-byte coverage
is claimed.

## Typed controls and local dispatcher plans

`SimulationPostDialog::legend_palette_inputs` and
`SimulationPostStyle::legend_palette_inputs` return four optional signed raw
words. The reversal and gray-enable views use zero/nonzero tests without
rewriting their source values. `base_count_within_nine_slots` is the same
supplementary comparison as the Source model; zero can satisfy that comparison
without establishing native safety. None of these controls resizes wire framing.

`dispatch_palette` returns an immutable `SimulationLegendPalettePlan` from
explicit `SimulationLegendPaletteRuntime` facts. It requires D4 first. A
nonzero flag requires a known gray-helper return; a nonzero return completes
the early-return selection without requiring a mode or plot facts. Otherwise,
the mode is required, and every value other than4 completes regular selection.
Only mode4 requires a known present plot pointer followed by its raw word18.
Unknown presence remains a typed missing-input error even when a cached word
was supplied. A known null pointer returns an unsupported-context error rather
than attempting or inventing the native dereference. Word18 equal3 selects
material; every other known value selects thickness.

The plan retains every supplied optional control and runtime fact, including
unconsumed parents. Completion distinguishes early-return, regular and tail
selection. Coverage records the exact consumed-member mask and reached runtime
facts, zero achieved local pattern error, unchanged wire state and unvalidated
native counts. Diagnostics retain the native-execution, mapping-algorithm,
gray-output, runtime-production and count-validity qualifications. The operation
models the local choice; it produces no mapped colors or argument-forwarding
emulation.

Independent expectations combine the unchanged copied Source raw-control
model with the complete byte-bound dispatcher truth table. They cover74,592
optional-control/runtime/diagnostic cases. Every425 owned and276 native
complete parent receives seven explicitly hypothetical contexts. Another2,205
complete style mutations cross raw modes, reversal flags, counts and gray
flags, preserving all nine named base slots even for negative or excessive
counts. Caller mutation, repeated getters and complete round-trip bytes are
checked without inferring actual invocation.

All seven Rust checks passed, including native contexts and eight-worker
comparisons. All eight public types satisfy `Send + Sync`. Clippy passed with
warnings denied. The strict Source suite passed19 checks in9.503seconds and
exited successfully after snapshot recording. Two fresh processes matched all
94,934 independent expected results, totaling4,177,096bytes.
The [operation record](legend-palette-operation-verification.json) and
[process proof](legend-palette-process-verification.json) retain exact hashes,
coverage and exclusions. Remaining documented feature166 models and exact
publication/platform integration remain outstanding.
Humanizer is unavailable in this checkout; maintainer prose review remains pending.
