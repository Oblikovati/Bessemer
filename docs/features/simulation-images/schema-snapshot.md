### 14.100 Simulation logical filenames and CWR root-stream lookup

*Status: Native lookup and installed compound-stream binding verified; stream payload schemas remain partial · Evidence: C-476*

`cwcommfuncs.dll!FindStreamFileFromCWStorageFile` at **0x180005320** interprets its Unicode logical filename as a compound-storage lookup. It finds the **last dot in the entire input**, takes the substring after that dot as the stream name, and concatenates the substring before that dot with literal **`.CWR`** as the storage filename. Helper **0x180004A10** calls the imported CString concatenation operation with the two strings and their lengths. The `.CWR` literal is retained at RVA **0x12828**. Thus `C:\study.GEN` selects root stream `GEN` in `C:\study.CWR`; `study.MAS` selects root stream `MAS` in `study.CWR`.

This is a lexical split. The routine does not trim or uppercase the suffix, strip directories before finding the last dot, or substitute a format-specific stream table. For example, the input `a.b\file` projects to storage `a.CWR` and stream `b\file`; this does not imply that COM accepts that stream name. Case-insensitive COM name matching, filesystem resolution and wildcard effects of the existence probe are outside the lexical model. Names without a dot and names containing embedded NUL are outside the reproduced profile.

The native procedure performs these operations:

| Step | Native operation | Observed condition/result |
|---|---|---|
| 1 | `FindFirstFileW(storage_name, ...)`, then `FindClose` | Invalid handle returns false |
| 2 | `StgOpenStorageEx` | Mode `0x10`, storage format `0`, attributes `0`; failed HRESULT or null storage triggers message resource `0x24F`, then returns false |
| 3 | `IStorage::OpenStream` at vtable offset `0x20` | Unchanged suffix, reserved pointer null, mode `0x10`, reserved word zero |
| 4 | Test stream-open HRESULT | **Exactly zero** sets the return flag to one and releases the stream; other values leave zero |
| 5 | Release storage and return flag | No payload reads occur in this predicate |

The decompiler declares the export `void` and loses its return flag. Native instructions are authoritative: **0x180005455** sets `EDI=1`, **0x1800054AA** returns `EAX=EDI`, and **0x1800054F7** sets the failure result to zero. The separate `FindStreamFile` at **0x1800052A0** only checks an ordinary filename with `FindFirstFileW`; it does not establish a compound-stream payload layout.

`solidworks/simulationstorage.py` provides the pure `locator(logical_name)` projection for NUL-free inputs containing a dot. It preserves case, whitespace and Unicode text, reports the two open modes and storage format, and explicitly marks payload decoding false. It performs no filesystem or COM calls. Five focused tests cover GEN/MAS lookup, case/whitespace, lexical directory-dot and empty-suffix cases, Unicode, and profile guards.

The read-only corpus check covers all **three installed CWR fixtures**, all MS-CFB major version **4** with **4,096-byte sectors**. The two composite fixtures each contain `GEN` (1,282,856 bytes), `MAS` (28,420 bytes) and `TimeStamp` (4 bytes). The benchmark fixture contains `GEN` (3,306,932), `MAS` (28,420), `ELF` (1,224), `OUT` (4,970), `RunState` (4) and `TimeStamp` (4). All **12 root streams**, totaling **5,964,114 logical bytes**, bind through constructed logical filenames to their observed storage and exact stream names. Constructed lookup inputs do not recover historical filenames supplied by the application. No semantic interpretation is assigned to these payloads by this check.

**Legacy GEN remains separate.** `cosworks.dll` routine **0x180EA3FA0** opens a `.GEN` filename through MFC `CFile` and reads a legacy flat-file layout. Its fixed blocks, version branches and mesh-container reads are not assigned to the CWR `GEN` stream: a native conversion or reader path establishing their relationship has not yet been recovered. Matching extensions and initial word patterns are insufficient evidence. GEN and MAS payloads remain opaque in this clause, and global byte-coverage percentages are unchanged.

`simulation_storage_evidence.py` retains module, native-byte, decompilation and sample/stream hashes in `simulation-storage-native-evidence.json`, including three complete native ranges. The installed `cwcommfuncs.dll` SHA-256 is `888490cb99a3978a63eb32a0095ccfad6fec95dcd8c377661c84e7f56b5fa2a9`. No installed native target was executed.

