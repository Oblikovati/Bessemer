# Complete display archives down to version zero

Reference baseline: specification edition 0.262, dated 2026-10-25, rechecked
against edition 0.264. The intervening model additions are outside this checkpoint;
all seven recorded reference-reader hashes remained unchanged.

The 1394 complete part and assembly roots cover eighteen unsigned versions from
zero through 596, with 377537 independently expected traced and physical fields.
They exercise both sides of every remaining low-version predicate. The archive
decoder now accepts version zero and applies the documented predicates directly.
The uncounted-window interval `0x1FAE`–`0x1FCE` remains explicitly unsupported.

Appearance records introduce colour after 144 (`0x90`), HLR colour after 172
(`0xAC`), and wire colour after 211 (`0xD3`). Their physical order remains colour,
wire colour, then HLR colour when present. Materials introduce transparency and
emission after 370 (`0x172`). The four earlier material doubles and both strings
remain stored throughout these checkpoints.

Old-light names and the entire user-property suffix begin after 380 (`0x17C`).
Earlier light bases end after their six doubles, three vectors, colour and
suppression word. All five old-light classes have independent fixtures, raw
base fields and exact writes. The builder checks that removing the name suffix
changes no class or object slots.

Panes omit perspective through 437 (`0x1B5`). Temporary edge records omit their
unknown final word through 561 (`0x231`). Buckets omit their display-item list
through 595 (`0x253`); the root-owned item list remains independent and populated.
The builder locates each omitted value or record through the complete reference
reader, verifies its bytes or identity, and removes only that observed span.

Sixteen additional roots each place 128 minimal empty edge records in both
buckets. A record occupies twenty bytes through 561 and twenty-four afterward.
These vectors prove the physical stride used by the count guard; they include
exact edits to the stored edge word and complete reverts. Negative empty source
lists are replaced explicitly rather than silently skipping a bucket.

Tests compare every retained typed field and MFC tag against the complete 596
control. They also check each member's presence, isolated point-word edits,
composite edits, truncation rejection and concurrent reconstruction. The 144
inline roots each retain two complete facets, checked against independently
built full records. Earlier fixture groups remain unchanged.

Eight owned containers at zero and 596 exercise part and assembly edits,
replacement, reopen and exact reverts while preserving unrelated streams.
Version zero resolves from its explicit DLUpdateStamp marker. Root and container
verifiers compare forced writes in two fresh processes.

The Windows verification run also reconstructed all 1086 native display streams
and 20 native drawing containers exactly in two processes. Both accepted corpus
manifests remained unchanged. Reports beside this note record input, output,
context and runtime source hashes for replay.

This checkpoint removes the lower-version cutoff. It does not establish full
documented-schema parity: unresolved representations, remaining display-list
acceptance work and the broader model schemas remain open. Byte fidelity does
not establish geometric validity or native application acceptance.
