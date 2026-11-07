# Raw style initialization and preferences

Specification section14.503 and C-885 bind initialization and preference
provenance for twelve raw PostStyle words. Later clauses add consumer roles
without changing these raw assignments. Their initializer values are:

| Member | Initial value |
|---|---:|
| 1F0 | 0 |
| 204 | 0 |
| 208 | 0 |
| 230 | 1 |
| 234 | 0 |
| 238 | 0 |
| 23C | 0 |
| 240 | 1 |
| 28C | 200 |
| 290 | 50 |
| 294 | 0 |
| 298 | 1 |

These values describe local assignments in `CDspPostStyle::Init`. They are not
file defaults, valid-value restrictions or proof that initialization preceded a
particular load. An eight-byte store is split into two little-endian words:
the store of1 at230 also writes0 at234; the store of50 at290 writes0 at294.

Four raw transfers connect distinct runtime structures:

| Style member | Source in preference | Destination in plotAdvPreference |
|---|---|---|
| 240 | 152C | 04 |
| 230 | 1574 | 4C |
| 204 | 15B4 | 8C |
| 208 | 15BC | 94 |

`SetDefaultOptionByPreferences` invokes its legend helper first. Its local style
transfer block is skipped only when the runtime pointer at2D0 is nonnull and
the pointed word at10 equals73. A null pointer or another word enters the local
block. The earlier legend call still occurs on the skip path.

`GetPreferences_Adv` also invokes its legend helper before the local exports.
`CopyPreferences` calls its own legend helper, then clears both234 and238 when
the runtime pointer is nonnull and its word equals70 or73. Otherwise it copies
two supplied stack words into those members. The inherited helper behaviors
remain separate from these local assignments.

`SimulationStyleRawWords` exposes missing serialized fields explicitly,
preserves arbitrary word values and models only the documented local effects.
For example, stored204=1,230=0 and28C=50 are observed counterexamples to the
initializer values. Smaller native denominators reflect versioned presence,
not values to fill from initialization. Parent and PostStyle `raw_style_words()`
queries use their canonical snapshots and leave all archive bytes unchanged.

`initializer_words()` returns the twelve explicit assignments. It does not
implement a default or fill fields during decoding. `after_local_preference_set`
applies only the four raw transfers above, and `after_local_preference_copy`
applies only the234/238 assignments. Both return a new value. Their pointer
argument is explicit: `None` means a known null pointer, while `Some(word)`
means a nonnull pointer with that raw signed word. It never means unknown state.
`local_preference_export()` requires the four exported words to be known and
returns a typed missing-field error otherwise. Other named style transfers and
inherited legend behavior require their own composition before whole preference
methods can be considered implemented.

The evidence replay binds complete method bytes and instruction lists using
the existing GNU disassembler. `BESSEMER_OBJDUMP` can select its executable.
The audit records the executable hash and verifies contiguous decoded bytes;
it adds no project dependency and executes no SolidWorks code. A redundant40
REX prefix is normalized only in decoder presentation and remains in the
verified instruction bytes.

The evidence checks cover all276 original native observations and complete425
owned/276 native parent and style contexts. A restricted test oracle traces
only the bound pointer-load, test, comparison and jump instructions to each
local block entry. It rejects unsupported instructions and looping traces.
The fixtures cover443 pointer contexts, including70/73 and their neighbors,
across26 optional-field profiles:11,518 complete raw-word cases. Only73 skips
the setter block;70 and73 reach the zero-pair block. Runtime and publication
validation results are recorded separately from these static/model checks.

Seven Rust tests, all-target Clippy and all144 integrated reference checks
passed. Eight workers matched serial transitions and all425 owned archive
views. Two fresh processes matched all12,219 policy/archive contexts and their
745,963 output words against the independent expectations byte for byte.
[Verification](raw-preferences-operation-verification.json) records the exact
evidence, exclusions and remaining publication and composition gates.
