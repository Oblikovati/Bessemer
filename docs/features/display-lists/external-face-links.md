# External face links

`DisplayArchive::check_external_face_links` checks the link from a decoded display
archive through its face directory to supplied cache blocks. It first requires
every nonempty directory block name, then checks block versions, unique stored
component indices and directory index/hash pairs. A stored component index is
not a vector offset. Component IDs retain exact spelling.

The operation resolves every serialized `ExternalFacet` position in display-field
order. Repeated identical directory targets agree; conflicting targets for a
requested component return `Unsupported`. Missing declarations, named blocks or
requested unlocated targets return `Missing`. Invalid positions and inconsistent
index/hash links return typed failures. No partial facet list is published after
a required check fails.

`ExternalFaceLinks` borrows the immutable display, directory and block snapshots.
Each result identifies its requesting field, exact component ID, block name,
stored component index, face position and borrowed facet. Coverage distinguishes
checked references, distinct referenced positions, available cache records and
unused records. Empty block declarations are counted without inferring zero
geometry. A completed empty request set differs from a failed request.

The check establishes exact archive identity links. It does not certify topology,
geometry consistency, intersections, tolerances or native application acceptance.
Facets are neither healed nor copied by the library check. A caller must preserve
the borrowed snapshots until it releases the result.

Ten integration tests cover duplicate declarations, noncontiguous stored indices,
unused components and blocks, missing and ambiguous targets, invalid hashes,
version mismatches, late failures, empty requests, ownership, Send/Sync and
concurrent deterministic checks. Seven complete owned documents use independent
wire primitives and cover three successful scopes plus missing, unlocated,
out-of-range and ambiguous requests.

The independent verifier observes new-face reader events in the approved sibling
and captures original facet byte spans. This follows the documented native
`getNextFaceTessHandle` call in the new-face reader; object aliases do not call that
reader. It compares request paths, IDs, block/index/face positions, coverage and
every complete facet byte against two isolated Rust processes. Failure reports
must contain a typed operation, entity and cause, with no partial facets.

On Windows x64 with Rust 1.99.0, all 170 native documents with face directories
passed: 193 blocks, 1,258 directory entries and 35,242 external face requests.
The 470 empty block declarations were not selected by those external requests.
All requested facet bytes matched the independent reader. The sorted native
document manifest SHA-256 was
`f773168914ae36264c14293789e005ffe051f1cdba5add54d5d2e3dec1670aa1`.
Owned failure cases retained their expected classes and published no facets.
Reports are in `external-link-owned-verification.json` and
`external-link-native-verification.json`; fixture provenance is recorded in
`external-link-documents.json`.

```text
cargo test --test display_external_links --test display_document_context
cargo build --example check_external_display_links
python -B -X utf8 docs/features/display-lists/verify_external_display_links.py target/pinned-1.99.0/debug/examples/check_external_display_links.exe
python -B -X utf8 docs/features/display-lists/verify_external_display_links.py target/pinned-1.99.0/debug/examples/check_external_display_links.exe --native
```

Full display schema parity and cross-platform CI remain unfinished under #28.
The local `cargo xtask pr` gate passed, including 184 documentation tests. The
separate whole-document check retained byte-identical reconstruction of all 1,086
native documents after component declaration collection moved into the directory.
The humanizer skill was unavailable; prose remains subject to maintainer review.
