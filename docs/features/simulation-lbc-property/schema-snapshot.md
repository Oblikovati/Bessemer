### 14.330 LBC manager metadata reader and symbolic word-array grammar

*Status: retained static field-order recovery; stream-literal text, several threshold values and native payload validation remain open · Evidence: C-707*

The LBC manager's read path invokes **0x18108CA50** before its load collections (14.325). The retained C-690 decompilation of this reader opens a stream using literal address **0x181F53138**, creates a read-mode persistence stream, and reads the following sequence. This clause does not bind that literal to the inventory name `Property`; a fresh literal binding and exact native payload replay are required before applying this grammar to those streams. Runtime owner offsets below identify destinations, not stream offsets.

All scalar and array elements in this reader use the four-byte integer extraction overload established in 14.262. The version argument is the preserved floating value described in 14.325. Write **D(address)** for the double constant stored at a listed address; unresolved values remain symbolic rather than guessed from stream lengths.

| Order | Guard | Runtime destinations in wire order |
|---|---|---|
| 1 | Always | **+0x438, +0x43C, +0x440, +0x46C, +0x478, +0x47C, +0x480, +0x494, +0x484, +0x488, +0x48C, +0x490, +0x508** |
| 2 | v > D(**0x181F5AAC8**) | **+0x4F8** |
| 3 | v >= D(**0x18201A2E0**) | **+0x550** |
| 4 | v >= D(**0x18201A2E8**) | **+0x498** |
| 5 | v >= D(**0x18201A2F0**) | **+0x470, +0x474**; ten words at **+0x444..+0x468**; variable-width array at **+0x49C** as below; five words at **+0x4E0..+0x4F0**; one word at **+0x4F4** |
| 6 | v >= D(**0x18201A2F8**) | **+0x520, +0x524, +0x528** |

The thirteen base words occupy **52 bytes**. Their destination order is not ascending: in particular, **+0x494 precedes +0x484**. Sorting destinations would change the decoded values. Each array advances its runtime destination by four bytes, with no stored count read by this routine.

**Variable array selection.** Inside order 5, the count begins at **14**, increases to **15** when **v >= D(0x181F5AB90)**, is then overwritten with **16** when **v >= D(0x18201A318)**, and is finally overwritten with **17** when **v >= D(0x18201A320)**. These tests execute in that order; they are assignments/overrides, not additive count increments. The final count determines how many words are read from **+0x49C** with stride four. The count is not inferred by consuming the remainder of the stream.

Including its two leading words, ten-word array, variable array, five-word array and final word, order 5 consumes **128, 132, 136 or 140 bytes** for counts **14, 15, 16 or 17**. With every optional group enabled and count 17, the complete listed sequence occupies **216 bytes**. This is a conditional layout calculation, not a claim that every guard combination corresponds to a real release or that a 216-byte stream proves its version. No alignment, inter-array padding, text or embedded object is read within this retained function.

**Evidence and limits.** C-707 matches the reader's decompilation SHA-256 to C-690, checks every directly read destination in order, and verifies the fixed-loop counts, variable-count overrides and array-width calculations. It emits a machine-readable ordered grammar in `cosworks-lbc-metadata-layout-static-evidence.json`. Unknown field meanings and unknown numeric thresholds remain explicit. This recovery uses retained source only, with no installed module reads, fresh corpus extraction, target execution or native coverage credit. It neither completes the binary LBC families nor supplies the missing literal/version/native validation needed for a production decoder.

Evidence: C-707.

### 14.334 Bound LBC Property thresholds, decoder and native mismatches

*Status: installed literal and all eight thresholds verified; 338 of 356 native streams complete under retained candidates; 18 version/layout mismatches remain explicit · Evidence: C-710*

Static reads bind the metadata reader **0x18108CA50** from 14.330 to literal **Property** at **0x181F53138**. C-710 verifies the installed `cosworks.dll` SHA-256, reader byte-span hash and retained decompilation hash against C-690. Its eight raw double constants resolve every symbolic threshold from 14.330–14.331:

| Address | Value | Read condition/effect |
|---|---:|---|
| **0x181F5AAC8** | **800027** | **v > 800027** adds word **+0x4F8** |
| **0x18201A2E0** | **800060** | **v >= 800060** adds word **+0x550** |
| **0x18201A2E8** | **820010** | **v >= 820010** adds word **+0x498** |
| **0x18201A2F0** | **840004** | **v >= 840004** adds the extended array block |
| **0x181F5AB90** | **890620** | Inside that block, **v >= 890620** raises initial variable count from 14 to 15 |
| **0x18201A318** | **891000** | Later override to count 16 at **v >= 891000** |
| **0x18201A320** | **891101** | Final override to count 17 at **v >= 891101** |
| **0x18201A2F8** | **840013** | **v >= 840013** adds words **+0x520/+0x524/+0x528** |

Together with the exact wire/destination order in 14.330, these give complete reader-selected record sizes:

