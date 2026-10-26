# Cut-list sort and face-color configuration records

The ordinary model decoder now reads complete `moCutListSortOptions_c` and
`moFaceColorConfig_c` bodies. Sort options preserve the raw signed word and
packed object list, including self-reference and repeated identity. Face-color
configurations preserve the typed component-face and visual-property pointers
and raw signed selector. Neither reader interprets the raw words as enums or
applies native postload changes.

Three owned fixtures cover a self-referential sort list, a separately generated
expected scalar edit and a face-color record with null pointers and a signed
extreme. Their independent primitive partitions contain nine, nine and seven
observations. Four focused Rust tests passed exact reconstruction, the expected
edit, all truncated prefixes, trailing bytes, unsupported versions, typed pointer
validation, missing references, impossible counts and completed empty lists.
Two fast independent Python face-color tests passed; their native-corpus test
was excluded from that command.

Exact sample reconstruction increased from 386 to 388 of 906 configurations,
preserving every previously accepted case. Two fresh processes produced
identical outcomes. The independent reader completed all 388 accepted cases
against matching selected header, model, manager and feature hashes, covering
15,305,243 feature bytes. The remaining 518 cases fail explicitly. These checks
establish stored framing, not native application or geometry acceptance.

The source and fixture hashes are in `folder-configuration-reference.json`;
the frontier and accepted-reference reports retain the corpus evidence. Clauses
11.132 and 11.265 are retained with verified raw hashes. The unfinished private
resolved-feature API still produces 20 dead-code warnings. Connecting that API
to a production caller remains required before integration; the warnings are
not suppressed. No dependency, version, release, PR or push changed. The
humanizer skill is unavailable in this session; its prose review remains with
the maintainer.

Full `cargo test` passed 413 library tests, the integration suites and 247
doctests, with three external gates ignored by default. The explicit sample
gate was run twice and still failed as reported above. The 20 dead-code
warnings remain in the retained log; no green PR gate is claimed.
