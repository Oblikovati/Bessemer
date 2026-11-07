### 14.349 Beam child Property bodies and raw-value preservation

*Status: all 163 retained child Property streams complete; scalar/link semantics, folder/joint schemas and runtime repairs remain open · Evidence: C-725*

BeamManager constructs each beam through **0x180A59D00** and invokes its storage slot **+0x28**. Primary table **0x181F551C8** slot **0x181F551F0** selects **0x180A5BA00**, which opens separate **Property** and **Material** streams. The Material grammar and all **150** retained beam materials are validated in 14.348. C-725 now covers the distinct child Property body, not the manager's small direct Property header from 14.345.

**Reference prefix and scalar header.** A child Property begins with a direct **CSWPersistReferenceContainer** body: bounded signed reference count, that many versioned references with explicit embedded-archive dependencies, then the two four-byte container-tail words. There is no outer LBC collection count or common load subtype/tail. Next read four signed four-byte words at beam members **+0xE8/+0xC0/+0xC4/+0xE4**, signed two-byte shorts **+0xE2/+0xE0**, and persisted strings **+0xD0/+0xC8**, in that exact order.

**Direct helper at beam +0x120.** Function **0x180A5C600** restores the following historical fields; offsets shown here are relative to this embedded helper, not the beam:

| Gate | Stored helper fields |
|---|---|
| **v >= 840014** | Four doubles **+0x48/+0x50/+0x58/+0x60** |
| Additionally, **v >= 850035** | Three doubles **+0x68/+0x70/+0x78** |
| Additionally, **v >= 840024** | Signed four-byte word **+0x80**, after the doubles |

Below the first gate, native defaults are assigned without consuming helper bytes. The native loader also range-checks some doubles and may replace several values with defaults. The independent parser retains the original double bits and does not reproduce that repair or infer physical units. The current writer emits all seven doubles and its final word; it is not evidence that older forms contain them.

**Counted triples and remaining fields.** At **v >= 820007**, read a signed four-byte count and exactly that many triples of doubles. Below that version the count/triples are absent; later native geometry-generation code is not a wire body. Next read one signed four-byte linking word, passed by the native reader to helper **0x1813C4D20**. Its complete identity domain is unresolved.

The beam embeds another object at **+0x1A8**, whose table **0x181F55130** slot **0x181F55158** resolves to **0x180A5C1D0**. This direct body is five four-byte words in order: signed **+0x08/+0x0C**, unsigned **+0x10**, signed **+0x14**, unsigned **+0x18**, relative to that embedded object. It has no tagged object header or stored byte length. At **v >= 810029**, helper **0x181004B00** then reads two signed four-byte longs at relative **+0x08/+0x0C** of the beam's **+0x60** helper. At **v >= 850036**, final signed word **beam +0xA8** follows.

C-725 verifies the six threshold doubles at **0x181F553B8/3C0/3D0/3E0/3E8/3F0** as **810029, 820007, 840014, 840024, 850035, 850036**, respectively, and binds the constructor, storage body, direct helpers and embedded vtable slots to installed byte/decompilation hashes.

**Native scope.** Fresh envelope/nested hash checks and explicit candidate replay complete all **163** exact **.../BeamManager/Beams/<child>/Property** occurrences / **43516 bytes** at exact EOF. Each of the **163 attempts** succeeds, with representative arguments ranging **830036–890620**. The counted arrays contain **501 triples**: 146 records have count 2, eight count 4, four count 19, four count 20 and one count 21. Candidate framing does not establish native runtime version selection, geometric units or every historical profile.

`cosworks_beam_property_layout.py` retains contiguous span/reconstruction hashes, signed/unsigned word views, both raw short values and the triple count. Count/size limits reject negative, truncated and trailing data; no unknown body is skipped. Three focused tests pass over eleven boundary versions, including signed-short/high-bit unsigned-word preservation, exact triple counts, every-byte truncation/trailing data and unsupported kinds. Results and source/test hashes are in `cosworks-beam-property-native-evidence.json`.

This completes retained beam child Property framing alongside the separately validated material carriers. It does not complete BeamFolders, JointGroups, runtime geometric generation, scalar/link meanings, native writer equivalence, all historical/context forms or the full installed-format goal. No target code executes and global semantic byte-coverage credit remains zero.

Evidence: C-725.
