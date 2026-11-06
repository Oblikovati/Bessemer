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

This is a fixture baseline for the typed reader/editor. Runtime context
provenance, native reference fixups, refinement postprocessing, engineering roles,
delegated versions 18 and later, and native application acceptance remain open.
