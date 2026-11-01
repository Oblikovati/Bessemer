use super::*;

fn collection_case(path: &str) -> (SimulationPostDialog, usize) {
    for body in owned_dialogs() {
        let ordinal = body
            .fields()
            .iter()
            .position(|field| field.path.as_ref() == path);
        if let Some(ordinal) = ordinal {
            return (body, ordinal);
        }
    }
    panic!("expected an independently frozen collection at {path}");
}

fn replace_collection(
    body: &SimulationPostDialog,
    ordinal: usize,
    count: i32,
    record_bytes: usize,
) -> Vec<u8> {
    let original = body.to_bytes();
    let (start, mut end) = body.field_offsets()[ordinal];
    let prefix = collection_prefix(body.fields()[ordinal].path.as_ref());
    for index in ordinal + 1..body.fields().len() {
        if !body.fields()[index].path.starts_with(prefix) {
            break;
        }
        end = body.field_offsets()[index].1;
    }
    let mut replacement = count.to_le_bytes().to_vec();
    replacement.resize(4 + count.max(0) as usize * record_bytes, 0);
    [&original[..start], replacement.as_slice(), &original[end..]].concat()
}

fn assert_collection_cap(path: &str, record_bytes: usize) {
    let (body, ordinal) = collection_case(path);
    for count in [-1, 10000, 10001] {
        let raw = replace_collection(&body, ordinal, count, record_bytes);
        let result = SimulationPostDialog::from_bytes(&raw, body.version(), LIMITS);
        assert_eq!(result.is_ok(), count == 10000, "{path} {count}");
        if let Ok(parsed) = result {
            assert_eq!(parsed.to_bytes(), raw);
            let rebuilt =
                SimulationPostDialog::from_fields(parsed.fields().to_vec(), body.version(), LIMITS)
                    .unwrap();
            assert_eq!(rebuilt.to_bytes(), raw);
        }
    }
}

#[test]
fn variable_collections_require_the_imported_count_cap_with_complete_records() {
    for (path, record_bytes) in [
        ("sensors.count", 8),
        ("sensor[0].points.count", 44),
        ("dialog.da4.count", 4),
        ("dialog.string_list_dd0.count", 4),
        ("equation.variables.count", 20),
    ] {
        assert_collection_cap(path, record_bytes);
    }
}

fn collection_prefix(path: &str) -> &str {
    match path {
        "sensors.count" => "sensor[",
        "sensor[0].points.count" => "sensor[0].point[",
        "dialog.da4.count" => "dialog.da8_integer_array[",
        "dialog.string_list_dd0.count" => "dialog.string_list[",
        "equation.variables.count" => "equation.variable[",
        _ => panic!("unexpected collection {path}; expected one of five tested paths"),
    }
}
