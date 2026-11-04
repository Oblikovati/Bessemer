### 14.479 PostStyle machine-readable field-role catalog and remaining gaps

*Status: existing semantic evidence reconciled into an explicit catalog; numerous field purposes remain unresolved — Evidence: C-859*

**cosworks-post-style-field-catalog.json** catalogs the PostStyle span labels emitted by the bounded parser. Each entry records its wire type, any established consumer role, semantic evidence identifiers, occurrence count in accepted native interpretations and presence at the eighteen probed version-boundary values. **cosworks_post_style_field_catalog.py** supplies the role classification. Representative version probes use synthetic zero payloads and do not constitute additional native observations or replace the versioned parser's conditions.

The catalog contains **118 distinct labels** across those probes: **66 with a bound consumer role**, **one with a framing role**, and **51 whose purpose remains unresolved**, including the later A8 renderer,21C tessellation-request and1B8 offset-decision bindings in14.490–14.494/C-872–C-876. These are labels, not independently complete semantic fields: each color component has a label, and the pre700006 raw-class region has one. A role such as an integer mapping control also does not establish every enum value, valid range or complete runtime rendering behavior.

In particular, a known red/green/blue/alpha float32 component does not by itself establish the enclosing color's purpose. The catalog records component identity while leaving purpose null for unresolved colors. For three style colors only the specifically bound alpha component receives a consumer role; the sibling RGB purposes remain unknown. Counted text retains its known code-unit framing and assignment evidence while its UI purpose remains unresolved.

The remaining raw-word purposes are explicitly enumerated:

| Owner | Unresolved object-member word labels |
|---|---|
|Legend |None in this raw-word list after14.490; unresolved color purposes remain |
|PostStyle |1F0,204,208,230,234,238,23C,240,28C,290,294,298 |

The read-time migration evidence for1F0 does not identify its complete visual meaning, so it remains on this list. Other unresolved entries include color purposes, counted text purpose and the old raw-class interpretation. No blanket signed-integer or boolean type is assigned to the unknown raw words.

Native reconciliation uses the first successful retained candidate per stream from the existing replay, with52 complete legacy style blocks and224 modern dialog prefixes. After the C-876 update, **65720 bytes** have the catalog's bound-consumer classification, **1104 bytes** have only the counted-length framing role and **40672 bytes** belong to purpose-unresolved spans. These are local accounting categories over already parsed bytes, not a semantic-completeness percentage or global coverage denominator. The zero-byte text spans are counted as occurrences separately from byte totals. PostInfo and other dialog-prefix spans are excluded.

C-859 records hashes of the contributing semantic evidence packets and checks that every native PostStyle label is present in the catalog. Two new tests prevent component typing from being promoted into an unproven color purpose, check the fixed palette-slot boundary, and keep unknown word/text purposes explicit. Four prior style/color tests repeat successfully. The eight legacy failures, modern prefix-only boundaries and candidate-version qualifications remain unchanged. No new global coverage credit is assigned and no target code executes.

Evidence: C-859.
