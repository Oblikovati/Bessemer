### 11.329 Template flat patterns and gusset corner surface IDs

*Status: Two feature lists complete; trailing stream data and semantics remain open · Evidence: C-371*

`moTemplateFlatPattern_c` primary RTTI table **0x3DC2D800** in `sldsmmu.dll` binds Serialize +0x28 to **0x3DB6B1C0**. That entry is an exact five-byte E9 jump to **0x3DB5FBC0**, the Serialize target of ordinary `moFlatPattern_c` at primary table **0x3DC2B130**. It adds no separate subclass wire suffix. This establishes reuse of the verified flat-pattern grammar, including its inherited body/bend group and version/helper-dependent payload (11.244), with the saved helper state established by 11.327. It does not establish names or meanings for raw flat-pattern parameters.

The two gusset surface-ID classes use the entity-ID representation dispatcher **0x4BBB79B0**, which reads inherited representation input and calls virtual suffix +0x2A0. Their concrete primary bindings in `sldmodu.dll` are:

| Class | Primary table | Suffix +0x2A0 |
| --- | --- | --- |
| moSMGussetEndCornerFilletSurfIdRep_c | 0x4CF76180 | 0x4BBBED80 |
| moSMGussetLateralCornerFilletSurfIdRep_c | 0x4CF76430 | 0x4BBBED80 |

Suffix **0x4BBBED80** reads three signed i32 values at memory members **+0x38/+0x3C/+0x40**, in that order. The inherited entity-ID representation contains feature and external feature-reference pointers (11.18). These three words are preserved as raw values with unknown meaning; no index domain or geometric interpretation is inferred. A synthetic null-pointer representation followed by -1/23/-41 consumes exactly **16 bytes**, charges **12 unknown bytes**, leaves a sentinel intact and rejects a truncated suffix.

`resolvedfeatures.read_data(..., template_dependencies=True)` installs the template-flat-pattern and two gusset-ID readers only in that separate archive. This is independent of `sheet_metal_references=True`; the current probes select both and derive helper state from the saved CMgr. Global registrations and prior disabled-mode boundaries remain unchanged.

The frame's direct feature list now completes **70 roots** at cursor **333,167**, leaving **11,442 bytes** after the list. The API-gusset list completes **27 roots** at cursor **76,808**, leaving **38,536 bytes**. Across the two lists, completed dependencies contain **two template flat patterns**, **32 end-corner IDs** and **16 lateral-corner IDs**. The feature lists contain **97 roots** in total. These observations complete list framing only; neither resolved stream reaches exact EOF. No trailing data is skipped or credited, and stored document coverage remains unchanged.

`resolved_persisted_context_audit.py` checks all **906 modern part configurations** with saved context and the scoped dependencies. The retained `resolved-persisted-context-audit-0.349.json` reports **830 complete lists / 76 explicit stops / zero context stops**, with **25,533 completed roots**. All **828** formerly complete lists preserve exact endpoints, root counts and unknown-byte totals. Compared with edition 0.277's frontier, only these two files add completed roots: four in the frame and eight in the API gusset. Saved context values are 0 in **812** configurations and 1 in **94**. This broad replay validates the context transfer without assigning credit to the still-open stream tails or semantic fields.

`template_dependencies_evidence.py` verifies both modules' primary bindings, the exact delegation thunk, dispatcher/suffix targets, retained native bodies and source-derived completed list endpoints. `template-dependencies-native-evidence.json` retains bindings, native bytes, existing flat-pattern decompilation identity and root intervals/hashes. `template-dependencies-corpus.json` retains the two completed probes with raw ID fields. Tests check the three-word suffix and sentinel/truncation boundary, complete root/end/tail totals and absence of global template registration. Full resolved-stream, semantic and part/assembly/drawing/project document coverage remains unfinished.

Validation: all **1,089 regression tests pass** (145.441 seconds); specification lint reports zero findings (553 headings, 96 anchors).

Evidence: C-371.
