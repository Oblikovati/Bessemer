# SolidWorks specification snapshot

Edition 0.273 · 2026-10-25 · generated parts: Annex A (solidworks/swstreams.py), Annex G (solidworks/swcover.py)

## 13 Display lists

<a id="st-dl-overview"></a>
### 13.1 Overview

*Status: Verified · 1,086/1,086 streams of parts and assemblies parse to their last byte · Evidence: [C-15] to [C-19], F.6*

`Contents/DisplayLists` holds what SOLIDWORKS needs to show a part or assembly without rebuilding it: the window and view layout, line fonts, the views with their transforms, the lights and appearances, the shaded tessellation of every face, and the scene. It is an archive (clause 11) with object records (11.5). The document's `iSaveDisplayLists` writes it, and `uiModelDoc_c::iRestoreDisplayLists` reads it [C-15].

The archive version `v` of this stream is the document's display-list version: the number N of the stream `_DL_VERSION_N/DLUpdateStamp` (B.2), taken as an unsigned 32-bit number, so that a negative N enables every condition of the form `v > X`. SOLIDWORKS 2026 writes 19000. Several fields depend on the document kind (part, assembly or drawing), which the stream does not record; a reader takes it from the file extension. The drawing `Contents/DisplayLists` layout remains open (OI-016); the separate drawing `Contents/VBLists` prefix is specified in 13.9.

Names in this clause are those of the SOLIDWORKS members, taken from the exported accessors that use each member, the setters that receive each value, or the saver's sources (Annex C.5). A field named `unknown_<offset>` has no such evidence; its meaning is open (OI-015).

<a id="st-dl-top"></a>
### 13.2 Stream layout

Definition: stream `Contents/DisplayLists` of a part or assembly.
*Status: Verified · Evidence: [C-15]*

| # | Field | Type | Presence | Description and rule | St. |
|---|---|---|---|---|---|
| 1 | `dl_format` | i32 | | 1. A reader stops on any other value. | V |
| 2 | `view_count` | i32 | | Number of entries in field 13. | V |
| 3 | `bbox` | box | | Model box (11.4). | V |
| 4 | `max_model_size` | f64 | v > 0xFCA | | V |
| 5 | `old_display_items` | DisplayItemList | v < 0x305 | 13.5. | B |
| 6 | `user_model_env` | object | | `uiUserModelEnv_c`, 13.3.1. | V |
| 7 | `line_font_mgr` | object | v > 0x2FE | `uiLineFontMgr_c`, 13.3.2. | V |
| 8 | `front_view_transform` | | | v > 0xC9C: a transform (11.6). | V |
| 9 | `named_view_transform` | | | 13.4.1. | V |
| 10 | `view_3d_transform` | | | 13.4.2. | V |
| 11 | `orig_font_scale` | | | v > 0x3331: f64. | V |
| 12 | `annotation_view_transform` | | | 13.4.3. | V |
| 13 | `views` | View[view_count] | | 13.4.4. | V |
| 14 | `has_shaded_data` | i32 | | Written 1 when a view is shaded and fields 15–24 follow; 0 ends the stream. | V |
| 15 | `old_lights` | | field 14 ≠ 0 | i32 non-null flag; when non-zero an object array of lights (13.6). | V |
| 16 | `visual_properties` | object | field 14 ≠ 0 | `moVisualProperties_c` of the model, 13.6.2. | V |
| 17 | `tessellation` | | field 14 ≠ 0 | Part: 13.7.1, embedded with no tag. Assembly: an object array of `uoTempAssemblySHDData_c` / `uoTempSubAssemblySHDData_c` (13.7.3). | V |
| 18 | `textured_planes` | | field 14 ≠ 0, v > 0x76B | i32 n, then n × (object `moSketchBitmap_c`, transform). No sample has n > 0 (OI-016). | B |
| 19 | `detail_mask_12` | i32 | field 14 ≠ 0, v > 0x11ED | Passed to `setDetailMask(12, value)`. | V |
| 20 | `scene_type` | i32 | field 14 ≠ 0, v > 0xF5E | 1 `swCubicalScene_c`, 2 `swSphericalScene_c`, otherwise no scene. | V |
| 21 | `scene` | | `scene_type` ∈ {1, 2} | 13.6.3, embedded with no tag. | V |
| 22 | `units_of_measurement` | | field 14 ≠ 0 | v > 0x1FB2: i32 (setting 0x220), `length_units` i32, `angle_units` i32. | V |
| 23 | `envelope_system_options` | | field 14 ≠ 0, assembly | v > 0x42DA: u32 colour (system colour 0x47), i32 (setting 0x347). | V |
| 24 | `markup_ink_views` | | field 14 ≠ 0 | 13.4.5. | V |
| 25 | `configurations` | | field 14 ≠ 0; assembly with v ≥ 0x2404, or part with v > 0x2B5E | 13.8. | V |

NOTE 1 A view whose display mode is below 3 (wireframe and hidden-line modes) is stored with its view bucket (13.4.6); shaded views are stored as tessellation, and field 14 records whether any view needs it.

NOTE 2 For 0x1FAD < v < 0x1FCF a part or assembly also stores one or two transforms and two i32 per *open window*, which the stream does not count; a reader cannot parse those versions and no sample has one.

<a id="st-dl-ui"></a>
### 13.3 User environment and line fonts

<a id="st-dl-usermodelenv"></a>
#### 13.3.1 uiUserModelEnv_c

*Status: Verified · Evidence: [C-16]*

