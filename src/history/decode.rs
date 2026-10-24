use std::collections::BTreeMap;

use crate::archive_codec::{ArchiveClass, ArchiveCursor, ObjectReader, ObjectRecord};
use crate::{BessemerError, FailureClass, Result};

use super::{
    ArchiveString, DateCodeHistory, DateCodeId, HistoryContents, HistoryMemory, HistoryRecent,
    HistoryTail, RevisionStamp, StateDateCodes, VersionState, invalid,
};

struct HistoryReader<'a> {
    cursor: ArchiveCursor<'a>,
    objects: ObjectReader,
    dates: BTreeMap<DateCodeId, DateCodeHistory>,
}

pub(super) fn history(bytes: &[u8]) -> Result<(u16, HistoryContents)> {
    let mut reader = HistoryReader::new(bytes);
    let schema = reader.root()?;
    let states = reader.states()?;
    Ok((schema, reader.finish_contents(states)?))
}

impl<'a> HistoryReader<'a> {
    fn new(bytes: &'a [u8]) -> Self {
        Self {
            cursor: ArchiveCursor::new(bytes),
            objects: ObjectReader::new(),
            dates: BTreeMap::new(),
        }
    }

    fn states(&mut self) -> Result<Vec<VersionState>> {
        let count = self.cursor.count(15, "version state")?;
        let mut states = Vec::new();
        for index in 0..count {
            states.push(self.state().map_err(|error| state_context(error, index))?);
        }
        Ok(states)
    }

    fn finish_contents(mut self, states: Vec<VersionState>) -> Result<HistoryContents> {
        let last = states
            .last()
            .ok_or_else(|| invalid("state count 0; expected at least one state"))?;
        let tail = read_tail(&mut self.cursor, last.file_version)?;
        self.cursor.finish()?;
        Ok(HistoryContents {
            states,
            date_histories: self.dates,
            tail,
        })
    }

    fn root(&mut self) -> Result<u16> {
        let record = self.objects.object(&mut self.cursor)?;
        let ObjectRecord::New { class, .. } = record else {
            return Err(invalid(
                "root object is null or a reference; expected new moVersionHistory_c",
            ));
        };
        expect_class(&class, "moVersionHistory_c")?;
        Ok(class.schema)
    }

    fn state(&mut self) -> Result<VersionState> {
        let stamp = read_stamp(&mut self.cursor)?;
        let file_version = self.cursor.u32()?;
        let date_codes = match file_version {
            0..0x175 => StateDateCodes::Absent,
            0x175..0x178 => StateDateCodes::Legacy(self.cursor.u32()?),
            _ => StateDateCodes::History(self.date_object()?),
        };
        Ok(VersionState {
            stamp,
            file_version,
            date_codes,
        })
    }

    fn date_object(&mut self) -> Result<DateCodeId> {
        match self.objects.object(&mut self.cursor)? {
            ObjectRecord::Null => Err(invalid(
                "date-history pointer 0; expected nonnull moDateCodeHistory_c",
            )),
            ObjectRecord::Reference { index, class } => {
                expect_class(&class, "moDateCodeHistory_c")?;
                Ok(DateCodeId(index))
            }
            ObjectRecord::New { index, class } => {
                expect_class(&class, "moDateCodeHistory_c")?;
                let identity = DateCodeId(index);
                self.dates
                    .insert(identity, read_date_history(&mut self.cursor, class.schema)?);
                Ok(identity)
            }
        }
    }
}

pub(crate) fn read_stamp(cursor: &mut ArchiveCursor<'_>) -> Result<RevisionStamp> {
    Ok(RevisionStamp {
        revision_status: cursor.u16()?,
        unknown_0c: cursor.u16()?,
        unknown_10: cursor.u16()?,
        time: cursor.u32()?,
        comment: ArchiveString::decode(cursor)?,
    })
}

fn read_date_history(cursor: &mut ArchiveCursor<'_>, schema: u16) -> Result<DateCodeHistory> {
    let count = cursor.count(4, "date code")?;
    let codes: Result<Vec<u32>> = (0..count).map(|_| cursor.u32()).collect();
    Ok(DateCodeHistory {
        schema,
        codes: codes?,
    })
}

fn read_tail(cursor: &mut ArchiveCursor<'_>, version: u32) -> Result<HistoryTail> {
    let memory = (version >= 0x319)
        .then(|| read_memory(cursor))
        .transpose()?;
    let unknown_30 = (version >= 0x45c).then(|| cursor.i32()).transpose()?;
    let recent = (version >= 0x46c)
        .then(|| read_recent(cursor))
        .transpose()?;
    Ok(HistoryTail {
        memory,
        unknown_30,
        recent,
    })
}

fn read_memory(cursor: &mut ArchiveCursor<'_>) -> Result<HistoryMemory> {
    Ok(HistoryMemory {
        max_failure_severity: cursor.i32()?,
        min_free_memory: cursor.u32()?,
    })
}

fn read_recent(cursor: &mut ArchiveCursor<'_>) -> Result<HistoryRecent> {
    Ok(HistoryRecent {
        unknown_38: cursor.u16()?,
        unknown_3c: cursor.i32()?,
        unknown_40: cursor.i32()?,
    })
}

fn expect_class(class: &ArchiveClass, expected: &str) -> Result<()> {
    if class.name == expected {
        return Ok(());
    }
    Err(BessemerError::new(
        "decode history",
        "object class",
        FailureClass::Unsupported,
        format!(
            "class {:?}, schema {}; expected {expected}",
            class.name, class.schema
        ),
    ))
}

fn state_context(mut error: BessemerError, index: usize) -> BessemerError {
    error.operation = "decode history";
    error.entity = format!("History state {index}: {}", error.entity);
    error
}
