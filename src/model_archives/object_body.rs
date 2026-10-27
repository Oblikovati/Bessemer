use super::ModelDecoder;
use crate::Result;
use crate::archive_codec::{ArchiveClass, MaterialRead, ObjectReader};

pub(super) fn read(
    reader: &mut ModelDecoder<'_>,
    map: &mut ObjectReader,
    class: &ArchiveClass,
) -> Result<()> {
    if let Some(kind) = super::body_features::BodyFeatureKind::parse(&class.name) {
        return super::body_features::read(reader, map, kind);
    }
    if let Some(kind) = super::reference_planes::PlaneDataKind::parse(&class.name) {
        return super::reference_planes::data(reader, map, kind);
    }
    if let Some(result) = super::resolved_feature_objects::read(reader, map, &class.name) {
        return result;
    }
    if let Some(result) = super::sweep_objects::read(reader, map, &class.name) {
        return result;
    }
    if let Some(result) = super::configuration_objects::read(reader, map, &class.name) {
        return result;
    }
    if let Some(result) = super::reference_points::read_object(reader, map, &class.name) {
        return result;
    }
    if let Some(result) = super::connection_points::read_object(reader, map, &class.name) {
        return result;
    }
    if let Some(result) = super::reference_geometry_objects::read(reader, map, &class.name) {
        return result;
    }
    if let Some(result) = super::feature_comments::read_object(reader, map, &class.name) {
        return result;
    }
    match class.name.as_str() {
        "moDome_c" => super::dome_records::read(reader, map),
        "moExtruRefSurface_c" => super::extruded_surfaces::read(reader, map),
        "moOrderedFaceArray_c" => super::extruded_surfaces::ordered_faces(reader, map),
        "moExtruRefSurfIdKeeper_c" => super::extruded_surfaces::keeper(reader),
        "moCustomPropertyList" => super::custom_properties::list(reader, "properties"),
        "moSensor_c" => super::sensor_records::read(reader, map),
        "moAttribute_c" | "moAttributeDef_c" => super::attribute_records::read(reader, map),
        "moParameter_c" => super::attribute_records::parameter(reader, map),
        "moScalerParameter_c" => super::attribute_records::scalar_parameter(reader, map),
        "moStringParameter_c" => super::attribute_records::string_parameter(reader, map),
        "Chamfer_c" => super::chamfer_records::read(reader, map),
        "moPlanarSurface_c" => super::planar_surface_records::read(reader, map),
        "moPlanarSurfIdKeeper_c" => super::planar_surface_records::keeper(reader, map),
        "moPerBodyChooserData_c" => super::body_chooser_records::chooser(reader, map),
        "moPerBodyChooserDataWithFileName_c" => {
            super::body_chooser_records::named_chooser(reader, map)
        }
        "moBBoxCenterData_c" => super::body_chooser_records::bounding_center(reader),
        "moProfileFeature_c" | "mo3DProfileFeature_c" | "moOriginProfileFeature_c" => {
            super::profile_features::read(reader, map)
        }
        "moSketchChain_c" => super::profile_chains::chain(reader, map),
        "moOneContour_c" => super::contour_objects::contour(reader, map),
        "moContourChooser_c" => super::contour_objects::chooser(reader, map),
        "moSketchRegion_c" => super::contour_objects::region(reader, map),
        "moEndSpec_c" => super::body_end_specs::end(reader, map),
        "moFromEndSpec_c" => super::body_end_specs::from(reader, map),
        "moRevEndSpec_c" => super::body_end_specs::revolution_end(reader, map),
        _ => read_fillet_thread_body(reader, map, class),
    }
}

