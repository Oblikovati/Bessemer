# XML metadata

`bessemer::metadata` reads, constructs, edits and writes all 17 normalized XML
stream families in [clause 9.4](specification-9.4.md). This is XML metadata behavior
version 1, tracked by #22. The [observed inventory](xml-metadata.md) records 219
element paths; occurrence counts are evidence, not mandatory cardinality rules.
These snapshots came from edition 0.127 of the approved `../inventor-interface`
working tree, based on commit `dd0537b3a142d8dda00ef846891efc736a6aac46` with evolving
uncommitted documentation. Corliss's lexical XML preservation informed this design.

The immutable model identifies every element and ordinary attribute by namespace
URI and local name. Prefixes remain in the exact source. Explicit default-namespace
resets produce unqualified names; default namespaces never qualify attributes.
All children, unknown extensions, attribute values and direct character content
remain accessible in document order. Comments, processing instructions, CDATA,
quotes, spacing and entity spellings remain in the lexical source and raw element
spans. Element identities belong to one unchanged snapshot. A changed source
creates a fresh identity context; stale and cross-document IDs fail explicitly.

UTF-8 and both UTF-16 byte orders support preserved BOM framing and checked
encoding declarations. The KeyWords leading byte is retained as opaque framing;
its meaning is not inferred. Completed empty MaterialTree, CustomProperties summary
and Tables streams are distinct from an empty XML root. Invalid Unicode, malformed
XML, duplicate expanded attributes, unbound or forbidden namespace bindings,
contradictory encoding declarations and unexpected family roots fail before a
snapshot is published. DTD declarations are unsupported; no external entity or
network resolution occurs. XML 1.0 is the supported XML version.

`XmlLimits` bounds physical and decoded bytes separately, lexical events including
the synthetic document frame, attributes including namespace declarations, and
element depth. Events, attributes and depth are charged during streaming parsing
before their model allocations. Operation-local interning shares repeated names
and inherited URIs. The model and namespace stack are flat; deeply nested input
does not create recursively owned element trees. Published views and snapshots
are Send/Sync and read without locks or global mutable state.

Property views retain section boundaries, duplicate PID aliases, empty names,
additional values and nested FPVals. Type classification uses expanded names.
Stored bool text such as `No`, integer spellings, locale-like numbers, blob text,
display markup and unknown values stay uninterpreted. Blob encoding, FPVals
meaning, unit enums, material constants and summary completeness remain open in
the specification. No numeric, unit or geometry conversion is claimed.

`with_leaf_text` replaces a leaf's character content while keeping comments,
processing instructions and surrounding markup. `with_attribute_value` changes
an existing ordinary attribute by expanded name and retains its quote and spacing.
Both escape XML characters and validate the complete edited source before
publication. Structural edits use `with_source`; it is a complete validated source
replacement. Equivalent edits and authentic source reverts retain original bytes.
`new` forces reconstruction without relying on the original byte buffer.

## Usage

```rust
use bessemer::metadata::{MetadataStream, XmlLimits, XmlMetadata};
let limits = XmlLimits {
    maximum_bytes: 64 * 1024 * 1024,
    maximum_nodes: 2_000_000,
    maximum_attributes: 2_000_000,
    maximum_depth: 10_000,
};
let stream = MetadataStream::parse("docProps/custom.xml")?;
let metadata = XmlMetadata::from_bytes(stream, &xml_bytes, limits)?;
for section in metadata.property_sections() {
    for property in section.properties() {
        for value in property.values() {
            println!("{:?}: {}", value.kind(), value.element().direct_text());
        }
    }
}
```

`rebuild_xml STREAM INPUT OUTPUT TRACE` accepts repeated argument groups, forces
source reconstruction and writes a binary expanded-name/tree trace.
`set_xml_property SOURCE STREAM SECTION_INDEX PROPERTY_INDEX VALUE_INDEX TEXT
DESTINATION` edits one selected leaf value and replaces the document stream.
Indices are section-scoped preorder selections, not inferred global property IDs.

## Verification

On Windows x64, Rust 1.99.0 MSVC, 2026-10-24, forced source reconstruction retained
all 28,499,247 bytes in 13,100 XML streams from 1,115 installed core documents.
Python ElementTree independently matched all 370,725 elements, 680,359 ordinary
attributes, expanded names, normalized direct text, parent links and child counts.
There were 169 completed empty streams and 17 normalized families. The sorted
relative-path/NUL/stream/NUL/payload SHA-256 was
`d2b467a640eb06977cd2b3a5760dfdfe765047e29fe56940a73fee92993e15a9`.

Run `python -X utf8 docs/features/xml-metadata/verify_corpus.py
 target/pinned-1.99.0/release/examples/rebuild_xml.exe` from the repository root
after building the example. The verifier imports the approved sibling reference
reader, uses the installed SolidWorks 2026 core corpus, and copies the executable
to a private temporary directory. Installed native samples are not redistributed.
`generate_fixtures.py` regenerates the owned independently authored documents.

Owned tests cover every family, UTF/BOM/keyword framing, namespace resets and
aliases, repeated section PIDs, FPVals, opaque values, long attribute names and
spacing, malformed/truncated input, all resource budgets, 5,000-level nesting,
immutable failure and source isolation, authentic reverts, full-document stream
preservation, two-process byte identity, and concurrent edits against serial bytes.
The full Linux CI gate adds ThreadSanitizer; the platform matrix also covers
Windows x64 and both macOS architectures.

During pre-integration review, roxmltree 0.21.2 accepted a forbidden xmlns prefix
binding and misclassified a valid ordinary prefixed xmlns attribute (#23). That
prototype and dependency were removed before publication. The implementation uses
quick-xml 0.42.0 as the streaming tokenizer, with explicit normalized namespace,
XML name, document and declaration checks. Regression fixtures cover both cases,
including character-reference namespace aliases. Dependency license and advisory
checks passed. No fallback parser ships.

These checks establish XML framing and tree fidelity. SolidWorks application
acceptance of edited documents and semantic CAD validity remain unverified.
The maintainer-side humanizer skill was unavailable; the prose pass remains for
review, as permitted by the contributor guide.
