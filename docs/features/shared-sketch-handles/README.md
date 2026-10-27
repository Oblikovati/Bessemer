# Shared sketch-handle serializers

Bessemer reads, edits and writes `sg3DParametricSurfaceHandle` and
`sgParabolaHandle` in ordinary modern non-DB external-XML part archives above
version `0x27CB`. Specification 11.341 / C-383 binds both primary serializer slots
directly to the existing entity-handle serializer. Their retained native parent
chains include `sgEntHandle`; the installed module hash was checked against the
reference evidence in `native-ancestry-reference.json`.

The finite existing handle family now includes these two exact class names.
Its existing codec and declared-role callers perform all reading and writing.
The body contains a u16 entity index, an additional signed i32 index only when
the stored u16 is `0x777F`, then signed i32 reference-ID and dimension-on-CM words.
There is no derived suffix or extra map allocation. Even a small index stored
using the escaped representation retains that representation after unrelated
field edits. Native entity lookup and object repair are not executed offline.

Independent readers verify 144 complete records and 2,696 primitive observations
across both classes, three versions, eight index encodings and three pointer
styles. Cases include standalone objects, shared/null/shared array entries and
complete component-sketch-entity handles whose declared entity pointer names
the new classes. Index cases cover both sides of the sentinel, the sentinel
itself, unsigned u16 maximum, signed i32 extremes and an escaped small index.
Every primitive, byte and map slot is compared. Private tests also reject every
truncated prefix, trailing bytes, old contexts, unrelated cached component
classes and similar class names.

Eight owned resolved streams import the real 97-slot configuration map and
finish at 99 or 101 slots. Two independent goldens change the component-owned
handle's reference ID without changing its escaped small index. Public tests
reconstruct whole documents and check isolation after source mutation and
rejected escape-marker edits. Two Python tests regenerate the complete records,
observations and independent edit goldens. Existing fixture builders are reused
without changing their defaults.

The approved section and hashes are retained in `schema-snapshot.md` and
`schema-reference.json`; `reference.json` records the complete independent
observations and reader-source hashes. Only these two handle bindings from
section 11.341 are added here. Native geometric validity, arbitrary index
validity, native application acceptance, special/DB and historical contexts,
other runtime handle classes and full documented file-schema parity remain
unproven. No dependency, runtime function, exported Rust API or package version changed.
The humanizer skill is unavailable; its prose pass remains with the maintainer.

The full local PR gate passes 559 library tests, four xtask tests, every integration
suite and 265 doctests; three explicit external-corpus tests remain ignored.
Two fresh native processes preserve all 709 earlier complete configurations,
with identical outcomes and causes and a stable executable hash. The one changed
case now passes its parabola contour handle and stops at the next unsupported
point-backup dependency. It receives no completion credit; 197 cases still fail.

The accepted set is unchanged, so `accepted-reference-basis.json` hash-binds the
existing independent exact-EOF report for those same 709 configurations and
verifies all four input hashes per configuration against the unchanged native
corpus. That prior audit covers 42,468,950 feature bytes under its recorded
1,143-module source snapshot. This is reuse of matching independent observations,
not a claim that a new full reference replay ran.

[Additional conic/cylinder handles and backups](../sketch-geometry-backups/README.md)
are covered separately by the native bindings in section 11.342.
