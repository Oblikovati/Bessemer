# Flow count prefix and conditional array binding

Issue: #241. References: specification sections 14.39 and 14.44,
`../inventor-interface/solidworks/flowcontextcounts.py` and `flowcountbinding.py`.

Record 406 starts with a positive signed version and nonnegative G.Fluid and
G.Solid counts. The mathematical sum and native signed 32-bit wrapped sum are
distinct. The prefix identifies a legacy body through version 17 and a delegated
body from version 18. Reading the prefix does not validate that body; every
remaining byte stays opaque under this scope.

The documented record-433 count assignment is conditional on initializing an
absent pointer map. It requires an explicit assumption and a mathematical sum
within the nonnegative signed allocation domain. Under that condition, the map
has eight-byte pointer slots, the solid partition begins at G.Fluid and its byte
offset is eight times that count. These are layout calculations, without native
allocation, pointer population or execution. A reused map is outside this binding.

The baseline contains 193 count-prefix views and 193 bound array payloads.
It covers raw/zlib transport, empty and arbitrary opaque bodies, versions
1/7/8/17/18 and the signed maximum, zero counts, large pointer layouts and wrapped
sums that cannot initialize the map. Small counts exercise selector-zero and
positive-selector arrays with signed extrema. The native case binds the CPT
prefix `[3368,2584]` to the CFLD's 5,952-element record 433 while retaining the
unexecuted initialization assumption.

Rust independently checks prefix bytes, wrapped arithmetic and pointer layout,
then reopens every array with the frozen conditional count and checks all signed
words. Python pins an independent array hex oracle and the distinction between
conditional count matching and actual native execution. Regeneration uses a
temporary directory and compares complete uncompressed fixture bytes.

`FlowCarrier::record_406_context_counts` reads a revision-bound record with
explicit raw/zlib transport and a complete decoded-byte bound. It retains
the original carrier and every decoded byte. Coverage identifies exactly
twelve known prefix bytes and reports the remaining body as opaque and
unvalidated. The version-selected body branch is metadata, not body acceptance.

`initialize_absent_map` requires the explicit `InitializeAbsentMap` assumption.
It rejects mathematical sums outside the signed allocation domain and returns
the conditional count and pointer partition layout without allocating a pointer
array. The binding retains the original count-prefix checkpoint. It does not
support an arbitrary existing map or claim native execution or pointer population.

`FlowInitializedCountBinding::read_record_433` applies that count through the
existing exact array reader. Callers supply independent byte/element limits,
without overriding the bound count. The returned view retains both the binding
and complete arrays, including their original carrier checkpoints. Count input
and array input may come from separate carriers, as in the native CPT/CFLD case.
Existing fixed-cell edits remain available through the array view and retain
the count assertion; callers can reopen the result under the same binding.

Validation compares every field and byte in all 193 prefix views and 193 bound
arrays. It checks known-prefix truncation separately from accepted opaque raw
tails, strict zlib boundaries, exact and one-below byte/element limits, invalid
signed domains, record identities and count mismatches. Eight workers read the
native conditional binding after caller-owned input buffers are mutated.
Two fresh processes match the complete independent Source packet byte for byte.
