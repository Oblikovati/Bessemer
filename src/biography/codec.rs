use super::{BiographyRecord, EnvironmentPath, EnvironmentSnapshot, invalid};
use crate::Result;
use crate::archive_codec::{
    ArchiveClass, ArchiveCursor, ArchiveString, ObjectReader, ObjectRecord, ObjectWriter,
};

pub(super) fn decode(bytes: &[u8]) -> Result<(u16, Vec<BiographyRecord>)> {
    let mut cursor = ArchiveCursor::new(bytes);
    let root = ObjectReader::new().object(&mut cursor)?;
    let ObjectRecord::New { class, .. } = root else {
        return Err(invalid(
            "root null or reference; expected new moBiography_c",
        ));
    };
    if class.name != "moBiography_c" {
        return Err(invalid(format!(
            "root class {:?}; expected moBiography_c",
            class.name
        )));
    }
    let count = cursor.nonnegative_count(219, "biography records")?;
    let records = (0..count)
        .map(|_| read_record(&mut cursor))
        .collect::<Result<Vec<_>>>()?;
    cursor.finish()?;
    Ok((class.schema, records))
}

fn read_record(cursor: &mut ArchiveCursor<'_>) -> Result<BiographyRecord> {
    Ok(BiographyRecord {
        file_version: cursor.u32()?,
        date_code: cursor.i32()?,
        unknown_10: cursor.i32()?,
        processor_architecture: cursor.u32()?,
        processor_count: cursor.u32()?,
        os_platform_id: cursor.u32()?,
        os_major: cursor.u32()?,
        os_minor: cursor.u32()?,
        os_build: cursor.u32()?,
        os_csd_version: ArchiveString::decode(cursor)?,
        language_id: cursor.u16()?,
        snapshots: [read_snapshot(cursor)?, read_snapshot(cursor)?],
    })
}

fn read_snapshot(cursor: &mut ArchiveCursor<'_>) -> Result<EnvironmentSnapshot> {
    let filetime = cursor.u64()?;
    let available_page_file_low32 = cursor.u32()?;
    let paths = [
        read_path(cursor)?,
        read_path(cursor)?,
        read_path(cursor)?,
        read_path(cursor)?,
        read_path(cursor)?,
        read_path(cursor)?,
    ];
    Ok(EnvironmentSnapshot {
        filetime,
        available_page_file_low32,
        paths,
    })
}

fn read_path(cursor: &mut ArchiveCursor<'_>) -> Result<EnvironmentPath> {
    Ok(EnvironmentPath {
        path: ArchiveString::decode(cursor)?,
        drive_type: cursor.i32()?,
        available_disk_bytes: cursor.u64()?,
    })
}

pub(super) fn encode(schema: u16, records: &[BiographyRecord]) -> Result<Vec<u8>> {
    let count = checked_length(records)?;
    let mut output = Vec::new();
    let class = ArchiveClass {
        name: "moBiography_c".to_owned(),
        schema,
    };
    ObjectWriter::new().object(&class, &mut output)?;
    output.extend_from_slice(&count.to_le_bytes());
    for record in records {
        write_record(record, &mut output)?;
    }
    Ok(output)
}

pub(super) fn checked_length(records: &[BiographyRecord]) -> Result<i32> {
    i32::try_from(records.len()).map_err(|_| {
        invalid(format!(
            "record count {}; expected 0..={}",
            records.len(),
            i32::MAX
        ))
    })
}

fn write_record(record: &BiographyRecord, output: &mut Vec<u8>) -> Result<()> {
    output.extend_from_slice(&record.file_version.to_le_bytes());
    output.extend_from_slice(&record.date_code.to_le_bytes());
    output.extend_from_slice(&record.unknown_10.to_le_bytes());
    for word in [
        record.processor_architecture,
        record.processor_count,
        record.os_platform_id,
        record.os_major,
        record.os_minor,
        record.os_build,
    ] {
        output.extend_from_slice(&word.to_le_bytes());
    }
    record.os_csd_version.encode(output)?;
    output.extend_from_slice(&record.language_id.to_le_bytes());
    for snapshot in &record.snapshots {
        write_snapshot(snapshot, output)?;
    }
    Ok(())
}

fn write_snapshot(snapshot: &EnvironmentSnapshot, output: &mut Vec<u8>) -> Result<()> {
    output.extend_from_slice(&snapshot.filetime.to_le_bytes());
    output.extend_from_slice(&snapshot.available_page_file_low32.to_le_bytes());
    for group in &snapshot.paths {
        group.path.encode(output)?;
        output.extend_from_slice(&group.drive_type.to_le_bytes());
        output.extend_from_slice(&group.available_disk_bytes.to_le_bytes());
    }
    Ok(())
}
