### 14.458 Legacy MaterialList header and retained body incompatibility

*Status: legacy carrier dispatch and all70 header prefixes verified; complete bodies fail retained-context replay — Evidence: C-838*

The exact child storage **MaterialList** at literal **182064060** is distinct from the study-manager Material ancestry in14.348 and AWDocument Material in14.452. Study helper **181429460**, on its nonexcluded study/object route and the non-shell branch, opens MaterialList, enumerates stream entries, opens each actual entry name and constructs a material through **1816D5780**. It passes the stream wrapper and its supplied integer saved version converted to double to shared material serializer **1816E15A0**. Other recovered helpers **181425430/1814299B0** likewise enumerate/open those material streams and invoke the same serializer using root-context version state. Allocation sizes and runtime lookup/copy operations are not wire fields. Actual shell/model/study pointer conditions remain relevant to runtime invocation.

This binds a current reader route, but does not make every historical named-study record compatible with that current serializer. Fresh C-838 inventory finds **70** direct `Cosworks/<named-study>/MaterialList/<actual-stream-name>` carriers, all currently named **Material 1**, totaling **20,426 bytes**. The stream names come from enumeration; no mandatory numeric naming rule is inferred. Their retained version candidates are **700339,800078,810027,830036,830037,830038,840017 or840030**, according to their source documents. No alternate version is invented to obtain EOF.

Every record admits the following prefix of the bound shared serializer, with raw counted-text preservation:

| Order | Reads at current material-object destinations |
|---|---|
| 1 | Text20, text28 |
| 2 | Four-byte word30, then three words34/38/3C |
| 3 | Text48, text50, text58 |
| 4 | Four-byte word68, then text60 |

`cosworks_legacy_material_header.py` stops immediately after text60, before the current serializer's next **text08** read. It always reports **complete=False**, preserves contiguous raw spans and labels the next dependency explicitly. All **70 header prefixes** succeed, totaling **6,902 bytes**, leaving **13,524 bytes** outside this prefix claim.

The full material parser fails on **all70 retained candidates** at that next current-text boundary, at per-record prefix offsets **82–132**. The failures are **text length exceeds limit** or **truncated text payload**, not accepted EOF or skipped opaque tails. For example, the262-byte `Cosworks/b7/MaterialList/Material 1` record has two initial `Shell-1` strings and reaches offset112 before the next field cannot be read as the current text08 grammar. This does not establish that historical producers emitted a text08 field, that the remaining bytes are corrupt, or that guessing a lower version would resolve the layout. C-838 retains complete failed-attempt raw bytes, original versions, exact prefix hashes and error endpoints for continued historical/context investigation.

Five tests pass in the audit: the four established material grammar tests plus a focused header-boundary test with every generated prefix truncation and unread-suffix preservation. Installed module/literal/current-reader/shared-serializer byte and decompilation hashes are checked. Previous1226 study-manager material successes are not replaced or broadened by this different carrier, and no legacy full-stream success is claimed. Historical remaining fields, actual runtime handler/state, physical meanings and the full installed-format objective remain open. No target code executes and global semantic coverage credit remains zero.

Evidence: C-838.
