use super::arrays::DynamicArray;
use super::edges::{FacetEdgeInfo, FacetEdgePoints};
use super::groups::{
    ArrayPair, ByteFlaggedArray, FacetCurvature, FacetDisplayColor, FacetString, Flagged,
};
use super::indices::FacetTriangleIndices;
use super::layout::FacetVertexLayout;
use super::record::archive_record;
use super::{FacetArchiveMode, FacetBounds, StoredDouble, StoredFloat};

archive_record! {
    /// Version-gated arrays describing edges without interpreting their element domains.
    FacetEdgeArrays(context, 8, Element) {
        /// Edge identifier array.
        edge_ids: required DynamicArray => true;
        /// Edge vertex-count array.
        edge_numbers: required DynamicArray => true;
        /// Edge-type array after version 0x533.
        edge_types: optional DynamicArray => context.version > 0x533;
    }
}

archive_record! {
    /// The two extra arrays present only in the documented legacy interval.
    FacetLegacyTriangles(context, 8, Element) {
        /// Legacy triangle-normal array.
        triangle_normals: required DynamicArray => true;
        /// Legacy edge-to-triangle index array.
        edge_triangle_indices: required DynamicArray => true;
    }
}

archive_record! {
    /// Raw normal-cone culling parameters.
    FacetConeCull(context, 24, Element) {
        /// Raw has-cone-data word.
        has_data: required i32 => true;
        /// Raw can-cull word.
        can_cull: required i32 => true;
        /// Cone angle, without normalization.
        angle: required StoredFloat => true;
        /// Three raw normal components.
        normal: required [StoredFloat; 3] => true;
    }
}

archive_record! {
    /// Complete auxiliary texture-tessellation record.
    FacetTextureTessellation(context, 32, Element) {
        /// Uninterpreted initial word.
        first: required i32 => true;
        /// Stored textured-face count.
        face_count: required i32 => true;
        /// Face strip-count array.
        face_strip_counts: required DynamicArray => true;
        /// Face strip-indicator array.
        face_strip_flags: required DynamicArray => true;
        /// Stored texture-strip count.
        strip_count: required i32 => true;
        /// Texture vertex-number array.
        vertex_numbers: required DynamicArray => true;
        /// Texture vertex-coordinate array.
        vertex_coords: required DynamicArray => true;
        /// Texture normal-component array.
        normal_components: required DynamicArray => true;
        /// Raw flag and three complete vectors after version 0x971.
        vectors: optional Flagged<[[StoredDouble; 3]; 3]> => context.version > 0x971;
    }
}

archive_record! {
    /// Raw analytic-surface metadata associated with a facet container.
    FacetSurface(context, 80, Element) {
        /// Stored surface code, with an incomplete known enum domain.
        face_type: required i32 => true;
        /// Stored surface root point.
        root_point: required [StoredDouble; 3] => true;
        /// Stored cone apex point.
        apex_point: required [StoredDouble; 3] => true;
        /// Stored radius.
        radius: required StoredDouble => true;
        /// Stored cone half-angle.
        half_angle: required StoredDouble => true;
        /// Ordered edge ID and packed curve/type words.
        edge_id_info: required Vec<[i32; 2]> => true;
        /// Stored torus minor radius.
        minor_radius: required StoredDouble => true;
    }
}

archive_record! {
    /// Final unknown word and face/body stamps, including version-0x3F0F omission state.
    FacetLateStamps(context, 8, Element) {
        /// Omitted only when the preceding edge returned one.
        unknown_2a8: optional u32 => !context.skip_edge_first;
        /// Face time stamp.
        face_time_stamp: required u32 => true;
        /// Part body stamp.
        part_body_stamp: required i32 => true;
    }
}

