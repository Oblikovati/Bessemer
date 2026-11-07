# Extended reference surfaces

Bessemer composes the complete reference-geometry and inline surface bodies
before the tagged extension-data pointer. The native surface selector keeps
kernel sheets inline even when the stored backend marker is nonzero. Empty
native keepers retain their tag and shared-map identity without a fabricated body.

The data object stores three direct lists in native order, a raw type, an end
condition and a final raw method word. Mode 0 selects a distance-dimension
pointer, mode 1 selects the `+20` wrapper and mode 2 selects the `+18` wrapper.
Every other signed selector consumes no pointer. Unknown enum values remain
stored values; the reader does not infer engineering meanings or new payloads.

Independent fixtures cover 126 complete graphs across three classes, seven
versions and six styles, with 12,751 primitive observations and 20 owned
documents. They cover populated and empty lists, shared references, nullable
data, complete distance dimensions, inline kernel bodies and two independent
method-word edit goldens. Five focused Rust tests, three public save/reopen and
isolation tests, and immutable fixture regeneration passed. The actual native
source regression verified a complete containing-root hash and boundary, its
stored selector and the recorded later dependency.

Two fresh native processes matched all 906 statuses and causes. Complete typed
reconstruction increased from 866 to 869 configurations, adding 301, 342 and 346
without regressions. The aggregate corpus still fails because 37 configurations
remain incomplete. The independent reader reached exact EOF for all 869 accepted
inputs, covering 72,616,720 resolved bytes and all four stream hashes. The full
`cargo xtask pr` gate passed formatting, Clippy, build, 1,045 library tests,
all integration tests and 277 doctests. Three external-fixture tests remain
ignored by the default gate. Engineering enum domains, whole-schema parity and native application
acceptance remain unfinished.

The humanizer skill was unavailable; its prose review is left to the maintainer.
