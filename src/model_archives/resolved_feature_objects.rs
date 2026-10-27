use super::ModelDecoder;
use crate::Result;
use crate::archive_codec::ObjectReader;

pub(super) fn read(
    reader: &mut ModelDecoder<'_>,
    map: &mut ObjectReader,
    name: &str,
) -> Option<Result<()>> {
    let result = match name {
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
