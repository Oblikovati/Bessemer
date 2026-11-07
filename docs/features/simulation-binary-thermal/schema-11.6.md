### 11.6 Transforms

*Status: Verified · Evidence: [C-16]*

`mgXform_c::restore` reads:

| # | Field | Type | Presence | Description and rule | St. |
|---|---|---|---|---|---|
| 1 | `has_matrix` | u8 | | Non-zero when a rotation matrix follows. | V |
| 2 | `matrix` | vector × 3 | `has_matrix` ≠ 0 | `x_axis`, `y_axis`, `z_axis` (`mgMatrix_c`). | V |
| 3 | `translation` | point | | | V |
| 4 | `scaling` | f64 | | | V |
| 5 | `flags` | u8 | | `flags` & 0xF0 selects field 6. | V |
| 6 | `perspective` | | `flags` & 0xF0 ∈ {0x10, 0x20, 0x40} | 0x10 `mgPerspInfoBase_c`: `eye_point` point, `plane_distance` f64. 0x20 `mgPerspInfo_c`: the same, then `min_far_clip_plane_distance` and `camera_fov_plane_distance` f64. 0x40 `mgMatrix5_c`: an i32 the writer sets to 0, then 25 f64. | B |

`operator>>(mgXform_c&)` is a different, fixed form of 120 bytes: 9 f64 matrix, 2 i32, 3 f64 translation, f64 scaling, 2 i32. Call-site proofs for the fixed operator include plane references (13.8), valued dimensions (11.353), deform geometry (11.357) and spline equations (11.360); it SHALL NOT be substituted for compact restore or vice versa.
