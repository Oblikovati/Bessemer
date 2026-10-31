use super::{Cursor, groups, limits, profile};
use bessemer::simulation::*;

pub fn table(encoded: &[u8]) -> SimulationEgrArchive {
    SimulationEgrArchive::from_bytes(encoded, profile(), limits()).unwrap()
}

pub fn metadata(image: &SimulationEgrArchive, expected: &[u8]) {
    let mut cursor = Cursor(expected);
    for word in image.header_words() {
        assert_eq!(*word as u32, cursor.word());
    }
    assert_eq!(image.header_padding(), cursor.span());
    assert_eq!(image.field_alignment(), cursor.span());
    assert_eq!(image.directory_padding(), cursor.span());
    fields(image, &mut cursor);
    directory(image, &mut cursor);
    physical(image, &mut cursor);
    live_rows(image, &mut cursor);
    free_rows(image, &mut cursor);
    assert!(cursor.0.is_empty());
    assert_coverage(image);
}

fn fields(image: &SimulationEgrArchive, cursor: &mut Cursor<'_>) {
    assert_eq!(image.fields().len(), cursor.word() as usize);
    for field in image.fields() {
        assert_eq!(field.tag as u32, cursor.word());
        assert_eq!(field.width, cursor.word() as usize);
        assert_eq!(field.row_end - field.row_offset, field.width);
    }
}

fn directory(image: &SimulationEgrArchive, cursor: &mut Cursor<'_>) {
    assert_eq!(image.directory().len(), cursor.word() as usize);
    for number in image.directory() {
        assert_eq!(*number as u32, cursor.word());
    }
    assert_eq!(image.coverage().allocated_records, cursor.word() as usize);
    assert_eq!(
        image.coverage().unallocated_capacity,
        cursor.word() as usize
    );
}

fn physical(image: &SimulationEgrArchive, cursor: &mut Cursor<'_>) {
    let raw = image.allocated_record_bytes();
    assert_eq!(raw.len() / 128, cursor.word() as usize);
    for record in raw.as_chunks::<128>().0 {
        assert_eq!(record, cursor.span());
    }
}

fn live_rows(image: &SimulationEgrArchive, cursor: &mut Cursor<'_>) {
    assert_eq!(image.coverage().live_rows, cursor.word() as usize);
    for (index, number) in image
        .directory()
        .iter()
        .enumerate()
        .filter(|(_, number)| **number != 0)
    {
        assert_eq!(index + 1, cursor.word() as usize);
        assert_eq!(*number as u32, cursor.word());
        let row =
            &image.allocated_record_bytes()[(*number as usize - 1) * 128..*number as usize * 128];
        let parts = image.parts();
        let start = 128 + parts.field_directory.len() + parts.row_directory.len();
        assert_eq!(start + (*number as usize - 1) * 128, cursor.word() as usize);
        let id = image.live_row_id(index as u32 + 1).unwrap().unwrap();
        live_fields(image, row, cursor);
        groups::assert_file_row(image, &id, cursor);
    }
}

fn live_fields(image: &SimulationEgrArchive, row: &[u8], cursor: &mut Cursor<'_>) {
    assert_eq!(image.fields().len(), cursor.word() as usize);
    let mut end = 0;
    for field in image.fields() {
        assert_eq!(field.tag as u32, cursor.word());
        assert_eq!(field.row_offset, cursor.word() as usize);
        assert_eq!(field.row_end, cursor.word() as usize);
        assert_eq!(&row[field.row_offset..field.row_end], cursor.span());
        end = field.row_end;
    }
    assert_eq!(&row[end..], cursor.span());
}

fn free_rows(image: &SimulationEgrArchive, cursor: &mut Cursor<'_>) {
    assert_eq!(image.free_records().len(), cursor.word() as usize);
    for link in image.free_records() {
        assert_eq!(link.record_number, cursor.word());
        assert_eq!(link.next_record_number as u32, cursor.word());
        let start = (link.record_number as usize - 1) * 128;
        assert_eq!(
            &image.allocated_record_bytes()[start..start + 128],
            cursor.span()
        );
    }
}

fn assert_coverage(image: &SimulationEgrArchive) {
    let coverage = image.coverage();
    assert_eq!(
        coverage.check_mode,
        SimulationEgrCheckMode::ExactMetadataAndAllocatedPhysicalPartition
    );
    assert_eq!(coverage.profile, profile());
    assert_eq!(coverage.checked_bytes, image.to_bytes().len());
    assert_eq!(coverage.logical_capacity, image.directory().len());
    assert_eq!(coverage.loaded_fields, image.fields().len());
    assert_eq!(
        coverage.live_rows + coverage.deleted_records,
        coverage.allocated_records
    );
    assert_eq!(
        coverage.allocated_records + coverage.unallocated_capacity,
        coverage.logical_capacity
    );
    assert!(!coverage.semantics_complete);
}
