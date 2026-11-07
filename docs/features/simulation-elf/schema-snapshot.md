### 14.107 Simulation ELF native initial image

*Status: Native initial-image constructor and exact installed reconstruction verified; populated ELF records and remaining parameter meanings unresolved · Evidence: C-483*

The benchmark CWR's **ELF stream is 1,224 bytes**: a four-byte transfer prefix (14.101) and a **1,220-byte copied body**. This body is reproduced exactly by the initial-image constructor in installed **`star.exe` at 0x140E44510**. Its `.ELF` filename literal is at RVA **0xF08A68**. Filename references in the front end and preprocessor led to availability/cleanup operations; the schema below instead follows the solver's actual buffer construction and file write.

The constructor executes when cached record-word global **`14105E290` is zero**. Let `P` be its input word parameter from `14105E28C`. It computes:

- `W = P + 1`, stored back to `14105E290`;
- `record_bytes = 4 × W`;
- `initial_marker = trunc(199 / W) + 3`;
- `written_bytes = (initial_marker - 1) × record_bytes`.

For the installed image, `record_bytes = 244`, hence **W = 61**, **P = 60**, **initial_marker = 6**, and **written_bytes = 5 × 244 = 1,220**. The five record-sized regions are not described as five populated data records. The marker's later allocation/index semantics are not assigned solely from this initialization equation. The reproduced profile uses this positive 61-word case; division errors, negative/overflowing inputs and other record sizes remain outside it.

The body starts with seven consecutive four-byte words:

| Stream offset | Body offset | Stored value | Native source / meaning established here |
|---:|---:|---:|---|
| 4 | 0 | 6 | Computed initial marker `trunc(199 / W) + 3` |
| 8 | 4 | 244 | Computed record size `4 × W` |
| 12 | 8 | 100 | Fixed constructor word; broader role unresolved |
| 16 | 12 | 14,642 | Input pattern from native global `14105E284` |
| 20 | 16 | 0 | Fixed initial zero word |
| 24 | 20 | 14,642 | Independent input pattern from native global `14105E288` |
| 28 | 24 | 1 | Fixed initial one word |
| 32 | 28 | 1,192 zero bytes | Zero-initialized continuation of the constructor buffer |

Native **0x140E44598** zeroes successive four-byte words after the seven-word header, and **0x140E4459D** bounds that initialization loop at **440 words**. The initialized buffer therefore covers the 1,220-byte write completely. Native **0x140E44604** passes the start of the header buffer, **0x140E44610** calculates the byte extent by multiplication, and **0x140E44616** calls its file-write wrapper. The open call receives mode **0x8102**, share mode **0x40** and permission word **0x180**. The retained CRT wrapper/core paths reach `CreateFileW` and the direct binary `WriteFile` branch. The constructor does not validate every open/write result, so truncated or failed writes are not equivalent to a valid initial image.

The separate native consumer at **0x140E49DA0** allocates **244 bytes per selected entry**, reads a header buffer, uses the stored record-size word for subsequent seeks/reads, and processes dynamic records. That is further evidence that the 244 word is a record byte extent. The populated record/index grammar, its identifiers and payload types are not implemented here; neither the value 60 nor the `.ELF` suffix is used to invent a force-vector layout. Current code reconstructing this historical fixture does not identify the exact historical writer build.

The two independent constructor-parameter words both equal **14,642** in the installed image, matching its paired GEN element count. That consistency is retained, but the parser does not require the parameters to equal each other or a mesh count: their universal meanings have not been established. Their full 32-bit source patterns remain available, including signedness ambiguity. The fixed words 100, 0 and 1 likewise do not establish complete flag, allocation or index semantics.

`solidworks/simulationelf.py::read_initial` validates the existing ELF transfer framing and this exact native initial-image profile. It checks the derived marker/extent and the all-zero continuation, retains the source and independent parameter words, and reports initial-image completion separately from field-semantic completion. It explicitly rejects populated/different images, including a nonzero byte after the header; it does not silently present unknown data as padding. Five focused tests cover the initializer equations, independent parameter preservation, header/extent disagreement, nonzero continuation and unchanged source bytes.

`simulation_elf_evidence.py` pins **nine native constructor/consumer/CRT ranges**, module/source hashes, the filename literal and preceding transfer evidence, and exact source reconstruction in `simulation-elf-native-evidence.json`. The installed body SHA-256 is `1085a7d2b93dbaa4416f9ee54d39a2b1b614fc41d8d64cdfe17c6a524b72cce5`. All **1,220 body bytes** are assigned to the native initial buffer image, while the open parameter meanings and populated format remain partial. No installed native target was executed and no global/all-project coverage percentage is regraded.

All **1,567 regression tests pass** (177.445 seconds). The native/corpus verifier passes; specification lint reports zero findings (669 headings, 96 anchors).

Evidence: C-483.

### 14.108 Simulation ELF directory updates and selected-record traversal

