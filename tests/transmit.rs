//! Independent primitive vectors, complete native framing, and immutable typed edits.
#[path = "support/transmit.rs"]
mod transmit_support;
use bessemer::parasolid::*;
use transmit_support::{CASES, DELTAS, LIMITS, NEUTRAL, base, list, list_mut, parse};

#[test]
fn every_owned_vector_rebuilds_every_byte_without_the_original_buffer() {
    for bytes in CASES {
        let parsed = parse(bytes);
        assert_eq!(parsed.to_bytes().unwrap(), *bytes);
        let rebuilt = TransmitFile::new(&base(), parsed.contents().clone()).unwrap();
        assert_eq!(rebuilt.to_bytes().unwrap(), *bytes);
        assert_eq!(
            parse(&rebuilt.to_bytes().unwrap()).contents(),
            parsed.contents()
        );
    }
}

#[test]
fn both_byte_orders_match_independently_assembled_vectors() {
    for (neutral, bare) in [(CASES[0], CASES[2]), (CASES[1], CASES[3])] {
        let parsed = parse(neutral);
        let mut contents = parsed.contents().clone();
        contents.format = TransmitFormat::Bare;
        assert_eq!(
            parsed.with_contents(contents).unwrap().to_bytes().unwrap(),
            bare
        );
        let parsed = parse(bare);
        let mut contents = parsed.contents().clone();
        contents.format = TransmitFormat::Neutral;
        assert_eq!(
            parsed.with_contents(contents).unwrap().to_bytes().unwrap(),
            neutral
        );
    }
}

#[test]
fn every_primitive_and_layout_edit_matches_independent_expected_values() {
    let parsed = parse(NEUTRAL);
    let records = &list(parsed.contents()).records;
    assert_eq!(records.len(), 6);
    let fields = &records[2].fields;
    assert_eq!(parsed.layout_fields(200).unwrap().len(), 14);
    assert_eq!(parsed.layout_fields(200).unwrap()[2].name, "Café");
    assert_eq!(fields[0], FieldValues::Integers(vec![i32::MIN]));
    assert_eq!(
        fields[1],
        FieldValues::Pointers(vec![TransmitPointer {
            reference: NodeReference::Node(NodeId::new(32770).unwrap()),
            delta_flag: None
        }])
    );
    assert_eq!(fields[3], FieldValues::Bytes(vec![255]));
    assert_eq!(fields[4], FieldValues::Bytes(vec![255]));
    assert_eq!(fields[5], FieldValues::Bytes(vec![255]));
    assert_eq!(fields[6], FieldValues::Shorts(vec![i16::MIN]));
    assert_eq!(fields[7], FieldValues::WideUnits(vec![0xd800]));
    assert_eq!(
        fields[8],
        FieldValues::Reals(vec![StoredDouble::from_bits((-0.0_f64).to_bits())])
    );
    let FieldValues::PartitionHVectors(hvecs) = &fields[11] else {
        panic!("expected hvec");
    };
    assert_eq!(hvecs[0][1].bits(), 0x7ff8123456789abc);
    assert_eq!(fields[13], FieldValues::Bytes(vec![255, 254, 253]));
    let expected_kinds = [
        FieldKind::Integer,
        FieldKind::Pointer,
        FieldKind::UnsignedByte,
        FieldKind::UnsignedByte,
        FieldKind::Character,
        FieldKind::Logical,
        FieldKind::Short,
        FieldKind::WideUnit,
        FieldKind::Real,
        FieldKind::Interval,
        FieldKind::Vector,
        FieldKind::HVector,
        FieldKind::Box,
        FieldKind::UnsignedByte,
    ];
    assert_eq!(
        parsed
            .layout_fields(200)
            .unwrap()
            .iter()
            .map(|field| field.kind)
            .collect::<Vec<_>>(),
        expected_kinds
    );
    let patterns = [
        StoredDouble::from_bits(0x8000000000000000),
        StoredDouble::from_bits(0x7ff8123456789abc),
        StoredDouble::from_bits(0x7ff0000000000000),
        StoredDouble::from_bits(0xfff0000000000000),
        StoredDouble::from_bits(1),
        StoredDouble::from_bits(0x3ff0000000000000),
    ];
    assert_eq!(
        fields[9],
        FieldValues::Intervals(vec![[patterns[0], patterns[1]]])
    );
    assert_eq!(
        fields[10],
        FieldValues::Vectors(vec![[patterns[0], patterns[1], patterns[2]]])
    );
    assert_eq!(fields[12], FieldValues::Boxes(vec![patterns]));
    assert_eq!(
        records[3].fields[0],
        FieldValues::Reals(vec![patterns[0], patterns[1]])
    );
    let defined = parsed.layout_fields(231).unwrap();
    assert_eq!(
        defined.iter().map(|field| field.kind).collect::<Vec<_>>(),
        expected_kinds[..2]
            .iter()
            .chain(expected_kinds[3..].iter())
            .copied()
            .collect::<Vec<_>>()
    );
    assert_eq!(records[4].length, Some(0));
    assert_eq!(records[4].fields[13], FieldValues::Bytes(Vec::new()));
    assert_eq!(
        list(parsed.contents()).terminator,
        NodeReference::Node(NodeId::new(32770).unwrap())
    );
}

