//! Independent facet records and explicit legacy array metadata expectations.
use super::*;

pub(super) fn fontless_facet_bytes(version: u32, index: usize, edited: usize) -> &'static [u8] {
    FACETS.iter().find(|row| row.0 == version).unwrap().1[index][edited]
}

pub(super) fn expected_fontless_facet(version: u32, index: usize, edited: usize) -> FacetArchive {
    FacetArchive::from_bytes(
        fontless_facet_bytes(version, index, edited),
        legacy_rendering::facet_context(version),
        TessellationLimits {
            maximum_components: 0,
            maximum_faces: 1,
            maximum_array_elements: 1000,
        },
    )
    .unwrap()
}

pub(super) fn check_array_values(
    values: &bessemer::tessellation::DynamicArrayValues,
    width: u32,
    index: usize,
    edited: usize,
) {
    let count = 1 + index + edited;
    assert_eq!(values.element_size.value(), width);
    assert_eq!(values.initial_capacity, 17 + edited as u32);
    assert_eq!(values.growth_step, 23 + edited as u32);
    assert_eq!(values.count, count as u32);
    let mut payload: Vec<u8> = (0..count).flat_map(|_| 0..width as u8).collect();
    if edited != 0 {
        *payload.last_mut().unwrap() = 255;
    }
    assert_eq!(values.payload, payload);
}

const FACETS: [(u32, [[&[u8]; 2]; 2]); 8] = [
    (
        596,
        [
            [
                include_bytes!("../fixtures/display-lists/legacy-fontless-facet-596-0-0.bin"),
                include_bytes!("../fixtures/display-lists/legacy-fontless-facet-596-0-1.bin"),
            ],
            [
                include_bytes!("../fixtures/display-lists/legacy-fontless-facet-596-1-0.bin"),
                include_bytes!("../fixtures/display-lists/legacy-fontless-facet-596-1-1.bin"),
            ],
        ],
    ),
    (
        597,
        [
            [
                include_bytes!("../fixtures/display-lists/legacy-fontless-facet-597-0-0.bin"),
                include_bytes!("../fixtures/display-lists/legacy-fontless-facet-597-0-1.bin"),
            ],
            [
                include_bytes!("../fixtures/display-lists/legacy-fontless-facet-597-1-0.bin"),
                include_bytes!("../fixtures/display-lists/legacy-fontless-facet-597-1-1.bin"),
            ],
        ],
    ),
    (
        716,
        [
            [
                include_bytes!("../fixtures/display-lists/legacy-fontless-facet-716-0-0.bin"),
                include_bytes!("../fixtures/display-lists/legacy-fontless-facet-716-0-1.bin"),
            ],
            [
                include_bytes!("../fixtures/display-lists/legacy-fontless-facet-716-1-0.bin"),
                include_bytes!("../fixtures/display-lists/legacy-fontless-facet-716-1-1.bin"),
            ],
        ],
    ),
    (
        717,
        [
            [
                include_bytes!("../fixtures/display-lists/legacy-fontless-facet-717-0-0.bin"),
                include_bytes!("../fixtures/display-lists/legacy-fontless-facet-717-0-1.bin"),
            ],
            [
                include_bytes!("../fixtures/display-lists/legacy-fontless-facet-717-1-0.bin"),
                include_bytes!("../fixtures/display-lists/legacy-fontless-facet-717-1-1.bin"),
            ],
        ],
    ),
    (
        760,
        [
            [
                include_bytes!("../fixtures/display-lists/legacy-fontless-facet-760-0-0.bin"),
                include_bytes!("../fixtures/display-lists/legacy-fontless-facet-760-0-1.bin"),
            ],
            [
                include_bytes!("../fixtures/display-lists/legacy-fontless-facet-760-1-0.bin"),
                include_bytes!("../fixtures/display-lists/legacy-fontless-facet-760-1-1.bin"),
            ],
        ],
    ),
    (
        761,
        [
            [
                include_bytes!("../fixtures/display-lists/legacy-fontless-facet-761-0-0.bin"),
                include_bytes!("../fixtures/display-lists/legacy-fontless-facet-761-0-1.bin"),
            ],
            [
                include_bytes!("../fixtures/display-lists/legacy-fontless-facet-761-1-0.bin"),
                include_bytes!("../fixtures/display-lists/legacy-fontless-facet-761-1-1.bin"),
            ],
        ],
    ),
    (
        766,
        [
            [
                include_bytes!("../fixtures/display-lists/legacy-fontless-facet-766-0-0.bin"),
                include_bytes!("../fixtures/display-lists/legacy-fontless-facet-766-0-1.bin"),
            ],
            [
                include_bytes!("../fixtures/display-lists/legacy-fontless-facet-766-1-0.bin"),
                include_bytes!("../fixtures/display-lists/legacy-fontless-facet-766-1-1.bin"),
            ],
        ],
    ),
    (
        767,
        [
            [
                include_bytes!("../fixtures/display-lists/legacy-fontless-facet-767-0-0.bin"),
                include_bytes!("../fixtures/display-lists/legacy-fontless-facet-767-0-1.bin"),
            ],
            [
                include_bytes!("../fixtures/display-lists/legacy-fontless-facet-767-1-0.bin"),
                include_bytes!("../fixtures/display-lists/legacy-fontless-facet-767-1-1.bin"),
            ],
        ],
    ),
];
