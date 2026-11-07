//! Independently assembled field vectors and archive identity contracts.
#[path = "support/tessellation.rs"]
mod support;
use bessemer::tessellation::*;
use support::*;

#[test]
fn all_independent_version_and_mode_vectors_reconstruct_without_original_bytes() {
    for row in include_str!("fixtures/tessellation/cases.tsv").lines() {
        let columns: Vec<&str> = row.split('\t').collect();
        let mode = if columns[2] == "full" {
            FacetArchiveMode::Full
        } else {
            FacetArchiveMode::Lightweight
        };
        let context = context(columns[1].parse().unwrap(), mode);
        let bytes = fixture(columns[0]);
        let decoded = FacetArchive::from_bytes(&bytes, context, LIMITS).unwrap();
        assert_eq!(decoded.context(), context);
        let rebuilt = FacetArchive::new(context, decoded.contents().clone()).unwrap();
        assert_eq!(rebuilt.to_bytes().unwrap(), bytes, "{}", columns[0]);
        assert_eq!(
            decoded
                .with_contents(decoded.contents().clone())
                .unwrap()
                .to_bytes()
                .unwrap(),
            bytes
        );
    }
}

#[test]
fn raw_flags_arrays_unknown_fields_and_ieee_payloads_survive() {
    let archive = parse(MODERN);
    let face = archive.contents();
    let FacetVertexLayout::Styled {
        mode,
        style,
        vertex_numbers,
    } = &face.vertex_layout
    else {
        panic!("styled fixture")
    };
    assert_eq!(*mode, -17);
    assert_eq!(
        (style.face_color, style.prune_flags, style.flags_18),
        (0x11223344, u32::MAX, u32::MAX - 2)
    );
    assert_eq!(style.ignored_ints, [-1, -2]);
    assert_eq!(style.ignored_floats[0].bits(), 0x7fc00042);
    assert!(style.ignored_floats[0].value().is_nan());
    assert_eq!(style.ignored_floats[1].bits(), 0x80000000);
    assert_eq!(
        vertex_numbers.signed_integers().unwrap().unwrap(),
        [0x03020100]
    );
    let points = face.vertex_coords.float_triplets().unwrap().unwrap();
    assert_eq!(points.len(), 3);
    assert_eq!(points[0][2].bits(), 0x0b0a0908);
    assert_eq!(face.container_processed, -2);
    assert_eq!(face.edge_ids_valid, Some(-3));
    assert_eq!(
        face.face_string
            .as_ref()
            .unwrap()
            .text
            .as_ref()
            .unwrap()
            .text(),
        "λ"
    );
    assert_eq!(face.edge_points.as_ref().unwrap().entries[0].edge_id, -51);
    assert_eq!(face.bounds.centre[0].bits(), 0x7ff8000000000042);
    assert!(face.bounds.centre[0].value().is_nan());
    assert_eq!(face.edges.as_ref().unwrap()[0].point.len(), 4);
    assert_eq!(face.edges.as_ref().unwrap()[0].vertices.count, -1);
}

#[test]
fn omission_state_is_carried_between_edges_and_reset_between_faces() {
    let bytes = fixture("facet-3f0f-1.bin");
    let context = context(0x3f0f, FacetArchiveMode::Full);
    let archive = FacetArchive::from_bytes(&bytes, context, LIMITS).unwrap();
    let edges = archive.contents().edges.as_ref().unwrap();
    assert_eq!(edges[0].first, Some(31));
    assert_eq!(edges[1].first, None);
    assert_eq!(
        archive.contents().late_stamps.as_ref().unwrap().unknown_2a8,
        None
    );
    let block = FaceTessellationBlock::new(
        context,
        FaceTessellationBlockContents {
            entries: vec![FacetBlockComponent {
                index_in_block: 4,
                hash_key: 5,
                faces: vec![archive.contents().clone(); 2],
            }],
        },
    )
    .unwrap();
    assert_eq!(
        FaceTessellationBlock::from_bytes(&block.to_bytes().unwrap(), context, LIMITS)
            .unwrap()
            .contents(),
        block.contents()
    );
}

