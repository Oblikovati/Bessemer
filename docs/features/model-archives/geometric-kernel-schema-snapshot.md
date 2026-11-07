# Geometric and kernel schema reference

Edition 0.340 · 2026-10-26 · generated parts: Annex A (solidworks/swstreams.py), Annex G (solidworks/swcover.py)

### 10.2 File header

Definition: start of every transmit file.
*Status: Verified · 2,869/2,869 · Evidence: [C-9]*

| # | Field | Type | Presence | Description and rule | St. |
|---|---|---|---|---|---|
| 1 | `format` | bytes[2] or bytes[1] | | `PS`: neutral binary, all values big-endian, in 2,869/2,869 samples. `B`: bare binary, values little-endian, written by the kernel for the machine format [C-9]. | V |
| 2 | `title` | String32 | | ": TRANSMIT FILE (partition) created by modeller version <m>" or ": TRANSMIT FILE (deltas) created by modeller version <m>". The word in parentheses selects the file kind (10.7, 10.8). | V |
| 3 | `schema_key` | String32 | | `SCH_<modeller>_<writer schema>_<base schema>`, e.g. `SCH_3701250_37102_13006`. | V |
| 4 | `n_types` | u16 | | Node-type table size of the writer schema: the first number of the writer schema file's count line (239 for 37102). | V |
| 5 | `user_field_size` | u32 | | Size of user fields on each node. 0 in 2,869/2,869; a reader that meets another value cannot rely on this clause (OI-011). | C 2869/2869 |

String32 is a u32 byte count followed by that many bytes of text. A reader MUST read the format byte or bytes first and then all values in the byte order they select [R-08].

<a id="st-xt-schema"></a>
### 10.7 Partition file

*Status: Verified · 1,851/1,851 · Evidence: F.3*

After the header, a partition file is one record list: node records until the terminator (type 1 and an index). Nothing follows. The records hold the partition's bodies, their topology (shells, faces, loops, edges, vertices) and geometry, attributes and lists; their node types and fields are those of the base schema as edited by the file.

<a id="st-xt-deltas"></a>
### 11.16 Geometric reference wrappers

*Status: Verified scoped component layouts; inherited component references and meanings partial · Evidence: [C-30], F.18*

This clause specifies v > 0x27BC in the ordinary external-XML model context of 11.15, with the helper's lightweight-only flag (+0x834) clear. The latter flag adds configuration data in other contexts; its absence is a runtime condition, not a record tag. `modelreferences.py` requires the explicit context. It rejects embedded geometry bodies and sketch-handle context rather than guessing their extent.

#### 11.16.1 Common reference wrapper

*Status: Verified scoped layout; member meanings partial · Evidence: [C-30], F.18*

`moRefWrapper_c` stores the following in order:

| Order | Fields | Types | Condition | Status |
|---|---|---|---|---|
| 1 | component reference | object | Usually a moCompRef_c-derived object. Sketch entity handles require another sketch pointer; that context is not implemented. | V / ? |
| 2 | `unknown_60` | u16 | | ? |
| 3 | component | object | Declared moComponent_c. | V |
| 4 | `unknown_64`, `unknown_08` | i32 × 2 | | ? |
| 5 | `has_transform_10` | u16 | | V |
| 6 | `transform_10` | mgXform_c body | has_transform_10 != 0 | ? |
| 7 | `has_transform_18` | u16 | | V |
| 8 | `transform_18` | mgXform_c body | has_transform_18 != 0 | ? |
| 9 | `unknown_0c`, `unknown_20`, `unknown_6c`, `unknown_70` | i32 × 4 | | ? |
| 10 | `unknown_74` | i32 | v > 0x2FAF | ? |
| 11 | `unknown_4c`, `unknown_68`, `unknown_34`, `unknown_30` | i32 × 4 | | ? |
| 12 | `unknown_90` | i32 | v >= 0x2B73 | ? |
| 13 | `unknown_38` | i32 | v >= 0x3B02 | ? |

The transform layout is shared with 13.4. Stored i32 values are preserved even where the native loader later truncates a value to a word, merges flags or replaces a runtime value. The current wrapper has 62 bytes when both pointers are null and both transforms are absent. Actual records may contain larger pointed-to objects and transforms. The reader permits the verified component-reference classes listed in 11.17 at field 1; unverified derived/sketch contexts remain incomplete.

