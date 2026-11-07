### 14.100 Simulation logical filenames and CWR root-stream lookup

*Status: Native lookup and installed compound-stream binding verified; stream payload schemas remain partial · Evidence: C-476*

`cwcommfuncs.dll!FindStreamFileFromCWStorageFile` at **0x180005320** interprets its Unicode logical filename as a compound-storage lookup. It finds the **last dot in the entire input**, takes the substring after that dot as the stream name, and concatenates the substring before that dot with literal **`.CWR`** as the storage filename. Helper **0x180004A10** calls the imported CString concatenation operation with the two strings and their lengths. The `.CWR` literal is retained at RVA **0x12828**. Thus `C:\study.GEN` selects root stream `GEN` in `C:\study.CWR`; `study.MAS` selects root stream `MAS` in `study.CWR`.

This is a lexical split. The routine does not trim or uppercase the suffix, strip directories before finding the last dot, or substitute a format-specific stream table. For example, the input `a.b\file` projects to storage `a.CWR` and stream `b\file`; this does not imply that COM accepts that stream name. Case-insensitive COM name matching, filesystem resolution and wildcard effects of the existence probe are outside the lexical model. Names without a dot and names containing embedded NUL are outside the reproduced profile.

The native procedure performs these operations:

| Step | Native operation | Observed condition/result |
|---|---|---|
| 1 | `FindFirstFileW(storage_name, ...)`, then `FindClose` | Invalid handle returns false |
| 2 | `StgOpenStorageEx` | Mode `0x10`, storage format `0`, attributes `0`; failed HRESULT or null storage triggers message resource `0x24F`, then returns false |
| 3 | `IStorage::OpenStream` at vtable offset `0x20` | Unchanged suffix, reserved pointer null, mode `0x10`, reserved word zero |
| 4 | Test stream-open HRESULT | **Exactly zero** sets the return flag to one and releases the stream; other values leave zero |
| 5 | Release storage and return flag | No payload reads occur in this predicate |

The decompiler declares the export `void` and loses its return flag. Native instructions are authoritative: **0x180005455** sets `EDI=1`, **0x1800054AA** returns `EAX=EDI`, and **0x1800054F7** sets the failure result to zero. The separate `FindStreamFile` at **0x1800052A0** only checks an ordinary filename with `FindFirstFileW`; it does not establish a compound-stream payload layout.

`solidworks/simulationstorage.py` provides the pure `locator(logical_name)` projection for NUL-free inputs containing a dot. It preserves case, whitespace and Unicode text, reports the two open modes and storage format, and explicitly marks payload decoding false. It performs no filesystem or COM calls. Five focused tests cover GEN/MAS lookup, case/whitespace, lexical directory-dot and empty-suffix cases, Unicode, and profile guards.

The read-only corpus check covers all **three installed CWR fixtures**, all MS-CFB major version **4** with **4,096-byte sectors**. The two composite fixtures each contain `GEN` (1,282,856 bytes), `MAS` (28,420 bytes) and `TimeStamp` (4 bytes). The benchmark fixture contains `GEN` (3,306,932), `MAS` (28,420), `ELF` (1,224), `OUT` (4,970), `RunState` (4) and `TimeStamp` (4). All **12 root streams**, totaling **5,964,114 logical bytes**, bind through constructed logical filenames to their observed storage and exact stream names. Constructed lookup inputs do not recover historical filenames supplied by the application. No semantic interpretation is assigned to these payloads by this check.

**Legacy GEN remains separate.** `cosworks.dll` routine **0x180EA3FA0** opens a `.GEN` filename through MFC `CFile` and reads a legacy flat-file layout. Its fixed blocks, version branches and mesh-container reads are not assigned to the CWR `GEN` stream: a native conversion or reader path establishing their relationship has not yet been recovered. Matching extensions and initial word patterns are insufficient evidence. GEN and MAS payloads remain opaque in this clause, and global byte-coverage percentages are unchanged.

`simulation_storage_evidence.py` retains module, native-byte, decompilation and sample/stream hashes in `simulation-storage-native-evidence.json`, including three complete native ranges. The installed `cwcommfuncs.dll` SHA-256 is `888490cb99a3978a63eb32a0095ccfad6fec95dcd8c377661c84e7f56b5fa2a9`. No installed native target was executed.

All **1,532 regression tests pass** (247.947 seconds). The native/corpus verifier passes; specification lint reports zero findings (662 headings, 96 anchors).

Evidence: C-476.