All **1,532 regression tests pass** (247.947 seconds). The native/corpus verifier passes; specification lint reports zero findings (662 headings, 96 anchors).

Evidence: C-476.

### 14.101 Simulation CWR copied-file transfer prefix

*Status: Native successful-copy framing verified for GEN/ELF/OUT; copied bodies remain opaque · Evidence: C-477*

Installed **`cwanalysisproc.exe` at 0x140002070** transfers a nonempty ordinary file into a CWR root stream. It opens the input file, obtains its unsigned 64-bit length `L`, allocates a **0xA00000-byte (10 MiB)** buffer, and uses the filename suffix after its last dot as the stream name. The solver's caller **0x140006240** supplies `.GEN`, `.ELF` and `.OUT` among its result-file suffixes, retained at RVAs **0x13208**, **0x13268** and **0x13218**. This establishes a copied-file envelope for those streams independently of their inner file grammars.

For a completed successful transfer with a representable positive signed count, the stream consists of:

| Stream offset | Size | Field | Meaning |
|---:|---:|---|---|
| 0 | 4 | `transfer_count` | Little-endian signed 32-bit value `floor((L - 1) / 0xA00000) + 1` |
| 4 | `L` | `copied_file` | Input file bytes, concatenated in read order without intervening chunk headers |

The count describes transfer-buffer iterations; it is **not a file-format version or mesh-record count**. Native **0x1400020ED** sets the buffer capacity to `0xA00000`; **0x140002167** writes the four-byte count through `IStream::Write`; **0x140002196** reads up to the remaining length or buffer capacity and **0x1400021B3** writes the returned positive byte count. There is no per-chunk length word, compression operation or alignment insertion in this path. A body of exactly 10 MiB has count 1; 10 MiB plus one byte has count 2.

The ordinal-only MFC imports are resolved through their actual provider export entries and code, rather than assigned names from an unavailable named export table. Solver thunks **0x14000D3EB**, **0x14000D3F7** and **0x14000D3FD** import `mfc140u.dll` ordinals **11644**, **5676** and **12033**, respectively. Their provider bodies at **0x1802306E0**, **0x180230C90** and **0x180230910** call `CreateFileW`, `GetFileSize` and `ReadFile`. Native call arguments establish the read request size and subsequent write length, including arguments omitted from decompiler output.

The native function ignores stream-write HRESULTs, can produce truncated output following read/write failures, and does not constitute an integrity validator. An empty source does not enter stream creation in this transfer routine. Its loop/result flag and cleanup behavior are not a guarantee that all requested bytes reached storage. Consequently the reproduced profile accepts only exact, nonempty successful-copy images whose body length agrees with the positive count. It does not claim to reproduce error-path outputs or signed-count overflow behavior.

`solidworks/simulationtransfer.py` implements this profile for the three explicitly bound names `GEN`, `ELF` and `OUT`. It returns the raw copied body, offset 4 and transfer count, with body decoding explicitly false. It rejects inconsistent/empty images and unbound names, including `MAS`, `RunState` and `TimeStamp`. The latter two streams have separate direct scalar writes in the solver; their four-byte size alone is not evidence for this envelope. The observed MAS leading word remains outside this profile until its producer or consumer is bound.

The three installed CWR files contain **five bound streams**: three GEN streams, one ELF stream and one OUT stream. Each has count **1**, matching its body length. This identifies **20 prefix bytes** and preserves **5,878,818 copied body bytes** without interpretation. No installed bound stream exceeds 10 MiB; synthetic tests exercise both sides of that boundary. The copied GEN image is now separated from its transfer prefix, but its revision and field layout have not been established as the legacy `cosworks.dll` CFile layout from 14.100. The corpus may contain historical producer variants; matching successful-copy framing does not identify a historical writer build.

