# Hidden silhouettes and the seven-version discarded edge word

Reference baseline: specification edition 0.248, dated 2026-10-25. Reader hashes
are recorded in the fixture manifests.

The 262 complete part and assembly roots cover versions 1857, 1858, 1859, 1860,
1866, 1867 and 1875, with 79282 independently expected traced fields. Hidden
silhouette lists (`silh_h`) are absent through 1857 (`0x741`). Edge-visibility
records contain one extra discarded i32 only from 1860 through 1866 inclusive
(`0x744` through `0x74A`). The codec retains it as `discarded_v0744`, separately
from the earlier discarded word that remains stored throughout this interval.

The reference reader drops the extra word. The oracle captures its actual i32
read, while retaining the reference's existing decoded fields. Independent
fixtures insert a distinct raw word after the five ordinary edge-header words
and before the polygon list. Baseline and edited values differ at the signed
maximum. Tests check that every complete edge record has the word inside the
interval and none outside it, including records with empty or nonpositive polygon
counts. A minimal interval record requires 28 bytes rather than 24.

The generator observes hidden-silhouette list objects in complete bucket parses
before removing their exact physical intervals. Neither primitive list removals
nor added i32 words alter MFC class or object identities. Each final root is
parsed to its end at its declared version. Earlier fixture groups stay unchanged.

Complete nested 2D and 3D buckets, empty/null models, all five old-light classes,
externally cached models and fifty-six inline models remain covered. Inline
facets match the independently built version-2011 records in full typed contents
and forced bytes at each actual context. Composite edits change the discarded
word together with fonts, text, polygons, edges and facet counts, then
authentically revert without changing the source.

Context changes reject both missing and extra interval words, and missing hidden
silhouette lists. Exact field-budget checks include each discarded word.
Truncation tests reject partial roots. Eight owned containers cover baseline and
edited part and assembly streams at 1857 and 1860, proving list omission and the
extra-word interval through replacement, reopen and revert. Unrelated streams
remain unchanged. Process verifiers compare roots and whole containers in two
fresh processes.

This checkpoint admits archive contexts from 1857. Earlier documented layouts
and unresolved representations remain tracked by #28. Byte fidelity does not
establish geometric validity or native application acceptance.
