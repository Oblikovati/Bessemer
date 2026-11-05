### 11.226 Net-blend features, curve sets and component groups

*Status: B / ? — constructor/vtable-bound ordinary modern net-blend/cut layout and five populated native roots complete; scalar domains, legacy variants and whole-document semantics remain partial.*

In the ordinary external-XML model context, `moNetBlend_c` and `moNetBlendCut_c` share serializer `0x4CB200D0`. Both primary vtables select this function at +0x28, and native RTTI includes `moBodyFeature_c` ancestry. The feature serializes that body-feature base, followed by its embedded `moNetBlendData_c` member at runtime +0x800. Its constructor and the data constructor establish each embedded type; the data is direct, without an additional tagged object header. The reconstruction requires the same modern archive context as the component-reference readers (version greater than 0x27CB).

*Table 11-3 — Net-blend direct data order*

| Runtime data offset | Serialized body |
|---|---|
| +0x50 | First net curve set |
| +0x2A8 | Second net curve set |
| +0x500 | Centerline curve set |
| +0x848 | Body options |
| +0x8D0 | i32 when version > 0x1FE4 |
| +0x8D8 | Surface-degree body; load bytes only when version > 0x2F4B |

For each curve set, the direct base framing starts with an object array of contacts, using ReadCount and tagged objects. For each of its N entries, a further direct object array follows. When version >0xB4B, N points follow with no second count; each point is three f64 values. When version >0xC71, an independent i32 M follows, then M points if M is positive. The load loop consumes no points for zero or negative M; the raw signed count is retained. The reconstruction does not impose the writer's observed zero-or-N allocation choice on this load-side count. When version >0xCFF, three direct DWORD arrays follow at +0x1C8, +0x1E8 and +0x208, each with ReadCount and u32 elements, then four u32 fields at +0x228, +0x22C, +0x230 and +0x234. Array extents are bounded before allocation. Point roles and the four trailing words remain unnamed.

The net-curve wrapper invokes that base for version >0xB4B and then its curve-options body at +0x238. The centerline wrapper uses threshold >0xB91 before the same base framing and then its centerline-options body. Both thresholds precede the supported modern scope. The net-curve load path can migrate a stored option value 0x80 to 0x20 and update contact runtime state; the investigative view retains the original stored value instead of rewriting archive bytes.

Curve options contain two u8 values (+0x08,+0x0C), a discarded i32 retained by the reconstruction, i32 +0x10, and i32 +0x14 for version >0xCB0. Centerline options contain, in order: i32 +0x0C/+0x28; u8 +0x44/+0x48/+0x4C/+0x08; f64 +0x38/+0x18/+0x20; u8 +0x10 for version >3000; u8 +0x2C for version >0xBC1; and f64 +0x30 plus i32 +0x40 for version >=0xBCA. All are present in the supported modern scope. Scalar meanings, units and full enum domains are not inferred from runtime offsets.

Body options contain u8 +0x08/+0x0C/+0x10, i32 +0x14, u8 +0x18, f64 +0x20/+0x28, then i32 +0x30: 28 wire bytes. The native `gc_Body_Options_c::getWantThinSolid` getter in sldgcu.dll reads the integer at +0x18, which this serializer loads by widening its byte. When version >0xC98 and that value is nonzero, the root appends a `moDisplayDim_c` pointer at runtime +0x10F8; if body-options +0x30 is 3, another pointer at +0x1100 follows. The exact nonzero stored byte is retained. Surface-degree loading after version 0x2F4B consumes three i32 values (+0x10,+0x14,+0x18), including a full four-byte flag word whose native runtime value is Booleanized. Older load paths consume none of these twelve bytes; raw bytes and the load gate are preserved separately from runtime normalization.

Eight body-option fields have exact gc_Body_Options_c property associations. Actual RTTI establishes gc_Body_Options_c as a zero-offset base of canonical mo_Body_Options_c, providing the typed subobject join rather than assuming matching numerical offsets. The selected shared NetBlend/NetBlendCut feature-prefix checkpoint dispatches embedded data at 0x800; the data serializer reaches body options at 0x848. The retained feature prefix is partial and proves only that initial checkpoint, not whole-feature control flow or final state. The body-options serializer itself is authenticated as the complete 316-byte native routine.

