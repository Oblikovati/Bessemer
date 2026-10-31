### 14.353 SolidManager direct Property header and native replay

*Status: complete framing for the retained direct manager headers; child/body semantics remain incomplete · Evidence: C-729*

The study constructor calls `1813B8F40` at `1814037C0` and stores its result in study member `+160`. That constructor installs primary vtable `18205ED28`; slot `+8` (`18205ED30`) points to `1813CC6C0`, the manager storage method invoked by the retained study reader. C-729 checks the installed constructor/call/pointer bytes, function hashes, decompilation hash, and all five numeric version constants. This binds the following layout to the direct `SolidManager/Property` stream; it does not bind the two body serializers in 14.352 to child classes.

Read the following in order. Each word occupies four bytes; destination offsets are not physical file offsets or assigned business meanings.

| Minimum version | Wire payload |
|---|---|
| Any supported caller version | signed word to manager +48 |
| 800068 | signed word to manager +4C |
| 850032 | signed word to manager +70 |
| 840013 | signed word to manager +180 |
| 870004 | signed text count followed by that many counted UTF-16 texts, appended to list +B0 |
| 870008 | signed text count followed by that many counted UTF-16 texts, appended to list +E8 |

The native branch reaches +180 when either `v >= 850032` or `v >= 840013`; the former implies the latter, yielding the table above. Both text-list reads occur inside that branch and retain their own minimum versions. The writer emits the current layout unconditionally: four words, first count/text list, second count/text list. Do not treat the writer's current layout as the historical reader layout. Independent decoding rejects negative or over-limit counts, bounds each text by its stored byte count, and rejects remainder bytes. Native list insertion behavior is not used to discard preserved wire values.

`cosworks_solid_manager_layout.py` and `cosworks_solid_manager_audit.py` replay all **361 retained headers / 5136 bytes** to exact EOF with contiguous lossless raw spans and SHA-256 reconstruction. Observed sizes are 8 bytes (188 streams), 12 (38), 16 (8), and 24 (127). Both lists occur in the 127 latest-layout headers and are empty in every retained native case. Populated lists therefore have synthetic boundedness/order/raw-preservation checks only. Three unit tests cover threshold boundaries, truncation/remainder rejection, lists/count failures, and explicit integer-version validation.

There are 395 candidate attempts: 365 complete and 30 failed alternatives, retained in C-729. At least one candidate succeeds per occurrence. Root version metadata and the existing study-array association supply candidates; framing success does not establish a universal runtime index-to-storage-suffix mapping. The audit verifies each source sample's outer envelope hash and each nested Property hash before decoding. This evidence does not replay child body records or grant global coverage credit.

The manager also opens child storages named `Solid Components` (`18205ED60`) and `Folders` (`18205ECB0`). Their independent enumeration and virtual child reads remain separate schema work; the manager header contains neither their raw bodies nor an inline component count inferred from their CFB names.

Evidence: C-729.
