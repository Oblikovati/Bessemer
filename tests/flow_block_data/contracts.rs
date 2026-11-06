use super::*;
use bessemer::FailureClass;
use std::io::Write;

fn selected_case(version: i32) -> fixture::BlockCase {
    fixture::cases()
        .into_iter()
        .find(|case| case.name == format!("populated-v{version}") && !case.compressed)
        .unwrap()
}

#[test]
fn all_public_block_types_are_send_and_sync() {
    assert_traits::<FlowLeafCount>();
    assert_traits::<FlowBlockDataOptions>();
    assert_traits::<FlowBlockCountPrefix>();
    assert_traits::<FlowBlockDataCoverage>();
    assert_traits::<FlowBlockDataCheckMode>();
    assert_traits::<FlowBlockDataDiagnostic>();
    assert_traits::<FlowLegacyBlockData>();
    assert_traits::<FlowLegacyBlock<'static>>();
    assert_traits::<FlowLegacyBlockEntry<'static>>();
    assert_traits::<FlowBlockWordArray<'static>>();
    assert_traits::<FlowBlockFieldValue>();
    assert_traits::<FlowBlockHeaderField>();
    assert_traits::<FlowBlockEntryField>();
    assert_traits::<FlowBlockGlobalCount>();
    assert_traits::<FlowBlockFieldEdit>();
    assert_traits::<FlowBlockDataEdit>();
    assert_traits::<FlowNativeBlockByteOperation>();
    assert_traits::<FlowDerivedLeafCount>();
    assert_traits::<FlowFreshContextAssumption>();
    assert_traits::<FlowBlockDataCompression>();
}

#[test]
fn type_width_absence_and_structural_requests_cannot_mutate_source() {
    let case = selected_case(14);
    let body = parse_case(&case);
    for request in rejected_requests() {
        assert!(body.replace_field(request).is_err());
    }
    let request = FlowBlockFieldEdit::Entry {
        index: 0,
        field: FlowBlockEntryField::Flag4,
        value: FlowBlockFieldValue::Byte(1),
    };
    assert_eq!(
        body.replace_field(request).unwrap_err().class,
        FailureClass::Missing
    );
    assert_eq!(body.decoded_bytes(), case.decoded);
    assert_eq!(
        body.source().as_bytes(),
        carrier(&case.stored, 500_000).as_bytes()
    );
}

fn rejected_requests() -> Vec<FlowBlockFieldEdit> {
    use FlowBlockFieldValue as V;
    let mut result = rejected_prefixes();
    result.extend(rejected_indices());
    result.extend([
        entry_edit(FlowBlockEntryField::State, V::Word32(0)),
        entry_edit(FlowBlockEntryField::Descriptor { index: 6 }, V::Word16(2)),
        entry_edit(FlowBlockEntryField::SecondCount, V::Signed32(2)),
        FlowBlockFieldEdit::Block {
            index: 0,
            field: FlowBlockHeaderField::Count12,
            value: V::Word16(1),
        },
    ]);
    result
}

#[test]
fn narrow_versioned_fields_and_member20_alias_keep_their_wire_domains() {
    let body = parse_case(&selected_case(1));
    check_member20_alias(&body);
    for (field, value) in [
        (
            FlowBlockEntryField::Descriptor { index: 0 },
            FlowBlockFieldValue::Word16(256),
        ),
        (
            FlowBlockEntryField::SecondCount,
            FlowBlockFieldValue::Signed32(-129),
        ),
    ] {
        assert!(body.replace_field(entry_edit(field, value)).is_err());
    }
    let body = parse_case(&selected_case(2));
    let request = entry_edit(
        FlowBlockEntryField::SecondCount,
        FlowBlockFieldValue::Signed32(-1),
    );
    assert!(body.replace_field(request).is_err());
}

#[test]
fn input_mutation_and_eight_workers_keep_isolated_results_and_reversals() {
    let mut case = selected_case(17);
    let source = parse_case(&case);
    case.stored.fill(0);
    std::thread::scope(|scope| {
        let workers: Vec<_> = (0..8)
            .map(|worker| {
                let original = &source;
                scope.spawn(move || check_worker(original, worker))
            })
            .collect();
        for worker in workers {
            worker.join().unwrap();
        }
    });
    assert_eq!(source.decoded_bytes(), case.decoded);
}

