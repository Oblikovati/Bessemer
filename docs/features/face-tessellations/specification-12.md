# Retained source: clause 12 and Annex B.3

Source: `../inventor-interface/docs/solidworks-file-format/specification.md`, Edition 0.117, working tree at `dd0537b3a142d8dda00ef846891efc736a6aac46`, read 2026-10-24.

## 12 Face tessellations

<a id="st-facettess-overview"></a>
### 12.1 Overview

*Status: Verified · 170 directories and 193 block streams in 176 assemblies parse to their last byte · Evidence: [C-13], F.5*

An assembly keeps the shaded tessellation of its part components in the `FaceTessellations` storage: a `Directory` stream (12.2) and one or more block streams named `NNN-NNN-NNN` (12.3). Each part component has a directory entry that names its block stream and its position there.

<a id="st-facettess-directory"></a>
### 12.2 Directory

Definition: stream `FaceTessellations/Directory`; one per assembly with face tessellations.
*Status: Verified · 170/170 · Evidence: [C-13]*

| # | Field | Type | Presence | Description and rule | St. |
|---|---|---|---|---|---|
| 1 | `directory_version` | i32 | | 2. | V |
| 2 | `last_block_number` | i32 | | Number of the last block stream written; block names are three groups of three digits ending in it, e.g. 7 for `000-000-007`. | V |
| 3 | `count` | i32 | | Entry count. | V |
| 4 | `entries` | PartCompInfo[count] | | 12.2.1. | V |

<a id="st-partcompinfo"></a>
#### 12.2.1 Part component entry

Definition: element of the Directory; `partCompInfo_c` (sldmodu.dll).
*Status: Verified · Evidence: [C-13]*

| # | Field | Type | Presence | Description and rule | St. |
|---|---|---|---|---|---|
| 1 | `tess_array_index` | i32 | | Index of the component in the assembly's tessellation array. | V |
| 2 | `comp_id` | CString | | Component name path, e.g. "bolt-1@bolt-assembly". | V |
| 3 | `model_update_stamp` | u32 | | Update stamp of the component model when tessellated. | V |
| 4 | `user_comp_time` | u32 × 2 | | FILETIME of the component file, high word first. | V |
| 5 | `shared_part_comp_info` | u32 × 2 | | 64-bit key of a shared entry, high word first; 0 when none. | V |
| 6 | `hash_key` | u32 × 2 | | 64-bit hash key of the component, high word first. The same key starts the component's entry in its block (12.3). | V |
| 7 | `face_tess_block_name` | CString | | Name of the block stream, e.g. "000-000-007". | V |
| 8 | `index_in_block` | i32 | | Position of the component's entry in that block. | V |
| 9 | `part_comp_tess_numbers` | map | doc version ≥ 0x3B1C | From doc version 0x3B23 an i32 count, then (key i32, first i32, second i32) per entry; for 0x3B1C–0x3B22 one (first, second) pair. | B |

<a id="st-facettess-block"></a>
### 12.3 Block stream

Definition: stream `FaceTessellations/NNN-NNN-NNN`; entries to the end of the stream.
*Status: Verified · 193/193 · Evidence: [C-13]*

| # | Field | Type | Presence | Description and rule | St. |
|---|---|---|---|---|---|
| 1 | `index_in_block` | i32 | | = the entry's `index_in_block` in the Directory. | V |
| 2 | `hash_key` | u32 × 2 | | = the Directory entry's `hash_key`, high word first. | V |
| 3 | `face_count` | i32 | | Number of faces. | V |
| 4 | `faces` | FacetContainer[face_count] | | 12.4, one per face. | V |

<a id="st-facet-container"></a>
### 12.4 Facet container

Definition: one face's tessellation; `gcFacetContainer_c` (sldmfcu.dll). Fields are listed for archive versions above 0x3EE7, which every sample uses; Annex B lists the older conditions.
*Status: Verified · 35,242 containers · Evidence: [C-14], F.5*

