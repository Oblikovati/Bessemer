# Mold parting geometry

Feature issue: [#217](https://github.com/Oblikovati/Bessemer/issues/217).

The ordinary modern part codec composes the reference-surface base, the direct
edge-reference collection, the signed method and a directly serialized
`moMoldPartingGeomData_c`. The settings body has no tagged object header.
The reference is inventor-interface specification edition 0.3099, sections
11.1287 and 11.1290–11.1295.

The installed DLL's primary table selects a zero-return implementation for
virtual slot `+0xAB0`. This establishes inline inherited surface framing,
including populated sheet/body branches, through the existing strict kernel
codec. The byte locations and source identity are recorded in
[native-binding-investigation.json](native-binding-investigation.json).
This is static evidence; no target code was executed.

Selector `+0x170 == 1` invokes a pre-existing child at runtime member `+0x178`
directly. Constructor `0x18017A2E0` allocates that child when its method argument
equals 1 and calls constructor `0x1801D8FF0`. That constructor installs primary
table `0x180441348`, whose RTTI names `moPartingDataNew_c` and whose serializer
slot selects `0x1801DAAE0`.

[native-child-capture.json](native-child-capture.json) retains the constructor,
serializer and one pointer-reader helper's instruction bytes. The serializer
visits three direct collections and four pointer fields. Named imports bind
three pointers to `moRefWrapper_c`, `moLoftSynchRef_c` and `moDirectionSpec_c`.
Ghidra confirms the fourth pointer is `moMoldPartingConnectedComponent_c`, the
base call is `su_CObject::Serialize`, and all three direct collections are
`suObArray`. Selected provider records are retained in
[child-serializer-rea.json](child-serializer-rea.json) and
[child-constructor-rea.json](child-constructor-rea.json), with their evidence
identities and limitations. The connected-component serializer calls the common
object base, two direct `suObArray` members (`+8`, `+0x90`), a direct
`suDWordArray` (`+0xB0`), a signed word (`+0x80`) and a tagged
`moLoftSynchRef_c` pointer (`+0x28`). Its recovered constructor and serializer are
retained in [connected-constructor-rea.json](connected-constructor-rea.json) and
[connected-serializer-rea.json](connected-serializer-rea.json).
Neither a tag nor a presence word may be inferred around the direct child.
The measured legacy selector-2 examples do not establish selector-1 framing.
The codec preserves this complete saved structure. The existing component-surface
caller also accepts the native-bound parting-geometry class.

The containing feature's method and the settings' saved selector are distinct
inputs. The constructor allocates `+0x178` only when the method equals 1. The
loader later invokes that existing child only when the saved selector equals 1
and the pointer is nonnull. A codec must carry the method into the direct
settings reader. It does not create a child solely because the saved selector
equals 1. Fixtures cover all four combinations of method/selector equal
to 1 or a different value. Native STORE has no null guard in its selector-1
branch; successful native writing for an inconsistent pair is unproven.

The independent literal builder reproduces 30 graphs: nine bare graphs and 21
complete resolved-feature streams for seven owned-document contexts. These
include empty and populated direct children, shared/null array entries,
connected-component DWORD arrays, signed presence and nonpositive vector counts,
the minimum modern version, suppressed bodies and a strict serialized kernel
sheet. Existing base, component, dimension, synchronization and kernel fixture
builders are reused. No Bessemer encoder or original producer parser generates
the expected bytes.

Focused validation covers 18 new private codec tests, four public document tests,
37 affected regression tests and two Python fixture checks. The document tests
exercise 14 independent Length/raw-word edits, fresh reversals, revision-bound
history, all carrier bytes, input isolation after failure/source mutation and
eight concurrent workers. Exact stored-archive coverage does not certify the
geometry. Full PR and native-corpus gate outcomes are recorded separately.
The retained native compatibility run accepted all 906 selected stream sets
with an explicit installed kernel schema; its packet, executable, schema and
log hashes are in
[native-resolved-regression-verification.json](native-resolved-regression-verification.json).

Native application acceptance, geometry acceptance, historical contexts and
whole-schema parity remain unproven. The humanizer skill is unavailable in
this environment; maintainer prose review remains outstanding.
