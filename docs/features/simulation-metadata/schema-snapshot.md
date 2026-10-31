### 14.110 Simulation optional TimeStampGUIDs string stream

*Status: Native four-string framing verified; populated fixtures and field roles remain unresolved · Evidence: C-486*

The compound stream **`TimeStampGUIDs`** is distinct from **`TimeStamp`**. The complete UTF-16 name, including its terminating NUL, is bound at **cwanalysisproc.exe RVA 0x13138**. Writer **0x14000B600..0x14000B951** creates the optional stream through `IStorage::CreateStream`, mode **0x1012**, and emits four caller strings in order. It returns before creating the stream when either the third or fourth caller CString has length zero. This gate does not imply that the first two strings must be nonempty.

Each field has this byte layout, with no transfer-count prefix around the stream:

| Relative offset | Size | Meaning |
|---|---|---|
| 0 | 4 | Little-endian unsigned byte count L |
| 4 | L, when nonzero | String bytes including the final two-byte NUL |

For a null BSTR pointer the writer emits **L = 0** and no content. Otherwise it uses `SysStringByteLen + 2` and writes that many bytes, including the terminator. An allocated empty string therefore has **L = 2**, distinct on disk from a null string. Native imports bind the conversion to `SysAllocString` and the byte count to `SysStringByteLen`. String lengths count bytes, not UTF-16 code units or Unicode code points. No GUID syntax, GUID width, timestamp unit or identity role is established merely by the stream name.

Reader **0x140004CE0..0x1400053B1** opens the same stream and performs four corresponding reads in order. It requires a complete four-byte count. A nonzero count passes an unsigned **`(L - 2) <= 0xFFFFE`** check, establishing the inclusive byte range **2..0x100000**; it allocates **L - 2** bytes through `SysAllocStringByteLen`, reads those bytes and then separately requires a complete two-byte zero terminator. The native check has **no even-length guard**. The portable model supports the even UTF-16 subset and rejects odd counts explicitly instead of assigning them Unicode semantics. Failure handling, allocation errors and seek restoration remain native behavior outside the model.

The native reader then assigns the recovered BSTR pointer into its caller CString. Embedded NUL code units can therefore leave a longer stored byte string than the caller-visible prefix. The portable `read` view retains the entire byte content and separately reports that prefix. It uses surrogate-preserving UTF-16 decoding; unpaired surrogate code units are retained rather than replaced. Null and allocated-empty records remain distinct in the raw field view even if a caller later treats both as empty text.

`solidworks/simulationguids.py` implements `read` and `project_writer`. The reader preserves offsets, counts, field bytes, whole source and any trailing bytes after the fourth field, reporting exact stream completion only when no tail remains. All four roles stay unassigned. The successful writer projection accepts four NUL-free caller strings or null pointers, applies the third/fourth-string creation gate and emits the inspected framing. Its maximum length is constrained to the paired reader profile; this is not a native writer limit. Embedded-NUL caller input is outside that projection because native `SysAllocString` scans a terminated source pointer. No COM storage or installed target is executed.

All **three installed CWR carriers lack `TimeStampGUIDs`**. An explicitly synthetic 26-byte stream encodes **null, empty, `A`, `B`** and reconstructs exactly from independent fixed bytes. Five focused tests cover field order/offsets, null versus empty, supplementary characters and unpaired surrogates, stored versus caller-visible NUL handling, the creation gate, trailing bytes, the inclusive reader limit, malformed lengths and missing terminators. `simulation_guid_evidence.py` retains two complete native ranges, import bindings, literal/source/module hashes, carrier absence and the synthetic replay in `simulation-guid-native-evidence.json`. This establishes an optional grammar without adding observed populated corpus bytes or changing global/all-installed-project coverage percentages. The four string roles, actual populated examples and other producer versions remain open.

All **1,583 regression tests pass** (185.871 seconds). The native/corpus verifier passes; specification lint reports zero findings (672 headings, 96 anchors).

Evidence: C-486.

### 14.111 Simulation TimeStamp update-stamp binding and RunState scalar writes

*Status: Bare scalar layout and current-build GetUpdateStamp source verified; RunState enum remains unresolved · Evidence: C-487*

`TimeStamp` and `RunState` are **bare four-byte little-endian scalar streams**, without the transfer-count prefix of 14.101. Their exact names are bound at **cwanalysisproc.exe RVAs 0x131B8 and 0x131A0**, respectively. The helper writer **0x140006240..0x14000B5FC** creates each root stream in mode **0x1012** and passes four bytes from its first and second caller arguments to `IStream::Write`: the first argument supplies `RunState`, and the second supplies `TimeStamp`.

