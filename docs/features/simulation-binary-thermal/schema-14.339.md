### 14.339 Binary LBC thermal load families

*Status: five installed binary layouts recovered; 60 of 68 native streams complete, eight explicit dependency/version stops remain · Evidence: C-715*

These binary **LBCManager** records are distinct from the legacy **ParametricData** text/thermal formats in 14.286–14.288. Each collection begins with a signed four-byte record count, then records using common load base **0x180B754B0**, including its explicit reference/archive dependencies and version gates. Collection readers allocate the following concrete types and dispatch slot **+0x80** of their **+8 subobjects**. C-715 binds installed literal bytes, constructor/collection/serializer spans, vtable slots and threshold doubles to the module hash.

| Family | Collection reader | Constructor | Serializer |
|---|---|---|---|
| Temperature | **0x18108D270** | **0x180B5FE30** | **0x180B76920** |
| Convection | **0x18108B880** | **0x180B5B8F0** | **0x180B73AD0** |
| HeatFlux | **0x18108BCD0** | **0x180B5CE80** | **0x180B74EA0** |
| HeatPower | **0x18108BE40** | **0x180B5D3E0** | **0x180B750C0** |
| Radiation | **0x18108CD50** | **0x180B5F300** | **0x180B75F40** |

**Tail grammar.** All offsets below are relative to the serialized +8 subobject, not the outer object or wire position. Words are signed four-byte values; doubles occupy eight raw bytes and are not normalized by the independent parser. Fields appear after the complete common base, in the listed order.

- **Temperature:** two words **+0x2F8/+0x2FC**, double **+0x300**, string **+0x2E8**, word **+0x24**. At **v >= 890501**, read selector **+0x2F0**; only selector exactly **1** adds a signed four-byte count and that many pairs of four-byte words. Other selector values add no pair table.
- **Convection:** two words **+0x300/+0x304**, doubles **+0x308/+0x310**, strings **+0x2E8/+0x2F0**. At **v > 800001**, read word **+0x320** and a four-byte presence value; nonzero presence delegates a body through the outer object's slot **+0x50**. At **v >= 890501**, read final word **+0x2F8**.
- **HeatFlux:** words **+0x318/+0x31C**, double **+0x320**, word **+0x328**; doubles **+0x340/+0x330**, word **+0x338**; three strings **+0x2E8/+0x2F0/+0x2F8**. At **v >= 800037**, read word **+0x308**. At **v >= 890501**, read two words **+0x30C/+0x310**.
- **HeatPower:** words **+0x310/+0x314**, double **+0x318**, word **+0x320**; doubles **+0x338/+0x328**, word **+0x330**; three strings **+0x2E8/+0x2F0/+0x2F8**, then word **+0x24**. At **v >= 800037**, read word **+0x308**; at **v >= 830027**, word **+0x344**. At **v >= 890501**, read selector **+0x30C**; exactly **1** adds a count and four-byte-word pairs as for Temperature.
- **Radiation:** four words **+0x308/+0x30C/+0x330/+0x334**; four doubles beginning at **+0x310**, stride eight; three strings **+0x2E8/+0x2F0/+0x2F8**. At **v >= 890501**, read final word **+0x300**.

Convection's optional-reader slot at **0x181F60800** resolves to **0x180B72100**, the already recovered shared optional-body helper. The parser uses that exact helper grammar, including its bounded text/header/conditional arrays, rather than skipping to the next field. Its object-state meanings remain unresolved. Temperature/HeatPower pair tables retain order and repeated raw pairs; native tree insertion/deduplication is not used to change the wire record. Nonpositive or over-limit counts fail in the independent decoder instead of emulating native skipped loops.

The storing branches and historical read gates differ in places: several final words/selectors are emitted by the current writer even when the reader omits them for an older version. Runtime assignments to fields behind **+0x208**, absent-field zeroing and other post-read state updates do not add bytes. These observations do not assign physical units or complete every runtime conversion.

**Native replay.** Fresh C-590 extraction rechecks outer envelope and nested stream hashes for **Cosworks/StudyManager/(StudyNumber-N|SimStudy-N)/LBCManager/(Temperature|Convection|HeatFlux|HeatPower|Radiation)**. The inventory contains **68 streams / 267820 bytes**:

| Family | Complete / inventoried streams | Complete bytes |
|---|---:|---:|
| Temperature | **34 / 39** | **164895** |
| Convection | **13 / 13** | **54408** |
| HeatFlux | **3 / 3** | **4415** |
| HeatPower | **5 / 8** | **8531** |
| Radiation | **5 / 5** | **11327** |

All **60 completed streams / 243576 bytes** reach exact EOF with contiguous hashed spans and reconstruction hashes. The **eight remaining streams / 24244 bytes** are five Temperature and three HeatPower records. Four furthest candidate stops encounter unsupported embedded **moCompXFormWrapper_c** objects; four stop in the legacy reference-name framing under their retained candidate arguments. No bytes are skipped, no unstored version is substituted, and those failures are not called native corruption. Candidate version success still does not prove runtime dispatch or study-index identity.

`cosworks_binary_thermal_layout.py` implements the bounded grammar with the existing explicit archive registry; `cosworks_binary_thermal_audit.py` retains all candidate results and installed bindings in `cosworks-binary-thermal-native-evidence.json`. Four focused tests pass across five kinds/eight version points, exact selector **1** versus other values, populated shared Convection optional state, every-byte truncation/trailing bytes and unsupported kind rejection. This adds native framing, not full field semantics, native writer equivalence, exhaustive historical/context coverage or the complete installed-format denominator. No target DLL code executes and global semantic coverage credit remains zero.

Evidence: C-715.
