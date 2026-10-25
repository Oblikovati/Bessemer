# Components before viewer metadata

Reference baseline: specification edition 0.257, dated 2026-10-25. Reader hashes
are recorded in the fixture manifest.

The 402 complete roots cover versions 903, 904, 915, 916, 930, 931 and 989,
with 120703 independently expected traced and physical fields. The fixtures
retain cached, empty, external-face and inline-facet models, nested buckets,
populated text and all five old-light classes. Earlier fixture groups remain
unchanged.

Before 931 (`0x3A3`), an assembly shaded component directly stores its identity
string, transform, bounds and appearance pointer. Later versions embed the
viewer-component record, which appends configuration and model metadata to
those same four fields. Both branches use the shared identity decoder and bind
external face context from the stored component string.

The independent builder observes the appearance pointer in a complete later
component and removes only the following primitive metadata. It checks that the
suffix introduced no MFC class or object slots. Tests compare the four identity
fields and every graph tag against the corresponding later root. Whole-stream
field and byte checks also cover external-face resolution in the old branch.

User environments omit the two browser words through 903 (`0x387`). The builder
locates them from the end of the sole splitter and verifies both stored values
before removing them. Polygon type words and their variable double arrays are
absent through 915 (`0x393`), including polygons nested inside edge visibility
records. Independent builders remove complete type records; tests inspect every
populated polygon and keep its coordinates intact.

View activity uses the same stored i32 across these checkpoints. Before 916,
the native reader turns nonzero words into logical one. Bessemer retains the
physical word; the comparison adapter performs the logical conversion only for
the native field trace. The fixtures store -7 and check both sides of this
boundary, including the independent physical scalar oracle.

Tests exercise complete edits and exact reverts without source mutation,
truncation rejection and context changes that require newly stored fields.
A part root remains compatible across the assembly-only viewer transition.
Fifty-six inline roots each contain two complete facets, checked against the
independent records from the preceding checkpoint: no facet layout changes in
this interval. Eight owned part and assembly containers at 903 and 931 exercise
replacement, reopen and revert while preserving unrelated streams. Root and
container verifiers compare forced writes in two fresh processes.

The Windows verification run also reconstructed all 1086 native display-list
streams and 20 native drawing containers exactly in two fresh processes. Both
accepted corpus manifests remained unchanged. Verification reports beside this
note record the fixture, output, context and runtime source hashes for replay.

This checkpoint admits archive contexts from 903. Earlier documented layouts,
unresolved representations and model-feature additions remain tracked by #28
and the broader parity work. Byte fidelity does not establish geometric validity
or native application acceptance.
