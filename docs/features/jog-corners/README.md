# Jogs and break corners

Bessemer reads and edits `moJog_c` and `moBreakCornerFeat_c` as documented in
section 11.248, C-290. The behavior version is `C290-jog-corners-v1`.

Both features retain their complete body-feature and bend-group prefix. A jog
keeps the stored reference-presence byte: any nonzero byte introduces both
optional reference tags, either of which can be null. Its final boundary pointer
appears from archive version `0x3AFE`. Stored options and coordinate bits are
preserved before native fresh-object normalization and profile transformations.

A break corner serializes its owned fillet surface-ID keeper directly, without
a keeper-object tag or map slot. Its final array remains a tagged pointer. The
last raw word appears from version `0x46CB`. The existing keeper reader validates
nonnegative bounded counts and retains entity IDs and reference identities.
Special archive modes that omit the keeper are not inferred from bytes.

Independent fixtures cover 108 graphs across nine versions and six pointer
styles, with 58,278 primitive observations, fourteen ordinary document streams
and four alternate saved-state streams. Tests verify exact body widths, omitted
versus null references, both gates, stored-zero option retention, typed reference
roles, keeper bounds, independent edit goldens and input isolation. Engineering
option domains and geometric validity remain unresolved.

Two fresh native-corpus processes produced identical statuses and failure causes,
preserving every earlier success and completing one additional configuration.
There are now 829 complete configurations and 77 explicit failures in this
906-context corpus. Independent reference decoding confirmed exact EOF and
matching four-stream hashes for all 829, totaling 58,223,637 resolved bytes.
The independent native regression also verified both containing roots, totaling
39,302 bytes, against stored hashes, fields, keeper counts and list boundaries.

Run `cargo test --lib jog_corner_tests` and
`cargo test --test resolved_jog_corners` for typed reconstruction and edits.
Run `python -B docs/features/jog-corners/run_pinned_tests.py` to regenerate all
fixtures using frozen copies of the actual reference readers.

The humanizer skill is unavailable; prose review remains with the maintainer.
