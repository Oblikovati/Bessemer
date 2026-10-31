### 14.319 Embedded solid wrappers/components and version-4281 endpoints

*Status: supported solid/endpoint branches validated; 110 native Force streams reconstruct exactly, two remain open · Evidence: C-695*

**Solid wrapper.** Installed model-module RTTI binds `moSolidRef_w` serializer to **0x4B90DED0**. Fresh static decompilation covers **0x4B90DED0–0x4B90E1B3** and its base helper **0x4B90DB10–0x4B90DBAA**. The base restores the common historical wrapper, then an unsigned four-byte word at object **+0xA8** only when archive version is greater than **0x1BBD**. The outer serializer next restores a signed four-byte geometry-presence value. Zero returns immediately. Nonzero selects `mo_loadBodyFromArchive` followed by a serialized attribute list; this branch remains an explicit stop in `cosworks_embedded_solid.py`, rather than receiving a guessed length.

**Nested solid component.** `moCompSolidBody_c` primary slot points to **0x4BBBA2D0**, whose five-byte relative jump resolves to **0x4BBB7B50**. C-695 checks both the installed slot/thunk bytes and the implementation fragment/decompilation **0x4BBB7B50–0x4BBB7E77**. The body restores the common component-reference base, then these fields:

| Version condition | Wire fields after the common component base |
|---|---|
| > **0x49A** | Tagged `moFaceRef_c` pointer |
| > **0x864** | Signed four-byte array count, followed by that many four-byte words read through the signed `long` overload |
| > **0x94D** | Signed four-byte word +0x108 |
| >= **0x1040** | Signed four-byte words +0x104, then +0x100 |

The explicit reader supports the existing **4307–19000** archive range, in which all four gates are satisfied. It bounds the array count and preserves values; negative/impossible counts fail. The base's unsupported old-reference, reference-info, transform and other optional bodies remain stops. This class calls the common component-reference serializer directly; it does not consume the boundary-component alias suffix used by face/edge/vertex components.

**Specific older endpoint.** The Force stream in the version-4281 archive path declares `moEndPointRef_w`. Its installed serializer **0x4C27D8F0** and point-base helper **0x4C281C60** establish the layout for this version: common historical wrapper, direct point (**24 raw bytes**), parameter double (**8 raw bytes**), signed four-byte final word. Version **4281** satisfies the point gate **>0x78**, double-parameter gate **>=0x1FB**, and final-word gate **>0x726**. An explicit `endpoint_4281` reader supplies this extension only to the callback accepting exactly **4281**. The default archive and endpoint-reader ranges remain unchanged.

**Native completion.** `cosworks_force_solid_audit.py` freshly extracts/hash-checks all **112 Force streams / 278085 bytes**, preserving the same candidate-version inputs as C-694. Exact EOF/reconstruction succeeds for **110 distinct streams / 265168 bytes**, adding **three streams / 11814 bytes** while retaining all 107 previous completions. There are **110** successful candidate attempts; success does not establish runtime dispatch.

One successful candidate per complete stream contains **two solid-component bodies**, **two solid-wrapper bodies** and **one version-4281 endpoint body**. The solid archives use versions **5000/6247**: each component array has one word (**101** or **102**) followed by three zero words; each wrapper has geometry presence **zero** and no version-gated +0xA8 word. The endpoint body range **45–768** is **723 bytes**, with zero point/parameter bits and final word **zero**. C-695 retains class/version, range, SHA-256 and decoded fields for each event. Nested spans overlap and are not independently added to byte coverage.

The remaining **two streams / 12917 bytes** have more specific stops: `tutorial/api/loop.sldprt` (**10470 bytes**) reaches Force optional helper **0x181503180**; `tutorial/api/weldment_box3.sldprt` (**2447 bytes**) reaches `moCompSketchEntHandle_c`. Incompatible stored-version candidate failures remain separately retained.

**Validation and limits.** All **36** focused archive/load tests pass. New checks cover solid wrappers around the **0x1BBD/0x1BBE** boundary, unsigned **0xFFFFFFFF** preservation, component-array framing, the explicit older endpoint, exact trailing-byte boundaries, every-byte truncation, negative array counts, default callback rejection of version **4281**, and the nonzero solid-geometry stop. C-695 binds model-module/RTTI/serializer/helper/thunk hashes, Force predecessor evidence and reader/audit/test sources. Populated solid geometry/attributes, unsupported optional records, physical meanings, writer round trips and full installed-format coverage remain partial. No target execution or global semantic byte credit occurs.

Evidence: C-695.