| Body-option member | Native property | Runtime versus wire policy |
|---|---|---|
| 0x08 | IsCut | Full32 integer API; STORE low8, LOAD zero-extends byte into runtime word |
| 0x0C | MergeBody | Full32 integer API; STORE low8, LOAD zero-extends byte into runtime word |
| 0x10 | WantSolid | Full32 integer API with coupled setter below; low8/zero-extend byte codec |
| 0x14 | BodyCreationMethod | Full32 API, declared gcBodyCreation_e; signed32 archive word |
| 0x18 | WantThinSolid | Full32 integer API with coupled setter below; low8/zero-extend byte codec |
| 0x20 | Thickness1 | Raw binary64 getter/setter and archive value |
| 0x28 | Thickness2 | Raw binary64 getter/setter and archive value |
| 0x30 | ThinSolidType | Full32 integer getter/setter and signed32 archive word |

The four byte codecs preserve the current low byte rather than Booleanizing it. Full32 API values can therefore change at a later save/load checkpoint by truncation and zero extension; accepted stored bytes are not restricted to 0/1. BodyCreationMethod supplies an enum type name only, without numeric labels or a closed code domain.

setWantSolid writes the full input to current 0x10 first. If input is zero and current 0x18 is nonzero, it also clears 0x18; nonzero input leaves 0x18 unchanged. setWantThinSolid writes the full input to 0x18 first, then sets current 0x10 to 1 only if input is nonzero and current 0x10 is zero. These are sequential native checkpoints, not atomic changes. The loader writes the zero-extended bytes directly and does not invoke those coupled setters, so no universal WantThinSolid/WantSolid file invariant follows.

The thickness names establish no physical units, positivity, finiteness, ordering or successful thin-body creation. Current receiver validity, aliases, lifetime, independent calls and archive identity remain qualified. The existing thin-dimension pointer conditions and all other curve-set/body fields retain their previous scope; no extra wire fields or accounting changes follow. Evidence: C-3544.

The populated roots also require three constructor-bound dependencies. `moContactData_c` is a tagged object pointer followed by u32 +0x10. `moCompEntGroup_c` serializes `moCompRef_c`, i32 +0xE8, then two direct `suObList` bodies at +0xF0 and +0x138. `moCompMultiEdge_c` serializes `moCompRef_c` and its owned implementation body; the null-owned fallback uses the same direct serializer. That body begins with a **u32 count**, not ReadCount, and that many `moCompEdge_c` pointers, then two generic object pointers, two i32 values, two f64 values and two points for version >0x8EE, and a final u32 for version >=0xC8E. The native constructors and RTTI prove that the two new component classes derive from `moCompRef_c`, permitting them at the typed reference-wrapper boundary without widening it to arbitrary objects. Unknown scalar meanings remain counted as unknown.

`netblends.py` reconstructs these layouts and preserves original scalar/point values and context-dependent object identities. Five original net-blend roots complete exactly, spanning 40,237 bytes. Four affected lists reach EOF; the mouse overmold case completes its 21,169-byte net-blend root and next stops at unsupported `moSewRefSurface_c`. The component dependencies also complete one further assembly-mates sample's part configuration.

The full 906-context audit now has **24,598 complete roots**, **785 complete lists** and **121 explicit stops**, with no regression in any preceding complete root boundary. **712 lists reach exact EOF**, spanning **48,774,033 decoded bytes**; the five new exact-EOF lists total **459,281 bytes**. **73 complete lists retain four-byte unknown tails**. The preceding checkpoint is retained as `resolved-feature-list-frontier-0.245.json`; the new checkpoint is `resolved-feature-list-frontier-0.246.json`. Four focused tests cover version/width boundaries, independent and negative point counts, thin-dimension conditions, exact native hashes and the next unsupported boundary. These results establish framing progress, not 100% field meaning or additional whole-document coverage credit.

Evidence: C-268.

### 11.229 Ruled surfaces from edges

*Status: B / ? — native-bound modern direct layout and one populated root verified; scalar meanings, historical variants and whole-document semantics remain partial.*

