# Display versions 16000–17999

Issue #28 extends admitted part/assembly contexts from 18000–19000 to
16000–19000, preserving the earlier explicit snapshots and file context 19000.
The new range is backed by complete owned roots at every relevant boundary;
older unsupported intervals are not inferred from nearby snapshots.

The component reader omits routing state through `0x42E7` and omits SpeedPak
state below `0x42F3`. Existing versioned readers select DSPBR presence above
`0x3F08`, envelope settings above `0x42DA`, and scene HDR/tone-map fields above
`0x42EC`. The complete fixtures include both inline facets and populated shared
scenes, spatial views and appearance records for both document kinds.

Polygons keep separate signed Boolean words through `0x41A0`, even though their
coordinates are already doubles in this interval. The reader preserves those
words, the arc and silhouette flags, optional arc payload and persistent IDs.
After that boundary it reads the packed flag word. Old supported snapshots keep
their float coordinates and earlier tails. The shared trace synthesizes the
independent reader's interpreted flags while retaining all raw words in the
archive. Thread/repair collections still restore the enclosing coordinate scope.

Forty-two complete shaded roots cover the lower endpoint, facet vertex-layout
transition at `0x3EE7/0x3EE8`, DSPBR boundary, the special facet stamp spelling at
`0x3F0F`, packed polygon boundary, envelope/routing/HDR/SpeedPak boundaries, upper
endpoint and overlap at 18000. Independently encoded material ambient edits are
included for both kinds. Ten complete unshaded roots cover both bucket coordinate
scopes before and after packed flags, an edited view mode and an empty bucket.
The sibling reader consumes every stream to EOF and supplies 13,200 appearance,
scene/component fields plus 2,874 bucket fields.

Exact budgets count every facet element and collection. A later styled facet
adds two two-element arrays over the older ordinary vertex array, so two facets
add eight entries at `0x3EE8`. Shaded parts therefore need 141/149 entries and
assemblies 146/154 below/above that boundary. The independently reviewed 3D/2D
bucket totals remain 91/86, with 45 for an empty bucket; flag spelling changes
wire bytes without changing collection sizes. Source object counts and fixture
hashes are recorded in the two manifests.

Twelve integration tests compare all independent fields and forced bytes,
validate field omission, raw flags, double coordinates and both scope states,
check exact global limits, reject malformed counts and conditional shapes, and
exercise every prefix of the populated 16000 part and 2D bucket fixtures. Both
document kinds have a
material edit/reopen/revert check with other streams preserved. The bucket edit
has its own container reopen/revert. Published roots and buckets own their input
and support eight parallel edits against independent golden bytes.

The process verifier regenerates the frozen fixtures, checks their hashes and
forces all 52 through two isolated Rust processes. Its byte/trace comparison and
source hashes are recorded in `middle-range-verification.json`. Integration tests
check the field oracles separately. The native 1,086-stream gate remains required.
These checks establish archive behavior; CAD application/geometric acceptance
remains separate.

```powershell
cargo build --example rebuild_display_archives
python -B -X utf8 docs/features/display-lists/verify_middle_range_processes.py target/pinned-1.99.0/debug/examples/rebuild_display_archives.exe
```

Earlier display intervals, additional caller contexts, unresolved object bodies
and whole-document context/validation remain required work for complete parity.
The maintainer-side humanizer skill is unavailable; its prose pass remains for review.
