# Owned manager context schema snapshot

### 11.254 Configuration-manager prefix and template sheet-metal context

*Status: B / ? — native configuration-manager prefix, template serializer alias and helper-state source verified; configuration bodies and automatic persisted-state selection remain incomplete.*

`Contents/CMgr` is an independent MFC archive rooted at `moConfigurationMgr_c`. Primary vtable `0x4CEC7AF8`, locator `0x4D31D0E0`, selects serializer `0x4C22C140` in hash-bound installed `sldmodu.dll`. The supported modern path calls `moFeature_c::Serialize`, then reads tagged configuration pointers associated with runtime members `+0x358,+0x360`. Configuration bodies must be decoded before later manager fields, including the sheet-metal archive state described below. The prefix reader uses a fresh null-at-zero object map for CMgr; it does not import a model configuration's cached map or read a model lightweight-section length before the feature prefix.

Descriptor `0x4DAEB4B8` binds factory `0x4C21C1A0` to default constructor `0x4B896990`. The constructor calls `moFeature_c(...,0)`. Revalidated feature constructor `0x4BB4BE10` forwards that zero argument to concept constructor `0x4BDB67C0`, which initializes child-list pointer `+0x58` to null and allocates a list only for a nonzero argument. CMgr's lightweight/resolved virtual slots select concept serializers `0x4C189B60` and `0x4C18A920`. The resolved concept serializer emits a direct child list only when that pointer exists. Therefore CMgr uses the established feature grammar **without the direct feature-list body/count**. Treating it as an ordinary folder inserts an incorrect two-byte count and misaligns the following fields. Constructor allocation 0x8C8 is runtime size, not stream length.

Three installed CMgr streams verify this distinction: Simulation `frame.sldprt` (**21,829 bytes**), API `SMGussetAPI.SLDPRT` (**2,758 bytes**) and API `fpf.sldprt` (**3,031 bytes**). Each begins with a 26-byte root class/object declaration, followed by a **139-byte inherited feature prefix**, ending at stream offset **165**. Each first configuration pointer declares `moPartConfiguration_c` at **0xA5** and stops before its body, with diagnostic cursor **192** after the class declaration. `configuration-manager-prefix-corpus.json` retains full stream and prefix SHA-256 values. The missing configuration body is an explicit dependency; no complete manager object or stream is claimed.

The manager serializer reads i32 runtime member **`+0x650`** for versions **above `0x1416`**, followed by a u32 map count and its entries. It is not a constant stream-tail word and cannot safely be obtained by indexing backwards from EOF. Native model functions **`0x4C249C50`** and **`0x4C250750`** obtain the model's configuration manager and copy this member into **archive-helper `+0x850`** when preparing model/configuration archives. This binds the conditional sheet-metal layouts in 11.154 and later clauses to persisted configuration-manager state. A freshly constructed archive helper's default zero alone does not prove the saved configuration's state.

`moTemplateSheetMetal_c` has primary vtable `0x3DC665D0`, locator `0x3DC77498` and serializer entry **`0x3DB6B1D0`** in hash-bound installed `sldsmmu.dll`. Its exact five-byte leaf **`E9 0B E1 FF FF`** jumps to established `moSheetMetal_c` serializer **`0x3DB692E0`**. The leaf has no unwind-table entry; the target is resolved from its instruction bytes rather than inferred from a nearby function range. It adds no derived wire payload. All inherited common-bend, dimension, fixed-face and helper-state conditions remain those of the target serializer.

Applying default helper state 0 to the two template cases misaligns the first derived display-dimension tag. An explicitly selected **helper-state-1 experiment** completes template roots of **8,291 bytes** in `frame.sldprt` and **8,764 bytes** in `SMGussetAPI.SLDPRT`; their raw stored `+0x8F0` binary64 values are approximately **0.003**. Both experiments advance to `moCompSheetmetalFeat_c`. This supports the state dependency and native alias, but does **not** prove that the persisted CMgr member has been decoded. `template-sheet-metal-state-probe.json` labels its assumed helper state explicitly. These experimental roots are excluded from the ordinary corpus completion totals and receive no whole-document coverage credit.

The production template reader now requires explicitly verified sheet-metal context before consuming its body. It does not infer state 1 from the class name, a plausible double, a successful object tag or a file-version threshold. The inherited-map resolved-stream handoff carries both the chosen state and the explicit context-verification marker. Since the CMgr prefix still stops before its configurations, the ordinary audit retains both template cases as **persisted configuration-manager context required** stops, before any template body bytes are accepted. The scalar state and its complete CMgr continuation remain open work.

The frozen 0.273/0.274 audit preserves every complete root record across 906 contexts. Ordinary totals remain **25,521 complete roots**, **828 complete lists** and **78 explicit stops**, including the two now-explicit configuration-manager context requirements. Exact-EOF totals remain **750 lists / 62,819,665 decoded bytes**, plus **78 complete lists with four-byte unknown tails**. Neither the three CMgr prefixes nor the mode-1 experiments are counted as completed whole streams.

Three focused tests verify the three source-bound CMgr prefixes and exact configuration-body stops, rejection before template body consumption without explicit context, and both mode-1 experimental root hashes with their assumption retained. The established sheet-metal tests continue to cover both conditional payload grammars. The historical split regression asserts the new explicit context requirement and its exact diagnostic cursor at the old template stop. Full part/assembly/drawing/all-installed-project semantic bit coverage remains incomplete; this increment provides native state provenance and a new bounded stream prefix, not complete manager or template-document coverage.

Validation: all **924 regression tests pass** (86.196 seconds); specification lint reports zero findings (478 headings, 96 anchors).

Evidence: C-296.

### 11.255 Part-configuration base fields, mass properties and collections

*Status: B / ? — part/base native bindings, constructor-owned containers and three populated base prefixes verified; common maps, derived part suffix and complete CMgr state remain partial.*

`moPartConfiguration_c` selects serializer **`0x4C235B40`** through primary vtable `0x4CE2DE88`, locator `0x4D3066A8`, in hash-bound installed `sldmodu.dll`. Its native ancestry includes `moBaseConfiguration_c`, whose serializer is **`0x4C225030`**, primary vtable `0x4D0287D8`, locator `0x4D341418`. The part serializer calls the base serializer before its derived fields. This clause establishes the base prefix through the next common-map boundary; the diagnostic reader does not return either class as a complete configuration.

