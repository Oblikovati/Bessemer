# Mid-surfaces and reference-surface records

Bessemer retains complete ordinary mid-surface features, mid-reference surfaces,
base reference surfaces and their documented helpers. The mid-reference keeper
is serialized directly, including its inherited surface IDs and both additional
object arrays. It has no outer pointer tag. Face-array counts retain signed
stored values; nonpositive counts select empty arrays.

Base reference surfaces use the archive backend marker to select external sheet
framing. External counts are nonnegative metadata and consume no inline sheet
body or attribute-list bytes. Large external counts allocate no placeholder
sheets. Extra-body and word-vector suffixes still follow normally. Mid-reference
surfaces retain inline sheet bodies even when their backend marker is nonzero.
Their separate extra kernel body is stored only when its signed presence equals
one. Other presence values remain unchanged and add no body.

Face pairs retain typed single-face references and distance dimensions. Face
and heuristic-edge records retain a direct object array before the component
face pointer. Edge pairs retain complete component edges, including reviewed
silhouette-edge subclasses and cached identities. Modified-surface IDs reuse
the established mirror/source entity prefix and retain the final signed word.
These layouts follow C-379 and specification section 11.337.

Independent fixtures cover 432 complete graphs across eight tagged classes,
nine file versions and six styles, with 42,730 primitive observations, 51 owned
documents and three independent signed-word edit goldens. They cover full
inherited geometry, direct keepers,
real shared targets, cyclic IDs, signed counts, external counts and complete
neutral kernel frames. The preceding surface-reader extraction passed all seven
existing fixture tests unchanged and was committed separately.

Immutable fixture regeneration, nine focused tests, three public document tests
and the actual four-source native regression passed. The full local gate passed
2,564 tests across 186 test executables, including 1,072 library tests and 277
doctests; three opt-in corpus tests remained ignored.

Two fresh native processes matched all 906 statuses and causes. Complete typed
reconstruction increased from 878 to 883 configurations, adding 114, 191, 227,
294 and 300 without regressions. The aggregate corpus still fails because 23
inputs remain incomplete. The independent reader consumed all 77,262,506 resolved
bytes of the 883 accepted inputs through EOF, with all four input streams checked
against the pinned manifest.
External sheet contents and backend resolution receive no geometry acceptance
claim. Raw engineering meanings, remaining schemas and native application
acceptance remain unfinished.

The humanizer skill was unavailable; its prose review is left to the maintainer.