| # | Field | Type | Presence | Description and rule | St. |
|---|---|---|---|---|---|
| 1 | `user_name` | CString | | Windows user name when saved (`GetUserNameW`). | V |
| 2 | `splitter_10` | SplitterInfo | part or assembly | The window splitter layout. | V |
| 3 | `unknown_1b0` | i32 | v > 0x387 | The saver writes the client-rectangle width. | V |
| 4 | `browser_view_width` | i32 | v > 0x387 | | V |
| 5 | `splitter_e0` | SplitterInfo | v ≥ 0x515 | | V |
| 6 | `unknown_1b8`, `unknown_1bc` | i32 × 2 | v > 0x8E5 | The saver writes the selected tab of two tab controls. | V |
| 7 | `unknown_1c0` | i32 | v > 0xF55 | | V |
| 8 | `unknown_1c4` | i32 | v ≥ 0x1FF1 | | V |

*SplitterInfo* (`uiSplitterInfo_c::Serialize`, embedded): u16 `rows`, u16 `cols`, six u16 (row and column sizes and the active pane in the saver, `unknown_10`…`unknown_24`); then rows × cols panes of: a transform (`unknown_28`), `pan_zoom_xform` transform, `display_mode` u16, [v > 0x45C] `hlr_display_quality` u16, [v > 0x1B5] `perspective` f64 (> 0 adds a perspective), [v > 0xBDA] a CString (`unknown_a8`). At most 4 panes.

<a id="st-dl-linefonts"></a>
#### 13.3.2 Line fonts

*Status: Verified · Evidence: [C-16]*

`uiLineFontMgr_c`: a u16 count n, then n × (`key` CString, object `uiLFConfig_c`). The older-version branches of its reader add default fonts in memory and read nothing.

`uiLFConfig_c`: `name` CString; `weight` line width; [v ≥ 0xC9F] `end_cap` i16.

A *line width* (`utLineWidth_c::Serialize`) is, for v > 0xF40, i16 `standard_width` and f32 `custom_width`. For older versions it is `standard_width` only, as i16 in line fonts and as i32 in display items and bucket polylines.

`moLineStyle_c`: `internal_name` CString, `external_name` CString, u16 n and n × f64 `segments`, u8 `thick_thin`. `moNodeName_c`: `name` CString.

<a id="st-dl-views"></a>
### 13.4 Views

All transforms in this subclause are `mgXform_c::restore` (11.6). A *plane* is `root_point` point, `normal` vector, f64, i32 (the last two of unknown meaning).

<a id="st-dl-named-views"></a>
#### 13.4.1 Named views

*Status: Verified · Evidence: [C-17]*

v > 0x1BE7: i32 n, then n × (`view_name` CString, `model_to_view_xform`, `pan_zoom_xform`, a CString; [v > 0x1BF4] i32 p and p × (plane, [v > 0x2001] 2 i32, [v > 0x2007] u32); [p > 0 and v > 0x2003] i32 `has_section_selections`, and when non-zero i32, i32 `is_part`, i32 m and m CStrings; [v > 0x200D] 2 i32).

<a id="st-dl-3d-views"></a>
#### 13.4.2 3D views

*Status: Verified · Evidence: [C-17]*

v > 0x1C18: i32 n, then n × (`view_name` CString; [v > 0x46C9] i32; 2 transforms; a CString; [v > 0x1C1E] a preview bitmap: i32 width, i32 height, i32 width_bytes, u16 planes, u16 bits_pixel, then height × width_bytes bytes (Win32 `BITMAP`); [v > 0x1C42] a CString; i32 p, p × (plane, [v > 0x2012] 2 i32 and u32); [p > 0 and v > 0x2012] i32; [v > 0x1FB5] 3 i32; [v > 0x3331] a CString).

<a id="st-dl-annotation-views"></a>
#### 13.4.3 Annotation views

*Status: Verified · Evidence: [C-17]*

v > 0x1C39: i32 n, then n × (`view_name` CString, `view_id` i32, `is_shown` i32, `is_current` i32, [v > 0x1C40] i32, a CString, 2 transforms, a CString, i32 p and p planes); then [v > 0x1FA8] 2 i32.

EXAMPLE bolt.sldprt has the annotation views "Notes Area" (202), "Unassigned Items" (199), "*Front" (200) and "*Top" (201).

<a id="st-dl-view"></a>
#### 13.4.4 View

*Status: Verified · Evidence: [C-15], [C-17]*

| # | Field | Type | Presence | Description and rule | St. |
|---|---|---|---|---|---|
| 1 | `view_index` | i32 | | 0, 1, … in order. | V |
| 2 | `model_view_xform` | transform | | | V |
| 3 | `pan_zoom_xform` | transform | | | V |
| 4 | `camera_name` | CString | v > 0xBB6 | Name of the view's active camera, or empty. | V |
| 5 | `is_active_view` | i32 | | | V |
| 6 | `display_mode` | i32 | | 3 and above are shaded modes. | V |
| 7 | `edges_in_shaded_mode` | i32 | v > 0x946 | | V |
| 8 | `view_bucket` | object | | `uiViewBucket_c` (13.4.6), null for shaded views. | V |
| 9 | `display_items` | DisplayItemList | v > 0x304 | 13.5. | V |

<a id="st-dl-markup"></a>
#### 13.4.5 Markup views

*Status: Verified · Evidence: [C-17]*

v > 0x2F80: i32 nv, i32 nc; nv × (`view_name` CString, i32); nc × (configuration name CString, i32, `suppressed_state` i32, `is_shown` i32); [v > 0x332B] a CString.

<a id="st-dl-bucket"></a>
#### 13.4.6 View bucket

