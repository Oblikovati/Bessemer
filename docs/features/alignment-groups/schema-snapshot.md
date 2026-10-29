### 11.351 Alignment groups, order dimensions and sketch-text kernel bodies

*Status: Native-bound ordinary modern part graph; three additional exact-EOF streams; meanings partial · Evidence: C-393*

This clause extends the ordinary modern external-XML part context above **0x27CB**, with helper +834 clear and archive conditions 1, 2, 4 and 0x10000 clear. The complete saved model/CMgr context and exact-EOF gates from 11.350 remain mandatory. It adds five concrete class readers and the guarded kernel branch of the existing direct sketch-text reader.

| Class | Primary vtable | Native serializer | Body selection |
|---|---|---|---|
| `moAlignGroup_c` | 0x4CE0C9F8 | 0x4C833270 | Feature body and direct owned collections below |
| `sgSplineLengthDim` | 0x4D1AB780 | 0x4C5A92E0 | Existing modern `sgValG1Dim` body |
| `sgBsOrdDim` | 0x4D1B4398 | 0x4C5A9750 | Existing modern `sgValG2Dim` body |
| `moDisplayOrdDim_c` | 0x4D255820 | 0x4C83AA00 | Shared display-dimension body and order suffix below |
| `moScalerLengthDim_c` | 0x4D242D90 | 0x4C83D2C0 | Existing geometric dimension body, shared with `moScalerDim_c` |

The spelling **ScalerLength** is the installed class spelling. The spline/order aliases are selected by exact native serializer and ancestry evidence, not by interpreting their names. The existing collection validators admit the two newly verified sketch subclasses only in the selected local reader scope. Dimension-handle validation likewise admits `moScalerLengthDim_c` only through the local `additional_feature_dimension_classes` extension; arbitrary dimension subclasses remain rejected. Earlier supported dimension extensions are preserved.

The alignment-group body first reads the complete `moFeature_c` layer. Its modern continuation is:

| Order | Body |
|---|---|
| 1 | Direct dimension-handle array +370: ReadCount and tagged `moDimHandle_c` entries |
| 2 | Raw i32 +3D8; direct byte array +3B8: ReadCount and exactly that many raw bytes |
| 3 | Raw u32 +40C; raw f64 +400; raw i32 +3F0 |
| 4 | Raw i32 +3F4 when version > **0x2F4D** |
| 5 | Version < **0x2F50**: body ends |
| 6 | **0x2F50 <= version < 0x2FA0**: two tagged `moDimHandle_c` pointers, then body ends |
| 7 | Version >= **0x2FA0**: direct dimension-handle array +450, with native maximum **two** entries |
| 8 | In the order-7 branch: tagged `moCompRef_c` +4C0; tagged `sgEntHandle` +4A8; two raw three-f64 vectors +478/+490 |
| 9 | In the order-7 branch, only when stored +3F4 is nonzero: one raw three-f64 point |

Both handle arrays are direct bodies without outer tags or array identity-map entries. Their tagged items retain order, repeated identities and nulls. Constructor **0x4B4EAA40** establishes the `internal_moDimHandleArray_c` table at **0x4CE0C968**, whose +28 slot selects **0x4BB44920**. That wrapper calls `suObArray::Serialize` in **sldmfcu.dll** at **0x3CB7A610**. The latter is a seven-byte tail jump to the imported core `su_CObArray::Serialize` in **swccu.dll**, **0x31D3D0C0**. Its load path reads ReadCount followed by tagged objects. These three provider layers establish the direct array grammar independently of source alignment.

The byte-array constructor in swccu.dll, **0x31D3B600**, selects table **0x31D50E78**, whose +28 slot selects **0x31D3BA70**. Its load path reads ReadCount, allocates that many bytes and reads the raw byte payload. It does not serialize each byte as a tagged object or widened integer. Count framing is structural; all byte values remain unknown. The unused constructor-created +350 object array and +390 string array belong to earlier serialization branches and add no bytes in this clause's modern scope.

Stored +3F4 remains a raw signed i32; negative or other nonzero values select the point branch without normalization. The two vectors and optional point remain fully unknown for value credit. An extended-count synthetic byte array, repeated mapped handles and every truncated payload boundary verify width, map behavior and endpoint preservation. The native maximum on the second array is enforced before reading unsupported extra entries.

`moScalerLengthDim_c` uses the already specified `Dimension_c` base, three points, optional pair of vectors, optional transform and the two-bit optional-point suffix from 11.28. Its serializer is exactly the existing **0x4C83D2C0** implementation. No extra scalar, length word or conversion record is inserted after that body.

The order display dimension shares the established display-object/item/text/dimension layers. Its primary virtual slots **+A78 / +8D8 / +8A8 return 1 / 0 / 1**. The shared load function **0x4C835790** routes +8A8-true classes into the point-reference branch above version **0x467**, already satisfied here. Thus the same modern point-reference bytes as distance dimensions are present even though the dispatch constants differ. Its suffix order is:

| Order | Body |
|---|---|
| 1 | Tagged `moDisplayOrdDim_c` shared-order pointer +FF0; tagged `suObList` +FF8 |
| 2 | Raw u16 +1008; raw f64 +1028/+1030; raw u16 +1038/+103C; raw u8 +1010 |
| 3 | Tagged `moCompFeature_c` +1000 in the ordinary condition-1/2-clear branch |
| 4 | Raw i32 +1040/+1044; tagged `moDispObjRef_c` +FB8 |
| 5 | Full u32 count, then that many tagged `moDispObjRef_c` pointers for +FC8 |
| 6 | Helper **0x4C8422D0**: no bytes when conditions 0x10000 and 2 are both clear |
| 7 | Raw i32 +FE8; raw f64 +1048 |

