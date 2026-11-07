### 14.278 Cosworks delegated ResultLocations and ParameterSets reader

*Status: installed reader order and version gates recovered; nested record and reference-container bodies remain partial · Evidence: C-654*

**Scope.** Installed `cosworks.dll` function **0x180EBB280–0x180EBB564**, called by the storage routine in 14.277, opens `ResultLocations` and `ParameterSets` in the supplied storage. Their UTF-16 literals are at **0x181F99418** and **0x181F99438**. This establishes the delegate's named streams; it does not establish consumption of the binary `Pressure` body. C-654 binds the module hash, 186 decoded instructions, raw function span, decompilation and seven existing C-590 inventory entries. It performs no new native extraction or target execution.

**Reference stream.** A zero status from opening `ResultLocations` with mode **16** constructs `CwPersistStream` in mode **1**, then calls imported `CSWPersistReferenceContainer.Serialization` on owner **+0x08**, passing the supplied integer version converted to double. The reference-container body remains unresolved. After the parameter-stream path, the function enumerates that container and changes a nonnull reference's zero field **+0x40** to **1**, including when opening `ParameterSets` failed.

**Parameter stream consumption.** On a zero open status, `ParameterSets` is read with the same persist-stream mode. The following is the observed order; owner offsets identify runtime destinations and do not assign domain meanings. Each listed scalar/table value uses the four-byte integer primitive in 14.262.

| Order | Consumption |
|---|---|
| 1 | Five integers to owner **+0x88, +0x9D14, +0x9D18, +0x9D20, +0x9D1C**, in that order. |
| 2 | One integer to **+0x9D2C** when version **> 700013**, then one to **+0x9D30** when version **> 700018**. |
| 3 | One signed integer record count. |
| 4 | Integer table of length **100** for version **< 810026**; otherwise the first integer, owner **+0x88**, supplies the length, with no loop for nonpositive values. |
| 5 | For a positive record count, repeated record reader **0x180EBB1B0**, receiving the version and effective table length. Each allocated **0x138B0** object is initialized by **0x180EB8960** and appended to owner list **+0x50**. Its body remains a dependency boundary. |
| 6 | One integer to **+0x9D10**; one to **+0x9D34** when version **> 820015**; one to **+0x9D38** when version **> 830009**. |

The table loop retains its first **10000** integers at owner **+0x8C** and consumes further integers into scratch storage. Only after consumption does the function cap owner **+0x88** to **10000**. A bounded independent reader must not truncate the wire table at that cap, because doing so moves the record boundary. Conversely, this native code is not evidence that arbitrarily large counts are safe to allocate. The function returns the `ParameterSets` open status; primitive reads do not establish complete successful body consumption.

**Native locations and limits.** The C-590 inventory contains five `ParameterSets` streams totaling **8412 bytes**: legacy `Cosworks/s/ParametricData/ParameterSets` (**1272**), `Cosworks/BasicStudy/ParametricData/ParameterSets` (**2944**) and `Cosworks/S/ParametricData/ParameterSets` (**3846**), plus two `Cosworks/StudyManager/StudyNumber-0/DesignScenario/ParameterSets` streams (**220** and **130**). The latter two documents also contain `DesignScenario/ResultLocations`, each **12 zero bytes**. Preserve exact storage paths and case. Shared leaf names do not prove identical legacy/current layouts, and the zero reference payloads do not prove a general reference-container grammar. No complete native parse or global semantic byte credit is claimed.

Evidence: C-654.

### 14.279 Cosworks ParameterSets nested record layout

*Status: installed record field order and primitive widths recovered; native version/context binding and domain meanings remain partial · Evidence: C-655*

**Reader binding.** The record dependency in 14.278 is **0x180EBB1B0–0x180EBB278** in the same hash-bound `cosworks.dll`. C-655 resolves its seven indirect calls through the installed import table, rather than relying only on decompiler types. The parent passes the effective table length as the fourth argument; this is independent of its record count. Per record, the following bytes are consumed in order:

| Order | Wire representation | Runtime destination / condition |
|---|---|---|
| 1 | CwPersistStream persisted CString, as in 14.262 | Record **+0x08**. |
| 2 | Signed four-byte integer | **+0x10**. |
| 3 | Unsigned four-byte integer | **+0x14**. |
| 4 | Unsigned four-byte integer | **+0x18**. |
| 5 | Four-byte Windows long | **+0x138A8**, only when version **> 700012**. |
| 6 | Eight-byte double payload | **+0x138A0**, only when version **> 700013**. |
| 7 | Repeated eight-byte double payloads | Effective table length entries when positive; first **10000** retained from **+0x20**, further entries consumed into scratch. |

