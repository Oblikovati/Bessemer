### 11.217 Move/copy body feature and nested-data boundary

*Status: B / ? — native outer serializer, inherited hooks and constructor bindings verified; nested data helper grammars and complete native feature endpoints remain unresolved.*

The ordinary non-DB external-XML feature context (version >0x27CB) binds moMoveCopyBody_c primary 0x4D110790 / COL 0x4D352258 to serializer 0x4BB2D4B0. Its RTTI parent and explicit base call are moBodyFeature_c; hooks +4B8/+4C0 select the ordinary inherited pair 0x4C189B60 / 0x4C18A920. After the complete body-feature prefix, the sequence is:

| # | Field | Type | Presence | Description and rule | St. |
|---|---|---|---|---|---|
| 1 | data +9C8 | tagged moMoveCopyBodyData_c | always | Separate payload object; a new object stops until its helper grammar is supported. | B |
| 2 | helper +9D8 | tagged moMatrixIndexHelper_c | always | Existing matrix-index-helper grammar. | B |
| 3 | raw +9F0 | i32 | always in scope | Preserve signed stored word. | B / ? |
| 4 | dimensions +808,+810,+818,+828,+830,+838 | six tagged moDisplayDim_c | version >0x3B0B | In this exact order. | B |
| 5 | raw +8B8,+8BC,+8C0,+8C4,+8C8,+8CC | six i32 | version >0x3B0B | Separate raw signed words. | B / ? |
| 6 | dimension +800; raw +8D0 | tagged moDisplayDim_c; i32 | version >0x42CC | Pointer then signed word. | B / ? |
| 7 | dimension +820; raw +8D4 | tagged moDisplayDim_c; i32 | version >0x42CC | Pointer then signed word. | B / ? |

The raw +9F0 native load gate (>0x12A0) predates scope. The complete outer body with null/cached references is 259 bytes through 0x3B0B, 295 through 0x42CC and 307 thereafter. Those sizes do not include newly serialized tagged targets. Native post-load mate ownership/fixup loops do not introduce another archive collection here.

The nested moMoveCopyBodyData_c primary 0x4D110700 / COL 0x4D352350 selects serializer 0x4BB2CE90, with su_CObject ancestry. Factory 0x4BB160A0 calls constructor 0x4C151F60 (runtime allocation 0x4D8). Constructor evidence identifies a direct suObArray at +370, direct suPtrArray at +3F0 and direct moPSMeshToleranceHandler_c at +338; the arrays at +390/+3B0 are also constructed but are not separately dispatched by this serializer. Runtime allocation sizes are not wire sizes.

For modern scope, the native nested sequence is: three tagged moRefWrapper_c pointers (+3D8,+3E0,+3E8); f64 +328; direct array +370; fixed archive mgXform_c +08; point +2D8; i32 +320; vectors +2F0/+308; i32 +368/+36C; direct suPtrArray +3F0; tagged moStock_c +3D0; i32 +410; vectors +418/+430/+448; tagged moRefWrapper_c +478; i32 +4C0; tagged moPlaneRef_w +4C8; i32 +4D0/+4D4/+330; direct mesh-tolerance handler +338. Version >0x2F69 then dispatches mgConformalDeform_c::Serialize at +90. Version >0x3B0B adds seven i32 words +484,+488,+48C,+494,+498,+49C,+4A4; version >0x42CC adds i32 +490/+4A0. Earlier nested-field gates precede scope. The direct suPtrArray and conformal-deformation helper bodies require further native verification before this sequence becomes a complete executable payload grammar.

The mesh-tolerance primary 0x4D110670 / COL 0x4D3523D8 selects 0x4BB2E410. Its ordinary body is exactly 40 bytes: i32 +08, f64 +10/+18/+20, i32 +0C, f64 +28, after the zero-byte ordinary su_CObject base. No field gate occurs in that helper. Stored values remain semantically unresolved; this helper is retained as native evidence and is not yet registered by the payload reader.

The scoped outer reader therefore deliberately has no new-payload fallback or opaque skip. Two focused tests verify all outer gate boundaries, cached pointer identity and ordering, signed limits, following-byte alignment, exact tagged self-map accounting, every truncated payload prefix, context rejection and the explicit unsupported nested-class stop. Complete native move/copy endpoints and additional whole-document coverage are not claimed.

Evidence: C-245.

### 11.218 Move/copy nested data, mesh tolerance and zero conformal branch

*Status: B / ? — two populated payloads verified inside complete native containing roots; raw meanings, nonzero multivectors, conformal function objects and special transforms remain partial.*

This extends and supersedes the unresolved nested-data boundary in 11.217 for ordinary non-DB external-XML archives with version >0x27CB. The existing moMoveCopyBodyData_c primary/factory/constructor binding is unchanged. Its executable sequence is:

