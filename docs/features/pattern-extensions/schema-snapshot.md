### 11.352 Table, sketch and dimension patterns with direct regeneration records

*Status: Native-bound ordinary modern part graph; three additional exact-EOF streams; meanings partial · Evidence: C-394*

This clause extends the ordinary non-DB external-XML part context above **0x27CB**, with the enabled variation baseline of 11.96 and archive condition **0x2000 clear**. Saved model/CMgr state, inherited map identities, eligible MBSM and exact EOF remain required. Runtime version-manager overrides and condition-0x2000 direct feature-reference substitutes remain excluded.

| Class | Primary vtable | Native serializer |
|---|---|---|
| `moTablePattern_c` | 0x4D107160 | 0x4BB36500 |
| `moSketchPattern_c` | 0x4D0F74E8 | 0x4BB32FF0 |
| `moDimPattern_c` | 0x4CDEE3E0 | 0x4BB23290 |
| `moDimPatternFeatInstance_c` | 0x4CDF1180 | 0x4BB22ED0 |
| `moSketchPatternSurfIdRep_c` | 0x4CF70100 | 0x4BBBBC30 |
| `moAllRangeDim_c` | 0x4D109978 | 0x4BB1E6D0 |

The three pattern serializers call the same common **0x4BB2E550** layer as the supported pattern family. Their inherited lightweight/resolved slots **+4B8/+4C0** select the existing concept implementations. Table and sketch patterns have virtual **+870 equal to zero**, so their common bodies reuse `featurepatterns.shared_pattern`. Dimension patterns have **+870 equal to one** and require an extra variation index array before the original-dimension records. The variation predicate remains **0x4C087430**, which tests the runtime version manager against **0x1437** as established in 11.96; neither empty shape data nor preceding flag bytes substitute for that predicate.

After the complete common pattern body, the table-pattern suffix is tagged `moCoordSysRef_w` at runtime +1140; tagged `moRefWrapper_c` +1148; nullable direct object array +1150; and a direct three-f64 point +1130. The nullable array uses the existing signed i32 presence word followed, when nonzero, by ReadCount and tagged entries. It has no outer array tag or array map slot. The point occupies **24 unknown bytes**. Earlier coordinate-wrapper conversion and point migrations precede this clause's modern minimum and add no modern fields.

The sketch-pattern suffix is tagged `moProfileRef_w` +1130; tagged `moRefWrapper_c` +1138; a direct three-f64 point +1140; and a direct transform-helper array +1148. The transform helper at **0x4BB1DC40** reads:

| Order | Field | Rule |
|---|---|---|
| 1 | element width u32 | Zero ends the array; otherwise divisible by **120** |
| 2 | capacity u32; growth u32; count u32 | Present only for nonzero width |
| 3 | exactly width × count bytes | Bounded against remaining input; all payload bytes remain unknown |

Native loading allocates a temporary `mgXform_64BitHelper_c` array, then reconstructs runtime transforms. The **0x78-byte helper width** and the **0x88-byte runtime transform stride** are different; the latter must not be used to advance the archive. Width multiples, metadata and raw payload are retained without assigning unverified transform fields semantic credit. Zero width is a four-byte body, not a four-word empty header.

If stored node flags at +2C contain **0x800**, the sketch suffix then reads raw i32 +1150, signed i32 pair count, and that many pairs of signed i32 values for +1158. Nonpositive counts consume no pairs, matching the native loop. The decoder retains pair order and duplicate keys; native map construction may subsequently replace a prior value. If the bit is clear, these fields are absent. The native setter that enables the bit after loading consumes no bytes and is not an extra stored flag. The point, +1150 value and pair values remain unknown; pair count framing remains structural.

The sketch-pattern surface-ID representation calls **0x4BBBD740**, then reads one raw signed i32 at +48. It reuses `modelboundary.derived_surface`: existing feature/external-reference header, a direct source entity-ID body at +38, then that final word. Runtime descriptor **0x4DAD7960** records an **0x50-byte** object, schema **0x80000001** and factory **0x4BBAA100**. Factory/constructor **0x4BBACE00** calls the embedded `moEntityId_c` constructor **0x4C6CB040** at +38; its serialization slot selects **0x4BBBC390**. The embedded ID has a tagged representation, without a separate outer ID tag or identity-map slot. Its final four bytes remain uncredited.

`moAllRangeDim_c` has `su_CObject` ancestry and is a small helper rather than a `Dimension_c` geometric body. Its serializer reads a tagged `moDisplayDim_c` at +08 and one raw **u8** at +10. The runtime integer storage of +10 does not widen its wire byte. The helper is bound locally; it does not broaden dimension-handle acceptance to unrelated classes.

The dimension-pattern common variation body retains the existing direct increments, u8 +9FC/+A00 and original-dimension records. In this modern +870-true branch, a **signed i32 count and that many i32 advanced indices** occur immediately after the two bytes and before the +A08/+A28 original-dimension arrays. Nonpositive stored counts consume no indices. Index values remain unknown. The two direct original-dimension bodies +A48/+CB0 and the established common tail follow unchanged; omitting the advanced array would misalign all subsequent records.

The dimension-pattern derived suffix is:

| Order | Body |
|---|---|
| 1 | Tagged display dimensions +1460, +1468, +1478, +1480, in that order |
| 2 | Raw u8 +14FC; tagged display dimensions +1470, +1488 |
| 3 | Unqualified tagged objects +1490/+1498; raw i32 +1650 |
| 4 | Tagged `suObList` +14F0 in the condition-0x2000-clear branch |
| 5 | Raw u8 +15F8; raw i32 +15FC |
| 6 | Signed i32 count; direct regeneration-status records for +1600 |
| 7 | Signed i32 count; direct regeneration-status records for +1620 |
| 8 | Signed i32 count; unqualified tagged references for +1648 |

