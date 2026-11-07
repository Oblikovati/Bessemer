# Derived-feature and surface-body references

Bessemer reads and reconstructs the ordinary modern `moDerivedFeatRef_w`
archive described in section 11.216, evidence C-244. It retains the wrapper,
feature pointer, unsigned and signed words, plane and axis data, direct object
array, and optional point. The signed point-presence word exists only after
version `0x2F85`; every nonzero value selects all three stored doubles.

`moSurfaceBodyRef_w` reuses the complete body-reference grammar without an
additional suffix. Both classes participate in the shared finite wrapper
family, which now contains 23 classes. Fresh and cached references retain
object identity, including repeated array entries and references to the root.
Unsupported nested classes, including a populated coordinate-data object,
remain explicit failures rather than receiving opaque coverage.

Independent reader fixtures cover 132 reference graphs and 180 generic-caller
graphs, with 49,404 primitive observations and 74 owned document fixtures.
These exercise all five existing caller forms, version boundaries, signed
presence values, floating-point bit patterns and independent edit goldens.
Five focused Rust tests and three public save/reopen and isolation tests passed.
Two immutable-reference tests regenerated every fixture without changing the
original caller baselines.

Two native corpus processes produced identical statuses and failure causes for
all 906 configurations. Exact typed reconstruction increased from 848 to 851,
adding configurations 347, 349 and 350 without losing a previous completion.
The independent source reader reached exact EOF for all 851 accepted inputs,
covering 67,980,353 resolved bytes and checking all four source-stream hashes.
Its native containing-root regression also verified the retained reference
words, point, target classes, array count and source-bound span.
The aggregate corpus still fails because 55 configurations remain incomplete.
This proves archive reconstruction for these inputs; whole-schema parity and
native application acceptance remain unfinished.

The humanizer skill was unavailable; its prose review is left to the maintainer.
