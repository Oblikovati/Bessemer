# Flow legacy reference-list groups

Issue: #241. Reference: specification section 14.50 and
`../inventor-interface/solidworks/flowreferencegroups.py`.

Record 416 versions 0–17 contain exactly two groups. The stored header word is
preserved independently of this fixed loop. Each entry contains a signed
reference-count preamble followed by versioned state/selector bytes, seven
descriptors and two raw four-byte arrays. Its reference words remain signed raw
values; they are not assumed to be record-406 identifiers or resolved pointers.

Version one uses byte descriptors and a signed byte secondary count. Versions
zero and two through nine use unsigned 16-bit secondary counts; later versions
use signed 32-bit counts. A negative secondary count consumes no patterns and
is retained unchanged. Header, prefix, member-20, selector and suffix fields
follow their documented version gates, including discarded values.

The first group's optional refinement level and mapping words require an
explicit mapping-branch context. They are absent for an empty group under either
context. The second group appends three wide patterns per entry; the first group
does so only in versions 15–17. No mapping shifts, pointer repairs, native flags
or engineering roles are synthesized.

The independent FRG1 baseline contains 1,397 views: one complete installed CPT
payload, both mapping contexts for all eighteen versions, all 256 version-one
secondary-count bytes, and maximum-width array profiles. Raw and zlib cases
retain complete decoded bytes, every group/entry interval, optional fields,
signed references, raw patterns and the documented exclusions. Rust rebuilds
every decoded byte from those fields. Python checks fixed hex, width transitions,
non-2 headers, negative counts, mapping/wide-array gates and full regeneration.

The native version-14 payload is 1,978 stored bytes and 4,464 decoded bytes.
Its two group counts are 64 and zero. It contains 128 reference words, 424 first
array patterns and 176 second array patterns. The mapping branch is explicitly
assumed disabled; successful EOF does not establish native runtime provenance.

`FlowCarrier::record_416_references` reads these layouts with
`FlowReferenceGroupOptions`. The mapping branch is a required Boolean option;
no native reset state is inferred. Positive independent bounds cover entries,
references, entry-array words, mapping words and wide patterns, alongside the
complete decoded-byte bound. Counts are checked before allocation or width
multiplication. Views retain flat immutable metadata and borrow array bytes.

Groups expose physical entry order, optional refinement levels and mapping/wide
arrays. Entry selectors use the existing `FlowBlockFieldValue` codec. Version-one
descriptors return `Word16` while retaining their one-byte stored width; secondary
counts return `Signed32` while retaining their original small raw count word.
Reference-count and group-count preambles are structural and have read-only views.
Header words remain editable raw fields and never alter the fixed group loop.

Editors replace existing headers, entry fields, levels, mapping words and wide
patterns on isolated carriers. They reopen the complete typed result under the
original options, compare every entry and array span, and check the full patched
decoded payload. This rejects structural changes even when a different complete
parse exists. Negative secondary counts may change while retaining empty arrays.
No-ops preserve the stored bytes and revision. Results include both typed
checkpoints, exact coverage, diagnostics and all physical carrier history.
Raw reversals restore the complete original carrier; zlib reversals restore the
decoded payload without assuming canonical compressed bytes.

The implementation decision is exact serialized interoperability for documented
versions 0–17. It reuses record 406's private continuation parser and scalar codec,
while retaining the distinct reference-list preamble and version-zero count width.
Runtime context provenance, native reference fixups, refinement postprocessing,
engineering roles, delegated versions 18 and later, and native application
acceptance remain open.

The focused gate passed 14 record-416 tests and 35 record-406/421 regressions;
two historical exhaustive native tests remained separately invoked. Clippy and
eight Python checks passed. Every exposed native field received a no-op, an
independent full-payload patch check, full history checks and a fresh reversal:
1,369 requests, with structural descriptor/count values retained where required.
The read-only count preambles are covered by the complete read projection.
Larger owned arrays use first/last edit samples; small owned fields are exhaustive.

Two fresh processes matched the complete independent FRG1 packet byte for byte:
43,028,027 bytes across 1,397 views. All eighteen versions also passed measured
compressed-growth rejection and eight-worker comparisons under both mapping
contexts and both transports. The full repository gate and remote CI are recorded
separately from this focused evidence.
