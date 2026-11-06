# Flow raw state-table baselines

Issue: #241. References: `../inventor-interface/solidworks/flowrecord129.py`,
`flowrecord138.py`, `flowrecord139.py`, `flowrecord417.py` and `flowrecord432.py`.

The FST2 packets freeze 2,404 complete payloads before typed read/edit APIs are
added. They preserve every stored field, count offset, scope interval and
documented derived projection. No production API is introduced by this baseline.

Record 129 version nine stores all tag bytes before its row bodies. Zero tags
have no body or width; every nonzero tag introduces an independently counted
wide vector. A positive row count adds one signed tail member.

Record 138 versions two through five store an object count. For positive counts
in versions below five, a separate caller-supplied count determines the number
of index words. The object count does not determine that width. Versions two
and three add a signed tail. Empty and version-five records ignore count context.
The native sample uses an explicit context assumption of 3,369 words derived
from its payload extent. Native owner context has not been verified.

Record 139 version zero is only its version word. Versions one through four
add a row count and two wide values per row. Version two adds exactly 320 wide
patterns and 100 word patterns; versions three and four add one signed member.
Row widths are 16, 2,976 and 2,980 bytes respectively.

Record 417 version one has exactly two independently counted groups. Positive
word counts introduce a separate wide count, including zero. Zero word counts
omit the wide count entirely. The frozen quotient uses integer division and
allows nondivisible counts; absence remains distinct from a zero quotient.

Record 432 accepts every nonnegative signed version and stores three signed
members. Only versions one and two add a discarded member. The reference marks
versions below two for native recomputation; this baseline preserves that flag
without performing runtime recomputation.

Python tests regenerate the packets in a temporary root, independently rebuild
complete payloads, reject compact truncations, trailing bytes, invalid counts,
versions and wrappers, and check context activation. Four Rust integration tests
rebuild all fields, verify intervals and counts, inspect native payload extents,
and retain every tag byte and numeric single bit/complement/special pattern.

Run `python -m unittest discover -s docs/features/flow-state-tables -p test_state_tables.py`
and `cargo test --test flow_state_table_fixtures`. Audited native payload and
source hashes are recorded in `baseline-verification.json`. Native runtime,
engineering interpretation and SolidWorks application acceptance remain untested.
