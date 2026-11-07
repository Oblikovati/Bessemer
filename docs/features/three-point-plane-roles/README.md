# Three-point plane reference roles

Bug #46 includes native case 281: an already implemented three-point plane body
rejects a complete endpoint at `point_reference_88`. A new regression reproduces
that failure before the fix at revision `18776bd78f29cdda8938b0bebef03637c3417d71`.

Clause 11.190 / C-218 declares three separate `moPointRef_w` pointer occurrences,
without extra counts, presence words or direct coordinate bodies. The reviewed
endpoint and loft-sync ancestry in 11.164 / C-192 and 11.204 / C-232 establishes
the same finite point family already used by other plane fields. All three
three-point fields now reuse that existing family. Nulls remain permitted;
unrelated fresh wrappers and cached arrays remain invalid. Every class body,
map identity and physical field remains unchanged.

Independent fixtures cover 78 complete graphs across thirteen versions,
10,084 primitive observations and seven owned streams. They include all-null,
base-point, fresh/cached endpoint and loft-sync targets, three distinct mixed
point classes, complete containing planes and repeated data/parent identities.
Two additional negative witnesses contain complete fresh/cached unrelated
wrappers. The source parser consumes those bytes to EOF but ignores expected-class
mismatches; native ancestry and Rust rejection establish the role contract.

Six private tests cover exact fields/bytes/map slots, every truncated prefix,
appended bytes, old context, all three pointer occurrences, nulls, actual cached
identity and unrelated targets. Three public tests cover complete document
reconstruction, an independently assembled endpoint-coordinate edit shared by
all three fields, and source isolation after success and rejection. Existing
plane-point corpora and earlier class bodies remain unchanged.

Run `cargo test --lib model_archives::three_point_plane_role_tests`,
`cargo test --test resolved_three_point_plane_roles` and `cargo xtask pr`.
Independent regeneration uses
`python -B -X utf8 -m unittest discover -s docs/features/three-point-plane-roles`.

The local PR gate passes with 780 library tests, all integrations and 274
doctests. Two fresh native processes agree on every status and cause across all
906 inputs. Case 281 now reconstructs completely, raising the accepted set from
800 to 801 without losing any earlier success. The aggregate assertion still
fails explicitly for 105 unsupported cases.

A verified immutable 1,429-module source snapshot independently checks all 801
accepted EOFs and four original stream hashes, covering 51,614,897 feature bytes.
Case 281 contains one three-point data object whose ordered point classes are
endpoint, base point and base point; all three objects are distinct. Its full
stream hashes, EOF and actual point identities are retained in
`native-case-281-reference.json`. The source's permissive class argument remains
separate from the native ancestry and negative role tests.

The change asserts no point distinctness, coplanarity, coordinate-system semantics
or engineering validity. Historical contexts, application acceptance and full
schema parity remain open. The humanizer skill is unavailable; maintainer prose
review remains open.
