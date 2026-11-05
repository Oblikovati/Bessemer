use std::ffi::OsString;

pub(super) fn appearance_export() -> (Option<OsString>, Option<Vec<u8>>) {
    let path = std::env::var_os("BESSEMER_APPEARANCE_EXPORT");
    let output = path
        .as_ref()
        .map(|_| [b"BAE1".as_slice(), &24000_u32.to_le_bytes()].concat());
    (path, output)
}

pub(super) fn append_appearance_export(output: &mut Option<Vec<u8>>, version: u32, actual: &[u8]) {
    if let Some(output) = output {
        output.extend_from_slice(&version.to_le_bytes());
        output.extend_from_slice(&u32::try_from(actual.len()).unwrap().to_le_bytes());
        output.extend_from_slice(actual);
    }
}

pub(super) fn finish_appearance_export(path: Option<OsString>, output: Option<Vec<u8>>) {
    if let (Some(path), Some(output)) = (path, output) {
        std::fs::write(path, output).unwrap();
    }
}
