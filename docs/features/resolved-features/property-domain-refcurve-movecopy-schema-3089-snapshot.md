### 11.218 Move/copy nested data, mesh tolerance and zero conformal branch

*Status: B / ? — two populated payloads verified inside complete native containing roots; raw meanings, nonzero multivectors, conformal function objects and special transforms remain partial.*

This extends and supersedes the unresolved nested-data boundary in 11.217 for ordinary non-DB external-XML archives with version >0x27CB. The existing moMoveCopyBodyData_c primary/factory/constructor binding is unchanged. Its executable sequence is:

| # | Field | Type | Presence | Description and rule | St. |
|---|---|---|---|---|---|
| 1 | references +3D8,+3E0,+3E8 | three tagged moRefWrapper_c | always | Preserve pointer identity in order. | B |
| 2 | raw +328 | f64 | always | Stored scalar. | B / ? |
| 3 | Surfaces array +370 | direct suObArray | always | ReadCount then tagged objects; named constructor-bound direct member. | B |
| 4 | transform +08 | 30 u32 | ordinary fixed-transform kind | Archive mgXform operator, distinct from variable restore grammar; stop if word 28 &0xF0 is nonzero. | B / ? |
| 5 | point +2D8; raw +320 | 3 f64; i32 | always | Point then raw signed word. | B / ? |
| 6 | vectors +2F0,+308 | two × 3 f64 | always | Preserve both vectors. | B / ? |
| 7 | raw +368,+36C | two i32 | always | Stored words. | B / ? |
| 8 | pointer array +3F0 | direct suPtrArray | always | Zero bytes in the supported ordinary archive mode; not a counted collection. | B |
| 9 | stock +3D0; raw +410 | tagged moStock_c; i32 | always in scope | Target followed by signed word. | B / ? |
| 10 | vectors +418,+430,+448 | three × 3 f64 | always in scope | Stored vectors. | B / ? |
| 11 | reference +478; AbsoluteXform +4C0 | tagged moRefWrapper_c; i32 | always in scope | Pointer then full signed32 property word. | B / ? |
| 12 | plane +4C8; raw +4D0,+4D4,+330 | tagged moPlaneRef_w; three i32 | always in scope | Preserve stored values before historical native fixups. | B / ? |
| 13 | mesh tolerance +338 | direct 40-byte body | always in scope | i32 +08; f64 +10,+18,+20; i32 +0C; f64 +28. | B / ? |
| 14 | conformal deformation +90 | direct conditional body | version >0x2F69 | Zero-multivector branch below; unsupported branches stop. | B / ? |
| 15 | raw +484,+488,+48C,+494,+498,+49C; EnableEulerAngles +4A4 | seven i32 | version >0x3B0B | Exact native order; +490/+4A0 are not interleaved. | B / ? |
| 16 | raw +490,+4A0 | two i32 | version >0x42CC | Late signed words. | B / ? |

Exact own-class moMoveCopyBodyData_c getAbsoluteXform/setAbsoluteXform read and write the full current DWORD 0x4C0. getEnableEulerAngles returns the full current DWORD 0x4A4; only that getter association is established here. Both saved fields use signed32 archive operators. Separate unsigned version checks admit 0x4A4 at >=0x3B0C on the selected STORE and LOAD paths. Their names do not establish Boolean domains, an absolute coordinate frame, an Euler-angle convention or angular units.

Named setStockFeature takes a declared moStock_c pointer and writes original member 0x3D0, matching the existing typed stock archive occurrence. This is a setter checkpoint, not a guarantee of ownership, lifetime or final archive identity.

The exact Surfaces methods bind the embedded array at 0x370. getSurfaces invokes typed su_CObArray::SetSize(0,-1,1) on the caller output collection, then Copy from current member 0x370. setSurfaces invokes the same SetSize on current member 0x370, then Copy from the caller input. These are ordered provider operations, not an atomic update or a proof of deep-copy semantics, safe self-aliasing, item classes or successful body modification. Existing array serialization and opaque callback qualifications remain unchanged. Runtime-only X/Y/ZDistanceSetByAPI words 0x4A8/0x4AC/0x4B0 are not added to this wire body. Evidence: C-3535.

