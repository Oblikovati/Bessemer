### 14.320 Force optional grid, sketch-component handles and complete native Force-family framing

*Status: all 112 native Force streams reconstruct exactly under retained candidates; wider coverage and semantics remain partial · Evidence: C-696*

**Optional Force body.** Installed helper **0x181503180–0x18150346C** supplies the mode-selected body previously stopped in 14.318. It restores two signed four-byte count-like words **r/c** at object +0x08/+0x0C, then three doubles at +0x10. The reader labels these counts rows/columns without assigning physical meanings. Its nested loops consume a four-byte float at column ordinal **1**, one at ordinal **2** when present, and one at ordinal **3** when present. Higher column ordinals consume no float. The exact float count is therefore:

`max(r - 1, 0) * min(max(c - 1, 0), 3)`.

The float payload is followed by three signed four-byte words (+0xA0/+0xA4/+0xA8), six doubles (+0xB0), three doubles (+0xE0), and one signed four-byte word (+0xF8). Version **>=830005** adds a double +0x100; **>=830006** adds a double +0x108; **>=830009** adds two signed four-byte words +0x110/+0x114. All three gates are satisfied when this helper is called from the currently supported Force mode branch, introduced at version **850004**. C-696 checks the installed constants at **0x181F61FA0**, **0x182069C18** and **0x182069C20**.

`cosworks_force_optional_layout.py` extends the prior Force framing parser while retaining the earlier parser as a reproducible checkpoint. It bounds **r** to **0–2498** and **c** to **0–10000** and preserves float/double bytes. These are explicit supported-reader limits; larger native allocation/early-return states are not generalized. Populated terminal records at **0x180B83BC0** remain disabled. The grid formula does not infer a dense r-by-c matrix or physical coordinate meanings.

**Sketch-entity component.** `moCompSketchEntHandle_c` installed primary serializer slot binds **0x4BBBB9C0**. For archive versions **4307–19000**, it restores feature-component base **0x4BBB9A60** (including its supported common component and direct value/time fields), then a tagged `sgEntHandle` pointer, followed by an unsigned four-byte word at +0x100. The installed older branches below **0x2D1** and the final-word gate **>0xA95** remain independently visible in static evidence; the enabled range satisfies the latter and excludes the former. Existing unsupported component-base bodies remain stops.

The native pointer declares **`sgLineHandle`**. Its installed slot binds shared `sgEntHandle::Serialize` **0x4C591ED0**. In the explicit embedded ordinary-stream context and supported archive range, restore an unsigned two-byte entity index; sentinel **0x777F** selects a following signed four-byte index. A signed four-byte reference identifier follows. A further signed four-byte `DimOnCM` word is present only at archive version **>=0x17D7**. Earlier index-sign conversions, DB-stream key encodings and other archive contexts are not generalized. The existing ordinary sketch reader is not weakened to bypass its context guards.

`cosworks_embedded_component_sketch.py` supplies the component and line-handle readers only through the explicit extension registry. C-696 checks model-module SHA-256, both RTTI/serializer slots, component-base helper bytes and retained decompilation hashes. Native archive version **5188** is below **0x17D7**, so its line handle consumes **six bytes**, not ten: entity index **1**, reference identifier **-1**, no dimension word. The containing component consumes **102 bytes** and restores final unsigned word **2**.

**Native optional bodies.** The Force stream in `tutorial/api/loop.sldprt`, candidate version **850024**, contains three successfully framed optional bodies:

| Byte range | r | c | Four-byte float values | Total body bytes |
|---|---|---|---|---|
| **3145–3329** | 6 | 3 | 10 | 184 |
| **5901–6069** | 4 | 3 | 6 | 168 |
| **9926–10134** | 9 | 3 | 16 | 208 |

Each body has its own retained SHA-256 and bounded labeled spans. The sketch/line-handle path completes `tutorial/api/weldment_box3.sldprt`; its handle range is **1327–1333**, nested inside component range **1235–1337**. Nested ranges overlap and are not independently added as byte credit.

**Complete observed family framing.** Fresh extraction/hash verification across **112 Force streams / 278085 bytes**, with all retained stored candidates, now yields exact EOF and reconstruction hash for **every distinct stream**. All 110 C-695 completions remain complete; the extension adds **two streams / 12917 bytes**. There are **112** successful candidate attempts. Incompatible coexisting candidate failures remain retained, and candidate success is not proof of runtime version dispatch. Native event assertions also revalidate **six** solid components, **six** solid wrappers and the previously completed older endpoint.

**Validation and limits.** All **40** focused archive/load tests pass. Four new tests cover zero/one/multiple grid dimensions; column ordinals above three; negative/over-limit counts; exact EOF and every-byte truncation; the two-byte versus extended handle index; both sides of the **0x17D7** gate; and the component's tagged handle/unsigned suffix. Native grid shapes/body lengths and native handle/component fields are asserted in C-696, which retains reader/parser/test/static/predecessor hashes.

This completes observed framing only for `Cosworks/StudyManager/StudyNumber-[0-9]+/LBCManager/Force` in the corpus. Unobserved optional/terminal bodies, other archive contexts, physical scalar meanings, runtime version selection, writer round trips and the full installed SolidWorks format scope remain incomplete. No target execution or global semantic byte credit occurs.

Evidence: C-696.
