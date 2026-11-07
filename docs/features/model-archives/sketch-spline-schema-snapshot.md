# Sketch spline schema reference

Edition 0.351 · 2026-10-26 · generated parts: Annex A (solidworks/swstreams.py), Annex G (solidworks/swcover.py)

### 11.35 Scoped ordinary spline collection

*Status: Two installed ordinary spline bodies verified; interpolation/subtype/kernel branches remain partial · Evidence: [C-50], F.38*

The conic collection is followed by an i32 spline count and direct entries with u16 subtype masks. The implemented direct collection branch accepts only mask bits 1, 4, 8 and 16, with the expanded native prototype/callback evidence in 11.210. Other mask bits can select additional prototype or serializer behavior outside this reader. Readers SHALL reject unsupported subtype masks rather than apply the common body speculatively. Direct entries do not allocate MFC object-map slots.

The body below applies to the modern ordinary nonminimal-owner context, with no interpolation arrays or secondary interpolation state. It starts with the common segment through NormalScale, then:

| Order | Fields | Encoding |
|---|---|---|
| 1 | flags_32c; unknown_2fc; point list | u32; u16; object pointer |
| 2 | unknown_170; unknown_f0; unknown_f4 | u16; u16; i32 |
| 3 | unknown_f8; unknown_100; unknown_180; unknown_154 | three f64; i32 |
| 4 | vector_118; vector_130; unknown_2f8 | two three-f64 vectors; u16 |
| 5 | parameters_148 | i32 count and that many f64 values |
| 6 | control_points_300 (only if flags_32c bit 1 is clear) | i32 count and that many three-f64 points |
| 7 | tangent map | u32 count; each: i32 index, u16 mode, three-f64 vector, and extra f64 when mode equals 2 |
| 8 | interpolation_count | u32; 0xFFFFFFFF denotes absent interpolation data in this scope |
| 9 | unknown_314; unknown_318; unknown_31c; unknown_174 | i32 (0 in scope); u16; u16; i32 |
| 10 | equation block (only if flags_32c bit 4 is set) | six CStrings; i32 equation_word; i32 unknown_1e0; direct mgXform_c |
| 11 | reserved_word | i32, retained even though current save code writes zero |

The point-list classes sgSplineList_c, modifSplineList_c and cptsSplineList_c share the suObList serializer: packed ReadCount and object pointers. The native loader can transfer list items into its internal spline list without consuming more bytes. A null list is accepted only for an entity with flags_54 bit 1 set. A runtime list discriminator at virtual slot +0x88 controls the later tolerance branch; this is not an emptiness test. sgSplineList_c and modifSplineList_c return nonzero and consume no tolerance word for either flag value. cptsSplineList_c returns zero: flags_32c bit 1 clear reads a u16 tolerance-presence word and optional f64 tolerance, while bit 1 set invokes the direct curve helper, including its header/values and modern tolerance suffix (corrected in 11.211). A null list has neither branch. This corrects the earlier tolerance condition; see 11.78.

For the control-point-list curve branch, the native curve helper begins with four u16 words named by byte offsets: header_0, header_2, header_4 and header_6. Their semantic mapping remains unresolved. Header_0 is 0, 3 or 4; when zero, all other header words are zero and no values follow. Otherwise the f64 value count is `(header_0 + 1) * header_4 + (header_6 != 0 ? 1 : header_2)`. A u16 tolerance-presence word and optional f64 tolerance follow. The stored array is preserved; geometric reconstruction and further knot constraints remain partial.

The spline ends with f64 unknown_188, a u32 fixed-length map count (each entry: i32 index and sgFixedLengthSplineControlPointData pointer), u16 unknown_340, u16 kernel-body presence and final i32 unknown_190. Nonzero kernel presence introduces an embedded body before the final word and is explicitly unsupported. Populated primary interpolation arrays are specified in 11.78. Nonzero unknown_314 and unsupported referenced objects SHALL cause explicit stops. Offset-named fields, equation roles, flags and map semantics remain partial; their raw values are retained.

### 11.78 Control-point spline list and primary interpolation arrays

*Status: Native dispatch/serializer evidence; installed case traverses interpolation arrays and stops at embedded kernel body · Evidence: [C-93]*

cptsSplineList_c dispatches to the imported suObList::Serialize implementation. Its body uses packed ReadCount followed by tagged object pointers, with no additional list-class suffix. Its virtual +0x88 discriminator returns zero, unlike sgSplineList_c and modifSplineList_c. The corrected tolerance condition in 11.35 follows the class discriminator and flags_32c bit 1, not list length. Readers SHALL use that condition and SHALL NOT consume a tolerance word for either nonzero-discriminator class.

