# Beam Property implementation decisions

The shared load reader already handles reference containers, counted blobs,
embedded archives and most scalar values. Beam Property adds two physical
signed 16-bit fields. The current shared public value enum has only 32-bit
signed words; encoding either short as that variant would change its wire
width. Adding a variant also affects exhaustive callers across already shipped
families. The local implementation uses a beam-specific value enum with a signed-short
variant and a shared-value variant. This keeps the existing public enum and all
its callers unchanged. Private short wire frames use the existing raw-byte
writer; the public beam view exposes their signed two-byte type. Construction
and edits validate both the public type and the complete parsed wire shape.
Shared bytes cannot masquerade as a signed-short replacement.

The child body is independent of the manager header and Material stream. Its
reference-container prefix must use existing typed embedded archive profiles,
including complete object-map and copy/edit history. Reuse shared reference
reading rather than reproducing its thresholds. Owned profiles must exercise
legacy and modern references, null and nonnull archives, every beam-specific
threshold neighbor and existing reference gates.

The complete native denominator is C-725: 163 Property streams, 43,516 bytes,
163 accepted attempts and 501 counted triples. Compare each exact retained
attempt, physical span and reconstructed byte stream after fresh extraction.
First retain full reports and source hashes, then replay the unmodified actual
parser from a frozen source copy with its explicit archive callback. Merely
reading the old report does not satisfy that gate.

Operation acceptance requires complete typed construction and edits, all source
and replacement parents, ordered outer and nested history, exact resource
budgets and source isolation on success and failure. Whole-body framing does
not establish link meanings, units, geometry or native application acceptance.
