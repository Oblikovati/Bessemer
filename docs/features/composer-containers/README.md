# Composer SMG containers

Feature #100 implements clause 14.6 / C-145 as the next project-carrier operation
under the full parity goal in #2. `composer::ComposerArchive` reads, constructs
and replaces members in immutable complete carriers. Member payload
schemas, geometry, scene graphs and native application acceptance are separate
requirements; completing this carrier does not complete Composer parity.

The container is the literal three-byte `SMG` prefix followed by ZIP32 records.
ZIP offsets are relative to byte three. The reader validates a complete
physical partition, local/central agreement, filename encoding, extra-field
framing, raw-deflate termination, decoded lengths and CRC-32. Split archives,
encryption, ZIP64 and data descriptors remain explicit unsupported variants.
Publication also requires forced metadata/payload reconstruction to match the
entire input exactly. Every member is decoded and CRC-checked before the
archive becomes available; reads take no lock or deferred validation step.

`ComposerLimits` bounds input bytes, member count, decoded member bytes and the
aggregate decoded size. New-member construction uses explicit stored or
raw-deflate compression and UTF-8 names. Member identities retain directory
order and belong to one immutable revision. Lookup by an ambiguous duplicate
name fails; identity lookup distinguishes those members and rejects foreign
revisions. Clones and completed no-op edits retain the original identities.

`edit_member` replaces a payload on a copy, preserves unrelated encoded members
and header metadata, updates the affected CRC/sizes and relocated offsets, then
requires another complete read. The result returns typed completion, exact
container coverage, achieved member-content fidelity, diagnostics and the
derived member's parent/result identities. Header extras and comments retain
their original bytes. `to_bytes` on an unchanged archive reproduces the input.

The owned fixtures use Python's independent `zipfile` writer with fixed
timestamps. They cover completed empty archives, stored and deflated members,
Unicode names, duplicate names, unknown extra fields, and raw member/archive
comments. Additional vectors cover every high CP437 character and central
directory order that differs from physical local-record order. A stored-member
replacement matches a complete independent ZIP-writer golden byte for byte.
A crafted comment contains a misleading end signature. The actual
reference reader finds the earlier consistent directory. Python `zipfile`
instead selects the false end record, so that case compares decoded members
against the producer's base archive, whose member bytes were unchanged by the
comment mutation. The report records both archive hashes.

`freeze_composer.py` pins the complete reference source tree before importing
`composer.py`. It checks both readers' member payloads and the reference byte
partitions. The installed audit requires all 38 archives and 550 members,
44,056,222 stored file bytes and 106,834,828 decoded member bytes. Input and
decoded-member hashes bind the audit to the reviewed source corpus.

Run `python docs/features/composer-containers/freeze_composer.py` to check the
retained baselines. Replacing fixture bytes requires `--write`. Adding
`--export` writes the local native probe packet, including Python `zipfile`'s
independent decoded payloads. The ignored `composer_native` test compares all
38 Bessemer reconstructions and 550 payloads exactly with that packet. It also
writes an edited archive for every input. `verify_native_edits.py` reopens those
38 outputs through actual `composer.py` and independent `zipfile`, checks all
550 payloads, and proves all 512 untouched encoded members stayed identical.
It checks names, raw extras, comments and unrelated fixed metadata as well as
the permitted changed sizes, CRCs and offsets. Original hashes bind every pair
to the reviewed native corpus.

Owned tests cover member edits, reopen, stored-member revert, input-buffer
mutation, concurrent replacements, foreign identities and all truncated prefixes
of the populated deflated fixture. Rejection tests exercise all four resource
budgets, invalid UTF-8, extra-field overruns, local/central disagreement, CRC
corruption, unterminated deflate, trailing compressed data and unsupported ZIP
variants. All public result types have compile-time `Send`/`Sync` assertions.
The full local PR gate passed 2,816 tests, including 1,249 library tests and 277
doctests. Final verification records source hashes, independent corpus/edit
results and the explicit member-schema and application exclusions.

The maintainer-side humanizer skill was unavailable. A maintainer still needs
to review this prose.