fn read_fillet_thread_body(
    reader: &mut ModelDecoder<'_>,
    map: &mut ObjectReader,
    class: &ArchiveClass,
) -> Result<()> {
    use super::fillets;
    match class.name.as_str() {
        "Fillet_c" => fillets::read(reader, map),
        "VarFillet_c" => fillets::variable(reader, map),
        "moFilletSIdKeeper_c" | "moFaceFilletSIdKeeper_c" => fillets::keeper(reader, map),
        "edgeRadiusObject_c" => fillets::edge_radius(reader, map),
        "faceRadiusObject_c" => fillets::face_radius(reader, map),
        "loopRadiusObject_c" => fillets::loop_radius(reader, map),
        "moEdgeDim_c" => fillets::edge_dimension(reader, map),
        "moVertDim_c" => fillets::vertex_dimension(reader, map),
        "moCosmeticThread_c" => super::cosmetic_threads::feature(reader, map),
        "moCThreadData_c" => super::cosmetic_thread_data::read(reader, map),
        "moGeneralSurfRef_w" => super::geometric_references::surface(reader, map),
        _ => read_existing(reader, map, class),
    }
}

fn read_existing(
    reader: &mut ModelDecoder<'_>,
    map: &mut ObjectReader,
    class: &ArchiveClass,
) -> Result<()> {
    if let Some(kind) = super::display_dimension::DisplayDimensionKind::parse(&class.name) {
        return super::display_dimension::read(reader, map, kind);
    }
    if let Some(layout) = super::entity_representations::layout(&class.name) {
        return super::entity_representations::read(reader, map, layout);
    }
    match class.name.as_str() {
        "moNodeName_c" => reader.text("name"),
        "moCStringHandle_c" => reader.text("value"),
        "moExtObject_c" => super::external_references::read(reader, map),
        "moVisualProperties_c" => crate::archive_codec::material::visual(reader),
        "gcCurvatureObject_c" => super::spatial_objects::curvature(reader),
        "moTransRefPlaneData_c" => super::spatial_objects::reference_plane(reader),
        _ => read_component_body(reader, map, class),
    }
}

fn read_component_body(
    reader: &mut ModelDecoder<'_>,
    map: &mut ObjectReader,
    class: &ArchiveClass,
) -> Result<()> {
    use super::component_references as component;
    match class.name.as_str() {
        "moCompRef_c" | "moCompHolder_c" => component::base(reader, map),
        "moCompRefPlane_c" => component::reference_plane(reader, map),
        "moCompFeature_c" | "moCompBodyFeature_c" => component::feature(reader, map),
        "moCompFace_c" | "moCompVertex_c" | "moCompSilhVertex_c" => {
            component::boundary(reader, map)
        }
        "moCompEdge_c" => component::edge(reader, map),
        "moCompLoop_c" => component::loop_body(reader, map),
        "moCompSilhEdge_c" => component::silhouette_edge(reader, map),
        "moCompAtomBody_c" | "moCompSolidBody_c" | "moCompSurfaceBody_c" => {
            component::body(reader, map)
        }
        _ => read_boundary_body(reader, map, class),
    }
}

fn read_boundary_body(
    reader: &mut ModelDecoder<'_>,
    map: &mut ObjectReader,
    class: &ArchiveClass,
) -> Result<()> {
    use super::boundary_references as boundary;
    match class.name.as_str() {
        "moBEntRef_c" | "moEdgeRef_c" => boundary::base(reader, map),
        "moSilhEdgeRef_c" => boundary::silhouette_edge(reader, map),
        "moFaceRef_c" => boundary::face(reader, map),
        "moVertexRef_c" => boundary::vertex(reader, map),
        "moEntityId_c" => boundary::entity_id(reader, map),
        "moFR_c" => super::entity_representations::feature_reference(reader, map),
        _ => read_part_body(reader, map, class),
    }
}

fn read_part_body(
    reader: &mut ModelDecoder<'_>,
    map: &mut ObjectReader,
    class: &ArchiveClass,
) -> Result<()> {
    match class.name.as_str() {
        "moUnitComponent_c" => super::part_objects::unit_component(reader, map),
        "gcXhatch_c" => super::part_objects::hatch(reader),
        "moMaterial_c" => super::part_objects::material(reader, map),
        "moDensityParameter_c" | "moLengthParameter_c" | "moAngleParameter_c" => {
            super::part_objects::density_parameter(reader, map)
        }
        "uoModelData_c" => super::part_objects::model_data(reader, map),
        "moAtom_c" => super::atom::read(reader, map),
        "moRelMgr_c" => super::resolved_managers::relation_manager(reader, map),
        _ => read_geometric_body(reader, map, class),
    }
}

