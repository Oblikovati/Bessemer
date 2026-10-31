### 14.384 Results plot Property and three header hierarchy levels

*Status: all 212 retained plot headers complete; graph/dialog and optimization streams remain separate · Evidence: C-764*

Category child handler 18118BE00 constructs plot objects through **1809DEDC0** and calls **1814AABE0** with plot storage and supplied version. The latter opens cwPostDlgParam and cwPostGraphParam through independent imported readers, then several auxiliary streams, and finally opens **Property**. C-764 binds the complete dispatcher function/decompilation hashes and parent call path. The fact that Property is read last does not concatenate preceding streams into its payload.

Plot Property contains **two signed four-byte words**, first read into a temporary and second to plot +28. At supplied version **>830034**, one counted CwPersistStream UTF-16 text targets plot +08. Native instructions encode the equivalent inclusive comparison against **830035 (0xCAA53)**. The first loaded word can subsequently be replaced using result-manager state when it is below one; this consumes no bytes and does not change the preserved wire value. Destination meanings are not assigned solely from those runtime mutations.

`cosworks_results_plot_header.py` freshly hash-checks/replays **all 212 retained plot Property streams / 7112 bytes** to exact EOF and complete reconstruction. Three tests cover the strict gate, signed values, nonempty text, truncation/remainder and explicit version contracts. No writer layout or older plot-dialog conversion behavior is inferred from this header alone.

Across the separate root/category/plot audits, the retained Results Property hierarchy now accounts for:

| Property level | Occurrences | Bytes | Evidence |
|---|---:|---:|---|
| Results root | 53 | 2192 | C-762 |
| Category | 185 | 1652 | C-763 |
| Plot | 212 | 7112 | C-764 |
| **Total** | **450** | **10956** | All three recorded header sets complete |

This sum is a retained inventory statement, not fresh combined replay or a global document denominator. Some plot storages contain other streams without a Property fixture; the counts do not establish a mandatory Property rule.

The dispatcher separately opens fixed/variable optimization and convergence streams and imported dialog/graph readers. Direct IStream reads may ignore short-read status, so nominal native read widths must not be used to fabricate missing sample bytes. Those payloads, historical conversion branches, physical meanings and complete installed-format coverage remain open. No target executes; global semantic coverage credit remains zero.

Evidence: C-764.
