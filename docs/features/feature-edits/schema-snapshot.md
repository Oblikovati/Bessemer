### 11.355 Move-face and bending feature composition

*Status: Verified framing in ordinary modern part archives; raw values remain partial semantics · Evidence: C-397*

The hash-bound installed `sldMoPartu.dll` selects three primary serializers:

| Class | Primary vtable | Serialize | Complete inherited stage |
|---|---|---|---|
| `moMoveFace_c` | 0x18043EA00 | 0x1801CCCC0 | body-feature base |
| `moBending_c` | 0x180499418 | 0x180301090 | body-feature base |
| `moBendingData_c` | 0x18040D378 | 0x180300BD0 | byte-free su_CObject base, then the owned array below |

Both feature vtables select imported lightweight/resolved concept serializers at offsets `+0x4B8/+0x4C0`; these inherited stages are not treated as empty. All gates below are satisfied throughout the supported modern part range, `v > 0x27CB`, in ordinary external-XML archive context.

**Move-face suffix.** After the complete body-feature base, the wire order is:

| Order | Native members | Encoding |
|---|---|---|
| 1 | +828, +820, +800 | f64, f64, i32 |
| 2 | direction +818 | tagged `moDirectionSpec_c` pointer |
| 3 | +838, array +810 | i32; nullable direct object array (i32 presence, then ReadCount and tagged entries when present) |
| 4 | line +830, list +808 | tagged `moLineRef_w`, tagged `suObList` pointers |
| 5 | dimensions +840, +848 | two tagged `moDisplayDim_c` pointers |
| 6 | +880, +884; point +888 | two i32; three f64 |
| 7 | vectors +8A0, +8B8, +8D0 | three three-f64 vectors |
| 8 | references +8E8, +8F0 | two tagged `moRefWrapper_c` pointers |
| 9 | vectors +8F8, +910 | two three-f64 vectors |
| 10 | transform +928 | compact `mgXform_c::restore` body |
| 11 | vector +9B0; +9C8, +A98, +A94 | three f64; three i32 |
| 12 | array +AA0 | direct object array: ReadCount and tagged entries |
| 13 | +AC4, +AC8 | i32, f64 |
| 14 | references +AD0, +AD8, +AE0 | tagged wrapper, point-reference, wrapper pointers |
| 15 | +AE8; reference +AF0; direction +AF8; +B30 | i32; tagged wrapper; tagged direction; i32 |

The direction read is selected when archive condition 1 is absent, or condition `0x200000` or `0x8000000` is present, or native helper `+0x834` is nonzero. The accepted ordinary archive context takes that stored branch. The list gate is `v > 0xA7B`; dimensions `v > 0xFB7`; the +880 through +9C8 group `v > 0xFCD`; +A98 `v > 0x1043`; +A94 `v > 0x10A7`; array +AA0 `v > 0x1294`; the +AC4 group `v > 0x1BDA`; final +B30 `v > 0x1BF3`. Runtime removal of selected array entries performs no extra archive reads.

The transform here explicitly calls **compact restore** at `0x321031D0`, unlike the fixed archive operator in 11.353. Its u8 matrix-presence prefix, optional matrix, three-f64 translation, f64 scale, u8 flags and any supported perspective continuation are consumed by the existing native-derived transform reader. Every serialized transform byte remains unknown semantics. A synthetic 34-byte no-matrix transform yields **266 unknown suffix bytes**; this count is not a fixed suffix size because the transform and collections vary.

**Bending feature and owned data.** The feature suffix is tagged `moBendingData_c` at +810, then tagged `moMatrixIndexHelper_c` at +820. The data pointer SHALL be non-null: native loading dereferences its owned array to populate an in-memory feature array. That copy consumes no further bytes.

The default and parameterized data constructors at `0x180103E70` and `0x1803004D0` construct a direct `gcBendingData_c` subobject at +8. Its constructor in hash-bound `sldgcu.dll` is `0x43BA9A60`; it allocates a `suObArray` at its own +98, giving outer pointer +A0. The constructed table `0x441A52F8` selects imported `suObArray::Serialize` at +28. This constructor/dispatch chain establishes a **direct array body without an additional MFC pointer tag** at the start of `moBendingData_c::Serialize`.

The data wire order is: direct object array +A0; tagged stock pointer +128; point +10 and vectors +28/+40/+58 (four three-f64 records); f64 +70/+78; one signed packed i32 containing runtime fields +80/+E8/+EC; f64 +88/+90; i32 +98; three tagged wrapper pointers +130/+138/+140; three `(tagged display-dimension pointer, i32)` pairs +148/+150, +158/+160, +168/+170; f64 +F0/+F8; final i32 +120. The modern dimension gates are `v > 0xAB5`, `v >= 0xADD`, and the final word `v >= 0xAE6`. The **168 raw scalar/geometry bytes** remain unknown. Preserve the stored packed signed word exactly; native division/remainder and default substitution modify runtime fields and add no serialized bytes.

| Installed source | Configuration | Exact EOF | Unknown bytes | Known structure bytes |
|---|---|---:|---:|---:|
| Plastics Examples/thin_wall_container.sldprt | 0 | 38,105 | 12,507 | 25,598 |
| tutorial/api/release.sldprt | 0 | 63,461 | 22,050 | 41,411 |

The two streams contain **101,566 decoded bytes**, including **34,557 unknown** and **67,009 known structure bytes**. `resolved-feature-edit-audit-0.375.json` records **896 complete / 10 explicit stops / zero context stops** among 906 modern part configurations, with **26,596 completed roots** across the audit. Every preceding 894 complete stream preserves endpoint, root count, source hash and unknown total. Complete streams now contain **80,649,945 decoded bytes**, including **28,578,762 unknown** and **52,071,183 known structure bytes**.

Add `--feature-edit-dependencies` to the full Edition 0.374 audit/regrade commands. The public reader enables the scope; the staged reader defaults it off. `resolved_coverage_evidence.py --feature-edit-dependencies` independently checks all **1,166 source rows**, hashes, stored spans and integer proportions. Added stored part credit is **10,756 bytes**. Cumulative resolved-family known bytes become **14,657,503**, leaving **10,479,963 unknown** of 25,137,466 stored bytes. Total part unknown bytes become **53,710,129** of 227,086,352. Rounded coverage remains **76.3% / 71.2%** for parts, **83.8% / 81.3%** for assemblies and **57.3% / 49.9%** for drawings. Remaining feature dependencies, raw meanings, complete documents and all installed project-format schemas remain open.

`feature_edits_evidence.py` checks three primary bindings, inherited concept imports, five part-DLL native bodies, the cross-module owned-array constructor and serializer, both public source EOFs and all 894 preceding positives. Five focused tests cover distinct field order, shared identities, signed packed-word retention, compact-transform framing, every suffix truncation, context rejection, null required data, local installation, disabled-scope zero credit and appended-byte rejection. The preceding C-396 packet retains its original hash through exact sources frozen under `research/frozen-decoder-0.374/`. All **1,173 regression tests pass** (165.333 seconds); specification lint reports zero findings (579 headings, 96 anchors).

Evidence: C-397.
