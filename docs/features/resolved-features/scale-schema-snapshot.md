# Scale and schema-reference snapshot

### 11.150 Ordinary modern scale-feature suffix and coordinate-system reference

*Status: B — native-derived suffix and reference framing; raw numeric meanings and positive native optional fields remain partial.*

moScale_c is implemented in sldMoPartu.dll. Primary table `1804D9078`, locator `1804F3840`, selects serializer `180385CB0`. Its +0x4B8/+0x4C0 hooks import the common concept lightweight/resolved serializers from sldmodu.dll. Runtime descriptor `180607080` selects factory `180385960`. That factory calls the imported moBodyFeature_c constructor and inlines the scale-specific initialization, including direct suObArray +830. Allocation size 0x858 is not serialized length.

For ordinary non-DB external-XML loading with v > 0x27CB, read the complete body-feature base, then this suffix:

| # | Field | Type | Presence | Description and rule | St. |
|---|---|---|---|---|---|
| 1 | raw +80C | i32 | Always in modern scope | Preserve the complete signed value; original vector-layout gate v >= 0x489 | B |
| 2 | vector +810 | Three f64 | Always in modern scope | Direct mgVector_c body; retain all three components independently | B |
| 3 | raw +808 | i32 | Always | Preserve the signed wire value; no inferred enumeration | B |
| 4 | coordinate system +828 | Tagged moCoordSysRef_w pointer | Always in modern scope | Original gate v > 0x4D5; null, inherited reference or new object | B |
| 5 | array +830 | Direct suObArray body | Always in modern scope | Original gate v > 0x70B; ReadCount and tagged object pointers as in 11.149 | B |

The minimum suffix is **36 bytes**, with a short null coordinate-system tag and empty short-count array. A new pointed-to object or nonempty array increases that length through its ordinary object grammar. There is no i32 presence field before the coordinate-system pointer or direct array. The constructor's +800 and +850 state is not appended wire data. Array count bounds and nested reader failures remain explicit; unsupported object bodies are not skipped.

The native historical branch v < 0x489 instead reads one f64, sets +80C to 1 and replicates that scalar into the three vector components, then reads +808 and returns. It does not follow the modern suffix grammar. This legacy branch is documented from native evidence but remains outside the scoped reader. Modern offline decoding does not replace anisotropic components, validate the raw words as booleans or normalize numeric values.

moCoordSysRef_w primary table `4D155808`, locator `4D358570`, selects serializer `4C27C0C0` in sldmodu.dll. Its complete modern body is the ordinary reference-wrapper body (`4C2842D0`, 11.16), followed by one direct compact mgXform_c transform at +A8 (11.6). The transform's original load gate is v > 0x4D5, satisfied in modern scope. This is neither the surface-wrapper's four-double extension nor the plane-reference body. Wrapper archive-mode restrictions and unsupported embedded-reference contexts continue to apply. New tagged coordinate-system objects occupy normal MFC map slots; repeated pointers preserve identity. The direct transform adds no object slot.

The frontier contains **30 fresh scale features** in completed roots. Raw (+80C,+808) is (1,0) in 29 cases and (1,1) in one. Vectors are (1.5,1.5,1.5) in 27 cases, with one each of (2.5,2.5,2.5), (1.2,1.2,1.2) and (1,1,1). All native coordinate-system pointers are null and all direct arrays empty. Positive coordinate-system objects, repeated shared references, populated arrays, unequal vector components, signed raw extrema and noncanonical values are tested synthetically. A hash-bound native root regression verifies the composed inherited body and exact end; malformed counts, truncation and unsupported contexts are rejected.

Edition 0.161 records **20,398 complete roots**, **435 list-complete configurations** and **471 explicit stops** across all **906 complete modern model contexts**. **405 lists reach exact EOF**, spanning **17,168,239 decoded bytes**. The other **30 complete lists retain four-byte unknown tails**, up from six before scale traversal; list completion does not establish those tail meanings. No scale-class stop remains at this frontier. Leading dependencies are hole-wizard features (38), base bodies (22), sheet-metal features (21), extruded reference surfaces (21), sweeps (17) and blend cuts (15). The two raw scale words remain unknown, and this traversal does not add whole-document coverage credit. Full part, assembly, drawing and all installed project-format bit coverage remains incomplete.

Evidence: C-178.