*Status: Verified · 18 buckets · Evidence: [C-16]*

`uiViewBucket_c`: `bucket` (Bucket); i32 flag and a transform `xform` when it is non-zero; [v > 0x509] i32 flag and `ghost_dl_xform`; `display_mode` i32; `colors` 4 i32; i32 (`unknown_68`); `flags` i32 (bit 0 active, bit 1 regenerated, 0x100 created for detailing mode; before 0x394 a boolean for bit 0); i32 n and n `sub_buckets` objects; [0xA42 < v < 0xA5A] i32 n and n × 3 i32; [v > 0xC90] object `visual_properties`; [v > 0x3340] (i32 flag when v > 0x3341) and a Bucket `original_view_bucket_in_detailing_mode` when set; [v ≥ 0x374B] i32.

*Bucket* (`gcBucket_c::restore`) holds the projected edges of a wireframe or hidden-line view as *polyline lists*: an i32 count, then that many polylines. In order: `normal` point, i32, `persp_mode` i32, `cross_hatch_scale` f64, `tess_number` i32; [v > 0x49C] `has_2d_info` i32 and, when set, `xform_2d`; the lists `silh`, [v > 0x741] `silh_h`, `wire`, `hidden`, `hatch`, `aux`, `tan_v`, `tan_h`, [v > 0x753] `explode_v`, `explode_h`, [v > 0x11A5] `weldbead_v`, `weldbead_h`, [v > 0x7F0] `solid_fill`; [v < 0xA5A] edge visibility data; [v > 0x253] `dl_bucket` DisplayItemList; [v > 0x4BF] `dl_ghost_bucket`; [v ≥ 0x4D4] `show_hidden_count` i32; [0xA59 < v ≤ 0x23D7] a word-to-string map, [v > 0x23D7] `component_str_map` (count, then i32 key and CString each); [v > 0xAD2] four cosmetic-thread maps (i32 m, then m × (i64 key, polyline list)); [v > 0x147D] `cross_hatch_trimmed` i32; [v > 0x184C] `halo_repair` list; [v > 0x1C3D] `surf_cut_body` i32; [v > 0x2789] i32 n and n × 7 i32 `hatch_bucket_set_info`; [v > 0x2F8B] i32; [v > 0x2FAA] `dl_model_bucket` DisplayItemList; [v ≥ 0x372A] i32.

*Polyline* (`gcBucketPoly_c::restore`): `node_id` i32, `occurence` i32, `flags` (u32 from 0x41A1, else i32), i32 point count n (nothing more when n < 1), n points of 3 coordinates (2 when the bucket has 2-D info), f32 before 0x2F9A and f64 from it; then version-dependent attributes: line properties, `type` with its parameters, `atom_id`, layer ids and colours, arc and silhouette data, persistent ids of the edge and its faces, `unique_disp_id` and the mid-point parameters. The full condition list is in solidworks/dl_ui.py, `read_gcBucketPoly_c`.

<a id="st-dl-items"></a>
### 13.5 Display items

*Status: Verified · 25,245 items of 19 types · Evidence: [C-17]*

A *DisplayItemList* (`suDynList<uoDisplayItem_c>`) is a count (11.5), then that many display items. A display item is u32 `size_of_element` (ignored by the reader), i32 `type`, then the payload of Table 13-1. Type names come from the `uoDisplayItemData_c::addX` functions.

| Type | Item | Payload |
|---|---|---|
| 1 | Point | point, `point_style` i32, i32 |
| 2 | Arc | 3 points, vector, i32 |
| 0x65 | ArcWithType | 3 points, vector, `line_type` i32 |
| 3 | Ellipse | 3 points, 2 vectors, 2 f64 |
| 4 | Line | 2 points; v > 0xBF8: u8 flags, else i32 |
| 5 | PolyLine | i32 n, n points, [v > 0x12B0] i32 |
| 6 | Polygon | i32, i32 n, n points |
| 7, 0xF | Text, SystemText | point, 2 f64, `text_position` i32, i32, `text` CString, [v > 0x72A] i32 and 2 f64, [v > 0xA03] i16 flag and, when set, 4 × len(text) bytes |
| 8 | SkText | point, transform, [v > 0x52D] f64, `text` CString |
| 9 | Pen | i32, i32, u32, `line_style` i32, `line_width` line width, `line_type` i32, [v > 0xCA5] `end_cap` i32, [v ≥ 0x185A] i32 |
| 0xA, 0xE | Font, SystemFont | f64, i32, 2 f64, u32, CString, [v > 0x52E] f64, [v > 0x7DF] f64 |
| 0xB | HyperLink | `hyperlink` CString |
| 0xC | UserPen | `color` u32, `width` line width, [v > 0xC57] `layer_id` i32, [v ≥ 0xCA6] `end_cap_style` i32, `style` CString |
| 0xD | Parabola | 4 points, vector, [v > 0x13EC] i32, [v > 0xC58] 2 f64 |
| 0x12 | SimpleText | point, [v > 0xB74] `is_3d_text` i32 and `plane_xform` when set, f64, `is_right_to_left` i32, `text` CString, i16 flag and, when set, 4n bytes (n = len(text), or an i32 when v > 0xAEB), [0x1447 < v < 0x144A] i32, [0x1C04 < v < 0x1FB5] f32, [v > 0x2000] `is_scale_invariant` i32, [v > 0x239A] `text_scale` f64 |
| 0x13 | SimpleSymbol | point, [v > 0xBBD] `is_3d_symbol` i32 and `plane_xform` when set, 2 f64, 2 CStrings, [v > 0x371F] i32 n and n nested display items |
| 0x17 | GTable | [v > 0x10B2] 2 i32, [v > 0x1115] i32 (0: u32 n and n u32 `ole_object_num_list` follow), `table_name` CString, [v > 0x1C56] i32, [v > 0x238C] `always_2d` i32, [v > 0x3332] CString |
| 0x18, 0x20, 0x74 | CustomProperty, SummInfoProperty, RevisionProperty | i32, point, 2 i32 |
| 0x19, 0x1A | XMLStartItem, XMLEndItem | u32 |
| 0x1C | FlipGroupEnd | point, 2 vectors, i32 |
| 0x1D | AnnotStart | 2 i32, then annotation data by version (solidworks/dl_doc.py), [v ≥ 0x35E9] `id_string` CString |
| 0x1E | AnnotEnd | vector, i32 |
| 0x1F | PipeSpline | i32 n, f64, [v ≥ 0x2016] i32, n points, n vectors, [v ≥ 0x1FF8] `display_name` CString |
| 0x21 | CenterOfMass | point, `style` i32, [v > 0x146E] f64 and i32, [v ≥ 0x1473] f64 |
| 0x64 | LineWithType | 2 points, `line_type` i32 |
| 0x6C | AtBackZpos | `z_pos` f32 |
| 0x71 | WatermarkDataStart | 2 i32, f32 |
| other | PrintingOff 0x15, PrintingOn 0x16, FlipGroupStart 0x1B, ArrowHead 0x66, Triangle 0x67, StartItem 0x68, EndItem 0x69, AtBack 0x6A–0x6B, COMPrimitives 0x6D–0x6E, CThreadGeom 0x6F–0x70, WatermarkDataEnd 0x72 | none |

