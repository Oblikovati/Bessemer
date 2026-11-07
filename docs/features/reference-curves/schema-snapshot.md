### 11.338 Reference-curve body and embedded array branches

*Status: Native-bound research reader; eleven source objects; no supported coverage credit · Evidence: C-380*

The primary `moRefCurve_c` vtable in the C-20 model DLL is `0x4D164450`, with Serialize at `0x4C282CF0..0x4C2839F7`. Its constructor `0x4C2A9DC0..0x4C2A9F1E` proves the directly owned collection types: DWORD arrays at +888/+8A8/+8F0/+910, `suObArray` at +8C8 and `suObList` at +948. Another DWORD array at +800 is constructed but is not serialized directly by this derived body. Constructor storage and runtime assignments must not be treated as extra wire fields.

The following ordinary modern layout follows `moBodyFeature_c::Serialize`. Numeric fields marked raw retain unknown meanings. Direct collections serialize their existing count and entries without outer object tags.

| Order | Wire field | Gate or interpretation |
|---|---|---|
| 1 | Tagged `suObArray` pointer +860 | Modern replacement for the pre-0xD5 single-face pointer |
| 2 | Existing kernel archive body +820 | Present after version 0xDF; strict existing kernel decoder |
| 3 | Direct `suObList` +948 | Version greater than 0x10C8 |
| 4 | Raw i32 node ID +828 | Version at least 0x215; runtime node lookup consumes no extra input |
| 5 | Raw i32 +87C, +880, +878; direct DWORD array +888 | Version at least 0x329 |
| 6a | Direct DWORD array +8A8 | Bit 1 of +878 is set |
| 6b | Direct object array +8C8 | Bit 1 clear and the +860 array is nonnull and nonempty |
| 6c | Tagged `moCompRefPlane_c` +8E8 | Bit 1 clear, +860 null/empty, and version greater than 0x2B94 |
| 7 | Direct DWORD arrays +8F0 and +910 | Follow the selected branch |
| 8 | Raw i32 +930 | Version at least 0x333 |
| 9 | Raw u16 +858, widened by the runtime | Version greater than 0x385 |
| 10 | Raw i32 +938 | Version at least 0x3BF |
| 11 | u16 body count; that many kernel bodies, each followed by a direct attribute list | Version greater than 0x2B87 |
| 12 | Raw i32 +934; tagged plane +868 and direction +850 | Same gate as order 11 |
| 13 | Tagged `moCurveRef_w` +870 | Version greater than 0x3EFB |

The modern ordinary context resolves the historical prefix gates, while retaining the three later version comparisons. Historical runtime repairs, entity lookups, attribute attachment and model assignment add no input. This research reader does not emulate them or claim support for legacy bodies.

At the edition 0.358 checkpoint, `solidworks/research/refcurve.py` installed the reader only in its local replay archive and left public resolved-feature registration and byte credit unchanged. Clause 11.339 integrates the verified body into the guarded public path. Across four installed Config-0 sources, the reader completes **11** reference-curve objects. `Simulation Examples/Verification/comp_buck.sldprt` reaches exact EOF at **33,236 bytes / 25 roots**, with 11,743 unknown bytes; `tutorial/api/hemisphere.sldprt` reaches exact EOF at **49,122 bytes / 39 roots**, with 16,421 unknown bytes and eight reference curves. `learn/Mouse/Mouse Master.SLDPRT` completes its curve and 29 roots before stopping at `moBlendRefSurface_c`; `tutorial/molds/telephone.sldprt` completes its curve and 26 roots before stopping at `moCompRefCurve_c`. Failure cursors are diagnostics, not credited endpoints.

Observed +878 values are 1 and 2. The flag-2 source uses the +8A8 branch; flag-1 sources use nonempty +860 arrays and direct +8C8 objects. All eleven extra-body counts are zero. Positive extra-body counts and the empty-reference +8E8 branch are not source-observed. Focused synthetic tests verify all three array/plane branch boundaries, a following sentinel, truncation and the version-19000 suffix. These checks do not establish complete runtime loading or all kernel-body representations.

