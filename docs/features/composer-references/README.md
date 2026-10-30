# Composer reference text and supplied maps

Feature [#105](https://github.com/Oblikovati/Bessemer/issues/105) implements
clauses 14.92–14.93 / C-468–C-469. These operations interpret stored reference
text and explicitly supplied runtime maps. They add no binary fields and do
not establish which files the native application loaded or registered.

`ComposerReferenceText::from_units` retains NUL-free UTF-16, including unpaired
surrogates and newlines. Positions use code units. `split()` searches for the
first case-sensitive `Ref:Ext:` anywhere in the original text, then for the
first global `#`. Its file slice starts at absolute position eight, even when
the marker begins later. Without a marker, file output remains unassigned.
External file text is an unresolved lexical value, not a resolved resource key.

The four lexical filename helpers preserve the native mixed-separator and
directory-dot rules. They do not call filesystem path APIs. `initial_file_key`
requires the documented ASCII C-locale context, including the literal doubled
backslash produced by bare `scenario.xml`. `fallback_file_keys` retains all
four candidates in native order, including duplicates.

`ComposerGeodeServer::from_files(&files, limits)` validates explicit primary
and override entries. File keys use ASCII lowercasing; GUID keys remain exact.
Non-ASCII server keys and embedded NUL fail. Duplicate normalized file keys and
duplicate GUIDs within one tier fail. File, entry and cumulative key-byte
budgets are explicit. Input-size checks precede map allocation.

`get(&file, &guid, initial_pointer)` searches override before primary. A found
null token shadows primary and remains distinct from a miss. A miss retains
the supplied pointer. Tokens are opaque `u64` identities; zero means null.

`register(&file, &guid, incoming)` returns an isolated immutable snapshot. An
existing non-null lookup retains both maps and reports the native request to
detach the incoming geode's first parent. Otherwise it applies the exact
documented insertion branches, including null overrides and unchanged inner
maps. It never executes graph detachment. The result includes completion,
exact supplied-state fidelity, parent/result identities, coverage and required
unchecked responsibilities. No-op branches retain the snapshot identity.

`lookup_nonempty(&file, &project, &guid)` requires an explicitly supplied
resulting file key. It performs the final four-attempt block and stops when
the pointer becomes non-null. Successful null lookups continue. A completed
miss returns zero with every attempt retained. Empty GUIDs fail because they
require the unresolved merge pipeline. The result reports completed local
coverage and the exact snapshot queried.

Published maps own their keys and tokens, are immutable, and need no read
locks. Registration mutates an operation-local copy before publication. Tests
cover source mutation and drop, failed-operation isolation, all allocation
budgets, full-width tokens, empty keys, and six concurrent isolated edits.
Every public type has a compile-time `Send`/`Sync` assertion.

Owned oracles run actual readers from an immutable copied source tree. They
cover UTF-16 splits, lexical paths, all supplied-map registration branches,
every fallback stop position, and duplicate candidates. Installed replay
decodes complete geometry and XML, explicitly supplies member-name map
registration, and compares every attempt for all 4,445 references across 38
projects: 4,416 non-null results, 29 misses and 4,532 attempts. This context is
supplied for testing; native registration order and file keys were not measured.

Run `python docs/features/composer-references/freeze_references.py` to replay
the retained readers and schema clauses. `--export` writes the local native
probe; `--write` updates reviewed baselines. Export the complete link probe
with `docs/features/composer-links/freeze_links.py --export`, then run
`cargo test --test composer_references -- --include-ignored`.
Clause snapshots normalize line endings and the final newline; their text is
otherwise retained exactly.

External path resolution, resource discovery/loading, callbacks, empty-GUID
merging, graph ownership/cloning, the running CRT locale, and native application
acceptance remain unchecked. This feature does not complete the Composer
project schema or the overall Corliss parity goal.

The maintainer-side humanizer skill was unavailable. A maintainer still needs
to review this prose.
