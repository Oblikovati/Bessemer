### 14.366 Complete retained Shells Property framing with inline Material boundaries

*Status: all 234 retained Shells Property streams complete; MidSurfShells and unrepresented delegated profiles remain open · Evidence: C-744*

`cosworks_shell_property_layout.py` implements the constructor-bound Shells layout in 14.365 with the established versioned reference-container grammar. The C-744 audit freshly rechecks native function hashes and all static version constants from C-743, source envelope hashes, and nested Property payload hashes. It preserves contiguous raw spans, archive boundaries, supplied integer versions, and exact EOF; unknown archive classes, nonzero virtual +50 bodies, populated final base curves, and newest layer curves remain explicit stops.

The derived layer parser uses an explicit Material **prefix** mode: `cosworks_material_layout.parse(..., require_eof=False)` returns the endpoint established by Material's own complete grammar, allowing the parent to consume that body and continue with its following fields. Default `require_eof=True` remains unchanged for standalone Material streams. The prefix mode does not scan for a marker, try arbitrary lengths, or treat a failed standalone parse as a usable prefix. A test verifies that standalone parsing rejects appended Property bytes while prefix parsing ends exactly at the known Material boundary and reconstructs only that prefix.

Fresh replay completes **all 234 Shells Property occurrences / 826556 bytes**. There are 239 version-candidate attempts, 236 complete; failed alternatives remain retained. Representative complete native subtypes are **0 (180 occurrences), 1 (41), and 2 (13)**. Seven inline layer Material bodies are consumed across the retained records. The replay does not count those embedded bodies as separate CFB stream occurrences or double-count them against the sibling Material inventory.

Thirteen tests pass across shell Property, Material, embedded ID and constant-surface profiles. The shell tests exercise older/modern empty-reference layouts, strict EOF/truncation, and nonzero virtual-body stops. Successful native replay preserves original scalar and floating-point bytes before native defaults/clamps; it does not emulate those runtime mutations.

The fifteen **MidSurfShells Property streams / 15770 bytes** remain outside this constructor-bound replay until their concrete dispatch is verified. Empty optional records in this corpus do not establish populated virtual +50 or final base-curve grammar, and the version-891106 layer flag/curve branch remains disabled. Separate composite stream profiles and their four-byte historical mismatch remain as in 14.364. No target executes; global semantic coverage credit remains zero and full installed-format schema coverage remains incomplete.

Evidence: C-744.