Part descriptor `0x4DAEBAE8` binds factory `0x4C21CAD0` to constructor `0x4B5B25D0`; base descriptor `0x4DAEB848` binds factory `0x4C21C770` to constructor `0x4BDCEF90`. The part constructor calls the base constructor. The base constructor calls `moFeature_c(...,0)`, giving the same absent concept child-list rule established for CMgr in 11.254. Both classes' lightweight/resolved virtual slots select concept serializers `0x4C189B60` and `0x4C18A920`. Thus their inherited feature payload omits a direct child-list count at concept `+0x58`; later configuration children use a distinct direct list at `+0x6A0`. The 0xDD8/0xC40 allocations are runtime sizes, not wire sizes.

After this inherited feature body, the supported ordinary modern base prefix stores:

1. CString `+0x4A0`, i32 `+0x4A8`, **u32** flags `+0x4BC`, then CString `+0x490`.
2. u32 values `+0x854,+0x858,+0x668`, in that order.
3. One **direct `moMassProp_c` body** owned through pointer `+0x660`, using the established mass-properties layout in 11.119. This has no tagged mass-property object header or separate map slot. Constructor `0x4BDCEF90` allocates the owned mass-property object.
4. One u32 archive time value associated with `su_CTime` member `+0x670`. Its in-memory representation does not determine its wire width.
5. Tagged parent-configuration pointer `+0x698`, then direct `suObList` children `+0x6A0` (ReadCount and tagged configuration pointers).
6. i32 `+0x6F0`, u8 `+0x680`, then u8 `+0x960`.
7. Direct `suObList` `+0x520`, u32 `+0x9A0`, then u8 `+0x964`.
8. One u8 presence byte for owned optional data `+0x608`. Nonzero introduces a direct virtual data body, not a tagged pointer. That body's schema remains unimplemented and stops explicitly after the presence byte. Zero introduces no placeholder.
9. Tagged `moToolboxPartSpecArray_c` pointer `+0x658`, direct `suObList` `+0xA90`, then i32 values `+0xAE8,+0xAEC`.
10. Signed i32 object-map count for runtime map `+0x610`; positive-count entries each contain two u32 key words followed by one tagged object pointer. The pair forms the native 64-bit key. The reader retains both original words, wire order and duplicates rather than simulating map overwrites.
11. i32 values `+0x77C,+0x780`, then **binary32** values `+0x784,+0x788`.
12. Two independent typed pointer arrays `+0x350,+0x378`, respectively containing single-face references and reference wrappers.
13. i32 `+0xB28`, then eight i32 values `+0x820,+0x824,+0x828,+0x82C,+0x830,+0x834,+0x838,+0x83C`.
14. A third typed reference-wrapper pointer array at `+0x3C8`.
15. i32 values `+0x748,+0xBCC`, followed by i32 `+0x848` only for versions **above `0x2F72`**.

Each typed pointer array uses a **signed i32 count** and tagged entries; it does not use ReadCount and does not have a tagged container header. The native positive-count loops do not iterate for negative values. The reader retains the declared signed count and rejects positive counts impossible within the remaining bytes, without declaring negative values valid engineering states. The object map follows the same positive-count loop rule. Shared pointers and duplicate entries retain identity. Constructor bindings independently identify the direct lists at `+0x520,+0x6A0,+0xA90` and typed arrays at `+0x350,+0x378,+0x3C8`.

All historical field gates before the final `+0x848` word precede the supported range. In particular, obsolete owner/bounding-box branches below `0x3B3`, old visual/relation maps below `0xE68`, optional data `+0x600` at or below `0x2399`, and scene data `+0x648` below `0x23A5` add no bytes to this modern prefix. The newer optional `+0x608` presence byte remains present. Its nonzero body, archive condition changes, common maps beginning at `+0x418` and the subsequent derived part-configuration payload remain explicit dependencies. No search for a plausible map count or following configuration tag is used to skip them.

The three CMgr fixtures from 11.254 now decode their first part configuration from stream body offset **192**. Simulation `frame.sldprt` reaches offset **1,662**, a **1,470-byte prefix**; API `SMGussetAPI.SLDPRT` and `fpf.sldprt` each reach **1,596**, giving **1,404-byte prefixes**. Combined verified prefix spans total **4,278 bytes**. Their inherited feature bodies end at **1,266 / 1,200 / 1,200**, and their direct mass-property bodies end at **1,548 / 1,482 / 1,482**. All three then pass empty child/list/map/typed-array collections, zero optional-data presence, and the native raw scalar sequence before stopping at the next common-map boundary. The flags `+0x4BC` are **2,151,940,480 / 2,151,678,336 / 2,151,940,480**, preserving their unsigned high bits; both binary32 fields are **-1.0** in all three samples.

`part-configuration-prefix-corpus.json` retains source-stream and prefix SHA-256 values, exact stage/end offsets, raw scalars/strings, mass-property boundary, time values and every decoded collection count. Opt-in `through_configuration_prefix=True` adds these bounded readers only to the CMgr archive instance. The original 11.254 prefix mode still stops at the first configuration class declaration, preserving its historical behavior. No configuration is returned complete and no persisted CMgr `+0x650` value is inferred from the partial configuration data. Automatic sheet-metal-state selection and the template-context requirement remain open.

The independent feature-list audit is byte-for-byte unchanged between frozen checkpoints 0.274 and 0.275: **25,521 complete roots**, **828 complete lists**, **78 explicit stops**, **750 exact-EOF lists / 62,819,665 decoded bytes**, and **78 complete lists with four-byte unknown tails**, across 906 contexts. No CMgr prefix is added to these completed feature-list totals.

Two focused tests verify signed pointer counts, shared identity, impossible-count/context rejection, and all three source-bound native prefix values and exact next-map stops. Full CMgr streams, assembly/drawing configuration variants, scalar semantics and complete part/assembly/drawing/all-installed-project bit coverage remain incomplete. Prefix framing earns no whole-document semantic coverage credit.

Validation: all **926 regression tests pass** (100.399 seconds); specification lint reports zero findings (479 headings, 96 anchors).

Evidence: C-297.

### 11.256 Configuration common maps and native pair filtering

*Status: B / ? — native common-map sequence and three populated map intervals verified; subsequent map `+0xA48`, remaining configuration bodies and CMgr state remain partial.*

This continuation begins at the `+0x418` common-map boundary retained in 11.255. It follows the ordinary modern base-configuration fields under native serializer **`0x4C225030`**. A runtime list at `+0x4D8` is serialized here only below version `0xCA0` or when archive condition 1 is set. It adds no count or placeholder to this supported modern condition-clear path. The subsequent maps are distinct collections with independent counts; none has an enclosing tagged map object header.

The wire sequence is:

