use super::admission::{intersection_candidate, reference_scope};
use super::projection::project_intersection_reference;
use super::{ModelIntersectionReferenceStoredEdit as Edit, ModelIntersectionReferenceStoredWords};
use crate::archive_codec::ObjectRecord;
use crate::header::ArchiveVersion;
use crate::model_archives::*;
use crate::{Document, FailureClass};

const LIMITS: ModelArchiveLimits = ModelArchiveLimits {
    maximum_bytes: 30000,
    maximum_fields: 10000,
    maximum_entries: 10000,
    maximum_depth: 64,
    maximum_objects: 256,
};

pub(super) fn reference_archive() -> PartResolvedFeaturesArchive {
    let document = Document::from_bytes(include_bytes!("../fixtures/blank-part.SLDPRT")).unwrap();
    let source = PartConfigurationArchive::from_document(
        &document,
        ConfigurationId::new(0),
        ArchiveVersion::new(19000),
        None,
        LIMITS,
    )
    .unwrap();
    PartResolvedFeaturesArchive::from_bytes(
        include_bytes!("../fixtures/intersection-reference-match/controls.bin"),
        source,
        &document.read_stream("Contents/CMgr").unwrap(),
        LIMITS,
    )
    .unwrap()
}

fn reference_group() -> (Vec<ModelField>, ModelFieldHandle) {
    let archive = reference_archive();
    let index = archive
        .fields()
        .iter()
        .position(|field| intersection_candidate(&field.path))
        .unwrap();
    (
        archive.fields().to_vec(),
        archive.field_handle(index).unwrap(),
    )
}

#[test]
fn reference_projection_retains_signed_words_and_all_revision_fields() {
    let (fields, handle) = reference_group();
    let reference = project_intersection_reference(&fields, &handle).unwrap();
    assert_eq!(
        reference.words(),
        ModelIntersectionReferenceStoredWords {
            raw_state: 0,
            raw_word_20: -2,
            span_count_state: i32::MIN,
            backward_offset_state: i32::MAX
        }
    );
    assert_eq!(reference, reference.clone());
    for (relative, source) in reference.source_fields().iter().enumerate() {
        assert_eq!(source.index(), handle.index() + relative);
        assert_eq!(source.revision, handle.revision);
    }
}

#[test]
fn reference_projection_rejects_each_wrong_type_and_cross_scope_word() {
    let (fields, handle) = reference_group();
    for relative in 0..4 {
        let mut wrong = fields.clone();
        wrong[handle.index + relative].value = ModelFieldValue::U8(9);
        let error = project_intersection_reference(&wrong, &handle).unwrap_err();
        assert_eq!(error.class, FailureClass::Invalid);
        assert!(error.cause.contains("expected"));
        wrong = fields.clone();
        wrong[handle.index + relative].path = "root.unrelated.raw_20".into();
        assert!(project_intersection_reference(&wrong, &handle).is_err());
    }
}

#[test]
fn reference_scope_rejects_noncanonical_indices_and_other_callers() {
    let prefix = "root.features[0].styled_intersections.intersections[1].edges_48[0]";
    for group in ["38", "80", "c8", "110"] {
        let path = format!("{prefix}.group_{group}.raw_00");
        assert!(intersection_candidate(&path));
        assert!(reference_scope(&path).is_ok());
    }
    for path in [
        format!("{prefix}.group_40.raw_00"),
        format!("{prefix}.group_38.raw_24"),
        format!("{prefix}.group_38.raw_00").replace("edges_48[0]", "edges_48[00]"),
        format!("{prefix}.group_38.raw_00").replace("intersections[1]", "intersections[+1]"),
        format!("{prefix}.group_38.raw_00").replace("styled_intersections", "other"),
        "root.group_38.raw_00".into(),
        String::new(),
    ] {
        assert!(reference_scope(&path).is_err(), "{path}");
    }
    assert!(!intersection_candidate("root.group_38.raw_00"));
}

#[test]
fn reference_projection_requires_real_sketch_marker_and_edge_count() {
    let (fields, handle) = reference_group();
    for (suffix, replacement) in [
        (".marker", ModelFieldValue::U16(1)),
        (".handle_count", ModelFieldValue::I32(0)),
        (".handle_count", ModelFieldValue::I32(-1)),
    ] {
        let mut wrong = fields.clone();
        let target = wrong[..handle.index]
            .iter_mut()
            .rev()
            .find(|field| field.path.ends_with(suffix))
            .unwrap();
        target.value = replacement;
        assert!(project_intersection_reference(&wrong, &handle).is_err());
    }
}

#[test]
fn reference_projection_rejects_other_runtime_class() {
    let (mut wrong, handle) = reference_group();
    let owner = wrong
        .iter_mut()
        .find(|field| field.path.as_ref() == "root.features[0]")
        .unwrap();
    let ModelFieldValue::ObjectTag(tag) = &mut owner.value else {
        panic!("expected sketch object tag")
    };
    let ObjectRecord::New { class, .. } = &mut tag.record else {
        panic!("expected sketch definition")
    };
    class.name = "sgEllipseHandle".into();
    assert!(project_intersection_reference(&wrong, &handle).is_err());
}

#[test]
fn reference_projection_rejects_missing_fields_and_maximum_index() {
    let (fields, handle) = reference_group();
    for end in 0..handle.index + 4 {
        assert!(project_intersection_reference(&fields[..end], &handle).is_err());
    }
    let mut outside = handle;
    outside.index = usize::MAX;
    let error = project_intersection_reference(&fields, &outside).unwrap_err();
    assert_eq!(error.class, FailureClass::Invalid);
    assert!(error.cause.contains(&usize::MAX.to_string()));
}

#[test]
fn reference_role_replacements_preserve_the_other_signed_word() {
    let (fields, handle) = reference_group();
    let reference = project_intersection_reference(&fields, &handle).unwrap();
    for (edit, expected) in [
        (Edit::SpanCountState(-2), [-2, i32::MAX]),
        (Edit::BackwardOffsetState(-1), [i32::MIN, -1]),
        (
            Edit::SpanAndBackwardStates {
                span_count_state: 0,
                backward_offset_state: i32::MIN,
            },
            [0, i32::MIN],
        ),
    ] {
        assert_eq!(reference.replacements(edit), expected);
    }
}

#[test]
fn reference_public_types_support_shared_immutable_publication() {
    fn assert_send_sync<T: Send + Sync>() {}
    assert_send_sync::<ModelIntersectionReferenceStoredWords>();
    assert_send_sync::<ModelIntersectionReferenceStoredState>();
    assert_send_sync::<ModelIntersectionReferenceStoredEdit>();
}
