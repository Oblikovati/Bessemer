use super::{ModelField, ModelFieldValue, encode_fields, manager_context_tests};

struct PrimitiveSpan {
    start: usize,
    end: usize,
    kind: &'static str,
    wire: Vec<u8>,
}

fn expected_primitives(oracle: &'static str) -> Vec<PrimitiveSpan> {
    oracle
        .lines()
        .map(|line| {
            let mut columns = line.split('\t');
            PrimitiveSpan {
                start: columns.next().unwrap().parse().unwrap(),
                end: columns.next().unwrap().parse().unwrap(),
                kind: columns.next().unwrap(),
                wire: hex_bytes(columns.next().unwrap()),
            }
        })
        .collect()
}

fn hex_bytes(value: &str) -> Vec<u8> {
    value
        .as_bytes()
        .as_chunks::<2>()
        .0
        .iter()
        .map(|pair| u8::from_str_radix(std::str::from_utf8(pair).unwrap(), 16).unwrap())
        .collect()
}

fn primitive_kind(value: &ModelFieldValue) -> Option<&'static str> {
    match value {
        ModelFieldValue::U8(_) => Some("B"),
        ModelFieldValue::U16(_) => Some("H"),
        ModelFieldValue::I16(_) => Some("h"),
        ModelFieldValue::U32(_) => Some("I"),
        ModelFieldValue::I32(_) => Some("i"),
        ModelFieldValue::U64(_) => Some("Q"),
        ModelFieldValue::F32(_) => Some("f"),
        ModelFieldValue::F64(_) => Some("d"),
        ModelFieldValue::Text(_)
        | ModelFieldValue::ObjectTag(_)
        | ModelFieldValue::Transform(_)
        | ModelFieldValue::Kernel(_) => None,
    }
}

fn compare_field(field: &ModelField, start: usize, spans: &[PrimitiveSpan]) -> usize {
    let wire = encode_fields(std::slice::from_ref(field), 30000).unwrap();
    let end = start + wire.len();
    if field.path.contains(".byte_plane_40[") {
        compare_plane_byte(field, start, end, &wire, spans);
        return end;
    }
    let fragments: Vec<_> = spans
        .iter()
        .filter(|span| span.start >= start && span.start < end)
        .collect();
    assert_eq!(fragments.first().unwrap().start, start, "{}", field.path);
    assert_eq!(fragments.last().unwrap().end, end, "{}", field.path);
    let expected: Vec<u8> = fragments
        .iter()
        .flat_map(|span| span.wire.iter().copied())
        .collect();
    assert_eq!(wire, expected, "{}", field.path);
    if let Some(kind) = primitive_kind(&field.value) {
        assert_eq!(fragments.len(), 1, "{}", field.path);
        assert_eq!(fragments[0].kind, kind, "{}", field.path);
    }
    end
}

fn compare_plane_byte(
    field: &ModelField,
    start: usize,
    end: usize,
    wire: &[u8],
    spans: &[PrimitiveSpan],
) {
    assert!(matches!(field.value, ModelFieldValue::U8(_)));
    assert_eq!(end, start + 1);
    let source = spans
        .iter()
        .find(|span| span.start <= start && end <= span.end)
        .unwrap();
    assert_eq!(source.kind, "raw");
    assert_eq!(
        wire,
        &source.wire[start - source.start..end - source.start],
        "{}",
        field.path
    );
}

#[test]
fn every_manager_field_matches_an_independent_reference_primitive_partition() {
    let bytes = include_bytes!("../fixtures/resolved-features/owned-manager.bin");
    assert_eq!(manager_context_tests::manager_bytes(), bytes);
    let parsed = manager_context_tests::decode(bytes).unwrap();
    assert_reference_fields(
        &parsed.fields,
        bytes,
        include_str!("../fixtures/resolved-features/owned-manager-primitives.tsv"),
        504,
    );
}

pub(super) fn assert_reference_fields(
    fields: &[ModelField],
    bytes: &[u8],
    oracle: &'static str,
    expected_reads: usize,
) {
    let spans = expected_primitives(oracle);
    assert_eq!(spans.len(), expected_reads);
    let mut end = 0;
    for span in &spans {
        assert_eq!(span.start, end);
        assert_eq!(&bytes[span.start..span.end], span.wire);
        end = span.end;
    }
    assert_eq!(end, bytes.len());
    let mut end = 0;
    for field in fields {
        end = compare_field(field, end, &spans);
    }
    assert_eq!(end, bytes.len());
}
