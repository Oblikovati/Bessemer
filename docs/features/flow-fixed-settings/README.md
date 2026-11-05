# Flow fixed settings — issue #241

Specification 14.34 defines two raw profiles: record 7 contains 100 little-endian
32-bit slots, and record 102 contains 320 little-endian binary64 slots. Neither
payload has a count prefix. Ordinary lookup selects the last occurrence. Missing
records remain absent; short and extended payloads fail this strict profile.

The frozen FFS1 fixture contains four native payloads (5,920 bytes) authenticated
against the source corpus and two owned payloads. The owned values distinguish
signed integer boundaries, signed zero, infinity, subnormal values and NaN payload
bits. Expected slots come from `../inventor-interface/solidworks/flowfixedsettings.py`;
Rust independently rebuilds every payload and checks carrier retention.

`FlowCarrier::fixed_settings` reads either typed profile and retains the selected
record identity and source checkpoint. `slot` returns raw integer or binary64 bits;
signed and floating-point interpretations are optional views. `replace_slot`
requires the matching profile, edits an isolated copy, reparses the complete
carrier and checks the full patched payload before returning physical record
history. No-op edits retain original wire bytes. Replacing the original slot
value in a fresh operation restores the original carrier exactly.

Native runtime normalization, individual slot meanings,
loader execution and application acceptance are separate requirements. These
fixtures do not establish complete Flow interoperability or global byte coverage.

Run `python -m unittest discover -s docs/features/flow-fixed-settings -p test_fixed.py`
and `cargo test --test flow_fixed_fixtures`. Regeneration tests write to temporary
directories. The maintainer humanizer pass remains for review because that skill
is unavailable in this environment.
