use std::collections::BTreeMap;

use bessemer::history::{
    DateCodeId, HistoryContents, HistoryTail, StateDateCodes, VersionHistory, VersionState,
};

pub const NATIVE: &[u8] = include_bytes!("../fixtures/history/blank-part.bin");
pub const MIXED: &[u8] = include_bytes!("../fixtures/history/mixed.bin");
pub const ANCIENT: &[u8] = include_bytes!("../fixtures/history/threshold-0026.bin");

pub const CASES: &[(&str, &[u8])] = &[
    ("blank-part.bin", NATIVE),
    (
        "blank-assembly.bin",
        include_bytes!("../fixtures/history/blank-assembly.bin"),
    ),
    (
        "blank-drawing.bin",
        include_bytes!("../fixtures/history/blank-drawing.bin"),
    ),
    ("mixed.bin", MIXED),
    ("threshold-0026.bin", ANCIENT),
    (
        "threshold-0174.bin",
        include_bytes!("../fixtures/history/threshold-0174.bin"),
    ),
    (
        "threshold-0175.bin",
        include_bytes!("../fixtures/history/threshold-0175.bin"),
    ),
    (
        "threshold-0177.bin",
        include_bytes!("../fixtures/history/threshold-0177.bin"),
    ),
    (
        "threshold-0178.bin",
        include_bytes!("../fixtures/history/threshold-0178.bin"),
    ),
    (
        "threshold-0318.bin",
        include_bytes!("../fixtures/history/threshold-0318.bin"),
    ),
    (
        "threshold-0319.bin",
        include_bytes!("../fixtures/history/threshold-0319.bin"),
    ),
    (
        "threshold-045b.bin",
        include_bytes!("../fixtures/history/threshold-045b.bin"),
    ),
    (
        "threshold-045c.bin",
        include_bytes!("../fixtures/history/threshold-045c.bin"),
    ),
    (
        "threshold-046b.bin",
        include_bytes!("../fixtures/history/threshold-046b.bin"),
    ),
    (
        "threshold-046c.bin",
        include_bytes!("../fixtures/history/threshold-046c.bin"),
    ),
];

pub fn semantic_projection(history: &VersionHistory) -> String {
    let contents = history.contents();
    let mut parts = vec!["root".into(), history.root_schema().to_string()];
    let mut identities = BTreeMap::new();
    for state in &contents.states {
        append_state(&mut parts, state);
        append_dates(&mut parts, state.date_codes, contents, &mut identities);
    }
    append_tail(&mut parts, &contents.tail);
    parts.join("|")
}

fn append_state(parts: &mut Vec<String>, state: &VersionState) {
    let stamp = &state.stamp;
    parts.extend([
        "state".into(),
        stamp.revision_status.to_string(),
        stamp.unknown_0c.to_string(),
        stamp.unknown_10.to_string(),
        stamp.time.to_string(),
        hex_text(stamp.comment.text()),
        state.file_version.to_string(),
    ]);
}

fn append_dates(
    parts: &mut Vec<String>,
    dates: StateDateCodes,
    contents: &HistoryContents,
    identities: &mut BTreeMap<DateCodeId, usize>,
) {
    match dates {
        StateDateCodes::Absent => parts.push("absent".into()),
        StateDateCodes::Legacy(value) => parts.extend(["legacy".into(), value.to_string()]),
        StateDateCodes::History(identity) => {
            let next = identities.len();
            let ordinal = *identities.entry(identity).or_insert(next);
            let record = &contents.date_histories[&identity];
            let codes = record
                .codes
                .iter()
                .map(u32::to_string)
                .collect::<Vec<_>>()
                .join(",");
            parts.extend([
                "dates".into(),
                ordinal.to_string(),
                record.schema.to_string(),
                codes,
            ]);
        }
    }
}

fn append_tail(parts: &mut Vec<String>, tail: &HistoryTail) {
    let memory = tail.memory.as_ref().map_or_else(
        || "-".into(),
        |value| format!("{},{}", value.max_failure_severity, value.min_free_memory),
    );
    let unknown = tail
        .unknown_30
        .map_or_else(|| "-".into(), |value| value.to_string());
    let recent = tail.recent.as_ref().map_or_else(
        || "-".into(),
        |value| {
            format!(
                "{},{},{}",
                value.unknown_38, value.unknown_3c, value.unknown_40
            )
        },
    );
    parts.extend(["tail".into(), memory, unknown, recent]);
}

fn hex_text(text: &str) -> String {
    text.as_bytes()
        .iter()
        .map(|byte| format!("{byte:02x}"))
        .collect()
}