For ordinary external-XML model archives with file version greater than `0x27CB`, `moRuledSrfFromEdge_c` selects serializer `0x1801BC7A0` in `sldMoPartu.dll`. It writes the reference-surface base, one direct `suObArray` body at runtime `+0x7B8`, then the data body pointed to by runtime field `+0x7B0`. The data body is **direct**, without a tagged class/object header or presence byte. The load branch constructs `moRuledSrfFromEdgeData_c` before calling its serializer. The containing feature's separate embedded data object at `+0x7D8` is not emitted by this function.

The primary vtable's archive-backend selector at `+0xAB0` points to `0x1800E3930`, whose `33 C0 C3` body returns zero, establishing inline surface-base framing. Root constructor `0x1801BB670` proves the `+0x7B8` array's concrete type.

Data serializer `0x1801BC470` writes the following modern payload in order. Offsets identify runtime fields, not positions in the archive.

1. `moFeatureData_c` base: one tagged owning feature pointer at `+0x08`. The native loader requires a feature object. In the populated fixture this is a back-reference to the containing ruled-surface root, which has already entered the object map.
2. Direct `suObArray` body at `+0x30`: ReadCount, then that many tagged object pointers.
3. Direct `su_CWordArray` body at `+0xF0`: ReadCount, then that many **u16** elements. These element values remain unknown for semantic coverage accounting.
4. Direct `suObArray` body at `+0x50`.
5. Tagged `moDisplayDistanceDim_c` pointer at `+0x18`, then tagged `moDisplayAngularDim_c` pointer at `+0x20`.
6. One binary64 at `+0x90`, then four i32 values at `+0x98`, `+0x9C`, `+0xA0`, `+0xA8`.
7. Tagged `moDirectionSpec_c` pointer at `+0xE8`.
8. One binary64 at `+0xB0`, one i32 at `+0xB8`, one `mgVector_c` at `+0xC0` (three binary64 components), then one i32 at `+0xD8`.

The raw scalars and vector components retain their stored values; the scoped native property joins below do not add unsupported numeric domains.

Exact own moRuledSrfFromEdgeData_c methods, joined through the feature current 0x7B0 data pointer, name seven existing saved associations.

| Data member | Native property | Qualified operation |
|---|---|---|
| 0x90 | Length raw storage | Binary64 setter writes input first; getter uses current dimension 0x18 or falls back to 0x90 |
| 0xB0 | Angle raw storage | Binary64 setter writes input first; getter uses current dimension 0x20 or falls back to 0x90, not 0xB0 |
| 0xB8 | DirectInput | Raw full32 integer getter/setter and signed32 archive word |
| 0xD8 | MakeConnectSrf | Raw full32 integer getter/setter and signed32 archive word |
| 0xA8 | TrimAndSew | Raw full32 integer getter/setter and signed32 archive word |
| 0xC0 | AlongDirection | Const vector-reference getter, typed vector-assignment setter and existing vector archive codec |
| 0xE8 | DirSpec | Declared moDirectionSpec_c pointer getter/direct setter and typed tagged archive occurrence |

When the respective current dimension is nonnull, both numeric getters delegate to its virtual slot 0x868 with EDX=0. The Angle null-dimension fallback to 0x90 is preserved as observed; the documentation does not substitute 0xB0 to make the getter and setter inverse.

Each numeric setter stores its original input before owner/provider work. On a reached path where the owner at 0x08 is nonnull and the corresponding dimension is null, it may allocate the typed distance/angular dimension using a sign-cleared re-read of the current raw 0x90/0xB0 value. The later reached nonnull-dimension update instead passes the original signed input to virtual slot 0x860 with arguments 1,0. Creation-time sign clearing does not normalize the stored raw double or establish a final effective value. Allocation outcomes, current-member changes and provider callbacks remain qualified.

The angular-pointer and MakeConnectSrf LOAD reads each use an independent unsigned current query >=0x8FE; the reached older MakeConnectSrf branch assigns 1. Existing array gates and final runtime processing remain separate. Integer property names close no Boolean/enum domains, and AlongDirection establishes no unit-vector normalization or coordinate frame. Pointer/vector aliases, ownership, length/angle units, finite ranges and successful surface construction remain unproved. Raw 0x98/0x9C/0xA0 retain their unresolved interpretations. Evidence: C-3543. Constructor `0x1801BB550` proves the direct array types and initializes other runtime containers, including an object array at `+0x70`, that are absent from this serializer. Do not infer wire fields from object size or constructor defaults.