The fixed raw suffix values occupy **43 unknown bytes**. The pointer-array count is a full four-byte value, not ReadCount. It is bounded against remaining tag capacity before allocation. The alternative condition-1/2 branch uses a CString in place of order 3; the alternative helper branch adds a +FC0 reference. Neither is selected in this scope. Shared-order pointers may refer back into the existing object map; the decoder does not serialize a duplicate body for such a reference. Native version-history normalization and reference-owner updates consume no input. All suffix version gates precede the supported modern minimum.

The direct `sgText` serializer, primary table **0x4D1BF488**, entry **0x4C5A8520**, extends 11.32. A nonzero `moDimText_c.has_curve_data` requires the ordinary tagged `suObArray` curve-reference pointer and then the common **0x4C260570** kernel-body helper. The body is read even if that pointer is null; the subsequent native runtime cleanup does not remove its stored bytes. The existing Text CString and two raw date words follow immediately. No new attribute list is added around this kernel body.

`sketch_text_kernel_dependencies=True` enables this continuation only in the selected archive. It retains the exact suppression byte. A nonzero byte ends the kernel record; zero accepts only the strict self-describing compressed Parasolid form of 11.99. Chunk lengths, compression GUID, block sizes, zlib endpoints, transmit-file endpoint and single-BODY requirement all remain validated by `kernelarchive.body`. Unsupported noncompressed, alternate-kernel or malformed forms still stop explicitly; a plausible length alone does not enable opaque payload skipping.

The three newly complete sources contain **six** such present text bodies: four in `Baseball Bat.SLDPRT`, one in `sps_cap_a.sldprt` and one in `transformer_core.sldprt`. Their complete kernel frames occupy **4,420 bytes** and inflate to **7,084 bytes / 102 Parasolid nodes**, with **zero unknown Parasolid bytes** under the existing schema. Each frame contains exactly one BODY. `alignment-native-evidence.json` records source and frame offsets/hashes, compressed/inflated hashes, frame lengths, kernel types, node counts and unknown totals. These bodies are decoded rather than treated as known-length opaque regions.

`alignmentgroups.install` composes the five new class readers, extends the local supported dimension sets and enables the strict text-kernel flag. `resolvedfeatures.read_data(..., alignment_dependencies=True)` selects it explicitly after the existing sketch-reader installation; its staged default remains **false**. Public `read_supported` and `documented_bytes` enable the option with complete ordinary context. **`alignment_dependencies=False, pattern_dependencies=False`** reproduces the Edition 0.370 public scope. Neither global reader registration nor the inherited caller table changes.

The native alignment source contains one complete group with **nine +370 handles**, **nine byte-array entries**, zero +450 handles and stored +3F4 zero. It also contains three spline-length dimensions, two B-spline-order dimensions, nine order display dimensions and five scalar-length dimensions. Its initial alignment stop advances through these dependencies and four text kernel bodies to exact EOF. The other two sources require only the text-kernel continuation from the new set.

| Source | Decoded bytes | Complete roots | Unknown bytes | Known structure bytes |
|---|---:|---:|---:|---:|
| Learn `Baseball Bat.SLDPRT` | 710,327 | 42 | 373,164 | 337,163 |
| Dynamics `sps_cap_a.sldprt` | 156,832 | 30 | 70,522 | 86,310 |
| Dynamics `transformer_core.sldprt` | 53,292 | 24 | 23,785 | 29,507 |

The added streams total **920,451 decoded bytes**, including **467,471 unknown** and **452,980 known structure bytes**. The full **906-configuration** audit `resolved-alignment-audit-0.371.json` reports **883 complete / 23 explicit stops / zero context stops**, with **26,495 completed roots** across the audit. All **880** preceding positives preserve endpoints, root counts and unknown totals. Complete streams contain **78,967,784 decoded bytes**, including **28,028,015 unknown** and **50,939,769 known structure bytes**. The preceding native C-392 packet reproduces byte-for-byte; its original decoder hash is checked against `research/frozen-decoder-0.370/resolvedfeatures.py`.

Add `--alignment-dependencies` to the full Edition 0.370 audit/regrade commands to generate the new reports. `resolved_coverage_evidence.py --alignment-dependencies` independently verifies all **1,166 source rows**, hashes, decoded totals, stored spans and integer proportions. Added stored part credit is **159,593 bytes**. Cumulative resolved-family known bytes become **14,346,537**, leaving **10,790,929 unknown** of the unchanged 25,137,466 stored bytes. Total part unknown bytes become **54,021,095** of 227,086,352. Parts measure **76.2% / 71.1%**, assemblies remain **83.8% / 81.3%**, and drawings remain **57.3% / 49.9%**. The Edition 0.370 report and frozen gap report still verify. No whole-document or installed-project-format completion is claimed.

Six focused tests cover version boundaries, direct arrays, repeated map identities, extended counts, native maximum rejection, nonzero signed point flags, exact unknown accounting, every truncated alignment/order payload boundary, non-finite values, scoped dimension sets, opt-in text kernels, invalid kernel rejection, three public EOF streams, disabled-profile zero credit and appended-byte rejection. All **1,153 regression tests pass** (168.512 seconds); specification lint reports zero findings (575 headings, 96 anchors).

Evidence: C-393.
