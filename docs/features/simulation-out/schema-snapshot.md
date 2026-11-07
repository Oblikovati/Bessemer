### 14.106 Simulation OUT report syntax, ordered controls and paired mesh totals

*Status: Complete byte-preserving syntax projection of the installed OUT report; report semantics and other encodings/versions remain partial · Evidence: C-482*

The benchmark CWR's **OUT stream is 4,970 bytes**: the four-byte transfer-count word from 14.101 followed by a **4,966-byte solver report**. Its transfer producer is already bound to `.OUT` by the native solver copier; this clause interprets the copied body from the installed source itself. The body has **91 CRLF-terminated lines**, including empty lines, and consists of seven-bit bytes with **five embedded NUL bytes** in one banner line. Its NUL positions in the full stream are **763, 765, 767, 769 and 771**. The reader preserves those bytes instead of repairing the banner, inferring a product version or treating the whole report as UTF-16.

`solidworks/simulationout.py` first validates the native-derived OUT transfer framing, then records each line's original content, CRLF bytes, full-stream offset and any embedded-NUL positions. Concatenating line contents/endings reproduces every report byte through exact EOF. The supported profile preserves seven-bit bytes, with C0 controls restricted to NUL, tab and CRLF; other encodings, other C0 controls and bare-LF/CR profiles stop. An unterminated final line is preserved with an empty ending. Unrecognized headings, paths, date/time text and malformed field-like lines remain byte-preserved rather than receiving invented meanings.

A numeric control line has a label, a parenthesized uppercase symbol, `=`, and a signed integer/decimal token with optional `E` or `D` exponent. The model retains the exact numeric lexeme and an exact decimal value. A normalized display label removes dot leaders and combines whitespace while the original line stays available. Following `EQ. integer, description` lines are associated with the preceding field occurrence in this observed profile. These are statements printed by the stored report, not a universal native enum registry.

The report contains **18 ordered numeric occurrences** and **31 legend entries**:

| Printed label | Symbol | Stored value |
|---|---|---:|
| Number of load cases | `NLCASE` | 1 |
| Solution mode | `MODEX` | 0 |
| Solver type | `ISOL` | 2 |
| Iterative solution mode | `IMODE` | 2 |
| Solver convergence tolerance | `CONVR` | `0.100E-03` |
| Thermal loading flag | `ITHERM` | 0 |
| Gravity loading flag | `IGRAV` | 0 |
| Centrifugal loading flag | `ICNTRF` | 0 |
| In-plane stiffening flag | `INPLN` | 0 |
| Soft spring addition flag | `ISOFT` | 0 |
| Save decomposed stiffness matrix flag | `ISAVK` | 0 |
| Form stiffness matrix flag | `IFORMK` | 0 |
| Spin softening flag | `ISPIN` | 0 |
| Inertia relief flag | **`IFORMK`** | 0 |
| Rigid connections flag | `IRIGID` | 0 |
| Number of equations | `NEQ` | 72,021 |
| Number of elements | `NUME` | 14,642 |
| Number of nodal points | `NUMNP` | 24,140 |

**Symbols are not unique keys.** This source prints `IFORMK` for both the form-stiffness and inertia-relief controls. The model keeps them as distinct ordered occurrences with their own labels, offsets and legends; it neither overwrites one nor silently renames a symbol. For this report, the printed legends describe solution mode 0 as static analysis, solver type 2 as iterative, and iterative mode 2 as memory saver for TETRA10. Other codes/versions are not inferred beyond the retained printed legends. `CONVR` is retained as `0.100E-03` and represented exactly as decimal `0.000100`; no binary64 rounding is required for parsing the report value.

The report explicitly states that SI is used for all input/output data and lists meter, kilogram, second, kelvin, newton, pascal, joule and watt for its common quantities. This is a **declaration by this stored report**. It does not establish the unit provenance of every historical GEN image, all runtime plot buffers or the two composite fixtures that have no OUT stream. The coordinate-unit/frame slots in 14.105 remain unassigned. The report's **NUME 14,642** and **NUMNP 24,140** independently match the paired benchmark GEN counts from MAS-selected binary decoding. The equation count is recorded without assigning individual activity-bit meanings or deriving a universal equation-elimination rule.

`simulation_out_evidence.py` retains source/OUT/body/paired-GEN hashes, all field occurrences and legends, byte offsets, NUL locations, line counts and exact reconstruction in `simulation-out-corpus-evidence.json`, linked to the preceding native transfer evidence. Five focused tests cover duplicate symbols with independent legends, exact decimal/Fortran exponent handling, embedded NULs and source reconstruction, explicit unit declarations, unknown lines and profile guards. Syntax reconstruction does not imply complete semantic coverage of banner/path/date provenance, every line or every installed report format. No installed native target was executed and no global/all-project coverage percentage is regraded.

All **1,562 regression tests pass** (271.673 seconds). The corpus verifier passes; specification lint reports zero findings (668 headings, 96 anchors).

Evidence: C-482.
