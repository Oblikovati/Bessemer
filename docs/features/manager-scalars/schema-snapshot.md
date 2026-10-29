### 11.260 Manager material, string and sheet-metal pair maps

*Status: Partial · Persisted state located in two native configurations · Evidence: C-302*

The ordinary modern CMgr input sequence continues from 11.259's +0x5B0 map through the fields below. The retained `moConfigurationMgr_c` serializer **0x4C22C140** and native helper-state consumers **0x4C249C50 / 0x4C250750** are revalidated against the installed module. The manager field **+0x650** is the persisted source copied into archive-helper **+0x850** when model/configuration archives are prepared (11.254). Member offsets identify runtime fields, not byte offsets within the stream.

| Order | Member | Wire encoding in supported modern input |
| --- | --- | --- |
| 1 | +0x5F8 | u32 count; each entry is i32 key then tagged `moCustomDefinedMaterialProperty_c` |
| 2 | +0x610 | signed i32 outer count; each positive-count entry is i32 key, signed i32 string count, then ordinary su_CArchive CString per positive-count value |
| 3 | +0x650 | i32 persisted helper state |
| 4 | +0x640 | u32 count; each entry is i32 key then tagged `moSMAndFPFeatIdPair` |
| 5 | +0x6A8 | u32 count; each entry is ordinary su_CArchive CString key then tagged `moCustomPropertyList` |
| 6 | +0x6C8 | tagged `suObList` |
| 7 | +0x6D8 | next direct owned data body; explicit stop before that body's bytes |

The +0x5F8 material-map field is present for version > 0x120F, +0x610 for > 0x12E1, +0x650/+0x640 for > 0x1416, +0x6A8 for > 0x182B and +0x6C8 for > 0x1BD4. The helper is scoped to the already-supported modern part-configuration path, beyond the obsolete interleaved historical layouts. Those older layouts remain disabled. All collection counts above are fixed-width four-byte fields, **not MFC ReadCount**, and map bodies have no additional tagged map-object header.

**String-map normalization.** The native +0x610 input loop ignores empty strings and inserts the others into a case-sensitive CString ordered set, deduplicating them before copying to its runtime CStringArray. It retains a map value only when the signed key is positive and at least one nonempty string remains. The decoder retains every original string, empty value, duplicate, key and entry order, plus the declared counts and a `native_kept` predicate. It does not replace wire values with the normalized runtime representation. Negative signed counts retain their native positive-loop skip behavior; impossible positive counts fail bounds checks before allocation.

**Dependent value bodies.** `moCustomDefinedMaterialProperty_c` has primary vtable **0x4CEC9828**, locator **0x4D31D1B8**, and serializer **0x4B8A0400**. After its no-op `su_CObject::Serialize`, its complete ordinary body is CString **+0x08**, f64 **+0x10**, f64 **+0x18**, i32 **+0x20**, f64 **+0x28**, in that order. Minimum body length is **29 bytes**, excluding its object's tag/class header. Numeric meanings remain raw; no units or material-property interpretation are assigned.

`moSMAndFPFeatIdPair` has primary vtable **0x4D1364C0**, locator **0x4D356008**, and serializer **0x4C238450**. Its complete ordinary body is exactly **three i32 values** at +0x08,+0x0C,+0x10, totaling **12 bytes**, with no inherited wire fields or additional list header. The decoder retains raw values and ordinary MFC identity; the class name alone is not used to assign individual feature roles. Map +0x6A8 reuses the CString/property-map input helper **0x4C21DD90** already bound in 11.257.

**Native intervals.** API `SMGussetAPI.SLDPRT` supplies **2,505..2,692** (**187 bytes**), with empty +0x5F8/+0x610 maps and persisted state **1** at **2,513..2,517**. Its +0x640 map has three entries:

| Map key | Pair +0x08 | Pair +0x0C | Pair +0x10 |
| --- | --- | --- | --- |
| 101 | 101 | 35 | 45 |
| 102 | 102 | 208 | 45 |
| 103 | 103 | 298 | 45 |

It then contains property-map key **SMGussetAPI@User Generated** with a tagged empty property list, and a tagged empty +0x6C8 list. API `fpf.sldprt` supplies **2,871..2,965** (**94 bytes**), with empty +0x5F8/+0x610/+0x640 maps and persisted state **0** at **2,879..2,883**. Its property-map key is **fpf@User Generated**, again followed by the tagged empty property/list values. Combined new intervals occupy **281 bytes**. The fresh/repeated pair class headers and named property maps following +0x650 provide additional evidence for alignment beyond preceding zero counts.

