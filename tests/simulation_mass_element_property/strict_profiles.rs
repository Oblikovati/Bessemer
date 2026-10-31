use super::*;

fn empty_strict_table() -> SimulationMassElementProperty {
    let original = owned_property(890011, None);
    let mut fields = original.fields().to_vec();
    fields.retain(|field| {
        !field.path.starts_with("record0.table0.") && !field.path.starts_with("record0.table1.")
    });
    for field in &mut fields {
        if matches!(
            field.path.as_ref(),
            "record0.table.mode" | "record0.table.count"
        ) {
            field.value = SimulationMassElementValue::Signed(0);
        }
    }
    SimulationMassElementProperty::from_fields(fields, original.version(), LIMITS).unwrap()
}

#[test]
fn existing_strict_profile_rejects_unused_out_of_range_modes_on_an_otherwise_complete_empty_table()
{
    let empty = empty_strict_table();
    let index = empty
        .fields()
        .iter()
        .position(|field| field.path.as_ref() == "record0.table.mode")
        .unwrap();
    let offset = empty.field_offsets()[index].0;
    for mode in [i32::MIN, -101, 101, i32::MAX] {
        assert_strict_mode(&empty, offset, mode);
    }
    assert_eq!(
        empty.fields()[index].value,
        SimulationMassElementValue::Signed(0)
    );
}

fn assert_strict_mode(empty: &SimulationMassElementProperty, offset: usize, mode: i32) {
    let mut raw = empty.to_bytes();
    raw[offset..offset + 4].copy_from_slice(&mode.to_le_bytes());
    let error =
        SimulationMassElementProperty::from_bytes(&raw, empty.version(), LIMITS).unwrap_err();
    assert_eq!(error.class, bessemer::FailureClass::Invalid);
    assert_eq!(error.entity, "record0");
    assert_eq!(
        error.cause,
        format!("table mode {mode}; expected -100..=100")
    );
}
