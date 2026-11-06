use super::*;

#[test]
fn eight_workers_match_complete_serial_edits_after_caller_storage_mutation() {
    let mut case = cases()
        .into_iter()
        .find(|c| c.name == "v8-rich" && !c.compressed)
        .unwrap();
    let source = extras(&case);
    let id = source
        .fields()
        .find(|f| f.kind() == FlowBlockExtraFieldKind::LastByte)
        .unwrap()
        .id();
    case.stored.fill(0);
    let serial = serial_edits(&source, &id);
    std::thread::scope(|scope| {
        for (value, expected) in serial.iter().enumerate() {
            let source = &source;
            let id = &id;
            let original = &case.decoded;
            scope.spawn(move || check_worker(source, id, value as u8, expected, original));
        }
    });
}

fn check_worker(
    source: &FlowBlockExtras,
    id: &FlowBlockExtraFieldId,
    value: u8,
    expected: &[u8],
    original: &[u8],
) {
    assert!(
        source
            .replace_field(id, 0, FlowBlockExtraValue::Signed32(0))
            .is_err()
    );
    let edit = source
        .replace_field(id, 0, FlowBlockExtraValue::Byte(value))
        .unwrap();
    assert_eq!(edit.result().source().as_bytes(), expected);
    let result_id = edit.result().field_at(id.ordinal()).unwrap().id();
    let reverse = edit
        .result()
        .replace_field(&result_id, 0, FlowBlockExtraValue::Byte(255))
        .unwrap();
    assert_eq!(
        reverse.result().source().as_bytes(),
        source.source().as_bytes()
    );
    assert_eq!(source.decoded_bytes(), original);
}

fn serial_edits(source: &FlowBlockExtras, id: &FlowBlockExtraFieldId) -> Vec<Vec<u8>> {
    (0..8)
        .map(|value| {
            source
                .replace_field(id, 0, FlowBlockExtraValue::Byte(value))
                .unwrap()
                .result()
                .source()
                .as_bytes()
                .to_vec()
        })
        .collect()
}
