//! Complete modern assembly archive reconstruction, graph and edit regressions.
use bessemer::{Document, display_lists::*};
#[path = "support/display_archive.rs"]
mod archive_support;
use archive_support::{LIMITS, check_scalar_table, context, decode_document, set_integer};
const ASSEMBLY: &[u8] = include_bytes!("fixtures/blank-assembly.SLDASM");

#[test]
fn owned_native_assembly_decodes_every_byte_and_forces_exact_reconstruction() {
    let context = DisplayArchiveContext {
        kind: DisplayDocumentKind::Assembly,
        ..context()
    };
    let document = Document::from_bytes(include_bytes!("fixtures/blank-assembly.SLDASM")).unwrap();
    let bytes = document.read_stream("Contents/DisplayLists").unwrap();
    let archive = DisplayArchive::from_bytes(&bytes, context, LIMITS).unwrap();
    assert_eq!(bytes.len(), 5446);
    assert_eq!(archive.to_bytes().unwrap(), bytes);
    let rebuilt = DisplayArchive::new(archive.contents().clone(), context, LIMITS).unwrap();
    assert_eq!(rebuilt.to_bytes().unwrap(), bytes);
}

#[test]
fn component_and_envelope_fields_match_the_independent_reference() {
    let archive = decode_document(ASSEMBLY, DisplayDocumentKind::Assembly);
    let expected = include_str!("fixtures/display-lists/blank-assembly-component.txt");
    assert_eq!(check_scalar_table(&archive, expected), 40);
}

#[test]
fn object_is_registered_before_its_body_so_owner_cycles_preserve_identity() {
    let bytes = include_bytes!("fixtures/display-lists/owner-cycle-assembly.bin");
    let context = DisplayArchiveContext {
        kind: DisplayDocumentKind::Assembly,
        ..context()
    };
    let archive = DisplayArchive::from_bytes(bytes, context, LIMITS).unwrap();
    let owner = archive
        .contents()
        .fields
        .iter()
        .find(|field| field.path.as_ref() == "root.tessellation.components[0].owner")
        .unwrap();
    let DisplayFieldValue::ObjectTag(owner) = &owner.value else {
        panic!("owner pointer")
    };
    assert_eq!(
        owner.target,
        DisplayObjectPointer::Reference(DisplayObjectId::new(53).unwrap())
    );
    assert_eq!(
        owner.runtime_class.as_ref().unwrap().name,
        "uoTempSubAssemblySHDData_c"
    );
    assert_eq!(
        DisplayArchive::new(archive.contents().clone(), context, LIMITS)
            .unwrap()
            .to_bytes()
            .unwrap(),
        bytes
    );
}

#[test]
fn assembly_setting_edit_reopens_and_reverts_the_entire_owned_container() {
    let original = decode_document(ASSEMBLY, DisplayDocumentKind::Assembly);
    let mut contents = original.contents().clone();
    set_integer(
        &mut contents,
        "root.envelope_system_options.unknown_setting",
        -123,
    );
    let edited = original.with_contents(contents.clone(), LIMITS).unwrap();
    let mut document = Document::from_bytes(ASSEMBLY).unwrap();
    document
        .replace_stream("Contents/DisplayLists", edited.to_bytes().unwrap())
        .unwrap();
    let serialized = document.to_bytes().unwrap();
    assert_eq!(
        decode_document(&serialized, DisplayDocumentKind::Assembly).contents(),
        &contents
    );
    let reopened = Document::from_bytes(&serialized).unwrap();
    for stream in document
        .streams()
        .filter(|stream| stream.name != "Contents/DisplayLists")
    {
        assert_eq!(
            reopened.read_stream(stream.name).unwrap(),
            document.read_stream(stream.name).unwrap()
        );
    }
    let reverted = edited
        .with_contents(original.contents().clone(), LIMITS)
        .unwrap();
    document
        .replace_stream("Contents/DisplayLists", reverted.to_bytes().unwrap())
        .unwrap();
    assert_eq!(document.to_bytes().unwrap(), ASSEMBLY);
    assert_eq!(
        original.contents(),
        decode_document(ASSEMBLY, DisplayDocumentKind::Assembly).contents()
    );
}

#[test]
fn assembly_truncations_impossible_counts_and_trailing_bytes_cannot_publish_partial_results() {
    let original = decode_document(ASSEMBLY, DisplayDocumentKind::Assembly);
    let bytes = original.to_bytes().unwrap();
    for end in 0..bytes.len() {
        assert!(
            DisplayArchive::from_bytes(&bytes[..end], original.context(), LIMITS).is_err(),
            "cut {end}"
        );
    }
    let mut trailing = bytes;
    trailing.push(0);
    assert!(DisplayArchive::from_bytes(&trailing, original.context(), LIMITS).is_err());
    for path in [
        "root.tessellation.components[0].viewer_comp_data.gobbled_up_atom_ids.count",
        "root.tessellation.components[0].viewer_comp_data.equ_dep_comp_names.count",
        "root.tessellation.components[0].hidden_body_atoms.count",
    ] {
        let mut contents = original.contents().clone();
        set_integer(&mut contents, path, i32::MAX);
        assert!(original.with_contents(contents, LIMITS).is_err(), "{path}");
    }
}

#[test]
fn independently_built_populated_component_records_preserve_both_display_mode_layouts() {
    let fixtures: [(&[u8], &str, usize); 2] = [
        (
            include_bytes!("fixtures/display-lists/component-mode-6.bin"),
            include_str!("fixtures/display-lists/component-mode-6.txt"),
            55,
        ),
        (
            include_bytes!("fixtures/display-lists/component-mode--3.bin"),
            include_str!("fixtures/display-lists/component-mode--3.txt"),
            57,
        ),
    ];
    let context = DisplayArchiveContext {
        kind: DisplayDocumentKind::Assembly,
        ..context()
    };
    for (bytes, oracle, fields) in fixtures {
        let archive = DisplayArchive::from_bytes(bytes, context, LIMITS).unwrap();
        assert_eq!(check_scalar_table(&archive, oracle), fields);
        assert_eq!(
            DisplayArchive::new(archive.contents().clone(), context, LIMITS)
                .unwrap()
                .to_bytes()
                .unwrap(),
            bytes
        );
        let endpoint = archive
            .contents()
            .fields
            .iter()
            .find(|field| {
                field.path.as_ref() == "root.tessellation.components[0].hidden_body_atoms[2]"
            })
            .unwrap();
        assert_eq!(endpoint.value, DisplayFieldValue::I32(i32::MAX));
    }
}
