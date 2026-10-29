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

[Reference points](docs/features/reference-points/README.md) retain all seven
documented data variants, required data tags and shared geometry references.

[Connection and route points](docs/features/connection-points/README.md) retain
direct configurable parameters, compatibility values and shared line/dimension references.

[Feature comments](docs/features/feature-comments/README.md) preserve direct lists,
shared text items, raw kinds, ordered extra members and four-byte time values.

[Coordinate systems](docs/features/coordinate-systems/README.md) retain numeric
axis order, equality-based selectors, direct arrays and exact vector bits.
Their [shared component geometry roles](docs/features/component-coordinate-systems/geometry-roles.md)
also accept documented fresh and cached coordinate-system targets.
[Component coordinate-system references](docs/features/component-coordinate-systems/README.md)
retain their inherited feature fields, geometry identities and fixed transform words.
[Base flanges](docs/features/base-flanges/README.md) preserve selected directions,
shared sketch handles and direct bend-group records.
[Flat patterns](docs/features/flat-patterns/README.md) preserve direct bounding boxes,
ordered pairs and typed raw compressed kernel bodies.
[Fold folders and transforms](docs/features/fold-transforms/README.md) preserve
typed transforms, shared property records and direct byte arrays.
[Blends and blend cuts](docs/features/blends/README.md) preserve independent counts,
numeric columns and conditional direction and dimension references.
[Reference-plane point roles](docs/features/plane-point-roles/README.md) accept
the documented point-reference family while retaining unrelated-class rejection.
[Weldment features](docs/features/weldments/README.md) preserve direct custom-property
lists, duplicate records and original stored expressions.
[Structural members](docs/features/structural-members/README.md) preserve full-width
point metadata, signed contour modes and complete shared contour/index graphs.
[Reference dimensions](docs/features/reference-dimensions/README.md) preserve entity
kind words, optional vectors, complete sketch handles and direct dimension planes.
[Radiated-edge identifiers](docs/features/radiated-edge-identifiers/README.md)
preserve inherited references and shared entity-representation identities.
[End-specification point roles](docs/features/end-spec-point-roles/README.md)
accept the verified point family in extrusion and revolution ends.
[Spline controls](docs/features/spline-controls/README.md) retain direct handle
records and complete sketch user-object identities.
[Shared sketch handles](docs/features/shared-sketch-handles/README.md) support
documented parabola and parametric-surface handles through the existing codec.
[Sketch geometry backups](docs/features/sketch-geometry-backups/README.md) preserve
parabola segment prefixes, raw cylinder records and their shared handle types.
[Draft features](docs/features/drafts/README.md) preserve signed presence words,
direct identifier keepers, strided payloads and counted face/point collections.
[Draft surface identifiers](docs/features/draft-surface-identifiers/README.md)
retain full signed words and distinct identifiers with shared feature references.
[Face-blend items](docs/features/face-blend-items/README.md) preserve pair identity,
typed face/edge references and optional point bits.
[Revolved surfaces](docs/features/revolved-surfaces/README.md) retain inline
surface bodies, shared angular dimensions, data words and keeper identity.
[Helices](docs/features/helices/README.md) preserve typed kernel wires, variable
pitch arrays, conditional dimension tables and wire-edge identifier identity.
[Component-reference curves](docs/features/component-reference-curves/README.md)
retain shared unit-component targets, wrapper aliases and signed ID/unsigned time pairs.
[Cut-list folders](docs/features/cut-list-folders/README.md) retain inherited
folder layers, direct properties, raw flags and shared tagged references.
[Weld corners](docs/features/weld-corners/README.md) retain independent arrays,
complete trimming data, raw scalars and the versioned late word.
[Solid references](docs/features/solid-references/README.md) retain wrapper
identity, signed body presence, typed kernel bodies and direct attributes.
[Mirrored solids](docs/features/mirror-solids/README.md) retain independent
planes and arrays, shared matrix helpers and unsigned-array records.
[Combine and delete bodies](docs/features/combine-bodies/README.md) retain
complete payloads, shared body targets and raw versioned options.
[Thickening](docs/features/thickening/README.md) retains shared dimensions,
versioned directions and complete constant/component surface-reference layers.
[Surface cuts and trims](docs/features/surface-operations/README.md) retain
direct chooser ownership, allocated-data wire members and shared references.
[Trim-edge identifiers](docs/features/trim-edge-identifiers/README.md) retain
complete source representations and direct entity-ID members for both sides.
[Trim-wire and pattern-body edge identifiers](docs/features/frontier-edge-identifiers/README.md)
retain native-bound layouts, shared references and direct entity-ID members.
[Lock/environment features and pattern/fillet helpers](docs/features/frontier-feature-helpers/README.md)
retain inherited bodies, raw item bits and unqualified radius-object pointers.
[Net blends and component groups](docs/features/net-blends/README.md) retain
direct curve-set bodies, independent point counts, shared contacts and thin dimensions.
[Sketch external references](docs/features/sketch-wrapper-roles/README.md) accept
the existing finite wrapper family through fresh and cached pointers.
[Sew-reference surfaces](docs/features/sew-surfaces/README.md) retain five direct
arrays, raw flag words, double order and inherited empty keepers.
[End-specification wrapper roles](docs/features/end-spec-wrapper-roles/README.md) accept
the reviewed wrapper family at conditional, starting-end and revolution fields.
[Sketch-intersection wrapper roles](docs/features/intersection-wrapper-roles/README.md)
accept the same reviewed family at all four fields through fresh and cached references.
[Valued-dimension fixed transforms](docs/features/fixed-valued-transforms/README.md)
retain the full presence word and thirty raw archive words before the derived suffix.
[Additional valued dimensions](docs/features/additional-valued-dimensions/README.md)
retain native spline-tangent, arc-length and plane-distance identities in shared collections.
[Sketch isocurve information](docs/features/sketch-isocurve-information/README.md)
retains twenty-byte numeric records, raw ratio bits and shared external-entity references.
[Deleted-face features](docs/features/deleted-faces/README.md) retain independent
nullable arrays, unsigned DWORDs and shared nested data.
[Composite curves](docs/features/composite-curves/README.md) retain strict kernel
frames, shared tagged arrays and independent direct object/DWORD arrays.
[Ruled surfaces](docs/features/ruled-surfaces/README.md) retain direct owned data,
inline surface frames, shared dimensions and independent WORD/object arrays.
[Thin extrusion and revolution features](docs/features/thin-extrusions/README.md) retain
shared specifications, independent dimensions and fixed-width bend triples.
[Three-point plane references](docs/features/three-point-plane-roles/README.md) accept
the reviewed point family at all three fields while retaining shared identities.
[Constraint-plane families](docs/features/constraint-plane-families/README.md) retain
complete data/interface/geometry bodies, shared targets and raw/native flag views.
[Projected-line subtypes and surface wrappers](docs/features/projected-line-subtypes/README.md)
retain required owners, nullable arrays, tagged lists and exact floating-point bits.
[PLine silhouette edge identifiers](docs/features/silhouette-edge-identifiers/README.md)
retain direct source identities and both signed suffix words.
[Split features and items](docs/features/split-features/README.md) retain direct
owned arrays, positive kernel bodies, fixed transforms and separate raw/native map views.
[Sweep-reference surfaces](docs/features/sweep-reference-surfaces/README.md) retain
signed edge sequences, direct sweep data and independent conditional dimensions.
[Copy, trim/extend and reference edge identifiers](docs/features/copy-trim-edge-identifiers/README.md)
retain direct source IDs, native map accounting and signed words.
[Filled surfaces and curve contacts](docs/features/filled-surfaces/README.md) retain
direct keeper arrays, one-byte fields and ordered contact references.
[Offset surfaces](docs/features/offset-surfaces/README.md) retain complete inline
surface payloads, required data, signed array presence and empty keepers.
[Reference curves](docs/features/reference-curves/README.md) retain complete
body-feature layouts, cached-array branches and versioned kernel-body lists.
[Component curves and blend-reference surfaces](docs/features/blend-reference-surfaces/README.md)
retain exact native spellings, shared direct blend data and raw surface bytes.
[Sheet metal](docs/features/sheet-metal/README.md) preserves saved helper state,
direct allowances, raw strided payloads and fixed-face references.


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

