### 14.187 Plastics USD User Data archive-string framing

*Status: USD save entry point, counted UTF-16 strings and first record recovered; remaining payload, matching reader and fixtures remain open · Evidence: C-563*

Installed filter **`User's Data Files (*.usd)|*.usd||`** (RVA `0x83E580`) reaches save entry **`0x18066AD10..0x18066AED3`**, then serializer **`0x18066B670..0x18066DB4F`**. The save entry lowercases a copy of the selected path and searches for substring **`.usd`**; only when absent does it append `.usd`. This is a substring check, not an exact final-extension test. The suffix does not establish interchangeability with other applications' USD formats.

This serializer uses an MFC archive path rather than the line-oriented custom text writes described for BEA/CG/RG. For its inspected string emissions, it obtains a wide-string length N, calls a length helper with Unicode flag **1**, then supplies **2 × N** bytes from the string pointer to an archive byte-write method. There is no terminating NUL included by those calls. N counts UTF-16 code units, not Unicode scalar values or encoded bytes.

Installed MFC ordinal **2374**, **`0x1801D7FA0..0x1801D82FF`**, emits **`FF FE FF`** for the nonzero Unicode flag, followed by this count representation, with little-endian multibyte integers:

| N range | Bytes following the Unicode marker |
|---|---|
| 0–254 | u8 N |
| 255–65,533 | `FF` + u16 N |
| 65,534–4,294,967,294 | `FF FF FF` + u32 N |
| at least 4,294,967,295 | `FF FF FF FF FF FF FF` + u64 N |

The provider supports the final count branch; the observed CString writer's signed length and byte-count operations do not prove that enormous strings can be emitted safely or reached in valid files. The archive raw-write method, MFC ordinal **14225** at **`0x1801D8C20..0x1801D8DA9`**, copies supplied bytes into buffers or forwards byte spans to its underlying file path without a text-encoding conversion in this body. Under normal successful writes, the string payload here is therefore the supplied **UTF-16LE code-unit bytes**, not locale-dependent multibyte text. The marker is archive framing and must not be consumed as a whole-file text BOM.

The first emitted string is exactly **42 hyphens** (RVA `0x83EA80`). Its modeled first record is **88 bytes**: `FF FE FF 2A`, followed by **42 repetitions of `2D 00`**. The next emitted record is the stored string at owner **`+0x38`**, followed by further separator strings at RVAs `0x83EAE0` and `0x83EB40`. The physical name and full sequence of later fields remain unassigned at this checkpoint. Header recognition alone cannot validate the complete file.

`plastics_usd_archive_evidence.py` retains the save entry, full serializer and two MFC helper ranges/source hashes in `plastics-usd-archive-native-evidence.json`. Seven synthetic count-boundary cases and the 88-byte prefix model check framing, not native allocations, flushing or a file fixture. No target execution, fixture equivalence, decoder/full-suite change or added byte-coverage credit is claimed. Complete Plastics USD and all-installed-format schemas remain unfinished.

Evidence: C-563.

### 14.188 Plastics USD linked records and diagnostic profile content

*Status: Initial record sequence, linked-node grouping and profile labels recovered; complete value mapping, reader and fixtures remain open · Evidence: C-564*

The USD serializer in 14.187 emits diagnostic/profile content. Its later archive strings include **`The Customer Profile`**, hardware/OS labels and **`#EOF`**. This classifies the inspected Plastics writer as a User Data/profile carrier; it does not establish mesh/project geometry content or another application's USD grammar. The installed-format inventory must distinguish that role from mesh-bearing formats while retaining this carrier within the requested installed-format scope.

The initial archive record order is: **42-hyphen separator**, owner string **`+0x38`**, separator, separator, **owner i32 `+0xCC` formatted with `%d`**, then separator. All are separate counted Unicode-string records using 14.187's framing. The formatted integer is not a raw binary i32 field. The following loop is controlled independently by a linked-list head at owner **`+0xE0`**, advancing through each node's pointer at **offset 0** until null. The writer does not use the formatted `+0xCC` value as that loop's bound; 14.194 identifies that scalar as a volume serial number, so no count/list agreement requirement follows.

Each linked node emits **ten archive-string records** in this observed order:

| Relative record | Source / construction |
|---|---|
| 1 | 42-hyphen separator |
| 2 | string-conversion helper `0x180606C60` applied at node `+0x110` |
| 3 | separator |
| 4 | six fields formatted as **`%02X %02X %02X %02X %02X %02X`**, sourced from bytes beginning at node `+0x198` |
| 5 | i32 at node `+0x1A4`, formatted `%d` |
| 6 | helper-derived string from node `+0x1D0` |
| 7 | helper-derived string from node `+0x1F0`; empty result replaced with **`N/A`** |
| 8 | **`N/A`** when i32 at node `+0x248` is zero; otherwise helper-derived string at `+0x258` |
| 9 | **`N/A`** when i32 at node `+0x1A8` is zero; otherwise helper-derived string at `+0x220` |
| 10 | empty string |

