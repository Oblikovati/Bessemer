# Per-configuration display records

Issue #28 adds populated multi-configuration restores for the admitted part and
assembly display contexts. Part records enter this path at versions above
`0x2B5E`; assembly records enter it at `0x2404`. Nonpositive configuration counts
complete with an empty result and retain the original signed count.

Positive counts carry the active name and ID, configuration names and records.
Part records contain an ID and a generic object pointer. Assembly records contain
an ID, a shared assembly-object array and a visual-property pointer when required
by the version. All pointers use the existing archive-wide class/object map;
configuration records do not reset it. Existing nulls, aliases and owner cycles
remain intact. Unknown new object bodies return an explicit unsupported error.

Assembly restoration temporarily sets the operation-local configuration mode.
That mode suppresses external face-directory lookup and forces inline facet
containers, matching the reference reader's `env_b14` behavior. The previous mode
is restored after success or failure. No global state, locks or thread-local
state are introduced.

The versioned view suffix contains two matrices: configuration IDs with generic
object pointers for each root view, then configuration IDs with display-item
lists for each root view. Both pointer tags and MFC list counts have a two-byte
minimum; extended count spellings remain unchanged. Zero root views still carry
the per-configuration IDs. Later versions append annotation-view visibility
string pairs and a tail of string keys with signed integer arrays. Tail counts
store the high 32-bit word before the low word.

Names, configuration records, matrix rows/cells, display items, visibility pairs,
tail entries/values and nested facets share the existing entry budget. Matrix
products are checked for overflow and physical fit before traversal. The shared
high-word-first count reader checks physical fit before converting to `usize`;
reference-geometry records reuse it with their existing field paths.

Fifteen owned complete roots cover version 19000 part and assembly records,
independent tail edits, negative and zero counts, null records, zero root views,
version 18000 parts, and version 10000/11000 assemblies. The approved sibling
reader consumes every root to EOF and provides 222 independent scalar fields.
The manifest records source hashes, fixture hashes, object counts, external IDs
and independently accounted entry totals. These fixtures establish wire behavior,
not CAD application or geometric validity.

The budget ledger includes 86 entries for the reused modern part-tessellation
body (including two facets at 37 elements each). A modern configuration component
adds 14 collection entries around that body. Complete populated part/assembly
roots therefore require 160/176 entries. Removing the root view also removes its
matrix cells, items and new view-name object, giving 153/169. Minimal null roots
require 59; nonpositive counts preserve the native total of 53. The 18000 part
starts from the established 68-entry scene root and totals 175. Older assembly
roots total 133 at 10000 and 141 at 11000; their facets contribute six/eight
elements each, and their configuration suffix omits view matrices and the tail.

Ten integration tests check independent fields and exact bytes, shared IDs and
cycles, inline facet contents against the owned standalone facet oracle, exact
budgets, completed empty results, nulls, old omissions, matrix bounds, high-word
ordering, every truncated minimal stream, trailing bytes and unsupported bodies.
Both document kinds have independent edit/reopen/revert checks that preserve
other streams, plus source isolation and eight concurrent edits against golden
bytes. `configuration-verification.json` records two separate Rust processes
reconstructing all 15 owned fixtures, with identical bytes and traces. It records
source hashes and the Windows platform; scalar expectations are checked separately
by the integration tests. The existing 1,086-stream native gate also passed.

Reproduce the owned process check after building the example with the pinned
toolchain:

```powershell
cargo build --example rebuild_display_archives
python -B -X utf8 docs/features/display-lists/verify_configuration_processes.py target/pinned-1.99.0/debug/examples/rebuild_display_archives.exe
```

Earlier part configurations with embedded tessellation, intermediate version
gates, automatic document context and cross-stream validation remain part of the
full schema work. The admitted version set has not been expanded here.

The maintainer-side humanizer skill is unavailable; its prose pass remains for review.