The two inspected helper callers **0x1400022B0..0x14000241E** and **0x14000BBF0..0x14000C14B** both supply **RunState = 1**. The second argument comes from a command-line string converted through `_wtol`. This proves these callers' stored constant and source path; it does not establish a complete state enum or identify 1 with success, failure or the solver return code. Negative/large bit patterns in portable tests are synthetic cases rather than additional observed state values.

The application-side `TimeStamp` literal is bound at **cosworks.dll RVA 0x1F590C8**. Three related operations establish its scalar handling:

- **0x180ABEBF0..0x180ABED6C** opens the stream and requests four bytes, then copies the resulting word to its caller output.
- **0x180D5AFD0..0x180D5B0F0** opens an existing stream for read/write and writes the supplied four-byte argument at its initial position.
- **0x18143AE40..0x18143AFA3** writes four bytes from study-object offset **0xEC** to the stream. The larger create path **0x18143D0D0..0x1814434EC** creates the stream and writes that same field. These offsets describe native runtime object storage, not an additional on-disk header.

In the inspected create path, when either of two application-state fields at offsets **0x46C / 0x470** is zero, the study field is refreshed through accessor **0x1814EA240..0x1814EA276**. Otherwise the path retains its existing study-field value. The accessor invokes dispatch ID **0x10145**, flags **1**, with result type **3**, and returns the resulting four-byte word. Its thunk **0x181712451** binds to `mfc140u.dll` ordinal **7836**. The installed interop assembly independently identifies that dispatch member as **`GetUpdateStamp`**.

The metadata binding uses `SolidWorks.Interop.sldworks.dll` SHA-256 **`bb8cc4837bf45c47d4a553a0e88c4c584e5b3f2c9d07a4d05e6fe15cfbe468f4`**, assembly version **34.2.1.2**, MVID **`ea2adaf7-38c8-4da6-a937-6b248dbfe246`**. `IModelDoc`, `ModelDocClass`, `IModelDoc2` and `ModelDoc2Class` all associate **GetUpdateStamp** with `DispIdAttribute` blob **`01 00 45 01 01 00 00 00`** and method signature **`20 00 08`** (instance method, zero arguments, signed 32-bit return). These are build-local metadata identities. They establish a **model-document update stamp** source for this write path, without assigning a wall-clock unit, epoch, increment rule or cross-session uniqueness guarantee. The current build's source binding does not independently identify every historical writer used for older installed fixtures.

`solidworks/simulationscalars.py` exposes an exact four-byte `read` view with both signed value and unsigned bit pattern. `TimeStamp` receives the update-stamp meaning; `RunState` retains an unassigned meaning and enum. `project_helper` accepts an explicitly supplied signed 32-bit update stamp and returns the two inspected callers' proposed scalar bytes, including fixed **RunState = 1**, without storage operations, command-line conversion or solver execution. It does not emulate every application-side writer. Native stream operations do not consistently check read/write HRESULTs and byte counts; the portable model covers complete successful scalar data and does not reproduce partially initialized native outputs or error-path behavior.

The installed corpus contains **three TimeStamp streams**, values **213, 213, 0**, and **one RunState stream**, value **1**: **16 bytes** in total. The known TimeStamp payload is **12 bytes**; the remaining four-byte RunState value has verified representation and caller constant but an unresolved enum. Each installed timestamp reconstructs exactly from its signed value, and the installed RunState matches the helper projection. This does not regrade global or all-installed-project percentages.

`simulation_timestamp_dispatch.py` retains the four matching custom attributes and method tokens in `simulation-timestamp-dispatch.json`. Static REA inspection records metadata-only evidence **ev_5c148d4ead6c990386043e2f09dd96252007f3e6de30fe54790adecd52e72e3b**, linked to complete metadata parent **ev_aa28171dac564aa23e929a242b5fcf817b23ecda8f113bb04c5d83d143b5f363**. `simulation_scalar_evidence.py` retains **nine native ranges/thunks**, source/module hashes, literal and import bindings, metadata linkage and all installed scalar bytes in `simulation-scalar-native-evidence.json`. Five focused tests cover observed values, signed extremes and raw patterns, fixed caller state, unknown enum preservation, input guards and rejection of an invented transfer envelope. No installed native code, CLR assembly or COM target was executed. Complete state semantics and historical producer behavior remain open.

All **1,588 regression tests pass** (245.050 seconds). The native/corpus verifier passes; specification lint reports zero findings (673 headings, 96 anchors).

Evidence: C-487.
