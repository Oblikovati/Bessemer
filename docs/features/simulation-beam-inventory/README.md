# Complete retained BeamManager corpus

Feature [#146](https://github.com/Oblikovati/Bessemer/issues/146) adds a combined
fresh interoperability gate for specification 14.351, C-727. The original
reconciliation binds 1,080 stream identities /620,654 logical bytes across
manager headers, beam Properties, Material, JointGroup, JointList and Joint.
It reconciles retained reports; it is not a fresh combined reader replay.

The [candidate denominator](candidate-denominator.json) preserves 1,114 arguments:
1,084 accepted and 30 failed. Manager headers contribute 387 arguments over 353
occurrences, including four additional accepted alternatives and all 30 failed
ones. Preserve every argument; do not pick a first-success version and infer
native dispatch. Count stream identities and logical bytes once per occurrence.

Fresh combined Source replay now recovers every original envelope/nested stream
and matches all original outcomes. All six families run together through typed
Bessemer reads and complete validated construction/write operations. The native
identity manifest contains metadata only; renamed/substituted/missing/extra
identities fail exact comparison. Duplicate identities and unsupported family
tags fail before overwrite or opaque fallback. Native payload bytes stay under
`target/`; the exact original C-727 report and clause remain retained here.

No native BeamFolder fixture is added. Storage framing does not establish
physical meaning, adjacency/enumeration order, runtime repairs, native writer or
application acceptance, geometry or all historical forms. The full local gate passed; exact-head platform integration remains pending.

Four strict copied-source checks and five Rust tests pass, including the explicit
full native packet. Two fresh Rust processes match all 845,315 packet bytes
exactly against independent Source expectations. The owned CI fixture exercises
all six families, two accepted header versions and a rejected alternative.
The [combined verification](combined-verification.json) records complete counts,
hashes, identity scope and exclusions.

Run `python -B -X utf8 docs/features/simulation-beam-inventory/freeze_inventory.py`
for strict fresh replay. `--write` initializes evidence for review. Set
`BESSEMER_BEAM_INVENTORY_NATIVE_INPUT` to
`target/simulation-beam-inventory-native.bin` to run the native Rust test; optional
full exports use `BESSEMER_BEAM_INVENTORY_OUTPUT`.

Bug [#147](https://github.com/Oblikovati/Bessemer/issues/147) corrected the new
verifier's missing reference-only dependency path. The existing bundle is reused;
no runtime dependency or production behavior changed. The
[correction record](reference-dependency-correction.json) retains the initial
import failure and successful write/strict replay regression.

The [full local gate](full-local-verification.json) records the exact committed
implementation, platform, toolchain, complete test totals and log hash.
