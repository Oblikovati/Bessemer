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
    if compare_raw_array_element(field, start, end, &wire, spans) {
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

fn compare_raw_array_element(
    field: &ModelField,
    start: usize,
    end: usize,
    wire: &[u8],
    spans: &[PrimitiveSpan],
) -> bool {
    let plane =
        field.path.contains(".byte_plane_40[") || field.path.contains(".payload.bytes_3b8[");
    let words = field.path.contains(".payload_words[");
    let transforms = field.path.contains(".payload.transforms_1148.words[");
    if !plane && !words && !transforms {
        return false;
    }
    let Some(source) = spans
        .iter()
        .find(|span| span.kind == "raw" && span.start <= start && end <= span.end)
    else {
        return false;
    };
    assert!(matches!(
        (&field.value, plane, transforms),
        (ModelFieldValue::U8(_), true, false)
            | (ModelFieldValue::U64(_), false, false)
            | (ModelFieldValue::U32(_), false, true)
    ));
    assert_eq!(
        end,
        start
            + if plane {
                1
            } else if transforms {
                4
            } else {
                8
            }
    );
    assert_eq!(
        wire,
        &source.wire[start - source.start..end - source.start],
        "{}",
        field.path
    );
    true
}

#[test]
fn packed_raw_array_words_keep_exact_little_endian_bytes_and_element_widths() {
    let spans = [PrimitiveSpan {
        start: 0,
        end: 16,
        kind: "raw",
        wire: vec![
            1, 2, 3, 4, 5, 6, 7, 8, 255, 254, 253, 252, 251, 250, 249, 248,
        ],
    }];
    for (index, word) in [0x0807060504030201_u64, 0xf8f9fafbfcfdfeff]
        .into_iter()
        .enumerate()
    {
        let field = ModelField {
            path: format!("root.array.payload_words[{index}]").into(),
            value: ModelFieldValue::U64(word),
        };
        assert_eq!(compare_field(&field, index * 8, &spans), (index + 1) * 8);
    }
}

#[test]
#[should_panic]
fn packed_raw_array_comparison_rejects_a_changed_word() {
    let spans = [PrimitiveSpan {
        start: 0,
        end: 8,
        kind: "raw",
        wire: vec![1, 2, 3, 4, 5, 6, 7, 8],
    }];
    let field = ModelField {
        path: "root.array.payload_words[0]".into(),
        value: ModelFieldValue::U64(0),
    };
    compare_field(&field, 0, &spans);
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

#[test]
fn packed_alignment_bytes_match_raw_spans_one_byte_at_a_time() {
    let spans = [PrimitiveSpan {
        start: 0,
        end: 3,
        kind: "raw",
        wire: vec![0, 255, 65],
    }];
    for (index, value) in [0, 255, 65].into_iter().enumerate() {
        let field = ModelField {
            path: format!("root.object.payload.bytes_3b8[{index}]").into(),
            value: ModelFieldValue::U8(value),
        };
        assert_eq!(compare_field(&field, index, &spans), index + 1);
    }
}

#[test]
#[should_panic]
fn packed_alignment_byte_comparison_rejects_a_widened_word() {
    let spans = [PrimitiveSpan {
        start: 0,
        end: 8,
        kind: "raw",
        wire: vec![0; 8],
    }];
    let field = ModelField {
        path: "root.object.payload.bytes_3b8[0]".into(),
        value: ModelFieldValue::U64(0),
    };
    compare_field(&field, 0, &spans);
}

#[test]
fn packed_sketch_transform_words_match_arbitrary_raw_bits() {
    let spans = [PrimitiveSpan {
        start: 0,
        end: 4,
        kind: "raw",
        wire: vec![0x98, 0xba, 0xdc, 0xfe],
    }];
    let field = ModelField {
        path: "root.object.payload.transforms_1148.words[28]".into(),
        value: ModelFieldValue::U32(0xfedcba98),
    };
    assert_eq!(compare_field(&field, 0, &spans), 4);
}

#[test]
#[should_panic]
fn packed_sketch_transform_words_reject_widened_values() {
    let spans = [PrimitiveSpan {
        start: 0,
        end: 8,
        kind: "raw",
        wire: vec![0; 8],
    }];
    let field = ModelField {
        path: "root.object.payload.transforms_1148.words[0]".into(),
        value: ModelFieldValue::U64(0),
    };
    compare_field(&field, 0, &spans);
}
