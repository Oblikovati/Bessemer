# Sheet-metal folds

Bessemer reads and edits the four native bindings documented in section 11.246,
C-288: `moSM3dBend_c`, `moFold_c`, `moUnFold_c` and `su_CMapObToInt`.
The behavior version is `C288-sheet-folds-v1`.

3D bends retain their complete body-feature base and bend-group layer. Folds
start with the body-feature base directly. Their arrays and the unfold map are
direct bodies, without additional object tags or map slots. Map entries retain
wire order, repeated keys and signed values before native runtime normalization.

Unfolds retain nonpositive body counts without reading bodies. Positive counts
use the existing strict compressed Parasolid reader, requiring an explicit schema
and expansion limits. The transform always uses the fixed thirty-word archive
operator. Special kinds with `word28 & 0xF0 != 0` stop explicitly.

Independent fixtures cover 168 complete graphs, seven archive versions and six
pointer styles, with 38,055 primitive observations and 28 complete document
streams. They include positive compressed-body counts, duplicate map keys,
signed extrema, floating-point payload bits and independently generated edits.
The small kernel fixtures validate framing and typed partition contents; they
make no geometric-validity claim.

Two fresh native-corpus processes produced identical statuses and failure
causes, preserving all 821 completed configurations and 85 explicit stops.
Native sample 407 advanced from the unsupported unfold class to an unsupported
hem class, so it remains incomplete. The independent reader's native regression
test verified five 3D-bend roots and two fold-family roots, totaling 91,363 bytes,
against their stored hashes, field values and containing-list boundaries.

Run `cargo test --lib sheet_fold_tests` for primitive oracles, collection sizes,
kernel-context requirements, unsupported transform kinds and exhaustive prefix
rejection. Run `cargo test --test resolved_sheet_folds` for complete document
reconstruction, independent scalar edits and input isolation. Run
`python -B docs/features/sheet-folds/run_pinned_tests.py` to regenerate fixtures
using frozen copies of the actual `../inventor-interface/solidworks` readers.

The humanizer skill is unavailable in this environment; prose review remains
with the maintainer.
