### 14.341 Binary BaseExcitation collection and historical tail gate

*Status: installed binding and sole retained native stream complete; older gate branch is synthetic/static, physical meanings remain open · Evidence: C-717*

Collection reader **0x18108B5E0** opens literal **BaseExcitation** at **0x18201A220**, reads a signed four-byte record count, allocates each concrete object through constructor **0x180B5AC00**, and invokes slot **+0x80** of its **+8 subobject**. Constructor table **0x181F5F418** selects serializer **0x180B72EB0** through slot **0x181F5F498**. The reader preserves incoming **XMM2** in **XMM6** and restores it before item dispatch; the decompiler's integer-looking third argument is not a different wire/version representation.

The concrete serializer first invokes common load base **0x180B754B0**. Its continuation uses the following order, with runtime offsets relative to the serialized +8 subobject:

| Order | Read fields |
|---|---|
| 1 | Five signed four-byte words **+0x320/+0x324/+0x328/+0x354/+0x358**, then one unsigned four-byte word **+0x3C0** |
| 2 | Three doubles **+0x330..+0x340**, stride eight |
| 3 | Three signed four-byte words **+0x348/+0x34C/+0x350** |
| 4 | Twelve doubles **+0x360..+0x3B8**, stride eight |
| 5 | Three persisted strings from the runtime string array; a fourth string only at **v >= 820012** |
| 6 | Three signed four-byte words **+0x310/+0x314/+0x318** |
| 7 | At **v >= 820012**, a fourth word **+0x31C**, then signed word **+0x3C4**, double **+0x3C8**, signed word **+0x3D0** |

The threshold double at **0x181F61F90** is verified as **820012**. The current storing branch emits all four strings, all four tail words and the final mixed scalar group. The older read branch has three strings/three words and omits that last group. The wire string count is determined by the version gate, not by the runtime array's allocated capacity: the reader initializes capacity for four entries even when loading the older three-string form.

After consuming the body, the native code accesses model/environment values and SafeArrays and calls conversion helper **0x180B7BFB0**. Those calls do not add persistence fields. Their physical/unit transformations and failure paths remain outside the independent parser's claims. No such runtime operation is executed during validation.

**Native scope.** The C-590 corpus has one matching **Cosworks/StudyManager/StudyNumber-1/LBCManager/BaseExcitation** stream, in `Dynamics/circuit_board.sldasm`. Fresh envelope/nested hash checks and exact parser replay complete all **596 bytes** under its retained candidate **870022**. It contains one record with common subtype word **15**. The derived continuation begins at stream offset **384** and ends at **596**, a **212-byte** span. Its five signed/one unsigned initial values are **2, -1, 3, 1, 0, 2**. These values are retained raw observations, not assigned enum or physical meanings. Candidate success does not prove native runtime version dispatch.

`cosworks_base_excitation_layout.py` uses the bounded common-base/reference/archive machinery and retains contiguous span hashes, complete reconstruction hash and the unsigned field separately from signed views. `cosworks_base_excitation_audit.py` verifies the installed collection/constructor/serializer bytes, literal, vtable slot, threshold constants and source hashes, retaining C-717 in `cosworks-base-excitation-native-evidence.json`.

Three focused tests pass for **820011/820012** and a modern version, three/four-string selection, high-bit unsigned-word preservation, every-byte truncation/trailing bytes and unsupported-kind rejection. The older branch is not represented by a native fixture here. Complete framing of this single occurrence does not establish every BaseExcitation form, field semantics, native writer equivalence or full installed-format coverage. No target code executes; global semantic byte-coverage credit remains zero.

Evidence: C-717.