The six-field hexadecimal representation is one counted string, not six binary bytes in the archive. Formatter width 2 is a minimum, and the first source has a char conversion in the retained decompilation; no normalized six-byte-address parser or physical interface identity is assigned without further evidence. The helper's full string-conversion behavior is also separate from the already bound archive encoding.

After the linked records, the archive contains a labeled profile region. Exact retained labels, in first-reference order, include **`The Customer Profile`**, **`Win32_BaseBoard SerialNumber`**, **`Win32_BaseBoard Product`**, **`Win32_BIOS SerialNumber`**, **`Win32_PhysicalMedia SerialNumber`**, **`Operating System`**, **`Update JanusLock Key Entry`**, **`Product Name`**, **`Memory Status`**, **`CPU Status`**, **`Virtual Machine or not`**, **`Graphic Card`** and **`CPU Process Type`**. Separators, data strings and empty strings occur between them; this label list is not a substitute for the complete runtime record sequence. No actual machine identifiers or profile values were collected for this analysis.

The serializer emits **`#EOF`** as a counted archive string before archive/file close calls. Its text is payload, not an unframed byte sentinel or proof that arbitrary earlier records were valid. A later path also references **`\\Customer.profile`**; its output behavior and complete linkage remain a separate frontier. No runtime export was performed.

`plastics_usd_payload_evidence.py` verifies the full writer bytes/source hash against C-563 and retains first-source-reference literals and linked-structure facts in `plastics-usd-payload-native-evidence.json`. Literal-reference order is explicitly distinguished from full runtime emission order. No native fixture, target execution, decoder/full-suite change or added byte-coverage credit is claimed. Complete profile value mapping, matching reader and all-installed-format coverage remain unfinished.

Evidence: C-564.

### 14.189 Plastics USD narrow-source conversion before archive encoding

*Status: Source byte termination, thread-code-page conversion and wide-copy helper recovered; runtime code page, invalid input and fixtures remain open · Evidence: C-565*

The linked-node string helper from 14.188, **`0x180606C60..0x180606D71`**, accepts a **narrow NUL-terminated byte string**, not a pointer to an already-wide CString. It scans to the first zero byte, allocates/copies that byte string including its terminator, prepares a temporary wide buffer and invokes imported **`MultiByteToWideChar`** with **code page 3**, **flags 0** and input length **−1**. The output buffer capacity is based on narrow byte length plus one. The inspected allocation/size paths do not establish safety for unterminated or enormous inputs.

The installed Windows SDK header identifies code-page constant **3** as **`CP_THREAD_ACP`**, the current thread's ANSI code page. The native instruction's numeric value and import establish this conversion policy; no UTF-8 or fixed legacy code page is hardcoded by this call. Actual thread code page and narrow source bytes were not observed. This dependency affects interpretation of source data before the fixed archive-byte representation is produced.

The subsequent helper **`0x180606FC0..0x18060702E`** scans the resulting wide string to its first zero code unit, allocates **2 × (length + 1)** bytes and copies code units through the terminator, storing the pointer at result object **`+8`**. The parent helper stores its caller-supplied ownership flag at result offset 0; USD call sites inspected in 14.188 pass **1** and conditionally free the returned allocation after use. These ownership fields are runtime bookkeeping, not additional serialized records.

The serializer assigns that helper-produced wide text to a CString and then writes the counted Unicode archive record from 14.187. Thus the two stages are distinct: **narrow bytes interpreted through the thread code page**, then **counted UTF-16LE code units in the archive**. Embedded narrow NUL terminates the conversion input; no original byte-preservation or reversible transcoding guarantee follows. The byte representation must not be described as locale-dependent multibyte archive output merely because its source conversion is code-page-dependent.

If conversion fails, the parent helper selects a null wide pointer and still reaches the wide-copy helper, whose inspected body scans its input without an explicit null guard. That is not a validated empty-string fallback or portable malformed-input recovery rule. Full native exception/failure behavior, allocation errors, replacement policy and nonfinite size conversions remain unresolved; no target failure was induced.

`plastics_usd_conversion_evidence.py` retains both native helper ranges/source hashes, the conversion import and hashed SDK constant declaration in `plastics-usd-conversion-native-evidence.json`. The SDK corroborates the numeric constant's name; it is not claimed as exact binary build provenance. No runtime code page or profile values were collected. No target execution, fixture equivalence, decoder/full-suite change or added byte-coverage credit is claimed. Complete USD field mapping and all-installed-format coverage remain unfinished.

