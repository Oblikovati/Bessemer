use super::ModelDecoder;
use crate::Result;
use crate::archive_codec::ObjectReader;

pub(super) fn read(
    reader: &mut ModelDecoder<'_>,
    map: &mut ObjectReader,
    name: &str,
) -> Option<Result<()>> {
    if let Some(result) = super::cut_list_folders::read_object(reader, map, name) {
        return Some(result);
    }
    if let Some(result) = super::reference_dimensions::read(reader, map, name) {
        return Some(result);
    }
    if let Some(result) = super::structural_member_objects::read(reader, map, name) {
        return Some(result);
    }
    if let Some(result) = super::fold_transforms::read_object(reader, map, name) {
        return Some(result);
    }
    let result = match name {
        "moFillRefSurface_c" => super::filled_surfaces::read(reader, map),
        "moFillRefSurfIdKeeper_c" => super::filled_surfaces::keeper(reader, map),
        "moCurveContact_c" => super::filled_surfaces::contact(reader, map),
        "moCurveContactData_c" => super::filled_surfaces::contact_data(reader, map),
        "moSewRefSurface_c" => super::sew_surfaces::read(reader, map),
        // #54 / C-269 binds this exact zero-byte su_CObject serializer thunk.
        "moSewRefSurfIdKeeper_c" => super::extruded_surfaces::keeper(reader),
        "moNetBlend_c" | "moNetBlendCut_c" => super::net_blends::read(reader, map),
        "moContactData_c" => super::net_blend_components::contact(reader, map),
        "moCompEntGroup_c" => super::net_blend_components::group(reader, map),
        "moCompMultiEdge_c" => super::net_blend_components::multi_edge(reader, map),
        // #54 / 11.350 / C-392 binds both suffix-free native feature dispatches.
        "moLockFeature_c" => super::features::read(reader, map),
        "moSMEnvTableFeat_c" => super::body_features::model_feature(reader, map),
        "moLPatternItemId_c" => super::frontier_feature_helpers::pattern_item(reader),
        "edgeParamsObject_c" => super::frontier_feature_helpers::edge_parameters(reader, map),
        "featRadiusObject_c" => super::frontier_feature_helpers::feature_radius(reader, map),
        "moRefCurve_c" => super::reference_curves::read(reader, map),
        "moSurfCut_c" => super::surface_operations::cut(reader, map),
        "moTrimRefSurface_c" => super::surface_operations::trim(reader, map),
        "moCombineBodies_c" => super::combine_bodies::combine(reader, map),
        "moDeleteBody_c" => super::combine_bodies::delete(reader, map),
        "moDeleteBodyData_c" => super::combine_bodies::delete_data(reader, map),
        "moMirrorSolid_c" => super::mirror_solids::read(reader, map),
        "suUIntArray" => super::mirror_solids::unsigned_array(reader),
        "moWeldCornerFeat_c" => super::weld_corners::read(reader, map),
        "moWeldBodyData_c" => super::weld_corners::body_data(reader, map),
        "moWeldTrimmedBodyData_c" => super::weld_corners::trimmed(reader, map),
        "moWeldTrimToolData_c" => super::weld_corners::tool(reader, map),
        "moDraft_c" => super::draft_records::read(reader, map),
        "moDraftSIdKeeper_c" | "moEdgeFaceSIdKeeper_c" => super::draft_keepers::read(reader, map),
        "moSplineTangencyControl_c" => super::spline_controls::tangency(reader, map),
        "moSplineControlPolygonControl_c" => super::spline_controls::polygon(reader, map),
        "moSplineCurvatureControl_c" => super::spline_controls::curvature(reader, map),
        "moWeldmentFeature_c" => super::weldment_records::read(reader, map),
        "moCustomProperty" => super::custom_properties::record(reader),
        "moBlend_c" | "moBlendCut_c" => super::blend_records::read(reader, map),
        "moFaceBlendItemPair_c" => super::blend_records::face_pair(reader, map),
        "moFaceBlendItem_c" => super::blend_records::face_item(reader, map),
        "moFlatPattern_c" => super::flat_patterns::read(reader, map),
        "moFlatPatBoundBox_c" => super::flat_pattern_boxes::read(reader),
        "moBoundBoxData_c" => super::flat_pattern_boxes::contents(reader),
        "moSMBaseFlange_c" => super::base_flanges::read(reader, map),
        "moSheetMetal_c"
        | "moTemplateSheetMetal_c"
        | "moFixedFaceInFlat_c"
        | "moCompSheetmetalFeat_c" => super::sheet_metal::read(reader, map, name),
        "moLPattern_c" => super::linear_patterns::read(reader, map),
        "moMirrorPattern_c" => super::mirror_patterns::read(reader, map),
        "moShell_c" => super::shell_records::read(reader, map),
        "moCirPattern_c" => super::circular_patterns::read(reader, map),
        "moHoleWzd_c" => super::hole_wizard::read(reader, map),
        "moImported_c" => super::imported_bodies::read(reader, map, false),
        "moBaseBody_c" => super::imported_bodies::read(reader, map, true),
        "moSlotCenterAxisData_c" => super::hole_wizard::slot_axis(reader),
        "moMatrixIndexHelper_c" => super::linear_patterns::matrix_helper(reader, map),
        _ => return None,
    };
    Some(result)
}
