# ShellManager headers and direct groups

Feature [#156](https://github.com/Oblikovati/Bessemer/issues/156) implements
complete reads, validated construction and isolated edits for specification
14.363/C-741 and 14.368/C-747. The wire-validation behavior profile is 1.

`SimulationShellManagerProperty` retains signed +C8 and adds +10C only above
reader version 800027. The explicit Writer profile always includes both words.
`SimulationShellGroup` reads a signed count followed by direct records: four MFC
CStrings, two signed words, a raw double, three signed words and +84 from reader
version 890016 or under Writer. These groups have no object tags or embedded
archive-version header. Header values, group counts and CFB enumeration remain
separate; destination offsets do not assign business meanings.

Both APIs require normalized `SimulationPersistVersion`, explicit
`SimulationShellWireProfile` and caller budgets. Writer selects unconditional
late fields; CString representation remains explicit and retained. The installed
string writer emits wide strings, while native narrow conversion can depend on
locale. Callers can construct wide values with `ArchiveString::new` and
`TextEncoding::Utf16`; the library does not infer a locale or normalize existing
narrow frames. This profile does not claim native writer execution.

Group limits cover all input/output bytes, primitive occurrences, declared records
and aggregate CString payload bytes. Length/encoding prefixes count toward whole
bytes, rather than text payload. CString lengths are checked against the remaining
aggregate text budget before decoding. The recovered bounded-reader cap is
100,000 records. Exact EOF is mandatory, including a zero-record group's count.

Published bodies and edits are immutable. IDs belong to one revision; no-ops keep
those identities. Changed values receive complete ordered source/replacement
history, with exact surrounding-byte preservation. CString prefix width and
encoding affect wire no-ops even when decoded text matches. Doubles retain raw
bits, including NaN payloads and signed zero. Coverage returns completion, selected
grammar, limits, diagnostics and achieved pattern error together.

Fresh recovery matches every original dictionary and payload hash. C-741 contains
430 streams /3,164 bytes and 474 arguments: 444 accepted, 30 failed. These are
361 direct headers and 69 empty groups. C-747 separately checks those same groups
/276 bytes with all 79 arguments accepted. Adding both report counts would inflate
the unique denominator. Native groups remain empty; populated-record evidence is
installed static recovery plus independent owned literals.

All 19 strict reference checks pass, including six original tests, complete retained
clauses, exact source hashes and installed function/threshold/literal bindings.
Thirty historical-reader and 30 current-writer header profiles match full fields
and edit history. Seventy-two group profiles cover empty/multiple records, all MFC
length widths, embedded NUL, Latin-1/UTF-16 and signed/raw-double patterns. All 312
owned typed edits include prefix-width and explicit encoding changes. The 876
mutants retain 192 accepted controls and 684 rejections. Every one of 1,583,832
proper prefixes and all trailing-byte cases fail. Complete 100,000/100,001-record
payloads independently establish the fixed cap without insufficient-byte masking.

All 36 focused Rust tests pass with explicit native inputs, and clippy passes with
warnings denied. Two fresh processes match complete fields and ordered histories
against independent Source packets. Compile-time assertions cover header/group,
profile and shared CString types. Each API's eight-worker, 32-edit mixed test
matches three serial options. Tests also cover exact/one-below budgets, zero text
requirements, typed errors, every field's no-op, foreign/stale IDs, malformed
construction, partial-reparse failure and mutable-input/caller-field isolation.
See [combined verification](interop-verification.json), the earlier
[header ledger](header-interop-verification.json) and isolated shared-helper proofs.

Run `python -B -X utf8 docs/features/simulation-shell-manager/freeze_shell.py`
for strict replay; `--write` initializes evidence for review. Original reports can
contain decoded values and spans. Generated native packets stay under `target/`.
No target code executes. The full local gate passed: 3,409 tests, zero failures and 84 explicit ignores
across 258 summaries. See [gate verification](full-gate-verification.json).
Exact-head platform integration remains pending.

Shell/midsurface Property, inline Material, composites, runtime dispatch, native
locale conversion, field meanings, geometry and native CAD writer/application
acceptance are separate work. Earlier source checkpoints remain retained; no
earlier Bessemer group implementation is replaced. Humanizer was unavailable;
maintainer prose review remains pending.