*Table 13-1 — Display item payloads*

<a id="st-dl-lights"></a>
### 13.6 Lights, appearances and scenes

<a id="st-dl-light"></a>
#### 13.6.1 Lights

*Status: Verified · 1,085 ambient, 2,117 directional, 12 spot lights · Evidence: [C-18]*

`moAmbientLight_c` is `moLightFeature_c`; `moDirectionLight_c`, `moSpotLight_c` and `moPointLight_c` derive from it and append their fields.

`moLightFeature_c` (v > 0x427): `ambient`, `diffuse`, `specular` f64; `color` u32; `suppressed` i32; `in_model_space` i32; i32 (dropped); [v ≥ 0x452] `use_cart_display` i32. (For v ≤ 0x427: six f64, nine f64, u32, i32.) Then [v > 0x17C] `id_name` CString; `user_name` CString when v < 0x407 or v > 0x453; [v > 0xCC0] `enabled_in_pv` i32; [v > 0xD01] `keep_light_in_scene` i32; [v > 0x11A3] `enabled_in_pv` i32, `pv_brightness` f64, `pv_shadow_enabled` i32, `pv_shadow_quality` f64; user properties (u32 n, then n × (u32 type; type 0 f32, 1 i32, 2 CString)); [v > 0x11F5] `id` i32.

Directional: [v > 0x427] `direction` vector; [v ≥ 0x11A4] `pv_shadow_softness` f64. Spot: [v ≥ 0x43F] `position` point, `target` point, `cone_angle`, `exponent`, `constant`, `linear` and `quadratic` falloff f64 and an f64; [v ≥ 0x11A4] `pv_soft_edge`, `pv_spot_light_radius` f64, `pv_fog_enabled` i32, `pv_fog_distance`, `pv_fog_quality` f64. Point: [v > 0x427] `position`; [v > 0x11A3] `pv_point_light_radius` f64, `pv_fog_enabled` i32, `pv_fog_radius`, `pv_fog_quality` f64.

<a id="st-dl-visprops"></a>
#### 13.6.2 Visual properties

*Status: Verified · 7,661 records · Evidence: [C-18]*

`moVisualProperties_c`: [v > 0x90] `color` u32; [v > 0xD3] `wire_color` u32; [v > 0xAC] `hlr_color` u32; material properties; [v > 0xB2B] `addin_identifier` CString and `valid_only_for_addin` i32; [v > 0xB53] `use_in_sw_lw_rendering` i32; [v > 0xB55] `appearance_name` CString; [v > 0xDB1] `hardcoded_facet_display` i32; [0xE55 ≤ v ≤ 0x1051] i32, [v > 0x1051] `default_appearance` i32; [v > 0x144B] i32; [v > 0x2419] i32; [v > 0x2794] `graphic_mesh_color_property` i32; [v > 0x3F08] `has_dspbr` i32 and, when set, DSPBR properties.

*Material properties* (`uoMaterialProperties::restore`): `user_name`, `id_name` CString; `ambient`, `diffuse`, `specular`, `shininess` f64; [v > 0x172] `transparency`, `emission` f64; [v > 0x937] `has_texture` i32 and texture properties; [v > 0x954] five i32 `use_advanced_graphics`, `use_defined_texture`, `use_defined_shader`, `use_defined_color`, `use_defined_hatch`; [v > 0xA05] `classification` CString; [v > 0xDB3] `use_new_shaders` i32; [v > 0xF1D] `has_rv` i32 and RealView appearance properties; [v > 0x3368] `plm_id` CString; [v > 0x372A] i32 and a second texture; user properties.

*Texture properties*: 13 f64 (`texture_ranges`, `texture_dir`, `texture_angle` among them), CString, `texture_render_mode` i16, `ignore_color_material` i32, [v > 0xF1D] `texture_trans_x`, `texture_trans_y` f64, [v > 0x372A] `for_c_thread` i32. RealView and DSPBR property blocks are fixed lists of u32, f32 and CString values by version; solidworks/dl_light.py gives them field by field.

