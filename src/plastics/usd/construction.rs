use super::{Result, UsdArchive, UsdLimits, UsdStringRecord, invalid};

impl UsdArchive {
    /// Construct a complete stream from exact records, e.g. `UsdArchive::from_records(&records, limits)?`.
    pub fn from_records(records: &[UsdStringRecord], limits: UsdLimits) -> Result<Self> {
        let size = construction_size(records, limits)?;
        let mut bytes = Vec::new();
        bytes.try_reserve_exact(size).map_err(|error| invalid("constructed byte extent", format!("{size} bytes, allocation {error}; expected an available representable stream extent")))?;
        for record in records {
            bytes.extend_from_slice(&record.bytes[record.span.offset..record.span.end]);
        }
        Self::from_bytes(&bytes, limits)
    }
}

fn construction_size(records: &[UsdStringRecord], limits: UsdLimits) -> Result<usize> {
    if records.len() > limits.maximum_records {
        return Err(invalid(
            "constructed record count",
            format!(
                "{}; expected at most {}",
                records.len(),
                limits.maximum_records
            ),
        ));
    }
    records.iter().try_fold(0usize, |sum, record| {
        checked_record_size(sum, record, limits.maximum_payload_bytes)
    })
}

fn checked_record_size(sum: usize, record: &UsdStringRecord, maximum: usize) -> Result<usize> {
    if record.payload().len() > maximum {
        return Err(invalid(
            "constructed payload",
            format!(
                "{} bytes; expected at most {maximum}",
                record.payload().len()
            ),
        ));
    }
    let size = record.span.end - record.span.offset;
    sum.checked_add(size).ok_or_else(|| {
        invalid(
            "constructed byte extent",
            format!("{sum} plus {size} encoded bytes; expected representable stream extent"),
        )
    })
}
