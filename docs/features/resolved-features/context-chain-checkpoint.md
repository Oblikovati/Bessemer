# Context-chain readers

The private resolved-feature decoder now dispatches complete bodies for
`moChainComponent_c`, `moContextChainComponent_c`, `moContextUnitComponent_c`,
`moReferenceChain_c` and `moExtFeatureList_c`. The layouts follow
`solidworks/contextchains.py` and `solidworks/assemblyreferences.py` in the
adjacent inventor-interface checkout. The feature list count is a nonnegative
signed 32-bit integer, not an MFC packed count.

`context-chain-reference.json` records five closed-form bodies, their hashes,
the independent reference results through EOF, and the hashes of the two layout
sources. Rust tests reconstruct these bodies exactly and reject every truncated
prefix, trailing bytes, an unsupported version, an incompatible owner, negative
and impossible list counts, short list members and an incompatible list member.
These checks establish stored framing; they do not establish native application
acceptance or geometric validity.

The full reference body remains unsupported when encountered as a fresh object.
Detail-folder support remains pending. This work does not
establish complete schema parity or a public resolved-feature editing API.
No dependency, package version or release changed. The humanizer skill is not
available in this session; its prose review remains with the maintainer.

The corrected focused suite passed three tests. The explicit sample gate still
reconstructed 294 of 906 configurations; its 612 failures remain visible in
`target/context-chain-frontier.log`. These readers did not increase the accepted
sample count because further referenced bodies remain unsupported. The broader
`cargo test` run passed on the initial implementation (398 library tests and
247 doctests, plus integration suites); the signed-count correction and added
regression were verified afterward by the focused suite. No PR or push was made.

## External-feature dependency

The decoder now also reads `moExtFeature_c`, reusing the existing header field
rules for its shared-file prefix. The body preserves the referenced
configuration, raw scalar fields, required nonnull string handle and nullable
reference pointer. Six owned fixtures cross the PDM and additional-byte version
boundaries. The independent reference parsed every fixture through EOF;
`external-feature-reference.json` records the source, packet and case hashes.

Exact sample reconstruction increased to 323 of 906 configurations. The
independent reader completed all 323 accepted cases against matching hashes for
the selected header, model, manager and feature streams. They cover 12,019,332
resolved-feature bytes. These are stored-grammar checks, not per-field semantics
for the sample corpus or native application acceptance. The 583 remaining
failures remain explicit; the broad parity gate still fails.

Two fresh processes produced identical outcomes for all 906 samples. Full
`cargo test` passed 400 library tests, the integration suites and 247 doctests,
with three external gates ignored by default. Additional scalar assertions
passed in the final four-test focused run. No PR or push was made.
