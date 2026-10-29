// Keep the existing sibling namespace because private tests share parser helpers.
#[cfg(test)]
#[path = "../../tests/model_archives/annotation_table_labels.rs"]
mod annotation_table_label_tests;
#[cfg(test)]
#[path = "../../tests/model_archives/annotation_views.rs"]
mod annotation_view_tests;
#[cfg(test)]
#[path = "../../tests/model_archives/appearance_fields.rs"]
mod appearance_fields;
#[cfg(test)]
#[path = "../../tests/resolved_features/application_names.rs"]
mod application_name_tests;
#[cfg(test)]
#[path = "../../tests/resolved_features/attribute_records.rs"]
mod attribute_record_tests;
#[cfg(test)]
#[path = "../../tests/resolved_features/reference_axis_owners.rs"]
mod axis_owner_tests;
#[cfg(test)]
#[path = "../../tests/resolved_features/base_flanges.rs"]
mod base_flange_tests;
#[cfg(test)]
#[path = "../../tests/resolved_features/blends.rs"]
mod blend_record_tests;
#[cfg(test)]
#[path = "../../tests/resolved_features/blend_reference_surfaces.rs"]
mod blend_reference_surface_tests;
#[cfg(test)]
#[path = "../../tests/resolved_features/body_chooser_records.rs"]
mod body_chooser_record_tests;
#[cfg(test)]
#[path = "../../tests/model_archives/body_features.rs"]
mod body_feature_tests;
#[cfg(test)]
#[path = "../../tests/resolved_features/body_state_roles.rs"]
mod body_state_role_tests;
#[cfg(test)]
#[path = "../../tests/model_archives/boundary_references.rs"]
mod boundary_reference_tests;
#[cfg(test)]
#[path = "../../tests/resolved_features/browser_items.rs"]
mod browser_item_tests;
#[cfg(test)]
#[path = "../../tests/model_archives/center_tail.rs"]
mod center_tail_tests;
#[cfg(test)]
#[path = "../../tests/resolved_features/chamfer_records.rs"]
mod chamfer_record_tests;
#[cfg(test)]
#[path = "../../tests/resolved_features/circular_patterns.rs"]
mod circular_pattern_tests;
#[cfg(test)]
#[path = "../../tests/resolved_features/combine_bodies.rs"]
mod combine_body_tests;
#[cfg(test)]
#[path = "../../tests/model_archives/complete_details.rs"]
mod complete_detail_tests;
#[cfg(test)]
#[path = "../../tests/resolved_features/component_coordinates.rs"]
mod component_coordinate_tests;
#[cfg(test)]
#[path = "../../tests/resolved_features/component_geometry_roles.rs"]
mod component_geometry_role_tests;
#[cfg(test)]
#[path = "../../tests/resolved_features/component_geometry.rs"]
mod component_geometry_tests;
#[cfg(test)]
#[path = "../../tests/resolved_features/component_reference_curves.rs"]
mod component_reference_curve_tests;
#[cfg(test)]
#[path = "../../tests/model_archives/component_references.rs"]
mod component_reference_tests;
#[cfg(test)]
#[path = "../../tests/resolved_features/configuration_atoms.rs"]
mod configuration_atom_tests;
#[cfg(test)]
#[path = "../../tests/resolved_features/configuration_graph.rs"]
mod configuration_graph_tests;
#[cfg(test)]
#[path = "../../tests/resolved_features/configuration_io.rs"]
mod configuration_io_tests;
#[cfg(test)]
#[path = "../../tests/resolved_features/configuration_scenes.rs"]
mod configuration_scene_tests;
#[cfg(test)]
#[path = "../../tests/resolved_features/configuration_tables.rs"]
mod configuration_table_tests;
#[cfg(test)]
#[path = "../../tests/resolved_features/configuration_toolbox.rs"]
mod configuration_toolbox_tests;
#[cfg(test)]
#[path = "../../tests/resolved_features/connection_points.rs"]
mod connection_point_tests;
#[cfg(test)]
#[path = "../../tests/resolved_features/context_chains.rs"]
mod context_chain_tests;
#[cfg(test)]
#[path = "../../tests/resolved_features/context_snapshot.rs"]
mod context_snapshot_tests;
#[cfg(test)]
#[path = "../../tests/resolved_features/coordinate_systems.rs"]
mod coordinate_system_tests;
#[cfg(test)]
#[path = "../../tests/resolved_features/corner_hems.rs"]
mod corner_hem_tests;
#[cfg(test)]
#[path = "../../tests/resolved_features/custom_properties.rs"]
mod custom_property_tests;
#[cfg(test)]
#[path = "../../tests/resolved_features/cut_list_folders.rs"]
mod cut_list_folder_tests;
#[cfg(test)]
#[path = "../../tests/resolved_features/detail_folder_records.rs"]
mod detail_folder_record_tests;
#[cfg(test)]
#[path = "../../tests/model_archives/detail_helpers.rs"]
mod detail_helper_tests;
#[cfg(test)]
#[path = "../../tests/model_archives/detail_prefix.rs"]
mod detail_prefix_tests;
#[cfg(test)]
#[path = "../../tests/model_archives/dimension_defaults.rs"]
mod dimension_default_tests;
#[cfg(test)]
#[path = "../../tests/model_archives/dimensions.rs"]
mod dimension_tests;
#[cfg(test)]
#[path = "../../tests/model_archives/direct_collections.rs"]
mod direct_collection_tests;
#[cfg(test)]
#[path = "../../tests/model_archives/display_dimensions.rs"]
mod display_dimension_tests;
#[cfg(test)]
#[path = "../../tests/resolved_features/domes.rs"]
mod dome_record_tests;
#[cfg(test)]
#[path = "../../tests/resolved_features/drafts.rs"]
mod draft_record_tests;
#[cfg(test)]
#[path = "../../tests/resolved_features/draft_surface_identifiers.rs"]
mod draft_surface_identifier_tests;
#[cfg(test)]
#[path = "../../tests/model_archives/embedded_title.rs"]
mod embedded_title_tests;
#[cfg(test)]
#[path = "../../tests/resolved_features/end_spec_point_roles.rs"]
mod end_spec_point_role_tests;
#[cfg(test)]
#[path = "../../tests/model_archives/entity_representations.rs"]
mod entity_representation_tests;
#[cfg(test)]
#[path = "../../tests/model_archives/environment.rs"]
mod environment_tests;
#[cfg(test)]
#[path = "../../tests/model_archives/explosion.rs"]
mod explosion_tests;
#[cfg(test)]
#[path = "../../tests/model_archives/external_references.rs"]
mod external_reference_tests;
#[cfg(test)]
#[path = "../../tests/resolved_features/extruded_surfaces.rs"]
mod extruded_surface_tests;
#[cfg(test)]
#[path = "../../tests/resolved_features/face_blend_items.rs"]
mod face_blend_item_tests;
#[cfg(test)]
#[path = "../../tests/resolved_features/feature_comments.rs"]
mod feature_comment_tests;
#[cfg(test)]
#[path = "../../tests/resolved_features/feature_states.rs"]
mod feature_state_tests;
#[cfg(test)]
#[path = "../../tests/resolved_features/filled_surfaces.rs"]
mod filled_surface_tests;
#[cfg(test)]
#[path = "../../tests/model_archives/fillet_threads.rs"]
mod fillet_thread_tests;
#[cfg(test)]
#[path = "../../tests/resolved_features/flat_patterns.rs"]
mod flat_pattern_tests;
#[cfg(test)]
#[path = "../../tests/resolved_features/flatten_bends.rs"]
mod flatten_bend_tests;
#[cfg(test)]
#[path = "../../tests/resolved_features/fold_transforms.rs"]
mod fold_transform_tests;
#[cfg(test)]
#[path = "../../tests/resolved_features/folder_configuration_records.rs"]
mod folder_configuration_record_tests;
#[cfg(test)]
#[path = "../../tests/model_archives/folders_parameters.rs"]
mod folder_parameter_tests;
#[cfg(test)]
#[path = "../../tests/resolved_features/frontier_edge_identifiers.rs"]
mod frontier_edge_identifier_tests;
#[cfg(test)]
#[path = "../../tests/resolved_features/frontier_feature_helpers.rs"]
mod frontier_feature_helper_tests;
#[cfg(test)]
#[path = "../../tests/model_archives/geometric_references.rs"]
mod geometric_reference_tests;
#[cfg(test)]
#[path = "../../tests/model_archives/header_load_plan.rs"]
mod header_load_plan;
#[cfg(test)]
#[path = "../../tests/resolved_features/helices.rs"]
mod helix_record_tests;
#[cfg(test)]
#[path = "../../tests/resolved_features/hole_wizard.rs"]
mod hole_wizard_tests;
#[cfg(test)]
#[path = "../../tests/resolved_features/imported_bodies.rs"]
mod imported_body_tests;
#[cfg(test)]
#[path = "../../tests/resolved_features/jog_corners.rs"]
mod jog_corner_tests;
#[cfg(test)]
#[path = "../../tests/model_archives/kernel_frames.rs"]
mod kernel_frame_tests;
#[cfg(test)]
#[path = "../../tests/model_archives/kernel_rows.rs"]
mod kernel_rows;
#[cfg(test)]
#[path = "../../tests/model_archives/lightweight_sections.rs"]
mod lightweight_section_tests;
#[cfg(test)]
#[path = "../../tests/model_archives/lightweight.rs"]
mod lightweight_tests;
#[cfg(test)]
#[path = "../../tests/resolved_features/line_reference_roles.rs"]
mod line_reference_role_tests;
#[cfg(test)]
#[path = "../../tests/model_archives/linear_patterns.rs"]
mod linear_pattern_tests;
#[cfg(test)]
#[path = "../../tests/resolved_features/manager_context.rs"]
mod manager_context_tests;
#[cfg(test)]
#[path = "../../tests/resolved_features/manager_oracle.rs"]
mod manager_oracle_tests;
#[cfg(test)]
#[path = "../../tests/resolved_features/mirror_patterns.rs"]
mod mirror_pattern_tests;
#[cfg(test)]
#[path = "../../tests/resolved_features/mirror_solids.rs"]
mod mirror_solid_tests;
#[cfg(test)]
#[path = "../../tests/model_archives/model_references.rs"]
mod model_reference_tests;
#[cfg(test)]
#[path = "../../tests/model_archives/model_tails.rs"]
mod model_tail_tests;
#[cfg(test)]
#[path = "../../tests/model_archives/native_corpus.rs"]
mod native_corpus_tests;
#[cfg(test)]
#[path = "../../tests/model_archives/native_kernel_frames.rs"]
mod native_kernel_frame_tests;
#[cfg(test)]
#[path = "../../tests/resolved_features/net_blends.rs"]
mod net_blend_tests;
#[cfg(test)]
#[path = "../../tests/model_archives/node_units.rs"]
mod node_units;
#[cfg(test)]
#[path = "../../tests/model_archives/object_fields.rs"]
mod object_fields;
#[cfg(test)]
#[path = "../../tests/model_archives/object_values.rs"]
mod object_value_tests;
#[cfg(test)]
#[path = "../../tests/resolved_features/offset_surfaces.rs"]
mod offset_surface_tests;
#[cfg(test)]
#[path = "../../tests/resolved_features/owned_features.rs"]
mod owned_resolved_feature_tests;
#[cfg(test)]
#[path = "../../tests/model_archives/part_archive.rs"]
mod part_archive_tests;
#[cfg(test)]
#[path = "../../tests/resolved_features/planar_surface_records.rs"]
mod planar_surface_record_tests;
#[cfg(test)]
#[path = "../../tests/resolved_features/plane_point_roles.rs"]
mod plane_point_role_tests;
#[cfg(test)]
#[path = "../../tests/resolved_features/polyline_records.rs"]
mod polyline_record_tests;
#[cfg(test)]
#[path = "../../tests/model_archives/position_sheetmetal.rs"]
mod position_sheetmetal_tests;
#[cfg(test)]
#[path = "../../tests/model_archives/profiles.rs"]
mod profile_tests;
#[cfg(test)]
#[path = "../../tests/model_archives/public_roles.rs"]
mod public_role_tests;
#[cfg(test)]
#[path = "../../tests/model_archives/radiate_representation.rs"]
mod radiate_representation_tests;
#[cfg(test)]
#[path = "../../tests/resolved_features/radiated_edge_identifiers.rs"]
mod radiated_edge_identifier_tests;
#[cfg(test)]
#[path = "../../tests/resolved_features/reference_axes.rs"]
mod reference_axis_tests;
#[cfg(test)]
#[path = "../../tests/resolved_features/reference_curves.rs"]
mod reference_curve_tests;
#[cfg(test)]
#[path = "../../tests/resolved_features/reference_dimensions.rs"]
mod reference_dimension_tests;
#[cfg(test)]
#[path = "../../tests/resolved_features/reference_points.rs"]
mod reference_point_tests;
#[cfg(test)]
#[path = "../../tests/model_archives/relations.rs"]
mod relation_tests;
#[cfg(test)]
#[path = "../../tests/model_archives/remaining_roles.rs"]
mod remaining_role_tests;
#[cfg(test)]
#[path = "../../tests/resolved_features/native_corpus.rs"]
mod resolved_corpus_tests;
#[cfg(test)]
#[path = "../../tests/model_archives/resolved_start.rs"]
mod resolved_start_tests;
#[cfg(test)]
#[path = "../../tests/resolved_features/revolved_surfaces.rs"]
mod revolved_surface_tests;
#[cfg(test)]
#[path = "../../tests/resolved_features/scale_schema_records.rs"]
mod scale_schema_record_tests;
#[cfg(test)]
#[path = "../../tests/model_archives/section_bounds.rs"]
mod section_bound_tests;
#[cfg(test)]
#[path = "../../tests/resolved_features/sensor_records.rs"]
mod sensor_record_tests;
#[cfg(test)]
#[path = "../../tests/resolved_features/sew_surfaces.rs"]
mod sew_surface_tests;
#[cfg(test)]
#[path = "../../tests/resolved_features/shared_sketch_handles.rs"]
mod shared_sketch_handle_tests;
#[cfg(test)]
#[path = "../../tests/resolved_features/sheet_folds.rs"]
mod sheet_fold_tests;
#[cfg(test)]
#[path = "../../tests/resolved_features/sheet_metal.rs"]
mod sheet_metal_tests;
#[cfg(test)]
#[path = "../../tests/resolved_features/shell_records.rs"]
mod shell_record_tests;
#[cfg(test)]
#[path = "../../tests/model_archives/sketch_curves.rs"]
mod sketch_curve_tests;
#[cfg(test)]
#[path = "../../tests/model_archives/sketch_dimensions.rs"]
mod sketch_dimension_tests;
#[cfg(test)]
#[path = "../../tests/resolved_features/sketch_geometry_backups.rs"]
mod sketch_geometry_backup_tests;
#[cfg(test)]
#[path = "../../tests/model_archives/sketch_geometry.rs"]
mod sketch_geometry_tests;
#[cfg(test)]
#[path = "../../tests/model_archives/sketch_references.rs"]
mod sketch_reference_tests;
#[cfg(test)]
#[path = "../../tests/model_archives/sketch_roles.rs"]
mod sketch_role_tests;
#[cfg(test)]
#[path = "../../tests/model_archives/sketch_splines.rs"]
mod sketch_spline_tests;
#[cfg(test)]
#[path = "../../tests/model_archives/sketch_tail.rs"]
mod sketch_tail_tests;
#[cfg(test)]
#[path = "../../tests/model_archives/sketch_text.rs"]
mod sketch_text_tests;
#[cfg(test)]
#[path = "../../tests/resolved_features/sketch_wrapper_roles.rs"]
mod sketch_wrapper_role_tests;
#[cfg(test)]
#[path = "../../tests/resolved_features/solid_references.rs"]
mod solid_reference_tests;
#[cfg(test)]
#[path = "../../tests/model_archives/spatial_objects.rs"]
mod spatial_object_tests;
#[cfg(test)]
#[path = "../../tests/resolved_features/spline_controls.rs"]
mod spline_control_tests;
#[cfg(test)]
#[path = "../../tests/resolved_features/structural_members.rs"]
mod structural_member_tests;
#[cfg(test)]
#[path = "../../tests/resolved_features/surface_operations.rs"]
mod surface_operation_tests;
#[cfg(test)]
#[path = "../../tests/resolved_features/sweeps.rs"]
mod sweep_record_tests;
#[cfg(test)]
#[path = "../../tests/resolved_features/thickening.rs"]
mod thickening_tests;
#[cfg(test)]
#[path = "../../tests/model_archives/transform_rows.rs"]
mod transform_rows;
#[cfg(test)]
#[path = "../../tests/resolved_features/trim_edge_identifiers.rs"]
mod trim_edge_identifier_tests;
#[cfg(test)]
#[path = "../../tests/resolved_features/ui_bends.rs"]
mod ui_bend_tests;
#[cfg(test)]
#[path = "../../tests/model_archives/views.rs"]
mod view_tests;
#[cfg(test)]
#[path = "../../tests/resolved_features/weld_corners.rs"]
mod weld_corner_tests;
#[cfg(test)]
#[path = "../../tests/model_archives/weld_helpers.rs"]
mod weld_helper_tests;
#[cfg(test)]
#[path = "../../tests/resolved_features/weldments.rs"]
mod weldment_record_tests;

