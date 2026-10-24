use std::collections::BTreeSet;

use super::{DateCodeHistory, DateCodeId, HistoryContents, StateDateCodes, VersionState, invalid};
use crate::Result;

pub(super) fn contents(contents: &HistoryContents) -> Result<()> {
    validate_count(contents.states.len(), "version state")?;
    let mut referenced = BTreeSet::new();
    for (index, state) in contents.states.iter().enumerate() {
        validate_state(state, index)?;
        if let StateDateCodes::History(identity) = state.date_codes {
            referenced.insert(identity);
        }
    }
    validate_references(contents, &referenced)?;
    validate_tail(contents)
}

fn validate_state(state: &VersionState, index: usize) -> Result<()> {
    let valid = matches!(
        (state.file_version, state.date_codes),
        (0..0x175, StateDateCodes::Absent)
            | (0x175..0x178, StateDateCodes::Legacy(_))
            | (0x178.., StateDateCodes::History(_))
    );
    if valid {
        return Ok(());
    }
    Err(invalid(format!(
        "state {index} version {}, dates {:?}; expected absent below 0x175, legacy before 0x178, object thereafter",
        state.file_version, state.date_codes
    )))
}

fn validate_references(
    contents: &HistoryContents,
    referenced: &BTreeSet<super::DateCodeId>,
) -> Result<()> {
    for identity in referenced {
        if !contents.date_histories.contains_key(identity) {
            return Err(invalid(format!(
                "date-code identity {}; expected a date-history object",
                identity.value()
            )));
        }
    }
    for (identity, history) in &contents.date_histories {
        validate_date_record(*identity, history, referenced)?;
    }
    Ok(())
}

fn validate_tail(contents: &HistoryContents) -> Result<()> {
    let Some(last) = contents.states.last() else {
        return Err(invalid("state count 0; expected nonempty history"));
    };
    let version = last.file_version;
    let actual = (
        contents.tail.memory.is_some(),
        contents.tail.unknown_30.is_some(),
        contents.tail.recent.is_some(),
    );
    let expected = (version >= 0x319, version >= 0x45c, version >= 0x46c);
    if actual == expected {
        return Ok(());
    }
    Err(invalid(format!(
        "last version {version}, tail presence {actual:?}; expected {expected:?} for memory/unknown_30/recent"
    )))
}

fn validate_count(count: usize, entity: &str) -> Result<()> {
    if (1..=i32::MAX as usize).contains(&count) {
        return Ok(());
    }
    Err(invalid(format!(
        "{entity} count {count}; expected 1..=i32::MAX"
    )))
}

fn validate_date_record(
    identity: DateCodeId,
    history: &DateCodeHistory,
    referenced: &BTreeSet<DateCodeId>,
) -> Result<()> {
    if !referenced.contains(&identity) {
        return Err(invalid(format!(
            "date-code identity {}; expected at least one referring state",
            identity.value()
        )));
    }
    validate_count(history.codes.len(), "date code")
}
