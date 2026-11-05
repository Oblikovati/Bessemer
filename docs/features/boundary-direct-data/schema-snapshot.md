### 11.393 Direct boundary temporary chooser time-pair reader

*Status: supported modern direct entry framing; positive entries have native and synthetic evidence, while timestamp roles remain unresolved.*

In the ordinary modern boundary body, read `temporary_count` as signed i32 after `unknown_48`. Each positive-count entry contains two direct su_CTime archive words: `time_0` u32 followed by `time_8` u32, eight stored bytes total. There is no per-entry MFC object/class tag. The suffix remains the version-gated +0xEC word and direct-model-data presence word for versions > 0x332C. At the C-1842 checkpoint a present direct body remained unsupported; 11.394 subsequently adds its framing.

The native pair-reader binding is established in 11.392. The four-byte su_CTime extraction is independently retained in `configuration-table-native-evidence.json` and 11.134: the imported swccu operator at 0x31D4BC60 forwards to the four-byte archive extractor, despite eight-byte runtime time objects. Raw 0xFFFFFFFF and high-bit words remain u32 wire values; no runtime sentinel widening or calendar conversion is applied.

`modelboundary.boundary` exposes `temporary_records`, including an empty list for zero or negative counts. Negative counts preserve the stored count and skip entries, matching the native loop. Positive counts must fit the remaining minimum eight bytes per entry with the known following suffix reserved. Every read remains bounds-checked. Each complete time word stays charged as unknown because neither timestamp's role is established.

Seven boundary tests pass, including two populated direct entries with distinct raw/sentinel values, exact following-byte alignment, truncation at every byte of the populated suffix, impossible positive counts, both negative-count cases and the +0x332C suffix transition. All 906 retained modern part resolved streams retain source hashes, exact EOF and unchanged unknown totals [C-1842]. Their 58,701 captured boundary bodies all have temporary_count zero. Thus populated entry framing has native/synthetic evidence, not installed positive-count sample evidence; no new semantic byte coverage is claimed.

Evidence: **[C-1842]** `part-boundary-temporary-times-replay-evidence.json` / `part_boundary_temporary_times_replay_audit.py` retain all 906 stream comparisons, target-stream-only count frequencies, seven normal test results, primitive/native evidence hashes and current parser/test sources. Target CAD code was not executed. Global coverage remains unchanged.

### 11.394 Boundary-owned direct downstream identifier data

*Status: supported ordinary modern framing from native and synthetic evidence; value semantics and populated installed samples remain incomplete.*

For version > 0x332C, a nonzero boundary `has_direct_model_data` word selects direct moEntRefDownstreamIdData_c. Constructor `0x4B559570` installs primary vtable `0x4CE180D8`, whose MSVC RTTI identifies that class and whose +0x28 slot selects serializer `0x4B559920` [C-1843]. The body is direct: it does not allocate an MFC class/object slot or start with an object tag.

| Order | Decoder field | Wire form | Presence |
|---|---|---|---|
| 1 | pair_count | i32 | always |
| 2 | pairs | max(pair_count, 0) entries of i32 key, i32 value | always |
| 3 | value_count | i32 | always |
| 4 | values | max(value_count, 0) i32 values | always |
| 5 | unknown_38 | i32 | always |
| 6 | values_40 | two i32 values | version >= 0x3359 |

The native serializer writes a tree-map count and its ordered key/value pairs, then an integer vector count and entries, then member +0x38. Both load and store paths gate the last two values at version 0x3359. Native loading replaces an existing duplicate key's value and drops zero vector entries. The file decoder preserves duplicate pairs, original order, zero values and all signed bits instead of applying these runtime changes. Negative counts retain their raw values and skip the corresponding loops, matching native signed comparisons. Positive counts must fit the remaining minimum bytes for entries and subsequent fields.

The map/list and scalar meanings remain unresolved. `modelboundary.downstream_id_data` therefore charges every key, value, list element and trailing scalar as unknown; collection counts provide framing only. The owner boundary retains its raw presence word, including nonzero values other than one. The direct body's native 0x70 allocation size is not its wire size.

Ten boundary tests pass, including populated pairs with duplicate negative keys, preserved zero vector entries, raw scalar values, exact trailing boundary, unchanged object map, truncation at every byte of the populated body, impossible counts, negative-count loop skips and the 0x3358/0x3359 tail transition. All 906 retained modern part resolved streams preserve source hashes, exact EOF and unknown totals [C-1844]. Of 58,701 captured boundary bodies, 58,622 store zero presence and 79 predate the presence word. None supplies a populated installed direct body; positive framing has native/synthetic evidence only.

Evidence: **[C-1843]** `part-boundary-downstream-data-native-evidence.json` / `part_boundary_downstream_data_native_audit.py` retain complete constructor/serializer bytes and disassembly, RTTI/vtable bytes, eight exact signed-extraction imports, version gates and duplicate/zero-handling assertions. **[C-1844]** `part-boundary-downstream-data-replay-evidence.json` / `part_boundary_downstream_data_replay_audit.py` retain all 906 stream comparisons, target-stream presence frequencies, ten test results and parser/test source hashes. Target CAD code was not executed; semantic byte credit and global coverage remain unchanged.
