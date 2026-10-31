### 14.383 Results category Property and replaced historical text lists

*Status: all 185 retained category Property streams complete; plot records remain separate · Evidence: C-763*

Results child handler **18118BE00** opens the enumerated category storage, constructs the category object with **1809DEA60**, and, for supplied version **>800037**, opens its direct Property and calls **1809E2B30**. It then enumerates nested plot storages, constructs separate objects with **1809DEDC0**, and dispatches further reads through **1814AABE0**. These are distinct hierarchy levels. C-763 retains the caller/record native bytes and decompilation hashes and verifies both category format thresholds.

The category reader always consumes one signed four-byte word (destination +10). Its continuation is versioned:

| Version interval | Category continuation |
|---|---|
| Below 800044 | None in this function |
| **800044 <= v < 810010** | Signed text count, then that many counted CwPersistStream UTF-16 texts |
| **v >=810010** | One unsigned four-byte word, destination +A8 |

The parent call gate still applies: standalone function layout below 800044 does not prove the parent opens a Property stream at all earlier versions. Current writer output is the signed +10 word followed by unsigned +A8 word, without the historical text list. Do not read that list as a numeric array or reuse the root Results header's string sequence. Counts/text payloads are bounded and original wire bytes remain preserved.

`cosworks_results_category.py` and its audit freshly hash-check native source envelopes/payloads and complete **all 185 retained category Property streams / 1652 bytes** to exact EOF and reconstruction. Observed widths are 8 bytes (176 streams), 24 (8) and 52 (1). **Nine populated historical lists contain twelve texts**, giving native evidence for that retired branch. There are 196 candidate attempts, 185 complete; failed alternatives remain retained. Three tests cover all profiles, both ends of the list window, unsigned values, populated text, invalid counts and strict EOF/truncation/version contracts.

This resolves category Property framing independently of the 53 root Results headers in 14.382. Plot Property and cwPostDlgParam/cwPostGraphParam, convergence and optimization payloads remain separate decoding work. Historical writer equivalence, category field meanings and complete installed-format coverage remain open. No target executes; global semantic coverage credit remains zero.

Evidence: C-763.
