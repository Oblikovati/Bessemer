//! Complete ordinary modern part configuration archives with explicit header/kernel context.
//! Validation covers the stored archive; geometric validity and external documents are separate.

#[cfg(test)]
include!("test_modules.rs");

mod archive;
mod attribute_records;
mod base_flange_directions;
mod base_flanges;
mod blend_columns;
mod blend_records;
mod body_chooser_records;
mod browser_folders;
mod bucket_adapter;
mod chamfer_records;
mod combine_bodies;
mod configuration;
mod configuration_atoms;
mod configuration_base;
mod configuration_graph;
mod configuration_io;
mod configuration_manager;
mod configuration_objects;
mod configuration_scenes;
mod configuration_suffix;
mod configuration_tables;
mod configuration_toolbox;
mod connection_payload;
mod connection_points;
mod context;
mod context_chains;
mod coordinate_systems;
mod custom_properties;
mod cut_list_folders;
mod detail_folder_records;
mod dimension_plane_tags;
mod edit;
mod edit_history;
mod edit_types;
mod field_change;
mod field_mismatch;
mod flat_pattern_boxes;
mod flat_patterns;
mod fold_transforms;
mod helix_records;
mod manager_tail;
mod mirror_solids;
mod planar_surface_records;
mod polyline_records;
mod reference_dimensions;
mod resolved_archive;
mod resolved_edit;
mod revolved_surfaces;
mod structural_contours;
mod structural_member_objects;
mod structural_member_records;
mod structural_member_references;
mod thicken_records;
mod weld_corners;
mod weldment_records;
pub use resolved_edit::PartResolvedFeaturesEdit;
mod resolved_failure;
mod resolved_features;
pub use resolved_archive::PartResolvedFeaturesArchive;
mod scale_records;
mod scene_adapter;
mod schema_references;
mod sensor_records;
mod validation;
pub use crate::archive_codec::{
    ArchiveClass as ModelClass, ArchiveMapIndex as ModelObjectId,
    FramedObjectRecord as ModelObjectTag, ObjectFraming as ModelObjectFraming,
    ObjectRecord as ModelObjectRecord,
};
pub use crate::archive_codec::{
    ArchiveString, CompactTransform, StoredDouble, StoredFloat, TextEncoding,
};
pub use crate::display_lists::{
    DisplayEditCheckMode as ModelEditCheckMode, DisplayEditCompletion as ModelEditCompletion,
    DisplayEditFidelity as ModelEditFidelity, DisplayFieldRelation as ModelFieldRelation,
};
pub use archive::PartConfigurationArchive;
pub use configuration::ConfigurationId;
pub use context::PartConfigurationContext;
pub use edit_types::{
    ModelArchiveCoverage, ModelArchiveDiagnostic, ModelFieldChange, ModelFieldHandle,
    PartConfigurationEdit,
};
pub use kernel_frames::{CompressedKernelContext, KernelFrame};
pub use reader::ModelFieldLimits as ModelArchiveLimits;
pub use values::{ModelField, ModelFieldValue};

mod annotation_defaults;
mod annotation_view;
mod atom;
mod body_end_specs;
mod body_features;
mod boundary_references;
mod center_helpers;
mod circular_patterns;
mod component_coordinates;
mod component_references;
mod construction;
mod contour_objects;
mod cosmetic_thread_data;
mod cosmetic_threads;
mod datum_helpers;
mod detail_definitions;
mod detail_helpers;
mod detail_prefix;
mod detail_tail;
mod dimension_defaults;
mod dimension_handles;
mod dimensions;
mod display_dimension;
mod display_dimension_tail;
mod display_item;
mod display_object;
mod display_placement;
mod display_records;
mod display_suffixes;
mod dome_records;
mod draft_keepers;
mod draft_records;
mod entity_representations;
mod environment;
mod explosion;
mod external_references;
mod extruded_surfaces;
mod feature_comments;
mod features;
mod fillet_cache;
mod fillets;
mod geometric_references;
mod header_imports;
mod header_order;
mod hole_parameters;
mod hole_wizard;
mod imported_bodies;
mod kernel_frames;
mod label_defaults;
mod layers;
mod lightweight;
mod lightweight_part;
mod lightweight_post;
mod lightweight_shared;
mod linear_patterns;
mod material;
mod mirror_patterns;
mod model_arrays;
mod model_lists;
mod model_references;
mod node;
mod object_body;
mod object_values;
mod objects;
mod part_archive;
mod part_objects;
mod part_tail;
mod pattern_shape;
mod pattern_variation;
mod position_groups;
mod post_views;
mod primitives;
mod profile_chains;
mod profile_features;
mod profile_geometry;
mod raw_fields;
mod reader;
mod reference_axes;
mod reference_geometry_objects;
mod reference_planes;
mod reference_point_data;
mod reference_points;
mod reference_tail;
mod relations;
mod resolved_feature_objects;
mod resolved_managers;
mod resolved_start;
mod shared_tail;
mod sheet_allowance;
mod sheet_arrays;
mod sheet_metal;
mod sheet_payload;
mod shell_records;
mod sketch_archive;
mod sketch_arrays;
mod sketch_backups;
mod sketch_collections;
mod sketch_constraints;
mod sketch_curves;
mod sketch_dimensions;
mod sketch_entities;
mod sketch_geometry;
mod sketch_intersections;
mod sketch_logical;
mod sketch_managers;
mod sketch_offsets;
mod sketch_patterns;
mod sketch_references;
mod sketch_splines;
mod sketch_surfaces;
mod sketch_tail;
mod sketch_text;
mod spatial_objects;
mod spline_arrays;
mod spline_controls;
mod sweep_data;
mod sweep_objects;
mod sweep_records;
mod sweep_reference_records;
mod swift_layout;
mod table_defaults;
mod tolerance_helpers;
mod units;
mod values;
mod views;
mod weld_helpers;

use crate::{BessemerError, FailureClass};
use objects::ModelObjectRole;
use reader::{ModelDecoder, ModelFieldLimits};
use values::encode_fields;

fn invalid(cause: impl Into<String>) -> BessemerError {
    BessemerError::new(
        "model archive",
        "model stream",
        FailureClass::Invalid,
        cause,
    )
}

fn model_failure(mut error: BessemerError) -> BessemerError {
    error.operation = "model archive";
    error.entity = format!("model stream: {}", error.entity);
    error
}
