### 14.461 PostInfo fixed-block partition and named cut-primitive fields

*Status: stored block structure and accessor-level field roles verified; enum domains, units and runtime interpretation remain partial — Evidence: C-841*

The460-byte PostInfo block need not remain one undifferentiated span. Installed **DGXPOST.dll:180041F70** (`CDspPostInfo` constructor) initializes twelve **CDspCutPrim** objects: six at **1C + i*24 hexadecimal**, then six at **F4 + i*24 hexadecimal**, for i=0–5. Their36-byte stride and28-byte leading header partition exactly **28 +12*36 =460** bytes. Constructor **180040220** is exported as `CDspCutPrim::CDspCutPrim`; its last initialized member is+20, and there is no serialized pointer/tag inserted between records. Direct `CDspPostInfo::WriteToStream` at **180045520** writes exactly0x1CC bytes from the object address, confirming the member offsets coincide with offsets within this raw stored block.

Named methods establish group roles. **SetSectionsDefault,180045330**, uses header word **14** as the section-record count and addresses records in the first group. **SetIsoDefault,1800451E0**, uses header word **18** for active iso count and records starting atF4; it also initializes remaining iso slots up to six. All six slots in each group remain physically present regardless of these counts. The first five header words at00/04/08/0C/10 retain raw identities without complete domain meanings. No array length is changed or memory-safety behavior inferred from out-of-range stored count bits.

The named accessors map each36-byte cut primitive as follows:

| Relative offset | Width | Verified role / view |
|---|---:|---|
|00 |4 | Primitive type, returned by `GetPrimType`. |
|04 |4 | Cut-direction enum, returned by `GetCutDirection`; value domain unresolved. |
|08/0C/10 |3*4 | Normal components, copied by `GetNormal`/`SetNormal`. |
|14/18/1C |3*4 | Center components, copied by `GetCenter`/`SetCenter`. |
|20 |4 | Shared float scalar returned by both `GetIso` and `GetRadius`; `GetD` returns its sign-flipped float view. |

Constructor/default float stores support float32 component views; original component bits remain authoritative. No units, coordinate frame or normalization invariant is assumed. `SetIso` at **180041300** writes+20 only when primitive type equals **3**, binding that type to the iso setter route. This does not assign all remaining primitive-type codes. The D getter at **180040AB0** loads+20 with MOVSS then XORs with the exact sign mask at **1802D7C00**; low scalar bits are XORed with **0x80000000**. Thus D is not a separately stored field. Iso/radius getters share address **180040AC0** and return the unmodified scalar. Context determines which view is meaningful; the decoder does not simultaneously impose iso/radius/plane geometry semantics.

`cosworks_post_info_fields.py` retains all header/record raw bytes, supplies signed integer display views, float32 views for finite values and the D sign-flip bit view. NaN/infinite values retain exact raw bits with no misleading finite numeric view. Unknown enum words are preserved, no normalization/default mutation is emulated, and raw header plus all twelve raw records reconstructs exactly the input block.

C-841 decodes all **60** retained standalone460-byte blocks (**27,600 bytes**) into **720** record slots. Stored section and iso active-count words both equal **1** in every sample; all360 first-group records have type1 and all360 second-group records type3. Those are observations, not a generalized enum or activation rule. The prior caller distinction remains unchanged: **52** blocks have a retained below700335-compatible argument; **eight** have helper-level framing only. This field refinement does not remove their context qualification or create new stored-byte credit.

The audit verifies exported method names/addresses, constructor/default method hashes/decompilations, thirteen exact leaf function ranges with raw instructions, sign-mask bytes and original native hashes. Five tests pass, including existing PostInfo framing plus two field tests for record partition, finite and NaN bit preservation, getter sign alias and length checks. Physical meanings of unknown header words, enum domains, actual activation, historical producer/runtime equivalence and full installed-format coverage remain unfinished. No target code executes and global semantic coverage credit remains zero.

Evidence: C-841.
