//! Immutable publication, exact edit/revert behavior, and concurrent reproducibility.
#[path = "support/tessellation.rs"]
mod support;
use bessemer::tessellation::*;
use support::*;

#[test]
fn published_types_are_send_and_sync() {
    fn shared<T: Send + Sync>() {}
    shared::<TessellationDirectory>();
    shared::<FaceTessellationBlock>();
    shared::<FacetArchive>();
    shared::<TessellationContext>();
    shared::<TessellationLimits>();
    shared::<FacetArchiveMode>();
    shared::<ArrayElementSize>();
    shared::<DynamicArray>();
    shared::<DynamicArrayValues>();
    shared::<PartComponentInfo>();
    shared::<TessellationDirectoryContents>();
    shared::<TessellationNumber>();
    shared::<ComponentTessellationNumbers>();
    shared::<FacetBlockComponent>();
    shared::<FaceTessellationBlockContents>();
    shared::<FacetContainer>();
    shared::<FacetStyle>();
    shared::<FacetVertexLayout>();
    shared::<FacetEdgeArrays>();
    shared::<FacetLegacyTriangles>();
    shared::<FacetConeCull>();
    shared::<FacetTextureTessellation>();
    shared::<FacetSurface>();
    shared::<FacetLateStamps>();
    shared::<FacetTriangleIndices>();
    shared::<FacetCurvature>();
    shared::<FacetDisplayColor>();
    shared::<FacetString>();
    shared::<ByteFlaggedArray>();
    shared::<FacetEdgeInfo>();
    shared::<FacetEdgePoint>();
    shared::<FacetEdgePoints>();
    shared::<EdgeVertexPairs>();
    shared::<Flagged<DynamicArray>>();
    shared::<ArrayPair<DynamicArray>>();
    shared::<BlockLinkCoverage>();
    shared::<StoredFloat>();
    shared::<StoredDouble>();
    shared::<FacetBounds>();
    shared::<ArchiveString>();
}

#[test]
fn source_mutation_success_failure_and_revert_are_isolated() {
    let mut bytes = MODERN.to_vec();
    let source = parse(&bytes);
    bytes.fill(0);
    let original = source.contents().clone();
    let mut contents = original.clone();
    contents.container_processed = 1234;
    let edited = source.with_contents(contents.clone()).unwrap();
    contents.container_processed = 5678;
    assert_eq!(source.to_bytes().unwrap(), MODERN);
    assert_eq!(edited.contents().container_processed, 1234);
    assert_eq!(
        parse(&edited.to_bytes().unwrap()).contents(),
        edited.contents()
    );
    let mut invalid = original.clone();
    invalid.face_string.as_mut().unwrap().text = None;
    assert!(source.with_contents(invalid).is_err());
    assert_eq!(source.to_bytes().unwrap(), MODERN);
    assert_eq!(
        edited.with_contents(original).unwrap().to_bytes().unwrap(),
        MODERN
    );
}

#[test]
fn concurrent_reads_and_edits_equal_serial_results_byte_for_byte() {
    let source = parse(MODERN);
    let expected: Vec<Vec<u8>> = (0..8).map(|index| edit(&source, index)).collect();
    for _ in 0..8 {
        std::thread::scope(|scope| {
            let tasks: Vec<_> = (0..8)
                .map(|index| {
                    let source = &source;
                    scope.spawn(move || edit(source, index))
                })
                .collect();
            for (task, expected) in tasks.into_iter().zip(&expected) {
                assert_eq!(task.join().unwrap(), *expected);
            }
        });
    }
    assert_eq!(source.to_bytes().unwrap(), MODERN);
}

fn edit(source: &FacetArchive, value: i32) -> Vec<u8> {
    let mut contents = source.contents().clone();
    contents.container_processed = value;
    source.with_contents(contents).unwrap().to_bytes().unwrap()
}