1. **Visual map `+0x418`: u32 count**, then that many entries containing one i32 key and one tagged visual-properties object pointer. All stored entries and pointer identities are retained. This count is not ReadCount.
2. **Nested map `+0x428`: signed i32 outer count**. Each positive-count entry contains one i32 outer key, then a **u32 inner count**, followed by that many pairs of i32 values. The inner pairs are retained in wire order, including duplicate pairs and duplicate outer keys.
3. **Integer map owned through `+0xAF8`, member `+0x08`**: the shared helper layout below.
4. One i32 value at runtime `+0xB00`.
5. **Filtered integer map `+0x458`: signed i32 count**, followed by that many positive-count pairs of i32 values. Native retention semantics are described below; the diagnostic reader keeps all original pairs.
6. **Integer map `+0xB08`**: the same shared helper layout.

Shared input helper **`0x4C21D810`** reads a four-byte count through the archive `long` overload, then two four-byte signed words per entry. Its loop compares against the count's **uint32 interpretation**; a high-bit count does not mean an empty collection. The decoder therefore reads this count as u32 and rejects counts impossible within the remaining bytes. Output helper **`0x4C21E0A0`** writes the count and corresponding pairs. Each entry is exactly **eight bytes**, with no object tags or pair-length fields. The native helpers populate integer maps; the diagnostic representation keeps the original ordered pairs and duplicates rather than reproducing insertion/overwrite behavior.

The signed outer count of `+0x428` and signed count of `+0x458` use native `count > 0` loops. Zero or negative values introduce no entries; their declared signed values remain available. This differs from the unsigned inner/helper counts and does not declare negative counts valid engineering states. Bounds checks precede iteration. With every map empty, the entire continuation occupies **24 bytes**: five independent four-byte counts and the four-byte `+0xB00` word.

Native loading into `+0x458` calls `suIntMap::SetAt` only when **signed key > 100 and signed value > 100**. A value equal to 100, negative, or carrying high bits that produce a negative signed value does not pass that predicate. The decoder records separate `native_kept` metadata while retaining every wire pair. This metadata adds no bytes and does not change stored values. Duplicate keys, order and entries rejected by native loading remain reviewable. Scalar/map domains beyond this explicit predicate remain unresolved.

All map-presence gates precede the supported version range: visual map above `0xF9A`; nested map above `0x1079`; first shared integer map above `0x10F1`; `+0xB00` above `0x10F7`; filtered map and final shared integer map above `0x1108`. None is optional within the ordinary modern path used here. The next native call, input helper `0x4C21D970`, serializes a separate map at `+0xA48`. Its schema and following configuration continuation remain unimplemented; the reader stops immediately before that map without treating any of its bytes as a trailer.

In Simulation `frame.sldprt`, the map interval is **1,662..2,182**, totaling **520 bytes**. The first shared integer map has **33 pairs** and the filtered map **29**. In API `SMGussetAPI.SLDPRT`, the interval is **1,596..1,660** (**64 bytes**), containing **3** and **2 pairs** respectively. In API `fpf.sldprt`, it is **1,596..1,652** (**56 bytes**), containing **3** and **1 pair**. Combined intervals occupy **640 bytes**, with **39 first-map pairs** and **32 filtered-map pairs**. All 32 native filtered pairs satisfy the retention predicate. The visual, nested and final integer maps are empty, and `+0xB00` is zero, in all three native cases.

`configuration-common-maps-corpus.json` retains the source-stream and exact interval SHA-256 values, every pair/count, raw word and native retention result. Opt-in `through_common_maps=True` composes this continuation after the earlier CMgr/configuration prefixes. Earlier prefix modes retain their stops. Three focused tests verify signed versus unsigned counts, empty minimum size, nested/duplicate pairs, shared visual-pointer identity, filter boundary cases, truncation/context rejection and all three native intervals. Populated visual/nested/final integer maps have instruction-derived synthetic tests; no populated native fixture for those three map kinds is claimed.

The independent feature-list audit is identical between frozen checkpoints 0.275 and 0.276: **25,521 complete roots**, **828 complete lists**, **78 explicit stops**, **750 exact-EOF lists / 62,819,665 decoded bytes**, and **78 complete lists with four-byte unknown tails**, across 906 contexts. No partial CMgr map interval is added to these totals.

The new stop is map `+0xA48`; no complete base/part configuration or CMgr root is returned. Persisted CMgr `+0x650`, automatic sheet-metal context selection, subsequent configuration variants and complete part/assembly/drawing/all-installed-project semantic bit coverage remain open. These prefixes and successful map framing receive no whole-document coverage credit.

Validation: all **929 regression tests pass** (100.842 seconds); specification lint reports zero findings (480 headings, 96 anchors).

Evidence: C-298.

### 11.257 Late configuration maps and linked-atom graphs

*Status: B / ? — late map layouts, recursive linked-atom bodies and three native intervals verified through supported base fields; derived part-configuration and complete CMgr bodies remain partial.*

After the common maps in 11.256, native base serializer `0x4C225030` stores the following ordinary modern sequence:

1. **Map `+0xA48`**, using input helper `0x4C21D970` and output helper `0x4C21E160`: u32 count, then pairs of **i32 key and u32 value**. These differ in signedness from the earlier integer/integer helper. Each pair occupies eight bytes. The count is read through a four-byte `long` overload but controls an unsigned loop; impossible high-bit counts are rejected rather than treated as empty.
2. **Direct string/integer map `+0x5F8`**, using `su_CMapStringToInt::Serialize`, with the MFC encoding below.
3. i32 `+0xB2C`.
4. Signed i32 direct-visual-map count `+0xB50`; positive entries contain an i32 key followed by a direct `moVisualProperties_c::restore` body. These are not tagged visual pointers. Positive native bodies are not yet decoded by this continuation and stop explicitly after the count. Zero/negative native positive-loop counts introduce no entries; raw count values are retained.
5. **Direct owned `moCustomPropertyList` body** at `+0xB60`, using the established property-list layout. It adds no tagged property-list object header.
6. Tagged abbreviated-view manager `+0xB78`, then i32 `+0x744`.
7. Signed i32 set count at `+0xA68`, followed by that many positive-count **u32 values**. Values, order and duplicates are preserved rather than reduced to the native set.
8. **Linked-atom map `+0x448`**: u32 count, then entries of i32 key and tagged `moLinkedAtomIdNode_c` pointer, including normal map references.
9. i32 presence for direct owned 3D-print settings `+0xB80`. Any nonzero value introduces a direct settings body; that body remains an explicit unsupported branch. Zero adds no placeholder.
10. **String/property-list map `+0xB68`**, input helper `0x4C21DD90`, output helper `0x4C21E460`: u32 count, then ordinary su_CArchive CString key and tagged `moCustomPropertyList` pointer per entry.
11. i32 `+0xBC8`, then integer/integer map `+0xB90` using the established helper.
12. i32 `+0x4C0` above version `0x2B7D`, i32 `+0x4C4` above `0x3721`, i32 `+0xA78` above `0x2BDA`, then tagged envelope publisher `+0xBD0` above `0x2F53`, in that wire order.
13. For versions `>=0x41A3`, a u32 count followed by that many i32 values, associated with native vector `+0xBF0`. Input helper `0x4BB17E10` proves the four-byte count/element sequence. Its unsigned count interpretation is preserved and bounded against remaining bytes.

