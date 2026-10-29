# Assembly configuration archives

Issue #99 advances the full parity goal in #2. Implementation is pending.
Clauses 11.120–11.128 describe complete ordinary modern assembly configuration
streams. The frozen snapshot includes the later clauses that complete the
earlier prefix-only descriptions.

The actual reference reader completed all 180 modern configurations: 174
`moAssembly_c` roots and six `moRouteAssembly_c` roots, totaling 5,671,757 bytes.
`verify_reference_corpus.py` replayed those files from an immutable copy of the
reference sources. It checked the input hashes, physical EOF, both inherited
section boundaries and the unit component's identity link back to its containing
root. No reader functions were replaced. The manifests record three additional
version-10000 configurations outside this modern layout.
Manifest snapshots normalize text line endings; a temporary-file regression
checks LF/CRLF equivalence without changing escaped string values. Native stream
and header hashes remain byte-exact.

The assembly root has the same basic external-header import order and two
bounded model sections as the part. Its section bodies have assembly-specific
continuations and omit the part-only stages. A routing root uses the assembly
sections, then reads its own trailer outside the resolved section. Its strings
and two unsigned time words retain their original values; their application
roles remain unresolved.

The implementation should first reuse the shared model stages without changing
the existing part results. Root kind must be checked from the actual tagged
object, and archive context must remain explicit. Assemblies must not borrow a
part context or a synthetic object map. Assembly references, exploded profiles,
local reader selections and root-owner checks need the documented assembly
rules. Keep input ownership and completed-root proofs separate from an
in-progress object's map identity.

The shared lightweight model stage has been extracted from the part-only suffix
as a separate structural change. All 30 existing part, lightweight, inline-part
and public forming-tool tests passed, along with workspace Clippy. The complete
906-case part corpus retained every accepted outcome, and the owned native,
independent edit and primitive-oracle fixtures retained their original hashes.
This extraction adds no assembly context or public assembly API by itself.

The first acceptance operation is a complete configuration read, independent
edit, forced construction, whole-document reopen and revert. Add precise
primitive oracles, malformed bounds and pointer-role tests, preserve unrelated
compressed streams and rerun the complete modern-part gate. Compare native
process results and retain explicit kernel, geometry and application acceptance
exclusions. The reference baseline establishes the target; it does not establish
that Bessemer implements it.

Corliss also exposes typed part, assembly and drawing model views and structural
edits. Bessemer's completed modern-part corpus is evidence for its selected
archive operation. It does not prove parity for those higher-level APIs, older
part layouts, assembly mate/feature streams, drawing models or installed project
formats. Those requirements remain in #2 after this configuration slice.

The maintainer-side humanizer skill was unavailable. A maintainer still needs
to review this prose.
