//! Paired IMS-value oracles retain explicit fresh definitions and every raw 18-word BL row.
#[path = "flow_result_values/fixture.rs"]
mod fixture;
#[path = "flow_field_values/packet.rs"]
mod packet;
#[path = "flow_result_values/rebuild.rs"]
mod rebuild;
#[path = "flow_state_tables/carrier.rs"]
mod state_carrier;
#[path = "flow_state_tables/streams.rs"]
mod streams;
use bessemer::flow::*;
use fixture::*;
use std::collections::BTreeSet;

#[test]
fn all_paired_oracles_rebuild_every_decoded_byte_and_check_exact_compressed_consumption() {
    for row in cases() {
        check_transports(&row);
        assert_eq!(row.frame.version, 2);
        assert_eq!(row.frame.end as usize, row.frame.decoded.len());
        assert_eq!(row.frame.qualifications, [true, true, false, false]);
        check_context(&row);
    }
}
fn check_transports(row: &ValueCase) {
    let definitions = streams::inflate_exact(&row.context.stored);
    let values = streams::inflate_exact(&row.frame.stored);
    let rebuilt = rebuild::reconstruct(row);
    assert_eq!(definitions, row.context.decoded, "{}", row.name);
    assert_eq!(values, row.frame.decoded, "{}", row.name);
    assert_eq!(rebuilt, row.frame.decoded, "{}", row.name);
}
fn check_context(row: &ValueCase) {
    let source = state_carrier::carrier(500, &row.context.stored);
    let id = source.records().nth(1).unwrap().id;
    let options = FlowResultDefinitionOptions {
        maximum_decoded_bytes: 2_000_000,
        maximum_objects: 100_000,
        maximum_values: 1_000_000,
    };
    let context = source.record_500_ims_definitions(&id, options).unwrap();
    state_carrier::check_history(&source.replace_payload(&id, &row.context.stored).unwrap());
    assert_eq!(context.decoded_bytes(), row.context.decoded);
    assert_eq!(context.stored_count(), row.context.identifiers.len());
    for (ordinal, identifier) in row.context.identifiers.iter().enumerate() {
        let object = context.definition(ordinal).unwrap();
        assert_eq!(object.identifier(), *identifier);
        assert_eq!(object.version(), FlowImsDefinitionVersion::V2);
        assert_eq!(object.allocation_byte(), Some(0));
    }
}
#[test]
fn native_pair_retains_all_74004_raw_patterns_and_3876_eighteen_word_rows() {
    let row = cases().into_iter().find(|row| row.native).unwrap();
    assert_eq!(row.frame.stored.len(), 337770);
    assert_eq!(row.frame.decoded.len(), 592076);
    assert_eq!(row.context.stored.len(), 6767);
    assert_eq!(row.context.identifiers, [0]);
    let object = &row.frame.objects[0];
    assert_eq!(
        (object.first_flag, object.bl_flag, object.last_flag),
        (0, 1, 1)
    );
    assert!(object.first.is_none());
    assert_eq!(object.main.count, 4056);
    assert_eq!(object.last.as_ref().unwrap().count, 180);
    let bl = object.bl.as_ref().unwrap();
    assert_eq!(bl.count, 3876);
    assert_eq!(bl.raw.len(), 3876 * 18 * 8);
    assert_eq!(bl.native_state, 0);
}
#[test]
fn every_flag_byte_raw_bit_complement_and_bl_column_is_owned() {
    let mut flags: [BTreeSet<u8>; 3] = std::array::from_fn(|_| BTreeSet::new());
    let mut patterns: [BTreeSet<u64>; 21] = std::array::from_fn(|_| BTreeSet::new());
    for row in cases() {
        for object in &row.frame.objects {
            flags[0].insert(object.first_flag);
            flags[1].insert(object.bl_flag);
            flags[2].insert(object.last_flag);
            collect_patterns(object, &mut patterns);
        }
    }
    assert_eq!(flags, std::array::from_fn(|_| (0..=255).collect()));
    patterns.iter().for_each(check_bits);
}
fn collect_patterns(object: &ImsValueObject, patterns: &mut [BTreeSet<u64>; 21]) {
    for (ordinal, array) in [
        object.first.as_ref(),
        Some(&object.main),
        object.last.as_ref(),
    ]
    .into_iter()
    .enumerate()
    {
        if let Some(array) = array {
            collect_words(&array.raw, &mut patterns[ordinal]);
        }
    }
    if let Some(bl) = &object.bl {
        let (rows, remainder) = bl.raw.as_chunks::<144>();
        assert!(remainder.is_empty());
        for row in rows {
            collect_bl_row(row, patterns);
        }
    }
}
fn collect_words(raw: &[u8], values: &mut BTreeSet<u64>) {
    let (words, remainder) = raw.as_chunks::<8>();
    assert!(remainder.is_empty());
    values.extend(words.iter().map(|word| u64::from_le_bytes(*word)));
}
fn collect_bl_row(raw: &[u8], patterns: &mut [BTreeSet<u64>; 21]) {
    assert_eq!(raw.len(), 144);
    for (column, word) in raw.as_chunks::<8>().0.iter().enumerate() {
        patterns[column + 3].insert(u64::from_le_bytes(*word));
    }
}
fn check_bits(values: &BTreeSet<u64>) {
    assert!(values.contains(&0) && values.contains(&u64::MAX));
    for bit in 0..64 {
        assert!(values.contains(&(1 << bit)));
        assert!(values.contains(&!(1 << bit)));
    }
    assert!(values.contains(&0x7ff0000000000000) && values.contains(&0xfff0000000000000));
    assert!(values.contains(&0x7ff8000000000042));
}
#[test]
fn empty_duplicate_and_multiple_context_objects_retain_physical_value_order() {
    let rows = cases();
    let counts: BTreeSet<_> = rows.iter().map(|row| row.frame.objects.len()).collect();
    assert!(BTreeSet::from([0, 1, 2, 3, 16, 64, 257]).is_subset(&counts));
    let row = rows.iter().find(|row| row.name == "objects-257").unwrap();
    for (ordinal, object) in row.frame.objects.iter().enumerate() {
        assert_eq!(object.identifier, if ordinal % 2 == 0 { 7 } else { -1 });
        assert_eq!(object.main.raw, (ordinal as u64).to_le_bytes());
    }
    let row = rows.iter().find(|row| row.name == "objects-0").unwrap();
    assert_eq!(row.frame.decoded, [2]);
}
