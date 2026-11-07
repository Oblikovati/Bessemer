# Composer packed arrays

Feature [#101](https://github.com/Oblikovati/Bessemer/issues/101) continues the
parity work in #2 after the complete Composer container operation in #100.
`composer::arrays` reads, constructs and edits complete V2/V3 vector blocks and
unsigned index/length blocks. Complete scene/project support remains separate.

The reference clauses are 14.88.2 and 14.97 of the SolidWorks specification in
`../inventor-interface`. V3 blocks store the packing mode, three canonical
constant-axis flags, constant patterns, varying-axis endpoint pairs when
quantized, and interleaved packed codes. V2 blocks omit the flags and constants.
Zero vectors still require the complete header. Index blocks store only their
values; caller-supplied count and domain select the exact byte span and width.
The domain controls width without imposing an unverified upper bound on values.

`ComposerVectorBlock::from_bytes` takes a typed `ComposerVectorLayout` and an
explicit maximum wire size. `from_parts` accepts typed lane headers and exact
packed bytes; incompatible dimensions, modes and lanes fail before publication.
`ComposerIndexBlock::from_bytes` takes the caller's count/domain and maximum
size, while `from_values` constructs the selected unsigned representation.
Both readers require exact EOF and forced typed reconstruction. `to_bytes`
serializes every header field and stored value without floating conversion.

`edit_packed` and `edit_values` replace codes on a copy while retaining the
original shape and header. Results include completion, exact wire coverage,
zero stored-bit error, parent/result revision history and explicit diagnostics
for unexecuted float, geometry and native application checks. No-op edits retain
revision identity. Published values own immutable storage and are `Send`/`Sync`.
Changing a source byte buffer cannot change an accepted block.

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
independent numerical oracle. The separate value operation checks exact rounding,
subnormal/signed-zero cases and unsupported nonfinite arithmetic. A native
control-state match is not established by this
baseline. Raw vector patterns and quantized codes remain authoritative; the
wire operations do not implicitly reconstruct values. The additional explicit
[vector-value operation](../composer-vectors/README.md) implements behavior 1 of
the declared arithmetic profile and retains the original wire block.

Run `python docs/features/composer-arrays/freeze_arrays.py` to check the retained
baselines. Add `--export` to write the installed Rust probe packet. The ignored
`composer_arrays_native` test reconstructs, constructs and edits all 8,196 arrays.
`verify_array_edits.py` reopens every edited output through the actual reference
reader and requires exactly the requested first-code bit replacement, preserved
constant patterns/endpoints, exact counts and EOF. Two Rust processes must
produce identical edited bytes. Use `--write` only for a reviewed baseline update. Imports come from
an immutable copy of the entire reference Python tree; the snapshot records
source hashes and verifies module origins before removing that copy. The
schema snapshot is selected by headings because upstream line numbers change.

Owned Rust cases also check every vector truncation/extension, all index width
transitions, zero rows with complete headers, incompatible typed lanes, failed
edits, source isolation, concurrent replacements, independent edit/revert goldens
and compile-time thread-safety assertions. All-constant vectors with the largest
declared row count do work proportional to their small wire header. No expanded
floating output is allocated for those declared rows.

Remaining work includes whole scene operations, project relationships and
native application acceptance. Topology, geometry, self-intersection and
tolerance checks are outside this complete wire operation.

The maintainer-side humanizer skill was unavailable. A maintainer still needs
to review this prose.
