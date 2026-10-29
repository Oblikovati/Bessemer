# Display notes and generic text

Bessemer reads and reconstructs the documented ordinary display-note body after
its complete display-object and display-item bases. The suffix retains separate
strings, signed words, ordered u32 flag chunks and all four independent tail
gates. Native flag repairs, vector normalization and rendering behavior do not
change stored fields.

The internal note component requires a null component pointer before reading its
two detail pointers. Note data reuses the existing detail-item base and adds its
tagged text pointer. Generic text reuses the dimension-text base, then reads its
tagged data list, raw word, display reference and direct paragraph. Text lists,
line heads and character records retain recursive map identities, f32 triples,
unsigned short flags and UTF-16 units, including lone surrogates. Absolute points
reuse the verified attachment body without an extra suffix.

These readers cover the documented ordinary modern scope above `0x27CB`. They
retain explicit failures for unimplemented nested classes and unsupported
component predicates. Complete framing does not establish the meanings of every
raw field or full part, assembly, drawing and installed-project semantics.

The owned fixtures contain 720 graphs across eight classes, fifteen versions and
six value styles, with 71,832 independent primitive observations. They include
note-to-text cycles, repeated detail pointers, recursive text lists, NaN payloads,
signed zero and exact historical boundaries. Fifty-five complete document
fixtures include seven independent edit goldens.

Two reference-fixture tests ran against an immutable source copy, and the actual
native note regression passed. Seven focused Rust tests passed, including every
truncated prefix, appended bytes, invalid chunk counts, unsupported component
predicates and unrelated cached text references. Three public integration tests
passed save/reopen, source-isolation, edit-golden and rejected-edit checks.

Two fresh native processes produced identical statuses and failure causes for
906 inputs. Complete resolved configurations increased from 836 to 842, retaining
every previous success; 64 inputs remain explicitly rejected. Both aggregate
corpus processes exited 101 because the corpus remains incomplete. Independent
readers confirmed EOF for all 842 accepted configurations, totaling 64,152,537
resolved-stream bytes, and matched all four source-stream hashes.

The JSON reports retain schema, fixture, native binding and reader hashes. The
humanizer skill was unavailable; its prose review is left to the maintainer.

Local `cargo xtask pr` passed on Windows x64 with Rust 1.99.0: formatting, Clippy,
build, 986 library tests, all integration tests and 274 doctests. Three external
fixture tests remain intentionally ignored by the default gate; the native
corpus was exercised separately as described above.
