//! Complete Source-admitted drawing property owner with real shared object keys (#181).
use bessemer::display_lists::*;
use bessemer::{FailureClass, header::ArchiveVersion, tessellation::StoredFloat};
#[allow(
    dead_code,
    reason = "Shared archive helpers serve the complete display corpus."
)]
#[path = "support/display_archive.rs"]
mod archive_support;
const VERSION: ArchiveVersion = ArchiveVersion::new(10200);
const VISUAL_OFFSET: usize = 442;
const ROOT_HEX: &str = concat!(
    "03000000ffff01000e007569566965774275636b65745f6300000000000000000000000000000000",
    "00000000000000000000000000000000000000000000000000000000000000000000000000000000",
    "00000000000000000000000000000000000000000000000000000000000000000000000000000000",
    "000000000000000000000000000002000000ffff010013006d6f436f6d706f6e656e7450726f7065",
    "72747900efff00000080000000efff00000080000000efff00000080000000efff00000080000000",
    "efff00000080000000efff00000080000000efff00000080000000efff00000080000000efff0000",
    "0080000000efff00000080000000efff00000080000000efff00000080000000efff000000800000",
    "00efff00000080000000efff00000080000000efff00000080000000efff00000080000000efff00",
    "000080000000efff00000080000000efff00000080000000efff00000080000000efff0000008000",
    "0000efff00000080000000efff00000080000000efff00000080000000efff00000080000000efff",
    "00000080000000efff00000080000000efff00000080000000efff00000080000000efff00000080",
    "00001700a5a5fd123456800080008000000080ffefffffff00000000040000000000000000000000",
    "00000000000000000000000000000000000000000000000000000000000000000000000000000000",
    "00000000000000000000000000000000000001000000020000000200000000000000",
);

fn drawing_visual_bytes() -> Vec<u8> {
    let (pairs, remainder) = ROOT_HEX.as_bytes().as_chunks::<2>();
    assert!(remainder.is_empty());
    let raw: Vec<_> = pairs
        .iter()
        .map(|pair| u8::from_str_radix(std::str::from_utf8(pair).unwrap(), 16).unwrap())
        .collect();
    assert_eq!(raw.len(), 554);
    raw
}

fn drawing_visual_archive() -> DrawingBucketArchive {
    DrawingBucketArchive::from_bytes(&drawing_visual_bytes(), VERSION, archive_support::LIMITS)
        .unwrap()
}

fn drawing_visual_position(source: &DrawingBucketArchive) -> usize {
    let positions: Vec<_> = source
        .contents()
        .fields
        .iter()
        .enumerate()
        .filter(|(_, field)| field.path.ends_with(".ent_vis_prop.unknown_0"))
        .map(|(index, _)| index)
        .collect();
    assert_eq!(positions.len(), 1);
    positions[0]
}

fn drawing_literal_values() -> [DisplayFieldValue; 7] {
    [
        DisplayFieldValue::U16(23),
        DisplayFieldValue::U16(0xa5a5),
        DisplayFieldValue::U8(0xfd),
        DisplayFieldValue::U32(0x80563412),
        DisplayFieldValue::U16(0x8000),
        DisplayFieldValue::I16(-32768),
        DisplayFieldValue::F32(StoredFloat::from_bits(0x80000000)),
    ]
}

fn flipped_drawing_word(value: &DisplayFieldValue, bit: usize) -> DisplayFieldValue {
    match value {
        DisplayFieldValue::U8(word) => DisplayFieldValue::U8(word ^ (1 << bit)),
        DisplayFieldValue::U16(word) => DisplayFieldValue::U16(word ^ (1 << bit)),
        DisplayFieldValue::I16(word) => {
            DisplayFieldValue::I16(((*word as u16) ^ (1 << bit)) as i16)
        }
        DisplayFieldValue::U32(word) => DisplayFieldValue::U32(word ^ (1 << bit)),
        DisplayFieldValue::F32(word) => {
            DisplayFieldValue::F32(StoredFloat::from_bits(word.bits() ^ (1 << bit)))
        }
        other => panic!("{other:?}; expected one of the seven stored visual scalar types"),
    }
}

