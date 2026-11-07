### 14.493 Dialog deformation-scale selector and two stored float32 scales

*Status: named scale-accessor branches and stored scalar types bound; UI mode names, scale computation and deformation geometry remain unresolved — Evidence: C-875*

Named **CcwPostDlgParam::GetDeformScaleFactor**, **DGXPOST:18008E9F0..18008EA1A**, selects a float result from the raw integer word at object **+51C**:

| Selector51C | Returned float32 source |
|---|---|
|0 |Stored member528 |
|1 |Stored member52C |
|Any other signed32 value |Constant1.0 |

The constant is loaded from **1802D7AEC**, exact bits **`0000803f`**. Both stored branches use MOVSS and return their source bits without arithmetic, clamping or validation in this accessor. Its complete leaf range includes all three returns. Unknown selectors are retained as raw values rather than rewritten to2 merely because they select the same constant branch.

The dialog reader places51C in the last four bytes of its initial56-byte group. The two float32 scale members are the first and second words of the following **eight_words528_544** group, at offsets0 and4 within that group. The temporary min/max combination word from14.492 lies between the groups; it is not part of the deformation selector or a scale value. Object offsets are not substituted for the stored absolute wire offsets reported by the parser.

**cosworks_deform_scale_fields.py** retains the selector and both raw scales, adds supplementary finite float views and reports the selected raw result plus its source. Nonfinite selected payloads and negative zero remain available as exact bytes; the helper does not canonicalize them or enforce positive scales. The names automatic/manual, physical units, computation of the two source values and actual deformation activation are not established by this accessor alone and remain unassigned.

C-875 verifies the exact named export, complete leaf instructions/constant and full reader probe against the installed provider. Repeated native replay finds **224 dialog prefixes**:150 selectors0 use member528, four selectors1 use52C, and70 selectors2 use the constant branch. Both stored values remain preserved even when unselected. This adds672 typed selector/scale annotations over2688 already parsed bytes, with no new global byte credit; the PostStyle-only field catalog is unaffected.

Two new tests cover all branches, arbitrary nonzero/unknown selector values, negative floats, exact negative-zero/NaN bits and input bounds. Four prior style/color tests repeat successfully. Native observations are stored settings and modeled selections, not execution of deformation geometry or the accessor. Prior prefix/version and legacy-failure qualifications remain explicit; no target code executes.

Evidence: C-875.
