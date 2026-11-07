# Sketch isocurve information

Feature #54 implements the exact `moSkIsoCurveInfo_c` body from clause 11.344 /
C-386. In ordinary external-XML parts above version `0x27CB`, its empty object
prefix is followed by twenty bytes: f64 ratio, three u16 words, an i32 dimension
ID and the final u16 hole word. The signed ID starts at unaligned wire offset
`0x0E`; runtime member `+14` is serialized last. Native DB-key labels add no
CString fields to this ordinary body.

All values remain raw. Ratio bits retain signed zero, NaN payloads and infinities;
unsigned words and the signed ID retain their full widths. No enum mapping,
ratio domain, boolean normalization or reference semantics is inferred. The
existing external-entity isocurve pointer retains its singleton role, stored
nulls and actual shared-map identities.

Independent fixtures cover 162 complete graphs, 3,942 primitive observations and
63 owned streams. Nine raw-value sets cross three supported versions and six
standalone/parent identity styles. They include fresh/cached information,
distinct roots, repeated first objects, null parent fields, shared parents,
signed/unsigned extrema and nine isolated ID-edit goldens. Actual complete
reference readers verify EOF, primitive partitions, map slots and identities.

Seven private tests cover the native regression, every independent byte/field,
every prefix, extra bytes, old contexts, exact twenty-byte size, absence of
owner/string fields, unaligned ID order, exact ratio bits and unrelated cached
parent references. Three public tests verify complete document reconstruction,
isolated edits and input isolation after accepted/rejected edits.

Run `cargo test --lib sketch_isocurve_information_tests`,
`cargo test --test resolved_sketch_isocurve_information` and `cargo xtask pr`.
Regenerate against stable approved sources with
`python -B -X utf8 -m unittest discover -s docs/features/sketch-isocurve-information`.

Historical/internal and DB-key modes, numeric meanings, geometry validity,
application acceptance and whole-schema parity remain outside this evidence.
Humanizer is unavailable; maintainer prose review remains open.

The local PR gate passed 838 library tests and 274 doctests. Two fresh native
processes matched all 906 outcomes and causes: complete reconstructions rose
from 816 to 817, with no prior success lost. The remaining 89 inputs failed
explicitly; both aggregate corpus commands therefore exited 101. Independent
source readers verified EOF for all 817 accepted configurations and all 69
roots of the native isocurve witness, including fourteen twenty-byte bodies.

The [latest wire-contract audit](latest-schema-audit.json) checked clauses
11.1679–11.1683 against the existing reader and independent fixtures. The new
producer and consumer evidence preserves the current wire behavior: raw f64
ratio bits, four unsigned-short words and a signed dimension ID. In particular,
FFFF remains 65535 and reference mode 65535 is distinct from mode one. Native
creation, point projection, spline rebuilding and cache refresh do not become
load-time normalization rules. The complete integrated local gate retained
under bitmap appearance includes the existing seven private and three public
IsoCurve tests. Runtime geometry workflows and broader parity remain open.

A fresh frozen-Source replay passed both fixture tests against the current
upstream readers. It regenerated the complete graph/owned-edit packets and
matched their existing expectations. The file, import and origin guards also
passed, and the process exited successfully. This confirms the existing wire
contract; it does not execute the newly described native geometry workflows.