<a id="st-dl-scene"></a>
#### 13.6.3 Scene

*Status: Verified · 271 cubical scenes · Evidence: [C-18]*

`swScene_c::Serialize` (both scene classes): the concept data (`moNodeName_c` name object, i32 flags and ids by version, [v > 0x23CD] `tool_tip_name` CString, a merged node list when flagged, [v > 0x1A5] a `moVisualProperties_c` object, and the feature node list); a `pwSceneData_c` object (scene, display and light file names, shadow and environment settings, wall information, colours as 3 f64); i32 n walls of i32 key (from 0xDF8), i32 type (1 rectangle, 2 circle, 3 sphere) and the wall; a box; `floor_orient` i32; `required_xform` transform; object `floor_selection`; floor direction, offset and rotation, environment rotation, shadow and reflectivity settings; then version-dependent settings up to [v > 0x46BA] `field_of_view` f64.

<a id="st-dl-tess"></a>
### 13.7 Tessellation

<a id="st-dl-parttess"></a>
#### 13.7.1 Part tessellation

*Status: Verified · Evidence: [C-19]*

`uoTempPartTessData_c::Serialize`:

| # | Field | Type | Presence | Description and rule | St. |
|---|---|---|---|---|---|
| 1 | `bodies` | object array | | `uoTempBodyTessData_c`, 13.7.2. | V |
| 2 | `model_viz_props` | | v ≥ 0x3E6 | One object up to 0x10AE; then i32 n and n × (i32 key, object). | V |
| 3 | `ddata_time_stamp` | u32 | v ≥ 0x4F7 | `su_CTime`; 0xFFFFFFFF for none. | V |
| 4 | `unknown_30`, `unknown_48` | | v > 0x852 | i32 n, n i32, n i32. | V |
| 5 | `decals` | | v > 0xFD7 | Before 0x239D: u32 n and n decals; then u32 n and n × (i32 key, u32 m, m decals). | B |
| 6 | `unknown_168` | i32 | v ≥ 0x10F0 | | V |
| 7 | `is_tessed_for_exploded_view` | i32 | v > 0x12FC | | V |
| 8 | `config_id`, `config_name` | i32, CString | v > 0x2B96 | | V |
| 9 | `graphics_connect_ref_mgr_data` | | v > 0x2B9D | i32 flag, then a `moGraphicsConnectRefMgrData_c` object when set. | B |
| 10 | `bbox` | box | v > 0x2BA3 | | V |
| 11 | reference geometry | | v > 0x2F5D | 13.7.4. | V |
| 12 | `show_c_thread` | u8 | v > 0x3751 | | V |

<a id="st-dl-bodytess"></a>
#### 13.7.2 Body and face tessellation

*Status: Verified · Evidence: [C-19]*

`uoTempBodyTessData_c`: `faces` object array of `uoTempFaceTessData_c`; [v > 0x506] i32; [v ≥ 0x82B] i32; [v > 0x852] i32; `body_prop_infos`: one `moVisualProperties_c` object for 0xB4B–0x10AE, i32 n and n × (i32, `moVisualProperties_c` object) for 0x10AF–0x10F5, and i32 n and n × (i32, `uoBodyPropInfo_c` object) above; [v > 0xFD7] u32 n and n decals; [v > 0x2017] a CString; [v > 0x2791] i32.

`uoBodyPropInfo_c`: a `moVisualProperties_c` object, i32, `display_mode` i32, i32, `hlr_display_quality` i32.

`uoTempFaceTessData_c`:

| # | Field | Type | Presence | Description and rule | St. |
|---|---|---|---|---|---|
| 1 | `facets` | FacetContainer | unless the face is stored in FaceTessellations | 12.4. | V |
| 2 | `face_viz_props` | | | One object before 0x10AF, then i32 n and n × (i32, object). | V |
| 3 | `unknown_34` | i32 | v ≥ 0xFD8 | | V |
| 4 | `unknown_68` | i32 | v ≥ 0x117D | | V |
| 5 | `unknown_58` | | v ≥ 0x1203 | i32 n and n × 2 i32. | V |
| 6 | `unknown_38` | i32 | v ≥ 0x1401 | | V |
| 7 | `face_string_empty` | i32 | v > 0x2BCF | 0 when field 8 follows. | V |
| 8 | `face_string` | CString | field 7 = 0 | Persistent face reference, e.g. "moEndFaceSurfIdRep_c,24,1,0,". | V |
| 9 | `face_pers_id` | i32 | v > 0x336B | | V |
| 10 | `unknown_78` | i32 | v ≥ 0x3743 | | V |

A face of an assembly component stores no facet container when the component's `comp_id_string` (13.7.3) is the `comp_id` of an entry of `FaceTessellations/Directory` (12.2); its facets are in the block stream instead. The rule does not apply inside the per-configuration data of 13.8.

*Decal* (`uoDecalProperties::restore`): u8; `u_dir`, `v_dir`, `center` 3 f32 each; `face_width` f32; `mask_filename` CString; `mask_type` u8; `mask_invert` i32; u32 n and n u32 `color_mask`; texture properties; material properties; i32 n and n × (u32 face id, 3 f32, [v > 0x12DB] 3 f32 and i32) `decal_faces`; [v > 0x1197] `use_underlying_appearance` i32; [v > 0x11BB] `orig_decal_id` i32; [v > 0x1BBE] `mirror_offset_x`, `mirror_offset_y` f32. No sample has a decal.

