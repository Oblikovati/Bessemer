# Legacy pane, line and symbol layouts

Display archives now admit contexts from version 2999. Three older caller
layouts extend the resolved interval from the previous 3065 boundary:

- Through `0xBF8`, line items store `unknown_30` as a signed 32-bit word;
  later layouts use one byte. The reference converts the old word to a logical
  boolean. Bessemer retains its physical word, including noncanonical nonzero
  values, so edits and reconstruction do not normalize away stored bits.
- Splitter panes omit their complete `unknown_a8` CString through `0xBDA`.
- Simple symbols omit both the 3D presence word and the plane transform through
  `0xBBD`. A missing presence word is distinct from a stored zero.

The generator observes each CString call inside the owned splitter bodies and
removes only those physical spans. It restores every reference hook after use.
These strings and the replaced display items add no MFC map slots. The remaining
root and model fields reuse the independently checked older layouts. For old
line items, an observer confirms the reference's boolean result against the raw
word before recording that word in the physical-field oracle.

The [owned corpus](legacy-pane-roots.json) contains 66 complete roots with 7,487
independent field expectations. It covers both document kinds, seven boundary
versions, all five old-light classes, absent and empty shaded data, cache-backed
faces, raw line edits and pen edits. Two isolated processes force exact bytes.
Tests check field types and omissions, neighboring-version construction errors,
sampled truncations, every short tail, trailing bytes and authentic reverts.
The previous 94-root corpus remains unchanged.

Two complete version-2999 containers pass independent context resolution and
forced reconstruction. Four further [cache-backed assembly containers](legacy-pane-external-documents.json)
cover versions 2999 and 3064, before and after independent line/pen edits. Their
cache versions remain 19000. The [external report](legacy-pane-external-verification.json)
matches all eight requested facet-reference reports and exact facet bytes against
the reference, then reconstructs the complete files in two processes. Integration
tests edit, reopen and revert these documents while preserving all other streams
and verifying their face-cache links.

Inline facet-object payloads in `0xA25`–`0xC7B`, versions below 2999 and the
uncounted-window interval remain explicit unsupported layouts. No unresolved
geometry is replaced with an empty result. Verification used Windows x64 and
Rust 1.99.0 MSVC. This is wire-schema evidence for #28; geometric interpretation,
native application acceptance and full documented-file parity remain separate
work. No version bump or release is included.

```text
python -B -X utf8 docs/features/display-lists/generate_legacy_panes.py
python -B -X utf8 docs/features/display-lists/generate_legacy_pane_external_documents.py
cargo test --test display_pre_facet_roots
cargo build --examples
python -B -X utf8 docs/features/display-lists/verify_legacy_panes.py target/pinned-1.99.0/debug/examples/rebuild_display_archives.exe
python -B -X utf8 docs/features/display-lists/verify_earliest_component_documents.py target/pinned-1.99.0/debug/examples/rebuild_display_documents.exe legacy-pane
python -B -X utf8 docs/features/display-lists/verify_pre_facet_external_documents.py target/pinned-1.99.0/debug/examples/check_external_display_links.exe target/pinned-1.99.0/debug/examples/rebuild_display_documents.exe legacy-pane-external
cargo xtask pr
```

The maintainer-side humanizer skill was unavailable; its prose pass remains for
review.
