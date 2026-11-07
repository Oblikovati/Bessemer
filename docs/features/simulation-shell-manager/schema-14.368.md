### 14.368 Populated ShellManagerGroup record grammar and CString helpers

*Status: populated record order statically recovered and synthetically bounded; native fixtures remain empty · Evidence: C-747*

The ShellManagerGroup parent path in `181344D10` reads its signed four-byte count, allocates each record with `180C70BF0`, and calls **180C7E650** directly. This is a sequence of direct records, not an MFC CObject-tagged collection and not an embedded archive-version header. C-747 binds the record function and both string helpers to installed function-byte/decompilation hashes and verifies the late-field threshold.

Each record contains the following in order:

| Payload | Native destination |
|---|---|
| Four MFC CStrings | +08/+10/+18/+20 |
| Two signed four-byte words | +28/+2C |
| Eight raw floating-point bytes | +30 |
| Three signed four-byte words | +70/+74/+78 |
| Signed four-byte word, reader only at version >=890016 | +84 |

The current writer always emits +84; the bounded parser therefore exposes explicit historical `reader` and current `writer` profiles. Destination offsets do not establish field meanings. Preserve raw double bits rather than normalizing NaNs or signed zero.

String read helper **180B59E10** obtains the archive string length/character width, reads either narrow bytes or twice the UTF-16 character count, then constructs a CString. Write helper **180B5A000** writes the MFC length with wide-character mode and then exactly twice the stored character count. This is the MFC variable length/UTF-16-marker encoding already implemented by `swarchive.Archive.cstring`, **not** the CwPersistStream u32-byte-count/terminal-NUL text format. The independent reader retains original narrow/wide raw spans even though native narrow conversion may use runtime locale.

`cosworks_shell_group_layout.py` validates signed count, available minimum record bytes, every primitive/string bound, EOF, and complete reconstruction. It accepts populated groups only under the recovered record grammar; unexpected version/profile inputs, truncation, negative/over-limit counts, and remainder are rejected. Three tests cover the 890015/890016 boundary, explicit writer asymmetry, narrow/wide text, raw NaN bits, count bounds and truncation/remainder.

C-747 freshly hash-checks/replays **all 69 retained native groups / 276 bytes**, all single zero counts, with exact EOF. These confirm the parent empty boundary only. Populated-record validation remains static/synthetic; no additional populated native fixture or global semantic coverage is claimed. C-747 resolves the previously unknown populated record order from 14.363, while physical meanings, historical variants outside the recovered gate, and full installed-format coverage remain open. No target executes.

Evidence: C-747.
