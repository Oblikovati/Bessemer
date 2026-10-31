### 14.382 Results root Property storage and versioned header

*Status: all retained direct Results headers complete; nested categories/plots remain separate · Evidence: C-762*

The study reader opens Results (`181F83F80`), constructs its object with **18118AAA0**, and invokes primary virtual +48. The constructor installs table **182041C28**; slot **182041C70** points to **18118C1F0**. C-762 verifies installed function-byte/decompilation hashes, slot pointer, stream literals and all three version constants. The root storage method opens direct Property, then enumerates child storages through a separate handler **18118BE00**. Nested Property names must not be assigned this root grammar merely because their basenames match.

Direct Results/Property contains, in order:

| Version condition | Wire payload / destination |
|---|---|
| Always | signed four-byte Windows-long +58 |
| Always | counted UTF-16 text +60 |
| Always | unsigned four-byte word +70 |
| >=840013 | signed four-byte word +74 |
| >=890402 | signed four-byte word +08 |
| >=890605 | counted UTF-16 text +68 |

The current writer emits all six fields unconditionally, after obtaining runtime strings from result/study objects. Those string lookup operations are not extra wire fields. The independent decoder preserves exact text bytes and the unsigned +70 domain rather than reinterpreting every word as signed. Native member offsets identify destinations only; complete business meanings remain unassigned.

`cosworks_results_header.py` and its audit freshly verify source envelope/nested payload identities and decode **all 53 direct headers / 2192 bytes** to exact EOF with full reconstruction. Three tests cover each threshold boundary, nonempty text, unsigned values, truncation/remainder and explicit nonnegative integer versions. This does not establish historical writer symmetry outside the recovered reader gates or universal study-array index selection.

The retained Results family also contains category/plot Property, cwPostDlgParam, cwPostGraphParam, convergence-graph and optimization/history/table streams. Their serializers are separate work; no graph/dialog payload bytes receive completion from this root header replay. The constructor-bound root is an entry point for that subsequent hierarchy analysis, not a substitute for it.

No target executes. Child hierarchy dispatch, category/plot layouts, graph/dialog records, field meanings and complete installed-format coverage remain open. Global semantic coverage credit remains zero.

Evidence: C-762.
