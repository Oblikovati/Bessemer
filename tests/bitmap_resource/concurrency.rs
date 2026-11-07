use super::*;

fn resource_operation_sequence() -> Vec<Vec<u8>> {
    let mut outputs = Vec::new();
    for case in resource_literals() {
        let source = BitmapResourceArray::from_bytes(case.raw, MAXIMUM).unwrap();
        for length in [0, 1, 255, 65534, 65535, 65536] {
            outputs.push(checked_resource_operation(&source, &case, length));
        }
    }
    outputs
}

fn checked_resource_operation(
    source: &BitmapResourceArray,
    case: &ResourceLiteral,
    length: usize,
) -> Vec<u8> {
    let requested = vec![0xa7; length];
    let edit = source.edit_payload(&requested).unwrap();
    let prefix = if case.prefix == 6 || length >= 65535 {
        6
    } else {
        2
    };
    let expected = literal_array(&requested, prefix);
    verify_resource_coverage(&edit, &expected, prefix);
    verify_resource_history(&edit, case.raw, &expected, [case.prefix, prefix]);
    let rejected = edit.resource().edit_payload(&vec![0; MAXIMUM + 1]);
    assert_eq!(rejected.unwrap_err().class, FailureClass::Invalid);
    assert_eq!(edit.resource().to_bytes().unwrap(), expected);
    let reverse = edit.resource().edit_payload(source.payload()).unwrap();
    assert_eq!(reverse.resource().to_bytes().unwrap(), case.raw);
    assert_eq!(source.to_bytes().unwrap(), case.raw);
    expected
}

#[test]
fn parallel_resource_edits_match_serial_bytes_and_preserve_inputs_after_rejection() {
    let expected = resource_operation_sequence();
    std::thread::scope(|scope| {
        let workers: Vec<_> = (0..8)
            .map(|_| scope.spawn(resource_operation_sequence))
            .collect();
        for worker in workers {
            assert_eq!(worker.join().unwrap(), expected);
        }
    });
    assert_eq!(expected.len(), 120);
}

#[test]
#[ignore = "Requires an explicit BESSEMER_BITMAP_RESOURCE_EXPORT evidence output path"]
fn export_actual_resource_operations_for_independent_reader_acceptance() {
    let outputs = resource_operation_sequence();
    let mut packet = b"BRE1".to_vec();
    packet.extend_from_slice(&u32::try_from(outputs.len()).unwrap().to_le_bytes());
    for raw in outputs {
        packet.extend_from_slice(&u32::try_from(raw.len()).unwrap().to_le_bytes());
        packet.extend_from_slice(&raw);
    }
    let destination = std::env::var_os("BESSEMER_BITMAP_RESOURCE_EXPORT").unwrap();
    std::fs::write(destination, packet).unwrap();
}
