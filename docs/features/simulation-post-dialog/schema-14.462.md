### 14.462 Cut-primitive type meanings and direction display tokens

*Status: known primitive-type meanings and display tokens bound to installed code; unknown codes and coordinate semantics remain qualified — Evidence: C-842*

Named factory **DGXPOST.dll:180041080**, `CDspCutPrim::GetPrim`, resolves the stored type word at record+00 through concrete exported constructors. C-842 checks those exports against their installed addresses rather than naming types from native value frequencies:

| Stored type | Bound construction / parameter sources |
|---|---|
|0 | `CDspCylinder` constructor **1800E6290** receives center at+14, normal vector at+08 and float scalar+20. |
|1 | `CDspPlane` constructor **1800E6410** receives normal at+08 and the sign-flipped scalar+20, consistent with GetD in14.461. |
|2 | `CDspSphere` constructor **1800E6610** receives center at+14 and float scalar+20. |
|3 | Iso route established by the SetIso type3 check at180041300; the factory's remaining branch builds an iso primitive using scalar+20. |

These are view/constructor meanings, not new wire fields. Center/normal/scalar data retain the36-byte layout and original float bits from14.461. The table does not infer units, axis normalization, radius validity or a plane equation beyond the exact constructor arguments. Cylinder and sphere records have static/synthetic type-binding evidence here, without a populated native slot of either type in this retained PostInfo set.

The factory's native last branch also handles type words other than0/1/2. That fallback must not be promoted into a claim that every unknown code is a defined iso enum value. The independent decoder names only **0=cylinder,1=plane,2=sphere,3=iso**; unknown raw type words remain preserved with no inferred name.

`CDspCutPrim::GetString`, **180041180**, maps the cut-direction word at+04 to literal display tokens:

| Stored direction | Exact installed token / literal |
|---|---|
|-1 | **Yon**, UTF-16 at1802D7B60 |
|1 | **Hither**, UTF-16 at1802D7B50 |
|2 | **Both**, UTF-16 at1802D7B68 |

The native display function falls back to Hither for other values. The decoder preserves unknown direction values without relabeling them as a defined1; display fallback is not a validation rule. These tokens establish current presentation names, not which geometric side, world axis or screen direction the enum denotes in every runtime context.

`cosworks_post_info_fields.py` now includes known primitive names and known direction display tokens alongside the existing raw/signed views. C-842 replays all **60 blocks /720 slots** with this annotation. The360 first-group slots are type1/plane with direction1/Hither; the360 second-group slots are type3/iso with direction-1/Yon. The prior52 caller-compatible versus eight helper-only block distinction remains unchanged. No record is rewritten, no fallback is applied to stored data and no additional stored-byte/global coverage credit is inferred from annotation.

Seven tests pass: existing block framing/field tests plus two new tests covering every known type/direction combination and preservation of unknown type/direction words. The audit retains module/native payload hashes, factory/display instruction and decompilation hashes, exact constructor export names/addresses and direction literal bytes. Unknown header words, remaining enum semantics, physical interpretation, runtime/writer equivalence and the full installed-format objective remain unfinished. No target code executes.

Evidence: C-842.
