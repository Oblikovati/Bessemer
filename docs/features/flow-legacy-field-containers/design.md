# Legacy field containers

Issue: #241. The source audit identified three documented layouts in
`../inventor-interface/solidworks/flowlegacyfields.py` without corresponding
typed record bindings in `src/flow`: records 105, 113, and 155. The audit checked
the Flow module declarations, record entry points, numeric identifiers, and
scalar-query and selection names. It is a targeted gap finding, not a claim
that other Flow requirements or other SolidWorks families are complete.

## Required behavior

Record 105 contains exactly 320 raw 64-bit patterns without a version or count
prefix. Record 113 has a nonnegative signed count followed by that many raw
64-bit patterns. Both require exact EOF and preserve every bit. The installed
reference corpus supplies two record-105 payloads and one record-113 payload.
Native conversion, engineering interpretation, and application acceptance stay
separate from complete stored-field framing.

The record-105 scalar projection uses slots seven and ten. Slot ten is nonzero
for every pattern except the two signed zeros. Slot seven follows the documented
completed comparison branches: NaNs and values in `(253, 565]` select the virtual
method branch, values above 565 have precheck status -1, and other values have
precheck status -2. No native virtual method is called and floating-point faults
or environment state are not reproduced. Independent bit classification and
adjacent encodings around both thresholds must verify this projection.

Record 155 has stored version zero followed by the same one-dimensional header
group and item grammar used inside record 315. Gate absence, empty counts,
nonzero presence and activity bytes, item versions zero through five, vectors,
subrecords, and raw pattern types must retain their distinct wire meanings.
No installed native record-155 payload has been authenticated.

Selection checks the latest record 113 first and uses the latest record 155
only when 113 is absent. A malformed selected record fails; it does not trigger
fallback. No matching records is a completed empty selection.

## Reuse and verification

The conditional fixture reader was extracted for reuse without changing its
packet grammar. Its existing 6,659 owned cases and full integration suite must
pass unchanged before raw record-155 fixtures use it. The public conditional
format enum remains unchanged; a new binding must not mislabel record 155 as
record 315 or invent a version for records 105 and 113.

Freeze the three layouts, scalar-query domain, and selection outcomes before
adding public readers and editors. Numeric edits must preserve unrelated bytes,
exact carrier history, and immutable inputs. Bounds and field identities must
be checked before allocation or access. Any production grammar extraction must
be a separate behavior-preserving change verified by the earlier owned corpus.

The humanizer skill is unavailable; the maintainer prose pass remains for review.
