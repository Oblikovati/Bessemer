### 14.338 Binary Gravity and CentriFugal load records

*Status: all 17 retained native streams complete; physical meanings, runtime conversions and broader contexts remain open · Evidence: C-714*

New installed collection-reader bindings identify **Gravity** at **0x18108BB60** and **CentriFugal** at **0x18108B710**. Each stream starts with a signed four-byte record count. Positive entries allocate their concrete object, invoke the **+0x80** serialization slot on the object's **+8** subobject, and pass the version through the same floating argument path used by the earlier LBC collections. The independent bounded parser rejects negative/over-limit counts rather than emulating a skipped native loop.

Gravity constructor **0x180B5CA20** installs the +8-subobject table **0x181F61138**, whose slot **0x181F611B8** selects serializer **0x180B74AF0**. CentriFugal constructor **0x180B5B4D0** installs table **0x181F614C8**, whose slot **0x181F61548** selects **0x180B73700**. Both serializers first invoke common load base **0x180B754B0**. References, optional bodies and all common-base gates retain the C-700 framing and explicit archive callback dependencies. The following offsets are relative to the serialized **+8 subobject**, not the outer object or wire stream.

**Gravity continuation**, immediately after the common base:

| Order | Stored fields |
|---|---|
| 1 | Four signed four-byte words at **+0x320/+0x324/+0x328/+0x32C** |
| 2 | Three doubles at **+0x330**, then twelve doubles at **+0x348**, each stride eight |
| 3 | Exactly three persisted strings from the runtime string-array storage |
| 4 | At **v >= 800037**, three signed four-byte words at **+0x310/+0x314/+0x318** |

**CentriFugal continuation**, immediately after the common base:

| Order | Stored fields |
|---|---|
| 1 | Seven signed four-byte words: **+0x300/+0x304/+0x308/+0x30C/+0x310/+0x314/+0x390** |
| 2 | Two persisted strings at **+0x2E8/+0x2F0** |
| 3 | Three doubles at **+0x318**, twelve at **+0x330**, then twelve at **+0x398**, each stride eight |
| 4 | At **v >= 800037**, two signed four-byte words at **+0x2F8/+0x2FC** |

The installed storing branches emit their final word groups unconditionally; the version gate describes reading historical forms. The trailing runtime code obtains model/environment data, uses SafeArray access and calls conversion helper **0x180B7BFB0**. Those runtime operations are not additional stream fields, and their unit/physical interpretation remains unassigned. No SafeArray, model API or target conversion code is executed by the parser.

The collection readers also show a legacy runtime repair at **v <= 800027**: use/increment manager member **+0x4F8** and assign the prior value to the outer load object's **+0x108**. This does not introduce an extra wire field. The **800027** and **800037** doubles are verified from installed raw bytes in C-714.

**Native verification.** Fresh extraction from C-590 checks the outer third-party envelope and nested stream hashes for the exact **Cosworks/StudyManager/(StudyNumber-N|SimStudy-N)/LBCManager/(Gravity|CentriFugal)** scope. All **14 Gravity streams / 15024 bytes** and **three CentriFugal streams / 2308 bytes** complete at exact EOF. Every stream contains one completed record. All **17 candidate attempts** succeed; no failed alternative attempt is removed. The selected candidates still do not prove native runtime version dispatch or the per-study-index association.

`cosworks_inertial_layout.py` preserves raw span hashes, contiguous boundaries, complete reconstruction hashes and record ends, using the existing explicit embedded-archive registry. `cosworks_inertial_audit.py` binds collection/constructor/serializer bytes, vtable slots, literal names, threshold constants and decompilations to the installed module hash. C-714 retains the results in `cosworks-inertial-native-evidence.json`. Three focused tests pass for the **800036/800037** boundary and modern profile, every-byte truncation/trailing data, unsupported kind rejection and nonfinite double-bit preservation.

This completes native framing for these **17 occurrences / 17332 bytes**, not physical field semantics, native writer round trips, every historical/context path or the full installed-format schema. No target executable or DLL code runs; global semantic byte-coverage credit remains zero.

Evidence: C-714.

> Current native result: C-716 in 14.340 completes all 68 thermal streams; the C-715 partial checkpoint below is retained for provenance.
