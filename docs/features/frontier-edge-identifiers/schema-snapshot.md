### 11.350 Lock/environment features, pattern identifiers and radius/edge helpers

*Status: Native-bound ordinary modern part readers; seven additional exact-EOF streams; meanings partial · Evidence: C-392*

This clause extends the ordinary modern external-XML part context above **0x27CB**. It preserves the saved model/CMgr context, inherited object map, eligible MBSM continuation and exact-EOF requirements of 11.349. It adds seven class bindings; the alignment-group and thread-instance classes inspected in the same research batch remain unsupported.

| Class | Primary vtable | Native serializer | Supported body |
|---|---|---|---|
| `moLockFeature_c` | 0x4CF93428 | 0x4BC23E20 | Tail jump to existing `moFeature_c::Serialize` at 0x4BB255E0; no suffix |
| `moSMEnvTableFeat_c` | 0x4D1E79D0 | 0x4B8D7CA0 | Tail jump to existing `moModelFeature_c::Serialize` at 0x4BB2CDE0; no further suffix |
| `moTrimWireExtEdgeIdRep_c` | 0x4CF050A0 | 0x4BBBBEC0 | Existing entity-representation body, then two raw signed i32 fields +38/+3C |
| `moPttnBodyEdgeIdRep_c` | 0x4CF06AF0 | 0x4BBBD460 | Existing entity-representation body; direct entity-ID +38; raw signed i32 +48/+4C |
| `moLPatternItemId_c` | 0x4D129340 | 0x4B6859B0 | Two raw i32 fields +20/+24; raw u8 +08; raw f64 +10 |
| `edgeParamsObject_c` | 0x18047AE70 | 0x180272390 | Inherited helper +08 dimension; tagged component edge +10; raw i32 +20; tagged dimensions +28/+30 |
| `featRadiusObject_c` | 0x18047AD38 | 0x180272670 | Inherited helper +08 dimension; unqualified tagged object +10 |

The first five bindings belong to the installed model DLL; the last two belong to the installed part DLL. RTTI ancestry, primary slot +28 and complete native code are retained in `frontier-features-native-evidence.json`. The two feature serializers are **five-byte direct jumps**, followed by native padding; that padding is not serialized data. The model-feature body retains its established feature layer and +354 word, including the existing historical-version qualification on its intermediate helper. No empty feature suffix is inferred merely from a class name.

The trim-wire representation reuses `modelboundary.end_face`. The pattern-body representation reuses `thinextrusions.silhouette_id`, whose native entry is exactly 0x4BBBD460. Its runtime descriptor at **0x4DAD6940** records an **0x50-byte** object and factory **0x4BBA7B30**. The factory calls **0x4BA954F0**, which constructs the embedded entity ID at +38 through **0x4C6CB040**. The embedded primary table at **0x4D1FB380** selects **0x4BBBC390** for serialization. Thus +38 is a direct entity-ID body containing a tagged representation; it has no separate outer object tag or identity-map slot. Its two trailing i32 values remain **eight unknown bytes**, as do the trim-wire suffix values. Existing feature/external-reference and representation pointer identities remain intact.

The linear-pattern item calls only the no-data `su_CObject` base before its listed values. The final u8/f64 pair is present above **0x1436**, so the complete modern payload is **17 bytes**, all conservatively unknown. The byte is not widened to a DWORD because its runtime storage is an integer. The original byte value, signed integer extremes, non-finite f64 values and following object boundary are preserved. No enum meaning is assigned from observed values alone.

The part-DLL helper base at **0x180272E50** calls `su_CObject::Serialize`, then reads a tagged `moDisplayDim_c` at +08. The edge intermediate layer at **0x180272420** reads tagged `moCompEdge_c` +10 and raw signed i32 +20 above **0x4F2**. The concrete edge-parameter reader then reads the two display-dimension pointers. The radius reader calls the same helper base and reads an unqualified `CObject` pointer +10; its version check above **0x70F** performs runtime version handling without consuming another field. The decoder preserves that unqualified reference instead of guessing a radius-specific target class. Edge raw +20 remains **four unknown bytes**. Shared, null and repeated pointers retain the normal cached archive-map behavior.

`frontierfeatures.install` installs these seven readers only in the selected archive. `resolvedfeatures.read_data(..., frontier_dependencies=True)` enables them explicitly; the staged default remains **false**. Public `read_supported` and `documented_bytes` enable them by default. Use **`frontier_dependencies=False, alignment_dependencies=False, pattern_dependencies=False`** for the preceding Edition 0.369 public scope. Historical regrade profiles explicitly forward false for this extension; the new option does not change global registrations or the caller's reader table.

The seven newly complete source streams are:

| Source | Decoded bytes | Roots | Unknown bytes | Known structure bytes |
|---|---:|---:|---:|---:|
| Learn/Mouse `On Off Switch.SLDPRT` | 167,509 | 43 | 80,789 | 86,720 |
| Simulation `pipes.sldprt` | 129,172 | 31 | 53,360 | 75,812 |
| API `db15-e.sldprt` | 322,471 | 48 | 114,481 | 207,990 |
| API `electronic housing.sldprt` | 137,465 | 34 | 46,357 | 91,108 |
| API `freezebarneedsrebuild2.sldprt` | 17,234 | 23 | 6,775 | 10,459 |
| DimXpert `bracket_auto_manual.sldprt` | 233,110 | 38 | 90,754 | 142,356 |
| Routing/Electrical `electronic housing.sldprt` | 137,482 | 34 | 46,357 | 91,125 |

The two electronic-housing sources remain distinct source/hash rows despite similar contents. Their contributions are not deduplicated by filename or decoded size. The seven added streams contribute **1,144,443 decoded bytes**, including **438,873 unknown** and **705,570 known structure bytes**. The sheet-metal source advances from its environment feature to explicit `moSketchPattern_c` at **0x3EAF3**; API `bottle.sldprt` advances from edge parameters to explicit `moDimPattern_c` at **0xB355**. Neither incomplete stream receives prefix credit.

`resolved-frontier-audit-0.370.json` audits all **906** modern part configurations: **880 complete / 26 explicit stops / zero context stops**, with **26,464 completed roots** across the audit. All **873** preceding positives preserve endpoints, root counts and unknown totals. Complete streams occupy **78,047,333 decoded bytes**, with **27,560,544 unknown** and **50,486,789 known structure bytes**. The preceding native C-391 packet reproduces byte-for-byte against its exact frozen `research/frozen-decoder-0.369/resolvedfeatures.py` source.

Add `--frontier-dependencies` to the full Edition 0.369 audit/regrade commands to generate the new reports. `resolved_coverage_evidence.py --frontier-dependencies` independently verifies all **1,166 source rows**, source hashes, stored spans, decoded totals and integer proportions. Added stored part credit is **119,469 bytes**. Cumulative resolved-family known bytes become **14,186,944**, leaving **10,950,522 unknown** of the unchanged 25,137,466 stored bytes. Total part unknown bytes become **54,180,688** of 227,086,352. Parts measure **76.1% / 71.0%**, assemblies remain **83.8% / 81.3%**, and drawings remain **57.3% / 49.9%**. The preceding Edition 0.369 report and frozen gap report still verify. No whole-document or installed-project-format completion is claimed.

Five focused tests verify exact byte widths, unknown accounting, non-finite preservation, every truncated item/edge-helper boundary, mapped reference order, direct entity-ID map behavior, local registration, context rejection, seven public EOF streams, disabled-profile zero credit and rejection of appended bytes. All **1,147 regression tests pass** (159.087 seconds); specification lint reports zero findings (574 headings, 96 anchors).

Evidence: C-392.
