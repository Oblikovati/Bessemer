# Carrier acceptance decisions

Behavior version 1 is the first compound-file implementation. No prior compound
runtime was published, so there is no older behavior profile to migrate or retain.
The existing newer-container `Document` API is unchanged.

The implementation uses MS-CFB field and allocation rules together with the
actual reference reader. It does not generalize illustrative values into stronger
constraints. In particular, #107 retains both stored color values instead of
requiring the example's black root.

For #109, a logical stream length determines the minimum required capacity, not
an exact allocation length. Every allocated tail sector is still claimed and
validated. Only the logical prefix becomes the user payload. Canonical output
uses minimal capacity; unchanged output retains the full original allocation.
Three new independent fixtures extend the initial seven cases with normal,
mini and root capacity. No native input, expected payload, metadata field or
tolerance was changed to accept the corrected implementation.

For #110, version-4 logical allocation addresses skip physical sector 524,286
when they reach the reserved 4,096-byte range. FAT self-coverage includes that
hole. User bytes, directory records, FAT, DIFAT and MiniFAT use the same mapping;
mini-sector indices within the mini stream remain ordinary contiguous indices.
The reserved sector has exclusive ownership and the required end marker.

The large-address oracle uses independent byte offsets and cardinalities around
2 GB, explicit expected split-copy spans, mapped chain values and reserved-marker
states. It also checks maximum identifier bounds without allocating enormous
inputs. These tests establish the arithmetic and ownership branches, while the
owned/native corpus checks complete encoded files below the boundary. They do
not constitute execution of a complete multi-gigabyte native file.

The carrier acceptance gate requires all owned input projections, independent
complete output goldens, every native stream and metadata record, every isolated
native stream edit, and deterministic process output. Application acceptance
remains separate. No payload geometry, native CWR model load, full Unicode name
collation or complete CAD/project semantics are inferred from these checks.

The maintainer-side humanizer skill was unavailable. A maintainer still needs
to review this prose.
