# Simulation OUT reports

Tracking: [#114](https://github.com/Oblikovati/Bessemer/issues/114).
Behavior profile: `simulation-out-syntax-v1`.

The implementation follows clause 14.106 in the approved
`../inventor-interface` specification. The clause is retained in
`schema-snapshot.md`. Reference scripts copy the actual Python readers before
importing them and record hashes and import origins. They do not replace reader
functions or supply fabricated MAS/GEN context.

## Stored syntax

`SimulationOutArchive` validates the complete successful-copy transfer prefix,
seven-bit content and CRLF boundaries. It retains every line, ending, offset
and embedded NUL. Empty lines and an unterminated final line are preserved.
Unknown lines and malformed field-like text remain source bytes.

Numeric controls remain distinct ordered occurrences. Symbols are not unique
keys: the installed report prints `IFORMK` twice. Each occurrence retains its
normalized display label, original numeric lexeme, exact decimal value and
following printed legends. Dot leaders and whitespace affect only the display
label. Printed legend integers are retained without a machine-integer bound.

Decimal values use a sign, decimal coefficient digits and a base-ten exponent.
Trailing zeros and negative zero survive; parsing and count comparison use no
binary floating conversion. Adjusted exponents must fit a signed 64-bit value.
Explicit limits bound complete bytes, lines and numeric occurrences. A failed
limit or malformed wrapped image returns a typed error before publication.

Construction takes complete line contents and endings. Numeric edits replace
one revision-bound token, derive transfer framing and reopen the complete
report. Every other body byte remains exact. Results include completion, byte
fidelity, all field derivations, exact syntax coverage, zero requested-token
byte error and semantic exclusions. No-edit saves preserve the original image.

## Complete CWR operations

`SimulationOutStudy` uses the complete compound carrier and actual paired
MAS/GEN images. OUT replacement reopens the complete report and preserves
unrelated logical streams and directory metadata. Unchanged binary mesh
identities are reused only after the replacement carrier's MAS and GEN bytes
have been checked against the validated snapshots.

Printed `NUMNP` and `NUME` are compared against actual GEN counts. Each comparison
retains every candidate occurrence. One exact nonnegative integer can match or
mismatch; a missing symbol is missing, and duplicate or non-count values are
inconclusive. Missing and inconclusive comparisons never become matches.
These checks establish report/binary consistency, not mesh validity.

The exact stored SI declaration is local to this report. It does not assign
units or a coordinate frame to GEN images or other historical studies.

## Verification

Ten independently built reports cover duplicate symbols and legends, exact
decimal spellings, signed zero, embedded NULs, empty lines, unknown text,
unterminated lines, the transfer boundary above 10 MiB and an edit that crosses
that boundary and updates the transfer count. Actual copied
readers supply every expected line, field, decimal tuple and legend. Independent
literal patches supply edit goldens for every numeric occurrence.

The installed benchmark report has 4,970 wrapped bytes, 91 lines, 18 numeric
occurrences, 31 printed legends and five embedded NULs. Its printed 24,140 nodes
and 14,642 elements match the actual MAS-selected GEN. All 18 occurrence edits
have independent byte goldens. The actual CFB, OUT, transfer, MAS and GEN readers
reopen the complete edited CWR and check all six logical streams and metadata.
The other two installed CWR fixtures have no OUT stream and are not report
coverage claims.

```powershell
cargo test --test simulation_out
python -B docs/features/simulation-out/freeze_reports.py --export
cargo test --test simulation_out -- --include-ignored
python -B docs/features/simulation-out/verify_out_edits.py
cargo xtask pr
```

Reference baseline changes require explicit `--write` and review. Run scripts
in this feature directory sequentially because the source snapshot recorder
uses one temporary report name. Tests also cover independent decimal
expectations, invalid controls and endings, resource limits, malformed edits,
foreign IDs, source-buffer mutation, compile-time Send/Sync assertions and
concurrent edit/revert operations.

The Windows PR gate passed 2,928 tests with 13 corpus-dependent tests ignored.
All ten focused tests passed with the installed report enabled. Two fresh
processes produced identical 3,366,912-byte edited CWR files, compared byte for
byte. `complete-verification.json` records code and source hashes, limits,
observed time and memory, and acceptance exclusions. The memory observations
are sampled lower bounds, not guaranteed process peaks. There is no prior
published implementation for a release comparison.

## Acceptance limits

This profile establishes complete stored syntax. Solver meanings, banner/path
and date provenance, other encodings, other line-ending profiles, unknown line
roles and native-application acceptance remain separate work. No geometry,
topology, self-intersection or tolerance checks are claimed. A stored control
edit does not establish that the solver would accept its value.

No dependencies, package versions or release tags changed. The maintainer
humanizer skill was unavailable; the prose pass remains for review.
