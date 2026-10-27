### 11.230 Deleted-face features and data

*Status: B / ? — native-bound modern layout and four populated roots verified; option meanings, historical variants and whole-document semantics remain partial.*

For ordinary external-XML model archives with file version greater than `0x27CB`, `moDelFace_c` selects serializer `0x18025C680` in `sldMoPartu.dll`. It writes the body-feature base followed by this payload, in order. Offsets identify runtime fields, not archive positions.

1. Nullable `suObArray` pointer at `+0x848`, serialized by `suObArray::serializePtr`: one i32 presence word, followed only when nonzero by a direct array body (ReadCount and tagged object pointers). Retain the raw presence value, including noncanonical nonzero values.
2. One i32 at `+0x840`.
3. Direct `suObArray` body at `+0x800`.
4. Direct `su_CDWordArray` body at `+0x820`: ReadCount followed by that many **u32** elements.
5. Four i32 values at `+0x850`, `+0x854`, `+0x858`, `+0x85C`.
6. Direct `suObArray` body at `+0x868`.

Constructors `0x18025BF90`, `0x18025C040` and `0x18025C130` independently establish the three concrete direct container types. The object arrays participate in the inherited class/object map; nested data objects are tagged normally. Raw scalar and DWORD-array values remain unresolved for semantic coverage accounting. Runtime field `+0x860` is initialized by constructors but is not written by this serializer.

`moDelFaceData_c` selects serializer `0x18025C520`. After the empty `su_CObject` base, it writes three nullable `suObArray` pointers at `+0x08`, `+0x10`, `+0x18`, each with its own i32 presence word and conditional direct body, followed by one i32 at `+0x20`. There is no owning-feature pointer in this base. The load branch's cleanup of preexisting runtime arrays consumes no archive bytes. The selected nullable-array helper is imported from `SLDMFCU.dll` as `?serializePtr@suObArray@@SAXAEAVsu_CArchive@@AEAPEAV1@@Z`.

Historical loading reads `+0x858` from version `0x91D` (otherwise zero), reads `+0x85C` after `0x964` (otherwise zero), and reaches the final `+0x868` array from `0x10E0`. The writer emits all these fields unconditionally. The implementation is restricted to the modern context where every field is present; these gates do not establish a general historical writer/reader equivalence.

Four complete native deleted-face roots total **65,197 bytes**: Carving Knife motor (29,212), Mouse Main Housing Overmold (26,389), and the molded-product tutorial mousebase and mousesplit (4,798 each). The motor's declared feature list completes; the overmold reaches `moCopyEdgeIdRep_c`, and both tutorial files reach `moFillRefSurface_c`. Native stream/root hashes, raw scalar values and container counts are retained. These are framing results; further feature schemas and whole-document semantic coverage remain open.

The 906-context audit now contains **24,720 complete roots**, **786 complete lists** and **120 explicit stops**. All preceding complete root records remain identical. This adds 66 roots and one exact-EOF list: the 692,348-byte motor stream. Totals are **713 exact-EOF lists / 49,466,381 decoded bytes**; 73 complete lists retain four-byte unknown tails. Checkpoints `resolved-feature-list-frontier-0.249.json` and `resolved-feature-list-frontier-0.250.json` retain the comparison. Exact EOF does not establish all field meanings or additional whole-document semantic coverage.

Validation: all 844 regression tests pass (89.640 seconds); specification lint reports zero findings.

Evidence: C-272.

