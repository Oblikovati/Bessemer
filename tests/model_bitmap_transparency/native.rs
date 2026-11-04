use super::*;
use bessemer::header::DocumentHeader;
use std::sync::Arc;
#[path = "native_documents.rs"]
mod native_documents;
#[path = "native_packet.rs"]
mod native_packet;
use native_packet::{NativeBitmapCase, native_bitmap_cases};
#[path = "../support/native_kernel_context.rs"]
mod native_kernel_context;

const NATIVE_LIMITS: ModelArchiveLimits = ModelArchiveLimits {
    maximum_bytes: 64 * 1024 * 1024,
    maximum_fields: 2_000_000,
    maximum_entries: 2_000_000,
    maximum_depth: 128,
    maximum_objects: 100_000,
};

fn native_bitmap_kernel() -> CompressedKernelContext {
    native_kernel_context::native_test_kernel_context()
}

fn native_bitmap_id(case: &NativeBitmapCase<'_>) -> ConfigurationId {
    ConfigurationId::from_stream_name(case.label.rsplit_once("::").unwrap().1).unwrap()
}

fn native_bitmap_configuration(
    case: &NativeBitmapCase<'_>,
    kernel: &CompressedKernelContext,
) -> PartConfigurationArchive {
    let header = Arc::new(
        DocumentHeader::from_bytes(case.header, case.version, NATIVE_LIMITS.maximum_objects)
            .unwrap(),
    );
    let context = PartConfigurationContext::ordinary_external_xml(
        native_bitmap_id(case),
        case.version,
        header,
        Some(kernel.clone()),
    )
    .unwrap();
    PartConfigurationArchive::from_bytes(case.configuration, context, NATIVE_LIMITS).unwrap()
}

fn native_bitmap_archive(
    case: &NativeBitmapCase<'_>,
    kernel: &CompressedKernelContext,
) -> PartResolvedFeaturesArchive {
    let configuration = native_bitmap_configuration(case, kernel);
    let archive = PartResolvedFeaturesArchive::from_bytes(
        case.features,
        configuration,
        case.manager,
        NATIVE_LIMITS,
    )
    .unwrap();
    assert_eq!(archive.to_bytes().unwrap(), case.features);
    archive
}

fn native_bitmap_state(source: &PartResolvedFeaturesArchive) -> ModelBitmapTransparencyState {
    let mut states = source.bitmap_transparency_states().unwrap();
    assert_eq!(states.len(), 1);
    let state = states.remove(0);
    assert_eq!(
        state.words(),
        ModelBitmapTransparencyWords {
            has_transparency_setting: 0,
            level_parameter: 0,
            type_selector: 0
        }
    );
    state
}

fn verify_native_bitmap_case(case: &NativeBitmapCase<'_>, kernel: &CompressedKernelContext) {
    let source = native_bitmap_archive(case, kernel);
    let state = native_bitmap_state(&source);
    for role in 0..3 {
        for value in DOMAIN {
            verify_bitmap_edit(
                &source,
                case.features,
                &state,
                role,
                value,
                (case.offsets[role], 0),
            );
        }
    }
}

#[test]
#[ignore = "Requires retained906-case native BRF1 corpus and explicit BESSEMER_KERNEL_SCHEMA"]
fn all_native_bitmap_typed_settings_match_independent_spans_and_literal_edits() {
    let packet = std::fs::read("target/resolved-native-probe.bin").unwrap();
    let kernel = native_bitmap_kernel();
    for case in native_bitmap_cases(&packet) {
        verify_native_bitmap_case(&case, &kernel);
        println!("BITMAP_NATIVE_TYPED\t{}\t27", case.index);
    }
}