The u32 interpolation_count in the spline body is 0xFFFFFFFF when primary interpolation data is absent. Otherwise it is n, followed by i32 width, two f64 values at interpolation-state +0x80/+0x88, n f64 parameters, n u16 words, n i32 indices and n * width f64 values, in that order. These are parallel arrays, not interleaved records. Readers SHALL preserve stored -1 indices; native reconstruction may replace them at runtime, which does not change the wire representation. The scoped reader requires nonnegative width and bounds every array against remaining bytes before allocation. Zero count still stores width and the two doubles.

The installed control-point spline case now traverses these fields and the corrected tolerance branch, then stops explicitly at its nonzero embedded-kernel-body marker. This is deeper verified parsing, not a complete additional sketch or manager. Successful manager-prefix totals remain 892, with 77 sketches across 24 configurations; all 14 incomplete configurations retain explicit stops. Secondary interpolation and embedded kernel framing remain unfinished.

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

### 11.210 Direct spline collection masks and shared load helper

*Status: B / ? — native collection selection/callbacks, synthetic mask combinations and 12 mask-0x10 splines in complete roots verified; mask-0x4 containing roots and other subtype branches remain partial.*

The sgSketch load function `0x4C59BDC0` restores its +40 spline collection with an i32 count and u16 category masks. The existing common spline helper is `0x4C5A55A0`. Native load branches establish the following restricted extension of 11.35:

| # | Field | Type | Presence | Description and rule | St. |
|---|---|---|---|---|---|
| 1 | stored category mask | u16 | each direct spline | Retain every accepted bit. Supported mask domain uses only bits `0x01`, `0x04`, `0x08`, `0x10`; unknown bits still cause an explicit stop. | B / ? |
| 2 | external selection | mask bit 0x01 | bit set | Takes precedence over underlying-offset selection and invokes the existing common helper directly, as already supported. | B |
| 3 | underlying-offset selection | mask bit 0x04 | external bit clear, bit set | Constructs an sgOldUnderlyingOffsetSpline prototype and explicitly calls `0x4C5A55A0` on it. Runtime conversion to a new prototype is separate from archive consumption. | B |
| 4 | constructor/category bit | mask bit 0x08 | retained if present | Existing constructor-state selection; no extra archive field is introduced by this accepted bit. | B / ? |
| 5 | extracted category bit | mask bit 0x10 | retained if present | Native load code extracts the value, but the retained complete load function does not use it to select an additional archive read. Its application meaning remains unassigned. | B / ? |
| 6 | spline body | existing direct common spline body | each accepted entry | Existing list, interpolation, equation, tolerance, fixed-length map and kernel conditions remain enforced. No MFC map slot for the direct spline itself. | B |

This rule is scoped to ordinary modern direct sketch collections, version greater than `0x27CB`. It does not register an arbitrary offset-spline object serializer. In particular, primary tables for sgOldUnderlyingOffsetSpline (`0x4CF85938`) and sgNewUnderlyingOffsetSpline (`0x4CF85C70`) select virtual serializer `0x4C597F40`; the collection instead explicitly invokes common helper `0x4C5A55A0` after constructing its temporary prototype. Substituting a virtual serializer based only on the class name would change the dispatch being documented.

Common-helper secondary interpolation depends on nonzero raw +314 and either a populated control-point collection or a true virtual +1F0 callback. Native primary bindings for sgSpline and both underlying-offset prototypes select exact false-return callback `0x4B29C5A0`, bytes `33 C0 C3`. Therefore the existing false-callback secondary-interpolation condition remains valid for the newly accepted underlying-offset path. Other prototype bits, including surface-associated or specialized states, are not admitted from these observations.

`spline_subtype_evidence.py` binds all three prototype primary COL/type records and callback slots, retains the complete installed module hash and native function ranges, verifies four direct common-helper calls in the sketch loader, and retains the relevant load-branch excerpt and decompilation hashes. Native source demonstrates external/offset branch precedence and the absence of a further use of the extracted `0x10` value in this load function.

The synthetic common-spline tests now wrap populated direct spline bodies in masks `0x4`, `0x10` and combined `0x1D`, retaining parameters, equations, list-discriminator tolerance branches and final-word alignment. Existing rejection of unknown bit `0x80` remains tested. A source-hash-bound native test verifies complete containing-root spans and stored mask/flags/parameters/control-point counts/fixed-length counts/final word for the completed new category records.

The audit retains **12 mask-0x10 splines** in complete containing roots. Mask-0x4 examples advance beyond category selection but still encounter later unsupported dependencies; no complete mask-0x4 containing-root proof or corresponding complete-root credit is assigned. Native prototype/helper evidence and synthetic framing cover that path at their stated scope.

