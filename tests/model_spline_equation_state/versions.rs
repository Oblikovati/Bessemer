use super::*;

const VERSION_LIMITS: ModelArchiveLimits = ModelArchiveLimits {
    maximum_bytes: 1_000_000,
    maximum_fields: 100_000,
    maximum_entries: 100_000,
    maximum_depth: 128,
    maximum_objects: 10_000,
};

struct EquationVersionCase {
    version: u32,
    inherited: usize,
    document: &'static [u8],
    original: [[&'static [u8]; 8]; 2],
    edited: [&'static [u8]; 2],
}

const VERSION_CASES: [EquationVersionCase; 2] = [
    EquationVersionCase {
        version: 11000,
        inherited: 413,
        document: include_bytes!(
            "../fixtures/spline-equation-state/versions/11000/document.SLDPRT"
        ),
        original: [
            [
                include_bytes!("../fixtures/spline-equation-state/versions/11000/owned-0-0.bin"),
                include_bytes!("../fixtures/spline-equation-state/versions/11000/owned-0-1.bin"),
                include_bytes!("../fixtures/spline-equation-state/versions/11000/owned-0-2.bin"),
                include_bytes!("../fixtures/spline-equation-state/versions/11000/owned-0-3.bin"),
                include_bytes!("../fixtures/spline-equation-state/versions/11000/owned-0-4.bin"),
                include_bytes!("../fixtures/spline-equation-state/versions/11000/owned-0-5.bin"),
                include_bytes!("../fixtures/spline-equation-state/versions/11000/owned-0-6.bin"),
                include_bytes!("../fixtures/spline-equation-state/versions/11000/owned-0-7.bin"),
            ],
            [
                include_bytes!("../fixtures/spline-equation-state/versions/11000/owned-1-0.bin"),
                include_bytes!("../fixtures/spline-equation-state/versions/11000/owned-1-1.bin"),
                include_bytes!("../fixtures/spline-equation-state/versions/11000/owned-1-2.bin"),
                include_bytes!("../fixtures/spline-equation-state/versions/11000/owned-1-3.bin"),
                include_bytes!("../fixtures/spline-equation-state/versions/11000/owned-1-4.bin"),
                include_bytes!("../fixtures/spline-equation-state/versions/11000/owned-1-5.bin"),
                include_bytes!("../fixtures/spline-equation-state/versions/11000/owned-1-6.bin"),
                include_bytes!("../fixtures/spline-equation-state/versions/11000/owned-1-7.bin"),
            ],
        ],
        edited: [
            include_bytes!("../fixtures/spline-equation-state/versions/11000/owned-0-0-edited.bin"),
            include_bytes!("../fixtures/spline-equation-state/versions/11000/owned-1-0-edited.bin"),
        ],
    },
    EquationVersionCase {
        version: 18000,
        inherited: 106,
        document: include_bytes!(
            "../fixtures/spline-equation-state/versions/18000/document.SLDPRT"
        ),
        original: [
            [
                include_bytes!("../fixtures/spline-equation-state/versions/18000/owned-0-0.bin"),
                include_bytes!("../fixtures/spline-equation-state/versions/18000/owned-0-1.bin"),
                include_bytes!("../fixtures/spline-equation-state/versions/18000/owned-0-2.bin"),
                include_bytes!("../fixtures/spline-equation-state/versions/18000/owned-0-3.bin"),
                include_bytes!("../fixtures/spline-equation-state/versions/18000/owned-0-4.bin"),
                include_bytes!("../fixtures/spline-equation-state/versions/18000/owned-0-5.bin"),
                include_bytes!("../fixtures/spline-equation-state/versions/18000/owned-0-6.bin"),
                include_bytes!("../fixtures/spline-equation-state/versions/18000/owned-0-7.bin"),
            ],
            [
                include_bytes!("../fixtures/spline-equation-state/versions/18000/owned-1-0.bin"),
                include_bytes!("../fixtures/spline-equation-state/versions/18000/owned-1-1.bin"),
                include_bytes!("../fixtures/spline-equation-state/versions/18000/owned-1-2.bin"),
                include_bytes!("../fixtures/spline-equation-state/versions/18000/owned-1-3.bin"),
                include_bytes!("../fixtures/spline-equation-state/versions/18000/owned-1-4.bin"),
                include_bytes!("../fixtures/spline-equation-state/versions/18000/owned-1-5.bin"),
                include_bytes!("../fixtures/spline-equation-state/versions/18000/owned-1-6.bin"),
                include_bytes!("../fixtures/spline-equation-state/versions/18000/owned-1-7.bin"),
            ],
        ],
        edited: [
            include_bytes!("../fixtures/spline-equation-state/versions/18000/owned-0-0-edited.bin"),
            include_bytes!("../fixtures/spline-equation-state/versions/18000/owned-1-0-edited.bin"),
        ],
    },
];

fn version_document(case: &EquationVersionCase, raw: &[u8]) -> Document {
    let mut document = Document::from_bytes(case.document).unwrap();
    replace_owned_resolved_stream(&mut document, raw.to_vec());
    Document::from_bytes(&document.to_bytes().unwrap()).unwrap()
}

#[test]
fn version19000_feature_images_are_rejected_in_foreign_saved_contexts_without_mutation() {
    for case in &VERSION_CASES {
        let document = version_document(case, OWNED_EQUATIONS[0][0]);
        let original = document.to_bytes().unwrap();
        let result = PartResolvedFeaturesArchive::from_document(
            &document,
            ConfigurationId::new(0),
            bessemer::header::ArchiveVersion::new(case.version),
            None,
            VERSION_LIMITS,
        );
        let error = result.unwrap_err();
        assert_eq!(error.class, bessemer::FailureClass::Invalid);
        assert_eq!(error.operation, "part resolved features");
        assert!(error.cause.contains("expected"));
        assert_eq!(document.to_bytes().unwrap(), original);
    }
}

#[test]
fn both_legacy_contexts_project_all_stored_roles_and_complete_empty_equations() {
    for case in &VERSION_CASES {
        for (kind, style) in (0..2).flat_map(|kind| (0..8).map(move |style| (kind, style))) {
            assert_version_projection(case, kind, style);
        }
    }
}

fn assert_version_projection(case: &EquationVersionCase, kind: usize, style: usize) {
    let raw = case.original[kind][style];
    let document = version_document(case, raw);
    let source = read_equation_at_version(&document, case.version, VERSION_LIMITS);
    assert_eq!(source.to_bytes().unwrap(), raw);
    assert_eq!(source.object_slots(), (case.inherited, case.inherited + 2));
    let states = source.spline_equation_states().unwrap();
    assert_eq!(states.len(), usize::from(style < 6));
    if let Some(state) = states.first() {
        assert_eq!(state.flags_word(), EQUATION_FLAGS[style]);
        assert_eq!(state.saved_option(), EQUATION_OPTIONS[style]);
        assert_eq!(state.compatibility_word(), i32::MIN);
        assert_version_texts(state);
    }
}

fn assert_version_texts(state: &ModelSplineEquationStoredState) {
    let roles = [
        ModelSplineEquationTextRole::XExpression,
        ModelSplineEquationTextRole::YExpression,
        ModelSplineEquationTextRole::ZExpression,
        ModelSplineEquationTextRole::DeclaredCharacters,
        ModelSplineEquationTextRole::RangeStart,
        ModelSplineEquationTextRole::RangeEnd,
    ];
    for (index, role) in roles.into_iter().enumerate() {
        assert_eq!(state.text(role).text(), format!("\u{3a9} equation {index}"));
    }
}

#[test]
fn both_legacy_contexts_edit_reopen_and_reverse_both_spline_prototypes() {
    for case in &VERSION_CASES {
        for kind in 0..2 {
            assert_version_lock_operation(case, kind);
        }
    }
}

fn assert_version_lock_operation(case: &EquationVersionCase, kind: usize) {
    let document = version_document(case, case.original[kind][0]);
    let source = read_equation_at_version(&document, case.version, VERSION_LIMITS);
    let state = source.spline_equation_states().unwrap().remove(0);
    let result = source
        .edit_spline_equation(&state, ModelSplineEquationStoredEdit::LockStart(true))
        .unwrap();
    assert_eq!(result.archive().to_bytes().unwrap(), case.edited[kind]);
    assert_eq!(result.completion(), ModelEditCompletion::Changed);
    assert_eq!(result.changes().len(), 1);
    assert_eq!(result.changes()[0].parents(), &state.source_fields()[..1]);
    assert_eq!(result.unchanged_fields(), source.fields().len() - 1);
    assert_reopened_equation_at_version(&document, &source, &result, case.version, VERSION_LIMITS);
    assert_eq!(source.to_bytes().unwrap(), case.original[kind][0]);
}
