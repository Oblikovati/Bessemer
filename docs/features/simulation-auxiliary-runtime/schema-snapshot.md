### 14.118 Simulation auxiliary fixed UTF-16 name buffer and residual bytes

*Status: Native name-buffer width, bounded conversions and untouched-tail behavior verified; MFC trimming policy remains outside the projection · Evidence: C-494*

The auxiliary rows in 14.103/14.117 contain a **256-byte array of 128 little-endian UTF-16 code units**. Its stored offset is **20** in the 284-byte profile and **24** in the 288-byte profile. The old-profile normalization copies all 256 bytes into normalized offset 24. The two four-byte words following the array remain separately unassigned; the array's post-NUL bytes must not be merged into presumed zero padding.

The producer **cosworks.dll!0x180A58740..0x180A58A39** indexes the normalized row by the element's body-selection ID minus one. For each encountered body transition it obtains a caller CString from a runtime name array, invokes named helper **`STRING2LPUSHORT`** with capacity **128**, then copies the complete **256-byte temporary region** into row offset 24. Its ID/range/list updates are separate operations. The runtime name array's CAD identity provenance is not established merely by the resulting text.

The exported conversion helper **cwcommfuncs.dll!0x180005CA0..0x180005D29** copies at most **capacity minus one** UTF-16 code units, stopping earlier at a source NUL, and writes a final NUL. With capacity 128, this gives at most **127 units plus a terminator**. It does **not** clear the rest of the destination buffer. A unit limit is not a Unicode code-point limit: truncation can split a surrogate pair, and the portable view preserves such code units without replacement or normalization.

The guarded name accessor **cosworks.dll!0x180F727B0..0x180F72878** requires a positive requested row ID no greater than the auxiliary count and equality with that row's stored identifier. It calls **`LPUSHORT2STRING`**, **cwcommfuncs.dll!0x180005BF0..0x180005C9B**, on normalized offset 24 with a limit of **127**. This helper copies until the first NUL or that limit, terminates its temporary CString, then invokes MFC **`TrimRight`** before assigning the caller result. The 128th stored code unit is outside this bounded source scan even if it contains a NUL or another character. The record-based view below does not emulate the accessor's row-selection guard or default result for a failed lookup.

**Stored prefix and native returned name are separate views.** The schema exposes the bounded UTF-16 prefix and retains the entire 256-byte buffer. It does not substitute Python `rstrip`, Unicode whitespace categories or case-folding for MFC's runtime character policy. The final trimmed caller-visible value remains unassigned by this projection. This boundary is material for names ending in spaces or other characters that a runtime might trim; raw stored bytes remain authoritative.

`solidworks/simulationauxiliary.py` exposes `read` for either exact stored row size. Alongside the existing ID/vector-count/range fields it returns the raw name buffer, bounded UTF-16 prefix, decoded surrogate-preserving prefix, source NUL position when encountered inside the scan, all unconsumed name bytes and the two trailing raw words. The remaining prefix bytes stay opaque. `native_returned_name` is unassigned and `native_trim_right_applied` is false.

`project_name_store` implements the inspected bounded copy and terminator write using an **explicitly supplied initial destination image**. It changes only the copied name prefix and terminating code unit; residual bytes and other row fields survive unchanged. Representing that initial buffer within a supplied row is a portable convenience, not proof that the native producer updates a file row in place: its inspected producer uses a temporary buffer and then copies all 256 bytes. The model does not invent an initial temporary tail, execute CString/MFC code or clear historical residual data.

The two composite fixtures each contain prefixes **`Surface-Trim1`** and **`Surface-Extrude2`**; the benchmark contains **`Move Face2`**. Their five arrays total **1,280 bytes**. Prefixes including their first NUL use **146 bytes**; the remaining **1,134 bytes** contain **565 nonzero bytes**. Their origin and meanings are not assigned. Replaying each stored prefix with its existing destination tail reconstructs its full row exactly; this verifies the store-stage projection under supplied initial bytes, rather than proving the historical writer's initial stack contents.

`simulation_aux_name_evidence.py` retains **four complete native producer/accessor/conversion ranges**, decorated conversion exports/imports, module/source hashes, capacity/destination instructions, linked normalization/range evidence, per-buffer hashes and residual-byte counts in `simulation-aux-name-native-evidence.json`. Six focused tests cover both row profiles, field boundaries, residual/source preservation, the ignored 128th unit, 127-unit truncation, a split surrogate pair, embedded NUL, explicit lack of native trimming and size/type guards. No installed target was executed. Remaining scalar words, residual-byte meanings, native trim policy and complete auxiliary/CAD identity semantics remain open; no global/all-installed-project percentage is regraded.