| # | Field | Type | Presence | Description and rule | St. |
|---|---|---|---|---|---|
| 1 | `num_facets` | i32 | | Triangle count = Σ (strip length − 2). | V |
| 2 | `num_strips` | i32 | | Triangle-strip count. | V |
| 3 | `header_mode` | i32 | | 1 in 35,242/35,242: fields 4–9 follow. 4: field 10 is read without its element size and fields 4–9 are absent. 0: fields 4–10 are absent. SOLIDWORKS writes 3 when it saves indexed triangles. | V |
| 4 | `face_color`, `edge_color` | u32 × 2 | mode not 0 or 4 | Colours of the face and its edges. | V |
| 5 | `prune_flags` | u32 | mode not 0 or 4 | Flag word; the reader keeps bits 0x7FF80000. | B |
| 6 | `render_flags` | u32 | mode not 0 or 4 | Facet render offset and texture mode bits; the reader keeps all but 0x02000000. | B |
| 7 | `flags_18` | u32 | mode not 0 or 4 | Flag word ORed into the container (OI-014). | ? |
| 8 | `ignored_ints` | i32 count, i32[] | mode not 0 or 4 | Read and discarded; SOLIDWORKS writes count 0. | B |
| 9 | `ignored_floats` | i32 count, f32[] | mode not 0 or 4 | Read and discarded; SOLIDWORKS writes count 0. | B |
| 10 | `vert_nums` | DynArray of i32 | mode not 0 | Strip lengths; Σ = vertex count. | V |
| 11 | `vertex_coords` | DynArray of f32[3] | | Strip vertices, metres. | V |
| 12 | `norm_comp` | DynArray of f32[3] | | Unit normals, one per vertex. | V |
| 13 | `edge_ids` | DynArray | | Edge identifiers. | B |
| 14 | `edge_nums` | DynArray | | Edge vertex counts. | B |
| 15 | `edge_types` | DynArray | | Edge types. | B |
| 16 | `curvature_calculated` | i32 | | Flag 0x20. | V |
| 17 | `curvature_colors` | i32 count, f32[] | field 16 ≠ 0 | Curvature values. | B |
| 18 | `container_processed`, `box_set` | i32 × 2 | | Flags 0x8 and 0x10. | B |
| 19 | `bbox` | Box | | Face box (11.4); contains every vertex in 10,709/10,709 checked faces. | V |
| 20 | `planar` | i32 | | Flag 0x100. | B |
| 21 | `edge_ids_valid` | i32 | | Bit 0 → flag 0x1000. | B |
| 22 | `stamp` | u32 | | Container stamp. | B |
| 23 | `has_cone_cull_data`, `can_cone_cull` | i32 × 2 | | Flags 0x400 and 0x800. | B |
| 24 | `cone_cull_angle`, `cone_cull_normal` | f32, f32[3] | | Normal cone of the face, for culling. | B |
| 25 | `texture_coords` | i32 flag, DynArray | | Present when the flag ≠ 0. | B |
| 26 | `tangents`, `binormals` | i32 flag, DynArray × 2 | | Present when the flag ≠ 0. | B |
| 27 | `u_params`, `v_params` | i32 flag, DynArray × 2 | | Surface parameters per vertex; present when the flag ≠ 0. | B |
| 28 | `tex_tessellation` | i32 flag, TexTess | | 12.4.2; present when the flag ≠ 0. | B |
| 29 | `save_triangle_indices` | i32 | | When ≠ 0 field 30 follows. | B |
| 30 | `tri_indices` | DynArray | field 29 ≠ 0 | Indexed triangles. | B |
| 31 | `display_color` | i32 | | Flag 0x40000; when set and field 16 is set, an i32 count and that many i32 colours follow. | B |
| 32 | `face_node_id` | u32 | | Node id of the face in the part. | B |
| 33 | `face_normal` | Vector | | Normal of a planar face. | V |
| 34 | `face_type` | i32 | | Surface type code: 4001 plane and 4002 cylinder in the bolt sample (OI-014). | S |
| 35 | `root_point`, `apex_point` | Point × 2 | | Surface root point and cone apex. | B |
| 36 | `radius`, `half_angle` | f64 × 2 | | Surface radius and cone half-angle. | V |
| 37 | `edge_id_info` | i32 count, (i32, i32)[] | | Edge id, then curve type × 16 + edge type. | B |
| 38 | `minor_radius` | f64 | | Torus minor radius. | B |
| 39 | `edge_types_valid` | i32 | | Bit 0 → flag 0x200000. | B |
| 40 | `body_atom_id` | u32 | | Atom id of the body. | B |
| 41 | `face_string` | i32 empty flag, CString | | The string follows when the flag is 0. | B |
| 42 | `aux_texture_coords` | u8 flag, DynArray | | Present when the flag ≠ 0. | B |
| 43 | `save_thread_texture_coords` | u8 | | | B |
| 44 | `edge_points` | i64 count, (i32, Point)[] | archive not lightweight | Edge id and a point on the edge. | B |
| 45 | `edges` | i32 count, EdgeFullInfo[] | | 12.4.1. | V |
| 46 | `u2a8` | u32 | the last edge record did not return 1 | Meaning open (OI-014). | ? |
| 47 | `face_time_stamp`, `part_body_stamp` | u32, i32 | | | B |

