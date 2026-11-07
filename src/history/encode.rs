use std::collections::BTreeMap;

use crate::Result;
use crate::archive_codec::{ArchiveClass, ArchiveMapIndex, ObjectWriter};

use super::{
    DateCodeHistory, DateCodeId, HistoryContents, HistoryTail, RevisionStamp, StateDateCodes,
    VersionState, invalid,
};

struct HistoryWriter {
    objects: ObjectWriter,
    dates: BTreeMap<DateCodeId, ArchiveMapIndex>,
}

pub(super) fn history(schema: u16, contents: &HistoryContents) -> Result<Vec<u8>> {
    let mut output = Vec::new();
    let mut writer = HistoryWriter {
        objects: ObjectWriter::new(),
        dates: BTreeMap::new(),
    };
    let class = ArchiveClass {
        name: "moVersionHistory_c".into(),
        schema,
    };
    writer.objects.object(&class, &mut output)?;
    output.extend_from_slice(&(contents.states.len() as i32).to_le_bytes());
    for state in &contents.states {
        writer.state(&mut output, state, contents)?;
    }
    write_tail(&mut output, &contents.tail);
    Ok(output)
}

impl HistoryWriter {
    fn state(
        &mut self,
        output: &mut Vec<u8>,
        state: &VersionState,
        contents: &HistoryContents,
    ) -> Result<()> {
        write_stamp(output, &state.stamp)?;
        output.extend_from_slice(&state.file_version.to_le_bytes());
        match state.date_codes {
            StateDateCodes::Absent => Ok(()),
            StateDateCodes::Legacy(value) => {
                output.extend_from_slice(&value.to_le_bytes());
                Ok(())
            }
            StateDateCodes::History(identity) => self.date_object(output, identity, contents),
        }
    }

    fn date_object(
        &mut self,
        output: &mut Vec<u8>,
        identity: DateCodeId,
        contents: &HistoryContents,
    ) -> Result<()> {
        if let Some(index) = self.dates.get(&identity) {
            ObjectWriter::reference(*index, output);
            return Ok(());
        }
        let record = date_record(contents, identity)?;
        let class = ArchiveClass {
            name: "moDateCodeHistory_c".into(),
            schema: record.schema,
        };
        let index = self.objects.object(&class, output)?;
        self.dates.insert(identity, index);
        write_codes(output, &record.codes);
        Ok(())
    }
}

pub(crate) fn write_stamp(output: &mut Vec<u8>, stamp: &RevisionStamp) -> Result<()> {
    for value in [stamp.revision_status, stamp.unknown_0c, stamp.unknown_10] {
        output.extend_from_slice(&value.to_le_bytes());
    }
    output.extend_from_slice(&stamp.time.to_le_bytes());
    stamp.comment.encode(output)
}

fn write_tail(output: &mut Vec<u8>, tail: &HistoryTail) {
    if let Some(memory) = &tail.memory {
        output.extend_from_slice(&memory.max_failure_severity.to_le_bytes());
        output.extend_from_slice(&memory.min_free_memory.to_le_bytes());
    }
    if let Some(value) = tail.unknown_30 {
        output.extend_from_slice(&value.to_le_bytes());
    }
    if let Some(recent) = &tail.recent {
        output.extend_from_slice(&recent.unknown_38.to_le_bytes());
        output.extend_from_slice(&recent.unknown_3c.to_le_bytes());
        output.extend_from_slice(&recent.unknown_40.to_le_bytes());
    }
}

fn date_record(contents: &HistoryContents, identity: DateCodeId) -> Result<&DateCodeHistory> {
    contents.date_histories.get(&identity).ok_or_else(|| {
        invalid(format!(
            "date-code identity {}; expected an object",
            identity.value()
        ))
    })
}

fn write_codes(output: &mut Vec<u8>, codes: &[u32]) {
    output.extend_from_slice(&(codes.len() as i32).to_le_bytes());
    for code in codes {
        output.extend_from_slice(&code.to_le_bytes());
    }
}
