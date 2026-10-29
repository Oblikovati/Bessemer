use super::{ModelDecoder, ModelFieldValue};
use crate::archive_codec::{MaterialRead, StoredDouble};
use crate::{BessemerError, FailureClass, Result};

/// Preserve the direct forty-byte tolerance helper in native field order.
/// Example: `move_copy_deformation::mesh(reader)?` for an owned helper.
pub(super) fn mesh(reader: &mut ModelDecoder<'_>) -> Result<()> {
    reader.require_ordinary_part()?;
    reader.i32("raw_08")?;
    reader.f64s(&["raw_10", "raw_18", "raw_20"])?;
    reader.i32("raw_0c")?;
    reader.f64("raw_28")?;
    Ok(())
}

/// Retain the proved zero-multivector branches and reject undocumented payloads.
/// Example: `move_copy_deformation::conformal(reader)?` after version `0x2F69`.
pub(super) fn conformal(reader: &mut ModelDecoder<'_>) -> Result<()> {
    reader.scope("multivector_00", zero_multivector)?;
    let flags = reader.u32("flags_1b8")?;
    if flags & 0x100 != 0 {
        return unsupported(
            reader,
            "flags_1b8",
            format!(
                "flags {flags:#010x}; expected bit 0x100 clear for the documented conformal branch"
            ),
        );
    }
    if flags & 0x200 != 0 {
        reader.scope("multivector_30", zero_multivector)?;
        reader.scope("multivector_60", zero_multivector)?;
    }
    Ok(())
}

fn zero_multivector(reader: &mut ModelDecoder<'_>) -> Result<()> {
    let bits = reader.scalar(|cursor| cursor.u64())?;
    reader.field("scale", ModelFieldValue::F64(StoredDouble::from_bits(bits)))?;
    let scale = f64::from_bits(bits);
    if scale == 0.0 {
        return Ok(());
    }
    unsupported(
        reader,
        "scale",
        format!(
            "multivector scale {scale:?} (bits {:#018x}); expected positive or negative zero",
            scale.to_bits()
        ),
    )
}

fn unsupported(reader: &mut ModelDecoder<'_>, entity: &str, cause: String) -> Result<()> {
    reader.checked(Err(BessemerError::new(
        "model archive",
        entity,
        FailureClass::Unsupported,
        cause,
    )))
}
