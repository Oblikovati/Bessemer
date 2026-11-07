# Approved schema snapshot

### 11.342 Parabola and 3D-cylinder backups with shared sketch handles

*Status: Native-bound ordinary modern bodies; two additional EOF streams; semantic coverage partial · Evidence: C-384*

The following primary tables bind four additional classes to the installed C-20 model DLL. Native object offsets identify members, not wire offsets. These readers are restricted to the existing modern ordinary archive context.

| Class | Primary vtable | Serialize | Native inheritance / modern body |
|---|---|---|---|
| `moParabolaBackedUpData_c` | 0x4D1641C8 | 0x4C280780..0x4C2809C7 | `moSegmentBackedUpData_c`; common segment prefix, four tagged point backups, two raw f64 values |
| `mo3DCylinderBackedUpData_c` | 0x4D158180 | 0x4C279CD0..0x4C279D68 | `mo3DSurfaceBackedUpData_c`; zero-byte ordinary object prefix, vector +08, point +20, raw f64 +38 |
| `sgConicHandle` | 0x4D1D01E0 | 0x4C591ED0 | `sgSegmentHandle` / `sgEntHandle`; exact shared handle body |
| `sg3DCylinderHandle` | 0x4CED9680 | 0x4C591ED0 | `sg3DSurfaceHandle` / `sgEntHandle`; exact shared handle body |

The parabola's common segment helper is **0x4C284FE0**, already documented for the line/segment backup layout: tagged `moPointBackedUpData_c` pointers +08/+10 and two raw i32 values +18/+1C. The native inheritance is **segment**, not line. `backupfrontier.parabola` reuses `sketchprefix.line_backup` solely because that function implements the same common segment wire body. The derived suffix is tagged point backups +20/+28, then +30/+38 when version is greater than **0x142C**, followed by raw f64 +40/+48 when version is greater than **0xC58**. Both comparisons hold throughout the supported modern context. With all pointers encoded as short null or map references, the complete body is **36 bytes**; fresh tagged point objects can increase that extent. Shared identity and null pointers retain normal archive behavior. The two raw words and doubles contribute **24 unknown bytes**, before any newly serialized point-backup contents.

The 3D-cylinder serializer directly calls `su_CObject::Serialize`, then reads a three-double vector, three-double point and one double. Its ordinary body is exactly **56 bytes**, with no tagged geometry header, derived owner field or version-conditioned suffix. All seven numeric values remain conservatively unknown. Floating-point bit patterns, including signed zero and a NaN payload, are retained by the primitive reader; this does not establish field meanings or geometric validity.

Both handle primary slots point directly to **sgEntHandle::Serialize**, and RTTI proves the stated ancestry. They consume the existing u16/extended-i32 index followed by two i32 fields; no conic- or cylinder-specific handle suffix is present. Native dispatch establishes this reuse independently of the source objects' values. Runtime sketch resolution remains outside this schema reader.

`resolvedfeatures.read_data(..., backup_dependencies=True)` installs the two backup readers and both shared handle readers locally. The staged default remains false. Public `read_supported` and `documented_bytes` enable them with all existing saved-context, declared-root, eligible-MBSM, exact-EOF and bounded-unknown guards. `backup_dependencies=False, sketch_dimension_dependencies=False, isocurve_dependencies=False, weld_dependencies=False, weldment_dependencies=False, frontier_dependencies=False, alignment_dependencies=False, pattern_dependencies=False` reproduces edition 0.361. Global registrations and inherited caller maps remain unchanged.

The targeted three-source replay yields:

| Installed source (under samples/) | Result | Decoded stream bytes | Complete roots | Unknown bytes at endpoint |
|---|---|---:|---:|---:|
| learn/Garden Trowel/Garden Trowel_Garden_Trowel_Handle.SLDPRT | exact EOF | 601,226 | 112 | 236,358 |
| tutorial/api/secondgrip.sldprt | exact EOF | 265,780 | 45 | 117,801 |
| Plastics Examples/trim_piece.sldprt | explicit unmapped-object-tag stop | 27,391 | 20 | 6,129 at diagnostic cursor |

The two complete source graphs contain **six** newly bound objects: one parabola backup, one conic handle, one cylinder backup and three cylinder handles. The trim-piece cursor at **13,813** follows the parser encountering unmapped tag **0x4874** at **0x35F3**. That diagnostic position does not complete its enclosing root or earn stream credit. The reader does not guess map entries or skip this failure.

The full **906-configuration** `resolved-backup-audit-0.362.json` reaches **866 exact-EOF streams / 40 explicit stops / zero context stops**, with **26,159 completed roots**. All **864** preceding positive streams preserve their exact endpoints, root counts and unknown totals. Complete streams contain **75,125,831 decoded bytes**, comprising **26,447,037 unknown** and **48,678,794 known structure bytes**. The two added streams contribute **512,847 known decoded bytes**.

`backup_frontier_evidence.py` verifies module identity, all four primary/ancestry bindings, three complete serializer ranges/instructions, the exact common-segment call, all three source graphs and preservation of every prior positive. `backup-frontier-native-evidence.json` binds these observations to the production decoder, retained decompilations and preceding segment/ellipse evidence. `backup-frontier-corpus.json` retains source hashes, completed root byte spans/hashes, handle values and the explicit remaining failure.

`resolved_coverage.py --surface-dependencies --remaining-planes --mid-surfaces --reference-curves --reference-surfaces --geometric-dependencies --backup-dependencies` writes `resolved-feature-coverage-0.362.json`. `resolved_coverage_evidence.py --backup-dependencies` independently verifies all **1,166 source rows**, hashes, decoded sizes, physical stored spans, proportions and completion decisions in `backup-feature-coverage-native-source-evidence.json`. Added stored credit is **103,053 part bytes**, taking cumulative resolved-family known bytes to **13,860,621** and leaving **11,276,845 unknown** of the unchanged 25,137,466 stored family bytes. Total part unknown bytes become **54,507,011** of 227,086,352. Part coverage is **76.0% / 70.8%**; assemblies remain **83.8% / 81.3%**, drawings **57.3% / 49.9%**. The frozen edition 0.361 gap report and historical verifier still pass. Full document and all installed project-format schemas remain unfinished.

Validation: three focused tests pass for common segment/suffix boundaries, shared/null point identities, truncation, raw cylinder bit patterns, context rejection, following sentinels, local handle registration and all three source graphs. All **1,120 regression tests pass** (173.659 seconds); specification lint reports zero findings (566 headings, 96 anchors).

Evidence: C-384.
