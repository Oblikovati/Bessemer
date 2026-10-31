### 14.514 Runtime solver element-name lookup and wire-binding boundary

*Status: complete numeric-code/name helper bound; source file field and GEN record association remain unproven — Evidence: C-896*

Installed **cosworks:180F7C3A0..180F7C3F5** searches a signed32 code array at **18254F0F0**, bounded by address18254F1B8. It uses the first matching array index to select a narrow name pointer from the separate table at **18254EF60**, then copies bytes through the terminal zero into the supplied destination. Both return paths are retained; stopping at the first RET would omit the copy path.

The33 nonnull name entries establish this installed lookup profile:

| Code | Native name | Code | Native name | Code | Native name |
|---|---|---|---|---|---|
|1 |TRUSS2D |8 |TRUSS3D |3 |BEAM2D |
|4 |BEAM3D |9 |PIPE |29 |ELBOW |
|14 |BOUND |18 |SPRING |21 |MASS |
|27 |GENSTIF |42 |PLANE2D |11 |SHELLAX |
|43 |SHELL3T |44 |SHELL4T |53 |SHELL3L |
|54 |SHELL4L |63 |SHELL3 |46 |SOLIDL |
|31 |RLINK |34 |CLINK |28 |RBAR |
|36 |HLINK |37 |GAP |98 |TETRA4 |
|45 |SOLID |2 |TRIANG |91 |SHELL9L |
|93 |SHELL9 |90 |SHELL6T |64 |SHELL4 |
|92 |TETRA10 |99 |SHELL6L |94 |SHELL6 |

The full loop covers50 slots. The last17 code entries are0 and their name pointers are null. Thus a query for0 reaches a matched null source, rather than the ordinary unmatched path. The projection retains that state without predicting a successful copy or executing the native access. A nonzero unmatched code performs no destination writes and returns0. A successful named copy also returns0, so that return value alone does not distinguish those outcomes. The inspected helper has no recovered destination-size argument or check; the independent projection requires enough supplied buffer bytes and preserves the suffix after the copied terminal zero.

The static caller at **180F95B90..180F99E73** obtains the code through **180EA5B80**, supplying runtime family **12**, an item ordinal and column **2**, and passes the returned EAX directly to the name helper's EDX. The query wrapper applies its family1 column adjustment only for family1; family12 uses the supplied column unchanged. Its runtime cache/availability checks remain part of that source path. This binds a runtime query relationship, **not an independently decoded file field**.

In particular, these solver-name codes must not be assigned to a GEN element field merely because a raw value happens to match. The stored GEN profiles from14.103–14.105 and this lookup have no verified field bridge in C-896. Nor is the nearby material-identifier pointer pool from14.513 the name table used by this helper. Those associations remain explicitly unbound.

**solidworks/simulationelementnames.py** provides a raw-destination-preserving conditional lookup projection with **wire_field_binding=false**. Three tests exercise all33 names, unchanged unmatched destinations, matched-null-source distinction, signed32/input bounds and insufficient destination rejection. **simulation_element_names_audit.py** verifies the complete helper bytes, all50 paired code/pointer slots, all names, full retained caller/query instruction probes and source hashes. No native fixture is assigned a new type, no target executes and no global coverage credit or complete installed-format schema claim is made.

Evidence: C-896.

### 14.515 EGR family12 source and conditional solver-type row field

*Status: logical suffix/default record profile and conditional column2 binding established; populated native EGR fixture and complete file grammar absent — Evidence: C-897*

The runtime family12 source retained in14.514 now has a logical-file binding. **cosworks:180EA5D90..180EA679C** dispatches selector12 through jump-table entry **180EA66F0** to **180EA60BF**. That case copies suffix CString global **182748AD8** and reaches the common setup with **32 columns**. Complete initializer **180324590..180324A12** constructs that exact global from UTF-16 **`.EGR`** at181F97490. The installed selector therefore denotes an EGR table rather than a GEN element field. The logical-name/storage mapping rules remain those of14.100; this clause does not assign an EGR body to every arbitrary file with that extension.

Common setup **180EA32E0..180EA33CE** initializes a physical record size rounded up to a128-byte boundary from four times the column count. With32 columns this is **128 bytes**. Its initial field starts advance by four bytes. The caller/query chain from14.514 selects family12, an item ordinal and column2, without family1's extra column adjustment. Under an explicitly established **32-column/four-bytes-per-column profile**, that code is the signed32 word at **row byte offset4**. Its supplementary name uses the33-code lookup from14.514.

