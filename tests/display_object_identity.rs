//! Revision-bound object handles keep wire indices local to immutable archive snapshots.
use bessemer::display_lists::*;
use bessemer::{FailureClass, Result};
#[allow(
    dead_code,
    reason = "Shared archive fixtures also serve scalar and mutation tests."
)]
#[path = "support/display_archive.rs"]
mod archive_support;
use archive_support::LIMITS;

fn native_part() -> DisplayArchive {
    archive_support::decode_document(
        include_bytes!("fixtures/blank-part.SLDPRT"),
        DisplayDocumentKind::Part,
    )
}

fn first_object(archive: &DisplayArchive) -> DisplayObjectId {
    archive
        .contents()
        .fields
        .iter()
        .find_map(|field| {
            let DisplayFieldValue::ObjectTag(tag) = &field.value else {
                return None;
            };
            match tag.target {
                DisplayObjectPointer::New(id) => Some(id),
                _ => None,
            }
        })
        .unwrap()
}

fn change_ambient(archive: &DisplayArchive) -> DisplayArchive {
    let mut contents = archive.contents().clone();
    let field = contents
        .fields
        .iter_mut()
        .find(|field| field.path.as_ref() == "root.visual_properties.material.ambient")
        .unwrap();
    field.value = DisplayFieldValue::F64(StoredDouble::from_bits(23.0_f64.to_bits()));
    archive.with_contents(contents, LIMITS).unwrap()
}

#[test]
fn wire_indices_and_aliases_resolve_to_one_revision_bound_definition() {
    let archive = DisplayArchive::from_bytes(
        include_bytes!("fixtures/display-lists/shared-font-part.bin"),
        archive_support::context(),
        LIMITS,
    )
    .unwrap();
    let mut aliases = 0;
    for field in &archive.contents().fields {
        let DisplayFieldValue::ObjectTag(tag) = &field.value else {
            continue;
        };
        let id = match tag.target {
            DisplayObjectPointer::New(id) => id,
            DisplayObjectPointer::Reference(id) => {
                aliases += 1;
                id
            }
            DisplayObjectPointer::Null => continue,
        };
        let handle = archive.object_handle(id).unwrap();
        assert_eq!(handle.object_id(), id);
        let definition = archive.resolve_object(&handle).unwrap();
        let DisplayFieldValue::ObjectTag(definition) = &definition.value else {
            panic!("expected definition tag")
        };
        assert_eq!(definition.target, DisplayObjectPointer::New(id));
        assert_eq!(definition.runtime_class, tag.runtime_class);
        assert_eq!(archive.object_handle(id).unwrap(), handle);
    }
    assert!(aliases > 0);
}

#[test]
fn separate_decodes_constructions_and_document_kinds_cannot_exchange_handles() {
    let archive = native_part();
    let id = first_object(&archive);
    let handle = archive.object_handle(id).unwrap();
    let constructed =
        DisplayArchive::new(archive.contents().clone(), archive.context(), LIMITS).unwrap();
    let assembly = archive_support::decode_document(
        include_bytes!("fixtures/blank-assembly.SLDASM"),
        DisplayDocumentKind::Assembly,
    );
    for other in [native_part(), constructed, assembly] {
        let error = other.resolve_object(&handle).unwrap_err();
        assert_eq!(error.class, FailureClass::Invalid);
        assert_eq!(error.operation, "resolve display object");
        assert!(error.entity.contains(&id.value().to_string()));
        assert!(error.cause.contains("another archive revision"));
        assert_ne!(other.object_handle(id).unwrap(), handle);
    }
}

#[test]
fn clones_and_equivalent_edits_keep_handles_while_changed_edits_get_new_revisions() {
    let archive = native_part();
    let id = first_object(&archive);
    let handle = archive.object_handle(id).unwrap();
    let equivalent = archive
        .with_contents(archive.contents().clone(), LIMITS)
        .unwrap();
    for same in [archive.clone(), equivalent] {
        assert_eq!(
            same.resolve_object(&handle).unwrap(),
            archive.resolve_object(&handle).unwrap()
        );
        assert_eq!(same.object_handle(id).unwrap(), handle);
    }
    let changed = change_ambient(&archive);
    assert!(changed.resolve_object(&handle).is_err());
    assert!(
        archive
            .resolve_object(&changed.object_handle(id).unwrap())
            .is_err()
    );
    assert!(archive.resolve_object(&handle).is_ok());
}

#[test]
fn authentic_byte_reverts_create_a_new_revision_without_reviving_stale_handles() {
    let archive = native_part();
    let id = first_object(&archive);
    let handle = archive.object_handle(id).unwrap();
    let changed = change_ambient(&archive);
    let changed_handle = changed.object_handle(id).unwrap();
    let reverted = changed
        .with_contents(archive.contents().clone(), LIMITS)
        .unwrap();
    assert_eq!(reverted.to_bytes().unwrap(), archive.to_bytes().unwrap());
    assert!(reverted.resolve_object(&handle).is_err());
    assert!(reverted.resolve_object(&changed_handle).is_err());
    assert!(
        reverted
            .resolve_object(&reverted.object_handle(id).unwrap())
            .is_ok()
    );
}

#[test]
fn class_slots_missing_indices_and_failed_edits_do_not_publish_object_handles() {
    let archive = native_part();
    for index in [1, 0x7fff_ffff] {
        let error = archive
            .object_handle(DisplayObjectId::new(index).unwrap())
            .unwrap_err();
        assert_eq!(error.class, FailureClass::Invalid);
        assert!(error.entity.contains(&index.to_string()));
        assert!(error.cause.contains("expected a loaded object"));
    }
    let handle = archive.object_handle(first_object(&archive)).unwrap();
    assert!(
        archive
            .with_contents(
                archive.contents().clone(),
                DisplayArchiveLimits {
                    maximum_bytes: 0,
                    ..LIMITS
                }
            )
            .is_err()
    );
    assert!(archive.resolve_object(&handle).is_ok());
}

#[test]
fn handles_are_send_sync_and_concurrent_lookup_and_edits_remain_isolated() -> Result<()> {
    fn assert_send_sync<T: Send + Sync>() {}
    assert_send_sync::<DisplayObjectHandle>();
    let archive = native_part();
    let handle = archive.object_handle(first_object(&archive))?;
    std::thread::scope(|scope| {
        let workers: Vec<_> = (0..16)
            .map(|_| {
                scope.spawn(|| {
                    assert!(archive.resolve_object(&handle).is_ok());
                    assert!(change_ambient(&archive).resolve_object(&handle).is_err());
                    archive.object_handle(handle.object_id()).unwrap()
                })
            })
            .collect();
        for worker in workers {
            assert_eq!(worker.join().unwrap(), handle);
        }
    });
    assert!(archive.resolve_object(&handle).is_ok());
    Ok(())
}
