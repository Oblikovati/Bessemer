# Composer packed arrays

Feature [#101](https://github.com/Oblikovati/Bessemer/issues/101) continues the
parity work in #2 after the complete Composer container operation in #100.
This directory currently retains reference evidence for the next implementation.
It does not add a Bessemer array API or claim complete scene/project support.

The pinned clauses are 14.88.2 and 14.97 of the SolidWorks specification in
`../inventor-interface`. V3 blocks store the packing mode, three canonical
constant-axis flags, constant patterns, varying-axis endpoint pairs when
quantized, and interleaved packed codes. V2 blocks omit the flags and constants.
Zero vectors still require the complete header. Index blocks store only their
values; caller-supplied count and domain select the exact byte span and width.
The domain controls width without imposing an unverified upper bound on values.

`freeze_arrays.py` constructs 54 owned vectors directly from that field order:
all three modes, zero/two rows, all eight V3 flag combinations, and V2. Constant
patterns include a signaling NaN payload, negative zero and negative infinity.
Twelve index cases cover empty/populated arrays and domain transitions at
255/256 and 65535/65536. Actual frozen reference readers must consume every
case completely. Checks reject 75 malformed vectors, every truncated populated
index span and every index extension, including an extension to an empty block.

The installed audit requires all 38 original carrier hashes and complete decoded
geometry framing before visiting arrays. It retains scene-order, length-prefixed
wire hashes for 4,367 vectors and 3,829 index blocks, totaling 44,629,425 bytes.
Compressed-member and decoded-geometry hashes bind those results to their inputs.
No apparent byte tags are searched inside arrays.

Floating-point projections are recorded separately under the reference's
`nearest-even-gradual` profile. These hashes are reference expectations, not an
independent numerical oracle. Implementation acceptance still requires exact
rounding checks, subnormal/signed-zero cases and explicit handling of unsupported
nonfinite arithmetic. A native control-state match is not established by this
baseline. Raw vector patterns and quantized codes must remain authoritative.

Run `python docs/features/composer-arrays/freeze_arrays.py` to check the retained
baselines. Use `--write` only for a reviewed baseline update. Imports come from
an immutable copy of the entire reference Python tree; the snapshot records
source hashes and verifies module origins before removing that copy. The
schema snapshot is selected by headings because upstream line numbers change.

The next gate is a complete read/construct/edit operation with explicit caller
context and resource limits, forced reconstruction, immutable published results,
owned goldens and a Bessemer comparison against these installed arrays. Scene
semantics, topology/geometry validity and native application acceptance remain
separate requirements.

The maintainer-side humanizer skill was unavailable. A maintainer still needs
to review this prose.