archive_record! {
    /// Complete archived `gcFacetContainer_c`; this is stored geometry, not a validated mesh.
    FacetContainer(context, 8, Face) {
        /// Stored triangle count, without recomputation.
        num_facets: required i32 => true;
        /// Stored triangle-strip count.
        num_strips: required i32 => true;
        /// Version- and mode-dependent vertex-number framing and raw style fields.
        vertex_layout: required FacetVertexLayout => true;
        /// Complete vertex-coordinate array, normally three f32 components per element.
        vertex_coords: required DynamicArray => true;
        /// Complete normal-component array.
        normal_components: required DynamicArray => true;
        /// Version-gated edge arrays.
        edge_arrays: optional FacetEdgeArrays => context.version >= 0x3de;
        /// Legacy triangle-normal and edge-triangle arrays.
        legacy_triangles: optional FacetLegacyTriangles => context.version > 0x4e5 && context.version < 0xa71;
        /// Raw legacy array or modern curvature flag/colours; controls later display colours.
        curvature: required FacetCurvature => true;
        /// Raw processed-state word.
        container_processed: required i32 => true;
        /// Raw box-set word; the box body is always stored.
        box_set: required i32 => true;
        /// Complete ten-double box body, without geometric normalization.
        bounds: required FacetBounds => true;
        /// Legacy tolerance word.
        old_tolerance: optional StoredDouble => context.version < 0x116a;
        /// Legacy discarded integer, retained for reconstruction.
        legacy_word: optional i32 => context.version > 0x49d && context.version < 0x7dc;
        /// Raw planar word.
        planar: optional i32 => context.version > 0x4e5;
        /// Legacy dynamic array retained even though native code discards it.
        legacy_array: optional DynamicArray => context.version > 0x4fb && context.version < 0xc6e;
        /// Raw edge-ID validity word, including bits beyond bit zero.
        edge_ids_valid: optional i32 => context.version > 0x689;
        /// Stored container stamp.
        stamp: optional u32 => context.version >= 0x82a;
        /// Cone-culling fields.
        cone_cull: optional FacetConeCull => context.version >= 0x83b;
        /// Texture-coordinate flag and nullable array.
        texture_coords: optional Flagged<DynamicArray> => context.version > 0x900;
        /// Flag and two arrays, tangent first and binormal second.
        tangents_binormals: optional Flagged<ArrayPair<DynamicArray>> => context.version > 0x900;
        /// Flag and two arrays, U parameters first and V parameters second.
        surface_parameters: optional Flagged<ArrayPair<DynamicArray>> => context.version > 0x1098;
        /// Flag and complete auxiliary texture record.
        texture_tessellation: optional Flagged<FacetTextureTessellation> => context.version > 0x966;
        /// Version-dependent index flag and nullable triangle array.
        triangles: required FacetTriangleIndices => true;
        /// Display-colour word and conditional colours selected with curvature.
        display_color: optional FacetDisplayColor => context.version > 0x2791;
        /// Stored face node ID.
        face_node_id: optional u32 => context.version > 0x279d;
        /// Stored face normal.
        face_normal: optional [StoredDouble; 3] => context.version > 0x279f;
        /// Analytic-surface metadata and ordered edge/type words.
        surface: optional FacetSurface => context.version > 0x27ad;
        /// Raw edge-type validity word, including bits beyond bit zero.
        edge_types_valid: optional i32 => context.version > 0x27b5;
        /// Stored body atom ID.
        body_atom_id: optional u32 => context.version > 0x27e0;
        /// Inverse empty flag and conditional CString.
        face_string: optional FacetString => context.version > 0x2bcf;
        /// Raw byte flag and optional auxiliary texture array.
        auxiliary_texture_coords: optional ByteFlaggedArray => context.version > 0x3751;
        /// Stored thread-texture save byte.
        save_thread_texture_coords: optional u8 => context.version > 0x3757;
        /// Direct edge points, with an i64 count, absent from lightweight archives.
        edge_points: optional FacetEdgePoints => context.version > 0x2bd0 && context.mode == FacetArchiveMode::Full;
        /// Complete ordered edge records with omission state carried between them.
        edges: optional Vec<FacetEdgeInfo> => context.version > 0x3b28;
        /// Final unknown word and face/body stamps.
        late_stamps: optional FacetLateStamps => context.version > 0x3b0d;
    }
}
