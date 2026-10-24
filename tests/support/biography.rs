use bessemer::biography::{BiographyRecord, EnvironmentSnapshot};

pub const CASES: &[(&str, &[u8])] = &[
    (
        "blank-part",
        include_bytes!("../fixtures/biography/blank-part.bin"),
    ),
    (
        "blank-assembly",
        include_bytes!("../fixtures/biography/blank-assembly.bin"),
    ),
    (
        "blank-drawing",
        include_bytes!("../fixtures/biography/blank-drawing.bin"),
    ),
    ("mixed", include_bytes!("../fixtures/biography/mixed.bin")),
    ("empty", include_bytes!("../fixtures/biography/empty.bin")),
];

pub fn projection(records: &[BiographyRecord]) -> String {
    records
        .iter()
        .map(record_projection)
        .collect::<Vec<_>>()
        .join("|")
}

fn record_projection(record: &BiographyRecord) -> String {
    let mut fields = vec![
        record.file_version.to_string(),
        record.date_code.to_string(),
        record.unknown_10.to_string(),
    ];
    fields.extend(
        [
            record.processor_architecture,
            record.processor_count,
            record.os_platform_id,
            record.os_major,
            record.os_minor,
            record.os_build,
        ]
        .map(|word| word.to_string()),
    );
    fields.extend([
        text_hex(record.os_csd_version.text()),
        record.language_id.to_string(),
    ]);
    for snapshot in &record.snapshots {
        snapshot_projection(snapshot, &mut fields);
    }
    fields.join(",")
}

fn snapshot_projection(snapshot: &EnvironmentSnapshot, fields: &mut Vec<String>) {
    fields.extend([
        snapshot.filetime.to_string(),
        snapshot.available_page_file_low32.to_string(),
    ]);
    for group in &snapshot.paths {
        fields.extend([
            text_hex(group.path.text()),
            group.drive_type.to_string(),
            group.available_disk_bytes.to_string(),
        ]);
    }
}

fn text_hex(text: &str) -> String {
    text.as_bytes()
        .iter()
        .map(|byte| format!("{byte:02x}"))
        .collect()
}
