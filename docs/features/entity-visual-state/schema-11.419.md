### 11.419 Stored entity visibility and printability bits

*Status: typed native names and corpus replay; remaining state bits unresolved.*

Read-only decompilation of the existing analyzed sldmodu project supplies the method identities missing in 11.417–11.418. Accessor `0x4C160910` is **`moEntVisProp_c::isPrintable`**, returning runtime state +4 masked by 2. Accessor `0x4C160920` is **`moEntVisProp_c::isVisible`**, returning 1 when runtime dword +18 is nonzero, otherwise state +4 masked by 1. Their retained native bytes already match C-1874; the newly recovered typed names establish the semantic binding.

The offline visual-property reader now exposes `stored_visible = bool(unknown_4 & 1)` and `stored_printable = bool(unknown_4 & 2)` when the stored state byte is present. These describe stored bits. `stored_visible` does not simulate the runtime override or claim effective visibility after all application state is applied. No aliases are synthesized for historical versions omitting the byte. Preserve `unknown_4` unchanged, including its high six bits. Because those bits remain unresolved and accounting is byte-based, the whole state byte retains its unknown charge; no global coverage gain is claimed.

All **906** retained modern part resolved streams replay with unchanged hashes, exact EOF and unknown totals against C-1852. The **127,773** captured visual-property bodies store state 1 in **112,268** cases and state 3 in **15,505** cases: stored visibility is set throughout this scope, while stored printability differs. These frequencies are not domain restrictions. Three visual-field tests pass, including all 256 possible stored state bytes with raw-value, framing and accounting preservation checks.

Evidence: **[C-1876]** `part-entity-visual-named-state-evidence.json` / `part_entity_visual_named_state_audit.py` retain native-packet/module hashes, typed decompilations, read-only Ghidra run provenance, per-stream comparisons, state frequencies, decoder/test hashes and test results. Target CAD code was not executed.