Historical gates differ from the modern layout: the WORD array is serialized from `0x8FD`; the `+0x50` object array is serialized after `0x989`; the angular dimension is read after `0x8FD`; and the final `+0xD8` word is read from `0x8FE` (otherwise initialized to 1). The writer always emits the angular pointer and final word. These differences do not authorize extending the modern reader into historical contexts. The loader's final `0x1801C1060` call performs runtime processing without an archive argument; it does not add another wire field.

The Mouse Main Housing Overmold sample contains one complete **8,261-byte** ruled-surface root. Its data owner is the root object itself. Parsing proceeds through further roots before an explicit `moDelFace_c` stop. Native source hashes and exact root spans are retained; this result proves the selected record framing and does not establish whole-document semantic bit coverage.

The complete 906-context audit now contains **24,654 complete roots**, adding sixteen while preserving every preceding complete root record. Totals remain **785 complete lists / 121 explicit stops**, including **712 exact-EOF lists / 48,774,033 decoded bytes** and 73 lists with four-byte unknown tails. The previous and current checkpoints are retained as `resolved-feature-list-frontier-0.248.json` and `resolved-feature-list-frontier-0.249.json`. No additional whole-document semantic coverage is credited.

Validation: all 841 regression tests pass (86.422 seconds); specification lint reports zero findings.

Evidence: C-271.

### 11.235 Punch features and internal record revisions

*Status: B / ? — native-bound revision-dependent layout and two populated revision-7 roots verified; raw option meanings and historical/native fixture diversity remain partial.*

For ordinary external-XML model archives with file version greater than `0x27CB`, `moPunch_c` selects serializer `0x18032DB30` in `sldMoPartu.dll`. It writes the body-feature base, then reads an **i32 internal record revision** at runtime `+0x858`. This revision is independent of the enclosing file version and MFC class schema. Constructor `0x18032CBC0` initializes it to 7.

The selected loader's suffix is:

1. Internal revision at `+0x858`, followed by tagged `moDisplayDim_c` pointers at `+0x800` and `+0x808`.
2. If revision is at least 1, i32 at `+0x864`; otherwise the native runtime field is initialized to zero.
3. Nullable `suObArray` at `+0x810`, using the established i32-presence/conditional-direct-body grammar. If revision is below 3, another nullable array at `+0x818`.
4. Two i32 values at `+0x860`, `+0x840`, then nullable `suObArray` at `+0x820`.
5. If revision is below 2, one direct `mgPoint_c` (three binary64 values) and one i32. The native loader appends them to its runtime point/integer arrays. Otherwise, serialize the point dynamic array at `+0x848`, then integer dynamic array at `+0x830`, using the grammar below.
6. If revision is at least 4, i32 at `+0x910`; at least 5, i32 at `+0x914`; at least 6, two i32 values at `+0x918`, `+0x85C`. Missing fields receive native defaults; the raw decoder records only fields actually present.
7. If revision is at least 7, one **tagged** `suObArray` pointer at `+0x920`. This differs from the nullable direct-array grammar used earlier.
8. Extension sequence described below.
9. If revision is below 2, a direct `moPunchSIdKeeper_c` body at runtime pointer `+0x908`, after the parent post-load call. Its primary vtable selects thunk `0x1803AC9DA`, bound to the existing `moEdgeFaceSIdKeeper_c::Serialize` layout. The keeper's enclosing-file-version gates still apply.

Exact own moPunch_c methods name four existing signed32 options. They are native integer properties, not recovered Boolean or enum domains.

| Parent member | Native property | Qualified LOAD policy |
|---|---|---|
| 0x864 | Exclude | Signed internal revision>=1 reads it; otherwise current runtime default 0 |
| 0x910 | ClearanceDirection | Signed internal revision>=4 reads it; otherwise current runtime default 1 |
| 0x914 | Cut | Signed internal revision>=5 reads it; otherwise current runtime default 0 |
| 0x918 | CutDir | Signed internal revision>=6 reads it; otherwise current runtime default 0 |

