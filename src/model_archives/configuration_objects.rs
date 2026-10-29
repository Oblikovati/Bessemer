use super::ModelDecoder;
use crate::Result;
use crate::archive_codec::ObjectReader;

pub(super) fn read(
    reader: &mut ModelDecoder<'_>,
    map: &mut ObjectReader,
    name: &str,
) -> Option<Result<()>> {
    let result = match name {
        "moPWMaterialProperties_c" => super::photoworks_materials::properties(reader, map),
        "mo3dView_c" => super::three_dimensional_views::read(reader, map),
        "moSMAndFPFeatIdPair" => super::manager_scalar_records::feature_ids(reader),
        "moCustomDefinedMaterialProperty_c" => super::manager_scalar_records::material(reader),
        "moToolboxPartSpecArray_c" => super::configuration_toolbox::array(reader, map),
        "moToolboxPartSpec_c" => super::configuration_toolbox::item(reader),
        "moConfigurationMgr_c" => super::configuration_manager::body(reader, map),
        "moPartConfiguration_c" => super::configuration_base::read(reader, map),
        "moLinkedAtomIdNode_c" => super::configuration_atoms::linked_node(reader, map),
        "swCubicalScene_c" | "swSphericalScene_c" => {
            super::configuration_scenes::scene(reader, map)
        }
        "pwSceneData_c" => super::configuration_scenes::pw_data(reader),
        "moNativeConfigurationTableFeature_c" => {
            super::configuration_tables::native_table(reader, map)
        }
        "moDesignTableFeature_c" => reader.scope("model_feature", |reader| {
            super::body_features::model_feature(reader, map)
        }),
        _ => return None,
    };
    Some(result)
}
