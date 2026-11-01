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
records exact hashes, qualifications and remaining gates.

## Remaining operation work

Implement the eight typed routes with an explicit nullable-pointer/type context,
reusing the alpha-preserving nearest-even packed assignment. Preserve original
RGBA separately from projected RGBA, raw preference words and their source offsets.
Require missing pointer/type/source facts only when reached; a skipped block
must not require preference bytes. Retain the preceding legend-call boundary.

The local projection needs immutable completion, diagnostics, exact coverage,
all-parent provenance, source isolation, independent rounding/raw-alpha/bounds
fixtures, concurrency and fresh-process checks. It must leave serialized bytes
unchanged. The full Source and publication gates and the remaining documented
feature166 models still require their separate acceptance evidence.