| # | Field | Type | Presence | Description and rule | St. |
|---|---|---|---|---|---|
| 1 | references +3D8,+3E0,+3E8 | three tagged moRefWrapper_c | always | Preserve pointer identity in order. | B |
| 2 | raw +328 | f64 | always | Stored scalar. | B / ? |
| 3 | array +370 | direct suObArray | always | ReadCount then tagged objects; constructor-bound direct member. | B |
| 4 | transform +08 | 30 u32 | ordinary fixed-transform kind | Archive mgXform operator, distinct from variable restore grammar; stop if word 28 &0xF0 is nonzero. | B / ? |
| 5 | point +2D8; raw +320 | 3 f64; i32 | always | Point then raw signed word. | B / ? |
| 6 | vectors +2F0,+308 | two × 3 f64 | always | Preserve both vectors. | B / ? |
| 7 | raw +368,+36C | two i32 | always | Stored words. | B / ? |
| 8 | pointer array +3F0 | direct suPtrArray | always | Zero bytes in the supported ordinary archive mode; not a counted collection. | B |
| 9 | stock +3D0; raw +410 | tagged moStock_c; i32 | always in scope | Target followed by signed word. | B / ? |
| 10 | vectors +418,+430,+448 | three × 3 f64 | always in scope | Stored vectors. | B / ? |
| 11 | reference +478; raw +4C0 | tagged moRefWrapper_c; i32 | always in scope | Pointer then raw word. | B / ? |
| 12 | plane +4C8; raw +4D0,+4D4,+330 | tagged moPlaneRef_w; three i32 | always in scope | Preserve stored values before historical native fixups. | B / ? |
| 13 | mesh tolerance +338 | direct 40-byte body | always in scope | i32 +08; f64 +10,+18,+20; i32 +0C; f64 +28. | B / ? |
| 14 | conformal deformation +90 | direct conditional body | version >0x2F69 | Zero-multivector branch below; unsupported branches stop. | B / ? |
| 15 | raw +484,+488,+48C,+494,+498,+49C,+4A4 | seven i32 | version >0x3B0B | Exact native order; +490/+4A0 are not interleaved. | B / ? |
| 16 | raw +490,+4A0 | two i32 | version >0x42CC | Late signed words. | B / ? |

suPtrArray primary 0x3CDE62A8 selects thunk 0x3CD606CF in sldmfcu.dll, which resolves through IAT 0x3CDC94E8 to swccu.dll su_CObject::Serialize. The ordinary archive flag 0x10-clear path returns without reading fields. It is therefore incorrect to insert a count or serialize runtime pointers for row 8. Other archive modes are outside scope.

The exact conformal import resolves to sldmgu.dll export 0x321A6DC0. Its first multivector uses operator 0x32188370 and base serializer 0x3218A4C0. A multivector begins with f64 scale: zero (including negative zero) ends that multivector. A nonzero scale selects a signed word, subtype selector and subtype-dependent body; these branches remain explicit stops. After the first multivector, conformal serialization reads a raw u32 flags word (+1B8). Bit 0x100 selects a function-object body and f64 +1C0, still unsupported. If the first multivector is nonzero or bit 0x200 is set, a second multivector (+30) follows. Bit 0x200 also selects a third (+60). The executable reader supports zero first/second/third multivectors and preserves the stored flag word rather than native clearing of selector bits. It does not infer a nonzero multivector's isZero predicate from scale alone.

The complete empty data body with null/cached tagged references is 358 bytes through 0x2F69, 370 through 0x3B0B, 398 through 0x42CC and 406 thereafter. Zero multivectors selected by bit 0x200 add 16 bytes. New tagged targets and populated arrays contribute their normal bodies/map slots. Mesh tolerance is also scoped under its own class name.

Five focused tests now cover outer and nested version boundaries, signed-word limits, point/vector/transform retention through native source equality, direct arrays, zero-byte pointer-array framing, mesh order, negative-zero multivectors, flags/unsupported branches, exact tagged self identity and following-byte alignment, every truncated empty data prefix and context rejection. Source-hash-bound regressions reproduce both native containing-root spans and all retained payload scalars, vectors, fixed-transform words, array count, mesh/conformal data and target classes.

The audit contains **two complete move/copy features and two payloads**, each with one direct-array item, zero mesh words/scalars, first multivector scale zero and conformal flags zero. Across 906 complete modern contexts there are now **24,305 complete roots**, six more than Edition 0.223. **765 complete lists**, **141 explicit stops**, **692 exact-EOF lists / 46,159,654 decoded bytes**, and **73 four-byte unknown tails** are unchanged. The affected configurations now reach moCompCoordSys_c or later unsupported features. `resolved-feature-list-frontier-0.223.json` retains the prior checkpoint. Native populated unsupported branches and complete part/assembly/drawing/all-installed-project bit coverage remain unfinished; whole-document coverage credit is unchanged.

Evidence: C-246.
