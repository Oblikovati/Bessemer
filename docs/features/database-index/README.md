# Conditional database file index (#230)

`bessemer::database_index::HashFileGraph` reads complete explicitly selected
default-provider hash files and replaces existing same-length values by physical
bucket/cell identity. It returns immutable source/result snapshots, exact graph
coverage, diagnostics and complete byte-ordered history. Edits reparse the entire
result before publication and preserve all unrelated bytes.

The caller declares the provider, current engine extent and current pager extent.
This operation requires equal extents in 512..=65536 and matches the pager extent
to stored P; it does not infer that equality from native buffer code. It reuses
the header, bucket and overflow codecs. File sizes must contain complete pages,
and the stored engine name must be exactly `hash`. Aggregate byte, page, record
and payload budgets are checked before constructing successful owned results.

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
before defining duplicate-key lookup precedence. The Rust complete-graph API
rejects duplicate logical IDs, missing partition mappings and physical aliases.
It also admits the initialized empty state with maximum1, split0 and no mappings.
No lookup precedence or repair is inferred for rejected saved indexes.

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
The complete-file fixture adds a linked bucket, two free pages, an empty value,
inline values and a value spanning two overflow pages. Four independent edits
change only declared value spans, preserve every other byte and reverse to
the original file. `tests/fixtures/database-index/complete-file.bin.gz` retains
the original and complete expected results in BFI1 framing. Its integrity
record pins both the compressed and decompressed bytes. Two independent writes
and regeneration against the checked-in fixture are tested.
The `hash-file-graph-v1` fixture revision adds S512 and the exact `hash` name to
the earlier index-only fixture header so the complete header codec can validate
it. The value intervals and edit expectations are unchanged.
`hash-vectors.bin.gz` supplies 34 independent polynomial-hash cases, including
unsigned bytes, empty keys and both sides of the 2048-byte prefix limit. Rust
checks each vector through a complete 8192-byte-page file.

`native-files.bin.gz` retains all 2486 authenticated source streams, complete
cell payloads and independent value intervals in BFG1 framing. Its integrity
record pins 63,716,892 decompressed bytes and the original census hash. The Rust
corpus test checks all 22,748 cell edits against complete expected file bytes,
reopens each result and reverses it to the original file. It includes inline
cells in streams without overflow, beyond the earlier mixed-bucket cohort.
Four owned containing-document edits also reopen and reverse exactly while
preserving every unrelated stream. Two fresh processes produced identical
5,361,664-byte reconstruction/edit/reversal output for the owned graph and
three retained native files. `file-graph-local-verification.json` records the
executed checks and exclusions; the full PR gate and Linux ThreadSanitizer
remain separate gates.
Run them with:

```powershell
python -m unittest discover -s docs/features/database-index -p 'test_*.py' -v
cargo test --test database_index
```

These checks establish selected framing, ownership and routing. They do not
establish native allocation, transactions, custom hash callbacks, arbitrary
page-provider behavior, application acceptance or geometry validity. Key edits,
length changes, allocation writes and lossless access to noncanonical saved
indexes remain unfinished under Feature230. Maintainer prose review remains pending because
the humanizer skill is unavailable in this environment.
