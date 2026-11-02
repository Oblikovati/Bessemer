### 14.385 cwPostCommon graph parameters and character-count strings

*Status: all 224 retained cwPostGraphParam streams complete; dialog reader is a different provider · Evidence: C-765*

The cosworks plot dispatcher imports **cwPostCommon.dll!CcwPostGraphParam::ReadFromStream**. The installed provider exports that reader at **180027B20** and its **CStringReadFromStream** helper at **180028CF0**. C-765 hash-binds both modules, verifies the exact caller import/provider export, and retains native function/decompilation hashes. The adjacent dialog import belongs to **DGXPOST.dll** and is not substituted with this graph grammar.

This string helper reads a **signed four-byte UTF-16 character count**, then twice that many bytes. There is no CwPersistStream byte-count/terminal-NUL contract and no MFC variable-length CString marker. Native counts above 1024 are clamped before reading; nonpositive counts return without consuming characters. The independent decoder deliberately rejects negative or >1024 counts, preserves raw UTF-16 bytes including unusual code units, and does not clamp a longer serialized string and pretend its suffix belongs to the next field.

The graph reader rejects a null stream or supplied version below **800018**. For supported versions, read twelve four-byte words (+08..+34); optional word +38 from **870002**; three words +3C/+40/+44; optional word +48 from **870002**; then three character-count strings (+B0/+B8/+C0). Read six words +4C..+60 and eighteen words +64..+A8. Continue in this wire order:

| Inclusive minimum version | Graph continuation |
|---|---|
| 800026 | word +C8, character-count string +D0, sixteen raw eight-byte values +D8..+150 |
| 800062 | words +158/+15C/+160 |
| 820022 | word +16C, raw eight-byte values +170/+178, words +180/+184 |
| 840028 | word +190 |
| 890005 | six words +198..+1AC and raw eight-byte value +1B0 |
| 890110 | word +1F8 |

The >=890005 and >=890110 reads sit inside a native >=890002 continuation; their thresholds imply that outer gate. Earlier in-memory enum/default adjustments consume no extra fields. Widths are preserved without assigning chart semantics from destination names alone. Direct IStream short reads are not emulated as successful complete data: independent parsing validates every available byte and exact EOF.

`cosworks_post_graph_layout.py` freshly replays **all 224 retained graph streams / 83328 bytes** to exact EOF and full SHA-256 reconstruction. Three tests cover all recovered threshold widths, the two 870002 words, character-count text, invalid count profiles, truncation/remainder and explicit minimum integer versions. Candidate version association remains distinct from proven universal runtime study-index selection.

C-765 resolves retained graph-stream framing; DGXPOST dialog parameters, auxiliary optimization/convergence streams, field meanings, historical writer equivalence and complete installed-format coverage remain open. No target executes; global semantic coverage credit remains zero.

Evidence: C-765.