fn check_worker(source: &FlowLegacyBlockData, worker: u8) {
    let field = FlowBlockEntryField::State;
    let edit = source
        .replace_field(entry_edit(field, FlowBlockFieldValue::Byte(worker)))
        .unwrap();
    assert_eq!(
        edit.result().entry(0).unwrap().field(field).unwrap(),
        FlowBlockFieldValue::Byte(worker)
    );
    let original = source.entry(0).unwrap().field(field).unwrap();
    let reverse = edit
        .result()
        .replace_field(entry_edit(field, original))
        .unwrap();
    assert_eq!(
        reverse.result().source().as_bytes(),
        source.source().as_bytes()
    );
}

#[test]
fn measured_zlib_growth_obeys_original_payload_limit_without_source_mutation() {
    let mut raw = vec![0; 12 + 1024 * 5];
    raw[..4].copy_from_slice(&14_i32.to_le_bytes());
    let mut encoder = flate2::write::ZlibEncoder::new(Vec::new(), flate2::Compression::best());
    encoder.write_all(&raw).unwrap();
    let compressed = encoder.finish().unwrap();
    let mut configured = options(&selected_case(14));
    configured.compression = FlowBlockDataCompression::Zlib;
    configured.leaf_count = FlowLeafCount::try_from(1024).unwrap();
    check_growth(&compressed, &raw, configured);
}

fn check_growth(stored: &[u8], raw: &[u8], options: FlowBlockDataOptions) {
    let request = FlowBlockFieldEdit::Block {
        index: 0,
        field: FlowBlockHeaderField::BlockByte,
        value: FlowBlockFieldValue::Byte(7),
    };
    let source = carrier(stored, 500_000);
    let body = source
        .record_406_blocks(&source.latest(406).unwrap().unwrap().id, options)
        .unwrap();
    let grown = body.replace_field(request).unwrap();
    assert!(grown.result().coverage().stored_bytes > stored.len());
    let limited = carrier(stored, stored.len());
    let body = limited
        .record_406_blocks(&limited.latest(406).unwrap().unwrap().id, options)
        .unwrap();
    assert!(body.replace_field(request).is_err());
    assert_eq!(body.decoded_bytes(), raw);
    assert_eq!(body.source().as_bytes(), limited.as_bytes());
}

fn assert_traits<T: Send + Sync>() {}

fn entry_edit(field: FlowBlockEntryField, value: FlowBlockFieldValue) -> FlowBlockFieldEdit {
    FlowBlockFieldEdit::Entry {
        index: 0,
        field,
        value,
    }
}

fn rejected_prefixes() -> Vec<FlowBlockFieldEdit> {
    vec![
        FlowBlockFieldEdit::GlobalCount {
            field: FlowBlockGlobalCount::Fluid,
            count: -1,
        },
        FlowBlockFieldEdit::GlobalCount {
            field: FlowBlockGlobalCount::Fluid,
            count: 8,
        },
        FlowBlockFieldEdit::BitmapByte {
            index: usize::MAX,
            byte: 0,
        },
    ]
}

fn rejected_indices() -> Vec<FlowBlockFieldEdit> {
    vec![
        FlowBlockFieldEdit::Block {
            index: usize::MAX,
            field: FlowBlockHeaderField::BlockByte,
            value: FlowBlockFieldValue::Byte(0),
        },
        FlowBlockFieldEdit::Entry {
            index: usize::MAX,
            field: FlowBlockEntryField::State,
            value: FlowBlockFieldValue::Byte(0),
        },
    ]
}

fn check_member20_alias(body: &FlowLegacyBlockData) {
    let edit = entry_edit(
        FlowBlockEntryField::Member20,
        FlowBlockFieldValue::Signed32(17),
    );
    let alias = body.replace_field(edit).unwrap();
    for field in [
        FlowBlockEntryField::LegacyIndex,
        FlowBlockEntryField::Member20,
    ] {
        assert_eq!(
            alias.result().entry(0).unwrap().field(field).unwrap(),
            FlowBlockFieldValue::Signed32(17)
        );
    }
}
