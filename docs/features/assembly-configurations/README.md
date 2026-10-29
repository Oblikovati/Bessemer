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

The resolved stages through layers now also share one implementation, used by
the existing part reader. Six complete-part tests, nine inline-part tests,
workspace Clippy and all 906 native part configurations passed after this
structural extraction; every native outcome matched the preceding run.
`resolved-stage-verification.json` records the source and fixture hashes.

The owned blank assembly supplies an independent oracle for the next complete
reader: 23,359 bytes, 87 map slots and 5,056 primitive reads through physical EOF.
A separate name-edit golden adds eight bytes to the lightweight section, retains
the original map and passes the complete reference reader again. Replay with
`python docs/features/assembly-configurations/freeze_owned_assembly.py`; fixture
replacement requires `--write`. These reference results do not yet prove a
Bessemer assembly read or edit.

## MFC string-map prerequisite

The map codec preserves 16-, 32- and 64-bit count framing and all four text
length widths. It retains duplicate keys and wire order. The map-specific text
layout stores Latin-1 bytes and UTF-16 code units as typed fields, including
unpaired surrogates accepted by the reference. It does not change the existing
CString decoder's Unicode checks. Explicit input, entry and field budgets bound
the code-unit loops before allocation.

Twenty-eight independent graphs cover those framing combinations, empty maps,
code-unit edits and a tagged bend-note caller. The actual reference produced 611
primitive reads. A profile of its real `string64` calls identifies the raw text
spans; the fixture packet retains both that original inventory and its exact
788-primitive byte/code-unit projection. No raw span outside those identified
string payloads is projected. All fixture bytes regenerate from immutable
reference sources.

The codec is also connected to the existing bend-note format reader. A complete
owned part configuration contains the new tagged map, with later MFC references
relocated only at observed pointer positions and the resolved size updated.
Public archive tests force reconstruction, apply an independent key edit,
reopen the edited document, preserve unrelated streams and revert exactly.
Wrong-width edits fail without changing the source, and parallel reads use
independent maps. This prerequisite does not yet implement assembly roots.

The full local PR gate passed: 2792 tests, including 1242 library
tests and 277 doctests. The separate native modern-part corpus also retained
all 906 accepted configurations. Verification records the source hashes and
explicit application and geometry exclusions in `string-map-verification.json`.

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
