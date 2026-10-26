## Bessemer 

Bessemer is a Rust library being built to read and write SolidWorks documents
(parts .sldprt, assemblies .sldasm, and drawings .slddrw).
CAD kernels use it to exchange data with SolidWorks files. Martensite is its first consumer.

The first implemented layer reads SolidWorks version-4 containers, extracts streams
with size and CRC checks, and replaces existing stream contents. A save without
edits preserves the whole file byte for byte. An edited save preserves every
untouched stream's compressed bytes. `bessemer::block_stream` also reads and writes
the compressed-block carriers used by body partitions and add-ins, with explicit
framing and expansion limits. `bessemer::history` reads and writes standalone version
histories, including typed stamp edits and shared date-code objects.
`bessemer::header` reads, constructs, edits, and writes header archives, including
revision logs, shared external references, and version-dependent stored fields.
`bessemer::biography` reads and writes product-build and environment records,
preserving their raw values and offering explicit repeated-string expansion.
`bessemer::parasolid` reads and writes binary transmit partitions and rollback
deltas using explicit base schemas, preserving every typed field and raw flag.
`bessemer::tessellation` reads and writes complete face tessellation directories,
blocks, and facet records, including older documented layouts and explicit
full/lightweight context.
`bessemer::metadata` reads and writes all documented XML metadata families, with
namespace-aware property views, exact source framing and isolated text/attribute edits.
It also supports embedded `ThirdPty/InspectionXpert` projects and complete Job Number edits.
`bessemer::preview` reads and writes packed DIB previews, decodes RLE8 and BGRX
pixels, and supports isolated palette and pixel replacement.
It also reads, constructs, edits, and writes static PNG previews, preserving stored
chunks and exact samples across all defined color/depth pairs and Adam7 interlace.
`bessemer::display_lists` currently provides lossless compact/fixed transform and
versioned named, annotation and 3D-view codecs, plus part/assembly archive reading,
validated edits and writing with inline facets, explicit external face references
and reference geometry, scenes, display items, view buckets, graphics references,
markup-ink grids, per-configuration data, decals, null-bitmap textured planes and
lights for unsigned display versions, including zero, outside the
uncounted-window interval
`0x1FAE`–`0x1FCE`, with an explicit
native file-version context. Tangent bucket lists follow that context independently
of the display-list version.
`edit_with_history` returns a validated snapshot with exact archive coverage,
structural field history and explicit geometry/external-link check exclusions.
Early PW scene objects in bucket thread keys retain their versioned wall records
and background suffixes.
Before `0x117F`, scene walls support null legacy-material pointers; non-null
material bodies remain explicitly unsupported.
All 1,086 reference-readable part/assembly display-list streams reconstruct
exactly. The [display-list acceptance review](docs/features/display-lists/acceptance-review.md)
records supported layouts, explicit unresolved cases and the integration gate.
Drawing `Contents/VBLists` supports complete reading, construction, isolated edits
and writing for unsigned model versions from `0x27CC` onward. All 20 native
drawing streams
reconstruct exactly, including 33 shaded models; see the
[drawing bucket evidence](docs/features/drawing-buckets/README.md). Drawing
`Contents/DisplayLists` remains unresolved in the reference. High unsigned markers
follow the [documented version predicates](docs/features/display-lists/unsigned-versions.md).

Complete part, assembly, and drawing model archives, geometric interpretation and
validation, and semantic CAD editing remain to be implemented; this is not yet
Corliss feature parity.

```rust
let bytes = std::fs::read("part.sldprt")?;
let mut doc = bessemer::Document::from_bytes(&bytes)?;
let preview = doc.read_stream_limited("PreviewPNG", 16 * 1024 * 1024)?;
doc.replace_stream("PreviewPNG", preview)?;
std::fs::write("copy.sldprt", doc.to_bytes()?)?;
```

Run `cargo test` for the owned native fixtures and malformed-input tests.
Test builds use optimization level 1 while retaining debug assertions and integer
overflow checks; see the [test profile measurements](docs/features/model-archives/test-performance.md).
`cargo xtask pr` runs formatting, clippy, build, tests, and documentation examples.
Run `cargo fmt --all -- src/model_archives/test_modules.rs` to format the private
model suites as well; both validation gates check that included registry explicitly.
`cargo xtask full` currently runs those same gates across the CI platform matrix.
The Linux full check also runs `cargo xtask tsan` with the pinned nightly and
`rust-src`, instrumenting dependencies and the standard library as well as Bessemer.
See [container evidence and remaining work](docs/features/container/README.md).
The [compressed-block evidence](docs/features/compressed-blocks/README.md) records
the implemented carrier contract and its verification limits.
See [version-history evidence](docs/features/version-history/README.md) for archive
coverage and the installed-corpus verification.
See [header archive evidence](docs/features/header-archives/README.md) for field
coverage, version boundaries, graph preservation, and remaining limits.
The [biography evidence](docs/features/biography-archives/README.md) covers stored
environment observations and repeated-string reading and writing.
See [Parasolid transmit evidence](docs/features/parasolid-transmit/README.md) for
schema inputs, field coverage, complete-corpus reconstruction, and acceptance limits.
The [face tessellation evidence](docs/features/face-tessellations/README.md) records
version coverage, directory links, independent vectors, and native corpus checks.
See [DIB preview evidence](docs/features/dib-previews/README.md) for image formats,
pixel semantics, limits, and independent decoder checks.
The [Inspection project evidence](docs/features/inspection-projects/README.md) covers
embedded add-in XML, all observed paths and complete drawing Job Number edits.
The [XML metadata evidence](docs/features/xml-metadata/README.md) covers all 17
stream families, namespace-aware property views, lexical edits and corpus checks.
The [PNG preview evidence](docs/features/png-previews/README.md) covers chunk
preservation, ancillary copy rules, full pixel formats, and corpus verification.
The [part configuration API](docs/features/model-archives/public-api.md) provides
complete ordinary modern-part reading, typed construction, isolated edits and
writing with exact coverage and field history. The [codec evidence](docs/features/model-archives/plan.md)
records independent fixtures and reconstruction of 906 native configurations.

