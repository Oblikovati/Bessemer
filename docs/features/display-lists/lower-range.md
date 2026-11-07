# Display versions 14000–15999

Issue #28 extends admitted contexts to 14000–19000 while preserving the earlier
explicit snapshots and native file context 19000. Complete roots pin the lower
range's version gates before it is admitted; older gaps remain unsupported.

The lower range distinguishes material thread/second-texture fields above
`0x372A`, the outer face word at `0x3743`, detailing-view state at `0x374B`, part
thread visibility and facet auxiliary texture arrays above `0x3751`, and the
facet thread-texture byte above `0x3757`. Bucket arc data gains its byte at
`0x3724`; detailing buckets gain their post-process word at `0x372A`, inclusive.
The existing facet codec handles late stamps above `0x3B0D` and edge records
above `0x3B28` with their complete nested arrays.

Assembly component tessellation numbers are absent through `0x3B1B`, use a
single fixed pair before `0x3B23`, then use a counted triple array. The fixed
pair's identifier is implicitly -1 in memory and consumes no wire bytes. The
archive retains the two stored words without inventing a serializable ID field;
its shared budget charges one tuple. Display-state names appear above `0x3B1C`.
The independent scalar adaptor excludes that unstored identifier, and tests
assert its absence alongside the two stored values.

Forty-eight complete shaded roots cover both document kinds at each texture,
face, detailing, thread, facet and component-number boundary, both interval
endpoints, overlap at 16000, and independent ambient edits. They include populated
primary textures and both inline facets. Twenty complete unshaded roots cover
both bucket coordinate scopes at the inclusive arc/detailing boundaries, a view
mode edit and an empty bucket. The sibling reader consumes each stream to EOF,
providing 13,979 shaded-root fields and 5,940 bucket fields.

Source assembly/material/scene/facet builders are reused. Additional scoped
reference observers remove or replace only the documented fields, restoring
function bindings and the face reader registry in `finally` blocks. Sibling
files are not modified. Existing middle-range fixture payloads and expectations
remain unchanged; the new family uses a distinct scenario marker. The shared
middle converter accepts the additional patch list instead of duplicating its
version logic.

Exact budgets account for the omitted facet arrays and edge records. Relative
to the established ordinary-vertex roots (141 part / 146 assembly entries),
auxiliary texture omission subtracts two entries across the facets through
`0x3751`, and omitted edge records subtract sixteen through `0x3B28`. The new
assembly vectors add zero, one or two tessellation-number tuples at the stated
boundaries. Texture and scalar flag spellings do not change collection counts.
Bucket totals remain 91/86 for populated 3D/2D roots and 45 for an empty bucket.

The expanded integration suites preserve previous fixtures and add explicit
field-presence and fixed-pair checks, exact limits, independent edit/reopen/revert
for both document kinds and lower buckets, source ownership and eight concurrent
edits, malformed counts/shapes and every prefix of populated 14000 and 16000
part/bucket fixtures. The process verifier regenerates all 68 frozen roots,
checks their hashes and compares two Rust processes byte for byte. Its result
is recorded in `lower-range-verification.json`; field oracles run separately in
integration tests. The 1,086-stream native gate remains required.

```powershell
cargo build --example rebuild_display_archives
python -B -X utf8 docs/features/display-lists/verify_lower_range_processes.py target/pinned-1.99.0/debug/examples/rebuild_display_archives.exe
```

Earlier archive intervals, unresolved object bodies, whole-document context and
cross-stream validation remain required for full parity. Archive interoperability
does not establish CAD application or geometric validity.
The maintainer-side humanizer skill is unavailable; its prose pass remains for review.
