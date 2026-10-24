# Version-history fixtures

`blank-part.bin`, `blank-assembly.bin`, and `blank-drawing.bin` are the unmodified
`_MO_VERSION_19000/History` streams extracted with `swcontainer.Document` from the
owned native documents in the parent fixture directory. Each is 101 bytes. The
part and assembly histories happen to be byte-identical; both are retained to tie
the tests to their separate document kinds.

| Native history | SHA-256 |
| --- | --- |
| blank-part.bin, blank-assembly.bin | 1213d34ec481b89d97f2065be60f5ce9b2d091b8db2fccef569e267950000959 |
| blank-drawing.bin | 82d5ac0c49022011734573e60c42a7ed9b395ba1f03ea2b50bfd5a632c4df671 |

`threshold-*.bin` files were independently assembled with Python `struct` using
the specification's layouts, then read to the final byte by `modelhistory.py`.
They use root schema 7, stamp fields `(2, 17, 29, 123)`, an empty narrow comment,
and the file version in their filename. The versions bracket the date-code and
tail gates: 0x175, 0x178, 0x319, 0x45C, and 0x46C. Modern date objects use class
schema 11 and code 0x01123456; legacy date words use 0xFEDCBA98. Present tail
groups use deliberately distinct signed and unsigned values.

`mixed.bin` combines the branches, Latin-1 and surrogate-pair UTF-16 comments,
nonminimal word/dword CString prefixes, class reuse, shared object references,
extended tags, and distinct date-code objects. Its SHA-256 is
`465f17dae4ec2a0b788f202d6015a514dc832cbd67ba6e7c2b58091fa0e8c368`.

`expected.tsv` records the reference reader's semantic projection: root schema,
ordered stamp/state values, UTF-8 comment bytes in hex, date objects with ordinals
assigned on first encounter, class schemas and raw words, and optional tail groups.
This retains sharing without treating wire indices as permanent document IDs.
Updates require review with the reason and history behavior version. These are
the initial behavior-version-1 baselines, not Bessemer-generated expectations.