Cone-reference framing and its surface-axis callers are documented in
[cone references](docs/features/cone-references/README.md).

[Surface identifier variants](docs/features/surface-identifier-variants/README.md)
describes exact top-surface and end-cap chamfer representation framing.

[Display welds](docs/features/display-welds/README.md) covers the complete
display-item base, native weld suffix and preserved payload identities.

[Component-detail references](docs/features/component-detail-references/README.md)
documents exact detail bodies and verified current-part ownership.

[Weld symbols](docs/features/weld-symbols/README.md) documents constructor-bound
direct texts, surface-finish gates and cosmetic attachment subtype identities.

[Attachment records](docs/features/attachment-records/README.md) distinguishes
attachment data from inherited parametric attachment points.

[Cosmetic weld cut lists](docs/features/cosmetic-weld-cut-lists/README.md)
documents the complete reference-curve parent and native suffix order.

[Cosmetic weld tree](docs/features/cosmetic-weld-tree/README.md) covers direct
endpoint ownership, bead kernel bodies and signed wire-map lists.

[End caps](docs/features/end-caps/README.md) documents the complete body-feature
parent, native pointer order and preserved shared dimensions.

[Weldment finishing](docs/features/weldment-finishing/README.md) covers complete
gusset extrusions, ordinary weld-bead features and shared side data.

