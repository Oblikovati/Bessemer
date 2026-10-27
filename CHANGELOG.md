# Changelog

## [Unreleased]

### Added

- Added ordinary fold-folder, transform and property reading, field editing and writing with typed matrix extensions, ordered shared property pairs and direct byte arrays. (#54)

- Added ordinary flat-pattern reading, field editing and writing with typed raw compressed bodies, direct bounding boxes, shared references and ordered duplicate pair keys. (#54)

- Added ordinary base-flange reading, field editing and writing with selected directions, shared sketch-handle collections and preserved bend-group records. (#54)

- Added ordinary sheet-metal and template reading, field editing and writing with validated saved-state handoff, strided allowance payloads, fixed-face framing and complete component references. (#54)

- Added ordinary component coordinate-system reading, field editing and writing with shared geometry, preserved fixed-transform words and explicit special-kind errors. (#54)

- Added ordinary coordinate-system reading, field editing and writing with equality-selected axes, direct axis arrays, shared identities and preserved raw selector/vector values. (#54)

- Added ordinary feature-comment reading, field editing and writing with direct lists, shared text items, versioned extra members and preserved four-byte time values. (#54)

- Added ordinary connection-point and route-point reading, field editing and writing with preserved direct parameters, compatibility fields, shared references and complete radial-dimension dependencies. (#54)

- Added ordinary reference-point reading, field editing and writing with all seven point-data layouts, required tagged data, shared typed references and independent complete-stream fixtures. (#54)

- Added ordinary component-axis and component-point reading, field editing and writing with shared inherited layouts, typed geometry pointers and independent wrapper/whole-stream fixtures. (#54)

- Added ordinary part dome reading, field editing and writing with independent nullable pointers, tagged lists, shared geometric references and complete-stream edit fixtures. (#54)

- Added ordinary Toolbox specification arrays and items with preserved shared references, nonfinite floating-point bits, raw strings and independently verified Unicode edits. (#54)

- Added ordinary part sweep and sweep-cut reading, field editing and writing with versioned controls, conditional dimensions, shared references and complete endpoint, wire-contact and loft-synchronization dependencies. (#54)

- Added ordinary part extruded reference-surface reading, field editing and writing with required end specifications, ordered-face groups, preserved shared references and context-checked binary-empty keepers. (#54)

- Added ordinary part imported and base-body reading, field editing and writing with preserved external markers and names, explicit typed inline kernel frames and signed base-body vectors. (#54)

- Added ordinary part Hole Wizard reading, field editing and writing with compatibility gates, preserved raw parameters, direct slot-axis bodies, shared dimensions and independent Unicode-edit fixtures. (#54)

- Added ordinary part circular-pattern reading, field editing and writing with required dimension alternatives, preserved ordered integer collections, versioned geometry tails and independent complete-stream fixtures. (#54)

- Added ordinary part shell-feature reading, field editing and writing with preserved unequal lists, shared object references, complete dimension and component dependencies and independent full-stream fixtures. (#54)

- Added ordinary part mirror-pattern reading, field editing and writing with shared pattern bodies, tagged and direct plane references, exact inherited object maps and independent complete-stream fixtures. (#54)

- Added ordinary part reference-axis reading, field editing and writing with eight axis-data layouts, required typed data pointers and preserved shared references and floating-point bits. (#54)

- Added ordinary part detail-folder reading, editing and writing with shared graphics bucket and display-item codecs, typed BOM references and independent reconstruction fixtures. (#54)

- Added complete supported ordinary part resolved-feature reading, isolated edits and writing with explicit inherited model context, exact stored-field coverage, revision-bound history and geometry exclusions. (#54)

- Added complete ordinary modern-part configuration reading, typed construction, isolated edits and writing with explicit header/kernel context, exact stored-schema coverage and revision-bound field history. (#44)

- Added validated display-archive edit outcomes with completion, exact stored-field fidelity, validation coverage, explicit exclusions and revision-bound structural field history. (#28)

- Added complete drawing `Contents/VBLists` reading, validated edits and writing with unsigned model-version context, shared object graphs and shaded part/assembly models. (#38)

- Added exact display-to-face-cache link checks with borrowed facets, explicit coverage and typed failures for missing, invalid or ambiguous targets. (#28)

- Added lossless display-list transform and view codecs, plus part/assembly archive reading, validated edits and writing for unsigned display versions including zero outside the uncounted-window interval `0x1FAE`–`0x1FCE`, with shared object identities, revision-scoped object handles, typed whole-document context resolution, shared line-style, layer-manager and component-property graphs, direct and wrapped body-property maps, inline facets, external face references, all seven reference-geometry kinds, versioned appearance and scene layouts, display-item payloads, versioned view buckets with explicit native file-version context and gated tangent lists, graphics-reference graphs, markup-ink grids, per-configuration data, decals, legacy wall records with null material pointers, textured-plane records with null bitmap pointers and all five old-light classes. Unresolved bodies and window-dependent layouts returned typed failures without opaque fallback. (#28)

- Added embedded SolidWorks Inspection project XML reading, construction and writing, with preserved unknown content and a complete drawing Job Number edit example. (#26)

- Added lossless namespace-aware XML metadata reading, construction, section-scoped property views, immutable text and attribute edits, and writing for all 17 documented stream families, with bounded parsing and preserved Unicode and keyword framing. (#22)

- Added static PNG preview reading, typed raster construction, sample editing, and writing, with exact chunks, CRC and zlib checks, all color/depth pairs, Adam7, bounded metadata expansion, and ancillary copy rules. (#20)
- Added packed DIB preview reading, construction, palette and pixel editing, and writing, with checked RLE8 and BGRX decoding, retained raw fields, explicit unwritten pixels, and allocation limits. (#18)
- Added typed face tessellation directory, block, and facet reading, construction, editing, and writing, preserving version-dependent fields, raw arrays, exact floating-point bits, and component links. (#16)
- Added typed Parasolid binary transmit partition and delta reading, construction, field editing, and writing, with explicit base schemas, both byte orders, layout edits, pointer flags, and exact floating-point bits. (#14)
- Added typed SolidWorks biography archive reading, construction, record editing, and writing, with retained environment observations and explicit repeated-string expansion and abbreviation. (#12)
- Added typed SolidWorks header archive reading, construction, graph editing, and writing, preserving shared external references, unknown fields, embedded version overrides, and exact bounding-box bits. (#10)
- Added typed standalone SolidWorks version-history reading, graph editing, and writing, preserving shared date-code objects, unknown fields, and equivalent-edit byte identity. (#8)
- Added a Linux x86_64 ThreadSanitizer gate that instrumented the library, dependencies, and standard library in the full CI gate. (#8)
- Added compressed-block carrier decoding and chunk replacement with explicit framing, expansion limits, zlib integrity checks, and preservation of untouched chunk encodings. (#4)
- Added SolidWorks version-4 container reading, checked stream extraction, and stream replacement that preserved untouched compressed bytes. (#1)

### Changed

- Optimized test builds while retaining debug assertions, checked integer arithmetic and every exhaustive validation case. (#53)

### Fixed

- Accepted documented coordinate-system targets through shared component geometry pointers while retaining unrelated-class rejection. (#46)

- Fixed populated reference axes being rejected as inherited atom owners while retaining strict checks for unrelated classes and aliases. (#46)

- Fixed private model-test formatting coverage in local and CI gates after declarations moved behind an included registry. (#69)

- Fixed resolved-feature failures mislabeled as configuration operations and streams, retaining their failure classes, causes and CMgr dependency locations. (#64)

- Fixed valid temporary-axis references rejected by revolution, reference-plane and sketch line fields while retaining finite declared-type checks. (#46)

- Corrected model errors to identify the selected configuration instead of a hardcoded configuration zero. (#47)
- Reported the first mismatched model field, expected shape and bounded object-identity details during construction. (#52)
- Accepted all five complete display-dimension variants in sketch-loop slots while retaining unrelated-class rejection. (#46)

- Restored early PW scene layouts reached through generic bucket thread keys, preserving implicit wall keys, compact wall records and versioned background suffixes. (#41)

- Retained the temporary simple-text float in early display archives, including signaling NaN bits and validated edits. (#37)

- Restored inline part configuration tessellation before display version `0x2B67` and retained the short-lived assembly annotation-visibility prefix. (#36)

- Preserved the original file when a stream replacement matched its original content, including after an intermediate edit. (#5)