The [resolved-feature API](docs/features/resolved-features/public-api.md) reads
and edits supported complete separate feature streams with their inherited
model map, CMgr helper state and revision-bound field history. Supported
[detail folders](docs/features/detail-folders/README.md) include their graphics
buckets and display-item payloads. Supported
[reference axes](docs/features/reference-axes/README.md) retain their derived
data and shared references. Documented
[line-reference fields](docs/features/line-reference-roles/README.md) accept
both ordinary lines and temporary-axis derivatives. Unsupported layouts fail
explicitly; full SolidWorks schema parity remains in progress.

[Mirror patterns](docs/features/mirror-patterns/README.md) reuse the common
pattern codec and retain tagged/direct plane fields and shared face references
through complete reads, isolated edits and reconstruction.

[Shell features](docs/features/shell-features/README.md) retain all six tagged
references, full flag bytes and unequal list lengths through the same API.

[Circular patterns](docs/features/circular-patterns/README.md) preserve shared
dimensions, ordered integer collections and versioned geometry tails.

[Hole Wizard features](docs/features/hole-wizard/README.md) preserve parameter
strings, compatibility words, shared dimensions and direct slot-axis bodies.

[Imported and base-body features](docs/features/imported-bodies/README.md) retain
external body names, explicit typed inline partitions and signed vectors.

[Extruded reference surfaces](docs/features/extruded-surfaces/README.md) retain
ordered-face groups, required end specifications and concrete keeper identities.

[Ordinary-part sweeps and sweep cuts](docs/features/sweeps/README.md) preserve conditional
dimensions, versioned controls and complete shared reference dependencies.

[Toolbox specifications](docs/features/configuration-toolbox/README.md) retain
shared metadata items, raw string framing and exact floating-point bits.

[Dome features](docs/features/domes/README.md) preserve nullable dimensions,
tagged lists, shared geometric references and raw signed words.

[Reference-axis owners](docs/features/reference-axis-owners/README.md) retain
verified axis self references in populated atom choosers.

[Component geometry](docs/features/component-geometry/README.md) preserves
component-axis and component-point identities, geometry and feature ID/time words.


This library is named after Sir Henry Bessemer (1813–1898) an English engineer, inventor, and entrepreneur 
best known for developing the Bessemer process, the first inexpensive industrial method for the mass production
of steel. Patented in 1856, his technique revolutionised metallurgy and construction by blasting air through 
molten pig iron to rapidly oxidise impurities, dramatically lowering the cost and time required to produce 
high-tensile steel compared to traditional wrought-iron methods. 
Beyond his landmark achievement in steelmaking, Bessemer was a prolific inventor with over 100 patents to his 
name, contributing innovations in fields ranging from steamship design and sugarcane extraction to solar 
furnaces and the manufacture of bronze powder. His pioneering process laid the material foundation for the 
Second Industrial Revolution, enabling the modern expansion of railways, bridges, ocean liners, 
and skyscrapers worldwide.

Display archives provide `object_handle` and `resolve_object` for revision-scoped
object lookup. Clones and equivalent edits retain handles; changed edits and
fresh decodes require new handles. See the [identity contract](docs/features/display-lists/revision-identities.md).

Display-list CObject graphs also retain shared line-style definitions and versioned layer maps; see the [UI object coverage](docs/features/display-lists/ui-objects.md).

Component-property objects and holders retain their embedded fonts, visibility fields, occurrences and shared references; see the [component-property coverage](docs/features/display-lists/component-properties.md).

`DisplayArchiveContext::from_document` resolves exact display-version markers and external component declarations with explicit native file and tessellation versions. See [document context resolution](docs/features/display-lists/document-context.md).

`DisplayArchive::check_external_face_links` checks named cache blocks and resolves requested external facets with explicit coverage. See [external face links](docs/features/display-lists/external-face-links.md).
