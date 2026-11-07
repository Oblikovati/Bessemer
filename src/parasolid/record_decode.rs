use super::wire::TransmitCursor;
use super::{FieldDefinition, FieldExtent, FieldValues, NodeRecord};
use crate::Result;

pub(super) fn read(
    cursor: &mut TransmitCursor<'_>,
    code: u16,
    definitions: &[FieldDefinition],
    delta: bool,
    maximum: usize,
) -> Result<NodeRecord> {
    let variable = definitions
        .iter()
        .any(|field| field.extent == FieldExtent::Variable);
    let length = if variable { Some(cursor.u32()?) } else { None };
    let index = cursor.index()?;
    let fields = read_fields(cursor, definitions, length, delta, maximum)?;
    Ok(NodeRecord {
        node_type: code,
        index,
        length,
        fields,
    })
}

fn read_fields(
    cursor: &mut TransmitCursor<'_>,
    definitions: &[FieldDefinition],
    length: Option<u32>,
    delta: bool,
    maximum: usize,
) -> Result<Vec<FieldValues>> {
    definitions
        .iter()
        .map(|field| super::field_decode::read(cursor, field, length, delta, maximum))
        .collect()
}