Evidence: C-565.

### 14.190 Plastics USD adapter-node field identities

*Status: Adapter population API and SDK-matched node fields recovered; full profile values, count consistency and fixtures remain open · Evidence: C-566*

Population routine **`0x18066AA40..0x18066AD04`** dynamically resolves **`GetAdaptersInfo`**, invokes it with owner pointer **`+0xE0`**, and on buffer-size status **`0x6F`** allocates the returned size and retries. This is the same linked-list head serialized in 14.188. Combined with exact field offsets, the path binds those nodes to the installed SDK's **64-bit `IP_ADAPTER_INFO`** layout. No adapter API was executed during this analysis.

An explicit layout model using 64-bit pointers and four-byte Windows integer fields reproduces the serialized offsets:

| Node offset | SDK identity | USD node record from 14.188 |
|---|---|---|
| `+0x110` | `Description` | record 2, converted narrow string |
| `+0x198` | beginning of `Address[8]` | record 4, **first six bytes only**, formatted hexadecimal |
| `+0x1A4` | `Type` | record 5, decimal string |
| `+0x1D0` | `IpAddressList.IpMask.String` | record 6, converted subnet-mask text |
| `+0x1F0` | `GatewayList.IpAddress.String` | record 7, converted gateway text or N/A when empty |
| `+0x248` | `HaveWins` | condition selecting N/A versus record 8 data |
| `+0x258` | `PrimaryWinsServer.IpAddress.String` | record 8 data when HaveWins is nonzero |
| `+0x1A8` | `DhcpEnabled` | condition selecting N/A versus record 9 data |
| `+0x220` | `DhcpServer.IpAddress.String` | record 9 data when DHCP is enabled |

The serializer does not inspect `AddressLength` at **`+0x194`** before formatting six address bytes in this body. A generic decoder must retain the actual formatted string, including the minimum-width/char-conversion caveat in 14.188, rather than assume an always-normalized six-octet representation. Adapter type values are preserved as formatted integers; no complete type enum is assigned here.

Only the first embedded address/mask/gateway/WINS/DHCP strings at these offsets are used by the ten-record group. The writer does not traverse the nested `IP_ADDR_STRING.Next` chains here. It also does not explicitly serialize `IpAddressList.IpAddress.String` or `AdapterName` as one of those ten records. The group is therefore a selected adapter diagnostic representation, not a complete snapshot of all adapter settings or addresses. These omissions must not be filled by treating the subnet-mask field as a primary IP address.

The same population routine resolves a second API, retained by exact literal in C-566, and derives the owner string at `+0x38` through the conversion helper and uppercase operation on its successful path. Its complete role and profile-field mapping remain separate. The formatted scalar at owner `+0xCC` is not the serializer loop's bound; 14.194 identifies it as a volume serial number rather than an adapter count.

`plastics_usd_adapter_evidence.py` retains the population routine bytes/source hash, dynamic API literals, hashed SDK declaration source and nine checked field-offset relationships in `plastics-usd-adapter-native-evidence.json`. The SDK is corroborating structure evidence, not exact binary build provenance. No runtime adapter data, machine identifiers or profile values were collected. No target execution, native fixture equivalence, decoder/full-suite change or added byte-coverage credit is claimed. Complete USD semantics and all-installed-format coverage remain unfinished.

Evidence: C-566.

### 14.191 Plastics USD host-name record provenance

*Status: Successful population path identified; default/error output and runtime case mapping remain open · Evidence: C-567*

The second API resolved by population routine **`0x18066AA40..0x18066AD04`** is **`GetNetworkParams`**, named by ASCII literal **RVA `0x83C578`** in the dynamically loaded `iphlpapi.dll`. It receives the owner buffer at **`+0xF0`**, allocates the reported buffer size and retries on status **`0x6F`**. On successful status zero, the routine passes the **base of that buffer** to narrow-string conversion helper `0x180606C60`, assigns the converted value to owner CString **`+0x38`**, and invokes `CString::MakeUpper` on that string.

The retained SDK declaration places **`FIXED_INFO_W2KSP1.HostName` at offset zero**, before `DomainName` and DNS information. Together with the resolved API and unadjusted buffer pointer, this identifies the successful-path owner string as the **converted, uppercased host name**. The USD serializer writes this owner string as **archive record 2**, immediately after the first hyphen separator (14.187–14.188). It is a counted archive string using the established UTF-16LE framing; the narrow source conversion uses **`CP_THREAD_ACP`** (14.189). Exact character conversion and uppercase results depend on the runtime environment and input.