*Status: Native directory/header update and identifier-copy rules verified; installed fixture has empty slots, populated cases are synthetic · Evidence: C-484*

The ELF initial image in 14.107 also supports an indexed record area. Two independent native paths establish the new fields: the selected ELF branch inside **`star.exe` 0x1400DC8D0** writes directory entries and advances the record marker, while **0x140E49DA0** reads the first entry and visits its records. The latter uses 244-byte records, a signed identifier in the first four bytes, and an externally supplied ID map. The remaining **240 payload bytes** are still opaque; this clause does not invent force-component types or a complete populated ELF schema.

Within the supported 244-byte/100-slot profile, the header has these additional operational roles:

| Body offset | Header word | Role in the selected updater |
|---:|---:|---|
| 0 | 0 | One-based marker for the next block's first record; advanced by word 5 |
| 4 | 1 | Record size **B = 244** bytes; **W = B / 4 = 61** words |
| 8 | 2 | Nominal directory capacity **C = 100**, used to calculate the header/index write extent |
| 12 | 3 | Independent constructor parameter; untouched by this update |
| 16 | 4 | Selector watermark; replaced by the greater of its signed value and the resolved selector |
| 20 | 5 | Number of records assigned to the newly indexed block |
| 24 | 6 | Retained header word; initially 1, broader meaning unresolved |

**Raw instruction offsets establish the word mapping.** The header buffer begins at native stack offset `0x30`. Native **0x1400DCB40** updates stack offset `0x40`, hence header word **4**; **0x1400DCB66** loads the block count at header offset `0x14`, hence word **5**. Decompiler local names skip offsets and are not treated as a contiguous field declaration. This refines the source-based parameter descriptions in 14.107 without assigning a meaning to word 3 solely because its installed value equals the element count.

The nominal directory contains **C pairs of signed 32-bit values**, starting at body offset **B**. For a resolved one-based selector **S**, the pair starts at:

`pair_offset = B + 8 × (S - 1)`

The updater stores `(old_marker, header_word_5)` there, then replaces header word 0 with `old_marker + header_word_5`. Its write extent is:

`index_area_bytes = (trunc((2 × C - 1) / W) + 2) × B`

For C = 100 and W = 61, this is **1,220 bytes**: a 244-byte header record followed by enough record-sized space for 800 directory bytes. The directory itself occupies body offsets **244..1,044**, and the remaining **176 bytes** through offset 1,220 are cleared by this writer. The first header record's bytes **28..244** are cleared too. These spans are not populated result records.

**This update clears other directory entries.** Native **0x140E66410** zeroes **447 four-byte words** in its temporary buffer. The caller then reads only **28 bytes** of the existing file header at body offset zero, inserts its selected pair and writes the calculated header/index extent. Other directory entries and padding do not survive that operation. The zero count (447), read offset (0) and read length (28) are verified at RVAs **0xF11FA8**, **0xF11FB0** and **0xF11FB4**. Direct I/O helpers **0x140E62E30** and **0x140E63090** take offsets/counts by reference and dispatch seek plus read/write. Filename/open helper **0x140E637C0** receives `.ELF` and mode `B`, which selects open flags `0x8002`.

The selector is resolved from caller state outside this projection, and the enclosing ELF branch has its own runtime gate. The portable model requires an explicitly supplied selector in the nominal range **1..100**, positive nonoverflowing marker/count arithmetic and the supported record size. These are model/profile guards, not evidence that the native routine performs equivalent bounds checks. No live file or application state is changed.

The native getter reads the first directory pair at body offset **244**. A pair `(R, K)` selects records beginning at body offset **`(R - 1) × B`**, visited sequentially up to K times:

- A negative signed identifier stops the loop after that record has been read.
- Identifier zero is skipped.
- A positive identifier indexes the caller's source-to-destination map at **identifier minus one**. Map value zero skips the record. A nonzero map value selects output slot **map value minus one**, and the entire **244-byte source record** is copied there, preserving its original identifier and raw payload.

Repeated copies to one output slot overwrite it in visit order. The model leaves untouched destinations explicitly unassigned instead of inventing their prior memory contents. Missing maps and unsafe target indices stop the projection rather than reproducing out-of-bounds native memory access. Slot 1 is bound directly to the inspected native getter; positions of higher slots follow the native updater's selector formula and are exercised synthetically. No claim is made that this getter itself selects every slot.

`solidworks/simulationelf.py` now exposes `read_directory`, `project_index_update` and `project_record_mapping` alongside `read_initial`. The first preserves visited records and their stop/skip/map actions. The second returns only the proposed **header-body bytes**, starting at flat-file body offset zero (wrapped stream offset 4), and reports that no data records were written. The third requires an explicit external map and destination count. These projections do not open files, execute runtime gates, derive selector state, simulate native error handling or complete the 240-byte record payload grammar.

