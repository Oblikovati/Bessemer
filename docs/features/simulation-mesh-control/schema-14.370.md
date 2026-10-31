### 14.370 Complete retained counted MeshControl record framing

*Status: all twelve retained MeshControl streams complete; separate DocumentInfo header remains unresolved · Evidence: C-749*

`cosworks_mesh_control_layout.py` implements the count and direct-record grammar bound in 14.369. Each stream starts with signed record count; each record begins with shared base **180B754B0** at the constructor's **+8 serialization subobject**, then the versioned mesh-control suffix. This base is the same already verified for shell Property, but offsets in the derived mesh reader remain relative to the subobject; no extra eight bytes are inserted into the wire stream for that in-memory adjustment.

The bounded parser validates count and all reference/blob/text/archive boundaries, preserves contiguous raw spans and floating-point bits, and rejects EOF remainder. It follows the unchanged caller version through both base and derived gates. Nonzero virtual +50 records and populated final shared-base curves remain explicit stops rather than guessed fixed-size skips. Mesh-control defaults/toggles consume no bytes and do not mutate preserved wire values.

C-749 freshly checks source envelope and payload SHA-256 values, native serializer/function hashes, the verified +8/+80 slot pointer, and all shared/mesh-specific numeric gate bytes retained by C-743/C-748. All **12 retained streams / 86359 bytes** contain **16 complete direct records** and reach exact EOF with full reconstruction. All twelve supplied candidate attempts succeed; successful framing alone does not prove the general runtime study-array index mapping.

Thirteen tests pass across counted mesh-control, Material and embedded-reference dependencies. The mesh-control tests independently construct zero/one/two-record cases at older and newest fixed-tail profiles, reject truncation/remainder and negative counts, and enforce explicit nonnegative integer versions. Native successful records do not prove unrepresented nonzero virtual/base-curve branches.

This resolves the counted-record framing boundary in 14.369. The four-byte **DocumentInfo/MeshControlManager/Property** remains outside the bound study header layout, and no complete DocumentInfo handler is inferred from these record successes. Physical meanings, historical profiles, unrelated managers and full installed-format coverage remain open. No target executes; global semantic coverage credit remains zero.

Evidence: C-749.