fn read_geometric_body(
    reader: &mut ModelDecoder<'_>,
    map: &mut ObjectReader,
    class: &ArchiveClass,
) -> Result<()> {
    use super::geometric_references as geometry;
    match class.name.as_str() {
        "moGeneralCurveRef_w" => geometry::general_curve(reader, map),
        "moSingleFaceRef_w" => geometry::single_face(reader, map),
        "moCircleRef_w" => geometry::circle(reader, map),
        "moCylinderRef_w" => geometry::cylinder(reader, map),
        "moSphereRef_w" => geometry::sphere(reader, map),
        "moPointRef_w" => geometry::point(reader, map),
        "moLineRef_w" => geometry::line(reader, map),
        "moLoopRef_w" => geometry::loop_body(reader, map),
        _ => read_dimension_body(reader, map, class),
    }
}

fn read_dimension_body(
    reader: &mut ModelDecoder<'_>,
    map: &mut ObjectReader,
    class: &ArchiveClass,
) -> Result<()> {
    use super::dimensions;
    if dimensions::GEOMETRIC.contains(&class.name.as_str()) {
        return dimensions::geometric(reader, map);
    }
    match class.name.as_str() {
        "Dimension_c" => dimensions::base(reader, map),
        "AngleDim_c" => dimensions::angle(reader, map),
        "moRevolvedRadiusDim_c" => dimensions::revolved(reader, map),
        "moIntegerParameter_c" => dimensions::integer_parameter(reader, map),
        "moDimTolerance_c" => dimensions::tolerance(reader),
        "moAttachPoint_c" => dimensions::attachment(reader, map),
        _ => read_dimension_handle_body(reader, map, class),
    }
}

fn read_dimension_handle_body(
    reader: &mut ModelDecoder<'_>,
    map: &mut ObjectReader,
    class: &ArchiveClass,
) -> Result<()> {
    match class.name.as_str() {
        "moFeatureDimHandle_c" | "moRefDimHandle_c" => super::dimension_handles::model(reader, map),
        "moSkDimHandleRadial_c"
        | "moSkDimHandleOffset_c"
        | "moSkDimHandleValG2_c"
        | "moSkDimHandleValG3_c" => super::dimension_handles::sketch(reader, map),
        _ => read_sketch_dimension_body(reader, map, class),
    }
}

fn read_sketch_dimension_body(
    reader: &mut ModelDecoder<'_>,
    map: &mut ObjectReader,
    class: &ArchiveClass,
) -> Result<()> {
    use super::sketch_dimensions as dims;
    let name = class.name.as_str();
    if dims::G1_CLASSES.contains(&name) {
        return dims::valued_one(reader, map);
    }
    if dims::G2_CLASSES.contains(&name) {
        return dims::valued_two(reader, map);
    }
    if dims::OFFSET_CLASSES.contains(&name) {
        return dims::offset(reader, map);
    }
    match name {
        "sgDim" => dims::base(reader, map),
        "sgValG3Dim" => dims::valued_three(reader, map),
        "sgDimEntityHelpData_c" => dims::entity_help(reader),
        _ => read_sketch_logical_body(reader, map, class),
    }
}

fn read_sketch_logical_body(
    reader: &mut ModelDecoder<'_>,
    map: &mut ObjectReader,
    class: &ArchiveClass,
) -> Result<()> {
    use super::{sketch_logical as logical, sketch_patterns as pattern};
    match class.name.as_str() {
        "sgLogDim" => logical::base(reader, map),
        "sgLogG1Dim" => logical::one(reader, map),
        "sgLogG2Dim" => logical::two(reader, map),
        "sgLogG3Dim" => logical::three(reader, map),
        "sgIsoParam_c" => logical::isoparam(reader, map),
        "sgLinearPattern" => pattern::linear(reader, map),
        "sgCircularPattern" => pattern::circular(reader, map),
        "sgLinearPattCon" | "sgCircularPattCon" => pattern::constraint(reader, map),
        _ => read_sketch_tail_body(reader, map, class),
    }
}

