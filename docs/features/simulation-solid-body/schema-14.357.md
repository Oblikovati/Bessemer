### 14.357 Complete retained solid-body Property framing through four embedded ID readers

*Status: all 680 retained solid-body Property streams frame completely; broader semantics remain incomplete · Evidence: C-734*

C-734 adds four already established model ID suffix grammars to the **local embedded archive registry**, after the existing feature/external-reference representation prefix. It preserves the embedded archive's version, null model context, helper flag 0x4000, recursion guard, and unknown-class failure behavior. It does not enable the ordinary model-context parser by fabricating a part-model context.

| Embedded class | Continuation after representation prefix | Primary serializer / concrete +2A0 suffix |
|---|---|---|
| moSimpleEdgeIdRep_c | one signed word | 4BBBDE70 / 4BBB6D50 |
| moCopySurfIdRep_c | direct source entity-ID object pointer, one signed word | 4BBBBC30 / 4D333900 |
| moMacroFeatRefSurfIdRep_c | direct source entity-ID object pointer, three signed words | 4BBBD420 / 4BBBF630 |
| moEndCapChamferSurfIdRep_c | two signed words | 4BBB79B0 / 4BBBF3F0 |

These orders reuse the model grammars in 11.18 and the retained appearance/end-cap evidence. C-734 freshly checks installed `sldmodu.dll` SHA-256, primary serializer pointers and concrete suffix pointers against retained RTTI/virtual data, with the explicit simple-edge and end-cap addresses. Primary serializers are class-specific in the first three rows; only the last row uses the shared `4BBB79B0` entry directly. The registry extension remains scoped to this body audit and does not silently widen every existing load decoder. Versions below 4307 and depth 128 remain explicit stops; representation depth is restored on error.

`cosworks_solid_body_expanded_audit.py` freshly replays the same **680 occurrences / 855786 bytes** as C-731. **All 680 now reach exact EOF and reconstruct their complete SHA-256 identity.** The former 22 incomplete occurrences / 34700 bytes are complete; no further unsupported class appears in this retained set after the four readers are added. Every earlier successful version attempt is matched by path, stream, payload hash, candidate version, endpoint, and reconstruction hash, proving that previous successes remain intact. There are 745 candidate attempts, 691 complete and 54 failed alternatives; candidate success still does not prove universal runtime study-index selection.

Five tests pass: three solid-body boundary/version tests and two local suffix-order/guard/error-unwind tests. The new readers retain signed words and source-object boundaries without assigning geometric business meanings. C-734 supersedes the **remaining 22 framing stops** reported in 14.355/C-731 for this corpus; those older results remain a historical checkpoint, not current incompleteness for the same identities.

The six SurfaceBody tail mismatches in 14.356 remain separate, as do inherited sheet Property streams, auxiliary records, field semantics, and other installed formats. Complete retained solid-body framing is not a 100% document-schema claim. No target executes; global semantic coverage credit remains zero.

Evidence: C-734.
