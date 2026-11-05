use super::*;

// Eight elements: seven start/end pairs and one empty element; one document event.
const COUNTED: &[u8] = br#"<i:SOLIDWORKSINSPECTION xmlns:i="http://www.extensiblecad.com"><INSPECTION_SHEET><PackageOptions><IXCharacteristics><IXGroupProxy><IXCharacteristic><Measurements><Measurement Name="N" Value=""/></Measurements></IXCharacteristic></IXGroupProxy></IXCharacteristics></PackageOptions></INSPECTION_SHEET></i:SOLIDWORKSINSPECTION>"#;

fn exact_limits() -> XmlLimits {
    XmlLimits {
        maximum_bytes: (COUNTED.len() / 16 + 1) * 16,
        maximum_nodes: 16,
        maximum_attributes: 3,
        maximum_depth: 8,
    }
}

fn counted_project(profile: InspectionProfile) -> InspectionRecords {
    let envelope = InspectionEnvelope::from_xml(profile, COUNTED, exact_limits()).unwrap();
    InspectionRecords::from_envelope(&envelope).unwrap()
}

#[test]
fn independent_exact_limits_accept_both_profiles_and_one_below_rejects_each_axis() {
    for profile in [
        InspectionProfile::Current2020,
        InspectionProfile::Historical2015,
    ] {
        let source = counted_project(profile);
        assert_eq!(source.coverage().elements, 8);
        assert_eq!(source.coverage().records, 4);
        for axis in 0..4 {
            let mut limits = exact_limits();
            match axis {
                0 => limits.maximum_bytes -= 1,
                1 => limits.maximum_nodes -= 1,
                2 => limits.maximum_attributes -= 1,
                _ => limits.maximum_depth -= 1,
            }
            let error = InspectionEnvelope::from_xml(profile, COUNTED, limits).unwrap_err();
            assert!(error.cause.contains("expected"));
        }
        assert_eq!(source.envelope().xml_bytes(), COUNTED);
    }
}

#[test]
fn bounded_failed_edit_keeps_all_input_identities_and_successful_noop() {
    for profile in [
        InspectionProfile::Current2020,
        InspectionProfile::Historical2015,
    ] {
        let source = counted_project(profile);
        let id = &measurement(&source).id;
        let original = source.envelope().to_bytes().unwrap();
        assert!(
            source
                .edit_attribute(id, "Value", &"x".repeat(100))
                .is_err()
        );
        assert!(source.record(id).is_ok());
        let noop = source.edit_attribute(id, "Value", "").unwrap();
        assert_eq!(noop.completion, InspectionRecordCompletion::Unchanged);
        assert_eq!(noop.project.record(id).unwrap(), source.record(id).unwrap());
        assert_eq!(noop.project.envelope().to_bytes().unwrap(), original);
        assert_eq!(source.envelope().xml_bytes(), COUNTED);
    }
}

#[test]
fn every_proper_prefix_and_unexplained_suffix_rejects_without_partial_index() {
    for end in 0..COUNTED.len() {
        assert!(
            InspectionEnvelope::from_xml(InspectionProfile::Current2020, &COUNTED[..end], LIMITS)
                .is_err()
        );
    }
    let mut trailing = COUNTED.to_vec();
    trailing.extend_from_slice(b"unexpected");
    assert!(
        InspectionEnvelope::from_xml(InspectionProfile::Current2020, &trailing, LIMITS).is_err()
    );
    assert_eq!(
        counted_project(InspectionProfile::Current2020)
            .envelope()
            .xml_bytes(),
        COUNTED
    );
}

#[test]
fn caller_mutation_and_concurrent_projection_mutation_cannot_modify_snapshots() {
    let mut input = OWNED.to_vec();
    let source = project(&input, InspectionProfile::Current2020);
    input.fill(0);
    let characteristic = source
        .records()
        .iter()
        .find(|r| r.kind == InspectionRecordKind::Characteristic)
        .unwrap();
    let expected = source.loaded_measurements(&characteristic.id).unwrap();
    std::thread::scope(|scope| {
        let jobs: Vec<_> = (0..8)
            .map(|_| scope.spawn(|| mutate_local_projection(&source, &characteristic.id)))
            .collect();
        for job in jobs {
            assert_eq!(job.join().unwrap(), expected);
        }
    });
    assert_eq!(source.envelope().xml_bytes(), OWNED);
}

fn mutate_local_projection(
    source: &InspectionRecords,
    id: &InspectionRecordId,
) -> Vec<InspectionLoadedMeasurement> {
    let mut local = source.loaded_measurements(id).unwrap();
    local[0].value.push_str("local mutation");
    source.loaded_measurements(id).unwrap()
}
