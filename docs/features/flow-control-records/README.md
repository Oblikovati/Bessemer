# Flow strict control-record profiles

Issue: #241. Reference: `../inventor-interface/solidworks/flowcontrolrecords.py`.

These are writer-derived strict payload profiles for saved spans (`0xffffff9c`)
and terminal indexes (`0xffffff9b`). The source evidence contains three synthetic
writer examples and reports no installed control-record payload fixture. Neither
these examples nor the owned generated corpus establish native application
acceptance or universal reader acceptance of the zero-marker restriction.

Saved spans are one exact zlib stream. The decoded body stores a zero 64-bit
marker, an unsigned 64-bit count and ordered pairs of raw 64-bit keys/values.
The count is bounded by complete remaining 16-byte entries; this profile does
not impose a signed native-transfer cap on that unsigned count. Duplicate keys
and their original order remain intact.

A terminal-index payload begins with an eight-byte zero marker, then one zlib
stream and a footer. Its decoded body stores a nonnegative signed 32-bit count
and ordered 20-byte entries: unsigned identifier, unsigned 64-bit record start
and unsigned 64-bit record span. The footer contains an explicitly selected
four- or eight-byte unsigned backward distance and identifier `0xffffff9b`.
For either width, `footer_start = 8 + compressed_bytes` and
`backward_distance = footer_start + length_width + 4`. Complete decoded EOF and
exact footer extent are required. The four-byte branch is synthetic payload
context; it does not extend the ordinary physical carrier reader's formats.

The FCS1 and FCI1 packets retain 313 saved-span and 758 terminal-index cases.
Their uncompressed sizes are 5,599,437 and 13,460,234 bytes, each below the shared
16 MB fixture-read bound. The corpus covers empty entries, every identifier bit
and complement, every raw-wide bit and complement in each entry position,
counts through 65,536, both footer widths, compression variants, duplicates,
reserved identifiers, zero spans and intentionally unreconciled target values.
The original synthetic examples are authenticated against the native-evidence
manifest and retain their full reference projections. The three installed
carrier files are rehashed against the physical corpus and rescanned to confirm
that neither control identifier is present.

Rust reconstructs every decoded byte, checks exact zlib consumption, and verifies
both footer-width equations and literal reference entries. Python checks exact
and one-below decode bounds, marker/footer bit mutations, strict counts, all
proper prefixes of small complete frames, invalid checksums/members/wrappers,
raw-pattern coverage and deterministic regeneration under a temporary root.

```text
python -m unittest discover -s docs/features/flow-control-records -p test_controls.py
cargo test --locked --test flow_control_record_fixtures
```

Digests and focused results are recorded in `baseline-verification.json` and
`local-verification.json`. Typed readers/editors remain to be implemented. Index
adoption, saved-span key/value meaning, physical target validation, logical index
replay and native application acceptance are separate work.

The humanizer skill is unavailable; maintainer prose review remains pending.
