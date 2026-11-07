# Display versions 13000–13999

Issue #28 extends admitted contexts to 13000–19000 while retaining the earlier
explicit snapshots and file context 19000. Complete roots exercise the appearance,
view, annotation and bucket transitions around `0x3330`–`0x336B`.

Root font scale appears above `0x3331`, the temporary appearance symbol above
`0x3347`, PLM identity above `0x3368`, and outer face identity above `0x336B`.
Existing view and material readers already preserve their other gates. The
reference fixture converters now apply both bounds of the temporary-symbol
interval and remove only the newly absent fields from the owned source roots.

Bucket arc data retains the extra vector through `0x3350`; selected flags change
from signed words to bytes after that value. Other arc fields appear at their
individual bounds. Edge records retain earlier face/edge nodes and two discarded
words, and independently gate persistent IDs, feature IDs, assembly flags,
parameters and hole indices. The detailing-view body and its presence word are
absent through `0x3341`.

Before `0x333C`, a silhouette pair supplies two signed words. The independent
reader computes `high * 1000 + low`; after `0x3333`, a later stored word replaces
that temporary result. The codec retains both raw words even in that overwritten
case. Its verification trace computes the temporary value only where no stored
ID replaces it. Direct assertions pin `-17, 42 -> -16958`, the discarded edge
words, and the word/byte variants; they are not normalized into a common wire type.

Seventy complete shaded part/assembly roots cover the interval boundaries,
appearance fields, populated table/EDW items, independent ambient edits and ink
layouts. The table's 3D-scale string, EDW strings and annotation ID string follow
their individual version gates. Two complete roots contain 100 minimal ink views
with zero configurations: each row needs five bytes before the trailing-string
layout, not six. Populated ink grids on both sides of `0x332B` also retain their
versioned strings. The independent reader consumes all roots to EOF and supplies
22,447 appearance, scene, component, item and ink expectations.

Sixty-four unshaded roots cover both bucket coordinate scopes, with a mode edit
and empty collection, providing 18,037 independent field expectations. Manifests
and fixtures are split by document kind or coordinate scope so each supporting
file remains bounded. Existing lower/middle fixtures remain unchanged.

The new populated item list charges twelve entries: two display items, two EDW
records, and four nested collections per record. Minimal ink roots replace the
source's one configuration-dimension entry with 100 view entries, adding 99.
Populated two-by-two ink grids add seven relative to that source. Other scalar
width/omission changes do not change collection costs. Exact global budgets still
include both inline facets and every inherited collection.

The expanded suites compare all fields and forced bytes, assert omission and raw
values, enforce exact limits, exercise independent edits/reopen/reverts, preserve
other streams, and check source isolation plus eight concurrent edits. Every
prefix of populated 13000, 14000 and 16000 part/bucket fixtures is rejected. The
process verifier regenerates the 134 frozen roots, checks hashes and compares
two Rust processes; its report is `transition-verification.json`. Field oracles
run separately in integration tests. The native 1,086-stream gate remains required.

```powershell
cargo build --example rebuild_display_archives
python -B -X utf8 docs/features/display-lists/verify_transition_processes.py target/pinned-1.99.0/debug/examples/rebuild_display_archives.exe
```

Earlier intervals, unresolved object bodies and whole-document context/validation
remain required work. Archive interoperability does not establish CAD application
or geometric validity. The maintainer-side humanizer skill is unavailable; its
prose pass remains for review.
