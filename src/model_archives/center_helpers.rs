use super::raw_fields::{
    RawScalarKind::{Double, Octet, Signed, Text, Unsigned},
    groups, scalar,
};
use super::{ModelDecoder, detail_helpers};
use crate::Result;
use crate::archive_codec::MaterialRead;

pub(super) fn read(reader: &mut ModelDecoder<'_>) -> Result<()> {
    reader.require_ordinary_model()?;
    reader.scope("helpers[0]", center_mark)?;
    reader.scope("helpers[1]", datum_feature)?;
    reader.scope("helpers[2]", datum_target)?;
    reader.scope("helpers[3]", swift_options)
}

fn center_mark(reader: &mut ModelDecoder<'_>) -> Result<()> {
    groups(
        reader,
        &[
            (Octet, "08"),
            (Double, "10"),
            (Octet, "18"),
            (Double, "20"),
            (Octet, "28 30 34 38 3c 2c"),
        ],
    )?;
    reader.scope("annotation_80", detail_helpers::annotation)?;
    groups(
        reader,
        &[(Unsigned, "148"), (Octet, "40 44 48 4c"), (Double, "78")],
    )
}

fn datum_feature(reader: &mut ModelDecoder<'_>) -> Result<()> {
    groups(
        reader,
        &[
            (Octet, "08 0c 10"),
            (Signed, "20 24"),
            (Text, "18 28"),
            (Signed, "30 34 38"),
        ],
    )?;
    reader.scope("annotation_40", detail_helpers::annotation)
}

fn datum_target(reader: &mut ModelDecoder<'_>) -> Result<()> {
    groups(
        reader,
        &[
            (Text, "18 20 28 08 10"),
            (Signed, "30 40"),
            (Double, "50 58"),
            (Signed, "44"),
            (Octet, "34 3c 38 48 4c"),
            (Signed, "60"),
            (Double, "68"),
            (Signed, "70"),
            (Octet, "74 90 b0"),
        ],
    )?;
    for coordinate in 0..3 {
        reader.f64(&format!("raw_vector_78[{coordinate}]"))?;
    }
    reader.scope("annotation_c0", detail_helpers::annotation)
}

fn swift_options(reader: &mut ModelDecoder<'_>) -> Result<()> {
    let version = reader.version();
    let active = super::swift_layout::SWIFT_LAYOUT
        .iter()
        .filter(|(_, member)| swift_member(version, member));
    for (index, (kind, _)) in active.enumerate() {
        reader.scope(&format!("values[{index}]"), |reader| {
            scalar(reader, *kind, "value")
        })?;
    }
    Ok(())
}

fn swift_member(version: u32, member: &str) -> bool {
    match member {
        "c" => version > 0x3739,
        "3f8" => version >= 0x3aff,
        "418" => version > 0x46b5,
        _ => true,
    }
}