<a id="st-edge-full-info"></a>
#### 12.4.1 Edge record

Definition: element of field 45; `gcEdgeFullInfo_c`.
*Status: Typed · Evidence: [C-14]*

| # | Field | Type | Presence | Description and rule | St. |
|---|---|---|---|---|---|
| 1 | `first` | i32 | | Meaning open (OI-014). | ? |
| 2 | `tag` | i32 | | Edge map tag (`gcEdMapInfo_c`). | B |
| 3 | `types` | i32 | | Curve type × 16 + edge type; 15 means unknown. | B |
| 4 | `point` | i32 count, f64[] | | A point on the edge; the first three values are kept. | B |
| 5 | `zero` | i32 | | Written 0, not read back. | B |
| 6 | `list_38` | i32 count, i32[] | | Meaning open (OI-014). | ? |
| 7 | `vv` | i32 count, i64[] | | Pairs of vertex indices along the edge, low word first. | B |

<a id="st-tex-tess"></a>
#### 12.4.2 Texture tessellation

*Status: Typed · Evidence: [C-14]*

Fields: `first` (i32, OI-014), `num_faces_tex` (i32), `num_face_strip_tex` and `is_face_strip_tex` (DynArray), `num_strips_tex` (i32), `vert_nums_tex`, `vertex_coords_tex` and `norm_comp_tex` (DynArray), then an i32 flag that, when not 0, is followed by 3 vectors (OI-014).

<a id="cl-displaylists"></a>
### B.3 Archive versions of the facet container

Fields of 12.4 that depend on the archive version: `header_mode` and fields 4–9 from 0x3EE8; `edge_ids` and `edge_nums` from 0x3DE, `edge_types` from 0x534; two arrays (tri normals, edge triangle indices) for 0x4E6–0xA70; field 16 and its colours from 0x331 (before 0x2CD a DynArray, for 0x2CD–0x330 a DynArray of 24-byte elements); a f64 before 0x116A; an i32 for 0x49E–0x7DB; `planar` from 0x4E6; a DynArray for 0x4FC–0xC6D; `edge_ids_valid` from 0x68A; `stamp` from 0x82A; fields 23–24 from 0x83B; fields 25–26 from 0x901; field 27 from 0x1099; field 28 from 0x967 (its 3 vectors from 0x972); a serialized object for 0xA25–0xC7B; field 29 from 0x1C3B and field 30 from 0x1C24; field 31 from 0x2792; field 32 from 0x279E; field 33 from 0x27A0; fields 34–38 from 0x27AE; field 39 from 0x27B6; field 40 from 0x27E1; field 41 from 0x2BD0; field 42 from 0x3752; field 43 from 0x3758; field 44 from 0x2BD1; field 45 from 0x3B29; fields 46–47 from 0x3B0E. Every sample block uses archive version 19000.