Native loading sets runtime `+0xBC8` to zero when earlier configuration word `+0x77C` equals 1. The diagnostic reader retains `raw_bc8` separately from `loaded_bc8`, preserving the stored bytes. The archive-condition-1 path also serializes i32 `+0x6E8` before the final vector; condition 1 is clear in this ordinary reader's scope. Other raw scalar and map domains remain unresolved.

The earlier presence gates all precede the supported modern range: map `+0xA48` above `0x1193`, map `+0x5F8` above `0x12DE`, word `+0xB2C` above `0x142E`, direct visual map above `0x144B`, custom properties above `0x17F7`, abbreviated views above `0x1BD0`, set above `0x1C2B`, linked-atom map above `0x1FC0`, print settings above `0x1FF1`, string/property map above `0x1FFB`, word `+0xBC8` above `0x23DD`, and map `+0xB90` above `0x2416`. Later gates in items 12–13 remain conditional within supported modern files.

**MFC string/integer map.** In hash-bound `swccu.dll`, named export `?Serialize@su_CMapStringToInt@@QEAAXAEAVsu_CArchive@@@Z` resolves to **`0x31D376C0`**. Its exact 11-byte wrapper `F6 42 20 10 0F 84 76 FE FF FF C3` tests archive mode bit `0x10`: clear branches to **`0x31D37540`**, set returns without I/O. Within the implementation, load mode bit 1 selects input. This decoder covers the ordinary load path with the wrapper's bit `0x10` clear; the suppressing mode is outside scope.

Integer-element helper `0x31D31720..0x31D317FD` spans six adjacent unwind chunks. Its retained read/write loops multiply the element count by four and advance the data pointer by that width; a one-element transfer is exactly four bytes. The property-map output helper `0x4C21E460..0x4C21E519` is also retained across all three of its chunks.

The input path calls **MFC `CArchive::ReadCount` with 64-bit result** (ordinal 12087), then for each entry serializes one CString using the MFC string-element helper (ordinal 261), followed by one four-byte signed integer. Output uses `WriteCount` (ordinal 14278). Ordinal EAT entries, import thunks and module hashes are retained, rather than relying on a truncated export listing. Count widths are **2 / 6 / 14 bytes** with the established MFC escape sequence; string lengths also retain the MFC 64-bit escape behavior and UTF-16 marker. This differs from the fixed u32 count and ordinary su_CArchive CString used by map `+0xB68`. The reader keeps duplicate keys and entry order instead of overwriting them as a native map would. It adds no map-object tag or separate object-map slot.

**Linked-atom value.** `moLinkedAtomIdNode_c` has primary vtable `0x4CE24D48`, locator `0x4D305B80`, and serializer **`0x4B586400`** in hash-bound `sldmodu.dll`. Its complete ordinary body is i32 `+0x08`, direct `su_CDWordArray` `+0x90`, direct `su_CDWordArray` `+0xD0`, then **four independent linked-atom maps** at `+0xF0,+0x100,+0x110,+0x120`, in that order. Each auxiliary map uses input helper **`0x4B585DE0`** and output helper **`0x4B586BC0`**: u32 count, then i32 key and tagged linked-atom pointer per entry. The DWORD arrays retain their established ReadCount/u32-entry layout and add no array-object tags. With empty arrays/maps, a node body occupies **24 bytes**, excluding its own tagged object/class header.

Native output builds the two DWORD arrays from runtime component data before serializing them. Native input restores those arrays and the four pointer maps; those runtime source lists are not additional wire bodies. Stored DWORD meanings remain raw. Because nodes are mapped before their fields are read, shared references and back-references use normal MFC identity. The diagnostic snapshot represents reachable nodes once and records references by node index; it does not flatten repeated nodes, recursively expand cycles or invent forward slots. A synthetic self-reference verifies this composition independently of the native fixtures. DB archive dispatch is outside this ordinary reader's scope.

In Simulation `frame.sldprt`, the new interval is **2,182..7,854**, totaling **5,672 bytes**. API `SMGussetAPI.SLDPRT` supplies **1,660..1,928** (**268 bytes**), and `fpf.sldprt` supplies **1,652..1,892** (**240 bytes**). Combined verified intervals occupy **6,180 bytes**. Their linked-atom maps reference **33 / 3 / 3 distinct nodes** and **700 / 10 / 4 auxiliary references**, totaling **39 nodes and 714 auxiliary references**. DWORD-array entry totals are **39 at +0x90** and **11 at +0xD0**. All other new maps/lists and the final vectors are empty in these three fixtures; optional visual and print-setting bodies are absent. No populated native string/integer or string/property-map fixture is claimed.

`configuration-late-maps-corpus.json` retains source-stream and interval SHA-256 values, all late fields, ordered DWORD values, graph topology and exact end cursors. `through_late_maps=True` composes this continuation only in the CMgr archive instance. Earlier prefix modes retain their stops. The new stop is the **derived part-configuration suffix**, after the supported base fields. No part/base configuration or CMgr root is returned as complete, and the manager's persisted `+0x650` state remains undecoded. The template context requirement therefore remains in effect.

The ordinary feature-list audit is identical between frozen checkpoints 0.276 and 0.277: **25,521 complete roots**, **828 complete lists**, **78 explicit stops**, **750 exact-EOF lists / 62,819,665 decoded bytes**, and **78 complete lists with four-byte unknown tails**, across 906 contexts. The opt-in CMgr intervals do not add completed configurations or whole-document coverage.

Three focused tests verify signed/unsigned value extremes, 64-bit MFC count escapes and UTF-16 duplicate string keys, bounds/context/truncation rejection, a cyclic linked-atom self-reference, and all three native intervals with their exact graph identities and field snapshots. Positive direct visual/settings bodies, derived part/assembly/drawing configuration variants, engineering domains and complete part/assembly/drawing/all-installed-project semantic bit coverage remain open. No whole-document coverage credit is assigned.

Validation: all **932 regression tests pass** (103.562 seconds); specification lint reports zero findings (481 headings, 96 anchors).

Evidence: C-299.

### 11.258 Ordinary part-configuration derived suffix

*Status: Partial · Corrected direct-member boundary and two supported native endpoints · Evidence: C-300, C-301*

