### 11.247 Sheet-metal corners, hems and edge chains

*Status: B / ? — nine native bindings, four populated containing feature roots and two additional exact-EOF lists verified; nonnull native relief data, helper-state-1 native fixtures and engineering domains remain partial.*

`moCornerFeat_c` selects serializer `0x3DB5DA40` in the hash-bound installed `sldsmmu.dll`. It calls established bend-group serializer `0x3DB68EC0`. For ordinary external-XML modern archives (version `>0x27CB`), the derived body stores:

1. Two tagged `suObList` pointers, at runtime `+0x890,+0x898`.
2. i32 values `+0x8A0,+0x9A0`.
3. Binary64 values `+0x948,+0x950`.
4. i32 values `+0x958,+0x940,+0x95C`.
5. A direct `suObList` at `+0x8B0`, with its own `ReadCount` and tagged entries.

The two initial lists are tagged objects; the final direct list adds no list-object tag or map slot. All historical field gates precede the supported range: second tagged list/word `>=0x104C`, word `+0x9A0 >0x6A2`, doubles `>0xB21`, word `+0x958 >0xBE4`, word `+0x940 >=0x10FA`, word `+0x95C >0x1315`, and direct list `>0xAAC`. The pre-`0xE76` masking of `+0x958` is absent in supported modern files.

Native loading normalizes stored `+0x9A0` value 0 to runtime value 2; all other i32 values pass through. The decoder retains `raw_9a0` and exposes separate `loaded_9a0`, preserving the original bytes. The constructor identifies this member's parameter type as `enumCornerType_e`; value names and valid engineering combinations remain unresolved. Runtime references derived from the first elements of the tagged lists do not add wire fields. Constructor `0x3D867160` proves direct list `+0x8B0`; its separate runtime list at `+0x8F8` is not serialized by this body. With null/mapped tagged lists and an empty direct list the suffix occupies **42 bytes**.

`moFaceGeom_c` selects `0x3DB5F860`: direct point at runtime `+0x08`, then direct vector at `+0x20`, each containing three binary64 components. The body is exactly **48 bytes**, without count or presence fields. `moPlanarFaceGeom_c` selects the five-byte leaf at `0x3DB659D0`, whose bytes `E9 8B 9E FF FF` jump to `0x3DB5F860`. It adds no suffix. The leaf lacks an unwind-table entry; its exact instruction bytes and target are retained independently. Point/vector units and geometric validity remain open.

`moHem_c` selects `0x3DB62C40`, using the same inherited bend-group base. It serializes direct `suObArray` `+0x9A8`, then one **direct owned `moHemData_c` body** associated with pointer member `+0x880`, then i32 `+0x9C8`. The last word's gate (`>0x722`) is satisfied throughout the supported range. On loading, nonnull array references are used at runtime, the owned data object is allocated and its serializer is called directly. No tagged data header or data-object map slot is inserted. Constructor `0x3D9619B0` proves the array type; the embedded data copy at `+0x888`, separate common-bend member at `+0x968` and runtime pointer arrays are not appended as further bodies.

`moHemData_c` selects `0x3DB62760`. It calls established `moFeatureData_c` serializer `0x4B50C220`, which stores a tagged feature-owner pointer at `+0x08` and checks feature ancestry on native loading. The diagnostic reader requires a nonnull owner and preserves the actual pointer. Its modern body then stores:

1. Direct `suObArray` `+0x20`.
2. Tagged `moSheetMetal_c` pointer `+0x18` only for sheet-metal helper state `+0x850` equal to 0; helper state 1 omits this pointer.
3. Tagged distance dimensions `+0x40,+0x48`, then binary64 `+0x60`.
4. i32 values `+0x80,+0x14,+0x10`, in that wire order.
5. Tagged distance dimension `+0x78`, i32 `+0xD0`, then tagged `moReliefData_c` pointer `+0xD8`.
6. Direct `suObArray` `+0x90`.

