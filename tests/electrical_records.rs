//! Complete native and owned Electrical catalog/drawing graph comparisons.
#[path = "electrical_records/fixture.rs"]
mod fixture;
#[path = "electrical_records/projection.rs"]
mod projection;
#[path = "electrical_records/rejections.rs"]
mod rejections;
#[path = "support/sqlite_wire.rs"]
mod sqlite_wire;
use bessemer::electrical::*;
use bessemer::sqlite_image::SqliteValue;

#[test]
fn every_native_and_owned_complete_graph_matches_the_original_reference() {
    let cases = fixture::cases();
    assert_eq!(cases.len(), 7);
    for case in cases {
        let archive = fixture::archive(&case.raw);
        let view = archive.project_records(fixture::project_limits()).unwrap();
        projection::assert_matches(&view, &case.expected, &case.name);
        assert_eq!(view.source().to_bytes(), case.raw);
        assert_eq!(view.coverage().projects, view.projects().len());
        assert_eq!(
            view.coverage().mode,
            ElectricalProjectCheckMode::LiteralProjectLinks
        );
        assert_eq!(view.diagnostics().len(), 5);
    }
}

#[test]
fn duplicate_catalogs_databases_and_drawings_are_distinct_complete_occurrences() {
    let case = fixture::cases().remove(1);
    let view = fixture::archive(&case.raw)
        .project_records(fixture::project_limits())
        .unwrap();
    assert_eq!(view.projects().len(), 12);
    assert_eq!(view.coverage().checked_images, 5);
    assert_eq!(view.coverage().database_links, 10);
    assert_eq!(view.coverage().file_links, 50);
    assert_eq!(view.coverage().checked_tables, 2 * 2 + 3 * 6);
}

#[test]
fn all_drawing_matches_remain_literal_including_null_and_nontext_names() {
    let case = fixture::cases().remove(1);
    let view = fixture::archive(&case.raw)
        .project_records(fixture::project_limits())
        .unwrap();
    let project = &view.projects()[0];
    assert_eq!(project.databases.len(), 2);
    assert_ne!(project.databases[0].member, project.databases[1].member);
    let links = &project.databases[0].files;
    assert_eq!(links[0].members.len(), 2);
    assert!(links[1].members.is_empty());
    assert_ne!(links[0].members[0], links[0].members[1]);
    assert!(links[2].member_name.is_none());
    assert!(links[3].member_name.is_none());
}

#[test]
fn parent_path_components_match_only_their_literal_members() {
    let case = fixture::cases().remove(1);
    let view = fixture::archive(&case.raw)
        .project_records(fixture::project_limits())
        .unwrap();
    let link = &view.projects()[0].databases[0].files[4];
    assert_eq!(
        link.member_name.as_deref(),
        Some("Projects/2/Drawings/../literal.ewg")
    );
    assert_eq!(link.members.len(), 1);
}

#[test]
fn null_nontext_unknown_fields_and_distinct_versions_are_not_coerced() {
    let case = fixture::cases().remove(1);
    let view = fixture::archive(&case.raw)
        .project_records(fixture::project_limits())
        .unwrap();
    assert!(view.projects()[1].databases.is_empty());
    assert!(view.projects()[2].databases.is_empty());
    assert_eq!(
        view.projects()[1].record.value("pro_directory"),
        Some(&SqliteValue::Null)
    );
    assert_eq!(
        view.projects()[2].record.value("pro_directory"),
        Some(&SqliteValue::Integer(17))
    );
    assert_eq!(view.projects()[2].record.ordinal(), 2);
    assert!(view.projects()[0].record.value("PRO_ID").is_none());
}

#[test]
fn versions_and_unknown_fields_are_complete_and_separate() {
    let case = fixture::cases().remove(1);
    let view = fixture::archive(&case.raw)
        .project_records(fixture::project_limits())
        .unwrap();
    assert_eq!(
        view.projects()[0].catalog_versions.columns()[0],
        "ver_projects"
    );
    assert_eq!(
        view.projects()[0].databases[0].versions.columns()[0],
        "ver_projectdata"
    );
    assert_eq!(
        view.projects()[0].databases[0].files[0]
            .record
            .value("unknown"),
        Some(&SqliteValue::Blob([0, 255].into()))
    );
}

#[test]
fn empty_catalog_projection_and_every_build_bound_constant_remain_explicit() {
    let cases = fixture::cases();
    let empty = fixture::archive(&cases[0].raw)
        .project_records(fixture::project_limits())
        .unwrap();
    assert!(empty.projects().is_empty());
    assert_eq!(empty.coverage().checked_images, 0);
    let view = fixture::archive(&cases[2].raw)
        .project_records(fixture::project_limits())
        .unwrap();
    let files = &view.projects()[0].databases[0].files;
    assert_eq!(files.len(), 28);
    assert_eq!(files[0].file_type_constant(), Some("kFileFolder"));
    assert_eq!(files[22].file_type_constant(), Some("kFileMax"));
    assert!(
        files[23..]
            .iter()
            .all(|file| file.file_type_constant().is_none())
    );
    projection::assert_matches(&view, &cases[2].expected, "constants");
}

#[test]
fn immutable_results_and_revision_bound_members_isolate_source_mutation_and_eight_workers() {
    verify_project_send_sync();
    let mut case = fixture::cases().remove(1);
    let archive = fixture::archive(&case.raw);
    let view = archive.project_records(fixture::project_limits()).unwrap();
    case.raw.fill(0);
    std::thread::scope(|scope| {
        let workers: Vec<_> = (0..8)
            .map(|_| {
                scope.spawn(|| {
                    projection::assert_matches(&view.clone(), &case.expected, "clone");
                    let replay = archive.project_records(fixture::project_limits()).unwrap();
                    projection::assert_matches(&replay, &case.expected, "replay");
                })
            })
            .collect();
        for worker in workers {
            worker.join().unwrap();
        }
    });
}

fn verify_project_send_sync() {
    assert_send_sync::<ElectricalProjectView>();
    assert_send_sync::<ElectricalProject>();
    assert_send_sync::<ElectricalProjectDatabase>();
    assert_send_sync::<ElectricalFileLink>();
    assert_send_sync::<ElectricalRecord>();
    assert_send_sync::<ElectricalProjectCoverage>();
    assert_send_sync::<ElectricalProjectLimits>();
    assert_send_sync::<ElectricalProjectCheckMode>();
    assert_send_sync::<ElectricalProjectDiagnostic>();
}

fn assert_send_sync<T: Send + Sync>() {}