`refcurve_evidence.py` verifies the installed module SHA-256, primary RTTI binding, complete native function ranges/instructions, imported collection constructors and all four source replays. `refcurve-native-evidence.json` binds those results to the reader and retained decompilation hashes; `refcurve-corpus.json` retains source hashes, completed root endpoints, array counts and explicit next stops. Two focused tests pass. The preceding 1,107-test regression result remains the production baseline; no production reader changed in this edition. Full corpus integration, bounded unknown accounting and every-source stored revalidation are still required before adding coverage. Annex G remains at edition 0.357, and full document/all-installed-project schemas remain unfinished.

Evidence: C-380.

### 11.339 Guarded reference-curve integration and stored-byte revalidation

*Status: Native-bound ordinary modern body; two additional supported EOF streams; semantic coverage partial · Evidence: C-381*

`solidworks/referencecurves.py` implements the native layout in 11.338. `resolvedfeatures.read_data(..., reference_curves=True)` installs `moRefCurve_c` only in that archive's local reader map. The staged default remains false. `read_supported` and `documented_bytes` enable it by default, retaining complete saved CMgr/model context, declared root counts, eligible MBSM, exact EOF and bounded unknown accounting. `reference_curves=False, reference_surfaces=False, geometric_dependencies=False, backup_dependencies=False, sketch_dimension_dependencies=False, isocurve_dependencies=False, weld_dependencies=False, weldment_dependencies=False, frontier_dependencies=False, alignment_dependencies=False, pattern_dependencies=False` replays the preceding public scope; historical regrade scripts explicitly pass this false when their new option is absent. Global registration remains unchanged.

The four-source replay retains all eleven object boundaries and both next-dependency failures from 11.338. The two previously research-only EOFs now pass the complete supported path. Synthetic tests additionally exercise the exact comparisons at **0x2B87/0x2B88**, **0x2B94/0x2B95** and **0x3EFB/0x3EFC**, as well as version 11000. They include a positive extra-body count with an existing suppressed kernel body, following sentinels, truncation, all three array/plane branches and conservative raw-byte totals. Positive non-suppressed extra bodies remain source-unobserved; unknown DWORD contents and raw numeric meanings remain uncredited.

`resolved-reference-curve-audit-0.359.json` records **858 exact-EOF streams / 48 explicit stops / zero context stops** across all 906 modern configurations, with **25,936 completed roots**. Every one of the preceding **856** positives preserves its exact endpoint, root count and unknown-byte total. Completed streams contain **72,954,998 decoded bytes**: **25,542,573 unknown** and **47,412,425 known structure bytes**. The two added streams contribute 54,194 known decoded bytes. The broader document corpus and failed streams do not inherit these completion claims.

`reference_curves_integration_evidence.py` verifies all prior positives, both added streams and the aggregate totals, and binds the production decoder and native/source packets in `reference-curve-integration-evidence.json`. `refcurve_evidence.py` rechecks the installed native ranges, constructor imports, RTTI and four source replays; its retained packet remains the native basis. Reproduce the new full audit with the preceding saved-context options plus `--reference-curves`.

`resolved_coverage.py --surface-dependencies --remaining-planes --mid-surfaces --reference-curves` regrades every installed container document and writes `resolved-feature-coverage-0.359.json`. `resolved_coverage_evidence.py --reference-curves` independently verifies all **1,166 source rows**, hashes, decoded sizes, stored intervals, integer proportions and audit outcomes in `reference-curve-feature-coverage-native-source-evidence.json`. The preceding gap report is frozen as `coverage-gaps-0.357.md`; its historical verifier still passes.

The added stored part credit is **16,280 bytes**, taking cumulative resolved-family known bytes to **13,620,145** and leaving **11,517,321 unknown** of the unchanged 25,137,466 stored bytes. Total part unknown bytes become **54,747,487** of 227,086,352. Rounded part coverage remains **75.9% / 70.7%**, assembly coverage **83.8% / 81.3%**, and drawing coverage **57.3% / 49.9%**. This completes no whole document, and the schemas of all installed project formats remain within the unfinished objective.

Validation: all **1,110 regression tests pass** (156.606 seconds), including three focused reference-curve tests. Specification lint reports zero findings (563 headings, 96 anchors).

Evidence: C-381.