[Absolute plane data](docs/features/absolute-plane-data/README.md) documents the
shared non-default plane layout and its typed containing-plane role.

[Edge flanges](docs/features/edge-flanges/README.md) covers complete edge-flange,
single-bend and per-edge records, including verified saved sheet-metal state.

[Sheet-metal folds](docs/features/sheet-folds/README.md) documents complete 3D bends,
folds and unfolds, including raw compressed bodies and ordered maps.

[Sheet-metal corners and hems](docs/features/corner-hems/README.md) covers directly
owned hem data, independent arrays, chain identities and saved helper layouts.

[UI bends](docs/features/ui-bends/README.md) covers both saved layouts, direct ruled
lines, signed count preservation and versioned sketch-line records.

[Flatten and process bends](docs/features/flatten-bends/README.md) covers containing
node flags, signed presence and strict raw bodies.
[Feature states](docs/features/feature-states/README.md) documents the scoped
two-word state records and independent edits.

[Jogs and break corners](docs/features/jog-corners/README.md) covers stored presence,
direct owned keepers, tagged arrays and independent version gates.

[Version-18000 configuration managers](docs/features/legacy-owned-managers/README.md)
documents explicit dialect selection and independently decoded historical streams.

[Display notes](docs/features/display-notes/README.md) covers internal note components,
generic text, direct paragraphs and shared text-list identities.

[Stock features](docs/features/stock-features/README.md) covers stock and mirrored-stock
records, body-reference collections and component-body transforms.

[Derived-feature references](docs/features/derived-feature-references/README.md)
cover versioned point presence, direct shared arrays and surface-body aliases.

[Move/copy bodies](docs/features/move-copy-bodies/README.md) cover nested payloads,
mesh tolerances and the documented conformal-deformation branches.

[Selection sets](docs/features/selection-sets/README.md) cover node-based sets,
selection items, direct lists and shared comment and wrapper identities.

[Sketch bitmaps](docs/features/sketch-bitmaps/README.md) cover feature and data
records, bounded raw byte planes and conditional compact transforms.

[Sketch bends](docs/features/sketch-bends/README.md) cover complete inherited
One Bend records, tagged stored handles and explicit kind-7 dependency stops.

[Mitered flanges](docs/features/mitered-flanges/README.md) cover direct object and
DWORD collections and four sketch-associated handle groups.

[Stored curve files](docs/features/curve-files/README.md) cover inherited kernel
wires, verbatim filenames, signed point counts and bit-preserving coordinates.

[Extended surfaces](docs/features/extend-surfaces/README.md) cover inline surface
bodies, direct lists, empty keepers and end-condition-dependent pointers.

[Punch records](docs/features/punches/README.md) cover internal revisions, bounded
dynamic arrays, ordered extension tags and legacy direct keeper data.

[Body extensions](docs/features/body-extensions/README.md) cover cavities,
transformed stock and sketch holes through their complete inherited bodies.

[Feature edits](docs/features/feature-edits/README.md) cover move-face records,
compact transforms and bending data with direct arrays and packed raw words.

[Mid-surfaces](docs/features/mid-surfaces/README.md) cover inline and external
sheet framing, directly owned keepers, face arrays and shared face/edge pairs.

[Manager scalars](docs/features/manager-scalars/README.md) cover sheet-metal
feature-ID pairs and custom material values in their complete CMgr maps.

[Standalone Inspection envelopes](docs/features/standalone-inspection/README.md)
cover current encrypted templates and historical projects, exact XML preservation
and immutable complete-XML replacement.
