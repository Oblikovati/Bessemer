use super::*;

const LOAD: &[u8] = include_bytes!("../fixtures/inspection-records/load-projections.xml");
const OWNED_MEASUREMENTS: [(&str, &str); 5] = [
    ("same", ""),
    ("same", "Pass"),
    ("same", ""),
    ("qualified", "extension"),
    ("", ""),
];
const OWNED_PROPERTIES: [(&str, &str); 5] = [
    ("P", "1"),
    ("P", "2"),
    ("P", ""),
    ("", ""),
    ("Q", "extension"),
];

fn characteristic(project: &InspectionRecords) -> &InspectionRecord {
    project
        .records()
        .iter()
        .find(|record| record.kind == InspectionRecordKind::Characteristic)
        .unwrap()
}

#[test]
fn native_projection_admits_all_direct_element_names_with_exact_empty_defaults() {
    let project = project(OWNED, InspectionProfile::Current2020);
    let id = &characteristic(&project).id;
    let loaded = project.loaded_measurements(id).unwrap();
    check_owned_measurements(&loaded);
    check_owned_properties(&project.loaded_custom_properties(id).unwrap());
    assert_eq!(project.envelope().xml_bytes(), OWNED);
}

fn check_owned_measurements(loaded: &[InspectionLoadedMeasurement]) {
    let pairs: Vec<_> = loaded
        .iter()
        .map(|m| (m.name.as_str(), m.value.as_str()))
        .collect();
    assert_eq!(pairs, OWNED_MEASUREMENTS);
    assert!(loaded.iter().all(|m| m.part_name.is_empty()
        && m.measurement_type.is_empty()
        && m.creation_time.is_empty()));
    assert_eq!(
        loaded[3].element_name.namespace.as_deref(),
        Some("urn:extension")
    );
}

fn check_owned_properties(properties: &[InspectionLoadedCustomProperty]) {
    let pairs: Vec<_> = properties
        .iter()
        .map(|p| (p.name.as_str(), p.value.as_str()))
        .collect();
    assert_eq!(pairs, OWNED_PROPERTIES);
    assert_eq!(
        properties[4].element_name.namespace.as_deref(),
        Some("urn:extension")
    );
}

#[test]
fn repeated_exact_containers_keep_order_and_skip_nodes_and_nested_descendants() {
    let project = project(LOAD, InspectionProfile::Current2020);
    let loaded = project
        .loaded_measurements(&characteristic(&project).id)
        .unwrap();
    check_fixture_measurements(&loaded);
    check_fixture_properties(&project);
    check_empty_characteristic(&project);
    assert_eq!(project.envelope().xml_bytes(), LOAD);
}

fn check_fixture_measurements(loaded: &[InspectionLoadedMeasurement]) {
    assert_eq!(loaded.len(), 4);
    assert_eq!(
        (&*loaded[0].name, &*loaded[0].value, &*loaded[0].part_name),
        ("N", "Pass", "P")
    );
    assert_eq!(
        (&*loaded[0].measurement_type, &*loaded[0].creation_time),
        ("custom", "arbitrary")
    );
    assert_eq!((&*loaded[1].name, &*loaded[1].value), ("", ""));
    assert_eq!((&*loaded[2].name, &*loaded[2].value), ("W", ""));
    assert_eq!((&*loaded[3].name, &*loaded[3].value), ("last", "-0.00"));
}

fn check_fixture_properties(project: &InspectionRecords) {
    let properties = project
        .loaded_custom_properties(&characteristic(project).id)
        .unwrap();
    let pairs: Vec<_> = properties
        .iter()
        .map(|p| (p.name.as_str(), p.value.as_str()))
        .collect();
    assert_eq!(pairs, [("P", "1"), ("P", "2"), ("", ""), ("W", "")]);
}

fn check_empty_characteristic(project: &InspectionRecords) {
    let empty = project
        .records()
        .iter()
        .filter(|record| record.kind == InspectionRecordKind::Characteristic)
        .nth(1)
        .unwrap();
    assert!(project.loaded_measurements(&empty.id).unwrap().is_empty());
    assert!(
        project
            .loaded_custom_properties(&empty.id)
            .unwrap()
            .is_empty()
    );
}

#[test]
fn native_load_views_reject_wrong_roles_foreign_and_stale_occurrences() {
    let source = project(OWNED, InspectionProfile::Current2020);
    let wrong = &measurement(&source).id;
    assert!(source.loaded_measurements(wrong).is_err());
    assert!(source.loaded_custom_properties(wrong).is_err());
    let foreign = project(OWNED, InspectionProfile::Current2020);
    assert!(
        source
            .loaded_measurements(&characteristic(&foreign).id)
            .is_err()
    );
    assert!(
        source
            .loaded_custom_properties(&characteristic(&foreign).id)
            .is_err()
    );
    let id = &characteristic(&source).id;
    let changed = source.edit_attribute(id, "Nominal", "changed").unwrap();
    assert!(changed.project.loaded_measurements(id).is_err());
    assert!(changed.project.loaded_custom_properties(id).is_err());
    assert_eq!(source.envelope().xml_bytes(), OWNED);
}

#[test]
fn tutorial_has_twelve_empty_measurements_and_120_ordered_property_inputs() {
    assert_load_types_send_sync();
    let raw = include_bytes!("../fixtures/standalone-inspection/historical.xml");
    let source = project(raw, InspectionProfile::Historical2015);
    let mut measurements = 0;
    let mut properties = 0;
    for record in source
        .records()
        .iter()
        .filter(|r| r.kind == InspectionRecordKind::Characteristic)
    {
        let loaded = source.loaded_measurements(&record.id).unwrap();
        assert!(
            loaded
                .iter()
                .all(|m| m.value.is_empty() && m.measurement_type == "UserDefined")
        );
        measurements += loaded.len();
        properties += source.loaded_custom_properties(&record.id).unwrap().len();
    }
    assert_eq!((measurements, properties), (12, 120));
}

fn assert_load_types_send_sync() {
    fn send_sync<T: Send + Sync>() {}
    send_sync::<InspectionLoadedMeasurement>();
    send_sync::<InspectionLoadedCustomProperty>();
}
