# Implicit visual properties

Display versions 3238 through `0x10AE` store single visual-property pointers for faces, bodies and part models. Their wire representation contains no collection count or key. Bessemer preserves the stored pointer and its identity without inventing either field. Later versions retain counted maps, including duplicate keys and null pointers.

The owned corpus contains 154 complete part and assembly roots across the implicit-map boundary and adjacent version gates. It includes null pointers and edited payloads. Independent reference traces cover the material, scene, item and component fields; forced reconstruction checks the complete bytes. A further 188 shared-property roots exercise holder state and reference identities. Exact aggregate budgets account for the four surface-parameter elements absent through `0x1098`.

Twenty complete containers exercise document context resolution at versions 3238, 3330, 3508, 3555, 3800, 3890, 3905, 4000, 4100 and 4200. The earlier fixtures remain unchanged.

Fifty additional complete roots cover all five old-light classes on both sides of the early state-word boundaries. `enabled_in_pv` starts after `0xCC0`; `keep_light_in_scene` starts after `0xD01`. Class identities, derived geometry, populated user properties and absent-versus-zero distinctions are checked independently.

Twelve additional display-item roots cover the signed 32-bit line-style and user-pen widths before `0xF41`. Font and visibility widths use signed 16-bit words in that interval. Polygon line properties also use the signed 32-bit form. Those callers store no custom float there. Full-range integer cases, absent custom fields, polygon flags and strict cross-version construction are checked independently.

Visual properties omit `hardcoded_facet_display` through `0xDB1`. Materials omit `use_new_shaders` through `0xDB3`. These are separate boundaries: `0xDB2` and `0xDB3` store the facet word but omit the shader word. Owned construction rejects synthetic zero words and incompatible material shapes while preserving unshaded roots across those boundaries.

Through `0xF1D`, material records omit both the RV presence word and its complete body, and texture records omit both translation doubles. At `0xF32`, embedded property font arrays grow from 15 to 23 records. Owned construction rejects cross-version field shapes and rejects a synthetic false RV word in a layout where the word is absent.

The earlier environment word starts after `0xF55`, embedded property fonts grow from 23 to 24 at `0xF5C`, and scene bodies start after `0xF5E`. Assembly shrink-wrap words start after `0xF76`; part layouts do not change at that boundary. Before scenes exist, complete unshaded fixtures store properties in bucket CObject keys and retain holder self-references.

Versions through `0xFCA` omit the root model-size double. Versions before `0xFD8` omit face state words and the complete body, part and assembly decal-list headers. These omissions are distinct from stored empty lists.

When an RV body is present, versions before `0x1052` omit its color-form and reflectivity fields. Visual properties store `unknown_old_5c` only from `0xE55` through `0x1051`; later versions store `default_appearance`. Both sides have independently generated roots and reject construction with the other layout.

Assembly visibility begins after `0xDCA`. Configuration ID, file type, children count and owner pointer begin after `0xDDE`; pattern reference and its companion word begin after `0xDE2`. Earlier layouts omit those fields. The fixtures distinguish missing hierarchy records from stored empty arrays and null owners, and reject synthetic empty hierarchy records in the older layouts.

Assembly component reference IDs begin after `0xE11`; part layouts do not change at that boundary. Earlier visual properties omit the appearance word entirely, and owned construction rejects a synthetic zero word.

Viewer display-state words begin after `0x10AE`, table header words after `0x10B2`, material color mixing after `0x10A1`, and original illumination parameters after `0x10BF`. Holder state words begin at `0x106A`. The fixture observer changes only reference map-reading decisions; embedded facet decoding retains the actual display version.

This checkpoint does not establish complete display-list parity. Non-null legacy `moPWMaterialProperties_c` payloads remain unsupported because the reference does not resolve their body layout. Earlier display versions and the documented uncounted-window interval remain excluded.

Regenerate with `generate_earliest_visuals.py`, `generate_earliest_visual_properties.py`, `generate_earliest_widths.py` and `generate_earliest_light_states.py`, then check isolated reconstruction with `verify_earliest_visuals.py`, passing the built `rebuild_display_archives` executable. Integration coverage is in `tests/display_earliest_visuals.rs`.

The humanizer skill was unavailable; the prose pass remains for review.
