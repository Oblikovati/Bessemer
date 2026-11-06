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

The record-112 context/read/edit API is still to be implemented against this
baseline. It must retain immutable input revisions and the explicit fresh-map
assumption, edit existing values without changing selection, completely reopen
results and return carrier history. Native runtime scatter, reused nonempty maps,
owner binding, engineering meanings and SolidWorks application acceptance remain
outside this fresh serialized profile.
