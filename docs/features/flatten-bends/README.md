# Flatten and process bends

Bessemer reads and edits `moFlattenBends_c` and `moProcessBends_c` as documented
in section 11.245, C-287. The behavior version is `C287-flatten-bends-v1`.

Both features retain their complete body-feature and bend-group prefix. The
flatten-bend suffix starts with one raw i32. Only containing-node bit `0x800`
introduces the body-presence word; unrelated nonzero flags do not add a zero
placeholder. Any nonzero signed presence selects the existing strict raw kernel
reader, without a suppression byte and with an explicit schema and limits.

The containing `ModelNodePrefix` returns through the established readers, so
nested objects cannot replace the node flags used by this suffix. That refactor
was committed separately after all 956 existing library tests passed with their
owned fixtures unchanged. See [prefix reuse](prefix-reuse.md).

Process bends retain one tagged fixed-face reference and one raw i32. The
ordinary context keeps archive helper `+0x834` and partition-transfer conditions
clear. The saved sheet-metal helper `+0x850` remains independent and still
selects its inherited bend/fixed-face layout. Special archive modes are not
inferred from node flags or payload bytes.

Independent fixtures contain 84 graphs across seven versions and six styles,
with 21,188 primitive observations, fourteen ordinary document streams and four
alternate-state document streams. Tests cover omitted versus stored-zero
presence, negative nonzero presence, raw frames, fixed-face roles, independent
edit goldens, input isolation and flag edits that require a missing field.

The combined flatten/process and feature-state change completed two additional
native configurations, increasing the count from 826 to 828 while preserving
every earlier success. Two fresh processes matched all 906 statuses and failure
causes, leaving 78 explicit failures. Independent reference decoding confirmed
exact EOF and matching four-stream hashes for all 828 complete configurations,
totaling 57,652,797 resolved bytes. The native regression tests also verified all
four containing flatten/process roots and both raw frames against their hashes
and field values.

Run `cargo test --lib flatten_bend_tests` and
`cargo test --test resolved_flatten_bends` for typed reconstruction and edits.
Run `python -B docs/features/flatten-bends/run_pinned_tests.py` to regenerate all
fixtures using frozen copies of the actual reference readers.

The humanizer skill is unavailable; prose review remains with the maintainer.