The installed `moPartConfiguration_c` serializer at **0x4C235B40** calls the base serializer from 11.255–11.257, then reads the following derived fields. The class binding, complete retained serializer bytes and decompilation are revalidated against hash-bound `sldmodu.dll`. This clause covers the modern ordinary archive with **condition 1 absent**. Runtime member offsets identify fields; they are not wire offsets. Existing scoped tagged-object identity continues across the base/suffix boundary.

| Order | Runtime member | Wire encoding and input version gate |
| --- | --- | --- |
| 0 | +0x4C8 | **Direct owned `moPartConfigObject_c` body**, without a tagged object header; version > 0xBC; see 11.259 |
| 1 | +0xC48, +0xC40 | f64 followed by tagged `gcXhatch_c`; version > 0x9FE |
| 2 | +0xC58 | i32; version > 0xA56 |
| 3 | +0xCA0 body-state entries | signed i32 count; positive-count loop of tagged `moCompAtomBody_c`, i32 state, tagged `moVisualProperties_c`; version > 0xC51 |
| 4 | +0xCB0, +0xCB8 | tagged `moExplPrtId_c`, tagged `suObArray`; version > 0x12A0, condition 1 absent |
| 5 | +0xCC0 | u32 checksum; version > 0x12CC, condition 1 absent |
| 6 | +0xCC4 | i32; version > 0x141D |
| 7 | +0xCE0 | signed i32 count, then i32 key and u32 value per positive-count entry; version > 0x1425 |
| 8 | +0xD18 | fixed u32 count and i32/i32 pairs using the integer-map helpers from 11.256; version > 0x144D |
| 9 | +0xC5C, +0xCCC, +0xCC8 | one i32 each, respectively version > 0x1478, > 0x240B and > 0x2BB1 |
| 10 | +0xDC0 set | u32 high count word, u32 low count word, then count signed i32 values; version > 0x2BDA |

The direct +0x4C8 body is allocated by the base configuration constructor but serialized by the derived part serializer. It precedes the double. **Edition 0.278 omitted this call and incorrectly interpreted 58 bytes of the owned body as a complete derived suffix, including a false NaN value. Those alignment and endpoint claims are withdrawn.** The raw fixture bytes and native serializer were correct evidence; the omitted call invalidated the field interpretation. The corrected decoder reads the owned body before accessing +0xC48 and verifies continuation into the manager in two fixtures.

The body-state loop temporarily adds archive condition **0x200**, then removes it. The earlier prefix mode retains its diagnostic stop before the first positive-count entry; the complete-reader extension is specified in 11.266. Nonnull crosshatch and exploded-part pointers remain subject to their scoped readers. Version 0x12CC has an additional legacy conversion and is explicitly unsupported here. Older inherited configuration layouts and condition-1 paths are outside this reader's modern context.

Native input retains a +0xCE0 pair only when **signed key > 100 and unsigned value < 2**. The decoder preserves all raw pairs, their order and duplicates, and records the native retention predicate separately. The +0xDC0 count is **(high << 32) | low**, in that word order, rather than a little-endian contiguous u64 or MFC ReadCount. Values remain ordered raw entries even if the native set would deduplicate them. Count bounds reject impossible input before allocation. The C48 double retains its exact eight-byte encoding alongside the decoded value.

In Simulation `frame.sldprt`, the owned body is **7,854..7,908**, followed by double **-1.0** through 7,916. The next tagged pointer introduces fresh class **`gcXhatch_c`**. The existing ordinary hatch reader is explicitly bound in the diagnostic: name **ISO (Steel)**, angle 0, scale 1, override/solid 0, identifier 55, layer -2, command-layer color -1 and randomize-scale 0. The next i32 is +0xC58 = 1. The body-state count is **9**, and decoding stops before its first entry at **8,006**. In API `SMGussetAPI.SLDPRT`, the supported derived suffix is **1,928..2,088** (**160 bytes**); API `fpf.sldprt` supplies **1,892..2,004** (**112 bytes**). Both contain the same 54-byte owned body and double -1.0. Their later +0xCE0 maps have three/zero entries, +0xD18 maps have three/zero entries, and the remaining supported pointer dependencies are null. The corrected diagnostic intervals total **424 decoded bytes**. Hatch reader registration is explicit so that the boundary does not depend on which other modules or tests were imported first.

The native serializer performs a separate named StockBodies save operation afterward when runtime stock bodies exist; that operation is not an inline CMgr suffix body. `through_part_suffix=True` remains a bounded diagnostic stop. `through_manager_fields=True` accepts only its supported native endpoint and resumes the manager with the same object map; it does not register an unrestricted global configuration reader. No whole-document coverage credit is assigned. Persisted CMgr helper state and complete part/assembly/drawing/all-installed-project semantic coverage remain open.

Evidence: C-300, C-301.

### 11.259 Owned part-configuration object and manager continuation

*Status: Partial · Three direct 54-byte bodies and two manager intervals · Evidence: C-301*

**Owned object binding.** `moPartConfigObject_c` has primary vtable **0x4CEC9EF8**, constructor **0x4BDD3630** and serializer **0x4C234990** in hash-bound `sldmodu.dll`. `moBaseConfiguration_c` allocates 0x110 runtime bytes, invokes that constructor and stores its pointer at **+0x4C8**. `moPartConfiguration_c` invokes its Serialize slot directly after the base serializer, before +0xC48. The constructor creates a direct `suObList` at +0x10. The owned object itself has **no class tag and consumes no MFC object-map slot**; tagged values inside it continue to use the containing map.

For the supported modern layout, the body is:

| Order | Member | Wire encoding |
| --- | --- | --- |
| 1 | +0x0C, +0x08 | two i32 values, in this order |
| 2 | +0xB0, +0xB8 | tagged `moReferencedConfiguration_c`, tagged `moVisualProperties_c` |
| 3 | +0x10,+0x60,+0x100,+0x80,+0x68,+0x58,+0x78 | seven direct `suObList` bodies in this order |
| 4 | +0xC8,+0xD0,+0xE0,+0xE8,+0xF0 | u16 presence per member, immediately followed by its direct body when nonzero |
| 5 | +0x70 | direct `suObList` |
| 6 | +0xD8,+0xF8 | two i32 values |
| 7 | +0x98 | u16 presence followed by direct `suObList` when nonzero |
| 8 | +0xA0,+0x88 | tagged `moPatternItemId_c`, tagged `suObList` |
| 9 | +0xA8 | tagged object, version > 0x4972 |

The +0xC8/+0xD0 optional bodies are `moBodyChooser_c`, +0xE0 is `mConfigScaleFactor`, +0xE8 is `mConfigCosmeticThread`, and +0xF0 is `moConfigLimitMate`. These five nonzero direct bodies are explicit unsupported stops. Lists +0x58/+0x78 and the five optional bodies run under archive condition 0x200; their nonempty/dependent layouts are not inferred from ordinary object decoding. Other direct lists retain their own counts, without array-object tags. Native input can discard empty temporary list allocations; this does not erase their wire counts.

