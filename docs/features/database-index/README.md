# Conditional database file index (#230)

This investigation supplies independent reference checks for the installed
SolidWorks database engine. It does not yet expose a Rust file-index API.
The selected physical profile uses complete 4096-byte pages at page-ID offsets
in an already extracted database stream.

## Native evidence

`native-index-rea-evidence.json.gz` retains twelve selected function dossiers,
the magic-byte search and the default-hash instruction bytes from
`sldDBImplu.dll`. `native-index-source-verification.json` binds the selected
code ranges to the original PE file. `default-hash-leaf-evidence.json`
independently checks 109 reachable hash-function bytes against the PE image.
These are static observations; the original native implementation was not run.

The default hash starts at 5381 and accumulates unsigned bytes modulo 2^32,
using multiplier 33 and at most the first 2048 key bytes. Its saved test word
for `chm@symisc` is `0x03f523f5`. The empty-key result is 5381.
`hash_reference.py` checks this recurrence against a modular polynomial and
checks the bucket request against an independent split-partition calculation.

The index header stores free-head, split and maximum fields at byte offsets
8, 16 and 24. Its map chain starts at offset 32, its record count at 40,
and its ordered 16-byte logical/physical pairs at 44. Continuation pages
store the next page at 0, count at 8 and pairs at 12. The reference reader
preserves duplicate entries before applying its selected canonical-partition
admission check. Native cache insertion and rebuilding need further analysis
before defining duplicate-key lookup precedence.

The native map reader can return success after consuming only a prefix of
the declared records. The reference check requires every declared pair to fit
completely. Native status zero therefore does not establish complete coverage.

## Checked corpus and limits

`native-index-census.json.gz` records all 2486 extracted database streams.
All passed the selected role and hash-routing checks: 13,406 physical pages,
3293 bucket pages, 22,748 cells and 5141 overflow pages. Every supplied page
had exactly one role. Every stored key hash and logical-bucket request matched
the independent default-hash reference.

This cohort contains neither continuation map pages nor free-list pages.
Owned tests cover those branches, complete record boundaries, cycles,
cross-role aliases, missing roles, invalid partitions and duplicate records.
Run them with:

```powershell
python -m unittest discover -s docs/features/database-index -p 'test_*.py' -v
```

These checks establish selected framing, ownership and routing. They do not
establish native allocation, transactions, custom hash callbacks, arbitrary
page-provider behavior, application acceptance or geometry validity. Complete
typed file operations and independent mutation expectations remain required
before the public API is added. Maintainer prose review remains pending because
the humanizer skill is unavailable in this environment.
