### 14.7 Composer geometry gzip wrapper and information prefix

*Status: Native read/write/string helpers and 38 complete information prefixes; scene graph remains opaque · Evidence: [C-146]*

`CLitSMGLoader::GetInfos` (`litinteropsmg.dll:180001740`) opens `.smgGeom`/`.geom` through `gzdopen`; its writer (`180007D80`) uses gzip with compression level 9. Every installed `product.smgGeom` member is one gzip stream with the observed ten-byte header `1F 8B 08 00 00 00 00 00 02 0A`, a raw-deflate payload and eight-byte little-endian CRC-32/ISIZE trailer. `composergeometry.py` verifies gzip member length/CRC and exact deflate termination. Optional gzip header flags and concatenated gzip members remain unsupported by this reader.

The decoded stream begins with the information prefix, then scene-graph data. The native methods `ReadInfos` (`180002C20`), `WriteInfos` (`1800087F0`) and `get_string` (`1800018B0`) establish its framing. In version **6.7 and later**, strings are UTF-16LE code units terminated by either `0000` or `000A`; the current writer emits `000A`. Preserve the original terminator and code-unit count, including unpaired surrogates. Earlier multibyte/code-page string variants are not implemented. All 38 installed geometry streams report f64 version **7.9**.

| Order | Prefix field | Wire representation |
|---|---|---|
| 1 | Geometry version | f64. |
| 2 | Source-name string | Terminated UTF-16LE code units in the supported version context. |
| 3 | Raw information word (`LitGeomInfos +20`) | u32. |
| 4 | Information logical size | u32; the logical counter excludes the two terminators of the following two strings. |
| 5 | Information fields | Size-selected sequence below. |
| 6 | Scene graph | Remaining decoded bytes; not parsed by this prefix reader. |

The information sequence stores six eight-byte words at native destination offsets `3C,34,54,44,4C,5C`, then u8 at `30`, u8 at `32`, u32 at `64`, a terminated string at `68`, a second terminated string at `88`, u32 at `A8`, an eight-byte word at `AC`, and u8 at `31`. These offset names identify native storage; the prefix reader retains the eight-byte words losslessly as u64. Clause 14.89 adds named native API roles and exact two-u32 projections, while their value encodings remain unresolved. Each field is present only while the accumulated logical count is below the declared size. Scalar widths must fit the remaining logical span. String contents contribute their serialized code-unit bytes, while their two-byte terminators do not contribute. Remaining logical bytes after known fields are retained as opaque future information.

This logical/physical distinction is observable in all samples: an information logical size of **67 bytes** occupies **71 physical bytes**, because both trailing strings' terminators are physically present. Treating 67 as a physical block length would misplace the scene-graph start by four bytes. A shorter size can omit later fields; the reader does not add native runtime defaults as wire bytes. Unterminated strings, partial scalar fields, impossible future spans, unsupported versions and damaged gzip data are errors.

Across 38 samples, gzip decoding yields **45,280,926 geometry bytes**. Complete information prefixes span **7,990 bytes**, with **7,534 retaining unresolved meanings**; the rest is scene-graph data. Raw strings, scalar words and future information retain unknown accounting. Version and logical-size framing are distinguished from those unresolved values. Native/read-write evidence plus synthetic fixtures check code-unit counting, both terminators, shorter information records, future bytes and truncated/corrupt inputs. Independent `gzip` decoding also verifies every native decoded stream.

The project inventory now includes these 38 Composer files and the standalone Inspection tutorial `.ixprj`, in addition to the five earlier Simulation/Flow carriers: **44 carrier fixtures**, including two compound files. The standalone Inspection file has a different binary header and remains opaque at this checkpoint; its later envelope profile is specified in 14.15. No SMG/container inference is applied to it. Clause 14.87 continues the Composer geometry prefix through first-geode attributes, and 14.88 frames all installed version-7.9 geometry streams to EOF. Complete value meanings, XML/project relationships and full-document bit coverage remain incomplete.

### 14.87 Composer geode headers: matrix, names and sized user data

*Status: Native read/write grammar and 38 installed first-geode prefixes verified; geometry and later scene records remain opaque · Evidence: C-463*

This continues the geometry information prefix in 14.7, within the already decoded gzip stream. The hash-pinned **litinteropsmg.dll** (**f3a66e5a25843eb019a4bc63c4fe3df1fb5121a6f9d082791ee337f2637520d3**) exports the reader/writer methods used here. The scoped decoder implements **geometry version 7.9**. Earlier string/version branches, other top-level record kinds and later geometry bodies are not silently interpreted with this profile.

