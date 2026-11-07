use super::fixture::*;
use super::streams;
pub(super) fn check_case(row: &ControlCase) {
    assert_eq!(row.marker, 0);
    assert!(row.complete);
    assert_eq!(row.decoded.len(), row.end as usize);
    let decoded = if row.identifier == SAVED {
        saved(row)
    } else {
        index(row)
    };
    assert_eq!(decoded, row.decoded, "{}", row.name);
}
fn saved(row: &ControlCase) -> Vec<u8> {
    assert_eq!(row.identifier, SAVED);
    assert_eq!(row.width, 0);
    assert!(row.footer.is_none() && row.signed_count.is_none());
    assert_eq!(row.compressed as usize, row.stored.len());
    assert_eq!(streams::inflate_exact(&row.stored), row.decoded);
    assert_eq!(row.entries.len(), row.count as usize * 16);
    let mut raw = row.marker.to_le_bytes().to_vec();
    raw.extend_from_slice(&row.count.to_le_bytes());
    raw.extend_from_slice(&row.entries);
    raw
}
fn index(row: &ControlCase) -> Vec<u8> {
    assert_eq!(row.identifier, INDEX);
    assert!(row.width == 4 || row.width == 8);
    let count = row.signed_count.unwrap();
    assert!(count >= 0);
    assert_eq!(count as u64, row.count);
    assert_eq!(row.entries.len(), row.count as usize * 20);
    let footer = row.footer.as_ref().unwrap();
    assert_eq!(row.stored[..8], row.marker.to_le_bytes());
    assert_eq!(footer.start as usize, 8 + row.compressed as usize);
    assert_eq!(
        streams::inflate_exact(&row.stored[8..footer.start as usize]),
        row.decoded
    );
    check_footer(row, footer);
    let mut raw = count.to_le_bytes().to_vec();
    raw.extend_from_slice(&row.entries);
    raw
}
fn check_footer(row: &ControlCase, footer: &ControlFooter) {
    assert_eq!(footer.identifier, INDEX);
    assert_eq!(
        footer.distance,
        u64::from(footer.start) + u64::from(row.width) + 4
    );
    assert_eq!(footer.distance as usize, row.stored.len());
    let footer_start = footer.start as usize;
    let width = row.width as usize;
    let mut distance = [0_u8; 8];
    distance[..width].copy_from_slice(&row.stored[footer_start..footer_start + width]);
    assert_eq!(u64::from_le_bytes(distance), footer.distance);
    assert_eq!(&row.stored[footer_start + width..], &INDEX.to_le_bytes());
}
