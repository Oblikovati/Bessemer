use super::kernel_frames::KernelFrame;
use crate::parasolid::{
    FieldDefinition, FieldKind, FieldValues, NodeRecord, NodeReference, StoredDouble, TransmitBody,
};
use std::collections::BTreeMap;

pub(super) fn rows(frame: &KernelFrame) -> BTreeMap<String, String> {
    let TransmitBody::Partition(list) = &frame.contents().body else {
        panic!("expected partition")
    };
    let mut rows = BTreeMap::new();
    for (index, record) in list.records.iter().enumerate() {
        record_rows(frame, index, record, &mut rows);
    }
    rows
}

fn record_rows(
    frame: &KernelFrame,
    index: usize,
    record: &NodeRecord,
    rows: &mut BTreeMap<String, String>,
) {
    let prefix = format!("records[{index}]");
    rows.insert(format!("{prefix}.type"), format!("i:{}", record.node_type));
    rows.insert(
        format!("{prefix}.index"),
        format!("i:{}", stored(record.index)),
    );
    let definitions = frame.layout_fields(record.node_type).unwrap();
    for (ordinal, (definition, values)) in definitions.iter().zip(&record.fields).enumerate() {
        field_rows(
            &format!("{prefix}.fields[{ordinal}]"),
            definition,
            values,
            rows,
        );
    }
}

fn field_rows(
    prefix: &str,
    definition: &FieldDefinition,
    values: &FieldValues,
    rows: &mut BTreeMap<String, String>,
) {
    let name: String = definition
        .name
        .chars()
        .map(|character| format!("{:02x}", character as u32))
        .collect();
    rows.insert(format!("{prefix}.name"), format!("t:{name}"));
    rows.insert(
        format!("{prefix}.kind"),
        format!("t:{}", kind(definition.kind)),
    );
    for (index, value) in field_values(values).into_iter().enumerate() {
        rows.insert(format!("{prefix}.values[{index}]"), value);
    }
}

fn stored(reference: NodeReference) -> u32 {
    match reference {
        NodeReference::Null => 1,
        NodeReference::Node(identity) => identity.value() + 1,
    }
}

fn field_values(values: &FieldValues) -> Vec<String> {
    match values {
        FieldValues::Integers(values) => integers(values),
        FieldValues::Pointers(values) => values
            .iter()
            .map(|value| format!("i:{}", stored(value.reference)))
            .collect(),
        FieldValues::Bytes(values) => integers(values),
        FieldValues::Shorts(values) => integers(values),
        FieldValues::WideUnits(values) => integers(values),
        FieldValues::Reals(values) => values.iter().map(|value| real(*value)).collect(),
        FieldValues::Intervals(values) => real_arrays(values),
        FieldValues::Vectors(values) | FieldValues::PartitionHVectors(values) => {
            real_arrays(values)
        }
        FieldValues::DeltaHVectors(values) => real_arrays(values),
        FieldValues::Boxes(values) => real_arrays(values),
    }
}

fn integers<T: std::fmt::Display>(values: &[T]) -> Vec<String> {
    values.iter().map(|value| format!("i:{value}")).collect()
}

fn real(value: StoredDouble) -> String {
    format!("f:{:016x}", value.bits())
}

fn real_arrays<const N: usize>(values: &[[StoredDouble; N]]) -> Vec<String> {
    values
        .iter()
        .flat_map(|row| row.iter().map(|value| real(*value)))
        .collect()
}

fn kind(kind: FieldKind) -> char {
    match kind {
        FieldKind::Integer => 'd',
        FieldKind::Pointer => 'p',
        FieldKind::UnsignedByte => 'u',
        FieldKind::Character => 'c',
        FieldKind::Logical => 'l',
        FieldKind::Short => 'n',
        FieldKind::WideUnit => 'w',
        FieldKind::Real => 'f',
        FieldKind::Interval => 'i',
        FieldKind::Vector => 'v',
        FieldKind::HVector => 'h',
        FieldKind::Box => 'b',
        FieldKind::SchemaOnlyQ => 'q',
        FieldKind::SchemaOnlyT => 't',
    }
}