fn drawing_bit_target(bit: usize, start: usize) -> (usize, usize) {
    let offsets = [0, 2, 4, 5, 9, 11, 13];
    let byte = bit / 8;
    let index = offsets.iter().rposition(|offset| *offset <= byte).unwrap();
    (start + index, (byte - offsets[index]) * 8 + bit % 8)
}

fn drawing_bit_result(source: &DrawingBucketArchive, bit: usize) -> Vec<u8> {
    let (index, local_bit) = drawing_bit_target(bit, drawing_visual_position(source));
    let mut contents = source.contents().clone();
    contents.fields[index].value = flipped_drawing_word(&contents.fields[index].value, local_bit);
    let edited = source
        .with_contents(contents.clone(), archive_support::LIMITS)
        .unwrap();
    assert_eq!(edited.contents(), &contents);
    let mut expected = drawing_visual_bytes();
    expected[VISUAL_OFFSET + bit / 8] ^= 1 << (bit % 8);
    assert_eq!(edited.to_bytes().unwrap(), expected);
    let reverted = edited
        .with_contents(source.contents().clone(), archive_support::LIMITS)
        .unwrap();
    assert_eq!(reverted.to_bytes().unwrap(), drawing_visual_bytes());
    assert_eq!(source.to_bytes().unwrap(), drawing_visual_bytes());
    expected
}

fn drawing_bit_projection(source: &DrawingBucketArchive) -> Vec<u8> {
    let mut output = 136_u32.to_le_bytes().to_vec();
    for bit in 0..136 {
        let raw = drawing_bit_result(source, bit);
        output.extend_from_slice(&(raw.len() as u32).to_le_bytes());
        output.extend_from_slice(&raw);
    }
    output
}

fn verify_drawing_object_roles(source: &DrawingBucketArchive) {
    let mut counts = (0, 0, 0);
    for field in &source.contents().fields {
        let DisplayFieldValue::ObjectTag(tag) = &field.value else {
            continue;
        };
        match tag.target {
            DisplayObjectPointer::New(_) => counts.0 += 1,
            DisplayObjectPointer::Reference(_) => counts.1 += 1,
            DisplayObjectPointer::Null => counts.2 += 1,
        }
    }
    assert_eq!(counts, (2, 3, 2));
}

#[test]
fn complete_drawing_owner_reconstructs_literal_visuals_aliases_and_rejects_legacy_context() {
    let source = drawing_visual_archive();
    let index = drawing_visual_position(&source);
    let actual: Vec<_> = source.contents().fields[index..index + 7]
        .iter()
        .map(|field| field.value.clone())
        .collect();
    assert_eq!(actual, drawing_literal_values());
    let rebuilt =
        DrawingBucketArchive::new(source.contents().clone(), VERSION, archive_support::LIMITS)
            .unwrap();
    assert_eq!(rebuilt.to_bytes().unwrap(), drawing_visual_bytes());
    verify_drawing_object_roles(&source);
    let error = DrawingBucketArchive::from_bytes(
        &drawing_visual_bytes(),
        ArchiveVersion::new(0x27cb),
        archive_support::LIMITS,
    )
    .unwrap_err();
    assert_eq!(error.class, FailureClass::Unsupported);
}

#[test]
fn drawing_visual_bits_match_literal_bytes_in_eight_workers_and_optional_process_export() {
    let source = drawing_visual_archive();
    let serial = drawing_bit_projection(&source);
    std::thread::scope(|scope| {
        let workers: Vec<_> = (0..8)
            .map(|_| scope.spawn(|| drawing_bit_projection(&source)))
            .collect();
        for worker in workers {
            assert_eq!(worker.join().unwrap(), serial);
        }
    });
    if let Some(path) = std::env::var_os("BESSEMER_DRAWING_VISUAL_OUTPUT") {
        std::fs::write(path, &serial).unwrap();
    }
}