### 11.264 Swift schema references in configuration archives

*Status: Partial · Complete ordinary reference body; 35 prior stops advanced · Evidence: C-306*

`moSwiftSchemaRef_c` has primary vtable **0x4D1FCD38**, locator **0x4D36D2E0**, and serializer **0x4C6E5310** in hash-bound `sldmodu.dll`. Its RTTI hierarchy includes `su_CObject`; the inherited ordinary Serialize adds no body fields. The complete ordinary body is:

| Order | Runtime member | Wire encoding |
| --- | --- | --- |
| 1 | +0x08 | ordinary su_CArchive CString |
| 2 | +0x10 | signed i32 |

The minimum body is **five bytes**, excluding its ordinary MFC object/class header. There is no presence byte, inline schema definition, extra map count or version-dependent suffix in this native serializer. Fresh/repeated class records and object back-references retain normal MFC slot identity. The reader preserves both raw values; observed names such as **Scheme2**, **Schema2**, **Scheme15** and **Scheme8** are not interpreted as paths, release identifiers or a proven enum/index mapping.

**Runtime remapping.** After reading the two fields, native input can replace the loaded +0x10 value when an archive-helper object at **+0x828** exists and condition **0x4000000** is set. A nonnegative explicit value at helper **+0x830** is used directly; otherwise the helper object's virtual +0x188 result can supply a nonnegative value. This branch reads runtime state, not additional archive bytes. The decoder retains the serialized i32 without applying that contextual remap. The remap's consumers and user-visible schema-name/value meaning remain open.

The reader is installed **locally in the CMgr archive**. It is not globally registered as a decoder for every archive context. The existing ordinary context check, unsupported configuration bodies and guarded singleton postload checks remain in force. Both the CString and the i32 are classified as raw unknown-field bytes for coverage; merely reading them does not establish their semantics.

**Native evidence.** The edition 0.282 part-manager audit contained **35 occurrences** stopped at this class. Instrumented replay retains **36 fresh reference bodies totaling 804 bytes**, each with its exact source-stream hash, body interval/bytes, object-map slot and decoded fields. The first body starts exactly at each old unsupported-class body's cursor. One targeted file contains two decoded references. UTF-16 names and signed integers compose with ordinary tagging; a synthetic object back-reference confirms that no additional slot or pointer header is invented inside the body.

All 35 prior stops advance. **12** now complete their guarded CMgr roots; **15** stop at positive condition-200 scene objects, **3** at legacy owned-manager layouts, **2** at nonempty part body states, **2** at unverified postload graph variants and **1** at a later `moView_c` dependency. The latter 23 receive no partial CMgr coverage credit.

The expanded source-bound audit retains every original 919 part occurrence: **493 complete supported managers / 426 explicit stops**. All earlier 481 complete results remain byte-for-byte identical in their summaries. Complete manager streams now total **1,503,422 decoded bytes**, comprising **745,968 bytes assigned to known structures** and **757,454 raw unknown bytes**. The twelve newly complete managers add **36,614 decoded stream bytes**; their known-structure share is **18,226 bytes**, with raw values still excluded.

**Coverage checkpoint.** Regrading the full container corpus with the same proportional stored-byte rule adds **7,471 documented part-file bytes** beyond edition 0.283. Total CMgr stored credit becomes **289,602 bytes**, and part CMgr unknown stored bytes fall to **1,576,510** out of the unchanged **1,866,112-byte** denominator. All part unknown file bytes become **68,955,127**. Rounded part coverage is **69.6% of file bytes / 63.1% excluding container and previews**. Assembly and drawing totals are unchanged. The previous gap report and coverage snapshot are retained, and the generated current gap report matches this checkpoint.

Three focused tests replay all 35 targeted native cases and their later frontiers, verify the exact 36/804 interval totals and twelve completions, and check UTF-16 text, signed extremes, empty names, truncation/context rejection, raw-byte accounting and back-reference identity. At the edition 0.284 checkpoint, the corpus-wide coverage test verified exactly **493 positive results and 745,968 known decoded bytes**. No whole document is completed. Remaining manager paths, field meanings, other part/assembly/drawing streams and all-installed-project semantic bit coverage remain open.

Validation: all **952 regression tests pass** (106.245 seconds); specification lint reports zero findings (488 headings, 96 anchors).

Evidence: C-306.

