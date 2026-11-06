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
temporary directory and compares complete uncompressed fixture bytes. This is a
baseline before the typed count-prefix and binding API, not a complete body reader.
