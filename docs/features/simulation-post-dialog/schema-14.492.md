### 14.492 Dialog user-defined legend minimum/maximum state and range values

*Status: combination-code expansion and four float32 range roles bound; units, baseline computation and runtime activation remain qualified — Evidence: C-874*

The four-byte **temporary_state_word** in the dialog prefix (immediately after its initial56-byte group) is a stored **user-defined min/max combination code**. **CcwPostDlgParam::ReadFromStream**, DGXPOST:1800953D0, first clears runtime words+520/+524 together, reads that code into a temporary and expands it as follows:

| Stored code | Runtime user-minimum flag+520 | Runtime user-maximum flag+524 |
|---|---:|---:|
|1 |1 |1 |
|2 |1 |0 |
|3 |0 |1 |
|Any other signed32 value |0 |0 |

The flags are derived object state, not two separately stored words in this prefix. Unknown codes remain preserved even though this reader maps them to the same pair as0. Named **GetUserDefinedMinMaxCombination**, **180090640..180090673**, reports0 for both flags zero,1 for both nonzero,2 for only minimum nonzero and3 for only maximum nonzero. Thus expanding and recombining an unknown stored code can lose its original value; the raw code remains authoritative.

Named **GetLegendMinMax**, **18008EC00..18008EC35**, binds four members in the existing eight-word528..544 group:

| Object member | Offset within that32-byte wire group | Role/type |
|---|---:|---|
|530 |8 |Baseline legend minimum, float32 |
|534 |12 |Baseline legend maximum, float32 |
|538 |16 |User-defined minimum, float32 |
|53C |20 |User-defined maximum, float32 |

The accessor copies the baseline values to its two float outputs, substitutes the user minimum only when runtime+520 equals **exactly1**, and substitutes the user maximum only when+524 equals **exactly1**. This differs from the combination reporter's nonzero tests if external code supplies a nonboolean flag. Stored code2, however, expands to minimum flag1 and therefore does select the user minimum. The method performs no range ordering, clamping or unit conversion; the schema does not invent those constraints or identify the baseline computation from these copies alone.

**cosworks_legend_range_fields.py** separates exact stored code, derived flags, four raw-preserving float views and selected-output views. Repeated native replay finds **224 dialog prefixes**, all with stored combination0. Their modeled range selection uses baseline values; the populated override branches are static/synthetic evidence rather than observed native execution. The resulting1120 typed code/float annotations cover4480 already parsed bytes and add no global byte credit. The PostStyle-only catalog's unresolved-label count is unaffected by these outer-dialog fields.

C-874 checks both named accessors, complete leaf return paths and the full reader probe against the installed provider. Three new tests cover all code branches, unknown-code retention, nonzero versus exact-one behavior, negative-zero/NaN raw-bit preservation and input bounds; four prior style/color tests repeat successfully. Prefix-only native scope, retained candidate versions and prior legacy failures remain explicit. No target code executes.

Evidence: C-874.