#[test]
fn deltas_preserve_plain_exceptions_flags_hvecs_and_the_untrusted_count() {
    let parsed = parse(DELTAS);
    let TransmitBody::Deltas(sections) = &parsed.contents().body else {
        panic!("expected deltas");
    };
    assert_eq!(sections.len(), 2);
    assert!(sections[1].changes.is_none());
    let first = &sections[0].records.records;
    let FieldValues::Pointers(plain) = &first[0].fields[0] else {
        panic!("expected pointer");
    };
    assert_eq!(plain[0].delta_flag, None);
    let FieldValues::DeltaHVectors(hvecs) = &first[2].fields[11] else {
        panic!("expected eleven doubles");
    };
    assert_eq!(hvecs[0].len(), 11);
    assert_eq!(hvecs[0][7].bits(), 0x7ff8123456789abc);
    let changes = sections[0].changes.as_ref().unwrap();
    assert_eq!(changes.count, u32::MAX);
    assert_eq!(changes.new_nodes[1].index.value(), 0x7ffffffe);
    assert_eq!(changes.modified.records.len(), 3);
    let FieldValues::Pointers(modified) = &changes.modified.records[0].fields[0] else {
        panic!("expected delta pointer");
    };
    assert_eq!(modified[0].delta_flag, Some(0));
    let FieldValues::Pointers(array) = &first[3].fields[1] else {
        panic!("expected pointer array");
    };
    assert_eq!(array[0].delta_flag, Some(0));
    assert_eq!(array[1].delta_flag, Some(255));
    assert_eq!(array[1].reference, NodeReference::Null);
}

#[test]
fn edits_reverts_and_caller_mutation_leave_published_inputs_unchanged() {
    let mut bytes = NEUTRAL.to_vec();
    let source = parse(&bytes);
    bytes.fill(0);
    let mut contents = source.contents().clone();
    list_mut(&mut contents).records[2].fields[0] = FieldValues::Integers(vec![123]);
    let edited = source.with_contents(contents.clone()).unwrap();
    list_mut(&mut contents).records.clear();
    assert_eq!(
        list(edited.contents()).records[2].fields[0],
        FieldValues::Integers(vec![123])
    );
    let reopened = parse(&edited.to_bytes().unwrap());
    assert_eq!(reopened.contents(), edited.contents());
    assert_eq!(source.to_bytes().unwrap(), NEUTRAL);
    assert_eq!(
        edited
            .with_contents(source.contents().clone())
            .unwrap()
            .to_bytes()
            .unwrap(),
        NEUTRAL
    );
}

#[test]
fn construction_empty_results_and_schema_ownership_are_explicit() {
    let source = parse(NEUTRAL);
    let mut contents = source.contents().clone();
    contents.layouts.clear();
    list_mut(&mut contents).records.clear();
    let schema = base();
    let result = TransmitFile::new(&schema, contents).unwrap();
    drop(schema);
    let encoded = result.to_bytes().unwrap();
    assert!(list(parse(&encoded).contents()).records.is_empty());
    let limits = TransmitLimits {
        maximum_records: 0,
        maximum_field_elements: 0,
    };
    assert!(TransmitFile::from_bytes(&encoded, &base(), limits).is_ok());
    assert!(result.layout_fields(200).is_err());
}

#[test]
fn independent_ids_and_keys_normalize_once_and_reject_invalid_shapes() {
    let key = SchemaKey::parse("SCH_003701250_037102_013006").unwrap();
    assert_eq!(
        (key.modeller.value(), key.writer.value(), key.base.value()),
        (3701250, 37102, 13006)
    );
    assert_eq!(SchemaNumber::new(13006).value(), 13006);
    for key in [
        "SCH_1_2",
        "SCH_1_2_3_extra",
        "sch_1_2_3",
        "SCH_-1_2_3",
        "SCH_+1_2_3",
        "SCH_1__3",
        "SCH_4294967296_2_3",
    ] {
        assert!(SchemaKey::parse(key).is_err(), "{key}");
    }
    for id in [0, 0x7fffffff, u32::MAX] {
        assert!(NodeId::new(id).is_err());
    }
    assert_eq!(NodeId::new(0x7ffffffe).unwrap().value(), 0x7ffffffe);
}

#[test]
fn all_public_transmit_types_are_send_sync_and_concurrent_edits_are_deterministic() {
    fn safe<T: Send + Sync>() {}
    safe::<TransmitFile>();
    safe::<BaseSchema>();
    safe::<BaseNodeType>();
    safe::<SchemaKey>();
    safe::<SchemaNumber>();
    safe::<NodeId>();
    safe::<NodeReference>();
    safe::<TransmitFormat>();
    safe::<TransmitLimits>();
    safe::<FieldKind>();
    safe::<FieldExtent>();
    safe::<FieldDefinition>();
    safe::<LayoutEdit>();
    safe::<LayoutDefinition>();
    safe::<TransmitPointer>();
    safe::<FieldValues>();
    safe::<NodeRecord>();
    safe::<RecordList>();
    safe::<NewNode>();
    safe::<DeltaChanges>();
    safe::<DeltaSection>();
    safe::<TransmitBody>();
    safe::<TransmitContents>();
    safe::<StoredDouble>();
    let source = parse(DELTAS);
    let threads: Vec<_> = (0..8)
        .map(|_| {
            let source = source.clone();
            std::thread::spawn(move || {
                for _ in 0..20 {
                    let mut contents = source.contents().clone();
                    list_mut(&mut contents).records[2].fields[0] = FieldValues::Integers(vec![123]);
                    let edited = source.with_contents(contents.clone()).unwrap();
                    let rebuilt = TransmitFile::new(&base(), contents).unwrap();
                    assert_eq!(edited.to_bytes().unwrap(), rebuilt.to_bytes().unwrap());
                    assert_eq!(source.to_bytes().unwrap(), DELTAS);
                }
            })
        })
        .collect();
    for thread in threads {
        thread.join().unwrap();
    }
    assert!(TransmitFile::from_bytes(DELTAS, &base(), LIMITS).is_ok());
}
