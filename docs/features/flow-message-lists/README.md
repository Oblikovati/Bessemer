# Flow record 301 message lists

Issue: #241. References: `../inventor-interface/solidworks/flowrecord301.py`,
`flowmetadataprefix.py` and specification section 14.74.

The FM31 packet freezes 992 complete record-301 payloads before the typed
read/edit API is added. Versions zero through three store a row count and raw
member words, a primary string and an independently counted first string list.
Version three adds a separately counted second list. Earlier versions omit
that list and its count entirely; omission remains distinct from stored empty.

Versions zero, two and three require explicit integer string mode zero, even
for empty records. Each string stores a nonnegative UTF-16 unit count. Positive
counts require a final NUL unit. The complete raw units remain intact, including
unpaired surrogates, BOMs, embedded NULs and hidden suffixes. The native text
projection stops at the first NUL without normalizing the stored units. Zero
counts store no terminator and project empty text.

Version one uses a different transfer: stored count N consumes exactly N raw
bytes. Native allocation of twice that size does not change the wire transfer.
The bytes may have odd lengths or contain arbitrary values. Text interpretation
stays unset, including for empty and apparently valid UTF-16 byte strings.
Unused string-mode inputs are ignored by this branch.

The packet retains every field, root/item/list/string interval, structural count
offset and raw text prefix projection. It includes all supported versions,
independent list shapes, multiple and empty rows, 1,024-string lists, all 65,536
UTF-16 units, all 256 legacy byte values, distinct NFC/NFD representations and
raw member-word single bits and complements. No production API is introduced
by the baseline.

The audited native FLD payload is version three with one row and exactly 328
bytes. Member pattern four accompanies the primary message “A vortex crosses
the pressure opening”, one first-list string and one second-list string. These
observations do not establish an enum for the member or engineering meanings
for either list. The mode-zero assumption does not verify native owner context.
Source, whole-file and payload hashes are recorded in `baseline-verification.json`.

Run `python -m unittest discover -s docs/features/flow-message-lists -p test_messages.py`
and `cargo test --test flow_message_list_fixtures`. Python regenerates the
packet in a temporary root, independently rebuilds every payload and rejects
compact truncations, all extensions, invalid counts, modes, versions and wrappers.
Rust rebuilds every field and checks scopes, counts, exact transfer lengths,
optional text projections, native endpoints and bit-pattern coverage.

Legacy string interpretation, runtime object capacity, engineering meanings
and SolidWorks application acceptance remain outside this raw framing baseline.
