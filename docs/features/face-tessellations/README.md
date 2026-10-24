# Face tessellation archives

`bessemer::tessellation` reads, constructs, edits, and writes the complete
directory, block, and facet layouts in [clause 12 and Annex B.3](specification-12.md).
This is tessellation archive behavior version 1, tracked by #16.

Callers supply `TessellationContext`: the unsigned archive version and an explicit
full or lightweight mode. Directory version 2 is supported. Blocks contain
component entries to EOF, without an outer count; an empty block is complete.
The documented unresolved object branch at versions 0xA25–0xC7B returns
`FailureClass::Unsupported` when a facet is entered. Empty directories and blocks
remain representable in that range.

Every documented field is retained, including ignored arrays, raw flags, unknown
words, all edge-point doubles, duplicate tessellation-number keys, dynamic-array
capacity and growth metadata, and IEEE f32/f64 bits. Absent arrays differ from
present empty arrays. Modes 0 and 4 have distinct framing; other stored mode words
carry style fields. Compact mode 4 omits the element-width word. Edge omission
state at version 0x3F0F follows the preceding edge and resets for each face.
Directory keys use high-word-first u64 framing; vertex pairs retain low words first.

Published archives are immutable and Send/Sync. `with_contents` validates an owned
edit before publication. Equivalent edits and reverts preserve the original bytes;
`new` forces reconstruction from typed contents. Required field presence, array
lengths, framing, and decode budgets are checked. Signed raw facet/strip counts
and unknown enum values remain stored observations. Array views require their
declared widths; they do not infer element meanings from bytes.

`check_block_links(name, block)` checks archive version, unique block indices, and
the index/hash of each matching directory reference. Its coverage counts distinguish
an empty selection from checked references. Unreferenced cache entries are retained.
It checks one supplied block; it cannot certify that other named blocks exist.

These checks establish archive structure and selected identity links. They do not
validate topology, geometric consistency, intersections, tolerances, native CAD
acceptance, or whether a cache edit matches the authoritative model. The API returns
archived fields, not a successful geometric operation result.

## Usage

```rust
use bessemer::tessellation::*;

let context = TessellationContext {
    archive_version: ArchiveVersion::new(19000),
    mode: FacetArchiveMode::Full,
};
let limits = TessellationLimits {
    maximum_components: 100_000,
    maximum_faces: 1_000_000,
    maximum_array_elements: 10_000_000,
};
let source = FaceTessellationBlock::from_bytes(&bytes, context, limits)?;
let reconstructed = FaceTessellationBlock::new(context, source.contents().clone())?;
let output = reconstructed.to_bytes()?;
```

`rebuild_tessellations` accepts `VERSION full|light directory|block|facet INPUT OUTPUT`
pairs and forces typed reconstruction. `set_facet_color` accepts
`SOURCE VERSION STREAM COMPONENT_INDEX FACE_INDEX COLOR DESTINATION`, edits an
existing styled face, and replaces its stream through `Document`.

## Verification

On Windows x64, Rust 1.99.0 MSVC, 2026-10-24, the installed SOLIDWORKS 2026 corpus
was enumerated through the approved sibling's `swcover.collect([])` and decoded
with its independent `facettess.py` reader. Forced Rust reconstruction matched
every byte of 170 directories and 193 blocks: 788 block components, 35,242 faces,
and 150,411,068 extracted bytes. The SHA-256 of sorted relative path, NUL, stream
name, NUL, and raw bytes was
`a105ed302113e04c050e436f7fd6f0e224870219b8f95a1bf8106aab9f4a1113`.
Files were enumerated using sorted Windows `Path` values; streams retained each
container's central-directory order. Sorting stream names produces a different
manifest (#32). The display-list reuse verifier reproduces this original order
and retains the frozen baseline.

The owned fixture generator uses Python wire primitives without calling Rust or
the reference writer. Its 199 facet vectors cover both sides of every reachable
version gate, populated and absent flag payloads, old layouts, modes 0/4, negative
styled modes, NaNs, negative zero, and lightweight omission. The independent
sibling reader also consumed all 198 full-mode vectors to EOF. Four directory
vectors cover each representation transition. The block fixture holds two faces.
No installed sample or proprietary schema is redistributed by this feature.

Integration tests cover exact reconstruction, explicit unsupported versions,
truncation, complete block boundaries, byte mutation, impossible counts, budgets,
field shape, link failures, caller/source isolation, edit/revert identity, compile
time Send/Sync assertions, and eight rounds of eight concurrent edits compared
with serial output. A separate whole-document test changed the first bolt face's
colour in `samples/introsw/bolt-assembly.sldasm`; the independent reader confirmed
the selected edit, every other extracted stream stayed identical, and two separate
processes produced identical documents. Native application reopening was not run.

The source snapshot records Edition 0.117 from the evolving sibling working tree
at commit `dd0537b3a142d8dda00ef846891efc736a6aac46`. That commit alone does not
identify its uncommitted schema updates. The snapshot retains the exact relevant
text. The maintainer humanizer skill is unavailable; its prose pass remains for
review. Broader display-list and model archive parity remains tracked in #2.
