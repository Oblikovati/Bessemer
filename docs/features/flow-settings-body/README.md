# Flow versioned settings body — issue #241

Specification 14.36 defines the strict raw record-511 profile: outer version zero,
inner versions 0–11, and 24–45 payload bytes depending on the inner version.
`FlowCarrier::settings_body` retains the entire source checkpoint and ordinary
last-record selection. Its fields retain exact payload offsets and native member
labels. These labels identify destination-object offsets, not wire offsets or
engineering meanings. Raw four/eight-byte members remain raw; the established
u32 member keeps all bits and byte fields are not coerced to Boolean values.

Conditional fields follow the documented wire order, including member_11 before
member_88. Missing fields are omitted, without invented zero defaults. Unsupported
outer/inner versions, every short payload and unexplained extensions fail this
strict profile. Native extension acceptance and return-value behavior are outside
that claim.

For versions zero and one, member_0f has no stored byte. Missing record 511 follows
the same legacy context rule. The caller may provide an explicit runtime signed
integer slot-88 value: zero derives one, and nonzero derives zero. Without that
context the view reports `RequiresRuntimeIntegerSlot88`. Stored record 7 is not
automatically treated as runtime state after normalization. Versions 2–11 retain
the complete stored member_0f byte regardless of the supplied context.

`replace_member` requires a physically present member and its exact representation.
It edits a copy, reparses the complete carrier and body, and checks the entire
patched payload before returning the existing complete physical record history.
No-op writes retain exact wire bytes. A fresh opposite edit restores the original
carrier bytes. Source checkpoints and earlier views remain unchanged. Creating
records, adding omitted members and migrating body versions remain separate work.

The FSB1 fixture contains twelve explicitly synthetic populated bodies and 72
runtime-context projections from the sibling `flowsettingsbody.py`. All three
source-bound native samples lack record 511; their hashes and absence audit are
retained separately. Synthetic bodies establish tested grammar branches, not
populated native-body acceptance. Literal checks pin payload lengths, versioned
wire order, raw fields and full non-Boolean byte values. Regeneration tests use
temporary directories and preserve the frozen fixture.

Run `cargo test --test flow_settings_body --test flow_fixed_fixtures --test flow_settings_fields --test flow_carriers`
and `python -m unittest discover -s docs/features/flow-settings-body -p test_body.py`.
Native loader execution, configuration mutation, engineering semantics and complete
Flow schema acceptance remain unverified. No global byte-coverage credit is
claimed. The maintainer humanizer pass remains for review because that skill is
unavailable in this environment.
