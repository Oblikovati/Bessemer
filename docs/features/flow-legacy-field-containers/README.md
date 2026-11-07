# Legacy field-container baseline

Issue: #241. This baseline pins records 105, 113, and 155 from
`../inventor-interface/solidworks/flowlegacyfields.py` before adding their typed
readers and editors. It also pins record-105 scalar decisions and the rule that
record 113 takes precedence over record 155.

The owned corpus contains:

- 1,007 raw word payloads, including two authenticated native record-105 payloads
  and one native record-113 payload, totaling 32,068 native payload bytes.
- 90,601 scalar-query pairs from a 301-pattern domain, including signed zeros, NaN signs and payloads, and adjacent encodings around 253
  and 565. An independent integer bit classifier checks the floating-point
  reference decisions.
- 1,986 complete raw record-155 groups, reusing the existing one-dimensional
  conditional-item generator and field/scope snapshot format. No native
  record-155 payload was found in the authenticated corpus.
- 400 ordered selection requests covering missing records, duplicates, malformed
  preferred records, and successful empty vectors.

Four independently bounded packets retain raw bytes, complete expected patterns,
scalar outcomes, fallback fields and scope intervals, and selection completion
or failure. Each decompressed packet stays below the existing 16,000,000-byte
fixture limit. The packets do not reproduce native floating-point faults,
virtual method results, conversion, engineering meaning, or application loading.

Run the baseline with:

```text
cargo test --locked --test flow_legacy_field_fixtures
python -m unittest discover -s docs/features/flow-legacy-field-containers -p test_legacy_fields.py
```

`freeze_legacy_fields.py` authenticates the source corpus and regenerates all
four packets. Tests compare regeneration byte for byte. The shared conditional
packet reader was extracted in `9ddcc851`; all 14 earlier integration tests
passed unchanged before the new record-155 fixtures used it.

The public profile APIs are still to be implemented. See [design.md](design.md)
for the required behavior and reuse boundary. The humanizer skill is unavailable;
the maintainer prose pass remains for review.
