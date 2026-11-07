### 14.292 Pressure load-base prefix and versioned optional continuations

*Status: base scalar/string order and gates recovered; reference records, virtual bodies and terminal table remain dependencies · Evidence: C-668*

**Binding and coordinate system.** The first call in 14.291 reaches **0x180B754B0–0x180B75D15**. C-668 binds its **479** decoded instructions, function-span and decompilation hashes, and eleven raw double version constants to the installed `cosworks.dll`. All offsets below are relative to the same **+0x08 subobject** used by the derived Pressure serializer. At entry it calls `CSWPersistReferenceContainer.Serialization` before the fields below. Temporary dispatch/model-context assignments around that call are runtime state, not extra identified wire bytes. Populated reference-container records remain dependent on 14.281–14.282's nested archive grammar.

**Unconditional read prefix after reference container.** The observed consumption order is:

| Order | Representation / runtime destination |
|---|---|
| 1 | Four-byte integer **+0x198**. |
| 2 | Four persisted CStrings: **+0x78**, temporary A, temporary B, **+0x60**. |
| 3 | Two integers **+0x68/+0x58**. |
| 4 | Double **+0xF8**, integer **+0xEC**, double **+0xE0**, unsigned integer read into a local, then integers **+0x110/+0xA8/+0x1E0**. |
| 5 | If version **> 800027**, integer **+0x100**. |
| 6 | Exactly six doubles beginning **+0x118**, then exactly six persisted CStrings in the runtime array addressed through **+0x88**. |

The local unsigned integer is later assigned to **+0xE8**. Copying **+0xEC** to **+0xF0**, wrapper/name checks and initialization of a six-element runtime string array add no wire bytes. The read path can signal an exception when a virtual condition is true and the **+0x78** string is empty; a byte-framing parser does not reproduce that domain check.

**Gated continuation after six strings.** Read these stages in table order, preserving strict versus inclusive comparisons:

| Version condition | Consumed continuation |
|---|---|
| **> 800001** | Integer **+0x218**, four-byte presence value; when nonzero, a virtual body through complete-object slot **+0x50** with destination **+0x210**. Then integer **+0x228**, presence value, and analogous body to **+0x220**. |
| **> 800002** | Integer **+0xAC**. |
| **> 800037** | Integer **+0x48**. |
| **>= 800066** | Integer **+0x230**. |
| **> 810001**, and **>= 870006** | Integer **+0xC8**. |
| **> 810001**, and **>= 870010** | Integer **+0x170**. |
| **> 810001**, and **>= 880008** | Double **+0x108**. |
| **> 810001**, and **>= 890009** | Three integers **+0x148/+0x14C/+0x150**, then CStrings **+0x158/+0x160**. |
| Within the preceding **>= 890009** branch, **>= 890011** | Integer **+0x2E0**, then signed count for the terminal table described below. |

For version **<= 810001**, the two temporary prefix strings feed runtime reconstruction helpers. For versions below the **890009** field group, the last three integer/two-string destinations are reset in memory, without consuming replacement bytes. The two optional virtual body layouts are unresolved; their presence values are four-byte integers, not one-byte booleans.

**Terminal table boundary.** After reading **+0x2E0** and its count, the installed path checks whether the mode is in **[-100,100]** using an unsigned range comparison. In range, it clamps the count to **[0,50]**, allocates **200-byte** objects, initializes them with **0x180D49150**, and calls record helper **0x180D59E50** for each clamped entry, appending to owner list **+0x2A8**. Out of range, it resets the mode and reads no table records here. This is native consumption behavior, not proof that excess declared entries can be discarded losslessly. Until record framing and native fixtures are validated, a bounded independent parser must stop or reject rather than silently infer a safe suffix boundary from the clamp.

**Current writer distinction.** The writer emits the scalar/string fields in their current order without historical read gates, including the two optional-body headers, final field groups and terminal count. Its two temporary prefix strings start empty. Present optional bodies use complete-object virtual slot **+0x48**. The terminal list is omitted with a zero count when runtime **+0x2C0 < 2**; otherwise its records use **0x180D59E50**. This does not establish historical writer layouts, native round trips or complete Pressure byte coverage. Reference records, optional bodies, terminal records, storage ancestry and field meanings remain explicit dependencies. No target execution or global semantic byte credit occurs.

Evidence: C-668.
