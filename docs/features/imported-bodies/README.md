# Imported and base-body interoperability

Bessemer reads, reconstructs and edits complete ordinary-part `moImported_c` and
`moBaseBody_c` features through the existing resolved-feature API. This advances
Feature #54 and goal #2. Both classes use the complete inherited body and the
imported layer documented in section 11.152. Only base bodies have the additional
fixed signed integer vector, and only above model wire version `0x2BCC`.

Filename and body-name strings retain their original encodings and values,
including empty names and Unicode text. Every nonzero external-body marker
skips inline loading; its full signed value is preserved. A zero marker selects
the complete direct kernel-body frame. Suppressed bodies preserve their original
byte. Compressed partitions reuse the existing typed kernel reader, require
explicit schema/expansion limits and reconstruct their complete framing. Unknown
kernel framing remains an explicit stop. No file is opened from a serialized
filename, and native post-load name replacement is not applied.

Independent fixtures cover 48 complete objects across both classes, four version
contexts, empty/populated fields and external/suppressed/compressed body modes.
Their 7,622 primitive observations partition every byte. The reference reader
reaches exact EOF and records final object-map slots. Tests compare every
primitive, byte and slot, reject every truncated prefix and trailing byte,
reject old contexts and malformed vector counts, and require explicit kernel
context only when a compressed body is present. Compressed fixtures contain a
complete synthetic structural partition; they do not establish geometric validity
or positive native imported-feature inline-body coverage.

Nine owned streams use the blank part's actual model and CMgr state with 97
inherited slots. Public tests reconstruct whole documents byte for byte for
both classes and every body mode. A longer Unicode body-name edit matches a
separately constructed expected stream, preserves the typed partition and signed
vector, records one field-history change and leaves source archives and documents
unchanged. A missing-context failure also leaves the document unchanged. Three
Python tests check deterministic regeneration, class/mode/gate coverage and invalid
construction options.

The staged `cargo xtask pr` passed on Windows x64 with Rust 1.99.0: 450 library
tests, all integration suites and 265 doctests. Three external corpus gates
remain ignored by default. Five private reader tests, three public operation
tests and three Python fixture tests passed. `local-verification.json` records
the executed scope and runtime/test source hashes.

Two fresh native corpus processes reconstruct 532 of 906 feature configurations
exactly with identical outcomes and failure causes, retaining every previous
516 success. The broad parity gate still fails for 374 explicit stops. The
independent reader verifies all 532 accepted streams, totaling 27,867,060 bytes,
against four-stream input hashes. `frontier-checkpoint.json` and
`accepted-reference.json` retain those observations. Full documented schema
parity remains incomplete.

`test-registry.md` records the separate structural test-declaration move.
`reference.json` binds fixtures, generators, reused builders and reference
readers. `schema-snapshot.md` preserves the relevant specification sections
with raw-byte hashes in `schema-reference.json`. The reference reader emitted
an unmanaged schema-file `ResourceWarning`, catalogued as
[Bug #68](https://github.com/Oblikovati/Bessemer/issues/68); its source
and parser expectations were not changed.

Runtime file version 19000 remains distinct from model wire version. Database
paths, nonzero imported-helper state, alternate archive conditions, historical
contexts outside the modern scope, geometry, topology and native application
acceptance remain outside stored-schema validation. No dependency or package
version changed. The humanizer skill is unavailable; its prose pass remains with
the maintainer.
