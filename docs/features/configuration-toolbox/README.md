# Ordinary Toolbox metadata interoperability

Bessemer reads, reconstructs and edits complete ordinary Toolbox specification
arrays and items through the existing model-archive readers. This advances
Feature #54 and parity goal #2. Sections 11.122 and 11.285 establish the shared
layout used by assembly references and part configuration managers; this change
enables it in the existing ordinary-part context above wire version `0x27cb`.

Arrays contain a direct ReadCount and tagged object pointers, followed by eight
CStrings, an i32, another CString and a final i32. The native assignment to
runtime member `+0x78` adds no stored field. Items contain an i32, two CStrings,
an f64 and two i32 values. The reader retains raw values and string framing,
including empty Unicode strings, extended ANSI lengths and duplicate strings.
Floating-point words retain negative zero, NaN payloads and infinities. Direct
arrays allocate no wrapper slots; tagged items, nulls and cached references keep
their original identities.

Seventy-two fixtures cover both classes at four versions, empty and populated
arrays, short and escaped counts, and three floating-point words. Their 1,836
primitive observations partition every byte. The reference's shared serializers
provide the consuming-read oracle. Its guarded ordinary Toolbox adapter is
checked separately for every case, including pointer identity and exact EOF.
This excludes its separate lookahead cursor from the primitive partition
without dropping any consumed fields or bypassing adapter validation.

Five owned metadata streams import the blank part's actual 97 model slots and
saved configuration-manager state. They retain the required MBSM continuation.
Both reference paths reach exact EOF with the expected slots. The helpers are
synthetic resolved-list roots used to exercise the existing public archive API;
their placement does not claim native feature-root semantics. Public tests
reconstruct whole documents and compare a Unicode edit with separately built
expected bytes, preserving aliases, floating-point bits, field history and
source documents. Four private tests cover primitives, slots, all truncated
prefixes, trailing bytes, unsupported versions and oversized counts. Three
Python tests verify deterministic regeneration, the matrix and invalid inputs.

Two fresh native runs have identical outcomes and failure causes. All previous
570 successes remain, and 574 of 906 configurations now reconstruct exactly.
Nineteen frontiers advance past Toolbox; fifteen encounter a later explicit
stop. The independent reference reader verifies all 574 accepted streams to
exact EOF, totaling 31,375,065 bytes, against four input-stream hashes per case.
The broad parity gate still fails for 332 configurations.
`frontier-checkpoint.json` and `accepted-reference.json` retain the measurements.

The staged `cargo xtask pr` passed on Windows x64 with Rust 1.99.0: 467 library
tests, four xtask tests, all integration suites and 265 doctests. Three external
corpus gates remain ignored by default. `local-verification.json` records the
executed checks, exclusions and source hashes.

`reference.json` binds fixtures, generators, builders and source readers.
`schema-snapshot.md` and `schema-reference.json` retain the relevant schema
sections and hashes. `dispatch.md` records the separate structural extraction
that precedes this behavior. Complete schema parity, native geometry, topology
and application acceptance remain outside this checkpoint. No dependency or
package version changed. The humanizer skill is unavailable; its prose pass
remains with the maintainer.