fn read_sketch_tail_body(
    reader: &mut ModelDecoder<'_>,
    map: &mut ObjectReader,
    class: &ArchiveClass,
) -> Result<()> {
    use super::{sketch_intersections as intersection, sketch_offsets as offset};
    match class.name.as_str() {
        "sgSketch" => super::sketch_archive::ordinary(reader, map),
        "sgPGMProfileData_c" => offset::profile(reader, map),
        "sgPGMLoopOffsetData_c" => offset::loop_offset(reader, map),
        "sgPGMLoopOffsetEdgeData_c" => offset::loop_edge(reader, map),
        "sgPGMSectionedOffsetEdgeData_c" => offset::sectioned_edge(reader, map),
        "sgPGMSectionHandles_c" => offset::section_handles(reader, map),
        "moExtensionLineIndicator_c" => super::sketch_tail::extension_indicator(reader, map),
        "moIntersectionEdge_c" => intersection::edge(reader, map),
        "moLoopRef_c" => intersection::loop_reference(reader, map),
        "bentRefArr" => intersection::bent_references(reader, map),
        _ => read_sketch_reference_body(reader, map, class),
    }
}

fn read_sketch_reference_body(
    reader: &mut ModelDecoder<'_>,
    map: &mut ObjectReader,
    class: &ArchiveClass,
) -> Result<()> {
    use super::sketch_references as sketch;
    if sketch::HANDLES.contains(&class.name.as_str()) {
        return sketch::handle(reader);
    }
    match class.name.as_str() {
        "moCompSketchEntHandle_c" => sketch::component_handle(reader, map),
        "sgExtEnt_c" => sketch::external_entity(reader, map),
        "moSketchExtRef_w" => sketch::external_reference(reader, map),
        "moTempAxisRef_w" => sketch::temporary_axis(reader, map),
        "moSilhVertexRef_c" => sketch::silhouette_vertex(reader, map),
        "moSkIntersectInfo_c" => sketch::intersection_info(reader),
        "moDimText_c" => super::sketch_text::read(reader, map),
        "utCharFormat_c" => super::detail_prefix::character_format(reader),
        _ => read_sketch_backup_body(reader, map, class),
    }
}

fn read_sketch_backup_body(
    reader: &mut ModelDecoder<'_>,
    map: &mut ObjectReader,
    class: &ArchiveClass,
) -> Result<()> {
    use super::sketch_backups as backup;
    match class.name.as_str() {
        "moPointBackedUpData_c" => backup::point(reader),
        "mo3DPlaneBackedUpData_c" => backup::plane(reader),
        "mo3DSphereBackedUpData_c" => backup::sphere(reader),
        "moLineBackedUpData_c" | "moSegmentBackedUpData_c" => backup::line(reader, map),
        "moArcBackedUpData_c" => backup::arc(reader, map),
        "moEllipseBackedUpData_c" => backup::ellipse(reader, map),
        "sgSlot_c" => super::sketch_entities::slot(reader, map),
        _ => read_explosion_body(reader, map, class),
    }
}

fn read_explosion_body(
    reader: &mut ModelDecoder<'_>,
    map: &mut ObjectReader,
    class: &ArchiveClass,
) -> Result<()> {
    use super::explosion;
    match class.name.as_str() {
        "moPrtExploder_c" => explosion::exploder(reader, map),
        "moPrtExplodeStep_c" => explosion::step(reader, map),
        "moBodyExplodeStep_c" => explosion::body_step(reader, map),
        "moPrtChainExplodeStep_c" => explosion::chain_step(reader, map),
        "moBodyXform_c" => explosion::body_transform(reader, map),
        "moPrtExplodeLine_c" => explosion::line(reader, map),
        "moDirectionSpec_c" => explosion::direction(reader, map),
        _ => read_relation_body(reader, map, class),
    }
}

