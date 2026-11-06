# Flow records 122 and 434 owner-state projections

Issue: #241. Reference: `../inventor-interface/solidworks/flowauxrecords.py`.

Record 122 stores exactly 50 raw 32-bit words in 200 bytes. It has no version
prefix. The first word gates the documented native update projection: only
`0xffffffff` disables updates. Other values, including high-bit values, enable
updates. All 50 words remain serialized even when they do not contribute to that
projection.

When enabled, words 5, 7, 1, 8 and 2 supply signed updates for native member offsets
`0x8`, `0xc`, `0x10`, `0x14` and `0x18`. The remaining two updates are constants:
`0x1c = -1` and `0x20 = 0`. These are native member offsets, not engineering names.

Record 434 stores a signed version of zero or one, six signed words and one
unsigned state byte, for exactly 29 bytes. The six words update member offsets
`0x8` through `0x1c`; the byte updates `0x20`.

Selection first looks for record 434, then record 122. It chooses the latest
physical occurrence of the preferred identifier before parsing any payload.
A malformed selected record fails; it does not fall back to an earlier occurrence
or the lower-priority identifier. Older malformed records are retained but do not
affect parsing of a later valid selected occurrence. An absent record differs from
a selected legacy record whose gate disables all updates.

The owned FOS1 packet freezes 4,613 complete records and 27 complete selection
carriers, totaling 1,775,516 uncompressed bytes. Every legacy word and modern
member has single-bit and complement coverage. Both modern versions cover every
state byte. Each projection retains all raw fields, optional version/state
presence, exact EOF and seven independently optional updates. Selection outcomes
distinguish success, completed absence and rejection.

Three native records come from separately hashed CPT and FLD sources: two identical
record-122 payloads and one version-one record-434 payload. The freeze checks whole
source digests, record extents, payload digests and complete original projections.
The two native selection fixtures contain only the relevant records, preserving
their relative physical order. They are projected carriers, not copies of the
complete original files. Their digests and source revisions appear in
`baseline-verification.json`.

Run:

```text
python -m unittest discover -s docs/features/flow-owner-state -p test_owner.py
cargo test --locked --test flow_owner_state_fixtures
```

Python checks independent raw update equations, exact gates and selection order,
all proper prefixes, extra bytes, unsupported modern versions and wrapper checks.
It also regenerates the packet under a temporary root. Rust reconstructs every
field, checks the independent member equations and verifies the frozen selection
outcomes against ordinary typed carrier lookup.

The typed readers, isolated field editors and update projection API remain to be
implemented. No native runtime updates were executed. Engineering interpretation
and SolidWorks application acceptance remain unverified.
