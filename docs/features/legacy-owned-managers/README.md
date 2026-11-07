# Version-18000 owned configuration managers

The configuration-manager reader selects an exact version-18000 or version-19000
dialect before reading bytes. Dependent configuration and scene readers retain
that selection. Unselected readers keep the current-version restriction, and a
selected dialect must match the stream version and ordinary part context.

For version 18000, the manager ends before the display-map presence word. The
nested configuration object's `+0xA8` pointer exists only above `0x4972`; the older
layout omits that pointer as well. Scene
field-of-view bytes are absent through `0x46BA`; the existing scene codec already
implements this boundary. HDR fields remain present under their separate gate.
This scope does not enable neighboring historical versions.

`freeze_native_managers.py` copies and hashes the independent Python readers from
`../inventor-interface`, reads ten native manager streams to EOF, and records their
complete primitive partitions. `native-reference.json` identifies those inputs
and reader revisions. The owned streams total 124,901 bytes and contain six scene
bodies. Manager completion alone does not establish complete document support.

The Rust regression tests compare typed fields with those independent primitive
partitions, reconstruct every manager byte, and reject unverified dialects,
contexts, truncated input, added modern tails and insufficient byte budgets.

Two fresh native processes agreed on every status and failure cause across 906
inputs. Complete resolved configurations increased from 829 to 836, preserving
every previous success; 70 inputs remain explicitly rejected. Both aggregate
corpus processes exited 101 because the corpus is still incomplete. The seven
newly accepted indices are recorded in `corpus-reference.json`.

Independent readers from a fresh immutable source copy reached EOF for all 836
accepted configurations, totaling 60,070,658 resolved-stream bytes. All four
input-stream hashes matched the corpus manifest. `accepted-reference.json`
records the source hashes and accepted indices. These checks establish stream
reconstruction, not full document semantics. No package version or release is
changed.

Local `cargo xtask pr` passed on Windows x64 with Rust 1.99.0: formatting,
Clippy, build, 979 library tests, all integration tests and 274 doctests. Three
external-fixture tests remain intentionally ignored by the default gate; the
906-input native corpus was run separately as described above.

The humanizer skill was unavailable; its prose review is left to the maintainer.
