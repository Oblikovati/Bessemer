# Graph parameter interoperability (#165)

Specification 14.385 and C-765 describe the complete `cwPostGraphParam` reader
provided by `cwPostCommon.dll`. The original corpus retains 224 streams and
83,328 logical bytes. Graph and DGXPOST dialog parameters are separate serializers.

The reference foundation checks every original source/decompilation hash,
complete installed caller/provider images, exact named import and IAT slot,
provider export and both reader/helper function spans. It reads installed bytes
without loading target code. Native replay recovers each original document
envelope and requires every complete original outcome, including string bytes,
field spans, reconstruction hashes and failed candidate traces. Complete corpus
identities and Results category/plot storage shapes are checked independently.

Graph strings contain a signed character count followed by twice that many raw
UTF-16 bytes. They have no required terminal NUL. The reference profile accepts
counts 0..1024, preserving unusual code units; it rejects negative or larger
counts instead of emulating native clamping. CwPersistStream byte-count text and
MFC variable-length CString are different wire grammars. The codec must keep a
character-count string as one editable value: a different-length replacement
updates count and payload together, preserving subsequent field offsets. Reusing
the existing byte-counted text type would impose the wrong terminator contract.

Run the reference foundation with:

```powershell
python -B -X utf8 docs/features/simulation-post-graph/freeze_post_graph.py
```

Initialization and strict replay each passed all nine checks: three original
tests and six foundation checks. Both processes exited successfully after
cleanup. All 224 original arguments were accepted; no failed alternative was
recorded in this inventory. [Foundation verification](foundation-verification.json)
records the complete native packet, source identity, log hashes and exclusions.

Independent literals now cover 270 complete contexts across 27 versions, five
signed/raw64/text rotations and separate empty-text controls. Their exact source
spans, raw strings and reconstruction hashes match the original parser. The
eight-byte values remain unassigned raw patterns. Text cases include embedded
NULs, noncharacters, lone surrogates and surrogate pairs without a terminal-NUL
requirement.

Owned inputs and complete field/history packets are retained under
`tests/fixtures/simulation-post-graph`. Field projections merge each character
count and payload into one atomic text value, using fixture tag 6 to distinguish
it from byte-counted text. Every owned context and all 224 native arguments also
have a complete first-word edit oracle with every copied/derived occurrence
parent in physical order. [Packet references](operation-packets-reference.json)
record exact bytes and hashes. These reference packets do not establish Rust
operation acceptance.

The final source initialization and strict replay each pass all 15 checks.
Independent resource packets cover every owned and native argument, recording
complete fields, total declared characters and maximum text payload bytes.
Empty strings retain zero character/text requirements. All four actual string
positions have full 1,024-character accepted and 1,025-character rejected
payloads, plus negative-count controls. Each accepted cap image retains all 79
physical typed fields and its complete graph suffix.

The mutation set retains 8,820 actual grammar outcomes: 8,400 rejected inputs
and 420 accepted results, including arbitrary changed code units. Every one of
the 90,630 owned proper prefixes and all 270 appended-byte cases rejects.
[Resource and adversarial verification](resource-adversarial-verification.json)
records packet identities, full-outcome hashes, source identity and log hashes.
These are independent expected results for the forthcoming Rust acceptance.

`SimulationPostGraphParameters` now reads and constructs complete bodies under
an explicit version. `SimulationPostGraphText` retains bounded raw UTF-16 code
units as one character-count frame. Signed words, unassigned `Raw64` patterns
and text remain distinct public value types, even when a raw pattern could
encode a valid text frame. Isolated edits preserve complete source/replacement
parents, all unaffected fields, shifted offsets and ordered occurrence history.

All 18 focused Rust checks pass, including three native checks. Reading and
construction pass exact byte, field, aggregate-character and per-text payload
budgets and fail one below every positive requirement. Fully supplied cap
controls, all 8,820 actual malformed outcomes, all 90,630 proper prefixes and
270 appended-byte cases match the source profile. All owned fields pass no-op
identity checks; foreign, stale, bounds and wrong-type requests reject.

Changed-value checks cover 21,390 edits: 12,720 signed words, 4,260 raw eight-byte
patterns and 4,410 character-text frames. Caller buffers, field vectors and text
payloads remain isolated after success and failure. Eight workers execute
175,440 mixed operations, matching complete serial bytes, fields, offsets and
coverage while checking each edit's ordered parents. All sixteen public contract
types satisfy `Send` and `Sync`; Clippy passes with warnings denied.

Two fresh Rust processes exported identical complete owned/native fields
(1,637,647 bytes) and history (2,513,677 bytes), each matching the independent
source packets byte for byte. [Runtime verification](runtime-verification.json)
records their exact hashes, final logs and exclusions. The full local
`cargo xtask pr` gate passed at runtime commit
`73353ff7083915ca925df1b5b61ee8cabd62d15e`: 3,521 tests passed, zero failed,
and 108 were ignored across 266 summaries. The process exited successfully;
[full gate verification](full-gate-verification.json) records its tree and log hash.
Exact-head platform and ThreadSanitizer acceptance remain pending.

Native application opening, short-read/clamp emulation, chart business meanings,
historical writer equivalence, dialog and auxiliary streams, geometry validation
and whole-format parity remain unproven.

Humanizer was unavailable; maintainer prose review remains pending.

A fresh replay before serialized publication passed all15 source checks against
the current documentation and retained all224 native arguments. Two fresh
combined-root Rust processes each passed all18 focused tests with no ignored
checks. Their complete field and edit-history exports matched each other and
the independent owned/native packets byte for byte; both hashes also match
the earlier acceptance run. [Publication source verification](publication-source-verification.json)
records the new snapshot, binary and logs. The historical source snapshot is
preserved. This refresh does not replace the required exact publication-head
Cargo and platform gates; PR #176 remains the sole publication PR.
