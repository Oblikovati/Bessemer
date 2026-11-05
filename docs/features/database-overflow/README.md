# Supplied-page database overflow interoperability

Feature #229 tracks complete overflow key/value storage operations. The current
public `database_hash` codec still accepts inline cells only.

The independent fixture corpus follows the stored-page writes recovered from
installed `sldDBImplu.dll`, rather than the distinct native key-reader callback
requests tracked in #226. The retained assembly, binary hashes and calling
convention are recorded in
[the writer investigation](../database-hash/overflow-writer-storage-investigation.json).
No installed target or original producer was executed to construct these images.

The 117 owned images cover empty keys and values, exact first-page and continuation
boundaries, multiple page extents and noncontiguous unsigned64 page identifiers.
Each image includes a bucket cell, complete overflow pages and separately stored
expected key/value bytes. Closed-form key-end positions and a separate
metadata-directed value reader check the construction.

At a supplied extent of 65,536, a key ending at the page boundary produces a
16-bit stored cursor of zero. Six images retain this conditional arithmetic
case. They are rejection controls, not successful value-decoding examples. The
fixtures do not establish that this extent is reachable in the installed engine,
or that any native sample contains this state.

The first overflow page stores a BE64 next link at byte 0, the BE64 value-start
page at byte 8 and the BE16 value-start cursor at byte 16. Key bytes start at
18; later pages reserve only their eight-byte next link. Values resume at the
recorded cursor and use byte 8 on subsequent pages. Unused bytes remain opaque.

The `HOV1` packet starts with its four-byte magic and a little-endian u32 record
count. Each record has a little-endian u32 byte extent, followed by
`<IIQBI`: page extent, key length, value length, cursor-wrap flag and page count.
Then come the complete bucket page, each `<Q` page identifier and complete page,
and the expected key and value bytes. The packet framing is a test format.

```text
python -B -X utf8 -m unittest discover -s docs/features/database-overflow -p test_*.py -v
python -B -X utf8 docs/features/database-overflow/freeze_writer_pages.py
```

The native search extracted 2,486 database streams from all 842 distinct files
in the 906-case corpus without extraction failures. A structural candidate scan
found 5,140 overflow cells with complete key/value chains and matching value-start
checkpoints. Keys have lengths 3 or 36; none cross a page boundary. One 4,380-byte
value crosses two pages.

Three complete native database streams are retained with literal reviewed key
and value ranges and separate expected payload files. The two-page value uses
stream ranges `[16438,20480)` and `[20488,20826)`. The tests compare these direct
ranges with metadata-directed reconstruction. All eight fixture tests passed.
The compressed census and `native-census-verification.json` retain the wider
search evidence. Physical page identity times 4,096 matches these selected
streams; this does not establish a universal engine/provider mapping.

Production admission, isolated edit/reopen/reversal, malformed graph and resource
checks remain required. Native keys spanning pages still need independent
evidence. These fixtures do not validate
file-wide indexing, allocation, transactions, commits, geometry or application
opening. The humanizer skill is unavailable; maintainer prose review is pending.