`simulation_transfer_evidence.py` retains two complete solver ranges, three complete MFC provider ranges, thunk bytes and provider ordinals, caller literals, source/model hashes and all five stream/body hashes in `simulation-transfer-native-evidence.json`. Five focused tests cover source-byte preservation, the 10 MiB boundary, invalid counts, short/empty data and stream binding. The broader installed-module import scan and geometry-postprocessor reader traces are retained separately; the recovered postprocessor readers select STE/STP/STN and other result streams and are not promoted to GEN/MAS payload evidence. No installed native target was executed, and no all-project byte-coverage percentage is regraded.

All **1,537 regression tests pass** (184.709 seconds). The native/corpus verifier passes; specification lint reports zero findings (663 headings, 96 anchors).

Evidence: C-477.

### 14.102 Simulation MAS table image, version context and GEN count prefix

*Status: Installed MAS record framing and paired version-8 GEN prefix verified; field meanings, padding and later GEN arrays remain partial · Evidence: C-478*

The legacy GEN reader's version context is not the leading CWR transfer word. **`cosworks.dll` at 0x181040000** obtains it through **0x180EA5B80**, with arguments `(study, 1, 0x15, 2)`, before calling GEN reader **0x180EA3FA0**. File selector 1 loads the master table through **0x180EA5D90**, **0x180EA32E0**, **0x180EA7000** and **0x180EA8870**. The filename-suffix initializer at **0x180324590** binds its first suffix global to Unicode **`.MAS`**, at RVA **0x1F6ED90**. The accessor increments the last argument for selector 1, selecting **logical row 21, field 3**, as a signed 32-bit version value.

All three installed CWR `MAS` streams have a leading word 1 and then a **28,416-byte master-table record image**, starting at stream offset 4. The leading word agrees with the generic transfer-count equation in 14.101; the historical MAS producer call path has not been recovered. The inner record image is independently bound to the native `.MAS` loader by its header, field directory, row directory, record-position equations and version access. The reproduced profile is limited to this observed 81-field/70-row shape. It does not assign this layout to every stream named MAS.

Native **0x180EA32E0** supplies 81 fields and row capacity 70, with record size rounded up from `81 × 4` to a multiple of 128, giving **384 bytes**. Native **0x180EA9270** reads one-based record `R` at body offset `(R - 1) × 384`; **0x180EA8870** computes the intervening field-table and row-directory record counts. The installed images have this exact layout:

| Copied-body offset | Bytes | Content |
|---:|---:|---|
| 0 | 36 | Nine header words: `384, 70, 81, 0, 70, 0, 0, 0, 70` |
| 36 | 348 | Unassigned header-record bytes, retained verbatim |
| 384 | 648 | 81 pairs of four-byte words; all installed pairs are `(1, 4)` |
| 1,032 | 120 | Unassigned field-table record padding |
| 1,152 | 280 | 70 signed row-directory entries; installed values are `1..70` |
| 1,432 | 104 | Unassigned directory-record padding |
| 1,536 | 26,880 | 70 physical row records of 384 bytes each |

Each installed row record contains **81 four-byte field words (324 bytes)** followed by **60 unassigned bytes**. The second word of each field-table pair determines the field's byte width; the native loader constructs cumulative field offsets from these widths. The first word of each pair is preserved without an invented type meaning. Most field words likewise remain unsigned raw bit patterns, not universally assigned signed-integer or floating-point types. The row-directory entry controls the physical row selected by the logical index; identity order is observed in the fixtures, not substituted for directory lookup in the model. Padding and unassigned header words are not semantic coverage credit.

The selected master version values are **830038** for both composite fixtures and **850023** for the benchmark. Their integer quotient by 100000 is **8**, selecting the native GEN family-8 branch. This resolves the context boundary left in 14.100–14.101 for these three paired samples. It does not establish every legacy branch or a universal product-year mapping for the version integer.

After the four-byte GEN transfer prefix, that branch has the following initial image. Stream offsets include the transfer prefix; copied-body offsets exclude it.

| Stream offset | Body offset | Bytes | Content |
|---:|---:|---:|---|
| 4 | 0 | 400 | Opaque legacy header bytes |
| 404 | 400 | 56 | Fourteen consecutive signed 32-bit count/scalar words |
| 460 | 456 | 140 | Opaque bytes read into the native temporary header buffer |
| 600 | 596 | 4 | Additional raw 32-bit word |
| 604 | 600 | remainder | Later context-dependent blocks and mesh arrays, unresolved here |