suPtrArray primary 0x3CDE62A8 selects thunk 0x3CD606CF in sldmfcu.dll, which resolves through IAT 0x3CDC94E8 to swccu.dll su_CObject::Serialize. The ordinary archive flag 0x10-clear path returns without reading fields. It is therefore incorrect to insert a count or serialize runtime pointers for row 8. Other archive modes are outside scope.

The exact conformal import resolves to sldmgu.dll export 0x321A6DC0. Its first multivector uses operator 0x32188370 and base serializer 0x3218A4C0. A multivector begins with f64 scale: zero (including negative zero) ends that multivector. A nonzero scale selects a signed word, subtype selector and subtype-dependent body; these branches remain explicit stops. After the first multivector, conformal serialization reads a raw u32 flags word (+1B8). Bit 0x100 selects a function-object body and f64 +1C0, still unsupported. If the first multivector is nonzero or bit 0x200 is set, a second multivector (+30) follows. Bit 0x200 also selects a third (+60). The executable reader supports zero first/second/third multivectors and preserves the stored flag word rather than native clearing of selector bits. It does not infer a nonzero multivector's isZero predicate from scale alone.

The complete empty data body with null/cached tagged references is 358 bytes through 0x2F69, 370 through 0x3B0B, 398 through 0x42CC and 406 thereafter. Zero multivectors selected by bit 0x200 add 16 bytes. New tagged targets and populated arrays contribute their normal bodies/map slots. Mesh tolerance is also scoped under its own class name.

Five focused tests now cover outer and nested version boundaries, signed-word limits, point/vector/transform retention through native source equality, direct arrays, zero-byte pointer-array framing, mesh order, negative-zero multivectors, flags/unsupported branches, exact tagged self identity and following-byte alignment, every truncated empty data prefix and context rejection. Source-hash-bound regressions reproduce both native containing-root spans and all retained payload scalars, vectors, fixed-transform words, array count, mesh/conformal data and target classes.

The audit contains **two complete move/copy features and two payloads**, each with one direct-array item, zero mesh words/scalars, first multivector scale zero and conformal flags zero. Across 906 complete modern contexts there are now **24,305 complete roots**, six more than Edition 0.223. **765 complete lists**, **141 explicit stops**, **692 exact-EOF lists / 46,159,654 decoded bytes**, and **73 four-byte unknown tails** are unchanged. The affected configurations now reach moCompCoordSys_c or later unsupported features. `resolved-feature-list-frontier-0.223.json` retains the prior checkpoint. Native populated unsupported branches and complete part/assembly/drawing/all-installed-project bit coverage remain unfinished; whole-document coverage credit is unchanged.

Evidence: C-246.

### 11.338 Reference-curve body and embedded array branches

*Status: Native-bound research reader; eleven source objects; no supported coverage credit · Evidence: C-380*

The primary `moRefCurve_c` vtable in the C-20 model DLL is `0x4D164450`, with Serialize at `0x4C282CF0..0x4C2839F7`. Its constructor `0x4C2A9DC0..0x4C2A9F1E` proves the directly owned collection types: DWORD arrays at +888/+8A8/+8F0/+910, `suObArray` at +8C8 and `suObList` at +948. Another DWORD array at +800 is constructed but is not serialized directly by this derived body. Constructor storage and runtime assignments must not be treated as extra wire fields.

The following ordinary modern layout follows `moBodyFeature_c::Serialize`. Numeric fields marked raw retain unknown meanings. Direct collections serialize their existing count and entries without outer object tags.

