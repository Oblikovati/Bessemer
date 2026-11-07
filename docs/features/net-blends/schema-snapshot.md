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

The populated roots also require three constructor-bound dependencies. `moContactData_c` is a tagged object pointer followed by u32 +0x10. `moCompEntGroup_c` serializes `moCompRef_c`, i32 +0xE8, then two direct `suObList` bodies at +0xF0 and +0x138. `moCompMultiEdge_c` serializes `moCompRef_c` and its owned implementation body; the null-owned fallback uses the same direct serializer. That body begins with a **u32 count**, not ReadCount, and that many `moCompEdge_c` pointers, then two generic object pointers, two i32 values, two f64 values and two points for version >0x8EE, and a final u32 for version >=0xC8E. The native constructors and RTTI prove that the two new component classes derive from `moCompRef_c`, permitting them at the typed reference-wrapper boundary without widening it to arbitrary objects. Unknown scalar meanings remain counted as unknown.

`netblends.py` reconstructs these layouts and preserves original scalar/point values and context-dependent object identities. Five original net-blend roots complete exactly, spanning 40,237 bytes. Four affected lists reach EOF; the mouse overmold case completes its 21,169-byte net-blend root and next stops at unsupported `moSewRefSurface_c`. The component dependencies also complete one further assembly-mates sample's part configuration.

The full 906-context audit now has **24,598 complete roots**, **785 complete lists** and **121 explicit stops**, with no regression in any preceding complete root boundary. **712 lists reach exact EOF**, spanning **48,774,033 decoded bytes**; the five new exact-EOF lists total **459,281 bytes**. **73 complete lists retain four-byte unknown tails**. The preceding checkpoint is retained as `resolved-feature-list-frontier-0.245.json`; the new checkpoint is `resolved-feature-list-frontier-0.246.json`. Four focused tests cover version/width boundaries, independent and negative point counts, thin-dimension conditions, exact native hashes and the next unsupported boundary. These results establish framing progress, not 100% field meaning or additional whole-document coverage credit.

Evidence: C-268.
