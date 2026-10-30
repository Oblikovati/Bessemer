### 14.114 Simulation family-mask filter and element selection key

*Status: Native mask/unsigned-key comparisons and exclusion direction verified; body-selection query binding follows in 14.115 · Evidence: C-490*

The exported **`ValidMeshElementBasedOnCode`**, **cwgeopost.dll!0x18005C310..0x18005C3B1**, accepts a one-based element number and a 32-bit mask. For a nonzero mask it subtracts one from the element number and accesses the same `CPostElemContainer` record used in 14.113. It returns 1 if any of the following tests succeeds, otherwise 0:

| Mask bit | Required family byte at record offset 20 |
|---|---|
| `0x01` | 4 |
| `0x02` | 8 |
| `0x10` | 8 |
| `0x04` | 2 |

Bits **0x02 and 0x10 are aliases in this helper**: neither has a distinct family-byte condition here. Bit 0x08 and higher bits have no independent accepting branch. An all-bits-set mask accepts families 2, 4 and 8, but does not accept every possible family byte. The helper itself does not test activity, node count or the stored code at offset 26. Zero mask returns false before accessing an element; this must not be generalized to its callers' zero-mask behavior.

In **`DoMidNodeAveragingForSolidMesh`**, **0x180038B90..0x18003986E**, mask zero **bypasses** the helper instead of rejecting the record. A nonzero mask calls the helper and skips a false result. The caller separately requires nonzero activity, family 4 and node count 10 as described in 14.113. Thus the helper's result and the enclosing averaging record gate are different predicates.

The same caller establishes an additional field:

| Record offset | Size | Verified role |
|---|---|---|
| 28 / `0x1C` | 2 | Unsigned little-endian selection key K |

A nonzero signed 32-bit scalar caller key is compared for equality against zero-extended K. A caller key outside **1..65535** cannot match; in particular **-1 does not match stored K = 65535**. Scalar key zero disables this equality selection and permits a separate set path. The bytes represent an externally selected key; 14.115 binds its use as a native body-selection ID, while CAD/assembly ownership mapping remains open.

When the scalar key is zero and the caller's set-selection flag is enabled, the native code searches a valid signed-int ordered tree using K. **Membership rejects the element; absence permits it.** This direction is verified in the native lower-bound/equality branch: a found ordinary node reaches the skip target, whereas a header/no-exact-match path continues. It is an exclusion set, not an inclusion list. A nonzero scalar key bypasses this tree path entirely, so the scalar equality rule takes precedence. The projection assumes a valid supplied set and does not reproduce native tree corruption, allocation or iterator state.

`solidworks/simulationselection.py` exposes `read` for the raw u16 key and family, `valid_for_mask` for the named helper, `solid_averaging_scalar_gate` for the inspected record gates with the set path disabled, and `solid_averaging_exclusion_gate` for the zero-scalar/enabled-set path. Signed or unsigned 32-bit mask inputs preserve native bit patterns. The record-based helpers require caller-selected records; they do not derive element numbers or runtime mask/key/set state. They preserve source bytes and do not perform averaging or file writes.

Across all **25,106 installed elements**, the key field occupies **50,212 bytes**. Each composite fixture contains **2,618 records with K = 1** and **2,614 with K = 2**; all **14,642 benchmark records have K = 1**. Under explicitly supplied masks, each composite accepts all 5,232 records for mask 2 or 16, while the benchmark accepts all 14,642 for mask 1. Mask zero accepts none in the helper. The default scalar averaging record gate, which bypasses a zero mask, accepts the 14,642 benchmark records. Supplying an empty exclusion set preserves that count; supplying a set containing each record's key rejects every otherwise eligible record. These are explicit context projections over stored fields, not recovered historical UI/session selections.

`simulation_selection_evidence.py` retains **two complete native ranges**, export/source/module identities, mask instructions, scalar zero-extension/equality, the exclusion branch direction, the linked accessor/topology evidence hash and per-file key/mask histograms in `simulation-selection-native-evidence.json`. Six focused tests cover mask aliases, ignored bits, signed-mask patterns, zero-mask helper/caller distinction, unsigned versus signed key comparison, activity/count separation, exclusion direction, source preservation and profile guards. This refines another field role without assigning a complete engineering domain or regrading global/all-installed-project percentages. No installed target was executed. Other selection callers, key ownership and complete result-processing context remain open.

All **1,604 regression tests pass** (254.914 seconds). The native/corpus verifier passes; specification lint reports zero findings (676 headings, 96 anchors).

Evidence: C-490.

### 14.115 Simulation body-selection ID and node-mark query

*Status: Named native body-selection query, mode/callback rules and installed node associations verified; CAD-body identity mapping remains open · Evidence: C-491*

