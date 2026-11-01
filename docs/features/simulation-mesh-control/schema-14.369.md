### 14.369 MeshControlManager headers and record-subobject dispatch

*Status: all retained study headers complete; DocumentInfo carrier and counted records remain open · Evidence: C-748*

The study-bound MeshControlManager storage reader **1810EEFF0** opens Property independently of the named **MeshControl** stream (`181F50D58`). Direct study Property contains word +08, word +50 only at version **>800027**, and word +60 only at **>=840013**. All are four-byte signed words. The current writer emits all three unconditionally. C-748 verifies both threshold bytes and the installed manager/stream literals, function hashes and decompilation hashes.

`cosworks_mesh_control_header.py` and its audit freshly hash-check retained source envelopes and payloads. All **357 study-level headers / 3532 bytes** reconstruct exactly and reach EOF. The name-based inventory also finds one **four-byte** Property under **Cosworks/DocumentInfo/MeshControlManager**, with candidate 890620. That carrier does not match the twelve bytes expected by the study reader at that version and remains rejected. The study dispatch chain does not bind a DocumentInfo handler merely because its last storage name matches. Across all 358 inventoried headers / 3536 bytes, the latter four bytes remain unresolved. Two tests cover both threshold boundaries, signed raw preservation, exact lengths and version contracts.

The separate MeshControl stream starts with signed record count, then direct records. Native allocation uses constructor **1810EA690**; serialization is called on **object +8**, through that subobject's virtual **+80**. The constructor installs its +8 table at **1820326D0**, slot **182032750** points to **1810EEBD0**, and C-748 freshly verifies this pointer. Using a primary-object slot would bind a different method. The writer opens the collection only for positive stored count; absence is not a four-byte zero-count payload.

Record reader 1810EEBD0 first delegates to the shared base **180B754B0** at the subobject pointer, then reads two doubles (+370/+378), three words (+380/+388/+24), and three counted texts (+2E8/+2F0/+310). Continue with these verified gates in wire order, using offsets relative to the serialization subobject:

| Minimum version | Record continuation |
|---|---|
| 800032 | word +320, double +328 |
| 850015 | text +318 |
| 850016 | words +330/+334 |
| 850018, inside previous branch | word +384 |
| 870016, inside previous branch | words +3C8/+3CC |
| 890111, inside previous branch | doubles +390/+398, words +3A0/+3A4/+3A8, words +368/+36C |

The text/later branches are reached only after the >=800032 path and its >=830000 continuation; all listed late minimums imply those conditions. Below 830000, native toggling/default assignments do not consume extra bytes. The first double's missing Ghidra destination is not interpreted as a missing wire field. Current writer fields are unconditional and must not overwrite historical reader gates.

The retained inventory contains **12 MeshControl streams / 86359 bytes** whose counted records are not claimed decoded by this header audit. Shared-base prefix boundaries and subobject-relative offsets must be verified in native replay before claiming complete records. DocumentInfo dispatch, loaded-field meanings, historical variants and complete installed-format coverage remain open. No target executes; global semantic coverage credit remains zero.

Evidence: C-748.
