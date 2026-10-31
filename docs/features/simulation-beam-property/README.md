# Beam child Property

Feature [#142](https://github.com/Oblikovati/Bessemer/issues/142) covers the
complete direct beam child Property body in specification 14.349, C-725.
The retained reference reports 163 streams totaling 43,516 bytes. This initial
snapshot preserves the clause, full report and source hashes. Fresh replay now
matches every retained native outcome and all physical spans. Seven checks pass,
including original boundary tests, complete source hashes, installed function and
threshold bytes, and 104 independent owned profiles. This is a reference
foundation; no production beam Property API has been accepted yet.

The body starts with a direct persistent-reference container, followed by raw
words, two signed shorts, text, versioned section values, counted interleaved
triples, a linking word and historical helpers. Embedded references use explicit
archive dependencies. It has no outer collection count or common-load tail.
Signed and unsigned values must keep their original bit patterns.

The six beam-specific thresholds are 810029, 820007, 840014, 840024, 850035 and
850036. Shared reference thresholds also apply. Native range repairs and defaults
must not rewrite persisted values. Material streams are separate and covered by
[#140](https://github.com/Oblikovati/Bessemer/issues/140).

Complete construction and edit/history oracles, resource limits, broader
malformed-input cases, isolation,
concurrency and local/platform gates remain required. Units, link domains,
geometry generation, native writer and application acceptance remain unresolved.

Run `python -B -X utf8 docs/features/simulation-beam-property/freeze_beam.py`
for strict replay from an immutable source copy. `--write` initializes evidence
for review. The complete native input/span packet stays under `target/`; owned
literals and spans are committed under `tests/fixtures/simulation-beam-property/`.
No target code executes. The initial report and current source snapshot retain
the distinction between historical evidence and this fresh replay.