The fourteen words feed, in order, native locations `1827AA2A4`, `1827AA2A8`, `1827AA2B8`, the node-count local, the element-count local, `1827AA2B0`, `1827AA2B4`, `1827AA2BC`, `1827AA2C0`, `1827AA2AC`, `18254E458`, `18254E450`, `18254E454` and `18254E45C`. Node/element roles are established by the later `CNodeContainer::resize/read` and `CElemContainer::resize/read` calls. Other roles remain named by native destination rather than guessed engineering meaning. Native model-count mismatch checks can abort a load before this prefix completes; this standalone file projection does not reproduce a live model's validation state.

| Installed fixture | Version | Node count | Element count | Fourteen prefix words |
|---|---:|---:|---:|---|
| Composite, 16 plies | 830038 | 10,553 | 5,232 | `0,0,0,10553,5232,0,0,0,2,0,1,1,1,1` |
| Composite, 8 plies | 830038 | 10,553 | 5,232 | `0,0,0,10553,5232,0,0,0,2,0,1,1,1,1` |
| Benchmark plate | 850023 | 24,140 | 14,642 | `0,0,0,24140,14642,0,0,0,1,3,1,1,1,1` |

`solidworks/simulationmaster.py` implements the exact observed MAS profile, retaining all original bytes and exposing the directory-selected raw row words and signed version. `gen_prefix` requires that paired context, accepts only family 8, validates complete GEN transfer framing, and returns the prefix through stream offset **604**. It retains the first 400 and next 140 body bytes as opaque and leaves the later body untouched. The three prefixes expose **180 count/scalar bytes** in total; this is not whole-GEN or whole-MAS semantic completion. Later GEN branches depend on preexisting model/global state and remain separate work.

`simulation_master_evidence.py` retains **17 native ranges/leaves/initializer prefixes**, source and module hashes, all MAS/GEN hashes, version/row projections and count-prefix hashes in `simulation-master-native-evidence.json`. Five focused tests cover directory-controlled row selection, version extraction, preservation of padding/source bytes, unsupported layouts and GEN context guards. No installed native target was executed, and global/all-project coverage percentages remain unchanged.

All **1,542 regression tests pass** (197.648 seconds). The native/corpus verifier passes; specification lint reports zero findings (664 headings, 96 anchors).

Evidence: C-478.

### 14.103 Simulation GEN mesh arrays, auxiliary-stride fallback and exact installed EOF

*Status: Complete installed GEN image framing verified; raw record fields, opaque header areas and runtime semantics remain partial · Evidence: C-479*

The paired MAS versions from 14.102 select native container readers for versions **at least 800050**. `cwmeshcontainer.dll!CNodeContainer::read` at **0x180003BB0** reads **52 bytes per node**, and `CElemContainer::read` at **0x180003AF0** reads **140 bytes per element**. The corresponding writers at **0x180003F70** and **0x180003EB0** use the same strides. Container chunking divides the memory allocation into blocks but inserts no serialized count or separator between these records. This clause assigns record boundaries, not coordinate units, field types or element-connectivity bit meanings.

For the installed profile, the six delegated array counts at prefix-word indices **0, 1, 2, 5, 6 and 7** are zero. The framer stops on positive values because their additional nested data are not implemented here. The full-read path in **`cosworks.dll` at 0x180EA3FA0** continues from stream offset 604 through a 400-byte state area before reading the mesh arrays:

| Stream offset | Bytes | Content |
|---:|---:|---|
| 604 | 56 | Fourteen raw capacity/state words copied to native globals |
| 660 | 144 | Opaque temporary-buffer bytes |
| 804 | 16 | Raw native state block |
| 820 | 40 | Raw native state block |
| 860 | 144 | Opaque temporary-buffer bytes |
| 1,004 | `node_count × 52` | Node records |
| after nodes | `element_count × 140` | Element records |
| after elements | `prefix_word[8] × selected_stride` | Auxiliary records described below |
| after auxiliary records | `prefix_word[9] × 144` | Additional raw records |
| after additional records | variable | Selected auxiliary vectors |
| after vectors | 48 | Version/state trailer for the supported interval |

