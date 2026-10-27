# Feature comments

Bessemer reads, edits and writes the documented ordinary modern
`moNewFeatComment_c`, `moCommentExtraData_c` and `moCommentTextItem_c` bodies.
These require an ordinary external-XML context with version above `0x27CB`.
The comment serializer starts with its direct list; runtime ancestry does not
add an inherited string/state prefix. Further comment item variants remain
unsupported until their complete bodies are documented.

The direct list preserves packed counts, repeated pointers, nulls and distinct
objects of the same class. Direct extra data consumes no object-map slot. Its
three signed words appear in native order +0C, +08, +10 at version `0x3B10`
and later. Earlier direct extra bodies contain no bytes. The stored kind word
is retained even before `0x332C`, when native loading transforms kind 2.

Text items retain each separate CString and their complete unsigned four-byte
time value. A fourth string appears at `0x3B10`; older bodies contain no
placeholder. Timezone, application meaning and string roles are not inferred.
Feature and chain pointers retain explicit class families and their separate
occurrences. Unknown bodies and unrelated owner pointers fail explicitly.

Independent readers consumed 48 complete cases to exact EOF and recorded 910
primitive observations. Six versions cover both kind-migration and string/extra
boundaries. Four private tests compare every primitive, reconstructed byte and
slot, reject every truncated prefix and extra byte, check owner families, and
pin time width and extra-word order. Three Python tests reproduce all fixtures
and verify matrix coverage and invalid constructor boundaries.

Four owned streams import the blank part's actual model and CMgr state with 97
slots. Public tests reconstruct complete documents and compare a Unicode text
edit with separately constructed expected bytes. Shared text identities, time,
extra words, owner objects and source inputs remain intact.

Two fresh native processes produce identical outcomes and failure causes.
All previous 630 successful configurations remain exact. Eleven comment
frontiers advance to later missing bodies or pointer-family gaps; none becomes
a complete configuration yet. The corpus still accepts 630 of 906 and the broad
parity assertion still fails for 276 configurations. `frontier-checkpoint.json`
retains these measurements. A fresh independent audit consumes all 630 accepted
streams to exact EOF, totaling 36,734,048 bytes, and verifies four input hashes
per case. It uses an immutable workspace copy of 1,061 Python files, checked
against stable original hashes before copying and unchanged hashes throughout
the audit. Its identity and file hashes are in `accepted-reference.json`; the
temporary copy is removed after verification. Complete schema parity, geometric/topological
validation and native application acceptance remain outside this checkpoint.

`reference.json` binds the independent readers, reused builders and generators.
`schema-snapshot.md` and `schema-reference.json` retain the relevant clauses and
hashes. No dependency or package version changed. The humanizer skill is
unavailable; its prose pass remains with the maintainer.

The workspace-wide `cargo xtask pr` gate passed on Windows x64 with Rust 1.99.0:
490 library tests, four xtask tests, all integration suites and 265 doctests.
Three external corpus tests remain ignored by default; the native measurements
above come from two separate explicit runs. `local-verification.json` records
the gate-log and source hashes.
