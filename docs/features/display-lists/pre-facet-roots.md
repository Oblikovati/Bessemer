# Earlier resolved archives and unresolved facet payloads

The version-3065 checkpoint replaced the blanket 3196 lower bound, which
rejected complete archives whose layouts the reference can read. The subsequent
[pane, line and symbol checkpoint](legacy-panes.md) admits contexts from 2999.
Versions `0xA25` through `0xC7B` contain an unresolved object inside each inline
facet payload. That remains a typed unsupported error when a facet is present;
it does not prevent reading an archive with no shaded data, an empty part, or a
component whose part-tessellation pointer is null. Assemblies may also retain
nonempty parts whose facets come from a declared external cache. No unresolved bytes are skipped
or replaced with empty geometry during production decoding.

Three stored-field boundaries apply in this interval. User-pen layer IDs start
after `0xC57`; parabola records gain their two trailing doubles after `0xC58`;
assembly components gain `unknown_320` after `0xC6E`. Earlier layouts omit those
fields. Root, font, material, light and other previously covered gates retain
their existing behavior. At this checkpoint, versions below 3065 and the uncounted-window interval
remained unsupported at the context boundary.

The [owned corpus](pre-facet-roots.json) contains 94 complete roots across eight
versions from 3065 through 3196. It covers both document kinds, unshaded roots,
empty shaded parts, null component part pointers, independent pen edits, and all
five old-light classes. Sixteen additional cases retain non-null part objects and
two external face references each. The 78 previous cases deliberately construct
empty geometry and retain their original bytes and metadata. Incoming files with
unresolved inline facet records still fail at that payload. The
source's logical `su_CTime` sentinel -1 is checked as its stored unsigned word
`0xFFFFFFFF`, as specified in clause 13.7.

The generator starts from owned, frozen version-3196 roots. Reference-observed
boundaries identify the item lists and complete tessellation bodies. Removing
trailing model objects requires no map-index relocation: the only following
payload is a verified empty textured-plane list. Kept pointers retain their
original map semantics. No native corpus bytes are newly redistributed.

Independent traces provide 7,230 field expectations. Forced construction matches
every complete byte in two isolated Rust processes. Tests distinguish absent
shaded data, empty parts and null part pointers; check each field gate and
cross-version shape rejection; preserve source snapshots through successful and
failed edits and authentic reverts; and reject sampled truncations, each short
tail and trailing bytes. A populated version-3196 facet root reinterpreted as
3195 must return an unsupported facet-layout failure, not a successful empty
result or a generic context rejection. Existing earlier fixture bytes stay
unchanged. External cases also resolve both exact cache indices and borrow the
corresponding immutable facets. Missing declarations or blocks fail explicitly;
pen edits preserve face identities and authentic reverts.

Two complete version-3065 containers also pass context resolution and forced
replacement/reopening in two processes. Verification uses Windows x64 with Rust
1.99.0 MSVC and the sibling reference reader. This is wire-schema evidence for
#28, not geometry or native application acceptance. Full documented-file parity
remains in progress; no version bump or release is included.

Four more [complete assembly containers](pre-facet-external-documents.json) cover
versions 3065 and 3195, before and after independent pen edits. Their directory
and facet blocks use version 19000, separately from the display version. The
[external verification](pre-facet-external-verification.json) matches all eight
resolved face-reference reports and complete facet bytes against the reference,
then forces whole-document reconstruction and resolved contexts in two processes.
Integration tests edit, reopen and revert these documents while checking that
every other stream remains unchanged and the cache links still resolve. These
real external cases also exercise the context-preservation correction in #39.

```text
python -B -X utf8 docs/features/display-lists/generate_pre_facet_roots.py
python -B -X utf8 docs/features/display-lists/generate_pre_facet_external_documents.py
cargo test --test display_pre_facet_roots
python -B -X utf8 -m unittest discover -s tests -p test_display_fixture_context.py
cargo build --examples
python -B -X utf8 docs/features/display-lists/verify_pre_facet_roots.py target/pinned-1.99.0/debug/examples/rebuild_display_archives.exe
python -B -X utf8 docs/features/display-lists/verify_earliest_component_documents.py target/pinned-1.99.0/debug/examples/rebuild_display_documents.exe pre-facet
python -B -X utf8 docs/features/display-lists/verify_pre_facet_external_documents.py target/pinned-1.99.0/debug/examples/check_external_display_links.exe target/pinned-1.99.0/debug/examples/rebuild_display_documents.exe
cargo xtask pr
```

The maintainer-side humanizer skill was unavailable; its prose pass remains for
review.
