# Assembly configuration archives

Issue #99 implements complete ordinary assembly and routing configuration
archives, advancing the full parity goal in #2.
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

`AssemblyConfigurationContext` selects the root kind, wire version, real model
header and optional kernel context. `AssemblyConfigurationArchive` checks the
actual fresh root tag and loads the header's identities in native order.
Shared model stages accept that explicit assembly context; part-only readers
retain their part checks. Assembly unit components use the documented owner
classes and reject the unsupported external-model branch.

`from_fields` derives both section lengths and requires an exact typed reparse.
For routing roots, the resolved length ends before the routing trailer.
`edit_fields` returns an immutable result with completion, exact stored-schema
coverage, fidelity, diagnostics and revision-bound field history. Unchanged
edits retain the source revision; changed results reject source-only handles.
The archive exposes imported header origins without opening external documents.

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

The owned blank assembly supplies an independent oracle for the complete
reader: 23,359 bytes, 87 map slots and 5,056 primitive reads through physical EOF.
A separate name-edit golden adds eight bytes to the lightweight section, retains
the original map and passes the complete reference reader again. Replay with
`python docs/features/assembly-configurations/freeze_owned_assembly.py`; fixture
replacement requires `--write`. Public tests reproduce that independent edit,
reopen the whole document, preserve unrelated streams, revert exactly and keep
the original archive isolated. A composed routing fixture and its independent
trailer edit check that changed trailer lengths do not alter either section.

## Configuration verification

All 180 native configurations pass complete Bessemer reads and forced typed
construction, including their original shared-map slot counts. The separate
906-case modern-part corpus still passes after the shared context changes.

Nine retained native configurations cover references, patterns, cameras,
exploded profiles, relations, toolbox records and routing. Their 316,486 bytes
are bound to 70,293 actual reference primitive reads. Normal tests compare every
field's bytes and scalar width with that inventory and force reconstruction.
The oracle retains zero-length string reads while excluding them from the next
scalar's storage; a focused regression checks that boundary.

Twenty additional fresh object graphs cover mirror data, reference and component
transforms, circular pattern items, all five camera faceted-reference variants,
repeated attachment identities and assembly exploded-view IDs. Their 1,151
primitive reads cover all compact-transform payload modes. Every truncated
prefix and appended byte fails; malformed counts, operation budgets, old
versions and unselected contexts cannot publish partial results. These graphs
use actual scoped serializers and fresh maps, without preloaded identities.

Replay the retained native and object fixtures with
`freeze_native_assemblies.py` and `freeze_assembly_objects.py` in this directory.
Both check existing bytes by default; `--write` explicitly replaces fixtures.
`verify_reference_corpus.py --export` checks the installed reference corpus and
writes the local probe packet used by the ignored assembly corpus test. The
full local PR gate passed 2805 tests, including 1249 library tests and
277 doctests. Two independent native assembly processes reconstructed all
180 inputs exactly; the part corpus retained all 906 accepted outcomes.
`complete-verification.json` records the source hashes, corpus hash and
explicit exclusions.

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
independent maps. Assembly lightweight maps reuse this codec under their own
explicit model context.

The full local PR gate passed: 2792 tests, including 1242 library
tests and 277 doctests. The separate native modern-part corpus also retained
all 906 accepted configurations. Verification records the source hashes and
explicit application and geometry exclusions in `string-map-verification.json`.

This acceptance operation covers complete configuration reads, independent
edits, forced construction, document reopen and revert. Its successful archive
checks do not establish topology, geometry, self-intersection, geometric
tolerance or SolidWorks application acceptance. External documents are not
opened. Kernel and application acceptance remain separate requirements.

Corliss also exposes typed part, assembly and drawing model views and structural
edits. Bessemer's completed modern-part corpus is evidence for its selected
archive operation. It does not prove parity for those higher-level APIs, older
part layouts, assembly mate/feature streams, drawing models or installed project
formats. Those requirements remain in #2 after this configuration slice.

The maintainer-side humanizer skill was unavailable. A maintainer still needs
to review this prose.
