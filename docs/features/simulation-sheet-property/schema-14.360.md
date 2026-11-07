### 14.360 Plane-section components and explicit 4281 inherited sheet Property completion

*Status: all 238 retained inherited sheet Property streams frame completely; separate SurfaceBody historical mismatches remain · Evidence: C-737*

Static RTTI binds `moCompPlaneSectionBody_c` primary table **4CEC3D18**, slot +28, to **4BBBA2D0**. This is the same component-body serializer already retained for solid components; the class therefore uses that verified wire grammar in the local embedded registry. C-737 retains the RTTI class/table/slot identity and freshly checks the installed pointer and model hash. No additional plane-section suffix is inferred from its class name.

The five older reference archives use **native archive version 4281**. The constant-surface base `4B90DB10` reads member +A8 only above **0x1BBD (7101)**; therefore that word is absent at 4281. The explicit local `constant_surface` profile reads the existing common wrapper and then geometry presence directly. Nonzero geometry remains an explicit kernel-body stop. This adds one exact historical profile without lowering the general embedded archive minimum or accepting arbitrary intermediate versions.

The corresponding explicit 4281 component profile reads the same version-conditioned component-reference prefix and shared body continuation. Native body gates are: face pointer above 0x49A, counted word array above 0x864, word +108 above 0x94D, and words +104/+100 from 0x1040. All apply at 4281. The component-reference +48 word starts above 0x11AB and is absent at 4281; later presence/text/transform fields keep their existing higher gates. The historical reader keeps bounded counts, rejects unresolved direct-reference branches, and never fabricates model context.

`cosworks_sheet_property_final_audit.py` freshly replays **all 238 inherited sheet Property occurrences / 186372 bytes** to exact EOF with complete SHA-256 reconstruction. This resolves the remaining **13 / 20815 bytes** from C-736: eight plane-section components and five historical wrappers/components. Every C-736 successful candidate is preserved by identity, version, endpoint and reconstruction hash. Across 243 attempts, 240 complete; failed alternatives remain retained and runtime study-index mapping is still a separate question.

Nine tests pass, including explicit 4281 wrapper word omission, zero/nonzero kernel presence, the 56-byte empty historical component boundary, and rejection of an unintended neighboring archive version. C-737 supersedes the 13 remaining framing stops in 14.359 for these identities; C-735/C-736 remain historical checkpoints. Full inherited sheet Property framing does not resolve field business meanings, auxiliary records, the six separate SurfaceBody tail mismatches, or other installed formats. No target executes; global semantic coverage credit remains zero.

Evidence: C-737.
