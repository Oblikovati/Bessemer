use super::annotation_view_tests::LIMITS;
use crate::header::ArchiveVersion;
use crate::model_archives::reader::{ModelArchiveContext, ModelDecoder};

#[test]
fn simulation_embedded_context_never_satisfies_ordinary_context_guards() {
    for version in [4281, 4307, 6000, 14000, 19000] {
        let mut reader = ModelDecoder::new_in_context(
            &[],
            ArchiveVersion::new(version),
            LIMITS,
            ModelArchiveContext::SimulationEmbedded,
        )
        .unwrap();
        assert!(reader.is_simulation_embedded());
        let error = reader.require_ordinary_model().unwrap_err();
        assert_eq!(error.class, crate::FailureClass::Unsupported);
        assert!(error.cause.contains("SimulationEmbedded"));
    }
}

#[test]
fn ordinary_contexts_never_select_the_simulation_embedded_registry() {
    let reader = ModelDecoder::new_in_context(
        &[],
        ArchiveVersion::new(19000),
        LIMITS,
        ModelArchiveContext::OrdinaryPartExternalXml,
    )
    .unwrap();
    assert!(!reader.is_simulation_embedded());
}

#[test]
fn embedded_prefix_completion_rejects_an_ordinary_context() {
    let reader = ModelDecoder::new_in_context(
        &[],
        ArchiveVersion::new(19000),
        LIMITS,
        ModelArchiveContext::OrdinaryPartExternalXml,
    )
    .unwrap();
    let error = reader.finish_embedded_prefix().unwrap_err();
    assert!(error.cause.contains("OrdinaryPartExternalXml"));
    assert!(
        error
            .cause
            .contains("expected explicit Simulation embedded prefix context")
    );
}