The two data arrays and the feature array have independent counts. Constructor `0x3D9610B0` proves both serialized data-array types; its additional `+0xB0` array is not a wire field here. The modern body occupies **40 bytes** with empty arrays and null/mapped pointers in helper state 0, or **38 bytes** in helper state 1. The minimum hem suffix including feature array and final word is therefore 46 or 44 bytes, excluding the inherited bend group and new pointed-object bodies. Historical temporary arrays/conversions at or below `0x1054`, the pre-`0x7FE` extra double and pre-`0x723` numeric gap branch are outside the supported range. Helper states other than 0 or 1 stop explicitly.

`moHemEdgeChainData_c` selects `0x3DB62AB0`: tagged `moHemEdgeChain_c` pointer `+0x08`, tagged profile `+0x10`, i32 `+0x28`, then tagged component-edge pointer `+0x60`. The last pointer's gate (`>0x10F4`) precedes the supported range. This body occupies **10 bytes** with mapped/null pointers.

`moHemEdgeChain_c` selects `0x3DB62C00` and serializes only a direct `suObArray` at `+0x08`. Runtime descriptor `0x3DDAF440` binds its factory to `0x3DB562F0`, which allocates 0x40 runtime bytes, installs the class vtable and constructs the array with `suObArray` vtable. The factory also initializes runtime words `+0x28,+0x2C` and pointers `+0x30,+0x38`; its serializer does not store them. An empty chain occupies only the two-byte direct-array count. `moHemEdgeChainElement_c` selects `0x3DB62BA0` and stores one tagged component-edge pointer at `+0x08`, with no suffix; a mapped/null element body is two bytes.

`moReliefData_c` selects `0x3DB65B50`: i32 values `+0x08,+0x0C`, binary64 values `+0x10,+0x18,+0x20`, then i32 `+0x28`. This fixed body occupies **36 bytes**. Its engineering option meanings remain open. All three native hem relief pointers are null; the nonnull relief body is verified from native instructions and synthetic scalar-order/bit-preservation tests, without claiming a populated native relief fixture.

The complex sheet-metal sample supplies one **2,946-byte corner root** with two tagged lists containing component faces and a direct list containing planar-face geometry. It also supplies one **19,509-byte hem root**. Hinge 1 and Hinge 2 supply hem roots of **165,019** and **167,031 bytes**, giving **351,559 hem bytes** and **354,505 bytes across all four feature roots**. Within these completed hem roots are **26 fresh chain-data records, 26 chains and 26 elements**. Native data-array counts are 12/12, 13/13 and 1/1; independent feature-array counts are 12, 13 and 2. All owned data pointers refer back to their containing hem feature through the inherited owner tag, without creating a fresh tagged `moHemData_c` entry.

The 906-context audit now contains **25,478 complete roots**, **823 complete lists** and **83 explicit stops**, with every prior complete root record unchanged. This adds 20 roots across three configurations. Hinge 1 and Hinge 2 now reach exact EOF (289,165 and 227,677 decoded bytes), adding **516,842 bytes**. The complex sample advances to `moJog_c`. Totals are **745 exact-EOF lists / 61,424,074 decoded bytes** and **78 complete lists with four-byte unknown tails**. Checkpoints `resolved-feature-list-frontier-0.266.json` and `resolved-feature-list-frontier-0.267.json` retain the comparison. Framing completion earns no whole-document semantic coverage credit.

Six focused tests verify tagged versus direct lists, shared identity, zero-to-2 normalization without overwriting raw values, binary64 bits, both hem helper paths, independent array counts, direct owned-data composition, every chain/relief body width, truncation/context rejection and all four hash-bound native containing roots through two complete lists and the remaining jog stop. Historical/DB modes, native helper-state-1 and nonnull relief fixtures, raw engineering domains and complete part/assembly/drawing/all-installed-project bit coverage remain open.

Validation: all **905 regression tests pass** (91.679 seconds); specification lint reports zero findings (471 headings, 96 anchors).

Evidence: C-289.
