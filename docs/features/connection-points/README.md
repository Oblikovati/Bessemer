# Connection-point and route-point interoperability

Bessemer reads, reconstructs and edits complete ordinary connection and route
features through the existing resolved-feature API. This advances Feature #54
and parity goal #2. Section 11.165 establishes their shared sketch-point layer:
the complete reference-geometry base and a point-reference tag. This layer has
no reference-point data object from section 11.163. The reader retains that
distinction and each feature's own suffix.

Connections preserve both line references, four dimension occurrences, signed
words, vectors, four direct configurable parameters and every compatibility
value. The direct parameter order is `798`, `7f8`, `7d8`, `7b8`; each body has
two CStrings and no class tag or map slots. The temporary parameter likewise
remains direct. All thirteen string occurrences and both temporary doubles
remain intact, including nondefault values that native loading may discard.
Routes retain their line-reference tag and vector without a connection suffix.

Tagged `moConfigurableParam_c` objects use normal MFC map entries and the same
complete two-string body. `ThreeDRadialOrDiameterDim_c` uses the complete
geometric-dimension base followed by a signed four-byte word; it is distinct
from the angle dimension's two-byte suffix. The existing dimension-handle
family already admits this class, and complete handle callers now decode it.
The inherited atom-owner family admits the two new native-proven feature
classes. Unknown classes and unrelated pointer targets still fail explicitly.

Independent readers consumed 124 complete cases to exact EOF, recording 19,502
primitive observations. Eight versions cover inherited geometry, temporary-axis
and dimension-string gates. Empty/populated connection and route cases retain
repeated line/dimension identities, ordered direct parameters, negative-zero/NaN
compatibility bits and nonempty temporary strings. Standalone parameter,
geometric-dimension and real handle cases distinguish direct/tagged storage,
all selected geometric flags and the full suffix width.

Six owned streams import the blank part's actual model and CMgr state with 97
slots. Public tests reconstruct whole documents and match a Unicode
compatibility-string edit against separately constructed expected bytes,
preserving temporary values, aliases, history and source inputs. Four private
tests cover every primitive, reconstructed byte and slot, all truncated prefixes,
trailing bytes, old contexts and unrelated pointer families. Three Python tests
verify deterministic regeneration, matrix coverage and invalid boundaries.

Two fresh native processes produce identical outcomes and failure causes.
All previous 611 successes remain, and 630 of 906 configurations reconstruct
exactly. All nineteen advanced frontiers complete. The independent reader
verifies all 630 accepted streams to exact EOF, totaling 36,734,048 bytes, against
four input-stream hashes per case. The broad parity gate still fails for 276
configurations. `frontier-checkpoint.json` and `accepted-reference.json` retain
these measurements.

The audit uses an immutable workspace copy of 1,052 Python files, verified
against a stable before/after hash map of the approved source checkout. Its
hashes stay unchanged throughout the audit; the temporary copy is removed after
verification. Its identity, original location and source hashes are recorded.
`reference.json` binds generators and reused builders; `schema-snapshot.md` and
`schema-reference.json` retain the relevant clauses and hashes. Full documented
schema parity, native geometry, topology and application acceptance remain
outside this checkpoint. No dependency or package version changed. The humanizer
skill is unavailable; its prose pass remains with the maintainer.

The staged `cargo xtask pr` gate passed on Windows x64 with Rust 1.99.0:
486 library tests, four xtask tests, every integration suite and 265 doctests.
The three external corpus gates remain ignored by default; native results above
come from their separate explicit runs. `local-verification.json` records the
source and gate-log hashes.
