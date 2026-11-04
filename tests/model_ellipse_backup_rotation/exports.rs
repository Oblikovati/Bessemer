use super::*;
use std::ffi::OsString;

pub(super) fn ellipse_export_start(
    variable: &str,
    magic: &[u8; 4],
    count: u32,
) -> (Option<OsString>, Option<Vec<u8>>) {
    let path = std::env::var_os(variable);
    let packet = path
        .as_ref()
        .map(|_| [magic.as_slice(), &count.to_le_bytes()].concat());
    (path, packet)
}

pub(super) fn append_ellipse_export(
    packet: &mut Option<Vec<u8>>,
    indices: [usize; 3],
    version: u32,
    raw: &[u8],
) {
    if let Some(packet) = packet {
        for index in indices {
            packet.extend_from_slice(&u32::try_from(index).unwrap().to_le_bytes());
        }
        packet.extend_from_slice(&version.to_le_bytes());
        packet.extend_from_slice(&u32::try_from(raw.len()).unwrap().to_le_bytes());
        packet.extend_from_slice(raw);
    }
}

pub(super) fn finish_ellipse_export(path: Option<OsString>, packet: Option<Vec<u8>>) {
    if let (Some(path), Some(packet)) = (path, packet) {
        std::fs::write(path, packet).unwrap();
    }
}

fn append_owned_ellipse_operations(
    index: usize,
    case: &EllipseCase,
    source: &PartResolvedFeaturesArchive,
    output: &mut Option<Vec<u8>>,
) {
    for owner in 0..2 {
        for (request, snapshot) in REQUESTS.into_iter().enumerate() {
            let actual = verify_ellipse_edit(source, case, owner, snapshot);
            append_ellipse_export(output, [index, owner, request], case.version, &actual);
        }
    }
}

#[test]
#[ignore = "Requires native historical contexts and explicit BESSEMER_ELLIPSE_OWNED_EXPORT path"]
fn export_actual_checked_ellipse_operations_for_independent_source_acceptance() {
    let (path, mut packet) = ellipse_export_start("BESSEMER_ELLIPSE_OWNED_EXPORT", b"EBO1", 1080);
    assert!(
        path.is_some(),
        "expected BESSEMER_ELLIPSE_OWNED_EXPORT output path"
    );
    for (index, case) in ellipse_cases().iter().enumerate() {
        let source = historical::historical_ellipse_context(case);
        append_owned_ellipse_operations(index, case, &source, &mut packet);
    }
    finish_ellipse_export(path, packet);
}
