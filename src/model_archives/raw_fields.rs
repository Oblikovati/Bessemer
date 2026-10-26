use super::ModelDecoder;
use crate::Result;
use crate::archive_codec::MaterialRead;

#[derive(Clone, Copy)]
pub(super) enum RawScalarKind {
    Octet,
    Word,
    Signed,
    Unsigned,
    Single,
    Double,
    Text,
}

pub(super) fn groups(
    reader: &mut ModelDecoder<'_>,
    recipe: &[(RawScalarKind, &str)],
) -> Result<()> {
    for (kind, offsets) in recipe {
        fields(reader, *kind, offsets)?;
    }
    Ok(())
}

pub(super) fn fields(
    reader: &mut ModelDecoder<'_>,
    kind: RawScalarKind,
    offsets: &str,
) -> Result<()> {
    for offset in offsets.split_ascii_whitespace() {
        let label = format!("raw_{offset}");
        scalar(reader, kind, &label)?;
    }
    Ok(())
}

pub(super) fn scalar(
    reader: &mut ModelDecoder<'_>,
    kind: RawScalarKind,
    label: &str,
) -> Result<()> {
    match kind {
        RawScalarKind::Octet => reader.u8(label).map(|_| ()),
        RawScalarKind::Word => reader.u16(label).map(|_| ()),
        RawScalarKind::Signed => reader.i32(label).map(|_| ()),
        RawScalarKind::Unsigned => reader.u32(label).map(|_| ()),
        RawScalarKind::Single => reader.f32(label),
        RawScalarKind::Double => reader.f64(label),
        RawScalarKind::Text => reader.text(label),
    }
}
