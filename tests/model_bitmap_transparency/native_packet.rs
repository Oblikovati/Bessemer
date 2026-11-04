use super::*;

pub(super) struct NativeBitmapCase<'a> {
    pub index: usize,
    pub version: ArchiveVersion,
    pub label: &'a str,
    pub header: &'a [u8],
    pub configuration: &'a [u8],
    pub manager: &'a [u8],
    pub features: &'a [u8],
    pub offsets: [usize; 3],
}

fn native_packet_bytes<'a>(cursor: &mut &'a [u8], length: usize) -> &'a [u8] {
    let (selected, remaining) = cursor.split_at(length);
    *cursor = remaining;
    selected
}

fn native_packet_word(cursor: &mut &[u8]) -> u32 {
    u32::from_le_bytes(native_packet_bytes(cursor, 4).try_into().unwrap())
}

fn next_native_bitmap_case<'a>(cursor: &mut &'a [u8], index: usize) -> NativeBitmapCase<'a> {
    let version = ArchiveVersion::new(native_packet_word(cursor));
    let lengths: [usize; 5] = std::array::from_fn(|_| native_packet_word(cursor) as usize);
    NativeBitmapCase {
        index,
        version,
        label: std::str::from_utf8(native_packet_bytes(cursor, lengths[0])).unwrap(),
        header: native_packet_bytes(cursor, lengths[1]),
        configuration: native_packet_bytes(cursor, lengths[2]),
        manager: native_packet_bytes(cursor, lengths[3]),
        features: native_packet_bytes(cursor, lengths[4]),
        offsets: [0; 3],
    }
}

fn bind_native_bitmap_offsets(case: &mut NativeBitmapCase<'_>, expected: &[usize]) {
    assert_eq!(case.index, expected[0]);
    assert_eq!(case.version.value() as usize, expected[1]);
    assert_eq!(case.features.len(), expected[2]);
    case.offsets.copy_from_slice(&expected[3..]);
    for offset in case.offsets {
        assert_eq!(&case.features[offset..offset + 4], &[0; 4]);
    }
}

pub(super) fn native_bitmap_cases(packet: &[u8]) -> Vec<NativeBitmapCase<'_>> {
    let mut cursor = packet;
    assert_eq!(native_packet_bytes(&mut cursor, 4), b"BRF1");
    assert_eq!(native_packet_word(&mut cursor), 906);
    let mut selected = Vec::new();
    for index in 0..906 {
        let case = next_native_bitmap_case(&mut cursor, index);
        if [340, 675, 717, 739].contains(&index) {
            selected.push(case);
        }
    }
    assert!(cursor.is_empty());
    assert_eq!(selected.len(), 4);
    for (case, row) in selected
        .iter_mut()
        .zip(include_str!("../fixtures/bitmap-transparency/native-spans.tsv").lines())
    {
        let expected: Vec<usize> = row.split('\t').map(|word| word.parse().unwrap()).collect();
        bind_native_bitmap_offsets(case, &expected);
    }
    selected
}
