### 14.386 DGXPOST dialog binding and bounded initial prefix

*Status: initial prefix validated for all retained dialog streams; no complete dialog-stream claim · Evidence: C-766*

The cosworks plot dispatcher imports **DGXPOST.dll!CcwPostDlgParam::ReadFromStream**, exported by the installed provider at **1800953D0**. C-766 hash-binds both modules, verifies the exact import/export identity, and retains the full dialog reader's native/decompilation hashes. An isolated static Ghidra project supplies the provider decompilation; no target is loaded for execution. The reader rejects null streams and supplied versions below **700335**.

DGXPOST imports **cwPostCommon.dll!CStringReadFromStream**, the character-count helper recovered in 14.385. These strings are signed UTF-16 **character** counts plus twice that many raw bytes, not byte-counted CwPersistStream strings or MFC variable-length CStrings. The independent prefix reader bounds counts to 0..1024 and preserves exact raw code units instead of applying native clamping or locale assumptions.

Before the first embedded PostInfo delegate, dialog fields have this wire order:

1. Fourteen four-byte words: +0C/+10/+14/+18/+1C/+20/+24/+28, +30/+34/+38/+3C, +518/+51C.
2. One temporary four-byte state word. Native decoding derives +520/+524 flags from it without reading additional bytes.
3. Eight words +528..+544, four raw eight-byte fields +548..+560, and three words +568/+56C/+570.
4. Character-count strings +578/+580, word +588, then sixteen raw eight-byte fields +590..+608.
5. Character-count strings +610/+618/+620/+628/+630.

With seven empty strings, this prefix occupies **296 bytes**. Destination offsets identify memory fields, not padding in the stream. Raw eight-byte values and state words are retained without premature physical meanings or native normalization.

The next wire dependency is **CDspPostInfo::ReadFromStream** at dialog +40, followed by **CDspPostStyle::ReadFromStream** at +240. Later branches include moving objects, six iso-plane and six section-plane records, many versioned fields and additional delegates. The retained instructions include 53 version comparisons, but their immediate values alone are not all asserted to be field gates; some guard runtime conversions. The prefix decoder deliberately stops before these dependencies.

`cosworks_post_dialog_prefix.py` returns **prefix_success** and always **complete=False**, with consumed-prefix spans/hash and explicit remaining length. C-766 freshly verifies all **224 Results dialog payloads / 496979 bytes**; every prefix parses and reconstructs exactly, establishing **78674 consumed prefix bytes** and leaving the remaining **418305 bytes** outside this prefix claim. This is not 224 complete stream decodes or an increase in global semantic coverage.

Two tests verify minimum boundary, explicit incomplete status, parent remainder, character-count text, invalid counts and version contract. PostInfo/PostStyle, later dialog dependencies, historical conversion layouts, populated fixtures and full installed-format coverage remain open. No target executes; global semantic coverage credit remains zero.

Evidence: C-766.