This is a conditional record binding, not a universal assumption about loaded EGR metadata. Existing-file header reader **180EA8870..180EA8B25** can replace the loaded field count and widths, constructing field starts from those widths. Therefore the initializer alone cannot prove that column2 retains byte offset4 after every load. The row projection requires the caller to provide the explicit loaded width profile; different widths/counts remain unsupported. The EGR file header, row-directory state, allocation/context variants and complete file framing are not implemented by this row-only view.

**solidworks/simulationegr.py::row** accepts exactly128 bytes with32 explicitly supplied four-byte widths, preserves the complete row, reads the raw second-word bytes and supplies the conditional solver-code/name view. Unknown codes retain the unmatched status; code0 retains14.514's matched-null-source status without attempting a native pointer read. It returns **complete_file_schema=false**. The remaining31 words stay raw, and this binding does not change the GEN projection or infer a GEN type offset.

C-897 freshly surveys the same **three retained CWR files** used by the GEN/MAS audit, verifying whole-file hashes and current root-stream inventories. None contains an EGR stream. Consequently **zero native EGR fixtures** are claimed; the row tests are explicitly synthetic, and the survey does not prove absence in every installed or external result file.

**simulation_egr_profile_audit.py** verifies the selector jump entry, complete load/setup/header/suffix initializer probes, exact EGR literal and global binding, prior lookup provenance and the retained CWR inventories. Three new tests cover second-column selection/raw preservation, known/unknown/zero code distinction and explicit width/row-size guards. No target executes, no complete EGR file is claimed and no global coverage credit is added.

Evidence: C-897.

### 14.516 Conditional dense EGR table-body framing

*Status: bounded dense-body profile implemented from shared native record equations; native EGR production and complete outer file remain unverified — Evidence: C-898*

**simulationegr.dense_body** extends14.515 to an explicitly **unwrapped table body**, excluding any copied-file transfer prefix. Its supported profile has128-byte physical records,32 loaded field descriptors whose widths are all4, a nonnegative bounded capacityN, used-row countN and zero free-row count. These restrictions define the independent reader's supported dense profile; sparse/deleted-row states, other widths/counts and outer transfer framing remain unsupported.

The shared native initializer/header methods **180EA8740** and **180EA8870** place field descriptors in whole records after the header. Thirty-two eight-byte descriptor pairs occupy256 bytes, or two128-byte records. The row directory then occupies enough whole records forN four-byte entries. Thus this profile has:

| Region | Body-relative boundary |
|---|---|
|Header |0..128; first nine words retained separately |
|32 descriptor pairs |128..384 |
|Row directory, including preserved alignment bytes |384..384+128*ceil(N/32) |
|Physical row data |Starts at384+128*ceil(N/32), followed byN128-byte records |

The writer **180EAA8E0** independently supplies the nine header words and descriptor-width differences. Descriptor tags remain raw integers. The reader preserves header bytes after the first36 and unused directory alignment bytes; neither is assumed zero or omitted from reconstruction.

Native **180EA7750** obtains an item's physical row number from the directory, with an equivalent cached-directory route. **180EA7860** subtracts1 from that physical number and adds the native data-record base before calling180EA9240. The reconstructed dense profile therefore requires a permutation of1..N and selects rows in **directory order**, retaining physical records separately. It does not assume logical ordinal equals physical position. The permutation and exact endpoint checks are independent-reader guards, not newly claimed native validation behavior.

Every selected row retains14.515's conditional second-column solver-code projection and all128 original bytes. The parser exposes **framing_complete_for_profile=true** only after exact reconstruction, while keeping **complete_file_schema=false**. N=0 requires no directory records; N=32 uses one directory record; N=33 uses two. The body must end after the supported physical records, without guessed trailers.

**simulation_egr_table_audit.py** rematches five complete native metadata/record functions and the complete four-instruction row-read leaf. Four explicitly synthetic fixtures cover0,2,32 and33 rows, including directory order[2,1], retained nonzero padding and exact reconstruction. Seven total EGR tests cover row projections, code states, header/width/count guards, invalid directories, truncation and trailing bytes. **Native EGR fixture count remains0.** No target executes, no complete outer EGR file or sparse-table support is claimed, and global coverage credit remains zero.

Evidence: C-898.

### 14.517 Constructor-bound flat EGR file transport

*Status: raw CFile route and conditional flat-file framing bound; copied CWR EGR envelope and populated native EGR fixtures remain unproven — Evidence: C-899*