| Order | Wire field | Gate or interpretation |
|---|---|---|
| 1 | Tagged `suObArray` pointer +860 | Modern replacement for the pre-0xD5 single-face pointer |
| 2 | Existing kernel archive body +820 | Present after version 0xDF; strict existing kernel decoder |
| 3 | Direct `suObList` +948 | Version greater than 0x10C8 |
| 4 | Raw i32 node ID +828 | Version at least 0x215; runtime node lookup consumes no extra input |
| 5 | Raw i32 +87C, +880; projection flags i32 +878; direct DWORD array +888 | Version at least 0x329 |
| 6a | Direct DWORD array +8A8 | Bit 1 of +878 is set |
| 6b | Direct object array +8C8 | Bit 1 clear and the +860 array is nonnull and nonempty |
| 6c | Tagged `moCompRefPlane_c` +8E8 | Bit 1 clear, +860 null/empty, and version greater than 0x2B94 |
| 7 | Direct DWORD arrays +8F0 and +910 | Follow the selected branch |
| 8 | Direction i32 +930 | Version at least 0x333 |
| 9 | BlankFlag u16 +858, zero-extended into runtime DWORD | Version greater than 0x385 |
| 10 | Raw i32 +938 | Version at least 0x3BF |
| 11 | u16 body count; that many kernel bodies, each followed by a direct attribute list | Version greater than 0x2B87 |
| 12 | BiDir i32 +934; tagged plane +868 and direction +850 | Same gate as order 11 |
| 13 | Tagged `moCurveRef_w` +870 | Version greater than 0x3EFB |

Exact own-class moRefCurve_c accessors bind the existing signed32 members 0x934 and 0x930 to BiDir and Direction. Their getters return the entire current word and their setters write the entire integer argument, without Boolean normalization or a recovered numeric enum domain. These are named native properties; their geometric direction, units and successful projection behaviour are not established.

BlankFlag at runtime 0x858 also has full32 getBlankFlag/setBlankFlag accessors. Its archive checkpoint differs: STORE writes only the low unsigned16, and the selected LOAD zero-extends the unsigned16 into the runtime DWORD. setBlankFlag writes the raw integer before virtual slot 0x1C90 and later opaque operations. The stored value therefore does not promise a final effective visibility state or preservation across callbacks.

At current 0x878, isSketchFaceProjection returns word&1, whereas isSketchSketchProjection returns word&2, which is 0 or 2. The latter is not a normalized Boolean. Mask 0x2 also selects the existing 0x8A8 archive branch; this does not identify array-item meanings or close the remaining flag domain. Raw 0x87C, 0x880, 0x938 and residual components retain their qualifications. These own-class property joins add no wire fields and do not execute native setters during offline decoding. Evidence: C-3534.

The modern ordinary context resolves the historical prefix gates, while retaining the three later version comparisons. Historical runtime repairs, entity lookups, attribute attachment and model assignment add no input. This research reader does not emulate them or claim support for legacy bodies.

At the edition 0.358 checkpoint, `solidworks/research/refcurve.py` installed the reader only in its local replay archive and left public resolved-feature registration and byte credit unchanged. Clause 11.339 integrates the verified body into the guarded public path. Across four installed Config-0 sources, the reader completes **11** reference-curve objects. `Simulation Examples/Verification/comp_buck.sldprt` reaches exact EOF at **33,236 bytes / 25 roots**, with 11,743 unknown bytes; `tutorial/api/hemisphere.sldprt` reaches exact EOF at **49,122 bytes / 39 roots**, with 16,421 unknown bytes and eight reference curves. `learn/Mouse/Mouse Master.SLDPRT` completes its curve and 29 roots before stopping at `moBlendRefSurface_c`; `tutorial/molds/telephone.sldprt` completes its curve and 26 roots before stopping at `moCompRefCurve_c`. Failure cursors are diagnostics, not credited endpoints.

Observed +878 values are 1 and 2. The flag-2 source uses the +8A8 branch; flag-1 sources use nonempty +860 arrays and direct +8C8 objects. All eleven extra-body counts are zero. Positive extra-body counts and the empty-reference +8E8 branch are not source-observed. Focused synthetic tests verify all three array/plane branch boundaries, a following sentinel, truncation and the version-19000 suffix. These checks do not establish complete runtime loading or all kernel-body representations.

`refcurve_evidence.py` verifies the installed module SHA-256, primary RTTI binding, complete native function ranges/instructions, imported collection constructors and all four source replays. `refcurve-native-evidence.json` binds those results to the reader and retained decompilation hashes; `refcurve-corpus.json` retains source hashes, completed root endpoints, array counts and explicit next stops. Two focused tests pass. The preceding 1,107-test regression result remains the production baseline; no production reader changed in this edition. Full corpus integration, bounded unknown accounting and every-source stored revalidation are still required before adding coverage. Annex G remains at edition 0.357, and full document/all-installed-project schemas remain unfinished.

Evidence: C-380.