All **100 installed directory pairs are `(0,0)`**, so the stored fixture visits no records. Under explicitly supplied selector **1**, projecting its header update produces pair **`(6,14642)`**, advances the marker to **14648** and updates the watermark to **1**. That proposed header alone is not a populated file: the projection does not append the 14,642 advertised records. A separate explicitly synthetic example advertises four records with IDs **7, 0, -1, 9**. The reader visits **7, 0, -1**, stops before 9, and a supplied map can copy ID 7 into output slot 2 without rewriting its stored identifier. No installed populated ELF case is claimed.

`simulation_elf_index_evidence.py` retains **seven native ranges/leaves**, constants, raw header-offset instructions, source/module hashes, the installed empty directory, the supplied-context header output and the synthetic traversal/mapping example in `simulation-elf-index-native-evidence.json`. Six focused tests cover indexed offsets, one-based record addressing, negative termination, zero-count directories, clearing other slots, signed watermark comparison, marker advance, overwrite/skip mapping behavior, context guards and source preservation. The broader populated index lifecycle, remaining header semantics, payload fields and other versions remain open. No installed native target was executed and global/all-project coverage percentages remain unchanged.

All **1,573 regression tests pass** (197.969 seconds). The native/corpus verifier passes; specification lint reports zero findings (670 headings, 96 anchors).

Evidence: C-484.

### 14.109 Simulation ELF record scalar slots

*Status: Native scalar storage verified for the 244-byte record context; physical component meanings remain partial · Evidence: C-485*

The ELF emitters rooted at **star.exe!0x1400FA920** and **0x1400FAEF0** establish the following record layout. Offsets are relative to a selected data record, after the header/index area in 14.108.

| Offset | Size | Stored field |
|---|---|---|
| 0 | 4 | Signed 32-bit identifier; stop/skip/external-map rules from 14.108 apply |
| 4 | 240 | Sixty little-endian IEEE 754 binary32 slots |

The payload occupies ten consecutive groups of six slots. Group **g**, slot **j**, with zero-based indices, begins at **`4 + 24*g + 4*j`**. The last slot begins at offset **240**, and the record ends at **244**. Native `VMOVSS` stores at six consecutive four-byte offsets, together with scalar float/double conversions and a **24-byte** group advance, establish the scalar representation and grouping. Nonzero caller entries advance the packed group position; zero caller entries are skipped without advancing it. These groups cannot yet be associated automatically with GEN nodes or other entities.

The serialization tail copies the caller identifier into the first four bytes and passes the record buffer to positioned-write helper **0x140E63100**. The write position is **`(record marker - 1) * record size`**, using the runtime record extent. This clause supports only the independently established **244-byte** context; it does not generalize arbitrary configured extents. Component names, force/moment roles, units, coordinate frame and caller association tables remain unresolved. Optional runtime transformations and corrections precede the final stores, so the stored values must not be treated as unmodified caller inputs.

The enabled clear path calls **0x140E66410** with **74 four-byte words**, verified at RVA **0xF12EF4**, starting at payload offset 4 in the temporary workspace. That clears **296 workspace bytes**. Only the first **60 words / 240 bytes** lie inside the serialized record; the remaining workspace bytes do not establish additional serialized components. The native packet retains the complete split writer envelopes **0x1400FA920..0x1400FAEF0** and **0x1400FAEF0..0x1400FB540**, including out-of-line clear blocks and shared serialization tails. A short unwind entry alone does not contain all these operations.

`solidworks/simulationelfrecord.py` exposes `read`, `project_store` and `project_directory`. `read` requires exactly 244 bytes and returns all sixty raw bit patterns alongside convenience floating-point values, grouped six at a time. It preserves the entire source record and leaves component roles, units, frame and row associations unassigned. Raw patterns are authoritative for signed zero, subnormals and NaN payloads. `project_directory` applies this view to records already visited by 14.108, without deriving an external entity map.

`project_store` accepts a signed identifier and up to ten groups of up to six **already resolved, post-computation binary32 bit patterns**. Omitted cells become positive zero, matching the inspected enabled clear path. This is a store-stage projection: it does not perform caller filtering, runtime arithmetic, transformations, unit conversion, native floating-point control or file I/O. Its input limits are portable profile guards, not claims about native bounds checks.

The installed ELF directory still visits **zero populated records**, hence this increment adds no observed payload components to corpus coverage. A separately labeled synthetic record exercises all sixty slots, including negative zero, a subnormal, infinity and a NaN payload, and reconstructs exactly from independent packed bytes. Five focused tests cover slot offsets, source preservation, zero filling, special bit patterns, identifier control actions, directory integration and profile guards. `simulation_elf_record_evidence.py` writes four native envelopes/helper ranges, module/source hashes, the installed empty case and the synthetic replay to `simulation-elf-record-native-evidence.json`. No installed native target was executed. Full field semantics, populated lifecycle evidence and all-installed-project coverage remain unfinished.

All **1,578 regression tests pass** (225.760 seconds). The native/corpus verifier passes; specification lint reports zero findings (671 headings, 96 anchors).

Evidence: C-485.
