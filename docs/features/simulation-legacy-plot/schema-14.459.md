### 14.459 Historical named-plot records and explicit caller-version compatibility

*Status: forty retained records frame exactly with compatible caller versions; helper-only fits and short records remain separate — Evidence: C-839*

Legacy plot dispatcher **1814AABE0** has a distinct branch for supplied version **<700335**: it opens the stream using the plot's supplied name, then calls **181187EA0** on a temporary plot-dialog record. Later versions instead open named **cwPostDlgParam** and use the imported dialog reader already described in14.385–14.386. Matching a historical helper's byte count therefore does not authorize applying the older named-stream route to every newer context. Subsequent runtime conversion into the current dialog object adds no bytes to this stream.

Helper **181187EA0** reads these historical forms:

| Version | Wire framing |
|---|---|
| <700002 | One raw **1,328-byte** block, then runtime conversion. |
| =700002 | One raw **1,360-byte** block, then runtime conversion. |
| =700003 | Ordered fixed reads totaling **1,344 bytes**, then character-counted wide text550. |
| >=700004 | The same fixed read widths, with raw fixed-buffer conversion in runtime state, then text550; text558 from700005; text560 from700006; words568/56C from700018. |

The ordered fixed reads use hexadecimal object destinations: ten four-byte words08–2C;256-byte buffer30; three words130–138;256-byte buffers13C/23C/33C; six words43C–450;128-byte buffer454; word4D4; eight-byte field4D8; five words4E0–4F0; four eight-byte fields4F8–510; word518; eight-byte fields520/528; seven words530–548; word54C. There is no inserted padding for gaps in those member offsets. The exact46-read sequence is retained in `cosworks-legacy-plot-fixed-reads.json` and independently rematched to the hashed decompilation in C-839. Raw blocks/buffers and scalar widths establish framing, not their UI/physical meanings.

Each appended text uses a **signed four-byte character count followed by exactly twice that many raw bytes**, with no CwPersistStream byte-count header or terminal-NUL requirement. The independent parser bounds the count to its supported512-character profile and retains raw code units; it does not emulate native CString conversions or unchecked buffer behavior. Negative/over-limit counts, truncated data and trailing bytes are rejected. Default state assignments and copies during historical conversion consume no additional bytes.

`cosworks_legacy_plot_record.py` reports helper completion and **named_stream_dispatch_enabled** separately. Fresh inventory selects **60** streams whose final name matches their immediate plot storage under `Cosworks/<study>/<category>/<plot>/<plot>`, totaling **81,744 bytes**:

| Retained result | Streams | Stored bytes |
|---|---:|---:|
| Exact helper EOF and version compatible with below700335 caller gate | **40** | **54,560** |
| Exact helper EOF but no compatible retained caller-version argument | **8** | **10,912** |
| No exact helper completion under retained arguments | **12** | **16,272** |

Compatible successful arguments are **700329** for34 records and **700021** for six. All48 helper-complete records are1,364 bytes. The twelve others are1,356 bytes and stop at the read for word568 under their retained arguments; the audit does not invent a version before700018 to omit the missing eight bytes. Root and coexisting legacy version candidates, every attempted endpoint/error, caller-gate result and original payload hash remain recorded. These context checks prove compatibility with the recovered version gate, not that all runtime study/object/open conditions succeeded in an actual application session.

Two focused synthetic tests cover all historical form boundaries, adjacent text/tail gates, nonempty raw wide text, every generated prefix truncation, caller gate700335, count bounds and trailing bytes. Native fixed-buffer internals, the eight caller-incompatible cases, twelve shorter records, writer/runtime equivalence and full installed-format coverage remain unresolved. No target code executes and global semantic coverage credit remains zero.

Evidence: C-839.
