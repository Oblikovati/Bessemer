# Preference color routes and legend initialization

Sections14.523–14.525 bind the legend100 initialization assignment and eight
conditional PostStyle preference-color routes. Original serialized RGBA remains
authoritative. Initializer values and projected preferences do not become wire
defaults or allowed-value restrictions.

`CDspLegend::Initialize` at1800DC530 calls the three-argument color constructor
with RGB0,0,1, then copies allfour words to legend100. The complete constructor
explicitly sets alpha to1; the copy assignment preserves allfour source words.
The initializer's one constant is retained as exact bytes0000803F. Its display
purpose remains unresolved. Fresh canonical replay rematches all109 original
legend observations:101 opaque blue and8 opaque white. Both values remain
unchanged, and the observed pair does not define a universal value domain.

`SetDefaultOptionByPreferences` at180045050 calls the legend preference setter
before reading the pointer at2D0. A null pointer admits the local style block.
A nonnull pointer with pointed type73 skips that block. Other known types admit
it. A local skip therefore does not establish that the earlier legend call was
skipped, nor do stored color bytes establish the applicable runtime context.

| Style member | Packed preference offset | Alpha preference offset |
|---|---|---|
|220 |1578 |157C |
|1BC |1588 |Preserved original |
|1CC |158C |Preserved original |
|1DC |1590 |1594 |
|2AC |15A8 |15B0 |
|2BC |15AC |Preserved original |
|1F4 |15D0 |15C8 |
|20C |15CC |15C4 |

These are in-memory offsets, not additional wire positions. All eight calls use
the packed assignment overload already bound in C906. Five routes separately
replace alpha with a raw preference word; three retain the original alpha,
including arbitrary NaN or signed-zero patterns. Constructor alpha initialization
cannot substitute for that assignment behavior. The documented numeric view
requires explicit nearest-even arithmetic and actual source bounds only when
the local block applies. It does not emulate the whole setter or resolve the
unresolved colors' display purposes.

The installed verifier checks all194 instructions across the two complete
callers, exact import/export slots, all eight argument chains and five alpha
load/store pairs. It retains complete constructor/copy-assignment bytes and the
80-byte packed-assignment body and scale. Independent Ghidra12.1.4 captures match
all194 caller locations and complete contiguous body extents. The original binary
remained unchanged; no target code executed and the temporary session closed.

All425 owned and276 native parent/style contexts retain complete EOF within
their original accepted scopes. Twenty older owned styles retain an opaque region
without inventing named route colors. Other version-conditioned absences remain
explicit, including1CC and2AC/2BC. Native preference bytes and actual invocations
remain unknown. Candidate-version, modern-prefix, legacy acceptance and rejection
qualifications remain unchanged. These observations add no newly decoded bytes.

The strict targeted suite passed18 checks in5.785seconds and its process exited
successfully after snapshot recording. The [verification proof](preference-color-evidence-verification.json)
records exact hashes, qualifications and remaining gates. The full Source suite
passed212 checks in434.935seconds and its process exited successfully.

## Local preference-color operation

`SimulationStylePreferenceColorRoute::from_style_member` normalizes the eight
bound members once and exposes their canonical field paths and source offsets.
`project_style_preference_nearest_even` takes an original `SimulationRgbaColor`,
the route, optional borrowed preference bytes and explicit pointer/type facts.
Unknown pointer presence fails before interpreting a supplied type. A known null
pointer admits the block even when an unused cached type says73; a known nonnull
pointer requires its type and skips only for exactly73.

Skipped plans preserve allfour original words without requiring source bytes.
Admitted plans validate both reached source extents, reuse alpha-preserving
nearest-even packed assignment and apply the five raw alpha overrides separately.
Results retain original/projected RGBA, every consumed packed/alpha word, source
offsets and availability, runtime context, completion, diagnostics and exact local
coverage. Borrowed bytes are not retained. The preceding legend-call boundary
remains explicit, and no wire bytes change. A projection whose bits already
match the original remains a completed projection, with its source reads recorded.

Independent expectations cover17,400 route/context/bounds/rounding/alpha cases.
All425 owned and276 native contexts preserve their complete retained bodies and
conditional route absences.2,457 owned and1,698 native route colors each receive
four explicitly hypothetical scenarios;1,453 unavailable routes remain absent.
No preference contents or invocation are inferred from native stored colors.

All eight Rust checks and Clippy passed. Eight workers matched serial results;
two fresh processes matched all2,584,032 independently expected bytes. The strict
Source suite passed20 checks in7.120seconds and exited after snapshot recording.
The [operation proof](preference-color-operation-verification.json) and
[process proof](preference-color-process-verification.json) retain exact scope,
hashes and exclusions. The full214 Source gate, exact publication checks and
remaining documented feature166 models still need their acceptance evidence.
