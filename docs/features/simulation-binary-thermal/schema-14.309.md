### 14.309 Integrated native Pressure framing with embedded reference archives

*Status: 72 of 89 Pressure streams fully reconstruct; 17 retain explicit named object/continuation stops · Evidence: C-685*

**Integration contract.** `cosworks_embedded_reference_archive.py` adapts the scoped readers validated in 14.298–14.308 to the enclosing Pressure audit. Every provider object call reads its own four-byte archive version, creates a fresh map, and uses a restricted class registry. The adapter currently supports archive versions **4307–19000** and the explicit embedded **0x4000** helper context. Unknown classes, unsupported populated bodies and other contexts remain boundaries. The outer audit checks each returned end is strictly after the start and within input, then incorporates the exact archive bytes into its ordered field spans and reconstruction hash. It does not search for a later field after failure.

The optional archive callback defaults to disabled, preserving C-671 baseline behavior. C-685 freshly reextracts/hash-verifies all **89** family streams, replays **every original candidate** without the callback and asserts equality with the retained successes/failures, ends and spans. It then runs the integrated callback using the same stored-version candidates. C-685 retains baseline, adapter, scoped-reader, audit and test hashes; preceding evidence checkpoints remain unchanged.

**Complete payload results.** **72 streams / 3894679 bytes** consume to exact EOF and reconstruct by SHA-256. This adds **31 complete streams** beyond the **41** in C-671/C-673. There are **73 successful candidate attempts** across those 72 distinct streams; multiple compatible stored candidates for one stream are not counted as extra files or bytes. Choosing one successful attempt per stream, the complete payloads contain **81 Pressure records** and **122 independently mapped object-archive calls**:

| Archive root object | Complete calls |
|---|---:|
| `moSingleFaceRef_w` | 60 |
| `moExtObject_c` | 60 |
| Null object | 2 |

The integrated modern reference path continues after both archive calls through its five integers/CString, reference tails, remaining container entries/tails, full load-base body and derived Pressure suffix. Thus exact EOF now validates the embedded archive boundaries against their enclosing fields, rather than only stopping after isolated objects. Seven successful load-base records exercise the newer terminal-table headers with **zero table entries**; populated terminal records remain unvalidated. Historical primitive/field meanings remain partial even when their byte boundaries are complete.

**Remaining native streams.** The other **17 streams / 111433 bytes** have no complete candidate parse. A supported newer stored-version candidate reaches these explicit stops:

| Named stop | Streams |
|---|---:|
| `moChainComponent_c` body | 8 |
| `moFeatureRefWrapper_c` body | 5 |
| `moRadiateEdgeIdRep_c` body | 1 |
| `moVertexRef_c` body | 1 |
| Edge-specific component continuation | 1 |
| `moDerivedSurfIdRep_c` body | 1 |

Some stops arise in later references/records after a first object pair already completes, explaining why isolated success in 14.308 did not imply a whole-stream result. Other coexisting version candidates can fail earlier; their errors remain separately retained. The audit preserves both enclosing archive starts and inner body-stop offsets in failure messages.

**Boundary tests.** Five focused tests pass for complete null-object consumption without swallowing the next call, truncation at every byte of its header, archive-version bounds, fresh-map rejection of unmapped references, restriction of globally registered classes, and rejection of invalid callback boundaries. These tests cover the integration contract; native EOF/hash reconstruction covers the scoped successful corpus. Neither proves domain semantics, runtime version choice, native writer round trips or all installed-format coverage. No target code executes and no global semantic byte credit is granted.

Evidence: C-685.
