use super::*;

type ReplayCase = (InspectionRecords, [InspectionRelationshipViews; 2]);

fn replay_cases() -> Vec<ReplayCase> {
    let projects = [
        project(),
        project_from_xml(include_bytes!(
            "../fixtures/standalone-inspection/historical.xml"
        )),
        package_project(
            "<PackageOptions><RelationshipsDictionary>a&amp;<!--c--><item><key><string>K</string></key></item></RelationshipsDictionary></PackageOptions>",
        ),
    ];
    projects
        .into_iter()
        .map(|project| {
            let retained = views_from(&project, InspectionRelationshipPolicy::RetainedNodes);
            let elements = views_from(&project, InspectionRelationshipPolicy::ElementsOnly);
            (project, [retained, elements])
        })
        .collect()
}

#[test]
fn eight_workers_replay_mixed_policies_against_complete_serial_results() {
    let cases = replay_cases();
    std::thread::scope(|scope| {
        for worker in 0..8 {
            let cases = &cases;
            scope.spawn(move || replay_worker(cases, worker));
        }
    });
    for (project, serial) in &cases {
        assert_eq!(
            project.envelope().xml_bytes(),
            serial[0].source.envelope().xml_bytes()
        );
    }
}

fn replay_worker(cases: &[ReplayCase], worker: usize) {
    for iteration in 0..16 {
        let policy_index = (iteration + worker) % 2;
        for (project, serial) in cases {
            let actual = views_from(project, serial[policy_index].policy);
            assert_same_views(&actual, &serial[policy_index]);
        }
    }
}

fn assert_same_views(actual: &InspectionRelationshipViews, expected: &InspectionRelationshipViews) {
    assert_eq!(actual.policy, expected.policy);
    assert_eq!(actual.sheet, expected.sheet);
    assert_eq!(actual.attempts, expected.attempts);
    assert_eq!(actual.entries, expected.entries);
    assert_eq!(actual.writer_entries, expected.writer_entries);
    assert_eq!(actual.completion, expected.completion);
    assert_eq!(actual.coverage, expected.coverage);
    assert_eq!(actual.history, expected.history);
    assert_eq!(actual.diagnostics, expected.diagnostics);
    assert_eq!(
        actual.source.envelope().xml_bytes(),
        expected.source.envelope().xml_bytes()
    );
    assert_eq!(
        actual.source.envelope().to_bytes().unwrap(),
        expected.source.envelope().to_bytes().unwrap()
    );
}