#[cfg(test)]
#[path = "../../tests/resolved_features/copy_edge_identifiers.rs"]
mod copy_edge_identifier_tests;

#[cfg(test)]
#[path = "../../tests/resolved_features/sweep_surfaces.rs"]
mod sweep_surface_tests;

#[cfg(test)]
#[path = "../../tests/resolved_features/split_features.rs"]
mod split_feature_tests;

#[cfg(test)]
#[path = "../../tests/resolved_features/silhouette_edges.rs"]
mod silhouette_edge_tests;

#[cfg(test)]
#[path = "../../tests/resolved_features/projected_line_subtypes.rs"]
mod projected_line_subtype_tests;

#[cfg(test)]
#[path = "../../tests/resolved_features/constraint_planes.rs"]
mod constraint_plane_tests;

#[cfg(test)]
#[path = "../../tests/resolved_features/thin_extrusions.rs"]
mod thin_extrusion_tests;

#[cfg(test)]
#[path = "../../tests/resolved_features/three_point_plane_roles.rs"]
mod three_point_plane_role_tests;

#[cfg(test)]
#[path = "../../tests/resolved_features/deleted_faces.rs"]
mod deleted_face_tests;

#[cfg(test)]
#[path = "../../tests/resolved_features/end_spec_wrapper_roles.rs"]
mod end_spec_wrapper_role_tests;

