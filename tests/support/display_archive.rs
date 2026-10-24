use bessemer::display_lists::*;
pub(crate) const LIMITS: DisplayArchiveLimits = DisplayArchiveLimits {
    maximum_bytes: 64 * 1024 * 1024,
    maximum_fields: 100000,
    maximum_objects: 10000,
    maximum_depth: 128,
    maximum_entries: 100000,
};

pub(crate) fn context() -> DisplayArchiveContext {
    DisplayArchiveContext {
        version: DisplayVersion::new(19000),
        kind: DisplayDocumentKind::Part,
        file_version: 19000,
        face_tessellation_components: Default::default(),
    }
}

pub(crate) fn set_integer(contents: &mut DisplayArchiveContents, path: &str, value: i32) {
    let field = contents
        .fields
        .iter_mut()
        .find(|field| field.path.as_ref() == path)
        .unwrap();
    assert!(matches!(field.value, DisplayFieldValue::I32(_)));
    field.value = DisplayFieldValue::I32(value);
}

fn scalar_oracle_value(value: &DisplayFieldValue) -> String {
    match value {
        DisplayFieldValue::U8(value) => format!("i:{value}"),
        DisplayFieldValue::U16(value) => format!("i:{value}"),
        DisplayFieldValue::I16(value) => format!("i:{value}"),
        DisplayFieldValue::I32(value) => format!("i:{value}"),
        DisplayFieldValue::U32(value) => format!("i:{value}"),
        DisplayFieldValue::F32(value) => format!("f:{:016x}", f64::from(value.value()).to_bits()),
        DisplayFieldValue::F64(value) => format!("f:{:016x}", value.bits()),
        DisplayFieldValue::Text(value) => format!(
            "s:{}",
            value
                .text()
                .as_bytes()
                .iter()
                .map(|byte| format!("{byte:02x}"))
                .collect::<String>()
        ),
        other => panic!("{other:?}; expected scalar oracle field"),
    }
}

pub(crate) fn decode_document(bytes: &[u8], kind: DisplayDocumentKind) -> DisplayArchive {
    let document = bessemer::Document::from_bytes(bytes).unwrap();
    let bytes = document.read_stream("Contents/DisplayLists").unwrap();
    let context = DisplayArchiveContext { kind, ..context() };
    DisplayArchive::from_bytes(&bytes, context, LIMITS).unwrap()
}

pub(crate) fn check_scalar_table(archive: &DisplayArchive, expected: &str) -> usize {
    for line in expected.lines() {
        let (path, expected) = line.split_once('=').unwrap();
        let field = archive
            .contents()
            .fields
            .iter()
            .find(|field| field.path.as_ref() == path)
            .unwrap_or_else(|| panic!("missing {path}"));
        assert_eq!(scalar_oracle_value(&field.value), expected, "{path}");
    }
    expected.lines().count()
}