This does not establish that the second record always contains a successfully queried host name. The population routine first assigns a default literal, and API lookup, loading, allocation and conversion failures require separate treatment. The record is not evidence that domain name, DNS-server lists or the rest of `FIXED_INFO` are serialized with it. No actual host name or other machine identifier was queried or retained during this static analysis.

`plastics_usd_hostname_evidence.py` checks the retained population source hash, API literal and SDK source hash and emits `plastics-usd-hostname-native-evidence.json`, including the population bytes. The SDK corroborates the field identity without asserting exact binary build provenance. Native fixtures, error-path output, remaining profile value mappings and all-installed-format coverage remain open. This addition carries **zero byte-coverage credit** and changes no production decoder.

Evidence: C-567.

### 14.192 Plastics USD companion Customer.profile output

*Status: Companion path construction and ordered text chunks recovered; value provenance and failure behavior remain partial · Evidence: C-568*

After closing the USD archive and its file at **`0x18066D8F3`** and **`0x18066D8FF`**, the serializer constructs a separate mode-zero custom text file. It obtains a module handle through `0x18067EA96` and a field at `+0x10`, calls **`GetModuleFileNameW` with capacity `0x200`**, finds the final backslash, takes the directory prefix and appends literal **`\Customer.profile`** (RVA `0x83BCA8`). This path is derived from a module pathname; it is not derived from the user-selected USD pathname. The precise module-handle provider and path-error behavior remain unresolved.

The companion opens through **`0x1805CA3E0` with flags `0x1001`** and emits **13 ordered chunks** through **`0x1805C9990`**:

| Chunk(s) | Content |
|---|---|
| 1 | 42 hyphens followed by LF, literal `0x83C460` |
| 2 | `The Customer Profile` followed by LF, literal `0x83C428` |
| 3 | 42 hyphens followed by LF, literal `0x83C4C0` |
| 4–5 | owner CString `+0x18`, then LF |
| 6–7 | owner CString `+0x20`, then LF |
| 8–9 | owner CString `+0x28`, then LF |
| 10–11 | owner CString `+0x30`, then LF |
| 12–13 | owner CString `+0x40`, then LF |

Offsets refer to the **original owner**. In particular, the decompiler reassigns `param_1 = param_1 + 0x40` earlier in the writer; the final companion value uses that shifted pointer. The five fields are emitted without companion labels in this body. Their complete population semantics remain open, so this clause assigns storage offsets rather than inferred customer-field names. Embedded newlines in their strings could produce more than eight visible lines; the table specifies write chunks rather than a guaranteed line count.

This companion has no archive string-length prefixes or `#EOF` write in the retained output block. It does not copy the adapter list or the complete labeled USD profile sequence. The shared mode-zero writer and `0x1001` open route are the **text path established in 14.172 / C-548**, with `wt` mode and the previously qualified locale-dependent conversion and ordinary disk newline translation. Literal LF characters do not establish LF-only file bytes, UTF-8, a UTF-16 file or a BOM. The companion open call's result is not tested by an explicit branch in this decompiled block; exceptions and partial-write recovery are not established here.

`plastics_usd_companion_evidence.py` checks the retained writer source hash and 13 text calls and retains the writer bytes, exact literal chunks and source-field offsets in `plastics-usd-companion-native-evidence.json`. No runtime files or machine values were collected. Native fixtures and full USD semantics remain open, and this addition carries **zero byte-coverage credit**.

Evidence: C-568.

### 14.193 Plastics USD labeled profile value mapping

*Status: Twelve ordered labeled groups and source fields recovered; underlying value population and native fixtures remain partial · Evidence: C-569*

In archive writer **`0x18066B670..0x18066DB4F`**, the labeled profile portion beginning with the separator at RVA **`0x83B940`** emits the following twelve groups in order. All offsets below refer to the **original owner**, including accesses through the saved owner after the decompiler shifts `param_1` by `0x40` (14.192).

| Group | Exact serialized label | Value source | Value representation |
|---|---|---|---|
| 1 | `Win32_BaseBoard SerialNumber` | `+0x48` | CString |
| 2 | `Win32_BaseBoard Product` | `+0x50` | CString |
| 3 | `Win32_BIOS SerialNumber` | `+0x58` | CString |
| 4 | `Win32_PhysicalMedia SerialNumber` | `+0x60` | CString |
| 5 | `Operating System` | `+0x78` | CString |
| 6 | `Update JanusLock Key Entry` | `+0x80` | CString |
| 7 | `Product Name` | `+0x88` | CString |
| 8 | `Memory Status` | four-byte integer `+0x90` | `%ld MB` |
| 9 | `CPU Status` | four-byte integers `+0x94`, then `+0x98` | `%d  %d MHz` (two spaces) |
| 10 | `Virtual Machine or not` | four-byte integer `+0x9C` | `%d` |
| 11 | `Graphic Card` | `+0x70` | CString |
| 12 | `CPU Process Type` | `+0x68` | CString |

