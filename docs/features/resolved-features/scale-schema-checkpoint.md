# Scale and schema-reference reconstruction

The private ordinary model decoder now reads complete `moScale_c`,
`moCoordSysRef_w` and `moSwiftSchemaRef_c` bodies. Scale records reuse the existing
body-feature, vector and direct object-array codecs. The coordinate-system
wrapper reuses the reference-wrapper body before reading its compact transform.
The inherited atom-owner and wrapper roles include their newly supported
concrete classes. Schema-reference names and signed values remain raw; the
decoder does not apply the native helper's runtime value remap.

Five owned fixtures cover empty scale dependencies, a fresh coordinate-system
wrapper shared with the direct array, an independent expected edit, a UTF-16
schema name with the minimum signed value and an empty name with the maximum
signed value. Their independent primitive partitions contain 89, 117, 117,
nine and six observations respectively. Four focused Rust tests passed exact
reconstruction, every truncated prefix, trailing bytes, unsupported versions,
wrong pointer classes, shared identity, input isolation and the independent edit.
Four fast reference Python tests passed; native-sample tests were excluded from
that command.

Exact sample reconstruction increased from 357 to 386 of 906 configurations,
preserving every previously accepted case. Two fresh processes produced
identical outcomes. The independent reader completed all accepted cases against
matching header, model, manager and feature hashes, covering 15,168,199 feature
bytes. The 520 rejected configurations remain explicit, and the broad parity
gate still fails. These checks establish stored framing, not native application
or geometry acceptance.

The source, fixture and primitive hashes are in `scale-schema-reference.json`.
The frontier and accepted-reference reports retain the corpus evidence. Clauses
11.150 and 11.264 are retained with verified raw hashes. No dependency, version,
release, PR or push changed. The humanizer skill is unavailable in this session;
its prose review remains with the maintainer.

Full `cargo test` passed 409 library tests, the integration suites and 247
doctests. Three external gates remained ignored by default. The explicit
906-case gate was executed twice and still failed as reported above.
