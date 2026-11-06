# Flow medium reference bodies and complete kind-one profiles

Issue: #241. References:
`../inventor-interface/solidworks/flowmediumliquids.py` and
`../inventor-interface/solidworks/flowmediumobjects.py`.

Reference bodies use an explicit external kind and the keys from the selected
first-table kind map. Both kinds start with two version-zero bytes and a
sixteen-byte key. Kind one stores a complete state byte; kind two does not.
A nonnegative signed count precedes ordered sixteen-byte references, each of
which must exist in that selected key context. Repeated references stay ordered.
The body's own key need not be a member of the first-table context. Native
loading sets kind-one runtime state to one independently of the stored byte.

The FMR1 packet freezes 2,580 views: native kind-one reference bytes with and
without their following outer-record bytes, both kinds, all 256 stored state
bytes, empty/duplicate/reordered key contexts, empty and repeated references,
large lists and arbitrary following bytes. The same 23-byte wire image is frozen
under both kinds: kind one ends at byte 23 with stored state zero, while kind two
ends at byte 22 with one opaque zero byte. Kind and key context are required
inputs rather than guesses from bytes. Kind-two views are generated; this corpus
contains no installed native kind-two fixture.

The native reference body is 39 bytes with one reference and a stored state of
one. Its original suffix contains 32 outer-record bytes. Rust reconstructs every
reference body from the frozen fields, verifies raw key sizes and selected-map
membership, preserves duplicates and appends the exact opaque tail. Python checks
packet reproducibility, all small/native prefixes, version/count boundaries,
strict external kind/key types, missing references, context isolation and native
loaded-state projection.

The FCP1 packet freezes 1,163 complete fresh-backend kind-one records. It includes
the native record, every active outer version 2–127, all stored reference state
bytes, empty tables, reference objects with empty first tables, duplicate keys in
both physical tables, repeated/reordered references, signed boundary attributes,
large reference collections and all existing rich metadata-prefix profiles.
Each first-table entry embeds the complete independent FMO1 body projection:
all seven curve arrays, raw numeric patterns, name units and exact body framing.
Every physical entry retains its ordinal, interval, kind and raw key.

Rust reconstructs every nested body field, both table headers, second-table
references, outer keys, signed attributes and zero final collection counts,
then requires exact EOF. The metadata prefix is retained as raw bytes; the
existing typed metadata API independently checks its boundary on every complete
profile. The existing first-kind-one-body API also matches every first reached
body against its nested complete projection. Python independently rebuilds the
records, rejects native/small prefixes and all appended bytes, enforces explicit
fresh-backend and string-mode context, validates membership in both tables and
rejects unsupported kinds or populated final collections.

The native record has one object in each table and one outer reference.
Metadata ends at byte 112, the backend ends at 13,159, and the complete record
ends at 13,191. Duplicate physical keys in generated records remain distinct
entries. Membership checks use the available key sets; native hash allocation,
lookup identity policy and other runtime state are not reconstructed.

These independent fixtures predate the complete kind-one profile API below.
The FMR1 corpus also supplies kind-two reference expectations for the later
mixed-kind implementation; those generated cases do not establish installed
kind-two acceptance. Populated final collections, engineering meanings,
numeric evaluation and native runtime/application acceptance remain separate
requirements toward documented schema parity.

## Complete kind-one profile API

`FlowCarrier::record_424_kind_one_profile` reads the complete supported record
through exact EOF. Options require an explicit `FreshBackend` context, metadata
and per-body limits, bounds on both physical table counts, and total budgets for
references, raw wide patterns and stored body name units. Duplicate physical
keys remain separate objects. Typed binary `FlowGuid` identities are read once
and reused for membership checks and returned references.

The immutable result retains the complete metadata/carrier checkpoint and all
body views, both table intervals, ordered reference lists and signed outer
attributes. Every reference must resolve in the appropriate fresh table. Empty
tables and reference lists are completed results; an empty first table may have
second-table objects whose own reference lists are empty. The two supported final
collection counts must be zero. Coverage reports `ExactFreshKindOneRecord`,
complete record framing and `full_engineering_schema = false`. Native hash order,
duplicate-key lookup identity, runtime state and application acceptance remain
explicitly unchecked.

Existing table keys, reference keys, reference states and outer attributes can
be edited. A key change that would leave a reference unresolved is rejected.
`apply_body_edit` composes an existing typed body operation at its own physical
object offset. `apply_metadata_edit` composes a current metadata-only edit while
requiring every backend byte unchanged. Both paths validate edit provenance and
reparse the entire record under the original options. Every result preserves
all structural intervals, complete source/result checkpoints and physical carrier
history. Exact no-ops retain identity; fresh reversals reproduce the whole input.
Counts, version framing and final collection shape remain structural.

The typed reader matches all 1,163 frozen profiles, including every nested body
field and the complete native record. Every exposed table/reference/attribute
field receives an exact no-op plus either an independently predicted valid patch
and reverse or an independently predicted unresolved-key rejection. Every body
in every profile receives a composed edit and reverse at its own offset.
Metadata edits compose through every stored metadata object while preserving
the complete backend. Tests check global exact/one-below/zero limits, exhausted
name budgets followed by zero-unit names, all native/small truncations, every
appended byte, unsupported kinds and versions, negative counts, missing keys,
foreign/stale/misdirected edits and input isolation.

Nine public profile types are asserted Send/Sync. Eight workers match complete
reads and edited carrier bytes for every profile after caller input mutation.
Two fresh processes reproduce the full independent FCP1 packet byte for byte,
including all nested FMO1 fields, reference occurrences and exact EOF. The shared
body parser's validated-offset extraction was verified separately with all
46 prior regression tests and unchanged owned fixtures.

The complete kind-two/mixed-kind record profile is not implemented here. This
API's explicit kind-one scope must not be treated as complete record-424 parity.