Each group emits a **42-hyphen separator, label, another separator, then value**, each as a separate counted archive string under 14.187. Groups **1–11 additionally emit one empty archive string**. Group 12 instead proceeds directly from its value to the terminal **`#EOF`** archive string. These are archive records rather than newline-delimited text lines. The earlier customer-profile heading and five unlabeled owner strings remain separate from this twelve-group mapping.

The nine CString values are copied into the archive without a numeric conversion in this block. The three numeric formats load the owner fields as four-byte values and use Windows signed decimal formatting (`long` is four bytes here). Memory's ` MB` and CPU's ` MHz` are **literal output suffixes**. They do not independently prove a physical measurement, scaling rule, processor-count interpretation, or unit convention in the population routines. In particular, the CPU format takes `+0x94` before `+0x98`; reversing their order would change the documented record. No boolean validity restriction is inferred merely from the VM label and `%d` formatting.

These field identities are established as **serialized labels paired with storage locations**. They do not prove the accuracy of the reported hardware properties, WMI acquisition implementation, licensing interpretation, or complete string/error normalization. A decoder must preserve the actual strings, including empty values and spacing, rather than reconstructing them from assumed system properties.

`plastics_usd_profile_evidence.py` verifies the retained writer source hash, ordered label references, owner field accesses and three format literals, and retains the native writer bytes and group mapping in `plastics-usd-profile-native-evidence.json`. No runtime hardware values were collected, no native fixture equivalence is claimed, and byte-coverage credit remains zero. Complete population semantics, matching-reader behavior and all-installed-format coverage remain unfinished.

Evidence: C-569.

### 14.194 Plastics USD system values and pre-list scalar correction

*Status: Three value-population routines recovered; root-path helpers, frequency field and fixtures remain open · Evidence: C-570*

Population routine `0x18066AA40` stores the return of **`0x18066FFB0..0x1806700AA`** at owner **`+0xCC`**. That helper initializes a four-byte result to zero and passes its address as the **volume-serial-number output of `GetVolumeInformationW`**, returning that value without testing API success. Its root pathname is constructed through string helpers not fully decoded here. Consequently, the scalar formatted before the USD adapter list is a **volume serial number**, not a declared adapter count. Earlier count terminology is superseded by this finding; the current C-564/C-566 evidence generators and packets use the corrected scalar identity (14.198). No count/list equality invariant exists on this evidence. The writer independently follows adapter `Next` pointers until null. Its `%d` rendering preserves the signed four-byte interpretation of the serial-number bits, including negative text when the high bit is set.

The memory helper **`0x180671C60..0x180671D16`** initializes a 64-byte status structure and calls **`GlobalMemoryStatusEx`**. It reads the eight-byte total-physical-memory field at structure `+8`, converts it as unsigned to double (adding **2^64** after a negative signed conversion), multiplies twice by the exact double **`1/1024`** at RVA `0x83EF20`, and invokes the imported **`ceil`** through `0x180686EBC`. Native instructions then use **`CVTTSD2SI EBX,XMM0`** and return `EAX=EBX`; the decompiler's `void` declaration loses this return and must not be used as the contract. Owner `+0x90` therefore holds **ceil(total physical bytes / 2^20)** on the successful representable path. The archive appends the literal ` MB` even though the divisor is a binary MiB. This is total physical memory, not available memory. The API return is unchecked; no successful query or safe behavior for every floating-point/int32 conversion is inferred.

CPU helper **`0x180671D20..0x180671DB2`** calls **`GetSystemInfo`** and initially takes `dwNumberOfProcessors` at structure `+0x20`. If **`GetActiveProcessorGroupCount()>1`**, it resets the accumulator and adds **`GetMaximumProcessorCount(group)`** for each group index, querying the active group count again during loop control. It returns this four-byte accumulator into owner **`+0x94`**. Thus the multi-group branch uses **maximum counts per group**, not `GetActiveProcessorCount`, physical-core counts or a direct topology enumeration. The second CPU-format field at `+0x98` remains separate; population initially zeros it, and its complete update path is still unresolved.

`plastics_usd_system_evidence.py` retains all three routine ranges, native bytes and decompiled-source hashes, verifies both exact double constants, and resolves the math thunk to the installed CRT `ceil` import in `plastics-usd-system-native-evidence.json`. Static analysis collected no system values or machine identifiers and executed no target code. Native fixtures and complete error behavior remain open; this adds zero byte-coverage credit.

Evidence: C-570.

### 14.195 Plastics USD OS labels and Simpoe.ver product metadata

