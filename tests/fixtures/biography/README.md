# Biography vectors

The three native streams were extracted from the owned blank documents in the
parent directory. `empty.bin` and `mixed.bin` were assembled independently from
clause 11.9. The mixed vector includes extreme signed and unsigned numbers,
Latin-1 and UTF-16 CStrings, empty paths, and repeated-string chains across three
records. Its paths are invented.

`expected.tsv` records every stored field and its expanded semantic counterpart,
as decoded by the Python reference reader. Strings are UTF-8 hex to keep the rows
unambiguous. The generator is
`docs/features/biography-archives/generate_fixtures.py`; it requires the reference
`solidworks` directory on `PYTHONPATH`. Normal Rust tests embed these fixtures.
Changes require the reason and biography behavior version in the feature notes.
