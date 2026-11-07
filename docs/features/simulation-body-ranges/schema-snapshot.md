### 14.116 Simulation runtime body-type descriptor and exclusive range query

*Status: Named runtime descriptor consumer verified; conditional three-field source mapping follows in 14.117, complete descriptor serialization remains unproven · Evidence: C-492*

The exported **`GetBodyTypeByBodyId`**, **cwgeopost.dll!0x18002B250..0x18002B433**, takes a `CDspAsmComp` descriptor. It reads a 32-bit body ID at descriptor offset **0x40**, a signed start at **4** and a signed upper bound at **8**. These are **runtime object offsets**, not a newly established GEN or other file record. Either negative bound returns **-1** before callback or mesh traversal. The conditional three-field source mapping in 14.117 does not establish a complete serialized descriptor layout.

For nonnegative bounds, the query calls the same postprocessing callback mechanism as 14.115, using operation **2** and a VT_I2 initialized from the low sixteen bits of the body ID. A supplied signed-i16 result other than **-1** immediately determines the returned type flag **16**. This callback is attempted even for mode **1**, unlike `GetNodesByBodyId`, whose mode 1 bypasses it. With callback result -1, fixed modes return:

| Mode | Returned type flag |
|---|---|
| 0 | 1 |
| 1 or 2 | 2 |
| 4 | 4 |
| 6 | 8 |
| Other than 3 | -1 |

In particular, mode **5** returns -1 here, while the node-mark query initializes flag 2 for that mode. These functions cannot be replaced by one shared mode table without preserving their differences.

Mode **3** scans **`[start, upper_bound)`**, with an exclusive upper endpoint. Native instructions compare the incremented index and bound using `JGE` to skip an empty range and `JL` to continue. Within the safe nonoverflowing profile this is equivalent to **start < upper_bound**. Equal bounds scan no records. This differs from 14.115's inclusive supplied range; the projection does not normalize one convention to the other or infer a descriptor producer's intended convention.

The mode-3 scan requires nonzero element activity and equality between the descriptor's full 32-bit body-ID bits and the element's zero-extended u16 body-selection ID at offset 28. It visits the node count at offset 23 and IDs at offset 96, skipping ID zero. On the first referenced node with nonzero activity, it returns **1** for family 4, **2** for family 8 or 3, **4** for family 2, or **-1** for another family. The first successful active-node association ends the query even when its family is unrecognized. Elements with no successful node association allow the scan to continue. This function returns the flag itself; unlike `GetNodesByBodyId`, it does not return constant zero or write a node-mark array.

The application-side descriptor supplier is also traced. **cosworks.dll!0x180CB7CD0..0x180CB8880** passes a runtime list at owning-object offset **0x200** to imported `SetAsmCompList@CDgxGeoPostApp`. Its preparation path **0x180C02710..0x180C0350C** reaches population routine **0x180C9A2B0..0x180C9E2D5** after GEN/study preparation. That larger routine depends on runtime study/geometry data. This call chain alone demonstrates where the runtime list is supplied, rather than establishing its file layout. Clause 14.117 separately follows three fields through an intermediate list in one conditional branch. The remaining auxiliary bytes and descriptor fields retain their partial status; no full file descriptor grammar or coverage credit is inferred from the class name, object offsets or call sequence.

`solidworks/simulationbodytype.py` exposes `project` with explicitly supplied descriptor body ID, bounds, mode and callback result. It rejects invalid record blocks, unsafe node indices and unsupported overflow cases, retains the exclusive endpoint and reports `file_descriptor_binding = False`. It executes no callback and changes neither nodes nor elements. Negative descriptor bounds can be projected without a callback result because the native guard precedes the callback.

For all five installed body-selection queries, supplying start 0, exclusive upper bound equal to the element count, mode 3 and callback result -1 yields flags **2 and 2** for the two selections in each composite fixture, and flag **1** for the benchmark selection. These descriptor values are explicit test contexts, not recovered historical descriptors. A separate synthetic case proves the endpoint distinction: bounds **0,0** return -1 without traversal, while **0,1** permit the first record and return 1. Six focused tests cover exclusive endpoints, negative-bound callback bypass, fixed-mode differences, callback precedence, first successful node/family behavior, signed versus unsigned ID bits and safe profile guards.

`simulation_body_type_evidence.py` retains **four complete native ranges**, export/import identity, exact runtime offsets and range branches, module/source hashes, the supplied corpus queries and synthetic endpoint case in `simulation-body-type-native-evidence.json`. The supplier trace and its unproven serialized-data connection are preserved as an explicit evidence boundary. No installed target was executed and global/all-installed-project percentages remain unchanged. Descriptor serialization, actual session bounds, callback semantics and CAD-body identity mapping remain open.

