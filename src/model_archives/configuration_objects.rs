use super::ModelDecoder;
use crate::Result;
use crate::archive_codec::ObjectReader;

pub(super) fn read(
    reader: &mut ModelDecoder<'_>,
    map: &mut ObjectReader,
    name: &str,
) -> Option<Result<()>> {
    let result = match name {
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