#[cfg(test)]
#[path = "../../tests/resolved_features/composite_curves.rs"]
mod composite_curve_tests;

#[cfg(test)]
#[path = "../../tests/resolved_features/ruled_surfaces.rs"]
mod ruled_surface_tests;

#[cfg(test)]
#[path = "../../tests/resolved_features/intersection_wrapper_roles.rs"]
mod intersection_wrapper_role_tests;

#[cfg(test)]
#[path = "../../tests/resolved_features/fixed_valued_transforms.rs"]
mod fixed_valued_transform_tests;

#[cfg(test)]
#[path = "../../tests/resolved_features/additional_valued_dimensions.rs"]
mod additional_valued_dimension_tests;

#[cfg(test)]
#[path = "../../tests/resolved_features/sketch_isocurve_information.rs"]
mod sketch_isocurve_information_tests;

#[cfg(test)]
#[path = "../../tests/resolved_features/cone_references.rs"]
mod cone_reference_tests;

#[cfg(test)]
#[path = "../../tests/resolved_features/surface_identifier_variants.rs"]
mod surface_identifier_variant_tests;

#[cfg(test)]
#[path = "../../tests/resolved_features/display_welds.rs"]
mod display_weld_tests;

#[cfg(test)]
#[path = "../../tests/resolved_features/component_detail_references.rs"]
mod component_detail_reference_tests;

