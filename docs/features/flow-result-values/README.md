# Flow paired result values

Issue: #241. Reference: `../inventor-interface/solidworks/flowresultvalues.py`.

Record 501's documented profile is an exact zlib stream containing byte version
two. It has no stored object count: complete version-one record-500 definitions
supply physical object order and signed identifiers. Every definition must be
IMS version two with allocation byte zero. The reference requires an explicit
fresh-object assumption; it does not support loading into reused native objects.

Each value body has signed version three, a raw first flag and optional shared
array, a mandatory main shared array, a raw BL flag and optional BL rows, then a
raw last flag and optional trailing shared array. Any nonzero flag means present,
including 128 and 255. Present-empty bodies remain distinct from absence. Shared
arrays require marker one, an unsigned count within the signed native-transfer
range and raw 64-bit patterns. BL version zero stores an unsigned row count and
exactly 18 raw-wide patterns per row. Complete decoded EOF is required.

The owned FIV1 packet contains 3,798 paired cases and 14,846,265 uncompressed
bytes. It retains both stored payloads, both complete decoded bodies, ordered
definition identifiers, all value intervals, exact flag bytes, array presence,
every BL row and explicit fresh/runtime/engineering qualifications. Generated
cases cover every raw bit and complement in all three array positions and all
18 BL columns, all flag bytes, NaN/infinity patterns, duplicate identifiers,
empty and multiple objects, array lengths through 65,536, BL row counts through
4,096 and compression variants. Definition-array counts and value-array counts
are deliberately varied independently; their engineering relationships are not
established by this profile.

The native pair uses record 500 from the installed CFLD carrier and record 501
from the installed FLD carrier. These are separate authenticated sources, paired
explicitly by the native reference manifest. The values payload has 337,770
stored and 592,076 decoded bytes. It contains 4,056 main values, 180 trailing
values and 3,876 BL rows: 74,004 raw-wide patterns in total. Its first flag is
zero; BL and last flags are one. The freeze checks both original source hashes,
physical record extents, payload hashes and full normalized projections.

Rust independently rebuilds every decoded value byte, verifies complete zlib
consumption and checks the typed record-500 context against the frozen paired
identifiers. Python checks independent extent equations, strict context and
fresh-object requirements, malformed counts/versions/markers, framing failures,
all flag/pattern coverage and deterministic regeneration under a temporary root.

```text
python -m unittest discover -s docs/features/flow-result-values -p test_values.py
cargo test --locked --test flow_result_values_fixtures
```

The typed reader requires `FlowImsValueContext`, created from complete typed
record-500 definitions and `FlowImsFreshObjects::FreshDeclared`. Context creation
checks IMS version two and allocation byte zero for every physical definition.
It retains the exact definition revision; separately parsed equivalent bytes
produce a distinct context. A new view explicitly pairs its carrier with a
context, while edits retain the original pairing and all its definition parents.

```rust,ignore
let context = FlowImsValueContext::new(&definitions, FlowImsFreshObjects::FreshDeclared)?;
let values = carrier.record_501_ims_values(&record, &context, FlowImsValueOptions {
    maximum_decoded_bytes: 2_000_000,
    maximum_objects: 100_000,
    maximum_shared_values: 1_000_000,
    maximum_bl_rows: 100_000,
    maximum_bl_values: 2_000_000,
})?;
let object = values.object(0)?;
let main = object.array(FlowImsValueArray::Main);
let edit = values.replace_flag(0, FlowImsValueFlag::Bl, 255)?;
```

Views expose object order, inherited signed identifiers and retained definitions,
complete body/array/flag intervals, raw flags, optional arrays and complete BL
rows. The main array is always present. Empty contexts admit exactly one decoded
version byte. Independent zero shared-value and BL row/value bounds still admit
present-empty bodies. Counts are checked against their native transfer range,
configured bounds and remaining complete elements before multiplication or
metadata allocation.

Editors replace existing shared or BL patterns and raw flags within the same
zero/nonzero class. Presence transitions are rejected because they need complete
body insertion/removal. Every edit copies the value carrier, reparses under the
same retained context and bounds, checks exact decoded bytes and unchanged
layout/totals/context, and returns `FlowSimpleTableEdit` checkpoints and complete
physical history. No-ops preserve stored compressed bytes. Fresh reversals
restore decoded bytes without promising the original compression encoding.

Typed checks read every owned raw pattern. The edit matrix covers all shared
values in arrays up to 257 entries and all BL columns in bodies up to 16 rows;
larger arrays and BL bodies use distinct first/middle/last positions. All native
values are read, with 240 native field positions selected for edits. Zero flags
receive exact no-op checks and rejected presence-transition attempts. An extra
three-object case with duplicate identifiers verifies aggregate shared/BL bounds.
Eight workers match serial mixed edits, and two fresh processes compare both
parent carriers, coverage and selected edit outputs byte for byte.

Digests and focused results are recorded in `baseline-verification.json`,
`local-verification.json` and `api-verification.json`. The observed BL
state-byte-after-load value zero is not serialized and does not establish native
execution. Native runtime mutation, nonfresh loads, engineering columns and
application acceptance remain unverified.

The humanizer skill is unavailable; maintainer prose review remains pending.

The combined local PR gate passed on source `a4da7848`: 5,096 tests passed,
none failed, and 265 were ignored. See the [gate evidence](../flow-control-records/full-pr-gate-verification.json)
for the exact revision and exclusions. Hosted matrix acceptance of that snapshot
remains pending.