The EGR table reader's file object is now independently bound. Owner constructor **cosworks:180EA0260..180EA03D8** constructs **54 objects of40 bytes** beginning at owner+28 through constructor thunk **181711C1D** and destructor thunk181711C35. The thunks import **mfc140u.dll ordinals488 and1120**, resolving in the installed provider to **180230260** and180230550. The constructor installs vtable **180335988**; its complete-object locator1803723A8 resolves exact RTTI name **`.?AVCFile@@`**. These are object/class facts, not additional wire fields.

The family-indexed open method **180EA9110..180EA9231** selects the object at owner+family*28. Its table slots resolve as follows in the verified provider:

| Virtual slot | Provider target | Verified raw I/O relationship |
|---|---|---|
|58 |1802306E0 |Open path includes imported CreateFileW |
|68 |1802309F0 |Seek uses imported SetFilePointer |
|80 |180230910 |Read uses imported ReadFile |
|88 |180230970 |Write uses imported WriteFile |

The provider constructor initializes its optional broker pointer+20 to zero. The verified ordinary path uses the raw filesystem handle; externally changed broker state, substituted file objects and other providers are not generalized. The common record reader computes file positions from **(physical record number-1)*record bytes** and uses the bound seek/read slots. On this constructor-bound route, record1 begins at **file byte0**. No copied-file count word is inserted by the observed table read path.

Accordingly, **simulationegr.dense_flat_file** now provides complete framing for14.516's supported dense profile on this explicitly selected transport. It requires **constructor_bound_flat=true**, reports table_file_offset0 and transfer_prefix_bytes0, preserves every input byte and returns **framing_complete_for_file_profile=true** only after the underlying dense profile's exact endpoint/reconstruction checks. It retains **complete_file_schema=false** because other profiles, metadata variants and field meanings remain unresolved.

This does **not** assign14.101's copied-file envelope to an EGR stream in a CWR container. A prepended count1 word is rejected by this flat profile, whose first header word must be128. The3 retained CWR samples still contain no EGR stream; no native flat EGR fixture or historical producer execution has been established. The earlier outer-transport uncertainty is narrowed only for this constructor-bound flat route.

**simulation_egr_flat_file_audit.py** verifies the owner array/constructor links, both import ordinals and targets, provider constructor's installed table, CFile RTTI, four virtual targets, complete native probe bodies and named raw I/O imports. Nine EGR tests pass, including explicit route selection, rejection of a copied-prefix image, directory boundaries/permutations and byte preservation. No target executes and no global coverage credit or complete installed-format schema claim is made.

Evidence: C-899.

### 14.518 EGR live rows, deleted-record chains and unallocated capacity

*Status: allocated-record profile with directory holes/free chains implemented; native EGR fixtures and complete field semantics remain absent — Evidence: C-900*

The common table's row state distinguishes **capacityN**, **live countM**, **deleted/free countF**, **free-chain head** and a **highest nonzero logical directory ordinal**. Their retained header words are at body offsets04,10,14,18 and20 respectively, in hexadecimal. The existing32-field/four-byte-width and128-byte-record profile remains explicit.

Native deletion routine **180EA75B0..180EA769C** obtains a logical item's physical record, clears its directory entry through **180EA76A0..180EA7744**, reads that physical record, overwrites its first word with the previous free-chain head and writes the record back. It decrements live count, increments free count and makes the deleted physical record the new head. If the removed logical ordinal was the stored high-water ordinal, it scans backward for a nonzero directory entry. The remaining124 bytes of a deleted record are not erased by that first-word update.

Allocation routine **180EA6F60..180EA6FF9** returns0 when live count equals capacity. With a nonzero free head, it reads that record's first word as the next head, decreases free count and increases live count. With no free head, it selects the next newly allocated physical record and advances the record count. These mutations and the header reader's record-count equation establish the allocated extent **M+F**, which can be smaller than capacityN.

**simulationegr.allocated_body** and **allocated_flat_file** now retain this conditional profile:

| State | Interpretation |
|---|---|
|Nonzero logical directory entry |Select a live physical record, keeping the logical ordinal and physical number separate |
|Zero logical directory entry |No live record for that logical slot |
|Free-chain record |First signed word is the next physical free-record number;0 terminates the chain |
|Unallocated capacityN-(M+F) |No additional physical row bytes are invented |

