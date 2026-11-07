use super::*;
use std::io::{BufWriter, Write};

#[test]
#[ignore = "requires explicit BESSEMER_INSPECTION_IDENTITY_EXPORT and native PDF source"]
fn complete_source_comparisons_export_deterministic_values_and_provenance() {
    let path = std::env::var_os("BESSEMER_INSPECTION_IDENTITY_EXPORT")
        .expect("expected explicit identity export path");
    let mut output = BufWriter::new(std::fs::File::create(path).unwrap());
    output.write_all(b"ISCR").unwrap();
    export_word(&mut output, 197);
    export_owned_comparisons(&mut output);
    export_metadata_comparisons(&mut output);
    export_native_comparisons(&mut output);
    output.flush().unwrap();
}

fn export_owned_comparisons(output: &mut impl Write) {
    for (index, case) in fixtures::checksum_cases().iter().enumerate() {
        let source = canonical_project(case.source.len(), &case.md5);
        export_comparison(output, &source, case.source, (0, index));
    }
}

fn export_metadata_comparisons(output: &mut impl Write) {
    for (index, case) in fixtures::metadata_cases().iter().enumerate() {
        let source = project(case.xml);
        export_comparison(output, &source, b"abc", (1, index));
    }
}

fn export_native_comparisons(output: &mut impl Write) {
    let (source, original) = native::tutorial_source();
    let mut modified = original.clone();
    let last = modified.len() - 1;
    modified[last] ^= 1;
    for (index, bytes) in [original.as_slice(), modified.as_slice(), &original[..last]]
        .into_iter()
        .enumerate()
    {
        export_comparison(output, &source, bytes, (2, index));
    }
}

fn export_comparison(
    output: &mut impl Write,
    source: &InspectionRecords,
    bytes: &[u8],
    identity: (usize, usize),
) {
    let checked = source
        .compare_document_source(&document(source).id, bytes)
        .unwrap();
    check_comparison_contract(source, &checked, bytes.len());
    export_word(output, identity.0);
    export_word(output, identity.1);
    export_bytes(output, checked.source.envelope().xml_bytes());
    export_identity_values(output, &checked.values);
    export_result_contract(output, &checked);
}

fn export_identity_values(output: &mut impl Write, values: &InspectionSourceIdentityValues) {
    export_optional_text(output, values.stored_size.as_deref());
    export_u64(output, values.actual_size);
    export_optional_text(output, values.stored_md5.as_deref());
    export_bytes(output, values.actual_md5.as_bytes());
    output
        .write_all(&[u8::from(values.size_matches), u8::from(values.md5_matches)])
        .unwrap();
}

fn export_result_contract(output: &mut impl Write, result: &InspectionSourceComparison) {
    let completion = match result.completion {
        InspectionSourceCompletion::Completed => 0,
    };
    let mode = match result.coverage.mode {
        InspectionSourceCheckMode::ExactCallerBytesAndStoredText => 0,
    };
    output
        .write_all(&[completion, mode, diagnostic_bits(&result.diagnostics)])
        .unwrap();
    export_u64(output, result.coverage.source_bytes);
    export_u64(output, result.coverage.unread_source_bytes);
    export_word(output, result.history.len());
    let history = &result.history[0];
    export_word(output, history.parents.len());
    let copied = u8::from(history.relation == InspectionRecordRelation::Copied);
    let identity =
        u8::from(history.result == result.document && history.parents[0] == result.document);
    output.write_all(&[copied, identity]).unwrap();
}

fn diagnostic_bits(diagnostics: &[InspectionSourceDiagnostic]) -> u8 {
    diagnostics.iter().fold(0, |bits, diagnostic| {
        bits | match diagnostic {
            InspectionSourceDiagnostic::ChecksumDoesNotAuthenticate => 1,
            InspectionSourceDiagnostic::StoredPathsNotOpened => 2,
            InspectionSourceDiagnostic::ApplicationCompatibilityNotChecked => 4,
        }
    })
}

fn export_optional_text(output: &mut impl Write, value: Option<&str>) {
    match value {
        Some(text) => export_bytes(output, text.as_bytes()),
        None => output.write_all(&u32::MAX.to_le_bytes()).unwrap(),
    }
}

fn export_bytes(output: &mut impl Write, bytes: &[u8]) {
    export_word(output, bytes.len());
    output.write_all(bytes).unwrap();
}

fn export_word(output: &mut impl Write, value: usize) {
    output
        .write_all(&u32::try_from(value).unwrap().to_le_bytes())
        .unwrap();
}

fn export_u64(output: &mut impl Write, value: usize) {
    output
        .write_all(&u64::try_from(value).unwrap().to_le_bytes())
        .unwrap();
}