*Status: OS selection table, product-source path and installed metadata fixture recovered; helper parsing and runtime version reporting remain partial · Evidence: C-571*

Owner CString **`+0x78`** is populated by **`0x180671A20..0x180671C57`**, which calls **`GetVersionExW`** with structure size `0x11C`, retrying with `0x114` on failure. If both calls fail, the result is empty. It resolves `GetNativeSystemInfo` from `kernel32.dll`, falling back to `GetSystemInfo` when that symbol is unavailable. Its fixed selection table is:

| Reported platform/version condition | Exact output |
|---|---|
| platform 0 | `Microsoft Win32` |
| platform 1, major 4, minor 0 / 10 / 90 | `Microsoft Windows 95` / `Microsoft Windows 98` / `Microsoft Windows Millennium Edition` |
| platform 2, major 10 (minor not tested) | `Windows 10` |
| platform 2, major 6, minor 1, product type 1 / other | `Windows 7` / `Windows Server 2008 R2` |
| platform 2, major 6, minor 0, product type 1 / other | `Windows Vista` / `Windows Server 2008` |
| platform 2, major 5, minor 2, product type 1 and architecture 9 | `Windows XP Professional x64 Edition` |
| platform 2, major 5, minor 2, remaining cases | `Windows Server 2003` when `GetSystemMetrics(0x59)==0`, otherwise `Windows Server 2003 R2` |
| platform 2, major 5, minor 1 / 0 | `Windows XP` / `Windows 2000` |
| platform 2, major below 5 | `Windows NT` |
| unmatched cases | empty string |

There is no branch for major 6/minor 2 or 3. The output is a **legacy program-generated label**, not an authoritative current OS identity: major 10 always selects `Windows 10`, and the version-query result may depend on execution context. No actual OS query was executed for this analysis. This table specifies the installed binary's mapping rather than prescribing labels a decoder should regenerate.

Owner CString **`+0x88`** is populated by **`0x180671EC0..0x180672246`**. It obtains a module pathname using global handle `0x180AA3078`, removes the filename and then one directory level, and uses the final component of that remaining directory as its fallback product string. It also appends literal **`\Simpoe.ver`** to the remaining directory path. If the existence helper and mode-zero read-open succeed, it performs **three line reads**, applies `CStringR::TrimToNext` to the third temporary, then builds the output from the first, second and transformed third strings separated by literal spaces. Exact trimming and malformed/short-file behavior remain unresolved; the read return values do not provide explicit rejection branches in this block. The module handle's precise runtime target is not established solely by this function.

An installed candidate exists at **`C:/Program Files/SOLIDWORKS Corp/SOLIDWORKS Plastics/Simpoe.ver`**, with **53 bytes**, retained exactly with its SHA-256 in C-571. Its first two visible lines are `SolidWorks Plastics x64 Edition` and `2014 R1.0 Standard`, each terminated by CRLF, with no third physical line (14.213). These are installed metadata contents, not proof of the running product's release or of the exact final USD string. The file supplies a concrete fixture for completing the text-reader/TrimToNext chain, rather than assuming that the label `Product Name` always comes from executable version resources.

`plastics_usd_os_product_evidence.py` retains both native routine ranges, source hashes, exact literal encodings and installed metadata bytes in `plastics-usd-os-product-native-evidence.json`. This adds a metadata carrier fixture but no verified schema byte-coverage credit. Target execution, native USD output equivalence and all-installed-format completeness remain unproven.

Evidence: C-571.

### 14.196 Plastics Simpoe.ver third-line token removal

*Status: TrimToNext token-removal body and reader mode branch recovered; locale/EOF and native USD equivalence remain partial · Evidence: C-572*

The product reader in 14.195 applies exported **`CStringR::TrimToNext` at `0x1805C71B0..0x1805C72A7`** to the third line. This helper **mutates the source string** and returns a copy of the resulting string. It first invokes `CString::TrimLeft`, then scans code units until one matches the explicit bit-mask delimiter test: code unit below `0x21` with its bit set in **`0x100002600`**. The resulting explicit delimiter set is **HT (`0x09`), LF (`0x0A`), CR (`0x0D`), SPACE (`0x20`)**.

If the scan finds a delimiter after a nonempty token and before the end, it deletes the prefix token and invokes `TrimLeft` again. Otherwise it replaces the source with the empty literal at RVA **`0x8287DC`**. It does not return the first token. A single-token input therefore becomes empty; `first second third` becomes `second third`. It does not trim the right end of a retained suffix. The `TrimLeft` calls retain their provider/locale semantics; the four-character mask describes the explicit scan, not a complete replacement definition for those calls.

