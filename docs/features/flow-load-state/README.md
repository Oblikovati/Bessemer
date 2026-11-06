# Flow record 11 load state

Issue: #241. Reference: `../inventor-interface/solidworks/flowrecord11.py`.

This baseline covers the complete raw version-18 grammar under an explicit
boolean load-mode context. It preserves the item count, signed root member,
array width, two names per item, nine ordered signed members, two states, raw
wide members, four item arrays, independent extra words and a description.
Nonempty records also contain four shared arrays and two final signed members.
Only a nonempty record with load mode false stores the final sixteen word and
sixteen wide patterns. Empty records suppress all shared and mode-tail arrays,
even when their stored width is large.

Strings are counted UTF-16LE code units. The reference accepts empty strings,
unterminated strings, unpaired surrogates and values beyond the first NUL.
Field snapshots retain every raw unit; the separate text descriptors retain
the exact first-NUL prefix as UTF-16 units. No lossy Unicode conversion or
normalization supplies expected results. The nine signed item members retain
their native offset order: 8, 24, 12, 16, 20, 28, 40, 36, 44.

The FL11 packet contains 356 complete cases. Generated cases cover both modes,
zero/nonzero counts and widths, present empty arrays, independent extra counts,
every UTF-16 unit pattern, and every single bit/complement and special raw
numeric pattern. Expected data includes complete scalar/vector fields, string
and item intervals, count offsets and text-prefix units.

The installed `1.fld` sample supplies one 11,474-byte payload that parses through
EOF with an explicit false-mode assumption. With true mode, the reference
returns an 11,282-byte prefix and a 192-byte opaque remainder; that result is
incomplete. The packet separately labels a true-mode payload derived by cutting
that prefix. It does not credit that derived payload as an original native
record. Whole-file, payload, prefix and remainder hashes are recorded in
`baseline-verification.json`. The caller's actual native mode, runtime
transformations, engineering meanings and application acceptance remain
unverified.

Python regenerates the packet in a temporary root, reconstructs all fields,
checks every proper compact-profile prefix and trailing byte, mutates all
reached counts and verifies strict version/mode/wrapper gates. Rust rebuilds
every payload from the frozen fields and checks complete intervals, counts,
UTF-16 prefixes, numeric bits and the distinction between absent and present
empty arrays.

`FlowCarrier::record_11_load_state` now returns a complete immutable
`FlowLoadState`. `FlowLoadStateOptions` requires an explicit `FlowLoadMode`
assumption and independent byte, item, text-unit, array-value, stored-value and
descriptor bounds. Empty records spend no array budget on an unused width.
Field IDs bind both the original occurrence/revision and the external mode.
Text views expose all raw UTF-16 units and the separate first-NUL prefix without
converting them to lossy Unicode strings. Typed `FlowLoadMemberOffset` values
preserve the unusual nine-word native-offset order.

`replace_field` edits existing nonstructural values on isolated copies, reparses
the complete payload and requires unchanged physical fields and scopes. It
recomputes text prefixes after unit edits. Version, count, width and string
length fields remain read-only. Completed edits retain both validated views and
complete carrier history. No-ops preserve the original wire/checkpoint, and
freshly opened raw reversals restore every original byte.
