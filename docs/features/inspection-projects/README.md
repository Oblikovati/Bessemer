# Embedded Inspection projects

`MetadataStream::parse("ThirdPty/InspectionXpert")` selects the
`InspectionProject` family and validates the unqualified `ExtInspectionProject`
root. The existing `XmlMetadata` codec reads, constructs, edits and writes its
complete lexical source and namespace-aware tree. This is Inspection carrier
behavior version 1, tracked by #26; XML parsing behavior remains version 1.

[Clause 14.4](specification-14.4.md) and the [233-path inventory](inspection-schema.md)
are snapshots from edition 0.128 of the approved `../inventor-interface` working
tree, based on commit `dd0537b3a142d8dda00ef846891efc736a6aac46` with evolving
uncommitted documentation. This stream belongs to an add-in and is separate from
the 17 core XML families in clause 9.4.

Every element, attribute, namespace binding, stored value, empty collection and
unknown extension follows the [XML metadata contract](../xml-metadata/README.md).
UTF-8 BOM framing, declarations, whitespace, escapes and comments retain their
source spelling. The carrier has no propertySection adapter: Inspection
ExtProperty collections remain visible through their actual XML elements.
An empty payload fails; a valid root with empty children remains a document.

`GUID` can contain `SOLIDWORKS Inspection`, so it is not parsed as a UUID. Version
and MajorVersion stay add-in strings and do not select a binary archive version.
Cached Path and Name retain the originating blank drawing across Save As. No
current-filename substitution is performed. Option numbers, units, enum domains,
regexes and boolean-looking text stay uninterpreted. Nonempty characteristics and
referenced-model structures remain open in OI-019; retaining their XML does not
establish their application semantics.

## Complete Job Number edit

```text
cargo run --example set_inspection_job -- drawing.SLDDRW "JOB-001" edited.SLDDRW
```

The example selects one unqualified Settings child and one JobNumber child, then
uses `with_leaf_text` and `Document::replace_stream`. Missing or ambiguous matches
fail explicitly. The output retains every other XML element and every other
container stream. JobNumber is the native mapping established by the supplied
single-change experiment; the example does not infer other settings mappings.
The native experiment also changed cached collections, so the two supplied files
are independent baselines rather than a byte-isolated before/after pair.

## Evidence and reproduction

The two native drawings were created in SolidWorks Design Premium 2026 SP2.1 on
2026-10-24, through New Inspection Project with `default.swidot`, then Edit Project
with Job # = `SCHEMA-JOB-001`. They contain no extracted characteristics. They are
additional fixtures and do not change the installed core-corpus denominator.
[verification.json](verification.json) records their complete file and payload
SHA-256 values, byte counts and independent element counts.

The 15,823-byte blank project has 353 elements. The 14,480-byte Job Number project
has 305. ElementTree independently generated parent, namespace, name, direct text,
attribute and ordered-child oracles for every element. Their union matches all
233 inventory paths. Forced reconstruction preserves both complete XML payloads;
whole-drawing tests verify selected-leaf isolation, every unrelated stream,
invalid-edit isolation, native cache distinctions and authentic byte-exact reverts.
Eight rounds of eight concurrent edits match serial bytes without source mutation.

On Windows x64 with Rust 1.99.0 MSVC, the shipped example ran twice for each native
drawing. Each pair produced identical document bytes. An independent Python
container reader and ElementTree confirmed the complete edited trees and exact
compressed bytes of every untouched stream. Wrong CLI argument counts failed.

```text
python -X utf8 docs/features/inspection-projects/generate_oracles.py
cargo build --release --example set_inspection_job
python -X utf8 docs/features/inspection-projects/verify_example.py target/pinned-1.99.0/release/examples/set_inspection_job.exe
cargo xtask pr
```

The Python checks require the approved sibling reference reader; the Rust tests
embed their owned fixtures and oracles and require no sibling checkout or native
installation. No dependencies were added. The full CI matrix covers Linux x86_64,
Windows x64 and both macOS architectures, including Linux ThreadSanitizer.
Native application acceptance of Bessemer-edited Inspection projects remains
unverified. The maintainer-side humanizer skill was unavailable; its prose pass
remains for review under the contributor guide.
