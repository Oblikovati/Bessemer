//! Owned logical component oracles establish no physical record decoder or chunk codec.
#[path = "flow_checkpoint_components/fixture.rs"]
mod fixture;
#[path = "flow_checkpoint_components/grids.rs"]
mod grids;
#[path = "flow_field_values/packet.rs"]
mod packet;
#[path = "flow_checkpoint_components/scalars.rs"]
mod scalars;
#[path = "flow_checkpoint_components/types.rs"]
mod types;
use fixture::cases;
use types::*;
#[path = "flow_checkpoint_components/api/mod.rs"]
mod api;
#[path = "flow_state_tables/process.rs"]
mod process;

#[test]
fn all_2899_owned_components_match_complete_source_spans_values_presence_and_qualifications() {
    let rows = cases();
    assert_eq!(rows.len(), 2899);
    let mut counts = [0; 5];
    for row in rows {
        assert!(!row.name.is_empty());
        assert_eq!(row.end, row.raw.len());
        assert_eq!(row.qualifications, [true, false, false]);
        check_projection(&row);
        counts[row.kind as usize] += 1;
    }
    assert_eq!(counts, [946, 1263, 509, 28, 153]);
}
fn check_projection(row: &ComponentCase) {
    match &row.projection {
        ComponentProjection::Graph1(values) => {
            assert_eq!(row.kind, ComponentKind::Graph1);
            assert_eq!(values.len(), 7);
            assert_eq!(scalars::wides(&row.raw, 0, values), 56);
        }
        ComponentProjection::Grid(expected) => {
            assert!(matches!(
                row.kind,
                ComponentKind::Graph2 | ComponentKind::Auxiliary
            ));
            grids::grid(&row.raw, expected, row.kind == ComponentKind::Graph2);
        }
        ComponentProjection::Trailing(containers) => check_trailing(row, containers),
        ComponentProjection::Chunks(expected) => {
            assert_eq!(row.kind, ComponentKind::Chunks);
            scalars::chunks(&row.raw, expected);
        }
    }
}
fn check_trailing(row: &ComponentCase, containers: &[ContainerProjection]) {
    assert_eq!(row.kind, ComponentKind::Trailing);
    assert_eq!(containers.len(), 3);
    let mut position = 0;
    for container in containers {
        assert_eq!(container.start, position);
        scalars::container(&row.raw, container);
        position = container.end;
    }
    assert_eq!(position, row.raw.len());
}
#[test]
fn literal_reference_examples_keep_raw_nan_payloads_cyclic_axes_and_opaque_chunks() {
    let rows = cases();
    assert_eq!(
        rows.iter()
            .filter(|row| row.name.starts_with("reference-"))
            .count(),
        5
    );
    check_reference_grid(named(&rows, "reference-graph2"));
    check_reference_chunks(named(&rows, "reference-chunks"));
}
fn check_reference_grid(graph: &ComponentCase) {
    let ComponentProjection::Grid(grid) = &graph.projection else {
        panic!("expected graph2");
    };
    assert_eq!(grid.dimensions, [1, 1, 1]);
    assert_eq!(grid.members, [4, 5, 6]);
    let first = grid.groups[0].rows[0].row.as_ref().unwrap();
    assert_eq!(first.indices, [7, 8]);
    let slots = first.slots.as_ref().unwrap();
    assert_eq!(slots[0].tag, 0);
    assert_eq!(slots[1].tag, 2);
    assert_eq!(
        slots[1].container.as_ref().unwrap().first,
        [0x7ff8000000000001]
    );
    assert!(grid.groups[1].rows[0].row.is_none());
}
fn check_reference_chunks(row: &ComponentCase) {
    let ComponentProjection::Chunks(chunks) = &row.projection else {
        panic!("expected chunks");
    };
    assert_eq!(chunks.declared, 500);
    assert_eq!(chunks.chunks.len(), 2);
    assert_eq!(chunks.chunks[0].encoded, b"abc");
    assert!(chunks.chunks[1].encoded.is_empty());
    assert!(!chunks.codec_decoded && !chunks.decoded_size_verified);
}
#[test]
fn all_byte_tags_and_presence_flags_keep_reader_permissiveness_separate_from_writer_canonicality() {
    let rows = cases();
    for tag in 0..=255 {
        let row = named(&rows, &format!("graph2-tag-{tag}"));
        let ComponentProjection::Grid(grid) = &row.projection else {
            panic!("expected grid");
        };
        let slot = &grid.groups[2].rows[0]
            .row
            .as_ref()
            .unwrap()
            .slots
            .as_ref()
            .unwrap()[0];
        assert_eq!(slot.tag, tag);
        assert_eq!(slot.canonical, tag <= 2);
        assert_eq!(slot.container.is_some(), tag > 1);
        check_presence_case(named(&rows, &format!("graph2-presence-{tag}")), tag);
        check_presence_case(named(&rows, &format!("auxiliary-presence-{tag}")), tag);
    }
}
fn check_presence_case(row: &ComponentCase, flag: u8) {
    let ComponentProjection::Grid(grid) = &row.projection else {
        panic!("expected grid");
    };
    for group in &grid.groups {
        assert_eq!(group.presence, [flag]);
        assert_eq!(group.rows[0].flag, flag);
        assert_eq!(group.rows[0].row.is_some(), flag != 0);
    }
}
#[test]
fn wide_counts_empty_containers_and_signed_members_have_independent_literal_expectations() {
    let rows = cases();
    let row = named(&rows, "graph2-row-2-65536");
    let ComponentProjection::Grid(grid) = &row.projection else {
        panic!("expected grid");
    };
    let body = grid.groups[2].rows[0].row.as_ref().unwrap();
    assert_eq!(body.count, 65536);
    assert_eq!(body.indices[1], 0x80000001);
    assert_eq!(body.indices[2], 2);
    assert_eq!(body.slots.as_ref().unwrap().len(), 65536);
    let row = named(&rows, "graph2-dimension-1-65536");
    let ComponentProjection::Grid(grid) = &row.projection else {
        panic!("expected grid");
    };
    assert_eq!(grid.groups[0].length, 65536);
    assert_eq!(grid.groups[1].length, 0);
    assert_eq!(grid.groups[2].length, 0);
    assert_eq!(grid.members, [i32::MIN, -1, i32::MAX]);
}
fn named<'a>(rows: &'a [ComponentCase], name: &str) -> &'a ComponentCase {
    rows.iter().find(|row| row.name == name).unwrap()
}