<a id="st-dl-asmtess"></a>
#### 13.7.3 Assembly components

*Status: Verified · Evidence: [C-19]*

`uoTempAssemblySHDData_c` (and `uoTempSubAssemblySHDData_c`, whose reader is the same): the component data (before 0x3A3: CString, transform, box, object; otherwise *viewer component data*); object `comp_part_tess_data` (`uoTempPartTessData_c`, 13.7.1); [v > 0xC6E] i32; [v > 0xDDE] `children` object array and object `owner`; [v > 0xF76] `has_shrink_wrap_config` i32; [v > 0xFD7] u32 n and n decals; [v ≥ 0x10F0] i32; [v > 0x11AF] i32 n and n i32 `hidden_body_atoms`.

*Viewer component data* (`moViewerComponentData_c::Serialize`): `comp_id_string` CString, `comp_xform` transform, `comp_bbox` box, `comp_viz_prop` object, `ref_configuration` CString, `num_user_comp_bodies` i32, `model_update_stamp` u32, `model_id` u32 (`su_CTime`), `model_name` CString, `path_name` CString; then by version `is_a_zone`, `gobbled_up_atom_ids`, display mode data, `visible`, `suppressed`, `comp_type`, `ref_configuration_id`, `file_type`, `has_pattern_ref`, `reference_id`, `display_state`, `is_virtual_comp`, `comp_ref`, `spool_ref`, `comp_description`, `config_description`, `depended_comp_names`, `equ_dep_comp_names`, `folder_comp`, `view_comp_type`, `asm_feat_type`, `belt_has_part`, `grouped_folder_comp`, `grouped_folder_type`, `fixed`, `solve_status`, `temp_fixed`, `flexible`, graphics connection data, reference geometry, `viewer_comp_tess_numbers`, `display_state_name`, `routing_assembly` and [v ≥ 0x42F3] `top_level_speed_pak` u8 (conditions in solidworks/dl_tess.py).

<a id="st-dl-refgeom"></a>
#### 13.7.4 Reference geometry

*Status: Verified · Evidence: [C-19]*

`moRefGeomFeatureDataCollector_c::Serialize`: n as two u32 (high word first), then n × (i32 type, data). Every data record starts with CString, 3 i32. Then by type: 0 origin (nothing more); 1 point (point); 2 axis (3 points, vector, [v > 0x2F5B] 2 f64); 3 plane (point, vector, the fixed 120-byte transform of 11.6, 4 f64, i32, [v > 0x2F66] i32); 4 coordinate system (3 vectors, 5 i32, point); 5 connection point ([v ≥ 0x2FB9] CString, i32, 3 points, vector, 2 f64, and further fields from 0x2FB9); 6 route point ([v > 0x2FB8] point, vector). Other types are invalid.

<a id="st-dl-configs"></a>
### 13.8 Per-configuration data

*Status: Typed · only the leading count is exercised (0 in every sample) · Evidence: [C-19]*

Part (`uiPartDoc_c::iRestoreDisplayListsMultipleConfigs`): i32 n, and nothing else when n < 1; active configuration name CString and id i32; n configuration names; n × (i32 configuration id, part tessellation: embedded before 0x2B67, otherwise an object); [v > 0x2B61] n × (i32 id, view_count objects) and n × (i32 id, view_count DisplayItemLists); [v > 0x2B7F] annotation view visibility (i32 n, n × 2 CStrings); [v > 0x2FA5] two u32 n (high word first) and n × (CString, two u32 m, m i32).

Assembly (`uiAssemblyDoc_c::iRestoreDisplayListsMultipleConfigs`): [v < 0x2404] CString and [v > 0x23DD] i32; i32 n (nothing else when n < 1); [v > 0x2403] active name and id; n names; n × (i32 id, component object array as in field 17 of 13.2, [v > 0x240E] `moVisualProperties_c` object); [v > 0x2B6F] the two per-view lists; [v > 0x2B77] annotation view visibility; [v > 0x2FA5] the same tail as for parts. Faces here always carry facet containers.

<a id="cl-other-files"></a>
### 13.9 Drawing view-bucket stream prefix

*Status: Native storage dispatch and serializer, 20 complete bucket-list prefixes; 19 nonempty shaded bodies remain opaque · Evidence: [C-154]*

Definition: stream `Contents/VBLists` of a drawing. The scoped reader supports modern model archive versions `v > 0x27CB`; all 20 installed drawing samples use model version **19000**. The native storage helper initializes the archive with the document's model version, so the decoder obtains it from `_MO_VERSION_N`, rather than substituting the `_DL_VERSION_N` display-list version or a default.

`SolidDrawingStorageMFC_c::iRestoreDrawingsDisplayLists`, in `drwcplu.dll` at `0x180005A30`, opens the stream named by literal `VBLists` and invokes `uiDrawingHLRShadedObject_c::Serialize` in `slddrwu.dll` at `0x3E127280`. The serializer writes/reads a direct signed 32-bit bucket count, that many `uiViewBucket_c` object pointers, and then a direct `uiDrawingShadedData_c` body. The leading value is a **pointer count**, not a byte length. The earlier classification of this stream as a length-prefixed MFC archive is corrected by this clause.

*Table 13-2 — Modern drawing VBLists prefix*

