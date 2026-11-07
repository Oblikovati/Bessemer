use bessemer::composer::geometry::*;
use std::io::{Cursor, Read};

pub const LIMITS: ComposerSceneLimits = ComposerSceneLimits {
    maximum_encoded_bytes: 100000000,
    maximum_decoded_bytes: 100000000,
    maximum_fields: 100000,
    maximum_records: 10000,
    maximum_text_units: 1000000,
};

pub struct SceneCase<'a> {
    pub context: ComposerSceneContext,
    pub encoded: &'a [u8],
    pub decoded: &'a [u8],
}

pub fn scene_cases(raw: &[u8]) -> Vec<SceneCase<'_>> {
    let mut reader = Cursor::new(raw);
    assert_eq!(span(&mut reader, 4), b"SCNE");
    let cases = (0..word(&mut reader))
        .map(|_| read_case(&mut reader))
        .collect();
    assert_eq!(reader.position() as usize, raw.len());
    cases
}

fn read_case<'a>(reader: &mut Cursor<&'a [u8]>) -> SceneCase<'a> {
    let enabled = span(reader, 1)[0];
    assert!(enabled <= 1);
    let (encoded, decoded) = (word(reader), word(reader));
    SceneCase {
        context: ComposerSceneContext {
            texture_import_enabled: enabled == 1,
        },
        encoded: span(reader, encoded as usize),
        decoded: span(reader, decoded as usize),
    }
}

fn word(reader: &mut Cursor<&[u8]>) -> u32 {
    let mut raw = [0; 4];
    reader.read_exact(&mut raw).unwrap();
    u32::from_le_bytes(raw)
}

fn span<'a>(reader: &mut Cursor<&'a [u8]>, count: usize) -> &'a [u8] {
    let input = *reader.get_ref();
    let start = reader.position() as usize;
    reader.set_position((start + count) as u64);
    &input[start..start + count]
}

pub fn check_scene(case: &SceneCase<'_>) -> ComposerGeometryArchive {
    let archive = ComposerGeometryArchive::from_gzip(case.encoded, case.context, LIMITS).unwrap();
    assert_eq!(archive.to_decoded().unwrap(), case.decoded);
    assert_eq!(archive.to_gzip().unwrap(), case.encoded);
    assert!(archive.coverage().input_gzip_integrity_checked);
    let constructed =
        ComposerGeometryArchive::from_fields(archive.fields(), case.context, LIMITS).unwrap();
    assert_eq!(constructed.to_decoded().unwrap(), case.decoded);
    let gzip = constructed.to_gzip().unwrap();
    let reopened = ComposerGeometryArchive::from_gzip(&gzip, case.context, LIMITS).unwrap();
    assert_eq!(reopened.to_decoded().unwrap(), case.decoded);
    archive
}
