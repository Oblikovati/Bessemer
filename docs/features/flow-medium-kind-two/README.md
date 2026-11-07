# Flow medium kind-two bodies

Issue: #241. Reference:
`../inventor-interface/solidworks/flowmediumkind2.py`.

Kind two shares the version-zero base, uninterpreted base byte, sixteen raw key
bytes, counted UTF-16LE name, seven curves and six shared arrays with kind one.
It then stores one raw wide scalar, a signed model tag, a version-zero model body
and a final stored state byte. Native loading sets the runtime state to one
independently of that stored byte. Raw text units, all curve flags and every
numeric pattern remain unnormalized.

| Model tag | Complete model fields after version zero |
| --- | --- |
| 0 | Two curves and one raw wide scalar. |
| 1 | Three curves, one raw wide scalar and two state bytes. |
| 2 | Three curves and one raw wide scalar. |
| 3 | Seven raw wide scalars. |
| 4 | Six raw wide scalars, one state byte and a final raw wide scalar. |
| 5 | Two counts A/B, two A-wide arrays, B-wide patterns, B raw 32-bit words, count C and C-wide patterns, signed member-8C, two wide scalars and two state bytes. |

The table model stores A and B before their arrays. All three counts are
independent: an empty first pair can precede nonempty second or third arrays.
The 32-bit words have no assumed floating-point interpretation. Curve counts,
conditional extra arrays and raw UTF-16 framing retain the same strict semantics
as the shared body prefix. A nonzero flag with zero entries still reaches the
extra-array branch.

The FMO2 packet freezes 2,492 generated views across all six tags. It covers all
base and final state bytes, every curve flag at zero/nonzero counts, every model
state byte, individual raw 64-bit bits and their complements, NaN payloads,
signed zero, infinities, subnormals and signed member boundaries. Table-model
cases cover every raw 32-bit bit and complement, independent count combinations,
empty arrays and 1,024-entry arrays. Names include zero units, stored NULs,
embedded NULs, surrogates, noncharacters and 65,536 units. Cases retain every
model/curve interval, complete stored bytes and an exact following opaque tail.

Rust reconstructs each body from the independent decoded fields, validates all
array cardinalities and tag-dependent state shapes, and compares complete bytes.
It reuses the shared BodyPrefix fixture parser and reconstruction; the previous
kind-one and full-record packets remain unchanged. Python independently rebuilds
every model, verifies packet reproducibility, rejects all compact representative
prefixes, all nonzero version bytes at reached body/model/curve positions,
unsupported tags, negative/impossible counts, invalid string modes and missing
terminators. Existing kind-one and complete-profile regression suites also run.

The typed API reads the complete first reached kind-two body with
`FlowMediumBackendPrefix::first_kind_two_body`. Its options bound body bytes,
wide words, name units and raw 32-bit words separately. Model tags are parsed
once into `FlowMediumModelKind`; borrowed curve and array views preserve raw
patterns. Coverage reports complete body framing while retaining the remaining
record as an exact opaque tail.

Existing-field edits reopen the complete carrier and preserve model boundaries,
array cardinalities and following bytes. Tests check exact no-ops, independent
expected patches, physical history and fresh reversals. The eight public types
are Send and Sync; eight workers check the entire generated corpus. Two fresh
processes reproduce the complete independent FMO2 packet byte for byte.

The first kind-two table key is distinct from the body's base key. The API
checks and skips its sixteen bytes before reading the body. Model tags, counts
and version framing cannot be changed through these fixed-shape edits.

Generated cases do not establish installed native kind-two acceptance; coverage
keeps `installed_fixture_validated = false`. Complete mixed-kind records,
engineering/model evaluation, native runtime and application acceptance remain
required work toward schema parity.
