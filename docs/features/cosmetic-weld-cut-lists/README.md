# Cosmetic weld cut lists

Feature #54 implements the complete ordinary modern `moCosmeticWeldCutList_c`
body from 11.347 / C-389. Behavior version:
`C389-cosmetic-weld-cut-list-v1`.

The installed model DLL binds primary table `0x4CE25158` to serializer
`0x4BB437B0`. The parent is `moReferenceCurve_c`, whose complete implementation
already retains reference geometry, a strict optional kernel body, node ID and
direct attribute list. This parent is distinct from `moRefCurve_c`.

The suffix stores five signed words in native serializer order: `+888`, `+780`,
`+778`, `+784`, `+788`. Their historical gates all hold in the supported modern
ordinary part context. The reader adds no pointer, count or object-map slot for
these twenty bytes and preserves every stored bit.

Independent fixtures cover 132 graphs, 26,820 primitive observations and fourteen
owned streams. Both present and suppressed kernel bodies cross eleven inherited
version boundaries, including the maximum unsigned version. Rich parents retain
shared references and attributes; repeated roots retain identity while distinct
roots remain separate. Signed extrema and unequal suffix values expose member
reordering. Two independent edit goldens change only `raw_888`.

Run `cargo test --lib cosmetic_weld_cut_list_tests`,
`cargo test --test resolved_cosmetic_weld_cut_lists` and `cargo xtask pr`.
Regenerate with
`python -B -X utf8 -m unittest discover -s docs/features/cosmetic-weld-cut-lists`.

Other cosmetic tree and bead bodies, earlier/internal contexts, scalar meanings,
geometry validity, application acceptance and whole-schema parity remain
incomplete. The copied clause and native evidence also describe these later
bodies; this implementation claims only the cut-list class. Humanizer is
unavailable; maintainer prose review remains open.
