# Cosmetic weld tree

Feature #54 implements the remaining ordinary modern tagged classes in
11.347 / C-389: cosmetic subfolder, bead, bead data, from/to parameters,
intermittent parameters and wire-edge/face map. Behavior version:
`C389-cosmetic-weld-tree-v1`. The preceding cut-list implementation remains
unchanged.

Subfolders and beads reuse the complete feature prefix. Property, bead-data,
parameter and symbol pointers retain their exact native roles. The bead's
display-item pointer admits the complete existing display-weld and five
display-dimension implementations. New unsupported display-item bodies still
stop independently.

The bead-data constructor owns nine direct arrays and two direct endpoint
records. These bodies add no outer tags or map slots. Each endpoint stores two
doubles, two signed words and another double, occupying 32 bytes. Between the
arrays, the reader retains the tagged parameters and cosmetic symbol, strict
kernel body and raw suffix scalars in native wire order. Empty arrays, null
pointers and a suppressed kernel body give a 113-byte body. From/to and
intermittent parameters occupy 20 and 24 bytes respectively.

The wire map retains its unsigned leading word and two signed list counts.
Negative counts consume zero entries while preserving the stored count.
Positive counts are bounded against remaining bytes and operation budgets.
No padding, allocation metadata or semantic normalization is added.

Independent fixtures cover 324 graphs, 38,477 primitive observations and 42 owned
streams across nine inherited display-item version boundaries. They include
null, fresh and cached typed pointers; generic array cycles; distinct and
repeated roots; both kernel modes; negative-empty and populated signed lists;
IEEE payload bits; signed extrema; and all five existing display-dimension
classes. Six edit goldens change only a selected scalar. Public tests verify
input isolation after accepted and rejected edits.

Two fresh native corpus processes matched all 906 outcomes and failure causes.
The `insert_weld.sldprt` resolved-feature stream now reconstructs all 29,940 bytes;
the complete count increased from 818 to 819, with no prior success lost. The
independent frozen reader reached exact EOF for the same 819 inputs, totaling
54,868,201 feature bytes. The remaining 87 inputs still fail explicitly.

Run `cargo test --lib cosmetic_weld_tree_tests`,
`cargo test --test resolved_cosmetic_weld_tree` and `cargo xtask pr`.
Regenerate with
`python -B -X utf8 -m unittest discover -s docs/features/cosmetic-weld-tree`.

Historical/internal contexts, unsupported display-item classes, geometry
validity, scalar meanings, application acceptance and whole-schema parity
remain incomplete. Humanizer is unavailable; maintainer prose review remains
open.