The native node post-read helper **0x180EA9370** retrieves model context and updates runtime globals; it is not assigned extra serialized bytes. Variable auxiliary vectors use leaf **0x180EA3880..0x180EA389D**, which dispatches read mode directly to the supplied CFile virtual read at vtable offset `0x80` with the requested byte count; its other branch dispatches write mode at `0x88`. Both leaf branches are retained, avoiding an incomplete single-path disassembly.

**Auxiliary-record stride is selected by native range checks.** The loader first reads the whole block at **288 bytes per record**. For each record it interprets signed range words at offsets **16 and 20** as `low` and `high`, requiring:

- `0 <= low <= element_count + 1000`;
- `0 <= high <= element_count + 1000`;
- `low != 0` or `high == 0`;
- `low <= high`.

If any record fails, it seeks back to the saved block start and rereads the entire block at **284 bytes per record**. It maps the old range words at offsets **12 and 16** to runtime offsets 16 and 20, copies the 256-byte old region at offset 20 to runtime offset 24, and maps the final old words at offsets 276/280 to runtime offsets 280/284. The four-byte expansion is in memory; it does not consume four extra source bytes per old record. Subsequent runtime range normalization is distinct from wire framing and is not applied to the preserved raw records.

Each selected auxiliary record begins with a signed identifier and signed vector length. In record order, the loader reads `length × 4` vector bytes only when the identifier equals that record's one-based index. All installed selected records have matching identifiers and zero lengths; populated vectors are tested synthetically. The other delegated collections are outside this profile. The model preserves opaque record bytes, including stored pointer-like words, without treating them as valid process addresses.

The trailer contains a four-byte stored version, two further four-byte words, two eight-byte patterns and five final four-byte words: **48 bytes** total. Their native destinations, in order, are writer version `18254E3B0`, `18254E428`, `1827AA298`, `18254E430`, `18254E438`, `1827AA24C`, `1827AA288`, `1827AA28C`, `1827AA294` and `18254E42C`. Numeric/engineering interpretations beyond the paired version remain open. Native versions **at least 890104** read an additional final word; the reproduced profile therefore requires **800050 <= version < 890104** and stops on other intervals. Equality of trailer and paired MAS version, and exact EOF, are validation guards in the reconstructed reader; they do not imply that the native loader enforces those checks.

All installed GEN images now reach exact EOF:

| Fixture | Nodes / elements | Selected auxiliary stride / count | Additional 144-byte records | Trailer stream offset | Exact stream bytes |
|---|---|---|---:|---:|---:|
| Composite, 16 plies | 10,553 / 5,232 | 284 / 2 | 0 | 1,282,808 | 1,282,856 |
| Composite, 8 plies | 10,553 / 5,232 | 284 / 2 | 0 | 1,282,808 | 1,282,856 |
| Benchmark plate | 24,140 / 14,642 | 288 / 1 | 3 | 3,306,884 | 3,306,932 |

The composite 288-byte trial fails the native range predicate and triggers the 284-byte retry. The benchmark trial passes. Strides are not guessed from remaining length or file version alone. Combined images span **5,872,644 bytes**, including **2,352,792 node-record bytes** and **3,514,840 element-record bytes**, totaling **5,867,632 mesh-record bytes**. Their raw record boundaries are verified; these byte totals are not a claim that every mesh field, padding byte or header value is semantically decoded.

`solidworks/simulationgen.py` composes the paired context/prefix reader with these blocks, retaining raw records, selected auxiliary stride, vector spans, trailer and exact end. It explicitly reports framing complete and field semantics incomplete. Five focused tests cover both auxiliary strides selected by the predicate, a populated vector, short/excess data, version disagreement and supported-profile guards. `simulation_gen_evidence.py` retains **seven complete native ranges/leaves** across the caller and mesh provider, source/module hashes, all block intervals/hashes and all three installed replays in `simulation-gen-native-evidence.json`. Runtime model validation, array transformations, unimplemented delegated collections, other versions and remaining field semantics are still open. No installed native target was executed and no all-project coverage percentage is regraded.

All **1,547 regression tests pass** (196.907 seconds). The native/corpus verifier passes; specification lint reports zero findings (665 headings, 96 anchors).