`through_manager_maps=True` enables this diagnostic extension and the two local dependent readers. It continues with the same CMgr object map and stops before direct +0x6D8 data; default prefix and earlier diagnostic stops remain available. Simulation `frame.sldprt` still stops at its nonempty condition-200 part-configuration body-state list. No complete CMgr, automatic sheet-metal context propagation, completed feature root or whole-document coverage credit is assigned. Positive material values and nonempty +0x610 maps are synthetic-tested layouts but lack a native fixture here. Complete part/assembly/drawing/all-installed-project semantic coverage remains open.

Validation: all **940 regression tests pass** (117.553 seconds); specification lint reports zero findings (484 headings, 96 anchors).

Evidence: C-302.

### 11.311 Older manager material, string and property maps

*Status: Partial · 59 map endpoints; 14 research cursor-EOF observations without postload completion · Evidence: C-353*

After the continuation in 11.310, native manager Serialize 0x4C22C140 reads these gated structures. Both source versions 9000/10000 satisfy every listed gate:

| Structure | Gate | Wire layout |
| --- | --- | --- |
| material map +0x5F8 | version >0x120F | u32 count; raw i32 key and tagged material-property pointer |
| string map +0x610 | version >0x12E1 | signed i32 outer count; each raw i32 key, signed i32 string count and direct CStrings |
| raw persisted state +0x650 | version >0x1416 | i32 |
| pair map +0x640 | version >0x1416 | u32 count; raw i32 key and tagged pair pointer |
| property map +0x6A8 | version >=0x182C | u32 count; CString key and tagged custom-property-list pointer |
| list +0x6C8 | version >0x1BD4 | tagged `suObList` pointer |

Material pointers declare `moCustomDefinedMaterialProperty_c`. Native Serialize 0x4B8A0400 consumes CString +0x08, f64 +0x10/+0x18, i32 +0x20 and f64 +0x28 after the ordinary zero-byte base. Pair pointers declare `moSMAndFPFeatIdPair`; Serialize 0x4C238450 consumes three signed i32 words at +0x08/+0x0C/+0x10. Neither scalar body has an input version gate. Their application value meanings remain unknown. The scoped readers count their bytes as unknown semantic data.

The string map uses signed counts rather than packed ReadCount. Nonpositive counts consume no entries. Native input builds a collection of nonempty strings and retains it only for a positive key. `configurationoldmanagermaps.read` records `native_kept = key >0 and any(strings)` separately while retaining every source string, including empty and duplicate strings, in wire order. Counts are bounded before payload reads. Persisted +0x650 remains a raw integer; the corpus does not establish a complete state domain or justify normalizing it to a boolean.

The property map uses helper 0x4C21DD90 and the explicitly scoped older property-list decoder of 11.302. Historical relationship-manager input restricted to 0x1295 < version <0x12FE, the direct +0x690 property list at 0x17F7 < version <0x1812, and transitional property-map handling at 0x1811 < version <0x182C are excluded. The +0x6C8 list is tagged and receives normal map framing. Reader tables are temporary and restored; default modern guards and earlier research modes remain unchanged.

`configuration_old_arrays.py --manager-maps` reaches the next boundary in all **59** cases that passed 11.310. Map spans total **5,174 bytes**, including **2,644 unknown bytes** and **2,530 known structure bytes**. Material, string and pair maps are empty in these sources. Property maps contain **45 entries**, each pointing to an empty property list. All +0x6C8 pointers are non-null empty lists. Raw state +0x650 is zero in 42 cases and one in 17; these are observations, not format restrictions. Populated material/pair bodies and string maps are covered by native binding and synthetic framing tests, with no populated older native example claimed.

The probe records actual stream size and cursor separately. **Fourteen version-9000 sources reach exact cursor EOF** after this stage; **45 version-10000 sources retain further bytes**. The following owned-data gates differ: +0x6D8 is serialized only above 0x2399 and +0x6E0 only at or above 0x23A5, excluding both at version 9000. Applying the modern unconditional owned-data layout would read fields that are absent there.

These EOF observations are research cursor results. The callback-based probe has not completed manager postload, reconstructed its runtime configuration graph or passed the supported reader's completion contract. It does not prove full semantic decoding, a supported complete manager or a complete CAD document. `configuration_old_manager_maps_evidence.py` explicitly records `postload_verified=false`, zero supported completion count and zero coverage credit. No coverage ledger is regraded from these results.

The verifier revalidates native manager/material/pair/property serializers, RTTI bindings and every exact source span in `configuration-old-manager-maps-corpus.json`. Tests cover shared pointers, signed counts, duplicate/empty string retention and native-filter outcomes, raw scalar bodies, bounds, scope rejection, reader restoration and the precise cursor/stream-size equality. The six additional-configuration/view stops from 11.310 remain unchanged. Version-10000 owned data, runtime postload and supported integration, remaining older dependencies and full document/installed-project semantics remain open.

Validation: all **1,042 regression tests pass** (108.419 seconds); specification lint reports zero findings (535 headings, 96 anchors).

Evidence: C-353.
