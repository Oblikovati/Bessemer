use super::BiographyRecord;
use crate::Result;
use crate::archive_codec::ArchiveString;

pub(super) fn expanded(records: &[BiographyRecord]) -> Vec<BiographyRecord> {
    let mut result = records.to_vec();
    for index in 1..result.len() {
        let (before, after) = result.split_at_mut(index);
        for (previous, current) in before[index - 1].strings().zip(after[0].strings_mut()) {
            if current.text() == "*" {
                *current = previous.clone();
            }
        }
    }
    result
}

pub(super) fn abbreviated(records: &[BiographyRecord]) -> Result<Vec<BiographyRecord>> {
    let expanded = expanded(records);
    let mut result = expanded.clone();
    for (pair, current) in expanded.windows(2).zip(result.iter_mut().skip(1)) {
        for ((previous, resolved), stored) in pair[0]
            .strings()
            .zip(pair[1].strings())
            .zip(current.strings_mut())
        {
            if !resolved.text().is_empty() && resolved.text() == previous.text() {
                *stored = ArchiveString::new("*", stored.encoding())?;
            }
        }
    }
    Ok(result)
}
