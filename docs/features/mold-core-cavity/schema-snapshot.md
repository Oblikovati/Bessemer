### 11.1318 Mold core/cavity solids native serializer binding

*Status: static native binding — Evidence: C-3002*

RTTI binds `moMoldCoreCavitySolids_c` to primary serializer `0x180174E40`. Its ancestry includes `moBodyFeature_c`, `moAtomFeature_c` and model-feature inheritance. The complete recovered serializer delegates to body-feature serialization, then reads or writes one tagged `moMoldCoreCavitySolidsData_c` pointer at member `+0x800`. No additional feature-specific archive fields follow that pointer in this function. Native bytes and retained `core-cavity-solids-decompilation-evidence.json` are SHA-bound. This binding alone advances no sample cursor and establishes no whole-feature completion.

Evidence `core-cavity-solids-native-evidence.json` retains static ancestry and serializer bytes. No target native code executes, production readers are unchanged, and coverage credit is zero. Full part-schema decoding remains incomplete and retains priority over assemblies.

### 11.1320 Mold core/cavity data native serializer binding

*Status: static native binding — Evidence: C-3004*

RTTI binds `moMoldCoreCavitySolidsData_c` to primary serializer `0x180174D10` with `moFeatureData_c` ancestry. The recovered order is feature-data base; direct virtual collections at `+0x70`, `+0xB8`, `+0x100`; tagged profile pointer `+0x50`; signed words `+0x30`, `+0x34`; two tagged display-distance pointers `+0x58`, `+0x60`; and tagged display-angular pointer `+0x68`. A final load-side function receives only the data receiver; it is not an additional archive field. Native bytes and `core-cavity-solids-data-decompilation-evidence.json` are retained. No concrete native container type is inferred solely from these virtual collection calls. This static binding alone advances no sample cursor.

Exact own moMoldCoreCavitySolidsData_c methods bind the following seven saved associations. The list getters add declared suObList reference contracts beyond the virtual serializer calls alone; they do not validate stored item classes or imply immutable snapshots.

| Member | Native property | Qualified saved association |
|---|---|---|
| 0x34 | SetbackSrf | Full 32-bit integer getter/setter and signed32 archive word |
| 0x50 | ProfileFeature | Declared moProfileFeature_c pointer getter/direct setter and tagged pointer codec |
| 0x70 | ListOfCoreBodies | Declared list-reference getter; existing direct virtual collection |
| 0xB8 | ListOfCavityBodies | Declared list-reference getter; existing direct virtual collection |
| 0x100 | ListOfPartingSurfaceBodies | Declared list-reference getter; existing direct virtual collection |
| 0x68 | Angle dimension | Current provider getter/setter, with runtime backup 0x38 |
| 0x58/0x60 | Distance dimensions at selected indices 0/1 | Current indexed provider getter/setter, with runtime backups 0x40/0x48 |

Angle uses dimension 0x68 virtual slot 0x868(0) when nonnull, otherwise runtime backup 0x38. Its setter stores the original input to that backup first. On the reached owner-nonnull/dimension-null path, creation uses a sign-cleared re-read of CURRENT backup 0x38; the later reached dimension update forwards the ORIGINAL input to virtual slot 0x860 with arguments 1,0.

Distance uses a sign-extended integer index with no local bounds check. Only indices 0/1 are joined here to the two existing saved distance pointers. It uses the selected dimension virtual slot 0x868(0) when nonnull, otherwise the corresponding runtime backup. The setter writes that backup first, and conditional dimension creation sign-clears the ORIGINAL preserved input in XMM2 rather than a re-read backup. Its later reached dimension update also forwards the ORIGINAL input through virtual slot 0x860 with arguments 1,0. Creation-time sign clearing is not normalization of a saved raw distance scalar.

The backup doubles 0x38/0x40/0x48 and PullDir member 0x18 are not additional fields in this direct suffix. Raw 0x30 remains unnamed. Current owner/storage validity, unchecked index preconditions, allocation outcomes, aliases, opaque providers and final postload processing remain qualified. No Boolean/enum restriction, units, frame, list-element domain, inverse getter/setter rule, stable archive/current value or successful mold construction follows. Evidence: C-3552.

Evidence `core-cavity-solids-data-native-evidence.json` retains static ancestry and serializer bytes. No target native code executes, production readers are unchanged, and coverage credit is zero. Full part-schema decoding remains incomplete and retains priority over assemblies.

### 11.1322 Mold core/cavity data owner and direct collections

*Status: measured legacy research boundary — Evidence: C-3006*

The feature-data owner reference resolves by identity to the enclosing partial core/cavity feature, a valid parent back-reference. Each direct collection at `+0x70`, `+0xB8` and `+0x100` has one complete `moCompSurfaceBody_c` in every advancing sample. The tagged profile reference resolves to a complete `moProfileFeature_c`. The following signed words `+0x30`, `+0x34` are1,0 and remain raw. Cursors600207,649884 and607400 stop before the first tagged display-distance dimension. None of the subsequent display pointers is skipped or treated as zero-byte bookkeeping.

Evidence `part-legacy-core-cavity-collections-evidence.json` frames6447 additional bytes with3300 charged unknown across three advancing streams. Four cohort rows and53 other first-child exclusions remain represented; aircraft sketch C0 stays deferred at32457. No target native code executes, production readers are unchanged, and coverage credit is zero. Full part-schema decoding remains incomplete and retains priority over assemblies.
