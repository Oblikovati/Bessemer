# Flow record 112 selected values

Issue: #241. References: `../inventor-interface/solidworks/flowrecord112.py`,
`flowlinkrecords.py` and `flowauxrecords.py`.

Record 112 contains raw 64-bit patterns with no version or count prefix. A
complete fresh-map context determines its count from complete record-402 links
and record-316 state bytes. Selection requires an accepted converted link code
and a state byte at least 128. Nonzero low-bit states do not select entries.
Empty state arrays select nothing even when links exist. The dense map retains
every unselected entry as absent, separately from selected zero-valued patterns.

The FSL1 baseline freezes 2,729 complete contexts and projections across three
packets, each under eight million uncompressed bytes. Every case retains both
input payloads, decoded input images, physical link spans, raw/converted codes,
the complete dense map, selected ordinals, raw value patterns and optional
per-entry projections. Fresh-map, completion, owner-binding, scatter and
engineering qualifications are recorded with the original schema results.

Versions zero and nine cover every combination of the 256 raw codes and 256
state bytes. The other eight versions cover every code with states
`0`, `1`, `127`, `128`, `129` and `255`. Cases also cover empty links, absent
states, mixed codes, selected zeros, every 64-bit single bit and complement,
negative zero, infinities and NaN payloads. Frozen raw and converted code columns
keep version-zero conversion separate from the accepted-code predicate.

The native context uses record 402 from CPT and records 316/112 from FLD. Its
5,532 links derive 3,876 selected values and exact record-112 EOF at 31,008 bytes.
The freeze checks each whole source file, selected occurrence, record extent,
payload hash, complete original context and complete original projection against
the dedicated native manifest and its evidence hash. Matching GUID and record-129
tag observations do not prove automatic pairing or native owner binding.

Run `python -m unittest discover -s docs/features/flow-selected-values -p test_selected.py`
and `cargo test --test flow_selected_value_fixtures`. Python checks the independent
35-code truth set, high-bit selection, all dense maps, optional projections,
raw-value framing, context guards and temporary-root regeneration. Rust checks
the same independent selection rules and binds every frozen input to the
existing `FlowLinkRecords` and `FlowByteStates` views. Compression checks reuse
the existing exact-stream fixture helper; state-header no-ops retain complete
compressed carrier bytes and physical history.

`FlowSelectionContext::derive` takes complete `FlowLinkRecords` and
`FlowByteStates` inputs with explicit `FlowSelectionContextOptions`. The caller
declares `FreshEmptyMap`; actual map entries and selected ordinals have independent
bounds. State arrays must be empty or match the physical link count. Contexts
retain both immutable input revisions and do not update when callers edit those
inputs elsewhere. Coverage distinguishes exact fresh-map derivation from native
owner and runtime acceptance.

`FlowCarrier::record_112_selected_values` checks a complete raw payload under
that retained context and independent byte/value bounds. A complete empty
selection admits a zero-byte payload and zero budgets. `values` borrows dense
raw patterns; `entry_value` and `entry_value_id` preserve unselected entries
separately from selected zero values. Value identities bind the value revision,
both input revisions, link transport and the fresh-map assumption.
Identities belong to parsed checkpoints; parsing identical bytes again creates
a foreign revision. Context-isolation checks reuse the same value checkpoint
while changing only a retained input revision.

`replace_value` accepts a checked identity and raw bits, edits a copy and reopens
the entire payload under the same context and bounds. Selection stays fixed.
The reused `FlowSimpleTableEdit` returns immutable checkpoints and full physical
carrier history. No-ops preserve original bytes, while fresh reversals recover
the whole carrier exactly. Stale and foreign value/context identities fail
without mutating the original snapshots.

Typed reads check all 15,104 stored patterns and every optional entry projection
in the 2,729-case corpus. Edit checks cover every generated pattern and distinct
first/middle/last native values: 11,231 positions, including three of the 3,876
native values. Each selected edit has an independent byte patch, no-op check,
complete history and fresh whole-carrier reversal. Two fresh processes compare
the same source-checked contexts, values and selected edits byte for byte. Eight
workers compare concurrent derivations and edits against serial results.
Compile-time checks cover ten new public types and two reused wide/edit types.
The native edit policy is selected coverage; full reads cover every native value.

Native runtime scatter, reused nonempty maps, owner binding, engineering meanings
and SolidWorks application acceptance remain outside this fresh serialized profile.
