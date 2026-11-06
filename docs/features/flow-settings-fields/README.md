# Flow settings diagnostic fields — issue #241

Specification 14.35 defines seven diagnostic projections from record 102.
`FlowCarrier::settings_fields` retains the selected raw arrays and projects
EXTERNAL, LAMTURB, CHT, US_DIM, IT, NT and NZ in that order. Each field retains
its native label, source slot, original binary64 bits and typed result state.

Conversion truncates toward zero only when the result fits a signed 32-bit
integer. EXTERNAL tests whether that integer is zero. CHT compares the original
floating-point value with zero, so positive fractions remain true and ordered
infinities remain comparable. NaN comparisons and unrepresentable conversions
require floating-point context; the projection supplies no guessed integer.

IT, NT and NZ use defaults 6, 2 and 1 after conversion to zero, followed by the
record-7 slot-88 override. A nonzero guard produces zero. Missing record 7 leaves
these fields unresolved when their conversion is otherwise established. Invalid
conversion remains unresolved even with a nonzero guard. Coverage counts derived
fields and both kinds of missing prerequisite separately.

Absent record 102 produces no projection. A malformed selected record 102 or a
present malformed selected record 7 fails the operation. Last ordinary occurrence
selection follows the existing carrier reader. Stored arrays are never normalized
or changed by projection. Existing raw slot edits can be projected in a fresh
operation while earlier views retain their exact source checkpoints.

The independent FSF1 packet includes three source-bound native selections and
138 owned cases: 23 binary64 patterns crossed with six guard states. Values cover
signed zero, fractions, signed integer boundaries and adjacent values, infinities,
subnormals and several NaN payloads. Expected results come from the sibling
`flowsettingsfields.py`; literal tests independently pin defaults, fractional CHT,
missing guard and invalid-conversion precedence. Regeneration uses temporary
directories and verifies the native source hashes before extracting arrays.

Run `cargo test --test flow_settings_fields --test flow_fixed_fixtures --test flow_carriers`
and `python -m unittest discover -s docs/features/flow-settings-fields -p test_fields.py`.
These checks establish the documented pure projection. They do not execute native
loaders, apply runtime changes, establish engineering units or enum domains, or
project the owner-dependent U_DIM_EXT_MAX field. No global byte-coverage credit or
complete Flow interoperability is claimed. The maintainer humanizer pass remains
for review because that skill is unavailable in this environment.