Top-level loader **0x180001E70** calls `ReadInfos`, then reads one-byte tags until EOF or cancellation. Tag **`u` (0x75)** calls **`ReadGeode` at 0x180002870**; tag **`v` (0x76)** calls a face reader. All 38 installed streams start their scene data with `u`. This clause parses the first geode's attributes and first LOD geometry count, stopping before its first geometry record. It does not seek to later geodes by scanning for tag-like bytes.

| Ordered geode-header field | Wire representation and rule |
|---|---|
| Geode tag | One byte `u` |
| Matrix presence P | One byte; any nonzero value selects the matrix |
| Matrix, if P≠0 | 64 bytes: sixteen four-byte patterns, native `sglMat4<float>` storage order |
| Geode name | Terminated UTF-16LE string, as in 14.7 |
| User-data size S | u32 byte count of the following user-data body |
| User-data body | Exactly S bytes; size-selected sequence below |
| First LOD geometry count | u32 only when the effective LOD count is nonzero |
| Geometry/later LOD data | Remains opaque to this prefix reader |

`ReadMatrix` **0x18000CC80** reads the presence byte and exactly 0x40 bytes when nonzero. Writer **0x18000CAA0** compares its computed matrix with identity and emits a presence byte plus the same 64-byte representation. The decoder preserves presence and every matrix word, without applying finite-value, affine, row/column interpretation or unit constraints. An absent matrix stays absent in the wire projection rather than being replaced with fabricated serialized identity values.

`WriteGeodeAttributes` **0x180008C60** emits `u`, the matrix, the name and user data. It replaces newline characters in the name with underscores before writing, then emits the UTF-16 newline terminator. The decoder preserves the stored name and terminator; it does not reverse that lossy normalization. Native `sglObject::setName` establishes the string's node-name role without making it a filesystem path.

**Sized user data.** Reader **0x18000D340** and writer **0x18000CD10** establish the following order. Before each known field, the reader tests whether its accumulated physical count is below S. S=0 has no stored fields. The strict decoder permits complete shorter sequences and rejects a size ending within a scalar or unterminated string. Bytes after all known fields remain preserved future data.

| Order | Field | Wire representation | Native binding |
|---|---|---|---|
| 1 | GUID text | Terminated UTF-16LE | `GetGuid` / `SetGuid`; syntax is not constrained to a UUID |
| 2 | Raw member +0xD0 | Four-byte pattern | Layer ID; signed native view in 14.90 |
| 3 | Assembly name | Terminated UTF-16LE | `GetAssemblyName` / `SetAssemblyName` |
| 4 | Raw member +0xD4 | Four-byte pattern | Volume; binary32 view in 14.90, units open |
| 5 | Raw member +0xF8 | Four-byte pattern | Area; binary32 view in 14.90, units open |
| 6 | Center of gravity | Three four-byte patterns | `GetCofG` / `SetCofG`, native float-vector API; units open |
| 7 | Inertia axes | Nine four-byte patterns | `SetInertiaAxes`, native three-vector storage; orientation/units constraints not imposed |
| 8 | Raw member +0x12C | One byte | Geometry type; copied into a runtime u32 without boolean normalization; 14.90 |
| 9 | Issued-from text | Terminated UTF-16LE | `GetIssuedFrom` / `SetIssuedFrom`; interpretation beyond that name open |
| 10 | LOD count | u32 | Returned to `ReadGeode`, also assigned to user-data member +0x138 |
| 11 | Future data | Remaining declared bytes | Native reader discards; decoder preserves |

**User-data size counts all string terminators.** For a full current writer body, **S = 71 + 2×(GUID code units + assembly-name code units + issued-from code units)**. The 71 includes six bytes for the three UTF-16 terminators. This differs from the information-block logical size in 14.7, which excludes its two terminators. Readers must not reuse that earlier logical-size counter here. Both NUL and newline string terminators are retained, including raw code units and unpaired surrogates.

`ReadGeode` initializes its LOD count to **1** before reading user data. If the LOD field is absent, that is the effective caller count; no missing field bytes are added. A stored count of zero suppresses the following LOD loop. With a positive count, each LOD begins with its own u32 geometry count; geometry entries then start with one-byte tags. Writer **0x180008AB0** counts nonempty LOD children and writes their geometry lists, corroborating this distinction. The prefix decoder reads only the first geometry count and leaves the remaining LOD structure opaque.