#[cfg(test)]
#[path = "../../tests/resolved_features/component_detail_provenance.rs"]
mod component_detail_provenance_tests;

#[cfg(test)]
#[path = "../../tests/resolved_features/weld_symbols.rs"]
mod weld_symbol_tests;

#[cfg(test)]
#[path = "../../tests/resolved_features/absolute_plane_data.rs"]
mod absolute_plane_data_tests;
#[cfg(test)]
#[path = "../../tests/resolved_features/attachment_records.rs"]
mod attachment_record_tests;
#[cfg(test)]
#[path = "../../tests/resolved_features/cosmetic_weld_cut_lists.rs"]
mod cosmetic_weld_cut_list_tests;
#[cfg(test)]
#[path = "../../tests/resolved_features/cosmetic_weld_tree.rs"]
mod cosmetic_weld_tree_tests;
#[cfg(test)]
#[path = "../../tests/resolved_features/edge_flanges.rs"]
mod edge_flange_tests;
#[cfg(test)]
#[path = "../../tests/resolved_features/end_caps.rs"]
mod end_cap_tests;
#[cfg(test)]
#[path = "../../tests/resolved_features/weldment_finishing.rs"]
mod weldment_finishing_tests;

#[cfg(test)]
#[path = "../../tests/resolved_features/legacy_manager_context.rs"]
mod legacy_manager_context_tests;

#[cfg(test)]
#[path = "../../tests/resolved_features/display_notes.rs"]
mod display_note_tests;

#[cfg(test)]
#[path = "../../tests/resolved_features/stock_features.rs"]
mod stock_feature_tests;

#[cfg(test)]
#[path = "../../tests/resolved_features/body_reference_wrappers.rs"]
mod body_reference_wrapper_tests;

#[cfg(test)]
#[path = "../../tests/resolved_features/derived_references.rs"]
mod derived_reference_tests;

#[cfg(test)]
#[path = "../../tests/resolved_features/move_copy_bodies.rs"]
mod move_copy_body_tests;

#[cfg(test)]
#[path = "../../tests/resolved_features/selection_sets.rs"]
mod selection_set_tests;

#[cfg(test)]
#[path = "../../tests/resolved_features/sketch_bitmaps.rs"]
mod sketch_bitmap_tests;

#[cfg(test)]
#[path = "../../tests/resolved_features/sketch_bends.rs"]
mod sketch_bend_tests;

#[cfg(test)]
#[path = "../../tests/resolved_features/mitered_flanges.rs"]
mod mitered_flange_tests;