Evidence: C-479.

### 14.104 Simulation mesh activity and element-to-node references

*Status: Named accessor-bound fields and all installed references verified; remaining record fields and flag bits partial · Evidence: C-480*

The GEN records framed in 14.103 expose these fields through named `cwmeshcontainer.dll` accessors and their native consumers. Offsets below are relative to an individual node or element record, not the GEN stream.

| Record | Offset | Width | Field / native evidence |
|---|---:|---:|---|
| Node, 52 bytes | 0 | 1 | Activity byte; `CNodeContainer::compress` tests zero/nonzero |
| Element, 140 bytes | 0 | 1 | Activity byte; the same compaction consumer skips zero elements |
| Element | 23 (`0x17`) | 1 | Unsigned stored node-list count; `GetTotNumNodesIgnoringRepeated` at **0x180002CA0** returns this byte |
| Element | 64 (`0x40`) | 32 | Eight signed four-byte slots before the node list; `elis` at **0x1800036F0** returns a signed-int reference at `record + 64 + 4 × slot` |
| Element | 96 (`0x60`) | 44 | Eleven signed four-byte node-list slots; `nodlist` at **0x180003AB0** returns a signed-int reference at `record + 96 + 4 × slot` |

The activity interpretation is a consumer predicate, not a normalization rule for stored flags. The benchmark contains **133 nodes with activity byte 3**; the other **45,113 nodes** have byte 1. All **25,106 elements** have activity byte 1. The model preserves the exact byte and reports `active = byte != 0` separately. Bit meanings and additional flag combinations remain open.

Container index operators at **0x180002B20** (elements) and **0x180002B40** (nodes) take a zero-based record index. They select allocation chunks of 8,192 elements or 16,384 nodes and then apply the 140/52-byte stride. These allocation boundaries add no bytes to the serialized arrays. The slot accessors themselves do not enforce an in-record upper bound. Eight `elis` slots and eleven node slots describe the distinct stored spans before the next field/record boundary; they are not a claim of native bounds checks or a complete legal-index contract.

Native **`CNodeContainer::compress` at 0x180003480** walks the leading number of node-list slots given by byte 23 for nonzero-activity elements. Its node remapping table access is `mapping + 4 × (node_id - 1)`, establishing **one-based node references**. It changes an ID only when the corresponding remapping entry is nonzero. The inspected accessor returns the stored count directly; this model does not deduplicate, sort or rewrite the selected IDs merely because the exported count method contains the words `IgnoringRepeated`. Native compaction, graph mutation and any coordinate movement are outside this projection.

The installed references verify this interpretation:

| Fixture | Element count | Stored count per element | Selected node references | Invalid selected references |
|---|---:|---:|---:|---:|
| Composite, 16 plies | 5,232 | 6 | 31,392 | 0 |
| Composite, 8 plies | 5,232 | 6 | 31,392 | 0 |
| Benchmark plate | 14,642 | 10 | 146,420 | 0 |

All **209,204 selected IDs** lie in `1..paired_node_count`. This is an observed consistency check, not a native validity guarantee. Unused slots are retained independently and do not become references solely because they occupy the same array. The `elis` words are retained with the signed-int type established by the accessor, but their engineering/adjacency roles remain unresolved. Element-type enums, connectivity ordering conventions, coordinate fields and units are not inferred from the observed counts 6 and 10.

`solidworks/simulationmesh.py` provides `node`, `element` and `project` over the unchanged framed record bytes. It preserves activity bytes, all `elis`/node slots and duplicate selected IDs; optionally reports out-of-range selected IDs against a supplied node count; and stops when byte 23 exceeds the eleven slots available in a record. It leaves the full source record available, marks field semantics incomplete, and does not apply native compaction. Five focused tests cover nonboolean activity values, one-based bounds and duplicates, unused-slot preservation, signed `elis` words, slot limits and projection/record guards.

`simulation_mesh_evidence.py` pins **six complete native ranges/leaves**, their export identities and source/module hashes, the preceding framing evidence, and all installed field/reference checks in `simulation-mesh-native-evidence.json`. Across the three images, the activity/count fields occupy **95,458 bytes** and selected node IDs occupy **836,816 bytes**. These **932,274 accessor-bound bytes** have stronger field attribution than the raw record framing; remaining bytes and meanings remain partial. No global/all-project percentage is regraded, and no installed native target was executed.

