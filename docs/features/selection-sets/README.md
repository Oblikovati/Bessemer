# Selection-set archives

Bessemer reads complete ordinary `moSelectionSetData_c` and
`moSelectionItemData_c` records using their node lightweight stage. The resolved
node stage is empty; these records do not inherit feature or folder payloads.
The set retains its signed word, raw CString, optional comment after version
`0x3720`, and direct ordered item list. Each item retains its shared wrapper
pointer after the node body.

The comment role accepts the documented `moNewFeatComment_c` subtype from its
proved `moFeatComment_c` ancestry. Fresh and cached targets retain the shared
archive map, including duplicate list entries and references to the containing
set. Unrelated cached targets and unsupported nested bodies remain explicit
errors.

Independent fixtures cover 84 graphs across two classes, seven versions and six
value styles, with 4,388 primitive observations and 13 owned document fixtures.
Four focused Rust tests and three public save/reopen, immutable edit and input
isolation tests passed. The pinned source reader regenerated all fixture bytes
and observations. Its four-file native regression also passed, retaining all
15 observed selection items and the recorded later dependency failures.

Two fresh native processes matched all 906 statuses and failure causes.
Complete typed reconstruction increased from 854 to 858 configurations, adding
indices 4, 5, 6 and 11 without losing previous completions. The aggregate corpus
still fails because 48 configurations remain incomplete. The independent reader
confirmed exact EOF for all 858 accepted configurations, covering 68,523,677
resolved bytes and all four source-stream hashes. The full `cargo xtask pr` gate
passed formatting, Clippy, build, 1,017 library tests, all integration tests and
277 doctests. Three external-fixture tests remain ignored by the default gate.
Archive reconstruction
does not prove complete CAD semantics or native application acceptance; parts,
assemblies, drawings and all documented project formats remain in scope.

The humanizer skill was unavailable; its prose review is left to the maintainer.
