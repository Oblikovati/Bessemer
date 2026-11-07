use super::{
    Result, UsdArchive, UsdCharacterWidth, UsdCheckMode, UsdRecordId, UsdRevision, invalid,
};
use std::sync::Arc;

const SEPARATOR: &str = "------------------------------------------";
const PROFILE_LABELS: [&str; 12] = [
    "Win32_BaseBoard SerialNumber",
    "Win32_BaseBoard Product",
    "Win32_BIOS SerialNumber",
    "Win32_PhysicalMedia SerialNumber",
    "Operating System",
    "Update JanusLock Key Entry",
    "Product Name",
    "Memory Status",
    "CPU Status",
    "Virtual Machine or not",
    "Graphic Card",
    "CPU Process Type",
];

/// One ordered labeled profile value, retained as an exact counted record.
#[derive(Debug, Clone)]
pub struct UsdProfileRecord {
    /// Exact fixed label from the documented writer grammar.
    pub label: &'static str,
    /// Complete value identity, without parsing or trimming its content.
    pub record: UsdRecordId,
}

/// Explicit limits of a successful writer-grammar projection.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UsdWriterDiagnostic {
    /// Host, adapter, customer and system values retain unvalidated literal content.
    ValuesNotSemanticallyValidated,
    /// No native loader, source code-page conversion or application acceptance check ran.
    NativeLoaderNotExecuted,
}

/// Exact Unicode writer-grammar coverage, separate from native loader success.
#[derive(Debug, Clone, Copy)]
pub struct UsdWriterCoverage {
    /// Every record's Unicode width and every fixed literal were checked.
    pub check_mode: UsdCheckMode,
    /// All framed records, including terminal #EOF.
    pub checked_records: usize,
    /// All encoded source bytes.
    pub checked_bytes: usize,
    /// Every required separator, label, empty record and final marker.
    pub checked_literal_records: usize,
    /// Native/runtime and value-domain limits.
    pub diagnostics: [UsdWriterDiagnostic; 2],
}

/// Complete explicit writer layout, with every projected value tied to its source record.
#[derive(Debug, Clone)]
pub struct UsdWriterLayout {
    /// Immutable complete source including all original count prefixes and values.
    pub source: UsdArchive,
    /// Source identity shared by every projected record.
    pub parent: UsdRevision,
    /// Host-name record, without stripping counted NUL or suffix units.
    pub host: UsdRecordId,
    /// Stored volume serial text, without numeric conversion.
    pub volume_serial: UsdRecordId,
    /// Ordered ten-record adapter groups, including their fixed records.
    pub adapters: Arc<[[UsdRecordId; 10]]>,
    /// Ordered five customer value records.
    pub customer: [UsdRecordId; 5],
    /// Ordered twelve labeled profile values.
    pub profile: Arc<[UsdProfileRecord]>,
    /// Exact grammar scope and native/value exclusions.
    pub coverage: UsdWriterCoverage,
}

impl UsdArchive {
    /// Check the explicit Unicode writer grammar, e.g. `archive.writer_layout()?`; framing alone does not imply it.
    pub fn writer_layout(&self) -> Result<UsdWriterLayout> {
        let adapters = adapter_count(self)?;
        require_unicode(self)?;
        validate_leading(self)?;
        validate_adapters(self, adapters)?;
        validate_customer(self, 6 + adapters * 10)?;
        validate_profile(self, 15 + adapters * 10)?;
        literal(self, self.spans.len() - 1, "#EOF")?;
        layout_result(self, adapters)
    }
}

fn adapter_count(source: &UsdArchive) -> Result<usize> {
    let count = source.spans.len();
    if count < 75 || !(count - 75).is_multiple_of(10) {
        return Err(invalid(
            "writer record count",
            format!("{count}; expected 75 + 10 * nonnegative adapter count"),
        ));
    }
    Ok((count - 75) / 10)
}

fn require_unicode(source: &UsdArchive) -> Result<()> {
    for (index, span) in source.spans.iter().enumerate() {
        if span.width != UsdCharacterWidth::Utf16CodeUnits {
            return Err(invalid(
                "writer record width",
                format!(
                    "record {index} has {:?}; expected Unicode for this explicit writer grammar",
                    span.width
                ),
            ));
        }
    }
    Ok(())
}

