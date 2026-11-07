### 11.448 Parameter roles in inherited node flag words

*Status: named native getter/setter bindings, class-scoped decoder projections and retained part replay.*

Additional roles in the inherited `moNode_c` flags are established for **moParameter_c and its parameter subclasses** (the global-variable row is added by 11.572 / C-2191):

| Stored word | Mask | Parameter role | Native binding |
|---|---|---|---|
| Runtime +28 (`flags_28`) | 0x10000000 | Read-only state | Exported `getReadOnlyState` at 0x4B2DC710 returns bit 28; `setReadOnlyState` at 0x4C1ABDE0 uses the same mask with paired node set/clear helpers. |
| Runtime +2C (`flags_2c`) | 0x00000001 | Shared-thickness state | Exported `setSharedThickness` at 0x4C1ABE80 selects the paired second-word set/clear helpers with mask 1. |
| Runtime +28 (`flags_28`) | 0x80000000 | Global-variable parameter state | Exported `isGlobalVarParam` at 0x4C1ABCB0 returns bit 31 directly; C-2191. |

The generic node set/clear helpers bind the first pair to runtime +28 and the second pair to +2C. `setReadOnlyState` also manages an auxiliary runtime pointer at +40; that allocation and its read-only-owner information are not simulated or added to this lightweight wire layout. These roles do not assign the same meanings to every other node class or prove an effective runtime sharing decision.

`modelnode.parameter_lightweight` reads the existing inherited node layout, exposing **`stored_read_only`** and, when the second word is present, **`stored_shared_thickness`**. Density, length, angle and integer parameter readers and the attribute parameter/scaler/string readers use this class-scoped helper. Generic `modelnode.lightweight` remains free of parameter-only interpretations. The +2C word and its projection remain absent at versions through **0x143A**; no extra bytes are read at that gate. Both complete raw flag words, signed update state, all other bits and existing unknown charges are preserved.

The new synthetic tests vary both named bits independently with unrelated high bits, check the +2C version boundary and raw-word preservation, verify generic nodes do not gain parameter-only roles, and exercise the floating, integer, attribute, scaler and string reader paths through their next-stream marker. The retained replay checks all **906** modern part streams against C-1959 for source hashes, exact EOF and unchanged per-stream unknown totals. It observes **39,487** parameter records with neither new bit set; the set-bit cases are covered synthetically and bound by native setters, not inferred from file frequency. All **118** related tests pass normally. The remaining scoped unknown total stays **16,143,727 decoded bytes**; neither partially understood flag word is regraded.

The broader related suite exposed a stale historical part-lightweight expected unknown total. **[C-1963]** verifies that disabling or enabling the new parameter projections produces the identical tuple `(906, 4590047, 2155929, 97)`. Only the old expected unknown-total constant 2290189 changes to 2155929, with AST proof. The stream count, decoded size and class count do not change; this inherited checkpoint repair adds no new semantic credit.

Evidence: **[C-1961]** `part-parameter-node-flags-native-evidence.json` / `part_parameter_node_flags_native_audit.py` retain typed exports, complete relevant native functions, mask/branch argument bindings, module hashes and read-only provenance. **[C-1962]** `part-parameter-node-flags-replay-evidence.json` / `part_parameter_node_flags_replay_audit.py` retain source-bound parameter observations, per-stream EOF/accounting checks, source hashes and normal test results. **[C-1963]** `part-parameter-node-flags-checkpoint-evidence.json` / `part_parameter_node_flags_checkpoint_audit.py` retain disabled/enabled comparisons, the pre-migration test snapshot and the single-constant AST proof. Target CAD code was not executed. Part-document work remains ahead of assembly fields; these partial roles do not establish completion of the part schema.
