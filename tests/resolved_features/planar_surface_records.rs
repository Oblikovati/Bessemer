use super::annotation_view_tests::{LIMITS, decode};
use super::direct_collection_tests::decode_kernel;
use super::{CompressedKernelContext, ModelField};
use crate::Result;
use crate::archive_codec::ArchiveClass;

const OBJECT: &[u8] = include_bytes!("../fixtures/resolved-features/planar-object.bin");
const KEEPER: &[u8] = include_bytes!("../fixtures/resolved-features/planar-keeper.bin");
const CURRENT: &[u8] = include_bytes!("../fixtures/resolved-features/planar-surface-current.bin");
const OLD: &[u8] = include_bytes!("../fixtures/resolved-features/planar-surface-old.bin");
const KERNEL: &[u8] = include_bytes!("../fixtures/resolved-features/planar-surface-kernel.bin");

fn read_payload(
    version: u32,
    bytes: &[u8],
    keeper: bool,
    kernel: Option<CompressedKernelContext>,
) -> Result<Vec<ModelField>> {
    decode_kernel(version, bytes, LIMITS, kernel, |reader, map| {
        map.append_loaded_object(Some(ArchiveClass {
            name: if keeper {
                "moCompEdge_c"
            } else {
                "moAttribute_c"
            }
            .into(),
            schema: 1,
        }))?;
        if keeper {
            return super::planar_surface_records::keeper(reader, map);
        }
        super::planar_surface_records::surface(reader, map)
    })
}

#[test]
fn complete_planar_reference_geometry_derived_keeper_and_inline_attributes_match_reference() {
    let fields = decode(19000, OBJECT).unwrap();
    super::manager_oracle_tests::assert_reference_fields(
        &fields,
        OBJECT,
        include_str!("../fixtures/resolved-features/planar-object-primitives.tsv"),
        181,
    );
    assert_eq!(super::encode_fields(&fields, 30000).unwrap(), OBJECT);
}

#[test]
fn inherited_chooser_accepts_new_concrete_atom_owners_without_admitting_unrelated_features() {
    use super::direct_collection_tests::decode_collection;
    let bytes = [1, 0, 0, 0, 1, 0];
    for (name, expected) in [
        ("moPlanarSurface_c", true),
        ("Chamfer_c", true),
        ("moSensor_c", false),
        ("moAttribute_c", false),
        ("moPlanarSurface_c_suffix", false),
    ] {
        let result = decode_collection(19000, &bytes, |reader, map| {
            map.append_loaded_object(Some(ArchiveClass {
                name: name.into(),
                schema: 1,
            }))?;
            super::body_features::chooser(reader, map)
        });
        assert_eq!(result.is_ok(), expected, "{name}");
    }
    for length in 0..bytes.len() {
        assert!(decode_collection(19000, &bytes[..length], super::body_features::chooser).is_err());
    }
    assert!(decode_collection(19000, &[1, 0, 0, 0, 0, 0], super::body_features::chooser).is_ok());
}

#[test]
fn planar_keeper_header_rejects_null_unrelated_and_class_suffix_lookalikes() {
    let fields = decode(19000, OBJECT).unwrap();
    let path = "root.object.reference_geometry.id_keeper";
    let null = super::manager_context_tests::replace_field(&fields, path, &[0, 0]);
    let error = decode(19000, &null).unwrap_err();
    assert_eq!(error.class, crate::FailureClass::Invalid);
    assert!(error.cause.contains("nonnull declared object"));
    for name in ["moNodeName_c", "moPlanarSurfIdKeeper_c_suffix"] {
        let mut fresh = Vec::new();
        crate::archive_codec::ObjectWriter::new()
            .object(
                &ArchiveClass {
                    name: name.into(),
                    schema: 1,
                },
                &mut fresh,
            )
            .unwrap();
        let wrong = super::manager_context_tests::replace_field(&fields, path, &fresh);
        let error = decode(19000, &wrong).unwrap_err();
        assert_eq!(error.class, crate::FailureClass::Invalid);
        assert!(error.cause.contains("id_keeper") && error.cause.contains(name));
    }
}

