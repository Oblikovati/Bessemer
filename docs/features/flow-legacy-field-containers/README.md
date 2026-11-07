# Legacy field containers

Issue: #241. The typed readers and editors follow records 105, 113, and 155 in
`../inventor-interface/solidworks/flowlegacyfields.py`. Their frozen baseline
also pins record-105 scalar decisions and the rule that record 113 takes
precedence over record 155.

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

`FlowCarrier::record_105_fields` and `record_113_fields` return complete raw
word views with independent payload and pattern limits. Record 105 contains
exactly 320 patterns. Record 113 has a signed nonnegative count, including zero.
Neither layout has a version prefix. `replace_pattern` edits existing slots on
an isolated carrier and returns the complete byte history.

`FlowCarrier::record_155_fields` reads its real stored version zero and reuses
the one-dimensional conditional grammar. Its fields and scopes retain their
actual record-155 identity. Existing nonstructural fields support typed edits;
complete reparsing rejects changes to field or scope structure.

`queries_105` models completed scalar branches using integer bits, preserving
NaN and signed-zero decisions without executing a native virtual method.
`legacy_field_container_latest` selects the latest 113, otherwise the latest
155. A malformed selected payload fails; no matching record returns `None`.

The focused suite passed 16 tests, including a parent that ran its independent
replay worker twice. The earlier conditional-item suite passed all 14 tests
unchanged. The new checks cover all frozen field values and paths, exact
resource limits, malformed framing, typed edits, history, inverse wires,
input isolation, and eight workers sharing an immutable record-155 view.
See [api-verification.json](api-verification.json) for exact check scope and
remaining publication gates, and [design.md](design.md) for layout constraints.
The humanizer skill is unavailable; the maintainer prose pass remains for review.

The combined local `cargo xtask pr` gate passed at `f0bd5f21`: 5,138 tests
passed, none failed, and 268 were ignored. Later timestamp work and hosted
acceptance are excluded. See [full-pr-gate-verification.json](full-pr-gate-verification.json).
