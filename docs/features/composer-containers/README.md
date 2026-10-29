# Composer SMG containers

Feature #100 implements clause 14.6 / C-145 as the next project-carrier operation
under the full parity goal in #2. The implementation is pending. Member payload
schemas, geometry, scene graphs and native application acceptance are separate
requirements; completing this carrier does not complete Composer parity.

The container is the literal three-byte `SMG` prefix followed by ZIP32 records.
ZIP offsets are relative to byte three. The reader must validate a complete
physical partition, local/central agreement, filename encoding, extra-field
framing, raw-deflate termination, decoded lengths and CRC-32. Split archives,
encryption, ZIP64 and data descriptors remain explicit unsupported variants.

The owned fixtures use Python's independent `zipfile` writer with fixed
timestamps. They cover completed empty archives, stored and deflated members,
Unicode names, duplicate names, unknown extra fields, and raw member/archive
comments. A crafted comment contains a misleading end signature. The actual
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
retained baselines. Replacing fixture bytes requires `--write`. The next gate
is a complete Bessemer read, isolated member edit, reconstruction and reopen,
with unchanged compressed members and unrelated metadata preserved exactly.
Member identities must be bound to immutable revisions so duplicate names and
foreign handles cannot silently select another entry. Construction, malformed
bounds, resource limits, input isolation and concurrent reads need owned tests
before the public API is published.

The maintainer-side humanizer skill was unavailable. A maintainer still needs
to review this prose.
