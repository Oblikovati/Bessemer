use super::*;
use std::collections::BTreeSet;

fn replace_byte_frame(body: &SimulationPostDialog, ordinal: usize, count: i32) -> Vec<u8> {
    let original = body.to_bytes();
    let (start, end) = body.field_offsets()[ordinal];
    let mut replacement = count.to_le_bytes().to_vec();
    replacement.resize(4 + count.max(0) as usize, 0xff);
    [&original[..start], replacement.as_slice(), &original[end..]].concat()
}

fn assert_byte_cap(body: &SimulationPostDialog, ordinal: usize) {
    let path = body.fields()[ordinal].path.as_ref();
    let cap = byte_cap(path);
    for count in [-1, cap, cap + 1] {
        let raw = replace_byte_frame(body, ordinal, count);
        let result = SimulationPostDialog::from_bytes(&raw, body.version(), LIMITS);
        assert_eq!(result.is_ok(), count == cap, "{path} {count}");
        if let Ok(parsed) = result {
            assert_eq!(parsed.to_bytes(), raw);
            let span = parsed.field_offsets()[ordinal];
            assert_eq!(span.1 - span.0, 4 + cap as usize);
        }
    }
}

fn check_byte_paths() -> BTreeSet<std::sync::Arc<str>> {
    let mut paths = BTreeSet::new();
    for body in owned_dialogs() {
        for (ordinal, field) in body.fields().iter().enumerate() {
            if !matches!(field.value, SimulationPostDialogValue::CountedBytes(_)) {
                continue;
            }
            if paths.insert(field.path.clone()) {
                assert_byte_cap(&body, ordinal);
            }
        }
    }
    paths
}

#[test]
fn opaque_reference_and_legacy_list_payloads_keep_distinct_imported_caps() {
    let paths = check_byte_paths();
    assert!(!paths.is_empty());
    assert!(
        paths
            .iter()
            .any(|path| path.starts_with("dialog.string_list["))
    );
    assert!(paths.iter().any(|path| path.ends_with(".blob")));
}

fn byte_cap(path: &str) -> i32 {
    if path.starts_with("dialog.string_list[") {
        return 0x100000;
    }
    1000000
}
