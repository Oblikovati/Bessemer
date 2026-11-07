use super::annotation_view_tests::LIMITS;
use super::lightweight_section_tests::next_case;
use super::sheet_metal_tests::decode_graph;
use crate::archive_codec::ArchiveCursor;

const PACKET: &[u8] = include_bytes!("../fixtures/cut-list-folders/cases.bin");
const MINIMA: [usize; 14] = [
    159, 159, 159, 175, 179, 224, 161, 161, 161, 161, 161, 161, 161, 161,
];
type CutListRecord = (u8, u32, &'static [u8], &'static str, usize);

fn records() -> Vec<CutListRecord> {
    let mut cursor = ArchiveCursor::new(PACKET);
    assert_eq!(cursor.take(4).unwrap(), b"CLF1");
    assert_eq!(cursor.u32().unwrap(), 252);
    let slots: Vec<_> = (0..252).map(|_| cursor.u32().unwrap() as usize).collect();
    let result = slots
        .into_iter()
        .map(|slots| {
            let (kind, version, bytes, oracle) = next_case(&mut cursor);
            (kind, version, bytes, oracle, slots)
        })
        .collect();
    assert_eq!(cursor.remaining(), 0);
    result
}

#[test]
fn complete_cut_list_folder_graphs_match_independent_fields_bytes_and_slots() {
    for (_, version, bytes, oracle, slots) in records() {
        let (fields, actual_slots) = decode_graph(version, bytes, None, LIMITS).unwrap();
        assert_eq!(actual_slots, slots);
        super::manager_oracle_tests::assert_reference_fields(
            &fields,
            bytes,
            oracle,
            oracle.lines().count(),
        );
        assert_eq!(super::encode_fields(&fields, 30000).unwrap(), bytes);
    }
}

#[test]
fn cut_list_folders_reject_every_prefix_extra_byte_and_old_context() {
    for (kind, version, bytes, _, _) in records() {
        for end in 0..bytes.len() {
            assert!(
                decode_graph(version, &bytes[..end], None, LIMITS).is_err(),
                "{kind}/{version:#x}/{end}"
            );
        }
        assert!(decode_graph(version, &[bytes, &[0]].concat(), None, LIMITS).is_err());
        assert!(decode_graph(0x27cb, bytes, None, LIMITS).is_err());
    }
}

fn body_prefix(kind: u8) -> &'static str {
    match kind {
        0..=2 => "",
        5 => "sub_weld.body_folder.",
        _ => "body_folder.",
    }
}

fn declared_paths(kind: u8) -> Vec<String> {
    let outer = if kind == 5 { "sub_weld." } else { "" };
    let body = body_prefix(kind);
    let mut paths = vec![
        format!("{body}material_folder"),
        format!("{body}cut_list_sort_options"),
    ];
    if kind >= 3 {
        paths.extend([
            format!("{outer}sub_atom.list_3d8"),
            format!("{outer}sub_atom.feature_reference_3e8"),
        ]);
    }
    if kind == 5 {
        paths.extend(["payload.feature_490".into(), "payload.plane_460".into()]);
    }
    paths
}

fn rejected_field(version: u32, bytes: &[u8], path: &str, replacement: &[u8]) {
    let fields = decode_graph(version, bytes, None, LIMITS).unwrap().0;
    let changed = super::manager_context_tests::replace_field(&fields, path, replacement);
    let Err(error) = decode_graph(version, &changed, None, LIMITS) else {
        panic!("{path}: expected invalid modified folder framing");
    };
    assert_eq!(error.class, crate::FailureClass::Invalid, "{path}");
}

#[test]
fn inherited_and_cut_list_pointer_roles_reject_unrelated_cached_arrays() {
    for (kind, version, bytes, _, _) in records()
        .into_iter()
        .filter(|row| row.0 % 6 == 2 && row.0 / 6 < 6)
    {
        for suffix in declared_paths(kind / 6) {
            let path = format!("root.object.items[0].{suffix}");
            rejected_field(version, bytes, &path, &2u16.to_le_bytes());
        }
    }
}

#[test]
fn direct_property_lists_reject_negative_and_impossible_counts_without_map_tags() {
    for (kind, version, bytes, _, _) in records()
        .into_iter()
        .filter(|row| [26, 32].contains(&row.0))
    {
        let fields = decode_graph(version, bytes, None, LIMITS).unwrap().0;
        let prefix = if kind == 32 { "sub_weld." } else { "" };
        let path = format!("root.object.items[0].{prefix}direct_properties_448.properties.count");
        for count in [-1i32, i32::MAX] {
            rejected_field(version, bytes, &path, &count.to_le_bytes());
        }
        assert!(
            fields
                .iter()
                .filter(|field| field.path.contains("direct_properties_448"))
                .all(|field| !matches!(field.value, super::ModelFieldValue::ObjectTag(_)))
        );
    }
}

fn minimum_object(name: &str, size: usize) -> Vec<u8> {
    let mut bytes = Vec::new();
    crate::archive_codec::ObjectWriter::new()
        .object(
            &crate::archive_codec::ArchiveClass {
                name: name.into(),
                schema: 1,
            },
            &mut bytes,
        )
        .unwrap();
    bytes.extend(vec![0; size]);
    bytes
}

#[test]
fn all_fourteen_native_bound_folder_classes_have_the_documented_minimum_layers() {
    for (kind, _, bytes, _, _) in records()
        .into_iter()
        .filter(|row| row.0 % 6 == 0 && row.1 == 19000)
    {
        let length = u16::from_le_bytes([bytes[4], bytes[5]]) as usize;
        let name = std::str::from_utf8(&bytes[6..6 + length]).unwrap();
        let size = MINIMA[usize::from(kind / 6)];
        let minimum = minimum_object(name, size);
        let (fields, slots) = decode_graph(19000, &minimum, None, LIMITS).unwrap();
        assert_eq!(slots, 3, "{name}");
        assert_eq!(super::encode_fields(&fields, 30000).unwrap(), minimum);
        assert!(decode_graph(19000, &minimum[..minimum.len() - 1], None, LIMITS).is_err());
    }
}

#[test]
fn cut_list_flag_byte_retains_zero_native_values_and_noncanonical_high_bits() {
    for (kind, version, bytes, _, _) in records().into_iter().filter(|row| row.0 / 6 == 5) {
        let fields = decode_graph(version, bytes, None, LIMITS).unwrap().0;
        let expected = [0, 1, 2, 3, 128, 255][usize::from(kind % 6)];
        for field in fields
            .iter()
            .filter(|field| field.path.ends_with("payload.raw_484"))
        {
            assert_eq!(field.value, super::ModelFieldValue::U8(expected));
        }
    }
}

#[test]
fn nonnull_undocumented_feature_wrapper_bodies_remain_explicit_dependencies() {
    let pointer = minimum_object("moFeatureRefWrapper_c", 0);
    for (_, version, bytes, _, _) in records().into_iter().filter(|row| row.0 == 20) {
        let fields = decode_graph(version, bytes, None, LIMITS).unwrap().0;
        let changed = super::manager_context_tests::replace_field(
            &fields,
            "root.object.items[0].sub_atom.feature_reference_3e8",
            &pointer,
        );
        let Err(error) = decode_graph(version, &changed, None, LIMITS) else {
            panic!("moFeatureRefWrapper_c: expected an unsupported complete-body dependency");
        };
        assert_eq!(error.class, crate::FailureClass::Unsupported);
        assert!(error.cause.contains("moFeatureRefWrapper_c"));
    }
}