Across the **38 installed streams**, the new geode prefixes occupy **6,618 decoded bytes**, including **4,268 user-data wire bytes**. **16** first geodes have matrices. User-data bodies have size **103 in 36 samples**, **275 in one**, and **133 in one**; none has future bytes. All first geodes have one LOD. Their first geometry counts are **1 in 33 samples, 2 in two, 3 in two, and 4 in one** (sum 47). Every new endpoint is immediately followed by `v`, matching the native geometry dispatch; those geometry bodies are not validated by this check. Combined with the earlier 7,990 information-prefix bytes, the bounded prefixes span **14,608** of **45,280,926** decoded geometry bytes; **45,266,318 bytes remain after those endpoints**. This is framing progress, not semantic or proportional compressed-byte credit.

`solidworks/composerattributes.py` provides a strict standalone `read_userdata` and a suffix-preserving `read_geode_prefix`, both requiring explicit version context. It reports complete component framing for bounded user data and **`complete_parent_record=False`** for the geode prefix. `composer_attributes_evidence.py` pins nine native ranges, version thresholds and named user-data imports, and records all source/member/prefix/suffix hashes and projections in `composer-attributes-corpus.json`. Five focused tests cover full/short/empty/future user data, terminator counting, raw numeric patterns, surrogate preservation, noncanonical matrix presence, LOD zero/default behavior, every truncation and strict version/size guards. Matrix and scalar value meanings, geometry records, later scene graph/XML relationships and all-installed-project coverage remain incomplete. Global coverage percentages are unchanged.

All **1,463 regression tests pass** (213.858 seconds). Specification lint reports zero findings (645 headings, 96 anchors).

Evidence: C-463.

### 14.88 Composer complete geometry-stream framing and packed drawable arrays

*Status: Native-derived version-7.9 framing reaches EOF in all 38 installed geometry streams; remaining value meanings and project relationships stay open · Evidence: C-464*

`solidworks/composerscene.py` continues from 14.7 and 14.87 through the complete decoded geometry stream. The installed **litinteropsmg.dll** hash remains **f3a66e5a25843eb019a4bc63c4fe3df1fb5121a6f9d082791ee337f2637520d3**. Twenty complete read/write/helper ranges and three exact leaves establish the profile. Scope is **version 7.9**, canonical constant-axis flags and explicit texture-import context; older naming/version branches are unsupported. Full framing does not complete the XML/project schema or establish every stored scalar's meaning.

After the information block, top-level tags are `u` geodes and `v` drawable records. A `u` header is as in 14.87, followed by its effective number of LOD lists. Each list starts with a u32 geometry count and that many `v` records. A top-level `v` uses the same version-7.9 drawable wire fields. The strict decoder reads declared counts and tags in sequence to exact EOF; it does not search for apparent tags inside arrays or discard an unexplained suffix. Unknown tags fail this profile. All counts and scalar words below are little-endian, with four-byte counts unless stated otherwise.

#### 14.88.1 Common drawable attributes and sized texture columns

*Status: Native read/write fields and complete installed material bodies verified; remaining enum/flag meanings open · Evidence: C-464*

Each `v` record starts with two terminated UTF-16LE strings, then **diffuse, specular and ambient triplets** (three f32 patterns each), a four-byte material member **+0xEC** pattern, an f32 alpha pattern, a u32 texture-body size S and exactly S body bytes. A one-byte drawable kind follows that body. The two names are retained separately; the writer supplies its parent geode's name for the first and a separate text argument for the second. Clause 14.96 identifies +0xEC as front shininess and the examined writer's second argument as the literal `-1`; it also adds named texture-column roles and native conversions. Broader identity/reference mapping remains open.

Writer **0x1800082D0** binds the three triplets through `getFrontDiffuse`, `getFrontSpecular` and `getFrontAmbient`, and alpha through `getFrontAlpha`. Reader material helper **0x180001C20** applies diffuse, specular and alpha. It supplies fixed shininess **30.0**, rather than using its stored +0xEC argument, and does not apply the ambient argument in this helper. Stored values therefore remain distinct from reconstructed runtime defaults. No color-range, finiteness or unit constraints are added by the wire decoder.

For S>0, the texture body begins with a u32 layer count L. For L≤4 the known fields are **column groups**, not interleaved per-layer records:

| Group, in wire order | Per-layer storage |
|---|---|
| L texture texts | Terminated UTF-16LE |
| L matrices | 64 bytes each, sixteen f32 patterns; `ReadMat` at 0x18000CC30 |
| L mapping values | One byte each |
| L first flags | One byte each |
| L environment values | Four bytes each |
| L wrap values | One byte each |
| L final flags | One byte each |