Evidence: C-492.

### 14.117 Simulation auxiliary body ID and element-range field bridge

*Status: Three stored fields mapped through a conditional native list/descriptor path; all installed intervals cross-check against element IDs · Evidence: C-493*

The GEN auxiliary rows framed in 14.103 now have a stronger interpretation for three fields. The native reader normalizes the old **284-byte** profile into its **288-byte** workspace profile before these fields are consumed. Offsets below are relative to the beginning of each stored auxiliary row:

| Field | Stored 284-byte profile | Stored 288-byte profile | Representation |
|---|---|---|---|
| Body-selection ID B | 0 | 0 | Four-byte identifier; bits preserved into runtime body-ID field |
| First element ordinal L | 12 | 16 | Signed i32 |
| Last element ordinal H | 16 | 20 | Signed i32 |

For the installed rows, **L..H is a one-based inclusive element interval** and B agrees with every member element's u16 body-selection ID at offset 28. This strengthens the identifier and range roles without assigning the rest of the auxiliary row or a CAD-body identity mapping. The identifier's separate framing/vector-selection use from 14.103 is retained.

In **cosworks.dll!0x180EA3FA0..0x180EA592A**, after the GEN read and normalization, a guarded fresh-list path allocates intermediate entries. The path requires a study data object, the saved nonzero mesh count and an uninitialized destination list. It copies normalized auxiliary offsets **0, 16, 20** to intermediate object offsets **8, 12, 16**, respectively, then appends that object to the list at study-data offset **0x330**. Native stores and the **288-byte row advance** are retained in C-493. This is a conditional initialization path, not evidence that every reused runtime list is rebuilt from the current file.

In **0x180C9A2B0..0x180C9E2D5**, a separate branch selected when the runtime study word at **study offset 0xC0 equals 1** walks that list through its head at study-data offset **0x338**. It skips an entry whose two range values are both zero. Otherwise it assigns the descriptor's runtime start/upper fields to **L - 1** and **H - 1**, and the body-ID field at runtime offset **0x40** to B. The native stack descriptor base and stores at base **+4**, **+8** and **+0x40** establish these relative offsets independently of decompiler local-variable names. Other builder branches remain dependent on different runtime data and are not generalized from this mapping.

The stored and runtime intervals must remain distinct. With these supplied descriptor values, `GetNodesByBodyId`'s inclusive range can cover stored ordinals L through H. `GetBodyTypeByBodyId`'s exclusive upper-end traversal, documented in 14.116, does not visit ordinal H through this same range input. That native consumer difference is preserved; no endpoint is silently adjusted and no complete historical session behavior is inferred. The full descriptor is not a packed copy of the auxiliary row.

The two composite fixtures each have the following stored rows in the 284-byte profile:

| B | Stored L..H | Proposed runtime bounds | Elements |
|---|---|---|---|
| 1 | 1..2618 | 0, 2617 | 2618 |
| 2 | 2619..5232 | 2618, 5231 | 2614 |

The benchmark's 288-byte profile has **B = 1, L = 1, H = 14642**, giving proposed runtime bounds **0,14641**. Across all three files, the five intervals partition all **25,106 elements exactly once**, with no gaps or overlap, and every interval's element body-selection IDs equal B. This is a static field/value cross-check; the required runtime builder branch and actual historical descriptors were not observed executing. These three four-byte fields occupy **60 bytes** across the five rows, including the previously framed identifier; this is not an exclusive global coverage increment.

`simulation_aux_body_ranges_evidence.py` provides a bounded field extractor in its verifier, rechecks the two native reader/builder ranges and preserves module/source hashes, intermediate stores, descriptor-relative stores, both profile offsets, per-row hashes and element-interval hashes in `simulation-aux-body-ranges-native-evidence.json`. Separate explicitly synthetic 284- and 288-byte rows with B=7 and L..H=2..5 verify that both profiles project to runtime bounds **1,4**. The public runtime query model still reports `file_descriptor_binding = False` for a complete descriptor; this limited three-field bridge does not change that boundary. No installed target was executed. Remaining auxiliary fields, reused-list provenance, other builder branches and CAD-body mapping remain open.

All **1,616 regression tests pass** (187.611 seconds). Both native/corpus verifiers pass; specification lint reports zero findings (679 headings, 96 anchors).

Evidence: C-493.
