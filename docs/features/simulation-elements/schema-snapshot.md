### 14.112 Simulation element classification inputs and legacy code rewrite

*Status: Native numeric selector and version-gated rewrite verified; stored values preserved and engineering enum names unresolved · Evidence: C-488*

The 140-byte GEN element record contains additional fields consumed by **cosworks.dll!0x180F80D20..0x180F80EAB**. The routine obtains an element through the same `CElemContainer` index operator used in 14.104 and selects a numeric classification from these bytes:

| Record offset | Size | Verified role |
|---|---|---|
| 20 / `0x14` | 1 | Selector family F; numeric domain remains unnamed |
| 23 / `0x17` | 1 | Unsigned accessor node count N, already documented in 14.104 |
| 24 / `0x18` | 1 | Unsigned selector variant V; numeric domain remains unnamed |
| 26 / `0x1A` | 2 | Little-endian stored code; legacy loader can overwrite this field |

The selector returns the following **32-bit** numeric codes. Offsets 20 and 24 are selector inputs, not inferred physical quantities. No engineering type names are assigned to the codes.

| Family | Condition | Selected code |
|---|---|---|
| 4 | N = 4 | 98 |
| 4 | Any other N | 92 |
| 2 | N = 3 | 4 |
| 2 | N = 2 | 8 |
| 2 | Other N | `0xFFFFFFFF` |
| 1 | N = 1 | 37 |
| 1 | Other N | `0xFFFFFFFF` |
| 8 | N = 3; V = 0, 1, 2 | 63, 43, 53, respectively |
| 8 | N = 4; V = 0, 1, 2 | 64, 44, 54, respectively |
| 8 | N = 6; V = 0, 1, 2 | 94, 90, 99, respectively |
| Other F | Any N/V | `0xFFFFFFFF` |

For family **8**, cases outside the three explicit N/V rows use these ordered fallbacks: if **N is divisible by 3 and V > 2**, return **2**; otherwise, if **N is divisible by 4**, return **42** for V > 2 or **93** otherwise; otherwise return **93**. Zero N follows those arithmetic tests too. These are the native branch rules, not a claim that every byte combination describes a valid mesh. The native zero-identifier case returns `0xFFFFFFFF` after an index lookup; the portable record-based view does not reproduce potentially invalid native memory access.

The GEN loader **0x180EA3FA0..0x180EA592A** has a legacy post-load loop guarded by a native comparison against **`0xC8338` / 820024**: the rewrite applies below version **820025**. For a record visited by that loop, **N = 2** overrides the selector: analysis code **4** stores **28**, and another analysis code stores **37**. For other N, the loop calls the selector and stores its low sixteen bits at offset 26; `0xFFFFFFFF` therefore becomes **65535**. The analysis code comes from runtime context, not the selector-family byte. Loop visitation and the enclosing reader's other context/gates must be established separately; this field projection is not a full loader emulator.

**Stored code and selected code are not interchangeable.** The two installed version-830038 composite files each contain 5,232 records with **F = 8, N = 6, V = 0**, so the current selector returns **94** for every record. Their stored codes are **63** in 5,225 records, **94** in six and **65535** in one. The version-850023 benchmark has 14,642 records with **F = 4, N = 10, V = 0**: the selector returns **92**, while every stored code is **0**. Thus **25,094 of 25,106 installed records differ**, and only twelve match. All three files are above the legacy rewrite threshold. This is not evidence of corruption and does not authorize replacing historical stored values.

`solidworks/simulationelementkind.py` exposes `read`, which preserves the entire record and reports the two selector bytes, existing count, stored u16 and independently selected u32/signed result. It leaves the engineering name unassigned. `project_legacy` requires an explicitly supplied version in the supported reader subset **800050..820024**, an analysis code and a record assumed visited by the native loop. It returns a copy changing only bytes **26..28**. It performs no file write and is not applied automatically to installed records.

`simulation_element_kind_evidence.py` retains the complete selector and GEN-reader ranges, module/source hashes, input-byte instructions, exact version comparison and rewrite stores, and per-file stored/selected histograms in `simulation-element-kind-native-evidence.json`. Across the corpus, the three selector inputs occupy **75,318 bytes**, including the already documented count byte; the stored code occupies **50,212 bytes**. These are overlapping field-attribution counts, not an exclusive new-coverage total or a global percentage regrade. Five focused tests cover family branches, explicit tables, unsigned fallback boundaries, two-node overrides, sentinel narrowing, source preservation and profile guards. No installed target was executed. Engineering enum meanings, other field roles and historical producer behavior remain open.

All **1,593 regression tests pass** (191.063 seconds). The native/corpus verifier passes; specification lint reports zero findings (674 headings, 96 anchors).

Evidence: C-488.

### 14.113 Simulation solid midside topology and first-record beam predicate

