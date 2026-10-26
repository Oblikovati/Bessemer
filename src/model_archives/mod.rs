//! Complete ordinary modern part configuration archives with explicit header/kernel context.
//! Validation covers the stored archive; geometric validity and external documents are separate.

#[cfg(test)]
#[path = "../../tests/resolved_features/application_names.rs"]
mod application_name_tests;
mod archive;
#[cfg(test)]
#[path = "../../tests/resolved_features/attribute_records.rs"]
mod attribute_record_tests;
mod attribute_records;
#[cfg(test)]
#[path = "../../tests/resolved_features/body_chooser_records.rs"]
mod body_chooser_record_tests;
mod body_chooser_records;
#[cfg(test)]
#[path = "../../tests/resolved_features/body_state_roles.rs"]
mod body_state_role_tests;
mod browser_folders;
#[cfg(test)]
#[path = "../../tests/resolved_features/browser_items.rs"]
mod browser_item_tests;
mod bucket_adapter;
#[cfg(test)]
#[path = "../../tests/resolved_features/chamfer_records.rs"]
mod chamfer_record_tests;
mod chamfer_records;
mod configuration;
#[cfg(test)]
#[path = "../../tests/resolved_features/configuration_atoms.rs"]
mod configuration_atom_tests;
mod configuration_atoms;
mod configuration_base;
mod configuration_graph;
#[cfg(test)]
#[path = "../../tests/resolved_features/configuration_graph.rs"]
mod configuration_graph_tests;
mod configuration_io;
#[cfg(test)]
#[path = "../../tests/resolved_features/configuration_io.rs"]
mod configuration_io_tests;
mod configuration_manager;
#[cfg(test)]
#[path = "../../tests/resolved_features/configuration_scenes.rs"]
mod configuration_scene_tests;
mod configuration_scenes;
mod configuration_suffix;
#[cfg(test)]
#[path = "../../tests/resolved_features/configuration_tables.rs"]
mod configuration_table_tests;
mod configuration_tables;
mod context;
#[cfg(test)]
#[path = "../../tests/resolved_features/context_chains.rs"]
mod context_chain_tests;
mod context_chains;
#[cfg(test)]
#[path = "../../tests/resolved_features/context_snapshot.rs"]
mod context_snapshot_tests;
mod custom_properties;
#[cfg(test)]
#[path = "../../tests/resolved_features/custom_properties.rs"]
mod custom_property_tests;
#[cfg(test)]
#[path = "../../tests/resolved_features/detail_folder_records.rs"]
mod detail_folder_record_tests;
mod detail_folder_records;
mod edit;
mod edit_history;
mod edit_types;
mod field_change;
mod field_mismatch;
#[cfg(test)]
#[path = "../../tests/resolved_features/folder_configuration_records.rs"]
mod folder_configuration_record_tests;
#[cfg(test)]
#[path = "../../tests/resolved_features/manager_context.rs"]
mod manager_context_tests;
#[cfg(test)]
#[path = "../../tests/resolved_features/manager_oracle.rs"]
mod manager_oracle_tests;
mod manager_tail;
#[cfg(test)]
#[path = "../../tests/resolved_features/owned_features.rs"]
mod owned_resolved_feature_tests;
#[cfg(test)]
#[path = "../../tests/resolved_features/planar_surface_records.rs"]
mod planar_surface_record_tests;
mod planar_surface_records;
#[cfg(test)]
#[path = "../../tests/resolved_features/polyline_records.rs"]
mod polyline_record_tests;
mod polyline_records;
#[cfg(test)]
#[path = "../../tests/model_archives/public_roles.rs"]
mod public_role_tests;
mod resolved_archive;
mod resolved_edit;
pub use resolved_edit::PartResolvedFeaturesEdit;
#[cfg(test)]
#[path = "../../tests/resolved_features/native_corpus.rs"]
mod resolved_corpus_tests;
mod resolved_failure;
mod resolved_features;
pub use resolved_archive::PartResolvedFeaturesArchive;
mod scale_records;
#[cfg(test)]
#[path = "../../tests/resolved_features/scale_schema_records.rs"]
mod scale_schema_record_tests;
mod scene_adapter;
mod schema_references;
#[cfg(test)]
#[path = "../../tests/resolved_features/sensor_records.rs"]
mod sensor_record_tests;
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
#[cfg(test)]
#[path = "../../tests/model_archives/body_features.rs"]
mod body_feature_tests;
mod body_features;
mod boundary_references;
mod center_helpers;
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
#[cfg(test)]
#[path = "../../tests/model_archives/display_dimensions.rs"]
mod display_dimension_tests;
mod display_item;
mod display_object;
mod display_placement;
mod display_records;
mod display_suffixes;
mod entity_representations;
mod environment;
mod explosion;
mod external_references;
mod features;
mod fillet_cache;
#[cfg(test)]
#[path = "../../tests/model_archives/fillet_threads.rs"]
mod fillet_thread_tests;
mod fillets;
mod geometric_references;
mod header_imports;
mod header_order;
mod kernel_frames;
mod label_defaults;
mod layers;
mod lightweight;
mod lightweight_part;
mod lightweight_post;
mod lightweight_shared;
#[cfg(test)]
#[path = "../../tests/resolved_features/line_reference_roles.rs"]
mod line_reference_role_tests;
#[cfg(test)]
#[path = "../../tests/model_archives/linear_patterns.rs"]
mod linear_pattern_tests;
mod linear_patterns;
mod material;
#[cfg(test)]
#[path = "../../tests/resolved_features/mirror_patterns.rs"]
mod mirror_pattern_tests;
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
#[cfg(test)]
#[path = "../../tests/model_archives/profiles.rs"]
mod profile_tests;
mod raw_fields;
mod reader;
mod reference_axes;
#[cfg(test)]
#[path = "../../tests/resolved_features/reference_axes.rs"]
mod reference_axis_tests;
mod reference_planes;
mod reference_tail;
mod relations;
#[cfg(test)]
#[path = "../../tests/model_archives/remaining_roles.rs"]
mod remaining_role_tests;
mod resolved_managers;
mod resolved_start;
mod shared_tail;
#[cfg(test)]
#[path = "../../tests/resolved_features/shell_records.rs"]
mod shell_record_tests;
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
#[cfg(test)]
#[path = "../../tests/model_archives/sketch_roles.rs"]
mod sketch_role_tests;
mod sketch_splines;
mod sketch_surfaces;
mod sketch_tail;
#[cfg(test)]
#[path = "../../tests/model_archives/sketch_tail.rs"]
mod sketch_tail_tests;
mod sketch_text;
mod spatial_objects;
mod spline_arrays;
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

