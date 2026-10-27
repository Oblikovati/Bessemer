### 11.232 Copy, trim/extend and reference edge identifiers

*Status: B / ? — three native serializer selections and 75 populated representations verified; scalar meanings, historical/context variants and whole-document semantics remain partial.*

This clause extends the scoped ordinary external-XML model archive reader for file versions greater than `0x27CB`. Primary RTTI/vtable bindings in `sldmodu.dll` select the following serializers; all three classes have `moEntityIdRep_c` ancestry.

| Tagged class | Primary vtable | Selected serializer | Modern payload after common entity-representation base |
|---|---|---|---|
| `moCopyEdgeIdRep_c` | `0x4CF065B0` | `0x4BBBBBD0` | direct source-entity ID, then i32 at runtime `+0x48` |
| `moTrimExtEdgeIdRep_c` | `0x4CF07570` | `0x4BBBBBD0` | direct source-entity ID, then i32 at runtime `+0x48` |
| `moEdgeRefIdRep_c` | `0x4CF03120` | `0x4BBBDE70` | i32 at runtime `+0x38` |

The common entity-representation serializer at `0x4BBBBF50` is already described in 11.18 and the preceding identifier clauses. In the supported ordinary context its wire fields are the tagged feature pointer and tagged external feature-reference pointer. Other helper conditions, historical migrations and database modes are not inferred from this context.

`0x4BBBBBD0` explicitly calls the common base, invokes the direct entity-ID serializer at embedded runtime member `+0x38`, then reads or writes the four-byte word at `+0x48`. The embedded ID contributes its tagged representation pointer without an additional tagged `moEntityId_c` wrapper. These classes select the same concrete serializer as previously decoded derived-edge and trim-edge representations; this is verified dispatch reuse, not a name-based schema guess.

`0x4BBBDE70` explicitly calls the same common base, then reads or writes the four-byte word at `+0x38`. It does not insert a source-entity ID. All three words retain their original signed values and bit patterns without assigning an unsupported option or index meaning. Null pointers and references to objects already present in the inherited archive map use the ordinary tagged-object rules.

The six affected native configurations contain **75 newly supported representations across twelve complete containing feature roots**. These include trimmed, planar and sew surfaces, profile and plane features, a fillet, combined-body and deleted-face features, and two stock features with payloads over one megabyte. The retained root byte spans and hashes describe complete containing features, not the size of just their nested edge-ID records.

The Main Housing with Overmold, tutorial mousebase and tutorial mousesplit feature lists complete. Main Housing Overmold continues to `moSweepRefSurface_c`; Main Housing and tutorial mousemultibody continue to `moSplit_c`. Native class order, raw words and source/root hashes are retained for every newly supported representation in these completed roots. The full-document and all-installed-project-format goals remain open.

The 906-context audit now contains **24,834 complete roots**, **791 complete lists** and **115 explicit stops**. Every preceding complete root record is unchanged. This adds 69 roots and three exact-EOF lists totaling **2,215,778 decoded bytes**: Main Housing with Overmold (1,866,314), mousebase (174,732) and mousesplit (174,732). Totals are **718 exact-EOF lists / 51,776,533 decoded bytes**; 73 complete lists retain four-byte unknown tails. Checkpoints `resolved-feature-list-frontier-0.251.json` and `resolved-feature-list-frontier-0.252.json` retain the comparison. These are framing gains, without additional whole-document semantic coverage credit.

Validation: all 849 regression tests pass (96.824 seconds); specification lint reports zero findings.

Evidence: C-274.

