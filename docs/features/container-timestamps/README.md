# Container timestamp baseline

Issue: #250. Clause 7.3.1 specifies native ZIP timestamp packing from explicit
raw metadata. Bessemer currently preserves existing timestamp words, but has no
typed packing or timestamp-edit API. This feature's runtime implementation is
pending; the files here pin its expected behavior first.

The frozen corpus contains 22,398 requests from `ziptimestamps.native_timestamp`
and 113 local/central timestamp pairs from the existing native blank part,
assembly and drawing fixtures. It covers all six input words' single bits and
complements, year values 0 through 4,096, threshold neighborhoods, noncanonical
cross-products, absent metadata and packed overrides. An independent weighted
integer oracle checks every reference result.

The Rust baseline checks the complete request corpus, native stream names and
both raw header intervals. It also verifies that the existing parser preserves
all tested packed patterns without requiring valid calendar dates and rejects
inconsistent stored copies. No new runtime API or native writer execution is
included in this baseline.

Run the checks with:

```text
cargo test --locked --test container_timestamp_fixtures
python -m unittest discover -s docs/features/container-timestamps -p test_timestamps.py
```

`freeze_timestamps.py` regenerates both bounded packets. The Python tests compare
regeneration byte for byte under an injected temporary output root. Source and
native fixture hashes are recorded in [baseline.json](baseline.json). The
planned operation and its acceptance requirements are in [design.md](design.md).
The humanizer skill is unavailable; maintainer prose review remains pending.
