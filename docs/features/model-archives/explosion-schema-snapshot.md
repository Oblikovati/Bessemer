# Part explosion schema reference

Edition 0.341 · 2026-10-26 · generated parts: Annex A (solidworks/swstreams.py), Annex G (solidworks/swcover.py)

### 11.117 Part resolved suffix: mass caches, exploded-view manager and maps

*Status: Native framing and 906 exact supported configuration ends; scalar meanings partial · Evidence: [C-132], [C-133], [C-134]*

After 11.116, the ordinary modern `moPart_c` resolved serializer (`4C244920`) stores i32 `+14F0`, u8 `+1420`, i32 `+14D0` and i32 `+1524`. Next is an i32 number of **direct** `moMassPropCache_c` bodies (the current writer emits at most two). If there are two, an i32 `+1630` follows. The native constructor `4BC306E0` installs the mass-cache vtable; a misleading inferred custom-property type in the part decompilation SHALL NOT determine the body schema. Direct cache objects reserve no MFC map slots.

Each cache starts with an i32 entry count. Zero ends the cache immediately. A nonzero count is followed by that many `(i32 key, i32 body-data word, f64 value)` rows, then one direct `moMassProp_c` body. Preserve duplicate rows and all raw values. In the supported version range that mass-property body has the following ordered fields; offsets identify native members, not proved physical meanings:

| Wire type | Native offsets in serialization order |
|---|---|
| 12 f64 | `20,28,30,10,08,18,38,40,48,50,60,58` |
| 5 i32 | `1C0,1C4,1C8,1CC,1E0` |
| f64 | `00` |
| 4 i32 | `134,138,13C,140` |
| 2 CStrings | `148,150` |
| i32 | `1F0` |
| 3 f64 | `C8,D0,D8` |
| 3 direct vectors, each 3 f64 | `68,80,98` |
| i32; u16 | `15C`; `128` |

The part continues with i32 `+13F8`, a tagged `moPrtExplViewManager_c`, i32 `+1690`, i32 `+1694`, and an i32 map-presence value. A nonzero map-presence value introduces two direct collections: an i32 count of `(i32 key, tagged generic object)` rows, then an i32 count of `(i32 key, u8 value)` rows. Preserve serialized order and duplicates.

`moPrtExplViewManager_c` serializes the ordinary `moFeature_c` body, a tagged `suObArray`, a tagged `moExplPrtId_c`, f64 `+390`, i32 `+398`, i32 `+39C`, and a **direct** `su_CMapWordToOb`. Its constructor `4BABE5F0` proves the embedded map identity. That map uses `su_CArchive::ReadCount` (u16, or `FFFF` followed by u32) and that many `(u16 key, tagged generic object)` rows. It is not an i32-count map and reserves no object slot. The ID object stores a CString and an i32 `+18`. The manager's referenced array can contain the exploded-view classes continued in 11.118; other new classes still require verified readers.

The next field is the sheet-metal bridge array from 11.119. After it, if the original part lightweight member `+1614` equals zero, an i32 cached-box map count follows. Each row starts with an i32 key; **only positive keys** have a following tagged `moCachedBoundingBoxData_c`. A box-data body stores i32 `+C0`, then two records, each an i32 word, an i32 presence value and, when nonzero, two direct three-f64 points. These point pairs are not the ten-f64 `mgBBox_c` serializer. Then the part stores three i32 words (`18BC,18C0,18E8`), an i32 `+1BC8` flag and, when that flag is nonzero, an i32 count of i32 body IDs. The caller must supply the original lightweight flag; it cannot infer it from the suffix.

`partresolved.py` now parses these fields in all 906 supported native configurations: **251,640 bytes**, including **180,629 bytes with unresolved semantics**. Clause 11.118 adds the populated exploded-view dependency, and 11.119 corrects a missing middle-of-suffix bridge count. All 906 parsed ends now equal both the declared resolved end and Config-N stream end. This completes framing of the supported ordinary modern part configuration records; it does not establish all field meanings, older formats, other archive contexts or complete document coverage.

### 11.118 Part exploded views, body transforms, steps and lines

*Status: Native serializers, constructors and a populated installed fixture; framing supported, scalar meanings partial · Evidence: [C-133]*

The ordinary modern `moPrtExploder_c` body starts with the scoped `moModelFeature_c` body (the ordinary feature body plus i32 `+354`), then two i32 words (`360,364`), two tagged `suObArray` pointers (`368,370`), f64 `+378`, another tagged array (`388`), one direct ten-f64 `mgBBox_c` and a direct `suObList` (`398`). Constructor `4BABE810` allocates the box and constructs the embedded list; the box has no wire presence value. These embedded values reserve no MFC slots. Preserve the raw box values without recomputing the sphere or normalizing corners.

`moBodyXform_c` stores a direct `mgXform_c` followed by a **nonnull** tagged `moCompSolidBody_c`. Use the transform flags and optional payloads from 11.6. `moCompHolder_c` uses the same serializer (`4BBB9F90`) as `moCompRef_c`; it is also an allowed component-reference target in the scoped wrapper layout. This alias is established by its primary vtable, not inferred from the name.

The common `moPrtExplodeStep_c` serializer starts with the ordinary `moFeature_c` body, then stores: a tagged `moDirectionSpec_c`; f64 `+368`; a tagged `suObArray` (`370`); two i32 words (`378,37C`); a direct three-f64 vector (`3C8`); a direct `mgXform_c` (`3E0`); and a direct `suDWordArray` (`3A0`). The DWORD array uses the packed u16/extended-u32 count and that many u32 values. Its constructor identity is verified; retain order and duplicate values. Do not serialize the unmentioned runtime members merely because they lie between these offsets.

`moBodyExplodeStep_c` appends a **nonnull** tagged `moCompSolidBody_c` (`478`) and a tagged `suObArray` (`480`). `moPrtChainExplodeStep_c` instead appends a direct `suObArray` (`478`) and i32 `+498`; that final word's historical gate is greater than `A80` hex, always true in this scoped version range. Direct arrays and lists retain the existing archive map, enabling cycles through tagged children.

`moPrtExplodeLine_c` begins with ordinary node lightweight fields. Its node serializer dispatches virtual slots `4B8` and `4C0`; the line's primary vtable selects `4C18A1A0` (node lightweight) and `4B29DAE0` (no-byte return), respectively. Then it stores a **nonnull** tagged explode-step pointer (`40`), two direct points (`48,60`, three f64 each), two f64 values (`78,80`), three i32 words (`88,8C,90`), and two nullable tagged `moAttachPoint_c` pointers (`A0,A8`). New attachment bodies remain unsupported. This scope is above the old attachment gate `2E1` hex.

The installed `Carving Knife/Knife Handle-DEMO.SLDPRT`, `Contents/Config-0`, supplies a populated manager containing one exploder, two body transforms, two body explode steps and two explode lines. Each step's line points back to that same already-mapped step. Native reparsing verifies these identities and the exact 5,841-byte part suffix (after the 11.119 correction) with its source/body hashes. Together with the other 905 cases, the audit now reports 906 parsed prefixes and no dependency errors. Edition 0.118 still observed a four-byte mismatch in every case; 11.119 identifies and corrects the omitted bridge field. Bit semantics remain partial.