#[test]
fn directory_versions_preserve_order_duplicate_keys_and_high_word_first_identifiers() {
    for version in [0x3b1b, 0x3b1c, 0x3b22, 0x3b23] {
        let context = context(version, FacetArchiveMode::Full);
        let bytes = fixture(&format!("directory-{version:04x}.bin"));
        let source = TessellationDirectory::from_bytes(&bytes, context, LIMITS).unwrap();
        let entry = &source.contents().entries[0];
        assert_eq!(entry.component_id.text(), "λ/bolt");
        assert_eq!(entry.user_component_time, 0x0123456789abcdef);
        assert_eq!(entry.shared_part_component_info, 0xfedcba9876543210);
        assert_eq!(entry.hash_key, 0x123456789abcdef0);
        match &entry.tessellation_numbers {
            ComponentTessellationNumbers::Absent => assert!(version < 0x3b1c),
            ComponentTessellationNumbers::LegacyPair(pair) => assert_eq!(*pair, [-1, 2]),
            ComponentTessellationNumbers::Map(records) => assert_eq!(
                records.iter().map(|record| record.key).collect::<Vec<_>>(),
                [-1, -1]
            ),
        }
        assert_eq!(
            TessellationDirectory::new(context, source.contents().clone())
                .unwrap()
                .to_bytes()
                .unwrap(),
            bytes
        );
    }
}

#[test]
fn block_links_report_checked_and_unreferenced_entries() {
    let source = block();
    assert_eq!(source.contents().entries[0].faces.len(), 2);
    assert_eq!(
        FaceTessellationBlock::new(source.context(), source.contents().clone())
            .unwrap()
            .to_bytes()
            .unwrap(),
        BLOCK
    );
    assert_eq!(
        directory()
            .check_block_links("000-000-007", &source)
            .unwrap(),
        BlockLinkCoverage {
            checked_references: 1,
            available_entries: 1,
            unreferenced_entries: 0,
        }
    );
    assert_eq!(
        directory().check_block_links("other", &source).unwrap(),
        BlockLinkCoverage {
            checked_references: 0,
            available_entries: 1,
            unreferenced_entries: 1,
        }
    );
}

#[test]
fn empty_and_absent_arrays_have_distinct_typed_views() {
    let empty = DynamicArray::Present(DynamicArrayValues {
        element_size: ArrayElementSize::new(4).unwrap(),
        initial_capacity: 0,
        growth_step: 0,
        count: 0,
        payload: vec![],
    });
    assert_eq!(empty.signed_integers().unwrap(), Some(vec![]));
    assert_eq!(DynamicArray::Absent.signed_integers().unwrap(), None);
    assert_eq!(DynamicArray::Absent.float_triplets().unwrap(), None);
    assert!(empty.float_triplets().is_err());
    assert!(ArrayElementSize::new(0).is_err());
    let invalid = DynamicArray::Present(DynamicArrayValues {
        element_size: ArrayElementSize::new(4).unwrap(),
        initial_capacity: 0,
        growth_step: 0,
        count: 2,
        payload: vec![0; 4],
    });
    assert!(invalid.signed_integers().is_err());
}

#[test]
fn minimal_omitted_edge_fields_do_not_inflate_the_count_bound() {
    let context = context(0x3f0f, FacetArchiveMode::Full);
    let source = FacetArchive::from_bytes(&fixture("facet-3f0f-1.bin"), context, LIMITS).unwrap();
    let mut contents = source.contents().clone();
    for edge in contents.edges.as_mut().unwrap() {
        edge.point.clear();
        edge.list_38.clear();
    }
    let encoded = source
        .with_contents(contents.clone())
        .unwrap()
        .to_bytes()
        .unwrap();
    assert_eq!(
        FacetArchive::from_bytes(&encoded, context, LIMITS)
            .unwrap()
            .contents(),
        &contents
    );
}

#[test]
fn populated_vertex_pairs_keep_low_word_first_order() {
    let source = parse(MODERN);
    let mut contents = source.contents().clone();
    contents.edges.as_mut().unwrap()[0].vertices = EdgeVertexPairs {
        count: 1,
        pairs: vec![[0x11223344, 0xaabbccdd]],
    };
    let bytes = source
        .with_contents(contents.clone())
        .unwrap()
        .to_bytes()
        .unwrap();
    assert!(
        bytes
            .windows(8)
            .any(|span| span == [0x44, 0x33, 0x22, 0x11, 0xdd, 0xcc, 0xbb, 0xaa])
    );
    assert_eq!(parse(&bytes).contents(), &contents);
}