With effective positive table length `T` (zero consumed for a nonpositive length), and `S` bytes for the complete prefixed CString, record width is **S + 12 + 4[version > 700012] + 8[version > 700013] + 8T**. No record-local array count or extra terminal field appears in this function. Preserve raw floating-point bits; neither domain units nor a finite-value constraint is established.

**Double provider.** Imported `??5CwPersistStream@@QEAAAEAV0@AEAN@Z` resolves in the hash-bound `cosutilities.dll` export inventory to **0x18005B7F0–0x18005B81A**. If its stream pointer is nonnull, it invokes `IStream::Read` at vtable **+0x18**, with byte count **8** and null returned-count pointer. It ignores the HRESULT and returns the persist-stream object. This proves byte width, without proving complete native reads on truncation.

**Initialization and limits.** Constructor **0x180EB8960–0x180EB89CD** initializes the CString, zeroes the three initial integer slots, clears **80000 bytes** beginning **+0x20**, and clears the final double/long slots. These memory defaults do not add wire bytes for older versions. As in the parent table, the **10000** retention limit must not shorten the consumed array. C-655 retains function-span hashes, instructions, decompilation hashes and resolved import calls; it executes no target code. An end-to-end native parse still needs a validated per-stream version and storage context. This record grammar does not resolve `ResultLocations` reference bodies or establish a `Pressure` body reader, and grants no global semantic byte credit.

Evidence: C-655.

### 14.280 Native ParameterSets reconstruction and legacy version divergence

*Status: four native streams reconstruct exactly under stored-version candidates; one legacy stream remains incompatible · Evidence: C-656*

**Audit scope.** `cosworks_parameter_layout_audit.py` freshly extracts all five `ParameterSets` occurrences from the C-590 sample inventory. It verifies the outer third-party envelope and each target stream against their retained SHA-256 values, and stores complete raw payloads, coexisting version metadata, attempted versions, and successful field spans in `cosworks-parameter-layout-native-evidence.json`. The bounded static parser applies 14.278–14.279, checks available lengths before consuming, limits counts to **10000**, and requires exact EOF. Each successful span sequence concatenates byte-for-byte to the source. No Simulation code executes.

| Native document / exact stream | Bytes | Matching stored version | Effective table count | Record count |
|---|---:|---:|---:|---:|
| `crank.sldasm` / `Cosworks/s/ParametricData/ParameterSets` | 1272 | Legacy **700338** | 100 | 1 |
| `Design Scenarios/hanger.sldprt` / `Cosworks/S/ParametricData/ParameterSets` | 3846 | Legacy **700021** | 100 | 4 |
| `Verification/static_12.sldprt` / `Cosworks/StudyManager/StudyNumber-0/DesignScenario/ParameterSets` | 220 | Primary **830038** | 3 | 2 |
| `Verification/static_6.sldprt` / `Cosworks/StudyManager/StudyNumber-0/DesignScenario/ParameterSets` | 130 | Primary **830036** | 3 | 1 |

These **four streams / 5468 bytes** establish native framing compatibility under a version actually stored in each document. They do not establish which stream/version the installed application selects at runtime. The legacy streams also coexist with primary versions **870022** (`crank`) and **890116** (`hanger`). Applying those primary versions leaves **1164** and **3438** trailing bytes respectively. Thus a document's primary version must not automatically be assigned to every old named stream. Gate-representative versions also fit some payloads; exact EOF alone cannot identify a unique historical version.

**Explicit unresolved case.** The same `hanger.sldprt` also contains **2944 bytes** at the distinct `Cosworks/BasicStudy/ParametricData/ParameterSets` path. No attempted stored version or gate-representative candidate fully parses under the recovered grammar. Stored legacy **700021** fails CString length framing; primary **890116** leaves **2788** bytes after the attempted layout. A **700012** hypothesis reaches offset **2944** before the required final four-byte field, which is evidence of incompatibility, not permission to omit that field or infer a version. Preserve this stream as raw bytes until its historical reader or a different layout is established. Do not merge it with the uppercase `S` stream.

**Credit boundary.** Complete consumption is framing evidence, not domain interpretation or a native writer round trip. This audit grants no global semantic byte credit. The remaining compatibility case, unresolved reference-container bodies, and Pressure storage binding remain open.

Evidence: C-656.

### 14.283 Bounded lossless ParameterSets decoder validation

*Status: executable framing decoder validated against four native streams; legacy mismatch remains rejected · Evidence: C-659*

