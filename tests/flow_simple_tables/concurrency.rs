use super::*;

#[test]
fn eight_workers_match_complete_serial_edits_for_all_four_profiles_after_input_mutation() {
    for case in cases()
        .into_iter()
        .filter(|case| !case.frame.compressed && selected(case))
    {
        check_threads(case);
    }
}

fn selected(case: &SimpleCase) -> bool {
    ["parallel-v0-2", "leaf-pairs-2", "five-v4-2", "bytes-v1-3"].contains(&case.frame.name.as_str())
}

fn check_threads(mut case: SimpleCase) {
    let source = read_case(&case);
    case.frame.stored.fill(0);
    let expected = source.decoded().to_vec();
    let serial = edits::first_edit_bytes(&source).unwrap();
    std::thread::scope(|scope| {
        for _ in 0..8 {
            let source = &source;
            let expected = &expected;
            let serial = &serial;
            scope.spawn(move || {
                let edited = edits::first_edit_bytes(source).unwrap();
                assert_eq!(&edited, serial);
                assert_eq!(source.decoded(), expected);
            });
        }
    });
}
