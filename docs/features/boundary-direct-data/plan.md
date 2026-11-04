# Complete direct boundary data

Feature #205 closes two explicit stops in the existing ordinary boundary reader:
temporary chooser time pairs and direct downstream identifier data. The owned
schema excerpts and current source/implementation hashes establish the gap;
they do not establish implementation acceptance.

First freeze complete original-reader goldens across the `0x332C`/`0x332D`
suffix transition and `0x3358`/`0x3359` downstream-tail transition. Include zero,
negative and positive declared counts, both raw time-word extremes, duplicate
negative keys, zero vector entries, signed scalar extremes and noncanonical
nonzero presence words. Check every proper prefix, oversized counts and exact
following boundaries. Direct bodies must leave object-map identities unchanged.

Then prove complete owned configuration/feature reading, typed construction,
one independent expected edit, document reopening and fresh reversal through
the existing public APIs. Reuse existing archive/count helpers. Preserve the
existing field paths and all zero-body baseline bytes; add only the documented
direct fields under their owner. Keep older containing-body admission and
archive modes unchanged.

Run focused primitive/resource/ownership checks, the native reconstruction
corpus and the full PR gate. Record exact revisions and exclusions. Generated
positive bodies do not imply positive installed population, application opening
or geometry validity. The independent reference reports no populated installed
direct bodies or temporary records at its recorded checkpoint.

Keep Bug #204's signed edge-count fix separate. `origin/main` at `7e5e8fac`
contains no model-reader implementation, so integration must first account for
the foundation dependency. PR #191 remains the only open integration PR; do not
publish another until its required exact-head checks are green and it is merged.
