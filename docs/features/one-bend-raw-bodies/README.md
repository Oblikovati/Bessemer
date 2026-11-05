# OneBend raw bodies

The `C285-one-bend-raw-v1` behavior reads and writes positive OneBend body
lists and nonzero optional bodies using the existing strict compressed
Parasolid decoder. Callers supply the kernel schema and expansion limits through
`PartConfigurationContext`. Empty branches continue to work without that context.

The AE0 count is always present in supported modern archives. The B00 count
appears after version `0x335A`. Signed counts remain unchanged; only positive
counts consume bodies. The B40 and B50 signed presence words appear at versions
`0x334C` and `0x334F`, respectively. Every nonzero value consumes one body.
Bodies have no suppression-byte prefix and allocate no object-map slots.
The existing A88 and final-word gates remain unchanged.

Section 11.243, C-285 in `../inventor-interface` supplies the surrounding grammar
and still reports explicit stops for positive bodies. The retained native
serializer independently binds all four branches to the named raw loader.
`native-binding-reference.json` records eight imports and verifies 4,802 bytes
of the installed OneBend serializer and 916 bytes of its raw loader against the
retained captures. Loader mode 2 changes post-read cleanup, while the length and
kernel-type framing remain the same. Bessemer does not reproduce native kernel
conversion, attribute cleanup or permanent-partition mutation.

Tests assemble positive tails independently across all nineteen existing archive
versions and six graph shapes. Eighteen complete document streams cover helper
states zero and one, empty and populated dependencies, distinct OneBend roots,
and a cached third occurrence. Each distinct root contains five kernel frames.
Independent edits change every body identifier from -7 to 9 or change the B40
presence from the signed minimum to -1. Fresh reversals retain original bytes.

Unchanged streams and presence-only edits require exact independent bytes.
Kernel edits compare every typed field against independently generated transmit
contents, then require exact reconstruction after reopening the produced carrier.
The independent Python fixtures use zlib level 6; Bessemer's existing writer uses
the Rust backend at fast compression. Compressed edited bytes are therefore not
claimed to match the Python encoding. No existing baseline was changed.

Run `cargo test --lib one_bend_raw_body_tests`,
`cargo test --test resolved_one_bend_raw_bodies`, and
`python -B docs/features/one-bend-raw-bodies/test_fixture_bytes.py`.
The source snapshot, native bindings, fixture hashes and local verification
records are kept in this directory for issue #220.

The accepted framing remains the strict type-zero compressed Parasolid profile.
Sentinel length-one bodies, other kernel types, historical and special contexts,
native geometry operations and SolidWorks application acceptance remain outside
this evidence. The humanizer skill is unavailable; prose review remains with the
maintainer.