All **1,552 regression tests pass** (254.386 seconds). The native/corpus verifier passes; specification lint reports zero findings (666 headings, 96 anchors).

Evidence: C-480.

### 14.105 Simulation node coordinate component slots

*Status: Native coordinate-slot type/order and installed stored values verified; units, frame and deformed runtime output unresolved · Evidence: C-481*

The 52-byte node records in 14.103 contain three coordinate component slots:

| Node-record offset | Width | Representation | Consumer order |
|---:|---:|---|---:|
| 32 (`0x20`) | 4 | Little-endian IEEE 754 binary32 | First |
| 36 (`0x24`) | 4 | Little-endian IEEE 754 binary32 | Second |
| 40 (`0x28`) | 4 | Little-endian IEEE 754 binary32 | Third |

Installed **`cwgeopost.dll!CDgxGeoPostDoc::GetDeformedNodeCoord` at 0x180014740** obtains node references through `CPostNodeContainer::operator[]`, reads these slots in order with `MOVSS`, promotes each using `CVTPS2PD` and appends the low binary64 result to its output vector. The native slot loads are at **0x18001485C**, **0x180014896** and **0x1800148D0**. This establishes their binary32 coordinate representation and emitted component order, independently of plausible-looking sample values.

The wrapper's native constructor at **`cwmeshcontainer.dll` 0x180002600** stores the supplied container pointer. Its index operator at **0x180002B90** follows that pointer, obtains the node allocation table at container offset `0x20`, selects the zero-based index's 16,384-node chunk, and returns the 52-byte-stride record address. These are the same record layout and container family read directly from GEN; there is no separate post-node record serialization in this accessor.

**Stored components and returned deformed coordinates are distinct observations.** The named consumer first invokes `GetPlotData`, then reads the runtime node slots, and finally invokes `RestoreFromDeformation`. The restore wrapper at **0x18001DD30** calls `ReadBackUpNodeFile`. Thus the consumer's output can reflect runtime plot/deformation state. This clause identifies the stored slots and types but does not assert that their on-disk values equal a particular call's deformed output, recreate the plot state, or execute the backup/restore path. Axis labels, units and coordinate frame are not assigned by the three slot loads.

`solidworks/simulationmesh.py::node` now exposes `coordinate_patterns` as the exact three unsigned 32-bit stored patterns and `coordinate_components` as convenience decoded binary32 values. It retains the full raw record, leaves `coordinate_units` and `coordinate_frame` absent in meaning (`None` in the model), and continues to report field semantics incomplete. The patterns remain authoritative for signed zero and NaN payloads; convenience Python floating-point values are not a bit-exact emulation of the native conversion's MXCSR controls, exceptional values or traps. No deformation, unit conversion or axis transformation is applied.

Across all **45,246 installed nodes**, this identifies **135,738 stored components**, occupying **542,952 bytes**. All observed components are either finite normal values (**132,976**) or positive zero (**2,762**). No installed negative zero, subnormal, infinity or NaN is observed; synthetic tests preserve their source patterns without promoting an unobserved native arithmetic profile. The two composite fixtures each have 31,470 finite-normal components and 189 positive zeros; the benchmark has 70,036 and 2,384, respectively. Component minima/maxima and source/component hashes are retained as raw-coordinate observations, not physical dimensions with inferred units.

`simulation_coordinate_evidence.py` pins **four complete native ranges/leaves**, export identities, source/module hashes and the preceding GEN framing evidence, together with every installed component span's aggregate hash and classifications, in `simulation-coordinate-native-evidence.json`. Five focused tests cover field order/width, signed zero and subnormal bits, nonfinite payload preservation, unresolved unit/frame context, unchanged source bytes and projection. Remaining node words, element fields, activity-bit meanings and runtime transformations remain open. No installed native target was executed and global/all-project coverage percentages remain unchanged.

All **1,557 regression tests pass** (185.719 seconds). The native/corpus verifier passes; specification lint reports zero findings (667 headings, 96 anchors).

Evidence: C-481.
