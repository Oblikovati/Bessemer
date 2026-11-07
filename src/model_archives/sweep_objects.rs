use super::ModelDecoder;
use crate::Result;
use crate::archive_codec::{ArchiveClass, ObjectReader};

pub(super) fn read(
    reader: &mut ModelDecoder<'_>,
    map: &mut ObjectReader,
    class: &ArchiveClass,
) -> Option<Result<()>> {
    let result = match class.name.as_str() {
        // #54 / C-205: modern sweep cuts inherit this complete layout without a suffix.
        "moSweep_c" | "moSweepCut_c" => super::sweep_records::read(reader, map),
        "moTwistDataForBiDirectionalSweep_c" => super::sweep_data::twist(reader, class.schema),
        "moWireContact_c" => super::sweep_records::contact(reader, map),
        "moCompProfile_c" => super::component_references::feature(reader, map),
        "moEndPointRef_w" | "moCurvePointRef_w" => {
            super::sweep_reference_records::endpoint(reader, map)
        }
        "moLoftSynchRef_c" => super::sweep_reference_records::synchronization(reader, map),
        _ => return None,
    };
    Some(result)
}
