# Material textures and per-view shaded edges

Reference baseline: specification edition 0.244, dated 2026-10-25. Reader hashes
are recorded in the fixture manifests.

The 190 complete part and assembly roots cover versions 2359, 2360, 2374, 2375
and 2388, with 52439 independently expected traced fields. Material textures,
including their presence flag, are absent through 2359. The per-view
`edges_in_shaded_mode` word is absent through 2374. Existing material-rendering,
classification and ordinary-text gates remain in force independently.

The generator observes complete material records at version 2388. Its owned
fixtures contain empty user-property lists; it verifies their count and the
texture flag before removing the preceding texture interval. The view word is
located after the two observed view transforms and the active-view and display-mode
words, and its stored value is checked against the reference result before
removal. These primitive-only removals leave every MFC class and object identity
unchanged. Each final root is parsed to its end at the declared version.

Forty inline part and assembly roots each contain two complete facets. Their
layout is unchanged across this interval; the independently built version-2388
facet records remain the expected typed contents and bytes. Tests force facet
writes at each actual archive version. Composite text, polygon, edge and facet
stamp edits match independently built roots and authentically revert. Changing
an old archive's context to require the newly stored fields fails before
publication and preserves the source. Truncation checks also reject partial roots.

Four owned containers cover baseline and edited part and assembly streams at
2359. Replacement, reopen and revert preserve unrelated streams. Process
verifiers compare complete roots and whole containers in two fresh processes.

This checkpoint admits archive contexts from 2359. Earlier documented layouts
and unresolved representations remain tracked by #28. Byte fidelity does not
establish geometric validity or native application acceptance.