Physical data starts after the capacity-sized,128-byte-aligned directory from14.516, but contains **M+F physical records**, not necessarilyN. The parser retains all allocated record bytes. Only live directory-selected rows receive solver-code/name projections. Deleted records retain their raw bytes and next pointer; a stale second word is **not** interpreted as a current solver code.

The supported profile checks live-directory count and uniqueness, pointer bounds, exact free-chain lengthF, cycles, active/free collisions, the logical high-water value, complete partition of allocated physical records and exact endpoint. These are conservative independent-reader profile guards; they are not claims that the native implementation enforces the same validations or repairs malformed files. Other widths/counts, compacted/recovery variants and copied CWR envelopes remain outside the profile. The flat-file entry still requires explicit constructor-bound transport from14.517.

**simulation_egr_free_rows_audit.py** rematches the complete allocation/deletion/directory-write/header functions and physical-record-write leaf. Four explicitly synthetic fixtures cover mixed live/free/unallocated state, a multi-link free chain, never-allocated capacity and all-deleted records; the dense profile remains accepted. Thirteen EGR tests pass, including corrupt chain/directory/count/endpoint rejection and raw preservation. **Native EGR fixture count remains0**, complete_file_schema remainsfalse, no target executes and global coverage credit remains zero.

Evidence: C-900.

### 14.519 EGR group identifier and eight signed EGROUP parameters

*Status: identifier/eight-word output consumer and parameter permutation bound; individual parameter meanings and native EGR fixtures remain open — Evidence: C-901*

The complete consumer **cosworks:180F95B90..180F99E73** supplies an additional semantic bridge for the explicit four-byte-width EGR profile. It iterates positive logical ordinals and reads family12 column1 through180EA5B80. Only when that value equals the logical ordinal does it continue to obtain column2's solver type/name and read **eight fields beginning at column3** through180EA5CA0.

| Conditional EGR row bytes | Column | Supplementary consumer view |
|---|---|---|
|00..03 |1 |Signed group identifier, checked against logical ordinal for this output path |
|04..07 |2 |Solver-type code/name from14.514–14.515 |
|08..27, hexadecimal |3..10 |Eight signed32 EGROUP parameter words, retained individually in wire order |
|28..7F, hexadecimal |11..32 |Remaining88 raw bytes; purposes not assigned here |

The parameter output is **integer**, not a floating-point projection. After reading the eight four-byte values, the consumer calls the complete swap leaf **180FA5020..180FA502B** on the last two array entries. It then supplies each four-byte word to narrow format **` %d,`**, exact bytes at181FA7DB4, through180C4EA30. That formatter's import binds to **`__stdio_common_vfprintf`**. No float widening or numerical unit conversion occurs in this eight-word output loop.

The preceding exact narrow format at181FA7DA0 is **` EGROUP, %d %s ,`**. Its integer is the logical ordinal plus a caller-supplied group offset using32-bit addition; its string is the verified solver name. A final newline comes from181F9CA34. The resulting parameter column order is **3,4,5,6,7,8,10,9**, while the original stored order remains unchanged. The spelling EGROUP establishes this consumer's group-record context; it does not identify the physical name, unit or legal domain of each of the eight parameters.

**simulationegr.row** now exposes the raw first-word bytes and signed group identifier, plus eight raw parameter words and supplementary signed integer views. **egroup_export_view** separately models the known-name narrow output line, requires a positive signed32 logical ordinal and signed32 group offset, and preserves native32-bit sum wrapping. An identifier mismatch yields a skipped view. Unknown or matched-null-source type-name states from14.514 block line construction instead of inventing text from an unknown destination buffer. This is a pure projection; no solver file or external output is written.

**simulation_egroup_parameters_audit.py** rematches the full consumer, query wrappers, formatter and complete swap leaf; verifies all three narrow literals and the stdio import hint/name bytes; and checks the raw-width/argument chain. Seventeen EGR tests pass, including output permutation, signed extremes, group-number wrap, mismatch/unknown-name handling and previous framing/free-record profiles. **Native EGR fixture count remains0**, other columns and physical parameter meanings remain unresolved, complete_file_schema remainsfalse and no target executes or global coverage credit is added.

Evidence: C-901.

### 14.520 EGR metadata-driven positive-width field framing

*Status: conditional loaded-width framing; no native EGR fixture — Evidence: C-902*

For the constructor-bound flat CFile route of 14.517, the supported profile has 128-byte records and zero through 32 loaded fields. Each descriptor consists of a raw signed32 tag and a signed32 byte width. The loaded header reader at `180EA8870` reads the second descriptor word and accumulates widths into one-based field starts; four-byte spacing is an initialization profile, not a universal loaded-field rule.