| Bounded ASCII input | Returned remainder |
|---|---|
| empty or `single` | empty |
| two leading spaces + `first second third` | `second third` |
| `first` + HT + `second` | `second` |
| `first` + two trailing spaces | empty |
| `first` + CR LF + `next` | `next` |
| `a b  c ` | `b  c `, including trailing space |

The shared line reader **`0x1805C95C0..0x1805C9755`** branches on custom-file mode `+0x30`. For **mode zero**, as selected by the Simpoe.ver reader, it immediately delegates to thunk **`0x18067F20A`** and returns that result. The longer buffer-scanning branch, including its explicit LF removal, is for a different mode and must not be assigned to Simpoe.ver merely because it appears in the same decompilation. Full mode-zero provider EOF and conversion behavior remains a separate verification step.

The retained 53-byte installed fixture has no third physical line; the third read reaches normal EOF and clears its destination under 14.197. Conditional on the successful file-open/provider path, its product string is **`SolidWorks Plastics x64 Edition 2014 R1.0 Standard `**, with a final space: the caller inserts both separators even when the transformed third string is empty. This is a static prediction, not captured USD output or proof that this module handle selects that path at runtime.

`plastics_usd_version_trim_evidence.py` retains the two native ranges/source hashes, checks the empty literal, derives the four mask bits and verifies seven bounded ASCII replay cases in `plastics-usd-version-trim-native-evidence.json`. These examples do not emulate all locale-sensitive behavior. No target execution or additional byte-coverage credit is claimed.

Evidence: C-572.

### 14.197 Plastics Simpoe.ver mode-zero line and EOF behavior

*Status: Installed MFC reader binding and normal wide-stream line/EOF rules recovered; CRT conversion and native USD equivalence remain partial · Evidence: C-573*

Mode-zero thunk **`0x18067F20A`** resolves to **`mfc140u.dll` ordinal 12111**, whose installed export address is **`0x180235330..0x180235500`**. This is the provider used by the three Simpoe.ver line reads in 14.195–14.196. Exact import-slot and export-address-table checks establish the binding; it is not inferred from similarly named overloads.

The provider **clears the destination CString before reading**, obtains a buffer for 128 characters and calls **`fgetws` with capacity `0x81`**. A full 128-character chunk without a final LF causes buffer growth and another read appended to the accumulated string. Thus 128 is a chunk size, not a documented maximum line length. The routine uses string-length operations; arbitrary embedded-NUL behavior is not established as a lossless line format here.

On its normal completion path, it removes **one terminal LF** when present. Its return test uses the **length before that LF removal**. Consequently, a blank LF line returns success and leaves an empty string. Immediate EOF with no characters returns false and leaves an empty string; a final nonempty line without LF returns success. If `fgetws` fails and `feof` is false, the code enters its error/exception path rather than treating the condition as normal EOF. These are rules at the provider's wide-stream boundary. This body does not independently remove CR or establish on-disk encoding, and CRT text-mode translation must be kept separate.

| Conceptual wide-stream read | Output CString | Return |
|---|---|---|
| immediate EOF | empty | false |
| LF | empty | true |
| `x` + LF | `x` | true |
| `x` then EOF | `x` | true |

The product reader does not branch on the three read results. On a normal short-file/EOF path, missing lines are therefore cleared to empty rather than retaining prior string contents. A missing third line and a blank third line both supply an empty input to `TrimToNext`; neither creates an explicit rejection in this caller. This does not extend to I/O exceptions, invalid encodings or all embedded-NUL cases. Successful open still takes the file-composition path even if subsequent normal EOF leaves fields empty.

`plastics_usd_version_reader_evidence.py` verifies both installed module hashes, the thunk bytes, MFC ordinal/export address, empty initializer literal and retained provider range/source hash in `plastics-usd-version-reader-native-evidence.json`. Its four conceptual cases explain the recovered control flow; they are not captured target executions or full CRT emulation. Runtime conversion, native USD comparison and full coverage remain open; added byte-coverage credit is zero.

Evidence: C-573.

### 14.198 Plastics USD composed archive-record grammar

*Status: Writer record-count composition established; matching reader, native fixtures and failure truncation remain open · Evidence: C-574*

The hash-bound full serializer and the section mappings in 14.187–14.197 compose into the following normal completed-write sequence. A record means one counted UTF-16 archive string, not a byte field or physical text line.

| Region | Records | Content |
|---|---|---|
| Initial | 6 | separator, host-name string, two separators, formatted volume serial, separator |
| Adapters | `10 × A` | ten-record group per node reached through the linked list |
| Customer introduction | 9 | separator, `The Customer Profile`, separator, five owner strings (`+0x18`, `+0x20`, `+0x28`, `+0x30`, `+0x40`), empty record |
| Labeled profile and end | 60 | eleven five-record groups, final four-record group, then `#EOF` |
| Total | **`75 + 10 × A`** | A is the number of linked adapter nodes actually traversed |