| Order | Field | Encoding | Meaning and handling |
|---|---|---|---|
| 1 | `view_bucket_count` | i32 | Number of following top-level object pointers; MUST be nonnegative and fit the available stream bytes |
| 2 | `view_buckets` | `view_bucket_count` MFC object pointers | `uiViewBucket_c` bodies as in 13.4.6; null pointers and references use the shared archive map |
| 3 | `shaded_data_present` | i32 | First field of the direct shaded-data body; zero ends that body, any nonzero value selects its continuation |
| 4 | shaded-data continuation | Remaining bytes when field 3 is nonzero | Retained opaque by this prefix decoder; no length prefix or complete internal layout is inferred |

The bucket count does not count newly allocated objects or MFC class declarations. One map is retained across the entire top-level pointer list and nested bucket bodies. Readers MUST NOT reset it per entry or expand an existing-object reference into additional wire bytes. The native writer emits the list count as a four-byte `long` and then visits its linked-list entries; it does not use the variable-width MFC `ReadCount` encoding for this outer count.

The shared bucket reader retains existing unknown-field accounting from 13.4.6, including fields whose meanings remain open. It uses drawing context; this stream has no `Contents/DisplayLists` format word, user environment or view-transform preamble. Those structures MUST NOT be prepended to this layout. New MFC classes and objects begin with the ordinary clause 11.5 map state, rather than an inferred map imported from the model configuration stream.

`uiDrawingShadedData_c::Serialize` at `0x3E127420` reads its presence word for the modern versions in scope and returns immediately when it is zero. For nonzero presence it continues with per-view arrays, string-array objects and shaded-model records. Those continuations are not decoded here. `solidworks/drawingvblists.py` stops the returned archive at `prefix_end`, preserves its map, returns the rest as `opaque_shaded_data`, and counts every remaining byte as unknown. This is a prefix reader, not validation of a nonempty shaded body. A zero presence word followed by extra bytes is rejected because the verified body has already ended.

Across the **20** installed drawing streams, **176** top-level pointers precede the shaded data. The streams contain **8,215,597 decoded bytes**; the verified prefixes occupy **3,281,626 bytes**. Of those prefix bytes, **411,416** retain unresolved meanings under the shared readers. The remaining **4,933,971** bytes are opaque shaded continuations. The resulting credited decoded bytes are **2,870,210**, with **5,345,387** decoded bytes remaining unknown. These quantities do not imply complete stream coverage.

The `samples/tutorial/AutoCAD/7550-021.slddrw` stream is **230 bytes**, has one top-level bucket and a zero shaded-data flag, and reaches exact EOF after the prefix. Its shared bucket fields still contain 12 unknown bytes. The other 19 streams store nonzero flag value 1 and retain opaque tails. The corpus report binds each stream and prefix to its SHA-256 and records its exact boundary, root count, class counts and unknown attribution.

At the edition 0.137 checkpoint, the coverage registry credited only these prefix fields, tags and existing documented bucket contents. Proportional attribution of their compressed stream bytes reduced unknown drawing file bytes by **792,707**, from 4,301,943 to **3,509,236**. Drawing file-byte coverage rose from **18.0% to 33.1%** under Annex G's measure; container and thumbnail exclusions gave **21.5%** content coverage. Clause 13.10 extends the prefix inside nonempty shaded bodies. The remaining shaded records, drawing `Contents/DisplayLists`, older drawing layouts and complete document bit coverage remain open.

### 13.10 Drawing shaded-data per-view prefix

*Status: Native array/string-list serializers and 19 extended prefixes; model/tessellation records and later fields remain opaque · Evidence: [C-155]*

When the shaded-data presence word in 13.9 is nonzero, `uiDrawingShadedData_c::Serialize` continues with arrays whose lengths are the outer `view_bucket_count`. In the modern scope `v > 0x27CB`, all version gates in Table 13-3 are enabled. The arrays do not repeat that outer count on the wire.

*Table 13-3 — Modern drawing shaded-data per-view prefix*

| Order | Field | Encoding | Native member or condition | Meaning and coverage |
|---|---|---|---|---|
| 1 | `shaded_model_indices` | `view_bucket_count` u32 words | +0xD8 collection | Copied into the drawing's named shaded-model-index array |
| 2 | `raw_words_20` | `view_bucket_count` u32 words | +0x20; v > 0x762 | Values retained as unknown |
| 3 | `string_lists_28` | `view_bucket_count` direct string-list bodies | +0x28; v > 0x813 | Each body is an i32 string count, followed by that many CStrings; text meaning remains unknown |
| 4 | `string_lists_68` | `view_bucket_count` direct string-list bodies | +0x68; v > 0x985 | Same direct body encoding; separate collection |
| 5 | `model_count` | i32 | +0xD0 | Number of following direct shaded-model records |
| 6 | shaded-model records and later fields | Remaining bytes | Model-record serializer begins at `0x3E128320` | Opaque to this reader |

The first array is stored in a native `su_CUIntArray` at shaded-data +0xD8. The drawing storage reader copies that collection through the `getShadedModelIndexArray`/`setShadedModelIndexArray` API pair. Its file words are retained unsigned; the observed values include `FFFFFFFF` and small model indices. The second array is distinct even though every observed word is `FFFFFFFF`. Its interpretation is not inferred from that repetition and all its bytes remain unknown.

Both string-list groups use fresh, directly constructed wrapper objects with vtable `0x3E41C0E0`. Their serializer, `0x3E127FC0`, reads a **fixed four-byte signed count** and then the ordinary clause 11.2 CString representation for each entry. It does not use the two-/six-byte MFC `ReadCount` encoding. The wrappers are not introduced with MFC object/class tags, do not insert slots in the enclosing object map, and are serialized separately for each view in each group. A reader MUST NOT combine the two groups, add object tags or reset the map between them.