fn read_relation_body(
    reader: &mut ModelDecoder<'_>,
    map: &mut ObjectReader,
    class: &ArchiveClass,
) -> Result<()> {
    use super::relations;
    match class.name.as_str() {
        "moRefWrapper_c" => super::model_references::wrapper(reader, map),
        "moDimRefWrapper_c" => super::model_references::dimension(reader, map),
        "moGlobalVarRefWrapper_c" => super::model_references::global_variable(reader, map),
        "moCompXFormWrapper_c" => super::model_references::component_transform(reader, map),
        "moRelation_c" => relations::relation(reader, map),
        "moRelEquationSide_c" => relations::equation_side(reader, map),
        "moRelElement_c" | "moRelComment_c" | "moRelOperator_c" | "moRelValue_c"
        | "moRelFunction_c" => relations::element(reader),
        "moRelDimension_c" => relations::dimension(reader, map),
        "moRelGlobalVar_c" => relations::global_variable(reader, map),
        "moCompGlobalVar_c" => relations::component_variable(reader, map),
        _ => read_scene_body(reader, map, class),
    }
}

fn read_scene_body(
    reader: &mut ModelDecoder<'_>,
    map: &mut ObjectReader,
    class: &ArchiveClass,
) -> Result<()> {
    match class.name.as_str() {
        "moEnvFolder_c"
        | "moFolder_c"
        | "moCommentsFolder_c"
        | "moConfigCommentsFolder_c"
        | "moSensorFolder_c"
        | "moDocsFolder_c"
        | "moInkMarkupFolder_c"
        | "moEqnFolder_c"
        | "moConfigTableFolder_c" => super::features::folder(reader, map),
        "moFavoriteFolder_c" => super::browser_folders::favorite(reader, map),
        "moFavoriteItemData_c" | "moHistoryItemData_c" => super::browser_folders::item(reader, map),
        "moFavoriteFeatItemData_c" | "moHistoryFeatItemData_c" => {
            super::browser_folders::feature_item(reader, map)
        }
        "moLiveSectionFolder_c" | "moPlaneSectionFolder_c" => {
            super::browser_folders::live_section(reader, map)
        }
        "moHistoryFolder_c" => super::browser_folders::history(reader, map),
        "moSelectionSetFolder_c" => super::browser_folders::selection_sets(reader, map),
        "moSurfaceBodyFolder_c" | "moSolidBodyFolder_c" => {
            super::browser_folders::body_folder(reader, map)
        }
        "moMaterialFolder_c" => super::browser_folders::material(reader, map),
        "moDetailCabinet_c" => super::browser_folders::detail_cabinet(reader, map),
        "moPosGroupFolder_c" => super::position_groups::read(reader, map),
        "moMateReferenceGroupFolder_c" | "moFtrFolder_c" | "moNotesAreaFtrFolder_c" => {
            super::features::feature_group(reader, map, class.name == "moNotesAreaFtrFolder_c")
        }
        "moView_c" => super::views::view(reader, map),
        "moAnnotationView_c" => super::annotation_view::read(reader, map),
        "moSketchBlockMgr_c" => super::post_views::sketch_blocks(reader, map),
        "moFeatColorTab_c" => super::post_views::feature_colors(reader),
        "moFlatPatternColorTab_c" => super::post_views::flat_colors(reader),
        "moLightFeature_c" | "moAmbientLight_c" | "moDirectionLight_c" | "moSpotLight_c"
        | "moPointLight_c" => super::environment::light(reader, map, &class.name),
        _ => read_tail_body(reader, map, class),
    }
}