*Status: Named native consumers establish selected topology and predicate roles; complete element enums and geometry remain open · Evidence: C-489*

The exported **`DoMidNodeAveragingForSolidMesh`** routine in **cwgeopost.dll, 0x180038B90..0x18003986E**, consumes the 140-byte element layout through `CPostElemContainer`. The wrapper accessor at **cwmeshcontainer.dll!0x180002B60..0x180002B82** follows the underlying container pointer and selects an 8,192-record chunk with **140-byte** stride. This binds the postprocessing consumer to the same record/node-list layout used in 14.104 and 14.112.

The inspected averaging path requires a nonzero activity byte, family byte **4** at record offset 20 and node count **10** at offset 23. It has additional caller/runtime selection gates. Within that path, the first four node-list entries supply corner values and the next six entries receive contributions from specific corner pairs. Node IDs are signed four-byte values in the list beginning at record offset **96 / 0x60**. The following slot indices are zero-based; stored node IDs retain their one-based indexing.

| Midside slot | Record byte offset | Endpoint slots | Endpoint byte offsets |
|---|---|---|---|
| 4 | 112 / `0x70` | 0, 1 | 96, 100 |
| 5 | 116 / `0x74` | 1, 2 | 100, 104 |
| 6 | 120 / `0x78` | 0, 2 | 96, 104 |
| 7 | 124 / `0x7C` | 0, 3 | 96, 108 |
| 8 | 128 / `0x80` | 1, 3 | 100, 108 |
| 9 | 132 / `0x84` | 2, 3 | 104, 108 |

These six endpoint pairs connect every pair of the four corner slots. The role comes from the named native averaging consumer and its explicit slot reads, rather than the numeric classification code. The routine adds half the sum of the two endpoint result values to each midside result accumulator; the binary32 constant **0.5** is bound at RVA **0x6D9F0**. Later accumulation/counting and selection behavior belong to the runtime postprocessing algorithm. The projection below extracts the node associations without emulating that arithmetic, assigning result-component names or imposing coordinate midpoint constraints. A curved element's midside coordinates need not equal the straight average of endpoint coordinates.

The separately exported **`IsPostElementTypeBeamOnly`**, **cwgeopost.dll!0x18001C760..0x18001C7B4**, implements a narrower predicate than its name might suggest. After positive document/container counts and a non-null container pointer, it examines **only record index 0**. It returns 1 when that first record has family byte **2** and the stored two-byte code at offset 26 is **not 37 / 0x25**; otherwise it returns 0. It does not scan the remaining elements or test the first record's activity byte. The stored-code test is independent of the derived selector in 14.112. This does not establish a complete family-to-engineering-type enum or prove that every record in a container is a beam.

Installed `SolidWorks.Interop.cosworks.dll` metadata defines **`swsSimulationElementTypes_e`** as `swsSolidElement = 0`, `swsShellElement = 1`, `swsBeamElement = 2`, `swsTrussElement = 3`. Those API values are retained with the assembly hash in the evidence packet as a separate numeric domain. No conversion from that enum to record-family or stored-code fields has been bound. Numeric equality for one member does not establish such a mapping; in particular, the API solid value is 0 while this named solid averaging path tests record family 4.

`solidworks/simulationtopology.py` exposes `solid_ten_node`, which requires the active family-4/count-10 context, preserves the record and returns four corner node IDs plus the six ordered midside/endpoint associations. It reports invalid IDs against an optional supplied node count, preserving duplicates and order. `beam_first_record_predicate` implements only the record test after the native count/pointer guards; it neither supplies those guards nor examines another record. Neither helper rewrites element fields, computes geometry or runs postprocessing.

All **14,642 benchmark elements** satisfy the topology projection's record gate, producing **87,852 associations** with **zero invalid node IDs** against the paired GEN node count. Neither composite fixture has an eligible family-4/count-10 record. All three installed first-record beam tests are false; true and stored-code-exclusion cases are explicitly synthetic tests. Five focused tests cover the exact edge-slot order, invalid/duplicate IDs, preservation, context guards and the beam predicate's lack of an activity test.

`simulation_topology_evidence.py` retains **three native ranges/leaves**, decorated export identities, source/module hashes, gate instructions, the half constant, API enum metadata and per-file association counts/hashes in `simulation-topology-native-evidence.json`. These refine the meaning of already decoded node-list bytes; they are not an additional exclusive byte-coverage total or a global/all-installed-project percentage regrade. No installed native code, CLR assembly or COM target was executed. Other families, full stored/derived engineering enums, geometry validation and complete result-averaging semantics remain open.

All **1,598 regression tests pass** (192.102 seconds). The native/corpus verifier passes; specification lint reports zero findings (675 headings, 96 anchors).

Evidence: C-489.