The fixed raw suffix values total **10 unknown bytes** before nested status records. Count fields are full signed i32 values, not ReadCount. Positive counts are bounded against minimum remaining record/tag widths; nonpositive counts consume no entries. The alternate condition-0x2000 branch replaces order 4 with direct feature-reference data and is not guessed from possible tag bytes. All suffix gates, including the six-dimension branch above 0x1826 and the later status/reference fields, precede this clause's minimum version.

`moDimPatternRegenStatus_c::Serialize` at **0x4BB23070** is a **direct record**, without an MFC class tag or record map slot. It reads three raw i32 values at +00/+04 and the feature-presence selector. When the third value is zero, a tagged `suObList` supplies +10; otherwise no list tag is present. A full signed i32 child count follows, then that many direct feature-status records. The fixed status values remain **12 unknown bytes**.

Each direct `moFeatureRegenStatus_c` record, native **0x4BB25430**, reads a raw i32 value and raw i32 string selector; selector zero adds a CString at +08. It then reads a raw i32 feature selector; selector zero adds a tagged `suObList` for +18. All three fixed words and any stored string remain unknown for value credit. Shared list pointers retain their cached identity. `SerializePass2` at **0x4BB38010** only reconstructs runtime status objects from already loaded data; it does not append another wire body. Native list extraction, null filtering and owner updates are runtime operations, not additional stored presence flags.

`moDimPatternFeatInstance_c` uses the existing complete `moBodyFeature_c` body; its concept lightweight/resolved slots match the inherited reader. Its suffix is raw i32 +800; tagged parent `moDimPattern_c` +808; raw i32 +810, +814 and +804, in that order. All four raw words remain **16 unknown bytes**. Gates for +814 above 0x1C34 and +804 from 0x1C3C precede this scope. Parent references can target the currently cached pattern object, so the decoder preserves recursion instead of parsing a duplicate parent.

`patternextensions.install` installs six readers locally. `resolvedfeatures.read_data(..., pattern_dependencies=True)` selects them explicitly; the staged default remains **false**. Public `read_supported` and `documented_bytes` enable the option with complete ordinary context. Use **`pattern_dependencies=False`** for the preceding Edition 0.371 public scope. Earlier regrade profiles explicitly pass false. Global registrations and caller reader tables are unchanged.

The native source replay reaches exact EOF for all three formerly blocked streams:

| Source | Decoded bytes | Complete roots | Unknown bytes | Known structure bytes |
|---|---:|---:|---:|---:|
| Learn/Sheet Metal `Simple Sheet Metal Part.SLDPRT` | 467,464 | 42 | 151,444 | 316,020 |
| API `bottle.sldprt` | 102,426 | 24 | 36,318 | 66,108 |
| API `tablepattern.sldprt` | 28,830 | 24 | 10,089 | 18,741 |

The sheet-metal source contains one sketch pattern and **150** sketch-pattern surface-ID representations. Its transform array has width zero; its stored 0x800 branch has pair count zero. The bottle contains one dimension pattern and **seven instance features**; its advanced indices are **1 through 7**, both regeneration arrays are empty and its trailing reference count is seven. The table-pattern source contains one table pattern and **four** all-range helpers. Populated transform payloads, repeated map pairs and populated nested status records have native-layout and synthetic boundary evidence; they are not misreported as populated native fixtures.

The three added streams total **598,720 decoded bytes**, including **197,851 unknown** and **400,869 known structure bytes**. `resolved-pattern-audit-0.372.json` audits all **906** modern part configurations: **886 complete / 20 explicit stops / zero context stops**, with **26,511 completed roots** across the audit. All **883** preceding positives preserve endpoints, root counts and unknown totals. Complete streams contain **79,566,504 decoded bytes**, including **28,225,866 unknown** and **51,340,638 known structure bytes**. The preceding native C-393 packet reproduces byte-for-byte with its original decoder hash retained in `research/frozen-decoder-0.371/resolvedfeatures.py`.

Add `--pattern-dependencies` to the full Edition 0.371 audit/regrade commands to generate the new reports. `resolved_coverage_evidence.py --pattern-dependencies` independently checks all **1,166 source rows**, hashes, decoded totals, stored spans and integer proportions. Added stored part credit is **71,241 bytes**. Cumulative resolved-family known bytes become **14,417,778**, leaving **10,719,688 unknown** of the unchanged 25,137,466 stored bytes. Total part unknown bytes become **53,949,854** of 227,086,352. Rounded coverage remains **76.2% / 71.1%** for parts, **83.8% / 81.3%** for assemblies and **57.3% / 49.9%** for drawings. The preceding Edition 0.371 report and frozen gap report still verify. No whole-document or installed-project-format completion is claimed.

Seven focused tests cover transform-width multiples, metadata/bounds and opaque-byte accounting, stored sketch flags, signed counts, duplicate map pairs, nullable direct arrays, advanced-index ordering, nested status strings/shared lists, instance parent identity, raw byte widths, truncation, scoped installation, three public EOF streams, disabled-profile zero credit and appended-byte rejection. All **1,160 regression tests pass** (174.808 seconds); specification lint reports zero findings (576 headings, 96 anchors).

Evidence: C-394.
