# Fixed valued-dimension archive transforms

Bug #86 / clause 11.353 / C-395 corrects the shared valued-dimension reader.
Every nonzero u16 plane-transform presence selects the fixed archive operator:
exactly thirty little-endian u32 words, followed by the existing derived suffix.
The reader reuses the strict fixed-transform decoder and rejects perspective bits
`0xF0` in word 28. All other bits, every presence word and all suffix identities
are retained. Compact restore is a distinct encoding and has no fallback here.

The native call chain and 30-word/4-byte loop independently establish the width.
The new fixed-form regression previously failed with the false diagnostic
`map index 7505; expected < 3`. It passes with the corrected framing.

Independent fixtures cover 216 complete G1/G2/G3/offset graphs across nine
versions, 16,645 primitive observations and 28 owned streams. They retain
noncanonical presence words, unsigned high bits, shared entity handles, distinct
equal roots, repeated roots and four isolated raw-word edits. Tests check all
independent bytes, fields and map slots, every truncated prefix, appended bytes,
old contexts, all four unsupported perspective bits and input isolation after
accepted/rejected document edits.

## Fixture migration: C395-fixed-valued-v1

The earlier synthetic corpus used compact transforms at this call site. Its
original packet and source/hash report remain frozen as negative replay evidence.
The corrected decoder rejects all 104 affected rich vectors. The remaining
positive corpus keeps all 632 cases; independent fixed-form observations replace
the erroneous compact projections, increasing its oracle from 19,916 to 21,164
fields. No native stream, count budget or rejection invariant was weakened.
The 442 pre-correction builder hashes also remain unchanged and are checked
through an explicitly named fixture-only compact replay.

Transform semantics remain unknown. Historical/internal contexts and whole-schema,
geometry or application acceptance are not established. Humanizer is unavailable;
maintainer prose review remains open.

Run `cargo test --lib fixed_valued_transform_tests`,
`cargo test --test resolved_fixed_valued_transforms` and `cargo xtask pr`.
Regenerate against a stable source tree with
`python -B -X utf8 -m unittest discover -s docs/features/fixed-valued-transforms`.

The local PR gate passed with 823 library tests, all integrations and 274 doctests.
Two fresh native processes agree on all 906 statuses and causes. Cases 7 and 10
now reconstruct completely, raising the accepted set from 812 to 814 without
losing an earlier success. The aggregate assertion still fails explicitly for
92 unsupported cases. A verified immutable source snapshot independently checks
all 814 accepted EOFs and four stream hashes across 54,231,178 feature
bytes. All four recorded Plastics configurations complete; their thirteen fixed
transforms retain every presence word and all 390 raw words. Reports are retained
in `frontier-checkpoint.json`, `accepted-reference.json`,
`native-transform-reference.json` and `local-verification.json`.