Before **each individual field** the native reader checks whether the consumed count is below S. Complete shorter bodies can omit later texts, matrices or scalar cells. The decoder returns `None` for those absent wire cells and does not add runtime default bytes. A partial scalar, matrix or unterminated string is rejected by the bounded profile. Remaining bytes after known columns are retained. For L>4 the body after its count remains opaque, matching the native sized-skip path without inventing a new column grammar.

The full current writer formula is **S = 4 + sum(2×text_code_units + 2) + 72L**. The 72 bytes per layer comprise 64 matrix bytes plus 1+1+4+1+1 scalar bytes. Terminators count toward S. The native types identify mapping, environment and wrap categories, but full enum domains and the two flag meanings remain unresolved. The raw bytes/words are preserved. S=0 has no layer-count word.

#### 14.88.2 Packed vector and index blocks

*Status: Native widths, component order and interpolation established; eight-bit vectors are native/synthetic only · Evidence: C-464*

Let **V3(N)** denote N three-component vectors. Its wire order is: one-byte mode; **three one-byte constant-axis flags** in X/Y/Z order; one f32 pattern for each constant axis; for modes `b`/`c`, two f32 endpoint patterns for each varying axis; then the packed values. Constant values precede all endpoint pairs. Packed values are **per vector**, in X/Y/Z order with constant axes omitted. With A varying axes, there are exactly **N×A** stored scalar values. Even N=0 still consumes the mode, flags, constants and any endpoint fields.

| Mode byte | Packed scalar width | Native interpretation |
|---|---|---|
| `a` (0x61) | 4 | Direct f32 patterns; no endpoint pairs |
| `b` (0x62) | 2 | Unsigned u16 codes with endpoint interpolation |
| `c` (0x63) | 1 | Unsigned u8 codes with endpoint interpolation |

**V2(N)** has a mode byte and N two-component vectors, with **no constant-axis flags or constant-value fields**. Modes `b`/`c` store four endpoint patterns (two per component) before their 2N packed codes. Mode `a` stores 2N direct f32 patterns. Array readers **0x18000B5F0 / 0x18000B2C0** and reconstruction helpers **0x18000BEE0 / 0x18000C430 / 0x18000C7D0** establish these counts and component order.

Exact leaves **0x18000BC80** and **0x18000BCB0** convert u8/u16 code q using **t=q/Q**, then **(1−t)×A+t×B**, with Q=**255** or **65535** and the stored endpoint pair A/B. Native scalar single-precision instructions perform the operations; endpoint ordering, finiteness, normal-vector length and floating-environment behavior are not constrained here. The decoder retains every endpoint pattern and packed code; it does not emulate runtime floating reconstruction or replace original codes with rounded Python floats. The scoped constant-axis flags are 0/1; noncanonical bytes are rejected rather than assigned unverified runtime behavior.

Let **Ix(N,D)** denote N unsigned index/length values, where D is a caller-supplied domain parameter. **No count, width tag or D is stored inside this block.** Reader **0x180005180** uses:

| D | Stored width per value |
|---|---|
| 0..255 | 1 byte |
| 256..65535 | 2 bytes |
| 65536..0xFFFFFFFF | 4 bytes |

The threshold uses D itself, not D−1 or the largest observed code. Values expand without delta coding. The decoder preserves values outside D; it does not attribute an additional rejection rule to the native helper. For coordinate/normal lists D is their stored count; for strip-length lists it is the separate stored length-domain word.

#### 14.88.3 Drawable kinds and conditional arrays

*Status: Six native-derived drawable layouts; installed types a–e and synthetic point sets · Evidence: C-464*

The kind byte uses a different context from the array mode byte. Counts below are explicit stored u32 words unless an expression is given. B is a stored normal-binding word; nonzero selects the indicated normal data. The common attributes precede every row's body.

| Kind / native reader | Ordered body fields |
|---|---|
| `a`, triangle set — 0x180005400 | T; V3(3T); B; if B≠0, V3(3T) normals; one trailing u32 word |
| `b`, indexed triangle set — 0x1800058D0 | T,V,I; V3(V); Ix(I,V); B; if B≠0, N,V3(N),Ix(I,N) for normals; texture-layer count and texture data below |
| `c`, indexed triangle-strip set — 0x180006200 | K,V,I,M; V3(V); Ix(I,V); B; if B≠0, N,V3(N),Ix(I,N) for normals; Ix(K,M) strip lengths; texture-layer count and gated texture data below |
| `d`, mono-indexed triangle-strip set — 0x180006C20 | K,V,I,M; V3(V); B; if B≠0, V3(V) normals; Ix(I,V); Ix(K,M) strip lengths; texture-layer count and texture data below |
| `e`, line-strip set — 0x180007480 | f32 line-width pattern; K,V,M; V3(V); Ix(K,M) strip lengths |
| `f`, point set — 0x1800079C0 | f32 point-size pattern; V; V3(V) |