fn read_tail_body(
    reader: &mut ModelDecoder<'_>,
    map: &mut ObjectReader,
    class: &ArchiveClass,
) -> Result<()> {
    match class.name.as_str() {
        "moLayer_c" => super::layers::layer(reader),
        "moLayerMask_c" => super::layers::mask(reader),
        "moLayerMgr_c" => super::layers::manager(reader, map),
        "moCThreadRefMgr_c" => super::shared_tail::thread_manager(reader, map),
        "moBendNoteFormat_c" => super::shared_tail::bend_format(reader, map),
        "moPrtExplViewManager_c" => super::part_tail::exploded_manager(reader, map),
        "moExplPrtId_c" => super::part_tail::exploded_id(reader),
        "moCachedBoundingBoxData_c" => super::part_tail::cached_box(reader),
        "moPerBodySheetMetalDataNew_c" => super::part_tail::sheet_metal(reader, map),
        "moPerBodySheetMetalData_c" => super::reference_tail::per_body_sheet_metal(reader, map),
        "moForwardedFeatRefs_c" => super::reference_tail::forwarded_feature(reader),
        "moPMarkRecord_c" => super::reference_tail::permanent_mark(reader),
        _ => read_detail_body(reader, map, class),
    }
}

fn read_detail_body(
    reader: &mut ModelDecoder<'_>,
    map: &mut ObjectReader,
    class: &ArchiveClass,
) -> Result<()> {
    match class.name.as_str() {
        "moSFDataHelper_c" => super::detail_helpers::surface_finish(reader),
        "moNoteDataHelper_c" => super::detail_helpers::note(reader, map),
        "moGTolDataHelper_c" => super::weld_helpers::gtol(reader, map),
        "moGTolDlgDataTol_c" => super::tolerance_helpers::read(reader),
        "moWeldDataHelper_c" => super::weld_helpers::weld(reader),
        "suObList" | "suObArray" | "su_CObArray" | "su_CObList" | "sgSplineList_c"
        | "modifSplineList_c" | "cptsSplineList_c" => {
            super::model_lists::pointers(reader, map, "items", super::ModelObjectRole::Any)
        }
        "su_CDWordArray" => super::model_lists::unsigned_words(reader, "items"),
        "moDetailDefs_c" => super::detail_definitions::read(reader, map),
        _ => read_display_support(reader, map, class),
    }
}

fn read_display_support(
    reader: &mut ModelDecoder<'_>,
    map: &mut ObjectReader,
    class: &ArchiveClass,
) -> Result<()> {
    match class.name.as_str() {
        "moDispObjRef_c" => super::display_object::reference(reader),
        "moChainComponent_c" => super::context_chains::component(reader, map),
        "moContextChainComponent_c" => super::context_chains::context_component(reader, map),
        "moContextUnitComponent_c" => super::context_chains::unit(reader, map),
        "moReferenceChain_c" => super::context_chains::reference_chain(reader, map),
        "moExtFeatureList_c" => super::context_chains::feature_list(reader, map),
        "moExtFeature_c" => super::context_chains::external_feature(reader, map),
        "moPLine_c" => super::polyline_records::read(reader, map),
        "moPLineData_c" => super::polyline_records::data(reader, map),
        "moPLineProject_c" => super::polyline_records::project(reader, map),
        "su_CWordArray" => super::polyline_records::word_array(reader),
        "moScale_c" => super::scale_records::read(reader, map),
        "moCoordSysRef_w" => super::scale_records::coordinate_system(reader, map),
        "moSwiftSchemaRef_c" => super::schema_references::read(reader),
        "moCutListSortOptions_c" => super::browser_folders::cut_list_sort(reader, map),
        "moFaceColorConfig_c" => super::configuration_atoms::face_color(reader, map),
        "moDetailFolder_c" => super::detail_folder_records::read(reader, map),
        "gcBucketEdgeData_c" => crate::archive_codec::bucket_polygons::edge(reader),
        "moCompDispObjRef_c" => super::display_object::component_reference(reader, map),
        "moArrowSize_c" => super::display_object::arrow_size(reader),
        "moFavoriteHandle_c" => super::display_dimension_tail::favorite_handle(reader),
        "moHwCalloutData_c" => super::display_suffixes::callout_data(reader, map),
        name => super::units::read_class(reader, map, name, class.schema),
    }
}
