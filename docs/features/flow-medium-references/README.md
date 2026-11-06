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

This is an independent fixture baseline for complete reference/profile APIs.
It does not add a production reference-body or complete-profile reader/editor.
Complete kind-two bodies and mixed-kind profiles, populated final collections,
engineering meanings, numeric evaluation and native runtime/application
acceptance remain required work toward the documented schema parity goal.
The complete kind-one profile assumes a fresh backend and reports complete
record framing while retaining `full_engineering_schema = false`.
