### 11.286 Version-18000 owned-manager frontiers

*Status: Partial · ten source-bound legacy frontiers; native tail gates reviewed · Evidence: C-328*

The edition 0.305 audit retains ten `unsupported legacy configuration-manager owned layout` stops. `configuration_legacy_owned_frontiers.py` replays those exact files, checks the decompressed manager source hashes and prior stop cursors, and confirms that **all ten use archive version 18000**. Five have **62 bytes** after the owned-manager stop, four have **6,057 bytes**, and one has **12,035 bytes**. These are remaining stream spans, not proven complete owned-manager body lengths.

Native manager Serialize **0x4C22C140** has more specific gates than the current conservative owned-stage minimum:

| Field/group | Native input gate | Version 18000 |
| --- | --- | --- |
| direct decal data +0x6D8 | version > 0x2399 | present |
| direct scene data +0x6E0 | version >= 0x23A5 | present |
| state +0x858 | version > 0x42E8 | present |
| final display-map presence +0x898 group | version > 0x46C1 | absent |

The scene serializer **0x4C6E84C0** reads field of view +0x2B0 only when version > **0x46BA**. That field is absent at version 18000, while the earlier HDR-exposure/tone-map fields gated above 0x42EC are present. Therefore the modern scene and manager suffixes cannot be reused at version 18000 merely by removing their early scope checks: the final scene f64 and manager display-presence u32 must follow their own native gates.

The five 62-byte remaining spans are consistent with the modern empty owned-manager tail lacking its final four-byte display presence, but this is a boundary observation rather than independent proof of a full older layout. The other five previews contain nonempty scene-map framing, which requires version-aware scene bodies as well as the manager suffix. No scene body count or completion claim is inferred from those short previews.

`configuration_legacy_owned_evidence.py` revalidates the exact installed manager and scene serializer bytes behind the reviewed gates and binds the ten-file inventory hash in `configuration-legacy-owned-native-evidence.json`. The existing scope rejection remains in production. Version-18000 reader integration, native scene/map replay and corpus regrading are required before any of these streams can receive credit. Measured coverage and full document/project completion status remain unchanged.

Evidence: C-328.

### 11.287 Scoped version-18000 owned-manager completion

*Status: Partial · ten additional complete managers and six older scene bodies · Evidence: C-329*

The owned-manager, scene-map and scene-body readers now accept an explicit **version-18000** case in addition to their existing versions above 0x46C1. `read_supported(doc)` defaults to **`legacy_owned=True`**, while **`legacy_owned=False`** retains the edition 0.305 owned-stage rejection. Prefix callers use **`legacy_owned_records=True`** together with the desired owned/scene depth. Direct reader calls require the matching archive context flag. Other unverified older versions remain rejected before owned-stage bytes are consumed; this is not a blanket removal of the legacy scope checks.

Two native field gates are applied separately. The manager records `display_maps_field_present=False` and `display_maps_presence=None` at version 18000, consuming **no final display-presence u32**. The scene records `field_of_view_present=False` and `field_of_view=None`, consuming **no final field-of-view f64**. At the existing modern versions these fields remain present and are read normally. An absent field is not represented as a serialized zero. Earlier HDR-exposure/tone-map fields and manager state +0x858 remain present at version 18000, as established by their independent native gates in 11.286.

`configuration_legacy_owned_supported.py` replays the full **919-part** audit and the ten source-bound older frontiers. All ten now complete, and all previous **822** positive summaries remain unchanged. The older scene-map cases contain **six scene bodies totaling 35,757 bytes**, with **17,220 unknown bytes**. These scene intervals are contained within the complete manager streams and are not separately added to file coverage.

`configuration-manager-supported-0.307.json` records **832 complete supported managers / 87 explicit stops**. New complete streams add **124,901 decoded bytes**, split into **58,706 known structure bytes** and **66,195 unknown bytes**. Cumulative complete streams total **8,342,618 bytes**: **3,529,055 known** and **4,813,563 unknown**. Exact EOF and guarded postload completion establish the supported stream framing, not complete semantic document coverage.

The versioned coverage regrade adds **13,235 stored part bytes** beyond edition 0.305. Cumulative CMgr credit becomes **750,951 bytes**, leaving **1,115,161 stored CMgr bytes** unknown in the unchanged **1,866,112-byte** denominator. Total part unknown file bytes become **68,493,778**. Rounded coverage remains **69.8% / 63.3%** for parts, 83.8% / 81.3% for assemblies and 57.3% / 49.9% for drawings. Installed project formats remain outside this measured container-corpus denominator.

`test_configurationlegacyowned.py` reproduces the full audit and all six scene intervals, checks explicit absence metadata, confirms that disabling the option retains the historical stop, and rejects unverified neighboring versions/context before input. `configuration_legacy_owned_supported_evidence.py` revalidates the native manager/scene bytes behind the version gates and binds the audit, corpus and coverage hashes. Historical replay scripts explicitly disable the new scope when preserving older frontiers. Remaining version/context and class dependencies, raw field meanings and full part/assembly/drawing/all-installed-project semantic bit coverage remain open; no whole document is declared complete.

Validation: all **980 regression tests pass** (139.869 seconds); specification lint reports zero findings (511 headings, 96 anchors).

Evidence: C-329.
