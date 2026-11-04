# Vector-direction model

`SimulationComponentVectorInputs` captures the four signed provider arguments in
native order: analysis, plot type, component and fourth control word. Its
`component_vector()` method returns exact raw words, output origin and selected
table RVA. An out-of-range component can use a defined fallback while retaining
the selected RVA. An all-zero output is a completed model result.

`SimulationVectorDirectionInputs` adds word570 and explicit loaded CString
presence. When the selector is nonnegative and the loaded string is nonempty,
the local branch returns the exact words for `(1,1,1,0)` before provider dispatch.
The parent projection requires the caller to supply that string state; it does
not infer runtime CString assignment from persisted text bytes.

The provider model covers all 23 C-879 tables, 262 entries and the special/default
branches documented in C-880. Production table constants come from the retained
static report. Verification checks actual table bytes and target addresses,
contiguous constant-write instructions through RET, all four reconstructed words
and the full named method/import/export bindings. No native code executes.

Output words remain authoritative. The fourth word's physical role, engineering
labels for numeric controls, normalization and runtime execution equivalence are
unproven. Native composition retains the original ordinary-wire-text assumption:
the 224 observed text payloads contain no embedded NUL code units. That condition
does not establish runtime state for arbitrary or reused objects.

Independent fixtures cover 106,560 provider tuples, 3,488 local/provider tuples,
all original native inputs, and both explicit string states for all complete
owned/native parent contexts. These views add no stored-byte coverage.

The initial reference attempt lacked the provider helper's JSON dependency in
the temporary snapshot. The corrected harness copies the actual hash-bound data
and checks its immutability before and after replay. It preserves the original
reader and all static checks.

Seven Rust tests, all-target Clippy and all 121 integrated reference checks passed.
Eight-worker output matched serial replay. Two fresh processes matched all
110,496 vector results, including provenance and every output word, against
Source byte for byte. [Verification](vector-operation-verification.json) records
the precise revisions, native bindings, fixtures, initial harness correction and
remaining publication gates.
