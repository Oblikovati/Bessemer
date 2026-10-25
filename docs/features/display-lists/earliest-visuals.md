# Implicit visual properties

Display versions 4100 through `0x10AE` store single visual-property pointers for faces, bodies and part models. Their wire representation contains no collection count or key. Bessemer preserves the stored pointer and its identity without inventing either field. Later versions retain counted maps, including duplicate keys and null pointers.

The owned corpus contains 44 complete part and assembly roots across the implicit-map boundary and adjacent version gates. It includes null pointers and edited payloads. Independent reference traces cover the material, scene, item and component fields; forced reconstruction checks the complete bytes. A further 32 shared-property roots exercise holder state and reference identities. Exact aggregate budgets account for the four surface-parameter elements absent through `0x1098`.

Four complete containers exercise document context resolution at versions 4100 and 4200. The earlier 4200 fixtures remain unchanged.

Before `0x1052`, rendering records omit color-form and reflectivity fields, and visual properties retain `unknown_old_5c` instead of `default_appearance`. Both sides have independently generated roots and reject construction with the other layout.

Viewer display-state words begin after `0x10AE`, table header words after `0x10B2`, material color mixing after `0x10A1`, and original illumination parameters after `0x10BF`. Holder state words begin at `0x106A`. The fixture observer changes only reference map-reading decisions; embedded facet decoding retains the actual display version.

This checkpoint does not establish complete display-list parity. Non-null legacy `moPWMaterialProperties_c` payloads remain unsupported because the reference does not resolve their body layout. Earlier display versions and the documented uncounted-window interval remain excluded.

Regenerate with `generate_earliest_visuals.py` and check isolated reconstruction with `verify_earliest_visuals.py`, passing the built `rebuild_display_archives` executable. Integration coverage is in `tests/display_earliest_visuals.rs`.

The humanizer skill was unavailable; the prose pass remains for review.
