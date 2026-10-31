### 14.316 Further embedded representation suffixes and 35 additional complete Restraint streams

*Status: nine installed class bindings added; 293 native Restraint streams reconstruct exactly, nine remain open · Evidence: C-692*

**Installed dispatch and suffixes.** C-692 binds each primary serializer slot in the installed model module and, where applicable, the class-specific continuation at virtual slot **+0x2A0**. The following representation bodies begin with the common representation keeper. A direct source-entity body below is one tagged representation pointer, including any nested body.

| Class | Installed serializer / continuation | Suffix after the common keeper |
|---|---|---|
| `moEndEdgeIdRep_c` | **0x4BBBDE70** | One signed four-byte word, +0x38 |
| `moEndFace3IntSurfIdRep_c` | **0x4BBB79B0**, slot +0x2A0 to **0x4BBBF0F0** | Three signed four-byte words, +0x38/+0x3C/+0x40 |
| `moSweepSideSurfIdRep_c` | **0x4BBB79B0**, slot +0x2A0 to **0x4BBBF3F0** | Two signed four-byte words, +0x38/+0x3C |
| `moWzdHoleSurfIdRep_c` | **0x4BBB79B0**, slot +0x2A0 to **0x4BBBF3F0** | Two signed four-byte words, +0x38/+0x3C |
| `moSideFace3IntSurfIdRep_c` | **0x4BBB79B0**, slot +0x2A0 to **0x4BBBED80** | Three signed four-byte words, +0x38/+0x3C/+0x40 |
| `moCirPatternSurfIdRep_c` | **0x4BBB7A40**, source helper **0x4BBBD740**, continuation **0x4BBBF020** | Direct source-entity body; one signed four-byte word +0x48; one additional signed four-byte word only when **0x23EB < archive version < 0x23EF** |
| `moLPatternSurfIdRep_c` | **0x4BBBBC80**, source helper **0x4BBBD740**, continuation **0x4BBBF190** | Direct source-entity body; two signed four-byte words +0x48/+0x4C |
| `moDerivedSurfIdRep2_c` | **0x4BBBBE30**, continuation **0x4BBBF210** | Direct source-entity body; signed four-byte word +0x48, then signed four-byte word +0x50 |

The circular-pattern historical interval is exactly versions **0x23EC–0x23EE**, inclusive; it is a four-byte field, not a count-driven payload. Static source-helper and derived-continuation branches for older versions remain outside the enabled **4307–19000** range. The new **0x4BBBBE30–0x4BBBBE89** decompilation confirms that the derived-surface variant calls the source/word helper and then restores its additional +0x50 word.

**Vertex component.** `moCompVertex_c` primary serializer binds **0x4BBB7E80**, the common boundary-component serializer already used by the face/edge paths. In the explicit embedded **0x4000** helper context, the supported body is the common component-reference base, alias byte/optional string, tagged boundary pointer and tagged external pointer. There is no edge-helper suffix. Unsupported reference-info, transform, old-reference and other conditional component-base bodies remain stops. This completes the component-vertex paths that previously prevented endpoint wrappers from progressing.

**Scoped implementation.** `cosworks_embedded_restraint_representation.py` adds these nine readers only to an explicit extension registry, preserves the **128** representation-depth bound, rejects earlier archive versions and leaves unknown nested classes as stops. C-692 checks installed module SHA-256, RTTI/serializer/continuation slots, helper function fragments, retained decompilations and parser/test/predecessor hashes. Shared serializer identity is supplemented by the derived continuation-slot checks.

**Native completion.** Fresh extraction/hash verification and candidate replay across **302 streams / 2023457 bytes** yield **293 distinct complete Restraint streams / 1705949 bytes**, exact EOF/reconstruction hashes. All 258 prior completions remain complete. This adds **35 streams / 415564 bytes** to C-691; **295** candidate attempts succeed. Runtime version selection is not inferred from candidate success.

One successful candidate per complete stream contains **468** newly supported bodies: **56** end-face three-word surfaces, **32** sweep-side surfaces, **14** end-edge representations, **70** wizard-hole surfaces, **282** circular-pattern surfaces, **four** vertex components, **two** linear-pattern surfaces and **eight** side-face three-word surfaces. Each event retains class/version, bounded byte range, SHA-256 and decoded fields. No derived-surface-2 body completes inside a complete native stream, and no successful native circular body uses the historical extra-word interval; those branches have static and synthetic evidence only. Nested event ranges overlap and are not summed as independent byte credit.

**Remaining stops and tests.** The remaining **nine streams / 317508 bytes** stop at `moPLineInterSurfIdRep_c` (**one**), `moPttnBodyEdgeIdRep_c` (**two**), `moSimpleDoubleSurfIdRep_c` (**one**), `moThinOffsetSurfIdRep_c` (**one**), `moPLineSilhIdRep_c` (**one**), `moDerivedEdgeIdRep2_c` (**two**), and embedded archive version **4281** (**one**), using the furthest-progress candidate per stream. Other incompatible candidate failures remain separately retained.

All **26** focused archive/Restraint tests pass. New tests cover each layout, both boundaries and interior of the circular historical interval, exact trailing-byte boundaries, every-byte truncation, nested source entities and recursive depth rejection. Physical field semantics, writer round trips, unsupported optional bodies and wider installed-format coverage remain partial. No target execution or global semantic byte credit occurs.

Evidence: C-692.
