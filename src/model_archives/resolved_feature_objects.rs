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