The fixed regions contain **6, 9 and 60 native raw-write call sites**, respectively. The labeled-group model independently yields `11×5 + 4 + 1 = 60`. Adapter optional-value branches change their strings but retain the ten-record shape. For zero, one, two and sixteen traversed adapters the corresponding modeled totals are 75, 85, 95 and 235 records. This is a writer-derived count relationship, not an on-disk declared count.

The formatted volume serial at initial record 5 does **not** supply A. A decoder must not allocate or loop according to that value. A complete record stream matching this writer can instead be checked against the fixed suffix and ten-record adapter grouping. Record-count congruence alone is insufficient: each string's length framing, fixed labels/separators, value positions and terminal record must also match the scoped grammar. Arbitrary data strings may contain heading or separator text, so searching for the first apparent heading does not establish the section boundary. Native reader compatibility and acceptance of alternate layouts remain unverified.

This composition applies to a normal completed write with a finite traversed list. It does not make a truncated or exception-interrupted output complete, prove list-cycle safety, or establish that every historical USD variant has this layout. The separate `Customer.profile` text file is excluded from the archive record total.

`plastics_usd_record_grammar_evidence.py` checks the retained writer source hash, counts the fixed-region raw writes and composes them with C-564/C-569 in `plastics-usd-record-grammar-native-evidence.json`. The C-564 and C-566 generators/packets now identify `+0xCC` as the volume serial and remove their stale count/list-consistency frontier, linking C-570. This is a correction to the active evidence model, not a new count interpretation. No native USD fixture or added byte-coverage credit is claimed.

Evidence: C-574.

### 14.201 Plastics USD archive input string framing

*Status: Archive helper and installed length/read imports bound; conversion details, oversized counts and native fixtures remain partial · Evidence: C-577*

The loader's string helper **`0x180408A40..0x180408C01`** calls thunk `0x18067FFCC`, resolved to **MFC ordinal 2308 at `0x1801D7D70..0x1801D7F94`**. This provider initializes character width to **1** and reads the standard archive count prefix. Initial byte below `FF` is the count. After `FF`, a little-endian u16 is read: **`FFFE` changes width to 2 and starts another count prefix**, while `FFFF` extends to u32 and a u32 `FFFFFFFF` extends to u64. Thus the writer's **`FF FE FF`** marker is recognized as Unicode, while unmarked narrow strings are also accepted by this helper. No shortest-form/canonical-length rejection appears in this provider body. This does not imply acceptance of arbitrary repeated Unicode markers.

The helper retains the provider return in RBX, but its subsequent length model uses **low 32 bits**: native **`0x180408A94: MOV ECX,EBX`** begins the narrow branch, and the wide branch likewise derives a zero-extended low-word count. It therefore does not preserve a general u64-length contract merely because the provider can decode u64 counts. Width 1 reads N bytes and constructs a wide CString from a narrow pointer plus explicit count; width 2 reads `2×N` bytes and constructs from a wide pointer plus explicit count. The narrow constructor's code-page/invalid-input behavior remains to be traced independently; it is not automatically UTF-8.

Payload reads use thunk **`0x18067EA7E`**, resolved to **MFC ordinal 12030 at `0x1801D89D0..0x1801D8C17`**. The helper compares the returned byte count with the required payload size; mismatch invokes **`0x18067EA6C` → MFC ordinal 2340**, passing error code **3**. The wide branch requests a four-byte byte count (`LEA R8D,[RBX+RBX]`) but compares the returned count with a 64-bit doubled low-word length. Consequently, very large counts require separate overflow analysis; no safe arbitrary-size support is claimed. Stack storage is selected for N≤128 narrow bytes or N≤64 wide code units, with allocation helpers used for larger counts. These are buffer thresholds, not file-format length limits.

The helper reads exactly the counted payload rather than adding a terminator read. It assigns the constructed CString to the destination and releases temporary storage. Unlike the mode-zero line reader in 14.197, this body does not remove LF, CR, surrounding whitespace or an empty record. This distinction is essential for the writer's fixed empty strings and exact labels. Malformed length headers may fail inside the provider before the helper reaches its payload comparison; complete exception behavior remains partial.

`plastics_usd_archive_input_evidence.py` retains both installed module hashes, the native helper and two provider ranges/source hashes, three import thunk bindings and exact narrowing instructions in `plastics-usd-archive-input-native-evidence.json`. No target execution, native USD equivalence, oversized allocation test or added byte-coverage credit is claimed. Complete reader state-machine validation and all-installed-format coverage remain open.

Evidence: C-577.