Each own getter/setter transfers the entire current 32-bit integer without normalization. The actual declaring-class signatures bind these roles even where implementation addresses are folded with other classes. The selected writer emits all four words without the LOAD revision gates; those gates use the record member 0x858 and signed comparisons, not enclosing archive version. Raw values and earlier/current revision checkpoints remain distinct.

Thickness and Clearance name the existing tagged dimension associations at 0x800 and 0x808. Their getters forward to the current dimension virtual slot 0x868 with EDX=0 and have no null guard. Their setters forward to virtual slot 0x860 with R8D=1 and R9D=0. Thickness forwards input XMM1 unchanged; Clearance first applies an authenticated bit mask clearing the scalar input sign bit while preserving its other 63 bits. This is a bitwise transformation, not a finite-value check or a new raw double wire field. Native pointer/lifetime, provider effects, units, final dimension values and successful punch geometry remain unproved. Other scalar/array/extension meanings retain their qualifications. Evidence: C-3542.

The point and integer dynamic-array loaders begin with a **u32 element width in bytes**, not an element count. Width zero ends that array immediately. A nonzero width must be divisible by 24 for point entries or 4 for integer entries. Next read two raw u32 allocator metadata words and a u32 entry count, then exactly `width × count` bytes of entries. Preserve width, metadata and count independently. For example, point width 48 means each entry carries two points; integer width 8 means two four-byte integers per entry. Bound count using remaining bytes before reading. The native writers additionally require the second metadata word to be 1 or 2; the loader passes those metadata fields to its allocator, so their broader meanings are not inferred here.

The point loader chain is `0x18032DAD0` → `0x18032C780` → `0x18032C650`; the integer chain is `0x18032DA70` → `0x18032C750` → `0x18032C570`. Constructor type declarations and the writers at `0x18032C8A0`/`0x18032C7B0` independently establish point versus integer widths. A runtime dynamic-array pointer or allocation size is not itself a wire field.

The extension loop reads one i32 tag at a time:

| Tag | Following payload | Next action |
|---|---|---|
| `0x3D2` | i32 | Continue reading tags |
| `0x13E` | binary64 | Continue reading tags |
| `0x1466` | generic tagged object pointer | Continue reading tags |
| Any other value, including 0 | No payload consumed by this loader | Terminate the loop |

The native loader consumes and discards recognized extension values; the diagnostic reader retains their order, values and object identity. It also retains the actual terminating tag, which need not be zero. An unknown terminating tag does not authorize skipping an invented payload. The selected writer emits a zero terminator and no recognized extension entries.

The writer emits its current scalar/dynamic-array suffix unconditionally rather than reproducing every older load branch. Therefore the loader revision gates must not be treated as a historical writer specification. Tests exercise revisions -1, 0 through 8 using the selected native loader's comparisons; the two installed positive fixtures both store revision 7. Stored option scalars, allocator metadata, point/integer values and consumed extension payloads remain unknown for semantic coverage accounting.

Two complete native punch roots total **12,839 bytes**: Bottom Housing (6,506) and Main Housing (6,333). Each has one point-array entry of width 24, one integer entry of width 4, no extension entries and a zero terminal tag. Main Housing's feature list completes; Bottom Housing reaches the explicit `sgIntersection_c` zero-marker-body stop. Source stream/root hashes and raw words are retained. These results establish selected framing without claiming whole-document or all-project-format bit coverage.

The 906-context audit contains **24,985 complete roots**, **794 complete lists** and **112 explicit stops**, with every preceding complete root record unchanged. This adds 41 roots and one exact-EOF list: Main Housing, **1,982,364 decoded bytes**. Totals are **721 exact-EOF lists / 55,171,677 decoded bytes**; 73 complete lists retain four-byte unknown tails. Checkpoints `resolved-feature-list-frontier-0.254.json` and `resolved-feature-list-frontier-0.255.json` retain the comparison. No additional whole-document semantic coverage is credited.

Validation: all 860 regression tests pass (99.668 seconds); specification lint reports zero findings.

Evidence: C-277.
