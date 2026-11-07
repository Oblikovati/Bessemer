# Extruded reference-surface interoperability

Bessemer reads, reconstructs and edits complete ordinary-part extruded reference
surfaces through the existing resolved-feature API. This advances Feature #54
and goal #2. The reader uses the complete body-feature base, requires a nonnull
end specification, retains the optional ordered-face helper and reads the three
raw words and direct object array documented in section 11.153.

Ordered-face helpers preserve two independent signed group counts. Each group
stores all unsigned words before its direct arrays; the words and arrays are
not interleaved. Every nested array retains its own ReadCount, item order,
nulls and shared references. Negative and oversized group counts fail explicitly.
Direct arrays allocate no MFC slots. Tagged helpers and keepers retain normal
class/object-map entries.

The concrete keeper has zero body bytes when archive-XML mode flag `0x10` is
clear, as established by the native serializer evidence. External model XML
storage does not imply that archive mode flag. The reader requires the explicit
ordinary context for the empty-body rule. An independent native audit verifies
the keeper in the inherited field of 26 complete surface roots across 21
hash-bound configurations. Its exact class is admitted to that field's existing
finite family; unrelated classes still fail. `native-keeper-role.json` retains
those source-reader observations and exact-EOF checks.

Independent fixtures cover 52 complete surfaces, eight standalone helpers and
two tagged keepers at four version contexts, including both sides of the
inherited end-specification gate. Their 13,948 primitive observations partition
every byte. Tests compare all primitives, bytes and final slots, reject all
truncated prefixes and trailing bytes, reject old/unspecified contexts, check
required and declared pointer roles, and reject malformed group counts. They
also check unsigned extrema and words-before-arrays ordering. The original 58
preparation cases remain unchanged; four positive inherited-keeper cases extend
the matrix. Populated groups and noncanonical words are synthetic; native helpers
in the documented frontier have empty groups.

Five owned streams use the blank part's actual model and CMgr state with 97
inherited slots. They cover null/empty/populated helpers, shared end and face
references, self references, and distinct inherited/array keepers with identical
empty bodies. Public tests reconstruct whole documents byte for byte. A raw-word
edit matches a separately constructed expected stream, preserves every alias
and unsigned group value, records one field-history change and leaves source
archives and documents unchanged. Three Python tests verify deterministic
regeneration, matrix coverage and invalid construction options.

The staged `cargo xtask pr` passed on Windows x64 with Rust 1.99.0: 457 library
tests, four xtask tests, all integration suites and 265 doctests. Three external
corpus gates remain ignored by default. Seven private reader tests, two public
operation tests and three Python fixture tests passed. `local-verification.json`
records the executed scope and runtime/test source hashes.

Two fresh native corpus processes reconstruct 548 of 906 feature configurations
exactly with identical outcomes and failure causes, retaining all previous 532
successes. The broad parity gate still fails for 358 explicit stops. The
independent reader verifies all 548 accepted streams, totaling 28,360,691 bytes,
against four-stream input hashes. `frontier-checkpoint.json` and
`accepted-reference.json` retain those observations. Full documented schema
parity remains incomplete.

`shared-dispatch.md` records the separate dispatcher extraction. `reference.json`
binds fixture generators, reused builders and reference readers.
`schema-snapshot.md` preserves the relevant specification sections with raw-byte
hashes in `schema-reference.json`. Runtime file version 19000 remains distinct
from model wire version. Database paths, archive-XML metadata mode, historical
contexts outside the modern scope, geometry, topology and native application
acceptance remain outside stored-schema validation. No dependency or package
version changed. The humanizer skill is unavailable; its prose pass remains with
the maintainer.