The exported **`GetNodesByBodyId`**, **cwgeopost.dll!0x18002BE50..0x18002C1AA**, compares its supplied body-ID argument against the **unsigned 16-bit word at element offset 28 / 0x1C**. This is the same selection key used in 14.114. It establishes the field's **native body-selection ID** role in postprocessing. It does not yet identify that value with a CAD-document body index, assembly component ID, material ID or persistent geometry identifier.

The query writes a caller-provided node-mark array and a separate type flag. **Its function return value is always zero**, including when nodes were marked. Existing marks are not cleared. Unvisited nodes retain prior caller contents; a portable projection without supplied initial values must leave them unknown rather than invent zeroes. Repeated node references can write one destination multiple times.

The caller's signed mode controls traversal:

| Mode | Traversal and element filtering | Initial type flag |
|---|---|---|
| 1 | Visit every stored node; mark each with nonzero activity directly. No element traversal or callback. | 2 |
| 2, 3, 4, 5 | Visit the supplied inclusive zero-based element range; require nonzero element activity and matching body-selection ID. No family restriction. | Mode 2/5: 2; mode 3: -1; mode 4: 4 |
| Other modes | Visit all elements; require nonzero activity, family byte 4 and matching body-selection ID. | Mode 0: 1; mode 6: 8; otherwise -1 |

For element-based paths, the query visits the unsigned node count at offset 23 and signed node IDs beginning at offset 96. ID zero is skipped. A nonzero ID accesses node **ID minus one**, and only a node with nonzero activity is marked. The written value is the signed body-ID argument when it is greater than 1, otherwise **1**. The element comparison still uses the zero-extended u16 field against the caller's full 32-bit bit pattern: a negative caller ID does not match stored 65535. Mode 1 instead writes the caller body-ID bits directly, even zero or a negative bit pattern, without this clamp. Native out-of-range/negative node accesses are not safe behavior to reproduce; the portable profile rejects them.

Except in mode 1, the routine initializes a **VT_I2** variant with the low sixteen bits of the body ID and calls the imported postprocessing callback table with operation **2**. A supplied callback result other than signed-i16 **-1** overrides the type flag to **16** and resolves it. A -1 result retains the initial flag and its resolution state. The callback's implementation and live result are external context; the projection requires that result explicitly and does not execute a callback.

If a range-mode flag remains unresolved, the **first successful active-node mark** resolves it from that element's family byte: family **4 → 1**, **8 or 3 → 2**, **2 → 4**. Another family leaves flag -1 but still resolves the state, preventing a later element from assigning a new flag. This inference belongs only to the range branch. An unknown mode using the fallback family-4 scan does not perform it. The numeric flags are not equated with a complete API enum.

`solidworks/simulationbodynodes.py` exposes `project`, requiring whole node/element blocks, a body ID, mode and—except mode 1—an explicitly supplied signed-i16 callback result. Range modes also require safe inclusive bounds. Optional initial marks preserve prior values; otherwise untouched entries are `None`. The model returns marks, ordered write visits, type flag and zero return value. It does not derive callback/session state, run native code or modify the mesh. `simulationselection.read` now adds `body_selection_id` and records this native query domain while retaining `selection_key` and raw source bytes.

The installed corpus supports five supplied-context body queries. With mode **3**, the full element range and callback result **-1**, each composite fixture marks **5,333 nodes for body ID 1** and **5,296 for body ID 2**, returning type flag **2**. Their union covers all **10,553 nodes**, with **76 shared nodes** in each fixture. The benchmark body ID 1 marks all **24,140 nodes**, with type flag **1**. Across these queries there are **209,204 write visits** and **45,398 per-query marked nodes**, including overlap. Under supplied mode 0, composite selections mark no nodes because of the fallback family-4 filter, whereas the benchmark still marks 24,140. Supplied mode 1 marks every active stored node and bypasses callback/element processing. These are explicit projections, not observed historical runtime selections.

`simulation_body_nodes_evidence.py` retains the complete named query range, decorated export, callback import, native u16/count/family instructions, source/module hashes, linked accessor evidence and per-query node-mark hashes/counts in `simulation-body-nodes-native-evidence.json`. Six focused tests cover mode-1 bypass/direct bits, inclusive range/family differences, untouched marks and duplicate writes, callback precedence, first-family resolution, fallback behavior, zero-ID clamping, unsigned key comparison and bounds/context rejection. No installed native code or callback was executed. This refines already framed field/reference roles without regrading global/all-installed-project coverage. CAD-body mapping, other callback operations and broader runtime identity semantics remain open.

All **1,610 regression tests pass** (200.581 seconds). The native/corpus verifier passes; specification lint reports zero findings (677 headings, 96 anchors).

Evidence: C-491.
