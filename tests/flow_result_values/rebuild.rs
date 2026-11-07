use super::fixture::*;
pub(super) fn reconstruct(row: &ValueCase) -> Vec<u8> {
    let mut raw = vec![row.frame.version];
    assert_eq!(row.context.identifiers.len(), row.frame.objects.len());
    for (object, identifier) in row.frame.objects.iter().zip(&row.context.identifiers) {
        assert_eq!(object.identifier, *identifier);
        append_object(&mut raw, object);
    }
    raw
}
fn append_object(raw: &mut Vec<u8>, object: &ImsValueObject) {
    assert_eq!(object.start as usize, raw.len());
    assert_eq!(object.version, 3);
    raw.extend_from_slice(&object.version.to_le_bytes());
    raw.push(object.first_flag);
    append_optional(raw, object.first_flag, object.first.as_ref());
    append_array(raw, &object.main);
    raw.push(object.bl_flag);
    assert_eq!(object.bl_flag != 0, object.bl.is_some());
    if let Some(bl) = &object.bl {
        append_bl(raw, bl);
    }
    raw.push(object.last_flag);
    append_optional(raw, object.last_flag, object.last.as_ref());
    assert_eq!(object.end as usize, raw.len());
}
fn append_optional(raw: &mut Vec<u8>, flag: u8, array: Option<&ValueArray>) {
    assert_eq!(flag != 0, array.is_some());
    if let Some(array) = array {
        append_array(raw, array);
    }
}
fn append_array(raw: &mut Vec<u8>, array: &ValueArray) {
    assert_eq!(array.start as usize, raw.len());
    assert_eq!(array.marker, 1);
    assert!(array.count <= i32::MAX as u32);
    assert_eq!(array.raw.len(), array.count as usize * 8);
    raw.extend_from_slice(&array.marker.to_le_bytes());
    raw.extend_from_slice(&(array.count as u64).to_le_bytes());
    raw.extend_from_slice(&array.raw);
    assert_eq!(array.end as usize, raw.len());
}
fn append_bl(raw: &mut Vec<u8>, bl: &BlRows) {
    assert_eq!(bl.start as usize, raw.len());
    assert_eq!(bl.version, 0);
    assert!(bl.count <= i32::MAX as u32);
    assert_eq!(bl.raw.len(), bl.count as usize * 144);
    assert_eq!(bl.native_state, 0);
    raw.extend_from_slice(&bl.version.to_le_bytes());
    raw.extend_from_slice(&(bl.count as u64).to_le_bytes());
    raw.extend_from_slice(&bl.raw);
    assert_eq!(bl.end as usize, raw.len());
}
