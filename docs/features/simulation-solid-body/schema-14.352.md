### 14.352 Body-property serializer field order and version gates

*Status: static reader/writer evidence; storage-class binding and native Property replay remain open · Evidence: C-728*

The retained `cosworks.dll` serializers at `180A32600` and `1813ACC90` open the UTF-16 literal `Property` (`181F53138`), independently of sibling `Material`. C-728 records the installed module hash, function-byte and decompilation hashes, and raw IEEE-754 bytes for every referenced version constant. These are function-relative layouts: their SolidManager/ShellManager ancestry has not yet been established by a constructor/vtable chain. Do not assign corpus streams to either layout solely because they have a Material sibling.

At `180A32600`, the read order is the delegated `CSWPersistID::Serialization` on member `+20`, two counted texts (`+38`, `+40`), three four-byte words (`+48`, `+4C`, `+50`), then a fourth word (`+54`) only at version **830004 or later**. The writer emits all four words. Member offsets identify destinations, not file offsets or established field meanings. The persist-ID delegate remains a separate versioned dependency; this clause does not replace it with a guessed fixed-length prefix. Material is optional on the write path when member `+30` is null; its read path allocates a material object only when that stream opens.

At `1813ACC90`, read one persist-ID delegate (`+38`), texts `+58/+60`, four words `+70/+74/+78/+7C`, then text `+68`. The following table gives **wire order**, with inclusive minimum versions. `I` is a four-byte word, `D` an eight-byte floating-point bit pattern, and `T` the existing counted UTF-16 text grammar. No physical meanings are assigned from destination offsets.

| Minimum version | Payload at this position |
|---|---|
| 800066 | I at +80 |
| 800068 | I at +84 |
| 810025 | I at +8C |
| 830007 | 2I at +90/+94 |
| 830011 | 3I at +A8/+AC/+B0 |
| 830013 | I at +B4 |
| 830025 | I at +B8 |
| 830029 | 4I at +BC/+C0/+C4/+C8, then 5D at +D0 through +F0 |
| 810029 | delegate `181004B00` on +20 (the two-word helper retained with the beam evidence) |
| 840006 | 3I at +160/+164/+168 |
| 840013 | I at +1C8 |
| 840025 | I at +10 |
| 850002 | I at +1E4 |
| 850013 | I at +1CC |
| 850026 | I at +1D0 |
| 870005 | I at +1D4 |
| 870008 | signed count, then that many T values added to the +128 list |
| 870019 | I at +1E0 |
| 880004 | 2I at +1F0/+1F4, then 3T at +1F8/+200/+208 |

The delegate and +160/+1C8/+10/+1E4 fields are inside a shared branch entered when `v >= 830011` **or** `v >= 810029`; each nested field still has its own gate. Below 830011 the assignment from +74 to +B0 consumes no bytes. The later change of an in-memory +1F4 value from 1 to 2 also consumes no bytes and must not rewrite preserved wire data. Preserve raw words, floating-point bits, and strings before any native normalization.

The same function separately opens `UserShells` (`18205A690`): signed count followed by that many four-byte entries, with no version gate around the stream-open/read loop. The count is distinct from the counted text-list count within Property. Runtime linkage, index meanings, loaded-list reconciliation, historical writer layouts, and malformed-count behavior are not established here. A safe independent reader must bound both counts and retain failures rather than infer missing fields.

C-728 is static evidence only: no target execution, no native Property corpus replay, and **zero global coverage credit**. The full installed-format schema remains incomplete.

Evidence: C-728.