Every observed string list is empty, but the bounded reader also accepts nonempty lists according to the native count/CString loop. It retains each string's physical bytes and span, alongside its decoded value, and counts the complete CString as unknown because the field's meaning is unresolved. Thus a successful read does not grant semantic credit for its text. The list counts themselves have established framing meaning. Negative or impossible list counts and invalid/truncated CString data are rejected.

`drawingvblists.read(..., through_shaded_prefix=True)` advances through this prefix and stops immediately after `model_count`; the default mode continues to expose the 13.9 bucket-only boundary. At the edition 0.138 checkpoint the coverage registry used the extended mode; it now uses the complete mode in 13.11. It preserves the original archive and map for the model records. The reader checks nonnegative model count against a conservative seven-byte minimum per modern record, derived from the initial i32, CString and u16 fields of `0x3E128320`; this is only a bound, not validation of those records or their tessellation bodies. All remaining bytes retain zero semantic credit.

The 19 nonempty bodies contain arrays for **175 view entries**, **350 empty direct string-list bodies**, and **33 shaded-model records**. This stage advances **2,876 bytes** beyond the 13.9 boundaries. It credits **2,176** of those decoded bytes: 700 shaded-model-index bytes, 1,400 list-count bytes and 76 model-count bytes. The 700 bytes of `raw_words_20` remain unknown. Synthetic fixtures distinguish fixed i32 list counts from MFC counts, preserve both narrow and UTF-16 CString wire forms, retain opaque model bytes, and reject malformed counts and truncated prefixes.

At the edition 0.138 checkpoint, the cumulative verified prefixes occupied **3,284,502** of the **8,215,597** decoded VBLists bytes. Credited decoded bytes total **2,872,386**; **5,343,211** remain unknown, including **4,931,095** opaque model/later bytes. Both bucket-only and extended prefix boundaries and hashes are retained in the corpus report. Proportional stored-byte attribution leaves **3,508,706** unknown drawing file bytes; rounded drawing coverage remains **33.1%**, or **21.5%** excluding container and thumbnails. Complete model/tessellation decoding and full-document bit coverage remain open.

### 13.11 Drawing shaded models and trailing collections

*Status: B — complete framing for the installed version-19000 drawing corpus; unresolved field meanings remain unknown.*

The complete mode, `drawingvblists.read(..., through_models=True)`, continues from 13.10 using the same archive and object map. Native model serializer `0x3E128320` writes the following direct record for each declared model. Scope remains the modern drawing archives of 13.9; earlier layouts are unsupported.

| # | Field | Type | Presence | Description and rule | St. |
|---|---|---|---|---|---|
| 1 | `is_part_word` | i32 | Always | 1 selects part tessellation; 0 selects assembly tessellation; other values rejected | B |
| 2 | `model_reference` | CString | Always | Preserve physical wire bytes; resolver and path rewrite rules remain open; no semantic credit | B |
| 3 | `raw_30` | u16 | v > 0x929 | Native member +0x30; preserve raw, no semantic credit | B |
| 4 | `old_lights` | Tagged pointer | Always | Shared old-light array serializer; ordinary archive-map rules apply | B |
| 5 | `visual_properties` | Tagged pointer | Always | `moVisualProperties_c`, shared serializer | B |
| 6 | `tessellation` | Direct shared body | Always | `uoTempPartTessData` for part; `uoTempAssemblySHDDataArray` for assembly; clause 13 shared layouts | B |

Native face serializer `0x49FC3F60` retains the ordinary i32 appearance-map count and key/value loop. The tessellation handle restore at `0x31BFA2A0` calls `gcFacetContainer_c::restore` without adding a wire field. The alignment difference is inside facet restore `0x3CAD9380`: for v > 0x2BD0 the edge-point block is present only when the archive backend is absent, its virtual +0x90 returns false, or v < 0x2BE5. At v >= 0x2BE5 a true selector omits that block. The selector is external archive context, not a serialized flag. Its precise backend class and semantic name remain unidentified. The scoped drawing reader supplies `facet_archive_mode_90=True`; standalone facet readers retain false. This matches the native branch and all 33 installed model layouts. No bytes are skipped heuristically, and no appearance-map representation is changed.

After all models, the shaded serializer writes three groups of direct lists. Each group contains `view_bucket_count` lists, in view order: collection +0xF8 for v > 0x90E, +0x118 for v > 0x9A5, then +0x48 for v > 0x11AF. Their respective wrapper vtables `0x3E41C200`, `0x3E41C290` and `0x3E41C170` share serializer `0x3E127DA0`. Every list is i32 count followed by that many four-byte words. Negative or impossible counts are invalid. These direct wrappers add no object tags or map slots. Word meanings remain unresolved; all populated words receive zero semantic credit. The native corpus contains only empty lists, totaling 2,100 count bytes. Synthetic populated lists verify count framing, ordering and unknown attribution.

All **20** installed VBLists streams now end exactly at EOF, including **33** shaded models (**23 part, 10 assembly**) in 19 streams and one absent-shaded stream. The reader frames **8,215,597 decoded bytes**, credits **7,567,905**, and retains **647,692 unknown bytes** for unresolved shared fields, reference strings and raw words. Earlier bucket and shaded-prefix boundaries and hashes remain in `drawing-vblists-corpus.json`. Full framing does not establish complete geometry semantics, editable-file round trips, coverage of older drawings, or full-document bit coverage. Proportional stored-byte attribution leaves **2,240,327 unknown drawing file bytes**, giving **57.3%** file-byte coverage or **49.9%** excluding container and thumbnails. Other drawing streams and installed project formats remain separate open work.

Evidence: C-156.
