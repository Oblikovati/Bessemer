use super::*;

const INPUT: &[u8] = include_bytes!("../fixtures/simulation-mesh-property/input.bin");

fn check_record_extents(property: &SimulationMeshProperty, cursor: &mut &[u8]) {
    let count = read_word(cursor) as usize;
    assert_eq!(property.record_spans().len(), count);
    for record in property.record_spans() {
        assert_eq!(record.index, read_word(cursor) as usize);
        assert_eq!(record.start, read_word(cursor) as usize);
        assert_eq!(record.end, read_word(cursor) as usize);
    }
    assert_eq!(property.coverage().record_count, count);
    assert_eq!(property.coverage().body_count, 1);
    assert_eq!(property.coverage().checked_bytes, property.to_bytes().len());
    assert_eq!(property.coverage().checked_fields, property.fields().len());
}

fn check_input(cursor: &mut &[u8]) -> usize {
    assert_eq!(read_word(cursor), 0);
    let version = SimulationPersistVersion::new(read_word(cursor) as i32).unwrap();
    assert_eq!(read_word(cursor), 1);
    let size = read_word(cursor) as usize;
    let property =
        SimulationMeshProperty::from_bytes(take_projection(cursor, size), version, LIMITS).unwrap();
    for _ in 0..read_word(cursor) {
        read_word(cursor);
        read_word(cursor);
    }
    check_record_extents(&property, cursor);
    property.coverage().nested_record_count
}

#[test]
fn all_direct_record_boundaries_and_nested_counts_match_independent_literal_extents() {
    let mut cursor = INPUT;
    let (mut contexts, mut nested) = (0, 0);
    while !cursor.is_empty() {
        nested += check_input(&mut cursor);
        contexts += 1;
    }
    assert_eq!((contexts, nested), (588, 716));
}