`solidworks/cosworkspersist.py::read_parameter_sets(data, version, ...)` implements the installed parent/record grammar in 14.278–14.280. The version is an explicit nonnegative integer supplied by the caller; the decoder does not select a document version, merge legacy paths, or fall back to a guessed historical grammar. It returns immutable header/count/text information and ordered `ParameterSpan` records containing labels, offsets and original bytes. Concatenating those payloads reconstructs the complete input, including raw floating-point bits and otherwise uninterpreted scalars.

The decoder validates available input before every consumption, enforces separate record/table/string limits, rejects negative effective counts and trailing bytes, and checks minimum available record bodies before entering the record loop. Defaults are **10000** records, **10000** table items and **0x100000** bytes per string. The table-item limit is a caller safety policy, not a wire retention rule: when explicitly raised, all entries beyond the native **10000** retention cap remain consumed and preserved. Legacy versions below **810026** use the observed fixed table length **100**, independently of the stored first header value. CString parity/embedded-data handling follows 14.263's existing bounded text decoder; raw spans remain authoritative.

C-659 rechecks complete raw payload hashes retained by C-656, uses each of its four successful stored-version candidates, and compares every decoded span label, offset, end and byte payload with the independently retained audit spans. All **four native streams / 5468 bytes** reconstruct exactly. The incompatible **2944-byte BasicStudy** stream is explicitly rejected at both coexisting stored versions **700021** and **890116**, rather than being accepted through truncation or suffix omission.

Focused tests now total **16 passing tests** across `test_cosworkspersist.py` and `test_cosworksparameters.py`. Added tests cover each version threshold on both sides, truncation at every byte of a populated fixture, suffix rejection, negative/over-limit counts, zero records/table, odd string framing, raw NaN/signed-zero payload preservation, and a **10001-entry** table/record array whose last bytes must survive when allowed. These tests and native reconstruction prove the scoped decoder's framing behavior; they do not prove field meanings, all historical layouts, automatic version selection, native writer round trips or installed-format completeness. No target code executes and no global semantic byte credit is granted.

Evidence: C-659.

### 14.284 Cosworks current ParameterSets writer and historical preservation boundary

*Status: current parent/record write order recovered; historical migration and native round trips unverified · Evidence: C-660*

**Binding.** The installed `ParameterSets` literal has a verified writer reference at **0x180EBC817** inside **0x180EBC750–0x180EBC983**; its other verified reference is the reader in 14.278. The writer calls nested record writer **0x180EBC6A0–0x180EBC74C**. C-660 binds both function spans, instructions and decompilation hashes to the installed `cosworks.dll`, plus the literal-reference evidence hash. No target code executes.

**Parent write order.** Before opening either named stream, the writer caps owner **+0x88** to **10000** when larger. It creates `ResultLocations` through storage vtable **+0x18**, flags **0x1012**, constructs a mode-0 persist stream, and serializes the reference container using integer global **0x18254E3B0** converted to double. It then independently creates `ParameterSets` with the same flags/mode. The runtime global's actual value is not established by this static body.

For `ParameterSets`, it writes eight four-byte values in order: owner **+0x88, +0x9D14, +0x9D18, +0x9D20, +0x9D1C, +0x9D2C, +0x9D30**, then the record-list count taken from owner **+0x68**. It writes the positive **+0x88** number of table integers starting at **+0x8C**, traverses the linked record list beginning at **+0x58**, and passes that same table length to every record writer. Finally it writes three integers from **+0x9D10, +0x9D34, +0x9D38**. There are no historical version gates in this parent body. The stored record count and traversed list length are taken from separate runtime structures; this function alone does not validate their agreement under malformed state.

**Record write order.** Each record emits its CString at **+0x08**, signed integer **+0x10**, unsigned integers **+0x14/+0x18**, Windows long **+0x138A8**, double **+0x138A0**, then the positive supplied table length of doubles beginning at **+0x20**. The long/double fields are unconditional here, unlike the older read gates in 14.279. The record writer itself has no **10000** cap; the observed parent supplies its already capped length.

**Preservation requirement.** This current writer matches the field presence of the newer reader branch but is not evidence of a historical layout emitter. In particular, legacy native streams in 14.280 consume fixed **100**-entry tables even when their first header integer is **3** or **7**. Re-emitting such raw headers through the current writer's variable-length rule would change boundaries. A lossless tool must preserve the validated historical wire layout and raw spans, or perform an explicitly implemented and verified version migration; it must not silently apply the current write order. Likewise, the native read path consumes oversized tables before capping, whereas this writer caps before consumption/emission. Those paths do not prove lossless preservation of oversized native input. The return value follows the `ParameterSets` create status, not a verified aggregate success of both streams and their writes. No native round-trip or global semantic byte credit is claimed.

Evidence: C-660.
