# Complete owned part operation

The private ordinary external-XML part codec reaches exact EOF for the owned
24,028-byte `Contents/Config-0` at model version 19000. All bytes are reconstructed
from typed fields, shared object records and transforms. There is no saved
source stream or unread tail in the parsed result. Header imports, version and
saved lightweight node/part flags supply the explicit parsing context.

Construction derives both section lengths from typed fields, encodes the result,
reparses it with the same header and limits, and compares paths, value types and
resolved object identities. Forged paths and missing/duplicate section markers
are rejected. Changing `Part1` to `Bessemer Ω` matches the independently generated
24,038-byte fixture: the lightweight body grows from 2903 to 2913 bytes and the
resolved body remains 21,103 bytes. The reference parser also reaches EOF for
that edited fixture and reports the expected name.

The document test replaces the configuration stream, writes and reopens the
container, and reads the edited model again. The header and every unrelated
stream remain equal. Restoring the original typed fields and stream reproduces
the original document bytes. Failed construction leaves the original snapshot
usable. Input overwrite and four concurrent writes check ownership and stable
serialization; the private parsed type has a compile-time Send/Sync assertion.
These checks do not replace ThreadSanitizer or the platform matrix.

| New record suite | Cases | Independent scalar values |
|---|---:|---:|
| Features, environment and all model light kinds | 24 | 1,768 |
| Views and compact-transform variants | 40 | 1,780 |
| Post-view, array, layer, reference, shared and part tails | 30 | 1,741 |

Each case reconstructs exactly and rejects every shortened prefix and added
trailing byte. The native environment spans 20,501..21,367, views span
21,367..23,379, and the remaining tails span 23,379..24,028. Native environment
and views also match 158 and 292 independently captured scalar values. Populated
maps, GUID records, mass properties, sheet-metal data, layer masks, duplicate
keys, noncanonical presence values and version boundaries have synthetic cases.
The old-version post-view cases have empty visual tables; the populated visual
record is deliberately paired with its actual model version 19000.

The display light payload, vector, visibility and compact-transform code is
shared with model parsing. Existing display type paths remain reexports.
`verify_shared_codec.py` compares the old display light branch against the
frozen candidate after explicit adapter normalization and separately verifies
visibility-only moves for both transform modules. It excludes the previously
extracted user-property functions from the light-body comparison. The dedicated
display light and transform suites passed. The full PR gate passed with 118
library tests and 204 doctests. After three small private helper extractions,
formatting, all-target Clippy and all 112 model tests passed again.
`owned-part-verification.json` binds both source states and the exact cleanup.

This is the first owned operation required by #44. The 906-configuration corpus,
additional nested runtime classes, explicit operation completion/coverage/history,
configuration selection, percent-unit assignability, native application checks
and public model API remain open. New transient view-list objects fail explicitly
until their helper-state body context is verified. Assembly, drawing-model and
separate resolved-feature streams remain in #2's scope. No version, release,
additional branch or PR is included. The humanizer skill is unavailable; the
prose pass remains for maintainer review.