#[test]
fn planar_payload_version_gate_suppression_and_plain_keeper_word_count_match_independent_bytes() {
    for (version, bytes, rows, reads, keeper) in [
        (
            19000,
            KEEPER,
            include_str!("../fixtures/resolved-features/planar-keeper-primitives.tsv"),
            12,
            true,
        ),
        (
            0x2bcc,
            OLD,
            include_str!("../fixtures/resolved-features/planar-surface-old-primitives.tsv"),
            7,
            false,
        ),
        (
            0x2bcd,
            CURRENT,
            include_str!("../fixtures/resolved-features/planar-surface-current-primitives.tsv"),
            10,
            false,
        ),
    ] {
        let fields = read_payload(version, bytes, keeper, None).unwrap();
        super::manager_oracle_tests::assert_reference_fields(&fields, bytes, rows, reads);
        assert_eq!(super::encode_fields(&fields, 30000).unwrap(), bytes);
    }
}

#[test]
fn present_extra_kernel_body_requires_explicit_schema_then_reads_its_attribute_list() {
    assert_eq!(
        read_payload(19000, KERNEL, false, None).unwrap_err().class,
        crate::FailureClass::Missing
    );
    let fields = read_payload(
        19000,
        KERNEL,
        false,
        Some(super::kernel_frame_tests::context()),
    )
    .unwrap();
    super::manager_oracle_tests::assert_reference_fields(
        &fields,
        KERNEL,
        include_str!("../fixtures/resolved-features/planar-surface-kernel-primitives.tsv"),
        13,
    );
    assert_eq!(super::encode_fields(&fields, 30000).unwrap(), KERNEL);
    assert!(
        fields
            .iter()
            .any(|field| field.path.as_ref() == "root.extra_body_attributes[1]")
    );
}

#[test]
fn planar_edit_matches_independent_bytes_and_retains_input_ownership() {
    let mut input = OBJECT.to_vec();
    let original = decode(19000, &input).unwrap();
    input.fill(0);
    let mut fields = original.clone();
    fields
        .iter_mut()
        .find(|field| field.path.as_ref() == "root.object.planar.raw_7b8")
        .unwrap()
        .value = super::ModelFieldValue::I32(-770);
    let expected = include_bytes!("../fixtures/resolved-features/planar-object-edited.bin");
    assert_eq!(super::encode_fields(&fields, 30000).unwrap(), expected);
    assert_eq!(decode(19000, expected).unwrap(), fields);
    assert_eq!(super::encode_fields(&original, 30000).unwrap(), OBJECT);
}

#[test]
fn planar_prefixes_extra_bytes_negative_sheet_counts_and_wide_keeper_counts_reject() {
    for (version, bytes, keeper) in [
        (19000, KEEPER, true),
        (0x2bcc, OLD, false),
        (0x2bcd, CURRENT, false),
    ] {
        for length in 0..bytes.len() {
            assert!(read_payload(version, &bytes[..length], keeper, None).is_err());
        }
        assert!(read_payload(version, &[bytes, &[0]].concat(), keeper, None).is_err());
    }
    for length in 0..OBJECT.len() {
        assert!(decode(19000, &OBJECT[..length]).is_err());
    }
    assert!(decode(19000, &[OBJECT, &[0]].concat()).is_err());
    let negative: Vec<u8> = [9i32, -1].into_iter().flat_map(i32::to_le_bytes).collect();
    assert!(read_payload(19000, &negative, false, None).is_err());
    let mut wide = KEEPER.to_vec();
    let count = wide.len() - 10;
    wide[count..count + 2].copy_from_slice(&u16::MAX.to_le_bytes());
    assert!(read_payload(19000, &wide, true, None).is_err());
    let context = super::kernel_frame_tests::context();
    for length in 0..KERNEL.len() {
        assert!(read_payload(19000, &KERNEL[..length], false, Some(context.clone())).is_err());
    }
    assert!(read_payload(19000, &[KERNEL, &[0]].concat(), false, Some(context)).is_err());
}
