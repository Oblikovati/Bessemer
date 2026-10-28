### 11.335 Surface wrappers and projected-line subtypes

*Status: Edition 0.355 checkpoint; eight added EOF streams and measured stored credit; semantics partial · Evidence: C-377*

`moSurfaceRef_w` primary table **0x4D17CC28**, locator **0x4D35CF58**, binds Serialize +0x28 to **0x4C2866C0** in `sldmodu.dll`. It invokes common reference-wrapper input **0x4C2842D0**, then reads four f64 values at **+0xA8/+0xB0/+0xB8/+0xC0** when version >0xF22. Earlier native versions initialize defaults instead; the supported ordinary context (>0x27CB) always reads the four doubles. `modelreferences.surface` already implements this exact body for derived references and is now locally available for fresh base-class objects. The four parameters remain unknown and retain unknown-byte attribution; observed values -1/1/1/-1 do not establish their domains or meanings.

Projected-line subtypes are defined in installed **sldMoPartu.dll**, SHA-256 **2201edfa07b1a7484acedb6db4600fa7a00e55b16090a03b92fe7d9b14cb2128**. Both call imported **moFeatureData_c::Serialize**, whose ordinary input is a nonnull feature-owner pointer at +0x08. They then call **suObArray::serializePtr** for +0x10: signed i32 presence, followed by a direct packed-count tagged-object array only when present. This is not an enclosing tagged `suObArray` object. Concrete suffixes are:

| Class | Primary table | Serialize interval | Input after owner and nullable array |
| --- | --- | --- | --- |
| moPLineDraft_c | 0x1804D87C0 | 0x18037AC20..0x18037AD10 | Raw f64 +0x18, then tagged moDirectionSpec_c pointer +0x20. |
| moPLineIntersect_c | 0x1804D89D0 | 0x18037AD10..0x18037ADD1 | Raw signed i32 +0x38, then three tagged suObList pointers at +0x20/+0x28/+0x30. |

The draft input omits its f64 only for historical versions **0x159..0xA10 inclusive**; modern input includes it. No intersection suffix version gate appears in the complete native implementation. Raw subtype values remain unknown. The lists retain null/shared pointer identity and use their existing typed bodies when fresh. Native imports, complete bounded instructions and exact module identities establish the fields independently of the source values.

`resolvedfeatures.read_data(..., surface_dependencies=True)` installs these three readers in the local archive and enables the PLine feature's subtype check for draft/intersection objects. Other feature-data classes remain rejected. The complete coverage path enables this verified extension; `read_supported` and `documented_bytes` accept `surface_dependencies=False, remaining_planes=False, mid_surfaces=False, reference_curves=False, reference_surfaces=False, geometric_dependencies=False, backup_dependencies=False, sketch_dimension_dependencies=False, isocurve_dependencies=False, weld_dependencies=False, weldment_dependencies=False, frontier_dependencies=False, alignment_dependencies=False, pattern_dependencies=False` to reproduce the edition 0.354 scope. Staged defaults and global registrations remain unchanged.

The nine targeted configurations contain **four draft objects**, **four intersection objects** and **three surface wrappers**. All eight projected-line objects have present arrays. Draft raw +0x18 is zero in all four cases. Intersection raw +0x38 is 7 or 15; their three lists are populated, including one two-item second list. These are observations, not complete value-domain claims. Eight streams reach exact EOF; the ninth, Verification nafems_ic22, advances to fresh **moPntSurfRefPlnData_c** and remains explicitly unsupported. `surface-dependencies-corpus.json` retains source hashes, root intervals/hashes, raw fields and populated collection counts.

The full modern part audit now reaches **847 exact-EOF streams / 59 explicit stops / zero context stops**, with **25,835 completed roots**. All **839** prior positives preserve exact endpoints, root counts and unknown-byte totals. Completed streams total **71,833,934 decoded bytes**, including **25,199,200 unknown bytes** and **46,634,734 known structure bytes**. `resolved-surface-audit-0.355.json` is reproduced with the earlier audit options plus `--surface-dependencies`.

`resolved_coverage.py --surface-dependencies` measures **81,710 additional known stored part bytes** beyond edition 0.354. Cumulative resolved-stream credit becomes **13,320,853** in the unchanged **25,137,466-byte** stored family denominator, leaving **11,816,613 unknown family bytes**. Total part unknown bytes become **55,046,779**; rounded coverage is **75.8% / 70.5%**, with assemblies **83.8% / 81.3%** and drawings **57.3% / 49.9%** unchanged. Unsupported contexts and streams earn zero. No full-document or installed-project completion is claimed.

`surface_dependencies_evidence.py` verifies both modules, primary bindings, complete serializer instructions/imports, common wrapper/feature-data dependencies and all previous positive audit rows. `surface-dependencies-native-evidence.json` retains those bytes. `resolved-feature-coverage-0.355.json` records every stored attribution; `resolved_coverage_evidence.py --surface-dependencies` independently verifies every source hash/span and ratio, writing `surface-feature-coverage-native-source-evidence.json`. The previous regrade remains reproducible with the extension disabled and `coverage-gaps-0.354.md` frozen under its original hash.

Tests check the different draft/intersection suffix boundaries with sentinels, signed values, null lists, truncation, all nine source outcomes and scoped registration. The complete-coverage regression checks all 906 modern decisions and the new known-byte sum, including zero credit for failed graphs. Remaining projected/surface features, unknown parameter meanings, other model streams, older/assembly/drawing contexts and all installed project formats remain unfinished.

Validation: all **1,102 regression tests pass** (144.073 seconds); specification lint reports zero findings (559 headings, 96 anchors).

Evidence: C-377.

