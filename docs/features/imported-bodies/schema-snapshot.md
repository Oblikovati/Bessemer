### 11.89 Atom partition/maps, late fields and body-feature suffix

*Status: Native serializer evidence; nine complete atom/body-base boundaries verified; enclosing extrusion suffix incomplete · Evidence: [C-104]*

In the scoped ordinary zero-condition archive, the continuation after 11.88 begins with a nullable direct array at +0x548, then i32 partition presence. A nonzero presence adds a tagged moPMarkRecord_c pointer and three i32 values at +0x3B8, +0x3C0 and +0x578. The mark record's body is one i32 permanent-mark ID. Runtime pointers and cached kernel objects in that class add no archive fields. Readers SHALL preserve the signed ID and raw presence value.

Next is i32 word_5C8 and three maps in order. Map +0x670 stores i32 count and count pairs of u32 key/u32 value. Map +0x5D0 stores i32 count and count pairs of tagged moCompEdge_c pointer/u32 value. Map +0x3F8 stores i32 count and count pairs of u32 key/u32 value. Readers SHALL preserve duplicates, pointer identity, unsigned high bits and original entry order independently of native hash-map insertion.

The late atom fields are i32 word_488, i32 appearance_count and appearance_count pairs of i32 key/tagged moVisualProperties_c pointer, then i32 direct-data presence at +0x6A0. Nonzero introduces an additional direct serializer that remains unsupported; readers SHALL stop explicitly there. Zero is followed by i32 face-name count and pairs of tagged moCompFace_c pointer/CString, then i32 index-name count and pairs of i32 key/CString, and final i32 word_6B4. All these fields are present in the documented modern scope. Legacy branches and nonzero archive masks may differ.

This completes the atom-feature helper. The ordinary moBodyFeature_c continuation is tagged moAsmFeatData_c pointer, tagged moCompFeature_c pointer, direct embedded suObArray at +0x778 and i32 word_798. The component-feature pointer is omitted under archive condition 1, outside this zero-condition path. These bytes precede the boss/extrusion-specific moEndSpec_c suffix in 11.81. Unsupported nested objects still stop the containing reader.

The dedicated map, late and body-tail audits each reach nine native boundaries. All nine cases store partition records; the three maps contain zero entries. The optional late direct-data body is absent in these cases. Twelve profile objects complete during traversal, while three other enclosing cases still stop on cosmetic-thread objects. These results verify atom and body-feature base framing, not complete boss/extrusion objects or streams. Default complete manager count remains 892; whole-document bit coverage is unchanged.

### 11.152 Ordinary modern imported and base-body feature layers

*Status: B — native-derived reference/inline-body framing with populated native body names; positive native inline/vector fixtures and vector meanings remain partial.*

Both classes are implemented in sldMoPartu.dll. moImported_c primary table `18048F940`, locator `1804F0500`, selects serializer `1802FCC20`. moBaseBody_c primary table `180491FA8`, locator `1804F05F0`, selects serializer `1802FC730`, which calls the imported serializer before its own suffix. Base-body runtime descriptor `180605768` selects factory `1802FB790`, calling constructor `1802FB830`; that constructor calls imported constructor `1802FBB50`, which calls moBodyFeature_c. Concept lightweight/resolved hooks import the common sldmodu.dll serializers. Base-body allocation size 0x868 is not wire length.

This grammar applies to ordinary non-DB external-XML loading with v > 0x27CB, helper state +834 equal to zero and archive conditions clear. The imported layer is shared by both classes; only moBaseBody_c has fields 6–7.

| # | Field | Type | Presence | Description and rule | St. |
|---|---|---|---|---|---|
| 1 | inherited body-feature base | Existing complete inherited body | Always | No additional imported class tag between inherited and derived fields | B |
| 2 | filename +800 | CString | Always in selected context | Original serialized filename, including empty or Unicode strings | B |
| 3 | external-body marker | i32 | Always in modern scope | Zero selects field 4; any nonzero value skips inline body loading | B |
| 4 | body +810 | Direct common kernel-body frame | Marker zero | Suppression byte or supported present-body framing; no MFC object tag | B |
| 5 | body name +808 | CString | Always in modern scope | Name used to resolve the external stored body; follows the complete inline frame when present | B |
| 6 | integer-vector count +850 | i32 | moBaseBody_c and v > 0x2BCC | Fixed signed count, not ReadCount; SHALL be nonnegative and fit available four-byte entries | B |
| 7 | raw vector entries +850 | count × i32 | Field 6 present | Preserve signed values and ordering; meanings unknown | B |

The minimum imported suffix is **6 bytes** for empty filename/name strings and a nonzero marker; the modern base-body vector adds at least four bytes for its count. A suppressed inline body adds its suppression byte instead of omitting field 4. A present inline body uses the existing kernel archive reader: it accepts the fully validated compressed single-body Parasolid form or stops explicitly at unsupported framing. A plausible length alone does not establish a body boundary. The imported-body call uses native mo_loadBodyFromArchive with the ordinary permanent-partition variant; it does not serialize a CObject pointer for the body.

The native imported serializer derives temporary +30C state from archive conditions 0x200000 and 1. Those conditions are clear in this scope, so there is no second transient-body-presence word after a nonzero field 3. Condition 0x40000 has a separate body-handle shortcut and is outside this grammar. Nonzero helper +834 returns from the imported stage after the inherited body and is also outside this reader. These branches cannot be inferred from serialized filename contents or field 3 alone.

After field 5, native code can replace an empty body name with the feature name, look up a body in the file system, remove third-party attributes and change runtime validity/load flags. The offline reader retains the original strings and marker, and does not apply those runtime mutations. Constructor members +810/+818/+820/+828 and the imported runtime map at +838 are not extra wire fields. The base-body vector is a separate collection initialized by the derived constructor; it has no tagged-array header.

Native historical behavior differs: base-body versions before 0x4F read the body-feature base, a filename and an old body representation directly. The imported serializer also has pre-0x3D3 kernel paths and only reads body name +808 after v 0xFC4. These older paths are retained as native evidence but are not implemented by the modern scoped reader. The vector threshold is independent: **v == 0x2BCC has no vector count; v == 0x2BCD does**.

The frontier contains **36 freshly serialized moBaseBody_c features** in completed roots. Every external-body marker is 1, every body name is nonempty and every vector is empty; one filename is nonempty. No standalone native moImported_c completion is claimed. Synthetic tests cover that class's composed dispatch, Unicode filename strings, arbitrary nonzero markers, a suppressed inline body, a validated native compressed Parasolid frame composed into the inline branch, populated vectors with signed extrema, the vector gate and malformed/truncated/context rejection. A stream-hash-bound native regression checks the full base-body containing-root end. Composed inline-body tests are not positive native imported-feature fixtures.

Edition 0.163 records **20,653 complete roots**, **476 list-complete configurations** and **430 explicit stops** across all **906 complete modern model contexts**. **438 lists reach exact EOF**, spanning **20,039,815 decoded bytes**; **38 list-complete streams retain four-byte unknown tails**. Leading stops are extruded reference surfaces (22), sheet-metal features (21), sweeps (17), face/point reference-plane data (16) and blend cuts (15). Full part, assembly, drawing and installed-project bit coverage remains incomplete. Integer-vector semantics, unsupported embedded-kernel contexts and post-list tails remain open; this traversal adds no whole-document coverage credit.

Evidence: C-180.

