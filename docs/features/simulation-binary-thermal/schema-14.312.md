### 14.312 Six embedded surface representations and four additional complete Pressure streams

*Status: six scoped surface branches validated; 86 native Pressure streams fully reconstruct, three remain open · Evidence: C-688*

**Installed bindings.** Static RTTI and primary serializer-slot checks bind `moFilletSurfIdRep_c` to **0x4BBBDE70**, `moShellSurfIdRep_c` to **0x4BBBD740**, and `moDerivedSurfIdRep_c` to **0x4BBBBE90**. The derived serializer calls common representation base **0x4BBBBF50** and continuation **0x4BBBF210**. `moMirPatternSurfIdRep_c` uses the **0x4BBBD450** relative-jump thunk to **0x4BBBD740**; C-688 retains the slot-target bytes and resolves this thunk before hashing the implementation. Fresh static range/decompilation output is retained for these implementations. Shell/mirror input paths require helper **+0x20** to remain zero, as initialized in this embedded context; historical versions below **0x180** take other conversion paths and are outside this reader.

`moThinClosingSurfIdRep_c` and `moFromSktEnt3IntSurfIdRep_c` use common serializer **0x4BBB79B0**, which calls the common representation base and dispatches through virtual slot **+0x2A0**. Installed slot checks resolve the respective continuations to **0x4BBBF3F0** and **0x4BBBED80**. Shared serializer identity alone does not determine a derived class's suffix; both continuation slots and function bytes are retained.

**Scoped wire layouts.** Each body begins with the already documented representation keeper: a tagged feature pointer and tagged feature-reference pointer, including any nested bodies. The suffixes are:

| Class | Suffix after the common keeper |
|---|---|
| `moFilletSurfIdRep_c` | One signed four-byte word, object +0x38 |
| `moThinClosingSurfIdRep_c` | Two signed four-byte words, +0x38/+0x3C |
| `moFromSktEnt3IntSurfIdRep_c` | Three signed four-byte words, +0x38/+0x3C/+0x40 |
| `moShellSurfIdRep_c` | Direct source-entity body, consisting of one tagged representation pointer |
| `moMirPatternSurfIdRep_c` | Same direct source-entity body in this context |
| `moDerivedSurfIdRep_c` | Direct source-entity body, then one signed four-byte word, +0x48 |

`cosworks_embedded_surface.py` supports these layouts only through the explicit embedded reader registry. It conservatively rejects archive versions **0xDAA** or earlier, retains the **128** representation-depth bound and leaves all unknown classes as stops. The derived continuation's historical external-object path is not generalized. Numeric words retain their raw values without assigning physical meanings.

**Native completion.** Fresh extraction/hash verification and replay of all original **89** stream baselines produce **86 distinct complete streams / 3993551 bytes**, exact EOF and reconstruction hashes. All 82 C-687 complete streams remain complete. The extension adds **four streams / 68189 bytes**: the chair assembly, two piping-assembly streams and vessel-weldment part. There are **87** successful candidate attempts; stored candidate versions remain distinct from proven runtime version dispatch.

The remaining **three streams / 12561 bytes** stop at `moRadiateEdgeIdRep_c` in `nafems_3dnlg7.sldprt`, `moVertexRef_c` in `nafems_ic9.sldprt`, and the edge-specific component continuation in `nafems_nl9.sldprt`. Incompatible coexisting version-candidate failures remain retained separately.

**Validation limits.** All **14** focused embedded-archive tests pass. New tests exercise each of the six suffixes at versions **6000/14000/19000**, exact boundaries with trailing bytes, truncation at every byte, and recursion/version guards. C-688 binds the installed model hash, serializer and continuation slots, static function fragments, retained decompilations, reader/audit/test sources and predecessor evidence. These results establish framing within the tested context; semantic interpretation, writer round trips and coverage across all installed formats remain partial. No target execution or global semantic byte credit occurs.

Evidence: C-688.
