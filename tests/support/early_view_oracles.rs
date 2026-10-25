use super::*;
use crate::complete_cases::framing;

struct ViewOracle {
    version: u32,
    kind: DisplayDocumentKind,
    scenario: u8,
    expected: [&'static [u8]; 3],
}

fn oracle_blob(remaining: &mut &'static [u8]) -> &'static [u8] {
    let length = framing::take_u32(remaining).unwrap() as usize;
    framing::take(remaining, length).unwrap()
}

fn read_oracle(remaining: &mut &'static [u8]) -> ViewOracle {
    let version = framing::take_u32(remaining).unwrap();
    let kind = if framing::take(remaining, 1).unwrap()[0] == 0 {
        DisplayDocumentKind::Part
    } else {
        DisplayDocumentKind::Assembly
    };
    let scenario = framing::take(remaining, 1).unwrap()[0];
    ViewOracle {
        version,
        kind,
        scenario,
        expected: [
            oracle_blob(remaining),
            oracle_blob(remaining),
            oracle_blob(remaining),
        ],
    }
}

fn oracles() -> Vec<ViewOracle> {
    let mut remaining: &'static [u8] =
        include_bytes!("../fixtures/display-lists/early-root-view-oracles.bin");
    let count = framing::take_u32(&mut remaining).unwrap();
    framing::require_count(count, remaining.len(), 18).unwrap();
    let records = (0..count).map(|_| read_oracle(&mut remaining)).collect();
    framing::require_finished(remaining).unwrap();
    records
}

fn collection_traces(archive: &DisplayArchive) -> [Vec<u8>; 3] {
    let get = |name: &str| {
        &archive
            .contents()
            .fields
            .iter()
            .find(|field| field.path.as_ref() == name)
            .unwrap()
            .value
    };
    let DisplayFieldValue::NamedViews(named) = get("root.named_view_transform") else {
        panic!("named views")
    };
    let DisplayFieldValue::SpatialViews(spatial) = get("root.view_3d_transform") else {
        panic!("3D views")
    };
    let DisplayFieldValue::AnnotationViews(annotation) = get("root.annotation_view_transform")
    else {
        panic!("annotation views")
    };
    let limits = DisplayViewLimits {
        maximum_bytes: LIMITS.maximum_bytes,
        maximum_views: LIMITS.maximum_entries,
        maximum_nested_entries: LIMITS.maximum_entries,
    };
    let version = archive.context().version;
    [
        named_trace::trace(&NamedViews::new(named.clone(), version, limits).unwrap()),
        spatial_trace::trace(&SpatialViews::new(spatial.clone(), version, limits).unwrap()),
        annotation_trace::trace(
            &AnnotationViews::new(annotation.clone(), version, limits).unwrap(),
        ),
    ]
}

#[test]
fn nested_named_spatial_and_annotation_views_match_independent_full_root_oracles() {
    let records = oracles();
    assert_eq!(records.len(), 76);
    let mut fields = 0;
    for oracle in records {
        let case = selected(oracle.version, oracle.kind, oracle.scenario);
        let archive = parsed(case);
        let actual = collection_traces(&archive);
        for (actual, expected) in actual.iter().zip(oracle.expected) {
            assert_eq!(
                actual, expected,
                "{} {:?} {}",
                oracle.version, oracle.kind, oracle.scenario
            );
            fields += expected.iter().filter(|byte| **byte == b'\n').count();
        }
    }
    assert_eq!(fields, 28338);
}