#[cfg(test)]
#[path = "../../tests/model_archives/appearance_fields.rs"]
mod appearance_fields;

#[cfg(test)]
#[path = "../../tests/model_archives/object_fields.rs"]
mod object_fields;

#[cfg(test)]
#[path = "../../tests/model_archives/header_load_plan.rs"]
mod header_load_plan;

#[cfg(test)]
#[path = "../../tests/model_archives/node_units.rs"]
mod node_units;

#[cfg(test)]
#[path = "../../tests/model_archives/spatial_objects.rs"]
mod spatial_object_tests;

#[cfg(test)]
#[path = "../../tests/model_archives/lightweight.rs"]
mod lightweight_tests;

#[cfg(test)]
#[path = "../../tests/model_archives/lightweight_sections.rs"]
mod lightweight_section_tests;

#[cfg(test)]
#[path = "../../tests/model_archives/resolved_start.rs"]
mod resolved_start_tests;

#[cfg(test)]
#[path = "../../tests/model_archives/detail_prefix.rs"]
mod detail_prefix_tests;

#[cfg(test)]
#[path = "../../tests/model_archives/section_bounds.rs"]
mod section_bound_tests;

#[cfg(test)]
#[path = "../../tests/model_archives/dimension_defaults.rs"]
mod dimension_default_tests;

#[cfg(test)]
#[path = "../../tests/model_archives/annotation_table_labels.rs"]
mod annotation_table_label_tests;

#[cfg(test)]
#[path = "../../tests/model_archives/detail_helpers.rs"]
mod detail_helper_tests;