The current helper requires version > 0x2B8A and handles +0xA8's later gate explicitly. Native historical gates for the earlier list/presence fields remain visible in the retained serializer, but those older layouts are not enabled. All three native bodies are **54 bytes** at version 0x4A38: integers 1/-1, null tagged pointers, empty direct lists and zero presences. The observed bodies total **162 bytes**. A synthetic test verifies that parsing them leaves the MFC map size unchanged, rejects truncation and stops before a nonempty condition-200 list.

**Manager continuation.** The CMgr constructor binds +0x368 to a direct **`suObList`**, correcting the earlier generic description as an array. After tagged configurations +0x358/+0x360, its modern input sequence continues:

| Order | Member | Wire encoding |
| --- | --- | --- |
| 1 | +0x404 | i32 |
| 2 | +0x368 | direct `suObList` |
| 3 | external model sequence | signed i32 count, then tagged `moExtObject_c` for each positive-count entry |
| 4 | +0x554,+0x570 | i32, then ordinary su_CArchive CString |
| 5 | +0x588 | signed i32 count; positive-count value bodies remain unsupported |
| 6 | +0x5A8 | tagged `suObList` |
| 7 | +0x5B0 | signed i32 count; each positive-count entry is i32 then u32 then u32 |
| 8 | +0x5F8 | next material-map stage; explicit stop before its count |

Input shares external models with the runtime reference manager; that sharing is not additional wire data. For +0x5B0, native input invokes its insertion helper only when the first signed word is positive. All raw triples remain available in the decoder. The two unsigned words are retained in serialized order without assigning unproved time or identifier semantics.

API `SMGussetAPI.SLDPRT` reaches manager interval **2,090..2,505** (**415 bytes**), and `fpf.sldprt` reaches **2,006..2,871** (**865 bytes**), totaling **1,280 bytes**. Both configurations +0x358/+0x360 reference the same mapped object; their direct +0x368 lists also contain that configuration reference. External-model counts are **1/5**, +0x588 counts are zero, and +0x5B0 supplies **4/9 triples**. The nonnull +0x5A8 list and nested external-model class headers provide continuation evidence beyond an all-zero suffix. Simulation `frame.sldprt` remains stopped at its nine condition-dependent body-state entries before the manager interval.

This diagnostic does not decode +0x5F8 or the later persisted +0x650 field, and supplies no automatic sheet-metal context or whole-document coverage credit. The default CMgr prefix behavior and ordinary resolved-feature reader registrations remain unchanged.

Validation: all **937 regression tests pass** (92.123 seconds); specification lint reports zero findings (483 headings, 96 anchors).

Evidence: C-301.

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

### 11.261 Owned decal/scene stages and manager field endpoint

*Status: Partial · Two explicit field sequences reach EOF; virtual load stages remain open · Evidence: C-303*

After the tagged +0x6C8 list in 11.260, native CMgr input constructs two owned data containers and reads them directly. The +0x6D8 container uses constructor **0x4BA1EEE0** and input helper **0x4BA25CC0**; +0x6E0 uses constructor **0x4C6E5AF0** and input helper **0x4C6EF030**. Each constructor initializes runtime maps and owner links. Neither adds an object/class header or MFC slot to the containing archive. Both input helpers add archive condition **0x200**, read their sequences and remove that condition before returning.

| Owned member | First sequence | Second sequence |
| --- | --- | --- |
| +0x6D8 | signed i32 count; each positive-count entry is tagged `moDecalProperties_c` followed by i32 association value | signed i32 count; each positive-count entry is i32 key, signed i32 value count, then tagged decal pointers per positive-count value |
| +0x6E0 | signed i32 count; each positive-count entry is tagged `swScene_c` followed by i32 association value | signed i32 count; each positive-count entry is i32 key followed by tagged scene pointer |

The current decoder implements **nonpositive counts only** for both sequences, retaining negative count values and skipping their native positive-count loops. It stops before the first object/association entry when either count is positive. The condition-dependent decal and scene object bodies remain open. With zero counts, each direct container occupies **eight bytes**. Runtime map allocation, scene equivalence tests and decal association filtering do not add inline bytes.

The supported modern field sequence then continues:

| Order | Member or stage | Wire encoding |
| --- | --- | --- |
| 1 | +0x700 conversion IDs | signed i32 count, then tagged `moConvertFeatIds_c` for each positive-count entry |
| 2 | +0x728 | direct `su_CStringArray`, using its established ReadCount/CString body |
| 3 | +0x750,+0x754 | i32 then u32 |
| 4 | +0x7F0 plane map | u32 count; positive entries require i32 key followed by three direct `mgPlane_c::restore` bodies; decoder stops before the first entry |
| 5 | helper 0x4C23B8D0 | named external MsgPack/DB lookup; **zero inline archive bytes** |
| 6 | +0x808,+0x810,+0x814 | CString, i32, i32 |
| 7 | +0x818 | signed i32 count; each positive-count entry is CString key then CString value |
| 8 | +0x828 | `su_CTime`: **one u32 on the wire**, with 0xFFFFFFFF representing the native -1 sentinel |
| 9 | +0x6B8 | signed i32 count; each positive-count entry is i32 key, u32 element count, then signed i32 values |
| 10 | +0x858 | i32 flag; native loaded value is nonzero-to-true |
| 11 | +0x898/+0x8A8 group presence | u32 presence; zero omits both groups; nonzero is an explicit unsupported stop |
| 12 | final load hook | virtual +0x1C90, then manager virtual +0x11C0 and delegated configuration load stages; explicit unresolved diagnostic stop |

The owned diagnostic is limited to the modern path with version > 0x46C1, where all listed fields are present and earlier obsolete interleaved layouts are absent. Native gates include +0x6D8 > 0x2399, +0x6E0 >= 0x23A5, conversion IDs > 0x23BB, CStringArray > 0x23C3, +0x750 > 0x27BE, +0x754 > 0x27D5, planes > 0x3733, named pairs/time > 0x3765, +0x6B8 > 0x41A2, flag > 0x42E8 and the final display-group presence > 0x46C1. Earlier variants remain disabled rather than inferred from remaining byte length.

Helper **0x4C23B8D0** reads named MsgPack data through `sldunqliteDBUtils::GetMsgPackMap` and passes any result to a separate runtime setter. It does not consume the CMgr archive cursor. This zero-inline result does **not** document that external data's schema or assign its bytes coverage. Helper **0x4C24A230** reads the named pairs/time block. Its bound su_CTime import resolves to **swccu.dll 0x31D4BC60**, whose retained extractor calls a four-byte word reader. The decoder keeps raw u32 timestamp bits, including the sentinel, and does not consume the runtime member's eight-byte size.