Across 906 complete modern contexts the frontier now has **24,113 complete roots**, **754 complete lists** and **152 explicit stops**: 12 additional complete roots and one additional complete list relative to Edition 0.215. **684 lists reach exact EOF**, spanning **44,828,706 decoded bytes**; **70 complete lists retain four-byte unknown tails**. One additional list reaches EOF, adding 26,200 exact-EOF bytes. Completed-list endpoint/tail invariants pass. `resolved-feature-list-frontier-0.215.json` preserves the preceding checkpoint. Remaining fixed-length spline failures and object dependencies are explicit stops; whole part/assembly/drawing and all-installed-project bit coverage remains unfinished.

Evidence: C-238.

### 11.211 Control-point spline direct curve branch correction

*Status: B / ? — native caller/helper framing and 18 populated curve bodies in complete containing roots verified; curve-header engineering meanings and full document/project coverage remain partial.*

The fixed-length spline map-count failures were caused by an earlier missing curve body, not by excessive map counts. In common spline helper `0x4C5A55A0`, a nonnull point list whose virtual +88 discriminator returns zero enters the following branch when flags_32C bit 1 is set: it calls curve helper `0x4C5AA410` before raw +188 and the fixed-length map. cptsSplineList_c has that zero-return discriminator. The previous statement that this branch performed only runtime reconstruction was incorrect and is superseded here.

For ordinary non-DB external-XML archives with version greater than `0x27CB`, this direct curve body is:

| # | Field | Type | Presence | Description and rule | St. |
|---|---|---|---|---|---|
| 1 | header_0 | u16 | always | Accepted native values are 0, 3 or 4. Engineering meaning remains unresolved. | B / ? |
| 2 | header_2/header_4/header_6 | three u16 | always | Retain each word independently. With header_0 zero, native requires all three to be zero. | B / ? |
| 3 | values | f64 array | header_0 nonzero | Count is `(header_0 + 1) * header_4 + (header_6 != 0 ? 1 : header_2)`. Bound against remaining bytes before reading. Zero header_0 has no values. | B / ? |
| 4 | tolerance presence | u16 | always in modern scope | This word follows the values even for a zero header. Preserve raw nonzero values. | B |
| 5 | tolerance | f64 | tolerance presence nonzero | Native passes the restored double to its tolerance update helper; offline retains the stored value. | B / ? |

The minimal curve body is ten bytes: four zero header words and zero tolerance presence. Populated values add eight bytes each; a present tolerance adds eight bytes. Native also has an additional historical word for unsigned `v - 0x4FD <= 0x274`. That obsolete word is separate from rows 4–5 and absent in every supported version greater than `0x27CB`.

After this body the common spline resumes at raw +188 f64, then the u32 fixed-length map count and its existing records, followed by the remaining spline fields. This direct curve branch is distinct from the flags-bit-1-clear tolerance-only branch: with zero list discriminator and bit 1 clear, native reads just its tolerance presence/optional double at that point. Nonzero-discriminator lists and null lists do not gain this curve body.

`sketchprefix.spline` now invokes `spline_curve` under that exact list/flag condition. Existing populated common-spline fixtures were corrected to include the missing ten-byte empty curve body. Tests cover that composition across accepted masks/list families, populated periodic/nonperiodic header formulas, raw tolerance presence, zero-header curves, every truncated empty-curve boundary, and source-hash-bound complete native containing-root spans with curve/flag/fixed-length-count/final-word equality.

`spline_subtype_evidence.py` now retains the common-spline-to-curve call, a hash-bound curve-helper decompilation and explicit native code window covering its fragmented load path. The window is not a function-length or wire-size claim. Native helper code confirms unconditional modern tolerance framing before the separate historical-word gate.

The audit retains **18 complete control-point curve bodies**. Header_0 is 3 or 4, header_2 is 4, header_4 ranges from 4 through 73, and all observed header_6 words are zero. Value counts range from 24 through 296. Both zero and nonzero tolerance presence occur. All 18 later fixed-length map counts are zero; this does not restrict valid maps in other files. All six previously observed invalid-map-count failures disappear after the corrected earlier framing; the remaining affected configuration reaches an unsupported backed-up segment object rather than receiving an opaque skip.

Across 906 complete modern contexts the frontier now has **24,148 complete roots**, **759 complete lists** and **147 explicit stops**: 35 additional roots and five additional complete lists relative to Edition 0.216. **689 lists reach exact EOF**, spanning **45,507,362 decoded bytes**; **70 complete lists retain four-byte unknown tails**. Five additional lists reach EOF, adding 678,656 exact-EOF bytes. Completed-list endpoint/tail invariants pass. `resolved-feature-list-frontier-0.216.json` preserves the preceding checkpoint. Whole part/assembly/drawing and all-installed-project bit coverage remains unfinished.

Evidence: C-239.

