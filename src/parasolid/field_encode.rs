use super::wire::TransmitWriter;
use super::{FieldValues, StoredDouble, TransmitPointer, invalid};
use crate::Result;

pub(super) fn write(writer: &mut TransmitWriter, values: &FieldValues) -> Result<()> {
    match values {
        FieldValues::Integers(values) => {
            write_elements(writer, values, |writer, value| writer.u32(*value as u32))
        }
        FieldValues::Pointers(values) => write_elements(writer, values, write_pointer),
        FieldValues::Bytes(values) => writer.output.extend_from_slice(values),
        FieldValues::Shorts(values) => {
            write_elements(writer, values, |writer, value| writer.u16(*value as u16))
        }
        FieldValues::WideUnits(values) => {
            write_elements(writer, values, |writer, value| writer.u16(*value))
        }
        FieldValues::Reals(values) => {
            write_elements(writer, values, |writer, value| writer.u64(value.bits()))
        }
        _ => return write_group_values(writer, values),
    }
    Ok(())
}

fn write_pointer(writer: &mut TransmitWriter, pointer: &TransmitPointer) {
    writer.index(pointer.reference);
    if let Some(flag) = pointer.delta_flag {
        writer.u8(flag);
    }
}

fn write_elements<T>(
    writer: &mut TransmitWriter,
    values: &[T],
    encode: impl Fn(&mut TransmitWriter, &T),
) {
    for value in values {
        encode(writer, value);
    }
}

fn write_group_values(writer: &mut TransmitWriter, values: &FieldValues) -> Result<()> {
    match values {
        FieldValues::Intervals(values) => write_groups(writer, values),
        FieldValues::Vectors(values) => write_groups(writer, values),
        FieldValues::PartitionHVectors(values) => write_groups(writer, values),
        FieldValues::DeltaHVectors(values) => write_groups(writer, values),
        FieldValues::Boxes(values) => write_groups(writer, values),
        _ => {
            return Err(invalid(
                "scalar field values; expected grouped floating elements",
            ));
        }
    }
    Ok(())
}

fn write_groups<const N: usize>(writer: &mut TransmitWriter, groups: &[[StoredDouble; N]]) {
    for group in groups {
        for value in group {
            writer.u64(value.bits());
        }
    }
}
