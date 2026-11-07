# Composer XML/geode identifiers

Feature [#104](https://github.com/Oblikovati/Bessemer/issues/104) implements the
complete local equality operation in clause 14.91 / C-467. It indexes project
XML against a complete version-7.9 geometry snapshot without running external
resolution, loading, cloning, merge or procedural-redline branches.

`ComposerXmlLinks::from_bytes(xml, &scene, limits)` requires UTF-8 with optional
BOM, an unqualified `DocumentSMG` root and stored `Version` text `7.9` or
`7.900000`. Malformed XML and DTD/entity declaration markers fail. The existing
metadata grammar validates all XML, resolves namespaces and normalizes attributes
according to XML 1.0. It never loads external declarations or resource paths.
The same lossless lexical attribute editor supplies isolated reference edits.

Every `IdentGeom` occurrence remains separate, in element preorder. References
retain child-element ordinal paths, expanded element names, all ordinary
attributes, exact parsed identifier text and every equality candidate. Namespace
declarations are not ordinary attributes. Comments and processing instructions
remain in the original bytes and do not increment child-element ordinals.

Matching uses nonempty raw UTF-16 binary GUID keys exactly. It performs no
trimming, case folding, Unicode normalization, UUID parsing or URI stripping.
Empty or absent binary keys produce unkeyed geodes. Empty XML values remain
empty. Nonempty references can be unmatched, unique or ambiguous. Duplicate
GUIDs retain every candidate; a unique equality candidate does not establish
the native runtime's final choice. Unreferenced keyed geodes retain first-seen
key-group order and candidate order within each group.

Candidates contain a typed scene revision and original top-level entry index,
including any intervening top-level faces. XML occurrence IDs belong to one
immutable index revision. Original XML bytes, unknown nodes, whitespace,
comments, quoting and BOM are preserved. Published results own the XML and
immutable identifier catalog; reads take no lock and all public types are
`Send`/`Sync`.

`edit_reference(&id, value)` changes one existing unqualified `IdentGeom`
attribute, retains its quote/spacing and escapes the requested parsed text. It
then reparses the complete XML and rebuilds every equality result against the
same geometry revision. No-op edits retain identity; changed results reject old
IDs. Completion, exact attribute-text fidelity, occurrence history, complete
coverage and runtime/project exclusions are returned together.

`ComposerLinkLimits` carries the metadata engine's byte, event, attribute and
depth bounds, plus geode count, reference count and cumulative reference-path
units. Candidate groups share immutable storage across repeated references,
including ambiguous references, so repeated matches do not copy every candidate
list or require a quadratic visited-geode walk.

Eleven owned complete binary-scene/XML pairs cover case/whitespace distinctions,
repeated references, duplicate/empty/missing GUIDs, group ordering, namespace
attributes, root references, comments/PIs/BOM, Unicode and unpaired binary
surrogates, qualified references and a completed empty index. The expected
binary oracle retains every reference projection field from actual frozen
`composerxmllinks.py`. A literal independent edit golden pins lexical fidelity;
special-character edits check exact decoded values after XML escaping.

The installed audit covers all 38 XML members, 21,529,648 XML bytes and 1,787
binary geodes. All 4,445 occurrences match the complete reference projections:
4,416 unique local candidates and 29 unmatched references. Every binary geode
is referenced in this corpus. These observations do not impose uniqueness or
local-only references on other files.

The native edit gate changes one reference in every project and records its
complete reindex. `verify_native_edits.py` uses Expat element offsets and a
separate attribute-span check to require exactly the intended byte replacement.
It then decodes each complete binary scene and XML through actual frozen
readers and compares every edited index field with the Rust result.

Run `python docs/features/composer-links/freeze_links.py` to replay retained
baselines. Add `--export` for the local Rust native packet, then run
`cargo test --test composer_links_native -- --ignored`. Run
`verify_native_edits.py` after that test. Use `--write` only for reviewed baseline
updates. Every source import uses an immutable copied tree with retained hashes
and origin checks. Native runtime resolution, graph side effects, complete
actor/property/project schemas and application acceptance remain separate work.

The maintainer-side humanizer skill was unavailable. A maintainer still needs
to review this prose.