Raw +0x818 pairs preserve order and duplicate keys. The +0x6B8 vector is copied into a native runtime set; the decoder retains duplicate and ordered raw values rather than replacing them with that set. Counts are bounded before allocation. +0x858 retains the original i32 alongside the normalized loaded boolean. Nonzero display-group presence is not treated as a count; it precedes further count/value structures that remain unimplemented.

**Fixture endpoints.** API `SMGussetAPI.SLDPRT` supplies interval **2,692..2,758**, and `fpf.sldprt` supplies **2,965..3,031**: **66 bytes each, 132 combined**. Both owned containers have zero counts; conversion IDs, string array, plane map, named pairs and set map are empty. +0x750 is 0, +0x754 is 2, +0x808 is empty, +0x810/+0x814 are 0, the raw timestamp is 0xFFFFFFFF, and both final flag/presence fields are zero. The explicit field endpoint equals each stream's EOF. Native class/body layouts, the earlier nonempty pair/property maps and shared object references establish alignment; EOF alone does not establish semantic completeness.

**Deferred load chain.** CMgr vtable **0x4CEC7AF8** binds +0x1C90 to **0x4C247A90**. That hook enumerates configurations, applies runtime state/version fixups and calls manager virtual +0x11C0, bound to **0x4C23ED10**. The latter delegates to each configuration's own virtual +0x11C0 with the archive argument. Both part and base configuration vtables bind their +0x11C0 slots to **0x4C23E890**, which begins with inherited `moFeature_c::SerializePass2` before configuration runtime fixups. The inherited pass-two stage is not yet proven byte-free or decoded here. `through_manager_owned=True` therefore raises a retained **postload_1c90** stop even at exact EOF; it does not return a complete root or award complete-manager/document coverage. Simulation `frame.sldprt` still stops earlier at its nine condition-dependent body states.

Three focused tests verify exact source-bound native suffixes and EOF stops, four-byte timestamp composition, signed/unsigned extremes, duplicate pairs/set values, raw/nonzero boolean normalization, unchanged object-map size for direct empty bodies, truncation and explicit condition-dependent/plane stops. All-installed-project, part, assembly and drawing semantic bit coverage remains incomplete.

Validation: all **943 regression tests pass** (89.783 seconds); specification lint reports zero findings (485 headings, 96 anchors).

Evidence: C-303.

### 11.262 Guarded singleton manager completion and pass-two loading

*Status: Partial · 481 complete supported CMgr streams among 919 installed part occurrences · Evidence: C-304*

The remaining inherited configuration pass-two call from 11.261 resolves to **`moFeature_c::SerializePass2` at 0x4BB4D520**. Its named export is bound through the raw export name/ordinal/EAT arrays. The complete native function does not read, copy or save its archive argument. Its first direct call is **0x4B29DAE0**, whose complete three-byte body is **C2 00 00**, a return with zero stack adjustment. The decompiler's destructor alias for this shared address is not evidence of object destruction. The next call uses feature virtual slot **+0x1250**; both part and base configuration vtables bind that slot to the same return-only body.

The function's remaining branch requires both runtime **+0x1E8 == 0** and **+0x1E0 != null**. The complete reader requires the serialized optional +0x1E0 field to be absent. This disables that branch without inferring results of its virtual +0x118/+0x128 predicates. Their targets are retained as **0x4BB5FA30 / 0x4C04E130**, but the enabled reader does not rely on the nonnull branch being byte-free. On this guarded path, inherited feature pass two adds **zero archive bytes**. The calling configuration pass-two routine **0x4C23E890** then performs runtime list/owner/version fixups; the old version conversions are excluded by the modern version gate. No additional archive field is introduced by the supported path through the manager load chain.

**Complete-reader boundary.** `configurationmanager.read_supported(doc)` enables the existing scoped first-pass readers, completes their explicit fields and accepts final postload only when all of these conditions hold:

- The stream follows the ordinary external-XML part context and the owned manager layout has version **> 0x46C1**.
- Both configuration pointers +0x358/+0x360 are the **same mapped `moPartConfiguration_c` object**.
- The direct +0x368 list contains exactly that same object, once.
- That configuration has no parent +0x698 and an empty child list +0x6A0.
- Its feature +0x1E0 optional value is absent and raw +0x77C is zero.
- All preceding dependent bodies are implemented on the selected path; positive unsupported body-state, decal/scene, plane or display-map cases still stop explicitly.
- The accepted root ends at **exact stream EOF**, with no trailing bytes.

These are checked runtime conditions, not assumptions inferred from file extension or remaining byte count. A multi-configuration graph or a nonnull +0x1E0 field remains unsupported even when the explicit fields happen to reach EOF. Earlier opt-in prefix APIs still raise their diagnostic postload stops; enabling a complete read does not silently change their contract. The complete root is returned through the original `ObjectArchive.object` reader, preserving its mapped root identity and every configuration back-reference. It is not assembled afterward from an interrupted object's partial dictionary.

**Installed-part audit.** `configuration_manager_supported.py` attempts every **919 installed .sldprt occurrence**, retaining each CMgr source hash or its explicit failure. It finds **481 complete supported streams** and **438 explicit stops**. The complete streams total **1,466,808 decoded bytes**. Raw-field accounting still marks **739,066 bytes unknown**, leaving **727,742 bytes assigned to known archive structures**. These are decompressed stream measurements; they are not physical compressed-file credit, engineering-value semantics or whole-document coverage. The audit does not turn an exact EOF into 100% semantic bit coverage.

The 481 complete results contain persisted +0x650 states **0 in 420 streams** and **1 in 61 streams**. API `SMGussetAPI.SLDPRT` completes at byte **2,758** with state 1, and `fpf.sldprt` completes at **3,031** with state 0. The former unsupported postload stops for those two streams are therefore resolved on the guarded complete-reader path. Other manager variants remain available for continued schema work rather than being treated as complete.

Among the frozen isolated audit's explicit failures, **152** stop at nonempty condition-200 part body states, **77** at unsupported component version/archive context, **62** at positive scene-container object counts and **52** at unverified postload graph variants. The remainder includes Swift schema references, face-color/Toolbox dependencies, legacy layouts and other explicit dependency errors. Those counts describe this audit's full installed-part inventory; they are not a claim to have inventoried every installed project/add-in format.

