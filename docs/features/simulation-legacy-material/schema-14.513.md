### 14.513 Installed literal corroboration for legacy material candidates

*Status: all196 candidate identifier prefixes match installed literals; historical serialization and physical-property meanings remain unbound — Evidence: C-895*

The installed `cosworks.dll` contains narrow zero-terminated spellings matching all nine identifier prefixes from14.512. Their pointers occur at the following positions relative to **18254F1B8**:

| Spelling | Pointer-table position | Native candidate occurrences |
|---|---|---|
|EX |1 |70 |
|NUXY |4 |66 |
|GXY |7 |3 |
|DENS |10 |45 |
|SIGXT |11 |2 |
|SIGYLD |13 |2 |
|ALPX |14 |4 |
|KX |17 |2 |
|C |20 |2 |

These positions describe **installed pointer locations only**. They are not stored enum codes, verified property numbers, physical units or historical field-order requirements. For example, the EX pointer at18254F1C0 resolves to exact narrow bytes `45 58 00` at181F48AE4. C-895 records and verifies the pointer bytes, target addresses and exact literal bytes for all nine spellings. No claim is made that the current shared material serializer reads this pointer pool.

Every one of the196 fixed-record candidates has one of those matching ASCII prefixes before its first zero byte. Importantly, **182 of196** sixteen-byte identifier fields contain nonzero bytes after that first zero. A prefix view therefore cannot replace the complete field for lossless reconstruction. Those suffix bytes remain uninterpreted; they are not classified as padding, inferred memory contents or additional named fields.

**cosworks_legacy_material_identifier_audit.py** verifies the installed module hash and all nine pointer/literal pairs, repeats the fresh70-stream candidate audit and checks all196 raw identifier fields. The five existing header/material tests and three candidate tests pass again. The candidate partition still reaches exact EOF for20,426 bytes, but its current-reader failures, original versions, **complete=false**, zero newly completed schema records and zero global coverage credit remain unchanged. Installed spelling corroboration strengthens the candidate lead without supplying a historical reader/writer binding. No target code executes.

Evidence: C-895.