Let K be the field count, N directory capacity, M live rows and F deleted rows. The field directory begins at byte128 and occupies `128*ceil(8*K/128)` bytes. The row directory follows and occupies `128*ceil(4*N/128)` bytes. Exactly M+F physical records follow. The active/free-chain partition and logical high-water requirements of 14.518 apply. The supported parser requires positive widths whose sum does not exceed128; these are support bounds, not a claim that other native configurations are invalid.

Within each live row, field i starts at the sum of earlier widths and occupies its stored width. Descriptor alignment bytes, header and directory remainder bytes, and the row tail after the summed widths shall be preserved. Tags and variable-width values remain raw; no solver type, parameter name or units shall be inferred from a changed width. Only the exact 32-field/four-byte profile receives the established row interpretation of 14.519.

`solidworks/simulationegr.py::metadata_body` and `metadata_flat_file` implement this conditional profile. Twenty synthetic tests cover the descriptor-record boundary at16/17 fields, mixed widths, zero-field metadata, raw reconstruction, allocated/free partition, invalid widths and exact endpoints. `complete_file_schema` remains false; native EGR fixture count is zero.

Evidence: C-902.

### 14.521 EGR descriptor writer assignments

*Status: static descriptor assignment loop bound; full native save and first-word meaning unresolved — Evidence: C-903*

The descriptor loop in `180EAA8E0` assigns signed32 value1 to the first word of every eight-byte descriptor (`180EAAA12`). It writes the difference between adjacent in-memory one-based field starts into the second word (`180EAAA1E` through `180EAAA33`). For the positive-width profile of 14.520, this difference reproduces each loaded width.

The loader `180EA8870` reads descriptor words at offsets `8*i+4` and accumulates them into field starts. It does not read the first descriptor word in that loop. A reader shall retain first-word bytes, including values other than1; this evidence does not establish a type enum or a rejection rule for those values. Rewriting descriptors through the observed writer loop replaces those values with1. This is a writer assignment, not evidence that existing descriptors always contain1.

`solidworks/simulationegr.py::descriptor_writer_view` returns original descriptor bytes separately from the assigned-byte projection. It does not modify the input or project a complete save. Alignment bytes after the last descriptor remain separately preserved; this loop does not assign them. Header-buffer reuse, directory writes, output errors and complete native save behavior are outside this view.

Twenty-two synthetic EGR tests pass, including noncanonical tags, canonical descriptors, an empty descriptor set and unchanged original bytes. The writer and loader instruction captures are rechecked against the installed DLL bytes. Native EGR fixture count remains zero and complete schema coverage is not claimed.

Evidence: C-903.

### 14.522 EGR live-row field ranges and missing-row limitation

*Status: native live-row byte-range calculation bound; cache and missing-row outcomes remain conditional — Evidence: C-904*

The family range reader `180EA8B30` receives inclusive first and last field indexes. For positive widths w, the zero-based start is `sum(w before first)` and the byte count is `sum(w from first through last)`. Its uncached path subtracts1 from the first field's stored one-based start (`180EA8C04`) and subtracts that start from the boundary after the last field (`180EA8C08`). The cached path performs the same boundary subtraction at `180EA8C98`, with a separate cache-buffer base adjustment. Field tags do not participate in these calculations.

A nonzero directory entry selects the physical row, independently of logical ordinal. `solidworks/simulationegr.py::live_field_range` validates the supported metadata profile and returns the exact raw inclusive range for that live row. It does not infer a type from the range length, emulate native cache state, or change the original record. Out-of-bounds ranges and zero directory entries are rejected by this independent view; these guards are not claims about native validation.

The missing-row path must not be simplified to a terminal zero result. After the directory lookup returns0, the native routine calls `181712F0C` with destination, zero and the calculated byte count at `180EA8BB7`. The next instruction is `180EA8BBC`; there is no intervening return or branch out. The uncached continuation subsequently passes the zero record number to the physical-record reader at `180EA8BDD`. The final output therefore requires additional native state and I/O behavior; this specification does not assign a final byte result for a missing row.

Twenty-five synthetic EGR tests pass, including mixed-width inclusive ranges, directory permutation, missing-row rejection and invalid ranges. Static instruction replay checks the range arithmetic and missing-row fall-through against installed DLL bytes. Native EGR fixture count remains zero and full schema completion remains unproven.

Evidence: C-904.