| Integer version | Bytes |
|---|---:|
| At most 800027 | 52 |
| 800028–800059 | 56 |
| 800060–820009 | 60 |
| 820010–840003 | 64 |
| 840004–840012 | 192 |
| 840013–890619 | 204 |
| 890620–890999 | 208 |
| 891000–891100 | 212 |
| At least 891101 | 216 |

`solidworks/cosworkspersist.py::read_lbc_property(data, version)` implements this bounded grammar with an explicit nonnegative signed-i32 integer version, retains ordered destination offsets, signed values and every raw byte, and requires the exact version-selected size. It neither sorts destinations nor derives a count by dividing the remainder. Unsupported sizes fail rather than silently selecting another version or truncating the variable array. The member meanings remain unknown.

**Native replay.** The exact C-590 inventory scope **Cosworks/StudyManager/(StudyNumber-N|SimStudy-N)/LBCManager/Property** contains **356 streams / 46568 bytes**: **354 StudyNumber** occurrences and **two SimStudy** occurrences. Fresh extraction rechecks each outer envelope and nested stream hash. Under retained primary/legacy/per-study candidate arguments, **338 streams / 42896 bytes** complete with exact size and byte-for-byte signed-word reconstruction. Three focused tests pass for all sixteen threshold boundary points, signed values, destination order/array endpoints, every shorter length for a modern record, trailing bytes and invalid version arguments. The existing twelve primitive decoder tests also pass.

**Eighteen unresolved records.** The remaining **18 streams / 3672 bytes** are each **204 bytes**, while their candidate set includes **890620**. Twelve have only that candidate; five also retain legacy **700010**, and one also retains **700329**. Version **890620** selects **208 bytes** in the verified installed reader. These records therefore do not validate that candidate's layout. A lower, unstored version could make the size agree, but C-710 does not invent such an argument or declare those streams complete. Stale metadata, historical producer behavior and runtime-selected version/context remain possible explanations requiring evidence. The size mismatch alone proves none of them.

All candidate successes and failures, complete raw payloads, runtime destination/value lists, installed constants and source hashes are retained in `cosworks-lbc-property-native-evidence.json`. The native executable is not run. The `CosmosXpress` ancestry is outside this replay, and the generic stream basename `Property` does not authorize this grammar for other managers. This closes the five unknown threshold values and literal-text binding noted in 14.331, while retaining the 18 native mismatches, version-dispatch uncertainty, physical meanings, writer behavior and full installed-format coverage as open. Global semantic coverage credit remains zero.

Evidence: C-710.

### 14.335 LBC Property writer fixed modern layout

*Status: installed writer field order recovered and cross-checked with eight retained modern payloads; historical mismatch cause remains unresolved · Evidence: C-711*

The write branch of manager **0x18108DA00** invokes **0x18108EF30** before its counted load collections. New static analysis of **0x18108EF30–0x18108F181** binds its literal to **Property**, its installed byte-span hash to the same module SHA-256 as C-710, and its retained decompilation to the following fixed write sequence:

| Order | Four-byte source members |
|---|---|
| 1 | **+0x438, +0x43C, +0x440, +0x46C, +0x478, +0x47C, +0x480, +0x494, +0x484, +0x488, +0x48C, +0x490, +0x508** |
| 2 | **+0x4F8, +0x550, +0x498, +0x470, +0x474** |
| 3 | Ten words from **+0x444** through **+0x468**, stride four |
| 4 | Seventeen words from **+0x49C** through **+0x4DC**, stride four |
| 5 | Five words from **+0x4E0** through **+0x4F0**, stride four |
| 6 | **+0x4F4, +0x520, +0x524, +0x528** |

The writer opens the named stream and constructs persistence mode **0**. Once a stream is available, these **54 words / 216 bytes** are emitted without version comparisons in this routine. The loop counts are fixed **10**, **17** and **5**. There is no stored element count or inter-array padding in the recovered sequence. Primitive insertion uses the four-byte overload from 14.262; its native I/O-status limitations still apply.

**Reader agreement.** This independently recovered order equals the reader's **v >= 891101** profile from 14.334, including the nonascending order of **+0x494** before **+0x484**. C-711 rechecks all eight retained **216-byte** native payload hashes, maps their decoded words to those runtime members, and reconstructs each input exactly in the writer-derived order. This is independent field-order reconstruction, not execution of the native writer or proof of a successful document save.

**Historical discrepancy is still open.** The current fixed writer provides no version-890620 equality branch that could explain the eighteen **204-byte** records from 14.334. It cannot establish the layout emitted by a historical build, the provenance of a stored version, or the version actually passed during native loading. Those eighteen records remain mismatches under the retained candidate set; this clause does not relabel them as complete or substitute an unstored version. Locating a historical writer/reader or establishing the runtime version path remains necessary to distinguish the possible explanations.

C-711 retains the module hash, writer range/raw hash, 127 decoded instructions, decompilation hash, ordered member sequence and eight native identities in `cosworks-lbc-property-writer-native-evidence.json`. Ghidra operated on the existing project in read-only/no-analysis mode; the target DLL was not executed. Field business meanings, native save equivalence, historical forms and full installed-format coverage remain incomplete. Global semantic coverage credit remains zero.

Evidence: C-711.
