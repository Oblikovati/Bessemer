# Punch record revisions

Bessemer retains the complete punch body and reads its internal signed revision
independently of the file version. Revision gates select raw words, a nullable
array, legacy points and direct keeper data, dynamic arrays, and the late tagged
array without normalizing stored values.

Dynamic point-array widths must be multiples of 24 bytes; signed-integer widths
must be multiples of four. A zero width stores only that word. Nonzero widths
retain both metadata words and the unsigned row count. Extents and aggregate
entry budgets are checked before iteration. Huge valid widths with zero rows
remain empty and allocate no payload values.

The extension sequence retains order and duplicate tags. Known tags select a
signed word, binary64 value or tagged object; the first other signed tag ends
the sequence and remains a stored terminal word. Older internal revisions then
read the established direct draft keeper. No guessed count or opaque byte skip
replaces any branch.

Independent fixtures cover 300 complete graphs across ten revisions, five file
versions and six styles, with 114,319 primitive observations and 63 owned
documents. They exercise empty/populated arrays, signed presence extremes,
shared and cyclic references, all extension forms, legacy keepers and three
independent terminal-tag edit goldens. Six focused Rust tests, three public
save/reopen and isolation tests, and immutable fixture regeneration passed.
The actual native source regression verified two complete containing roots,
source/root hashes, stored revisions, raw words and the recorded later boundary.

Two fresh native processes matched all 906 statuses and causes. Complete typed
reconstruction increased from 869 to 872 configurations, adding 344, 348 and 542
without regressions. The aggregate corpus still fails because 34 configurations
remain incomplete. The independent reader consumed all 76,035,724 resolved bytes
of the 872 accepted inputs through EOF. The full local gate passed 2,534 tests
across 183 test executables, including 1,051 library tests and 277 doctests;
three opt-in corpus tests remained ignored.
Engineering value domains, whole-schema parity and native application
acceptance remain unfinished.

The humanizer skill was unavailable; its prose review is left to the maintainer.