#### 11.16.2 Surface, face and curve suffixes

*Status: Verified scoped layouts; body-bearing cases and member meanings partial · Evidence: [C-30], F.18*

A surface-reference base appends four f64 members `unknown_a8`, `unknown_b0`, `unknown_b8`, `unknown_c0` to 11.16.1. A curve-reference base instead appends only `unknown_a8` and `unknown_b0`, both f64.

`moSingleFaceRef_w` uses the surface base, followed by `has_embedded_body` **i32**. When nonzero a kernel body and its attribute list follow; selected compressed-body framing is implemented in 11.156. After that optional body/list, the suffix is `unknown_c8` u32 and `unknown_d8` i32.

`moGeneralCurveRef_w` uses the curve base, followed by `has_embedded_body` **u8**. Its nonzero branch uses the selected common kernel-body framing in 11.156. After that optional body, the suffix is `unknown_c8` u32, `unknown_cc` i32, then three object records: `unknown_d8` (contour chooser), `unknown_d0` (component sketch-entity handle), `unknown_e0` (contour chooser). The differing face/curve presence-field widths SHALL be preserved.

`moCircleRef_w` appends to the general-curve body: `unknown_f8` point, `unknown_110` vector, `unknown_128` f64, `unknown_130` i32, then the optional frame below. `moCylinderRef_w` appends to the surface base: `unknown_c8` point, `unknown_e0` vector, `unknown_f8` f64, then the same optional frame. A point or vector is three consecutive f64; analytic roles and field units remain open.

The optional frame is a u16 `frame_mode`. If nonzero, three f64 `frame_vector` follow. If the word interpreted as **signed i16** is greater than 1, another three f64 `frame_point` follow. Thus 0xFFFF includes the vector but does not include the point. The current writer emits only mode 0 or 1; the loader also handles the point-bearing form. The schema reader preserves the original word and numeric values.

### 11.79 Embedded kernel-body archive framing

*Status: Native load/store evidence; common suppression byte verified; raw-stream variant conditionally traversed; kernel payload undecoded · Evidence: [C-94]*

The spline's nonzero u16 kernel-body marker invokes mo_loadBodyFromArchive. In the scoped modern versions this helper first reads a u8 suppression value. Any nonzero value ends the body helper without reading a geometry frame. Readers SHALL retain the original byte rather than normalize its stored representation. Zero opens a body whose framing depends on the model context and underlying stream class. Unknown context SHALL produce an explicit stop after the suppression byte unless the exact self-describing compressed Parasolid form validates as specified in 11.99.

For the ordinary raw-stream branch, v > 0xC9B stores an i32 length followed by a u8 kernel type and length - 1 payload bytes. The length includes the kernel byte and excludes its own four bytes. Native writing reserves the length, writes kernel/body data, then patches the length from the stream-position difference. A length of one is a no-geometry case in native loading. Readers SHALL bound the complete frame against available bytes. The explicit raw-only framing reader retains the payload as opaque bytes and SHALL count those bytes as undecoded, regardless of later archive alignment. Typed decoding of the self-describing compressed form is specified separately in 11.99.

This raw-stream branch is not universal. Native code omits its length when the underlying object is suCompressedStream_c; model-context virtual dispatch can also handle body loading and storing. Stream-class and context selection are runtime facts, not flags inferred from a plausible length. Outside the positively validated form in 11.99, the default composed reader leaves the variant unspecified. An explicit ordinary_raw audit is available separately and does not establish that variant for all documents.

In vessel weldment.sldprt, Contents/Config-0, the common suppression byte is zero and the following position is 0x713F. Selecting the raw variant consumes a declared 772-byte frame and advances parsing to sgDimEntityHelpData_c at 0x8A80. This sequential result supports the selected framing for that fixture but does not decode its geometry or prove arbitrary context behavior. Default successful-prefix totals remain 892, with 77 sketches in 24 configurations; all 14 remaining configurations stop explicitly. Whole-document bit coverage is unchanged.

### 11.99 Validated embedded compressed Parasolid bodies