T is the stored triangle count, K the strip count, V the coordinate count, I the index count and M the length-domain parameter. The reader retains these fields independently; it does not silently substitute 3T for an explicit I. `d` shares coordinate indexing for its normal/texture lists and has no separate normal-count/index words. `e` has sequential strip coordinates, not an added coordinate-index array. The trailing word in `a` is preserved; its reader does not use it to select additional arrays. Zero bindings omit stored normal arrays; generated runtime normals are not wire fields.

For `b`, positive texture-layer count L is followed by shared UV count U. For each layer a u32 binding is stored; **binding 3** adds V2(U), while other binding values add no coordinate block. A single **Ix(I,U)** follows all layers. For `c`, the same U/layer/index sequence is present only when **1≤L≤4** and the caller's appearance-or-decal import setting is enabled. For `d`, **1≤L≤4** selects per-layer binding words and V2(V) only for binding 3; no U word or separate UV indices are stored.

`texture_import_enabled` is a required boolean API context for the `c` branch, tied to the native **SWImportAppearance OR SWImportDecals** gate. The decoder does not infer its value by trying alignments. All three installed `c` records have L=0, so their files do not prove the globals' runtime values. The two feature states and positive-layer branches have native-derived synthetic tests. Other native LOD-selection/rendering state is not replayed.

#### 14.88.4 Corpus endpoints, independent shape checks and limits

*Status: All 38 installed geometry EOFs and independent index/strip checks pass; full project semantics incomplete · Evidence: C-464*

All **38** installed `.smgGeom` members pass existing gzip/CRC checks and now reach exact decoded EOF: **45,280,926 bytes**. They contain **1,787 geodes**, each with one LOD, and **2,056 drawable records**:

| Drawable kind | Installed records |
|---|---:|
| `a` triangle sets | 138 |
| `b` indexed triangle sets | 2 |
| `c` indexed triangle-strip sets | 3 |
| `d` mono-indexed triangle-strip sets | 1,903 |
| `e` line-strip sets | 10 |
| `f` point sets | 0 |

There are **1,578** stored geode matrices and **265** material texture layers. All user-data bodies and recognized material bodies end without future/opaque bytes. Vector blocks use mode `a` **578** times and mode `b` **3,789** times; mode `c` has native/synthetic evidence only. Index/length blocks use one-byte values **2,782** times, two-byte values **1,046** times and four-byte values **once**.

The corpus audit independently unpacks all **1,916 strip-length groups** and verifies that their sums equal the stored index totals (`c`/`d`) or coordinate totals (`e`). It also checks **1,913 coordinate/normal index arrays** against their referenced list sizes, with no out-of-domain values. These are verified corpus properties, not new universal decoder rejection rules. Exact EOF, native count/width bindings, hashes and these independent shape checks jointly support the framing result; none alone proves complete project semantics.

`read_decoded` and gzip `read` return all records, raw packed values, material fields, names and user data. Standalone `read_vector`, `read_indices` and `read_geometry` expose bounded component grammars. Counts are checked against available bytes; unsupported tags/modes/versions, partial fields, wrong context types and unconsumed component extensions raise errors. The result reports **`complete_scene_framing=True`**, **`complete_document_schema=False`** and **`runtime_float_reconstruction_reproduced=False`**. Sized unknown material extensions remain explicitly retained and counted outside the installed complete-body cases.

`composer_scene_evidence.py` pins the twenty native ranges, three leaves, named exports and constants, referencing the preceding user-data evidence. `composer-scene-corpus.json` retains all 38 projections with each packed block's length/hash/preview; the production reader retains the full raw blocks. `composer-scene-synthetic.json` retains six drawable fixtures and an eight-bit constant-axis case. Nine focused tests cover every drawable kind, all packing modes/constant-axis combinations, index-width boundaries, normal/UV branches, zero counts, both texture contexts, shorter material columns, unknown sized bodies, mixed/multiple-LOD scenes, every drawable truncation, extensions and all installed EOFs.

This replaces the **unparsed geometry tail** for the scoped installed profile with full wire framing. It does not resolve all user-data scalar roles, texture enum/flag meanings, quantizing-writer rounding, native runtime transformations, rendering, units, XML/hierarchy/identity relationships, unsupported versions or other installed project formats. No global stored-byte regrade or all-project coverage percentage is inferred from the decoded geometry total.

All **1,472 regression tests pass** (210.616 seconds). Specification lint reports zero findings (650 headings, 96 anchors).

Evidence: C-464.
