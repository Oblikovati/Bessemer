use super::ModelDecoder;
use crate::Result;
use crate::archive_codec::ObjectReader;

pub(super) fn read(
    reader: &mut ModelDecoder<'_>,
    map: &mut ObjectReader,
    name: &str,
) -> Option<Result<()>> {
    let result = match name {
        // #54 / C-205: modern sweep cuts inherit this complete layout without a suffix.
        "moSweep_c" | "moSweepCut_c" => super::sweep_records::read(reader, map),
        "moWireContact_c" => super::sweep_records::contact(reader, map),
        "moCompProfile_c" => super::component_references::feature(reader, map),
        "moEndPointRef_w" => super::sweep_reference_records::endpoint(reader, map),
        "moLoftSynchRef_c" => super::sweep_reference_records::synchronization(reader, map),
        _ => return None,
    };
    Some(result)
}