*Status: Four native embedded bodies fully decoded through the existing Parasolid schema; 903 manager prefixes complete; other kernel forms remain context-dependent · Evidence: [C-114]*

The model-context pointer at archive-helper offset 38 is a SldModelContext_c pointer, established by the helper-object constructor. The wrapper's +4E0 method is isModelWithCompressedStream; +90 tests drawing-document context, and +548 loads a Parasolid body. The document-context implementation delegates compression selection to a document virtual flag. These runtime decisions are not inferred from an arbitrary integer in the archive. The scoped reader instead recognizes the following exact self-describing form, validating its entire nested structure before accepting it.

After u8 suppression=0, the supported form is i32 frame_length, u8 kernel_type=0, then one size-prefixed compressed chunk from 9.3. The frame length includes the kernel byte and excludes its own four bytes. The chunk's u32 length equals its remaining byte count, its GUID is the documented 16-byte compression GUID, and its block sequence ends exactly at the declared frame boundary. Every block must inflate to its declared size (1 through 0x100000 bytes); its complete zlib stream must end with no unused or unconsumed bytes. A zero/zero block pair terminates the chunk. Readers SHALL validate all these constraints, not just a plausible frame length.

Concatenated inflated blocks must form a neutral PS Parasolid transmit file from clause 10, using an installed matching schema. The parser must consume the entire transmit file without a tail, identify a partition rather than deltas, and contain exactly one BODY record. Unknown schemas, malformed block boundaries, invalid sizes, trailing compressed data or unsupported transmit structures SHALL stop the reader. The archive cursor advances over the frame only after validation succeeds. Other kernel types, uncompressed payload forms and stream/context variants remain outside this recognition rule.

The decoded result retains the original payload, compressed block sizes, inflated bytes, Parasolid layouts and node values. Unknown-byte accounting uses the same proportional rule as compressed partition streams: compressed data inherits any unknown inflated-field fraction, while envelope/block framing is known. All four observed embedded bodies have zero unknown inflated bytes, so none of their compressed payload bytes remain opaque under this typed decoding. This is format decoding, not a claim that a native geometry kernel has reconstructed or validated the shape.

The profile geometry writer at 0x4BB404F0 writes its extra-body map when the body-store helper returns success. On the supported ordinary raw path, helper 0x4C261650 returns success after completing and patching a frame; write failures throw. Thus a validated serialized body establishes the file-layout continuation independently of recreating the native load-time body pointer. After the existing pair map, direct references and dynamic reference array, the profile reader stores u32 extra-body count and count records of i32 key/kernel-body archive record. Readers SHALL retain duplicate keys and parse each nested body using its own suppression/framing rules. The no-body suppressed path still omits this block.

The four observed frames contain the following fully consumed transmit files, all schema SCH_3701250_37102_13006:

| Source | Frame length including kernel byte | Compressed chunk bytes | Inflated bytes | Parasolid nodes |
|---|---:|---:|---:|---:|
| vessel weldment, Config-0 | 772 | 771 | 1257 | 22 |
| Sustainability base plate, Config-0 | 1552 | 1551 | 2898 | 66 |
| advdrawings base plate, Config-0 | 1551 | 1550 | 2898 | 66 |
| assemblyvisualize base plate, Config-0 | 1551 | 1550 | 2898 | 66 |

Caller traces establish that the vessel frame occurs in a sketch spline, and the three base-plate frames occur in profile geometry. They are not fillet-cache records. The four frames cover 5422 compressed chunk bytes, 9951 inflated bytes and 220 decoded nodes. Exact offsets and payload/inflated hashes are retained in the evidence inventories.

The default supported-feature audit now completes 903/906 relation-manager prefixes. The three remaining paths advance through profile geometry to moRefPlane_c dependencies at 0x11B08/0x11AB1/0x11B29. Seven fillets and the previously documented supported feature objects still complete. A separate `--raw-kernel` hypothesis audit remains available and writes a distinct report; it keeps payloads opaque and still refuses the three profile continuations without the typed serialized-body evidence. Neither audit establishes complete configuration streams or full document coverage. The aggregate whole-document coverage metric is unchanged because these feature readers remain audit-scoped.

