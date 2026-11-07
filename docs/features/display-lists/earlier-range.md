# Display versions 12000–12999

This checkpoint extends the complete part and assembly archive codec to display
versions 12000–12999 with explicit native file context 19000. It belongs to #28;
full documented-schema parity remains incomplete. Existing fixtures remain frozen.

The part reference-geometry collector appears after `0x2F5D`; the viewer-component
collector appears after `0x2F44`. Reference axes add their last two doubles after
`0x2F5B`, and reference planes add their last word after `0x2F66`. CPoint and route
point payloads appear after `0x2FB8`. Before `0x3200`, the CPoint extension also
contains three dimension pointers, one further dimension pointer and two
configurable-parameter pointers. Owned fixtures cover null pointers in those
roles. Unresolved non-null class bodies still return unsupported errors.

Markup ink is absent through `0x2F80`. The configuration tail appears after
`0x2FA5`; its absence does not remove the preceding configuration records or
annotation-visibility names. Both document kinds have populated configuration
fixtures on either side of that boundary.

Bucket polygons use float coordinates before `0x2F9A` and doubles from that value.
Before `0x2F63`, they have no persistent-data suffix. From `0x2F63` through
`0x2F95`, the suffix contains an edge-object pointer that the reference reader
loads and discards. Bessemer retains the pointer, its body and aliases to reproduce
the stream. The field oracle omits this discarded object; reconstruction and
explicit graph assertions check its retained bytes and role. Later polygons
restore the face-node, arc and silhouette suffix. Arc `unknown_68` appears after
`0x2FA9`. Edge records carry two discarded words after `0x2F7A` and before
`0x333C`; their linear/vertex flags appear after `0x2FAC`.

The detailing-saved word appears after `0x2F8B`. The model display-item list appears
after `0x2FAA`. The later postprocess word remains independently gated. Omitting
the populated model list removes one aggregate collection entry; omitted scalar
words remove none. A newly retained discarded edge object consumes one object
slot; aliases consume no additional slots.

The independent reader consumed 60 complete shaded roots, 54 complete unshaded
roots in both coordinate scopes, and nine roots containing all seven reference
kinds. It supplied 18,768, 12,444 and 1,030 field expectations respectively: 32,242
in total. The manifests record fixture hashes and reader provenance. These are
owned wire fixtures, not claims of CAD application or geometric validity.

Sixteen focused tests compare fields and forced reconstruction, enforce exact
budgets and their failing lower bounds, exercise edits and authentic document
reverts, reject every prefix of populated roots, validate discarded-pointer roles,
and compare eight concurrent edits after mutating the input buffer. The process
gate regenerates all 123 fixtures and compares independent Rust reconstruction
processes. The earlier 134-fixture transition gate remains unchanged in bytes and
field traces. Reports record process hashes and runtime source hashes.

```powershell
cargo build --example rebuild_display_archives
python -B -X utf8 docs/features/display-lists/verify_earlier_processes.py target/pinned-1.99.0/debug/examples/rebuild_display_archives.exe
cargo xtask pr
```

`cargo xtask pr` passed on Windows x64 with Rust 1.99.0 MSVC, including all 176
doctests. The native corpus gate passed all 1,086 streams with 1,849,563 independent field
comparisons and identical reconstruction in two processes. Earlier intervals, other
file-version contexts, unresolved classes, document-level validation and the
remaining schemas remain open. The maintainer-side humanizer skill is unavailable;
its prose pass remains for review.