#[cfg(test)]
#[path = "../../tests/model_archives/weld_helpers.rs"]
mod weld_helper_tests;

#[cfg(test)]
#[path = "../../tests/model_archives/center_tail.rs"]
mod center_tail_tests;

#[cfg(test)]
#[path = "../../tests/model_archives/complete_details.rs"]
mod complete_detail_tests;

#[cfg(test)]
#[path = "../../tests/model_archives/environment.rs"]
mod environment_tests;

#[cfg(test)]
#[path = "../../tests/model_archives/transform_rows.rs"]
mod transform_rows;

#[cfg(test)]
#[path = "../../tests/model_archives/views.rs"]
mod view_tests;

#[cfg(test)]
#[path = "../../tests/model_archives/part_archive.rs"]
mod part_archive_tests;

#[cfg(test)]
#[path = "../../tests/model_archives/model_tails.rs"]
mod model_tail_tests;

#[cfg(test)]
#[path = "../../tests/model_archives/native_corpus.rs"]
mod native_corpus_tests;

#[cfg(test)]
#[path = "../../tests/model_archives/annotation_views.rs"]
mod annotation_view_tests;

#[cfg(test)]
#[path = "../../tests/model_archives/component_references.rs"]
mod component_reference_tests;

#[cfg(test)]
#[path = "../../tests/model_archives/boundary_references.rs"]
mod boundary_reference_tests;

#[cfg(test)]
#[path = "../../tests/model_archives/entity_representations.rs"]
mod entity_representation_tests;

#[cfg(test)]
#[path = "../../tests/model_archives/external_references.rs"]
mod external_reference_tests;

#[cfg(test)]
#[path = "../../tests/model_archives/folders_parameters.rs"]
mod folder_parameter_tests;

#[cfg(test)]
#[path = "../../tests/model_archives/position_sheetmetal.rs"]
mod position_sheetmetal_tests;

#[cfg(test)]
#[path = "../../tests/model_archives/relations.rs"]
mod relation_tests;

#[cfg(test)]
#[path = "../../tests/model_archives/model_references.rs"]
mod model_reference_tests;

#[cfg(test)]
#[path = "../../tests/model_archives/radiate_representation.rs"]
mod radiate_representation_tests;

#[cfg(test)]
#[path = "../../tests/model_archives/kernel_frames.rs"]
mod kernel_frame_tests;

#[cfg(test)]
#[path = "../../tests/model_archives/geometric_references.rs"]
mod geometric_reference_tests;

#[cfg(test)]
#[path = "../../tests/model_archives/kernel_rows.rs"]
mod kernel_rows;

#[cfg(test)]
#[path = "../../tests/model_archives/native_kernel_frames.rs"]
mod native_kernel_frame_tests;

#[cfg(test)]
#[path = "../../tests/model_archives/embedded_title.rs"]
mod embedded_title_tests;

#[cfg(test)]
#[path = "../../tests/model_archives/explosion.rs"]
mod explosion_tests;

#[cfg(test)]
#[path = "../../tests/model_archives/dimensions.rs"]
mod dimension_tests;

#[cfg(test)]
#[path = "../../tests/model_archives/sketch_references.rs"]
mod sketch_reference_tests;

#[cfg(test)]
#[path = "../../tests/model_archives/sketch_text.rs"]
mod sketch_text_tests;

#[cfg(test)]
#[path = "../../tests/model_archives/sketch_geometry.rs"]
mod sketch_geometry_tests;

#[cfg(test)]
#[path = "../../tests/model_archives/object_values.rs"]
mod object_value_tests;

#[cfg(test)]
#[path = "../../tests/model_archives/direct_collections.rs"]
mod direct_collection_tests;

#[cfg(test)]
#[path = "../../tests/model_archives/sketch_curves.rs"]
mod sketch_curve_tests;

#[cfg(test)]
#[path = "../../tests/model_archives/sketch_splines.rs"]
mod sketch_spline_tests;

#[cfg(test)]
#[path = "../../tests/model_archives/sketch_dimensions.rs"]
mod sketch_dimension_tests;
