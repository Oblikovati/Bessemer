### 14.298 Embedded face-reference suffix gates and remaining wrapper dependency

*Status: historical suffix gates bound to installed model serializers; native object bodies remain unparsed · Evidence: C-674*

**Installed identity.** C-674 rechecks installed `sldmodu.dll` SHA-256 **346ad211760be406f587f338293d1793386518dddfc1bd461ac841dd0598487c**, the C-30 RTTI evidence, and the raw primary-vtable serializer slot for `moSingleFaceRef_w`: slot **5 / +0x28 → 0x4C2856E0**. The retained C-30 decompilation calls **0x4C2851F0** (face body), which calls **0x4C2866C0** (surface suffix), which calls **0x4C2842D0** (shared wrapper). C-674 hashes all four retained decompilations and their installed PE unwind fragments; it does not claim a new whole-function decompilation or body execution.

**Historical read gates after the common wrapper.** The surface/face/derived chain has the following version behavior, expressed in the nested model-archive version rather than the outer Simulation version:

| Stage | Read behavior |
|---|---|
| Surface suffix **0x4C2866C0** | Four doubles at **+0xA8/+0xB0/+0xB8/+0xC0** only when version **> 0xF22 (3874)**; older versions initialize runtime defaults without reading them. |
| Face body **0x4C2851F0** | A four-byte embedded-body presence value. Nonzero invokes kernel-body loading; an attribute `suObList` is read after that body when version **> 0x50B (1291)**. |
| Face integer suffix | Four-byte unsigned field **+0xC8** at version **>= 0xCDD (3293)**; older reads set it to zero without consuming bytes. |
| Derived single-face suffix **0x4C2856E0** | Four-byte integer **+0xD8** only when version **> 0x4F5 (1269)**. |

The current write path tests archive-helper condition mask **0x6000** before deciding whether to emit embedded geometry. That runtime condition is distinct from the serialized presence word. The read path follows the presence word, so a decoder must not assume geometry is absent solely because the Simulation provider constructs an archive helper with argument **0x4000**. Establishing all helper flag semantics and native presence values remains separate work.

**Comparison with native headers.** All **46** first-object `moSingleFaceRef_w` declarations in C-672 have archive versions greater than **3874**, and therefore also above the lower thresholds in this suffix chain. If their common wrapper/component prefix is parsed correctly, these samples require all four surface doubles, the presence word, the unsigned **+0xC8** suffix and signed **+0xD8** suffix; body-bearing records additionally require kernel and attribute-list framing. No missing face-suffix gate explains the current header-only boundary. This comparison does not consume the object bodies or prove that the presence words are zero.

The shared wrapper **0x4C2842D0** first reads a tagged component-reference object and can add sketch-specific context before its version-dependent scalars/transforms. Existing ordinary modern readers deliberately enforce narrower version/context support. Recovering that wrapper and its nested component references for the embedded historical versions is the outstanding dependency; relaxing their guards without evidence would not establish coverage. C-674 narrows the frontier while retaining the installed class/serializer provenance. No native object deserialization, target execution or global semantic byte credit occurs.

Evidence: C-674.