All **1,622 regression tests pass** (191.762 seconds). The native/corpus verifier passes; specification lint reports zero findings (680 headings, 96 anchors).

Evidence: C-494.

### 14.119 Simulation auxiliary pointer-slot normalization and zero-length allocation

*Status: Native workspace slot and ID-gated allocation behavior verified; stored bits are never portable pointers · Evidence: C-495*

The normalized **288-byte auxiliary row** has an eight-byte **runtime vector-buffer pointer slot at offset 8**. This role is established by **cosworks.dll!0x180EA3FA0..0x180EA592A**: for a row whose identifier equals its one-based logical row position, the reader loads the signed vector count at offset 4, requests **four times that count** from an indirect allocator, stores the returned value at row offset 8, and supplies that slot as the destination of the delegated vector read. The producer **0x180A58740..0x180A58A39** explicitly clears the eight-byte slot when initializing its normalized row. The flat GEN writer serializes the complete 288-byte rows, so slot bytes can occur in a stored image; their occurrence does not make them valid addresses for another process or a portable file reference.

The allocation gate is **identifier equality**, not positive vector length. It contains no intervening zero-count rejection: **a selected row with count zero still reaches the indirect allocation call and replaces the pointer slot with the result**, then supplies a zero byte count to the delegated read. The allocator dispatch slot is retained at **0x181F337F8**; its implementation/result is not executed or inferred by this model. Neither a null result nor a non-null zero-size result is assumed. An identifier mismatch skips this allocation/read block and leaves the slot alone. Allocation-failure behavior and delegated I/O error handling remain outside the successful-data projection.

Legacy **284-byte** conversion has a different rule for its four-byte word at offset 8. The native fallback copies only these spans into the existing normalized workspace:

| Stored legacy span | Normalized destination | Role |
|---|---|---|
| 0..8 | 0..8 | Identifier and vector count |
| 12..20 | 16..24 | Element range words |
| 20..276 | 24..280 | Complete name buffer |
| 276..284 | 280..288 | Two trailing raw words |

The stored legacy word **8..12 is not copied**. Normalized workspace bytes **8..16 are not cleared or assigned by this conversion**, and therefore retain their supplied prior bytes until a later operation replaces them. This is observable because the native fallback follows a trial read into the 288-byte workspace. Giving legacy conversion a fabricated zero-initialized slot would change that intermediate state. The discarded legacy word's original historical meaning is not assigned solely from its location. Subsequent selected-row allocation overwrites the normalized slot even when the vector count is zero.

`solidworks/simulationauxruntime.py` exposes `normalize`, `allocation_plan` and `project_pointer_binding`. Legacy normalization requires the caller's exact 288-byte initial workspace and preserves its pointer-slot bytes; modern normalization retains the supplied 288-byte image. The plan reports the ID gate, allocation request and delegated read size, without dereferencing stored bits. Selected negative counts and counts whose four-byte read size exceeds the supported signed-width profile are explicit stops. The binding projection requires an explicitly supplied u64 allocation result for a selected normalized row, including a zero-length row, and returns a memory-only image changing bytes 8..16. It performs no allocation, read or file write. A mismatched identifier preserves the row without requiring an allocation result.

All **five installed rows** match their one-based logical positions and have vector count **zero**. The corpus verifier constructs the four legacy initial workspaces from the exact original 288-byte trial-read slices, then confirms conversion preserves bytes 8..16. It records the nonportable slot bits and the discarded legacy words without using them as pointers. Separately, an explicitly synthetic allocation result **0** demonstrates slot replacement for every selected zero-length row while leaving all other bytes unchanged. This does not claim that historical or current native zero-size allocations actually returned zero. Installed populated positive-length vectors remain unavailable; positive counts and nonzero allocation-result bits are exercised synthetically.

`simulation_aux_runtime_evidence.py` retains **two complete native reader/producer ranges**, exact ID/size/call/store instructions, allocator dispatch-slot bytes, source/module hashes, linked normalization/name evidence, actual trial workspace projections and the supplied-result examples in `simulation-aux-runtime-native-evidence.json`. Six focused tests cover every copied span, preservation of the legacy workspace slot, modern identity conversion, zero-length calls, explicit result requirements, positive read size, untouched bytes, mismatch skipping and safe profile guards. No installed target was executed. The remaining trailing-word meanings, vector element domain, allocator behavior and complete auxiliary semantics remain open; global/all-installed-project percentages are unchanged.

All **1,628 regression tests pass** (218.228 seconds). The native/corpus verifier passes; specification lint reports zero findings (681 headings, 96 anchors).

Evidence: C-495.
