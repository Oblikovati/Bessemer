use super::fixture::*;

pub(super) fn reconstruct(row: &DefinitionCase) -> Vec<u8> {
    let mut raw = vec![row.version];
    assert_eq!(row.groups.len(), 5);
    for (ordinal, group) in row.groups.iter().enumerate() {
        assert_eq!(group.start as usize, raw.len());
        assert!(ordinal == 0 || group.objects.is_empty());
        raw.extend_from_slice(&(group.objects.len() as i32).to_le_bytes());
        for object in &group.objects {
            raw.extend_from_slice(&object.identifier.to_le_bytes());
        }
        for object in &group.objects {
            append_object(&mut raw, object);
        }
        assert_eq!(group.end as usize, raw.len());
    }
    raw
}
fn append_object(raw: &mut Vec<u8>, object: &DefinitionObject) {
    const GUID: [u8; 16] = [
        0x28, 0x84, 0x2c, 0xcb, 0xaa, 3, 0x13, 0x46, 0xb1, 0x5a, 0x36, 0x1e, 0x44, 0x61, 0xbd, 0x20,
    ];
    assert_eq!(object.guid, GUID);
    raw.extend_from_slice(&GUID);
    let body = &object.body;
    assert_eq!(body.start as usize, raw.len());
    raw.extend_from_slice(&body.version.to_le_bytes());
    check_presence(body);
    append_array(raw, body.arrays[0].as_ref());
    append_array(raw, body.arrays[1].as_ref());
    if let Some(flag) = body.allocation {
        raw.push(flag);
    }
    append_array(raw, body.arrays[2].as_ref());
    append_array(raw, body.arrays[3].as_ref());
    assert_eq!(body.end as usize, raw.len());
}
fn check_presence(body: &ImsBody) {
    let version = body.version;
    assert!((0..=3).contains(&version));
    let present = [version >= 1, version >= 2, version == 3, version == 3];
    assert_eq!(body.arrays.each_ref().map(Option::is_some), present);
    assert_eq!(body.allocation.is_some(), version >= 2);
}
fn append_array(raw: &mut Vec<u8>, array: Option<&DefinitionArray>) {
    let Some(array) = array else {
        return;
    };
    assert_eq!(array.marker, 1);
    assert_eq!(array.start as usize, raw.len());
    assert_eq!(array.raw.len(), array.count as usize * 4);
    assert!(array.count <= i32::MAX as u32);
    raw.extend_from_slice(&array.marker.to_le_bytes());
    raw.extend_from_slice(&(array.count as u64).to_le_bytes());
    raw.extend_from_slice(&array.raw);
    assert_eq!(array.end as usize, raw.len());
}