Three focused tests replay every one of the **481 positive source-bound streams**, verify their field endpoints/state/raw accounting and original mapped-root identity, reject nonnull optional-field and repeated-configuration graph variants without advancing the cursor, and reject appended trailing bytes. Assembly/drawing configuration variants, the remaining 438 part managers, nonnull pass-two branches, named external MsgPack/DB data, raw-field meanings and complete part/assembly/drawing/all-installed-project semantic bit coverage remain open.

Validation: all **946 regression tests pass** (117.591 seconds); specification lint reports zero findings (486 headings, 96 anchors).

Evidence: C-304.

### 11.263 CMgr known-byte accounting and root validation

*Status: Partial · Known structures credited in 481 supported part managers · Evidence: C-305*

The complete supported reader from 11.262 is now registered for the exact stream name **`Contents/CMgr`** in `solidworks/swcover.py`. It does not match `Contents/CMgrHdr2` or other similarly named streams. `configurationmanager.documented_bytes(doc)` first requires a complete supported root and exact stream EOF, then returns **decoded stream length minus raw unknown-field bytes**. Any unsupported or malformed path returns zero credit. Partial prefix progress, an EOF diagnostic stop or an unrelated fully decoded object does not qualify.

The full-root acceptance check explicitly requires nonnull class **`moConfigurationMgr_c`** with the completed stage. A two-byte null pointer or a fresh empty `suObList` can be a valid complete object archive by itself, but neither is a configuration manager. Both are rejected before coverage is assigned. Appended trailing bytes and the existing unverified configuration graph branches remain rejected. This root check supplements the guarded native pass-two evidence; it does not replace that evidence with a byte-count heuristic.

At the edition 0.283 checkpoint, the installed part corpus had 481 supported managers totaling **1,466,808 decompressed bytes**. Raw-field accounting leaves **739,066 bytes unknown**, so the known-structure credit is **727,742 decompressed bytes**. All other 438 part-manager paths receive **zero**. Assemblies remain unsupported by this singleton-part reader and receive no CMgr credit. The per-stream measurement retains each source hash, decoded size, known size, stored size and assigned stored-byte shares.

Annex G assigns compressed stream bytes proportionally. For each stored data region, the credited stored share is **floor(stored region size × known decoded bytes / decoded stream size)**; its unknown share is the remaining stored region bytes. The rule preserves rounding remainders as unknown. It does not equate decompressed bytes with physical file bytes or credit raw values merely because their storage type is readable.

| Measure | Before CMgr registration | Edition 0.283 | Change |
| --- | --- | --- | --- |
| Part CMgr stored bytes | 1,866,112 | 1,866,112 | unchanged denominator |
| Part CMgr unknown stored bytes | 1,866,112 | 1,583,981 | 282,131 newly documented |
| All part unknown file bytes | 69,244,729 | 68,962,598 | 282,131 newly documented |
| Part file-byte coverage | 69.5% | 69.6% | rounded Annex G measure |
| Part coverage excluding container/previews | 62.9% | 63.0% | rounded Annex G measure |

The edition 0.283 measurement reruns the complete existing coverage registry over **919 part, 176 assembly and 20 drawing container documents**. It verifies that every denominator is unchanged and that all unknown-byte reduction is exactly the independently recorded CMgr stored share. Assembly unknown bytes remain **11,264,940** and drawing unknown bytes remain **2,240,327**. Their coverage remains **83.8% / 81.3%** and **57.3% / 49.9%**, respectively, before/after the container-and-preview exclusion. The generated per-stream gap report is updated from the same run, and the pre-CMgr gap report and edition 0.283 attribution snapshot are retained.

At that checkpoint, three focused tests verified every installed part manager's conservative result against the 919-file audit, required exactly 481 positive results and 727,742 known decoded bytes, rejected null/unrelated roots and confirmed exact registry matching. The expanded current audit is documented in 11.264. No whole document is completed by this accounting change. Raw-field meanings, the 438 unsupported part managers, assembly/drawing manager variants, other document streams and all-installed-project formats remain open. Annex G still does not measure project formats.

Validation: all **949 regression tests pass** (103.007 seconds); specification lint reports zero findings (487 headings, 96 anchors).

Evidence: C-305.

### 11.327 Saved configuration-manager context for template sheet metal

*Status: Verified opt-in context propagation; two template bodies complete · Evidence: C-369*

The completed CMgr reader now supplies persisted sheet-metal helper state to model and separate resolved-feature readers. Native model functions 0x4C249C50 and 0x4C250750 copy manager memory member +0x650 into archive-helper +0x850, as established in 11.254/11.261. The state is not a constant EOF word or a default inferred from a fresh helper. Native CMgr Serialize 0x4C22C140, both state-copy functions and their decompilation identities are retained and revalidated in `template-sheet-metal-persisted-context-native-evidence.json`.

`configurationmanager.sheet_metal_context(doc)` first requires supported manager framing, the postload graph guard and exact EOF. It then obtains the saved +0x650 value and its recorded source position, verifies the original signed i32 at that position, and accepts only helper states 0 or 1. Versions <=0x27CB or manager layouts without the verified state-position field reject. The result records the value, field offset, CMgr source SHA-256, version and stream name. This establishes the state used for parsing; its broader application semantics remain partial and no additional semantic byte credit is assigned.

`modelprefix.read(..., configuration_context=True)` derives this context from the same document before reading the model root. It installs +0x850 and its verification marker on that archive instance. This mode currently requires the supported part-manager path; assembly context is not inferred. `resolvedfeatures.read_data` copies the state, verification marker and source provenance into the separate stream archive while preserving the model's inherited object identities. It copies the provenance dictionary so mutations do not change the model archive's record. The default model-prefix mode remains unchanged for historical replays.

Both existing template probes store state **1**. The frame and API-gusset sources now complete their `moTemplateSheetMetal_c` objects using that saved value, reproducing the earlier explicit-mode experiment without assuming a state. They reach cursors **324,619** and **27,885**, respectively, and stop explicitly on fresh `moCompSheetmetalFeat_c` objects. Neither resolved list is complete, so this work adds no whole-stream or stored-byte credit. All 919 part CMgr framing completions and current document coverage totals remain unchanged.

`template_persisted_context.py` reproduces the two probes in `template-sheet-metal-persisted-context-corpus.json`, retaining configuration names, resolved-stream hashes, root intervals/hashes, completed template records and CMgr provenance. `template_persisted_context_evidence.py` checks the native state propagation and source-derived endpoints. Tests verify the exact saved i32/hash, both template bodies, unchanged unverified historical context and rejection of unsupported state values, missing positions and mismatched source bytes. Other resolved-feature dependencies, complete lists and semantic/document/project coverage remain open.

Validation: all **1,085 regression tests pass** (122.231 seconds); specification lint reports zero findings (551 headings, 96 anchors).

Evidence: C-369.

