# Complete load-state read and edit operation

Require a typed, explicit load-mode context. Do not infer a native owner or mode
from a matching extent. Bind field and scope identities to that context as well
as the carrier occurrence and revision: an empty record can parse under both
modes with identical bytes while retaining different caller assumptions.

Implement version 18 through exact EOF. Keep prefix inspection separate from a
complete table: the original native payload under true mode has a known
incomplete prefix, not a successful complete read. Preserve unknown versions
through the existing raw carrier; do not assign them invented field layouts.

Preserve the distinction between count zero and a positive count with width
zero. The latter has four present empty arrays per item and four present empty
shared arrays, followed by final members and the mode-selected tail. An empty
record does not consume vector budgets merely because its unused stored width
is large. Bound bytes, items, descriptors, string units, actual array values and
all stored values independently before allocation or iteration.

Use the shared carrier history, raw-version wire path and checked scalar codecs.
Keep UTF-16 units as a separate typed representation instead of adding a variant
to an existing public scalar enum and breaking exhaustive callers. A text view
must retain all stored units and separately project the first-NUL prefix.
Unpaired surrogates and missing terminators remain accepted raw values. A UTF-8
convenience conversion cannot silently replace those units or normalize text.

The first complete operation must read and edit every existing nonstructural
scalar, raw wide/word value and UTF-16 unit. Version, count, width and string
length fields remain structural. Reparse each proposed edit, retain complete
field/scope layout and recompute the text prefix from the changed raw units.
No-ops retain the original wire and checkpoint; raw carrier edits must reopen
and reverse byte for byte while preserving every unrelated occurrence and full
physical history. Test success, failure, stale/foreign/context-mismatched IDs,
input destruction, concurrency and separate-process replay against the complete
owned expectations before publishing the API.
