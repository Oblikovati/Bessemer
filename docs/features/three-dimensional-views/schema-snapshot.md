### 11.268 Derived 3D views and direct annotation records

*Status: Partial · four native derived views; one additional complete manager · Evidence: C-310*

The modern `mo3dView_c` body calls the individual `moView_c` serializer of 11.267 directly, then appends the suffix below. There is no second class tag around the base. The native primary vtable is **0x4CE1E140**, locator **0x4D304EB0**, Serialize **0x4C223760** in the hash-bound sldmodu.dll. The reader supports ordinary archive versions **>= 0x3B07**; earlier preview and annotation-map branches remain unsupported.

| Order | Runtime member | Modern wire encoding |
| --- | --- | --- |
| 1 | base | direct moView_c body |
| 2 | +0x3F8, +0x3FC, +0x3FC, +0x400 | four i32; both serialized +0x3FC values retained |
| 3 | +0x408, +0x410 | CString, i32 |
| 4 | +0x418, +0x450 | i32, two u32 bitset words |
| 5 | +0x468, +0x46C | two i32 |
| 6 | +0x420 vector | signed i32 count, then i32 values for positive counts |
| 7 | +0x478, +0x480 | two f64 |
| 8 | +0x488 map | u32 high count word, u32 low count word; each entry is CString key and direct annotation-data body |
| 9 | +0x498 | u32 |

Native input overwrites +0x3FC twice. The parser preserves the first and second raw values and records the second as the resulting loaded value. Helper **0x4BD3D3B0** implements the CString/i32 pair at +0x408/+0x410; the integer is not a collection count. The empty modern suffix is **73 bytes**, giving a minimum derived body of **196 bytes** with the minimal base. New field meanings remain raw and receive no semantic byte credit.

**Direct annotation data.** `mo3dViewModelToAnnoData` has primary vtable **0x4CE1E020**, locator **0x4D3050A0**, Serialize **0x4B561BD0**. Its body begins with a signed i32 key count and all i32 keys. Only after the entire key vector does it read one group per key: a high-word/low-word u64 item count followed by direct display-item records. Negative signed counts are retained and skip their loops. Positive counts are bounded by remaining bytes. Key order and duplicate keys are retained. These directly allocated map values have no MFC class tag or object-map slot.

`mo3dViewDispItemInfo` has primary vtable **0x4CE1DF90**, locator **0x4D305018**, Serialize **0x4B561AB0**. Each direct item is exactly **16 bytes**: four i32 values corresponding to +0x08, +0x0C, +0x10 and +0x14. Constructor defaults do not add wire fields. Both map and item counts combine their first word as the high 32 bits; they are not ordinary little-endian u64 reads.

**Native replay and limits.** The part `samples/tutorial/api/nist_ftc_08_asme1_rc_sw1702_ra.SLDPRT` contains four derived bodies named **MBD_A**, **MBD_B**, **MBD_C**, **MBD_D**, totaling **1,184 bytes** (298, 298, 294, 294). Their +0x408 strings are A/B/C/D, +0x410 values are 2/3/4/5, and +0x418 values are 208/210/216/220. The +0x420 vectors are [200,208,206], [210,212,214], [216,199], [220,199]. All four annotation maps are empty. Populated annotation groups are supported by native serializer evidence and synthetic tests; a populated native example remains open. Raw-field accounting leaves 456 of these body bytes unknown.

The corpus audit now records **616 complete supported managers / 303 explicit stops** among the same **919 part occurrences**, preserving all 615 previous positives. Complete streams total **2,095,353 decoded bytes**, comprising **1,042,800 known structure bytes** and **1,052,553 raw unknown bytes**. The new complete stream contributes 11,187 decoded bytes, of which 5,457 are known structures. Regrading adds **1,619 stored part bytes**, bringing cumulative CMgr credit to **394,323 bytes** and remaining stored CMgr unknown bytes to **1,471,789** of the unchanged 1,866,112-byte denominator. Total unknown part file bytes become **68,850,406**. Rounded coverage remains unchanged; no whole document is completed and installed project formats remain outside this measurement.

`read_supported` enables derived views by default. `derived_views=False` reproduces the edition 0.287 frontier; `views=False` disables both view registrations. Direct prefix readers opt in with `view3d_records=True` together with `view_records=True`. Tests bind all four native spans, preserve duplicate annotation keys, verify direct item framing and outer object back-references, reject truncation and oversized high-word counts, and reject unsupported versions before consuming bytes. All **964 regression tests pass** (101.627 seconds). Raw meanings, historical variants, fresh helper-state objects, populated native annotation maps and full part/assembly/drawing/all-installed-project semantic bit coverage remain open.

Evidence: C-310.
