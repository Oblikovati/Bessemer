use super::history_projection::{history_view, project_history_case};

const EXPECTED: &[u8] = include_bytes!("../fixtures/simulation-inertial-loads/nested-history.bin");

fn project_nested_case(cursor: &mut &[u8], output: &mut Vec<u8>) {
    let edit = project_history_case(cursor, output);
    super::load_history::project_nested(cursor, output, &history_view(&edit));
}

fn project_nested_packet(cursor: &[u8], expected_count: usize) -> Vec<u8> {
    super::load_projection::project_packet(cursor, expected_count, project_nested_case)
}

#[test]
fn all_owned_nested_copy_relations_match_every_independent_physical_source_occurrence() {
    assert_eq!(project_nested_packet(EXPECTED, 135), EXPECTED);
}

#[test]
#[ignore = "Requires explicitly extracted complete native nested-history expectations"]
fn all_native_nested_copy_relations_match_source_with_optional_export() {
    let path = std::env::var("BESSEMER_INERTIAL_NATIVE_NESTED_HISTORY_INPUT")
        .expect("BESSEMER_INERTIAL_NATIVE_NESTED_HISTORY_INPUT=<unset>; expected explicit nested-history packet path");
    let native = std::fs::read(&path).unwrap_or_else(|error| {
        panic!("input {path:?}: {error}; expected readable complete native nested history")
    });
    let mut output = project_nested_packet(EXPECTED, 135);
    output.extend(project_nested_packet(&native, 18));
    if let Some(path) = std::env::var_os("BESSEMER_INERTIAL_NESTED_HISTORY_OUTPUT") {
        std::fs::write(&path, output).unwrap_or_else(|error| {
            panic!("output {path:?}: {error}; expected writable explicit nested-history output")
        });
    }
}
