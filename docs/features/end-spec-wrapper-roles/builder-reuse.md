# Complete end-specification fixture reuse

The existing end and revolution-end builders now accept optional wrapper
callbacks. End specifications also accept an optional starting-end callback so
whole-document fixtures reach that reader through a complete boss feature.
Defaults retain their original framing and identities. The 180 pre-change
body/end builder byte lengths and hashes are owned in
`legacy-builder-baselines.json`; `test_legacy_builders.py` rechecks them.

The new fixture caller reuses complete wrapper bodies from the earlier finite
wrapper corpus and the independently implemented surface base. Cached targets
reserve their identity before nested bodies. Complete boss/revolution parents
are used for owned documents; bare-object source cursors explicitly install the
actual source readers. No source parser is replaced with a fixture stub.

This fixture-preparation change is separate from the production role fix.
Earlier owned corpora remain unchanged. The source reader is an EOF, byte and
identity oracle; it ignores expected-class mismatches and cannot establish role
membership. Native declarations, reviewed ancestry and negative Rust tests are
required for that boundary.

The humanizer skill is unavailable; maintainer prose review remains open.
