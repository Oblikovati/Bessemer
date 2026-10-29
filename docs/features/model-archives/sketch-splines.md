# Direct sketch splines

Feature #44 now reads the complete documented direct spline body. It retains
the point-list class, raw flags, parameter/control-point arrays, tangent modes,
interpolation records, optional equations and transform, control-list tolerance
or curve branch, fixed-length references and optional kernel frame.

The three declared point-list classes reuse the typed list reader. New objects
and aliases resolve their stored compact/extended count through operation-local
fields. Missing or wrong-typed body facts fail instead of defaulting to an empty
list. Null lists are accepted only for the documented inactive entity state.
No unimplemented fixed-length target body is treated as an empty object.

Primary interpolation distinguishes the four-byte sentinel from an ordinary
zero count, which still stores its width and two doubles. Counts, width products
and total array bytes are checked before iteration. Secondary interpolation
preserves its signed width without applying the primary record's width rule.
Curve headers accept dimensions 3/4 or a completely zero header; the periodic
word remains raw while selecting the documented one-value branch.

The independent packet contains 934 cases and 86,266 scalar expectations. It
covers all supported subtype masks, three point-list classes and aliases,
inactive null lists, interpolation shapes, secondary records, equations,
tolerance/curve branches, suppressed bodies and typed compressed bodies.
Every case reconstructs exactly and every truncated prefix/trailing byte fails.
An edit matches separately assembled, reference-validated bytes and leaves its
source unchanged. Source-overwrite checks cover every case, with parallel
encoding of all twelve framed kernel cases.

The populated owned spline uses 29 aggregate array entries: 29 passes and 28
fails. Malformed nested counts, unsupported masks and null active lists fail.
Twelve compressed kernel cases require explicit schema context; twelve
suppressed cases complete without it. Kernel frames use the existing fully
typed codec, not an opaque byte fallback. The scalar oracle excludes the derived
interpolation-count duplicate and kernel nodes; suppression remains covered and
the existing separate kernel-value oracles remain in force.

The signed-count reader and new unsigned counts share the same checked extent
logic. Existing text arrays reuse the extracted scalar/vector helpers. Earlier
wire fixtures remain the baseline for these changes. The spline fixture schema
is created once per exhaustive-prefix test and shared immutably between its
independent decoders; there is no global parser state.

All twelve spline tests, all six referenced-field tests, all 215 model tests,
all-target Clippy and all 51 Python tests passed. Three new wire fixtures
reproduced exactly in fresh processes. All 906 native outcomes matched the
preceding run. Results and exact
source/fixture hashes are bound in `sketch-spline-verification.json`. The full
sketch root still needs dimension collections, patterns and its tail, so native
acceptance remains 866/906 and its complete gate still fails. Geometry/native
application acceptance and public model result/history contracts remain open.
The full PR gate belongs to the earlier geometric/kernel source state and must
rerun before push. Humanizer was unavailable; prose remains for maintainer review.

Behavior version 2 corrected the equation transform to thirty fixed u32 words.
The [recorded migration](../spline-equation-transform/README.md) preserves the
former fixture hashes and verifies every unaffected case unchanged (Bug #95).

The exact-mask 0x80 prototype and its true secondary callback are covered in
[the single-span evidence](../single-span-splines/README.md) (#54).