fn literal(source: &UsdArchive, index: usize, expected: &str) -> Result<()> {
    let record = source.record(&source.record_id(index)?)?;
    let actual = record
        .payload()
        .as_chunks::<2>()
        .0
        .iter()
        .map(|pair| u16::from_le_bytes(*pair));
    if !actual.eq(expected.encode_utf16()) {
        return Err(invalid(
            "writer literal",
            format!(
                "record {index} payload {:?}; expected exact UTF-16 units for {expected:?}",
                record.payload()
            ),
        ));
    }
    Ok(())
}

fn validate_leading(source: &UsdArchive) -> Result<()> {
    for index in [0, 2, 3, 5] {
        literal(source, index, SEPARATOR)?;
    }
    Ok(())
}

fn validate_adapters(source: &UsdArchive, adapters: usize) -> Result<()> {
    for number in 0..adapters {
        let start = 6 + number * 10;
        literal(source, start, SEPARATOR)?;
        literal(source, start + 2, SEPARATOR)?;
        literal(source, start + 9, "")?;
    }
    Ok(())
}

fn validate_customer(source: &UsdArchive, start: usize) -> Result<()> {
    literal(source, start, SEPARATOR)?;
    literal(source, start + 1, "The Customer Profile")?;
    literal(source, start + 2, SEPARATOR)?;
    literal(source, start + 8, "")?;
    Ok(())
}

fn validate_profile(source: &UsdArchive, start: usize) -> Result<()> {
    for (number, label) in PROFILE_LABELS.iter().enumerate() {
        let index = start + number * 5;
        literal(source, index, SEPARATOR)?;
        literal(source, index + 1, label)?;
        literal(source, index + 2, SEPARATOR)?;
        if number < 11 {
            literal(source, index + 4, "")?;
        }
    }
    Ok(())
}

fn record_group<const N: usize>(source: &UsdArchive, start: usize) -> Result<[UsdRecordId; N]> {
    let end = start.checked_add(N).ok_or_else(|| {
        invalid(
            "writer record group",
            format!("start {start}, count {N}; expected representable ordinal extent"),
        )
    })?;
    if source.spans.get(start..end).is_none() {
        return Err(invalid(
            "writer record group",
            format!(
                "{start}..{end}; expected within {} records",
                source.spans.len()
            ),
        ));
    }
    Ok(std::array::from_fn(|offset| UsdRecordId {
        revision: source.revision.clone(),
        index: start + offset,
    }))
}

fn layout_result(source: &UsdArchive, count: usize) -> Result<UsdWriterLayout> {
    let adapters = adapter_groups(source, count)?;
    let profile = profile_records(source, count)?;
    Ok(UsdWriterLayout {
        source: source.clone(),
        parent: source.revision.clone(),
        host: source.record_id(1)?,
        volume_serial: source.record_id(4)?,
        adapters: adapters.into(),
        customer: record_group::<5>(source, 9 + count * 10)?,
        profile: profile.into(),
        coverage: writer_coverage(source, count),
    })
}

fn writer_coverage(source: &UsdArchive, adapters: usize) -> UsdWriterCoverage {
    UsdWriterCoverage {
        check_mode: UsdCheckMode::ExactUnicodeWriterGrammarAndLiterals,
        checked_records: source.spans.len(),
        checked_bytes: source.bytes.len(),
        checked_literal_records: 56 + adapters * 3,
        diagnostics: [
            UsdWriterDiagnostic::ValuesNotSemanticallyValidated,
            UsdWriterDiagnostic::NativeLoaderNotExecuted,
        ],
    }
}

fn adapter_groups(source: &UsdArchive, count: usize) -> Result<Vec<[UsdRecordId; 10]>> {
    (0..count)
        .map(|index| record_group::<10>(source, 6 + index * 10))
        .collect()
}

fn profile_records(source: &UsdArchive, count: usize) -> Result<Vec<UsdProfileRecord>> {
    PROFILE_LABELS
        .iter()
        .enumerate()
        .map(|(index, label)| {
            Ok(UsdProfileRecord {
                label,
                record: source.record_id(18 + count * 10 + index * 5)?,
            })
        })
        .collect()
}
