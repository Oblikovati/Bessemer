use super::*;
pub(super) fn scalar(slot7: u64, slot10: u64, query: &fixture::ScalarQuery) {
    let magnitude = slot7 & 0x7fffffffffffffff;
    let nan =
        magnitude & 0x7ff0000000000000 == 0x7ff0000000000000 && magnitude & 0x000fffffffffffff != 0;
    let status = if nan {
        None
    } else if slot7 >> 63 != 0 || slot7 <= 0x406fa00000000000 {
        Some(-2)
    } else if slot7 > 0x4081a80000000000 {
        Some(-1)
    } else {
        None
    };
    assert_eq!(query.status, status, "{slot7:016x}");
    assert_eq!(query.call, status.is_none());
    assert_eq!(query.nonzero, slot10 & 0x7fffffffffffffff != 0);
    assert!(!query.faults && !query.virtual_result);
}
pub(super) fn group(case: &ConditionalCase) {
    assert_eq!(
        (
            case.identifier,
            case.version,
            case.scale,
            case.outer_version
        ),
        (155, 0, 1, None)
    );
    assert!(case.complete && !case.runtime && !case.engineering);
    assert_eq!(case.stored, case.decoded);
    rebuild_fields(case);
    scopes_and_counts(case);
}
fn rebuild_fields(case: &ConditionalCase) {
    let mut rebuilt = Vec::new();
    let mut paths = BTreeSet::new();
    for field in &case.fields {
        assert!(paths.insert(&field.path));
        assert_eq!(field.offset as usize, rebuilt.len());
        assert_eq!(field.raw.len(), field.count as usize * field.width());
        rebuilt.extend_from_slice(&field.raw);
    }
    assert_eq!(rebuilt, case.decoded);
}

fn scopes_and_counts(case: &ConditionalCase) {
    let mut paths = BTreeSet::new();
    for scope in &case.scopes {
        assert!(paths.insert(&scope.path));
        assert!(scope.start <= scope.end && scope.end as usize <= case.decoded.len());
    }
    assert_eq!(case.scopes[0].path, "[]");
    assert_eq!(
        (case.scopes[0].start, case.scopes[0].end as usize),
        (0, case.decoded.len())
    );
    count_locations(case);
}
fn count_locations(case: &ConditionalCase) {
    for offset in &case.counts {
        let count = i32::from_le_bytes(
            case.decoded[*offset as usize..*offset as usize + 4]
                .try_into()
                .unwrap(),
        );
        assert!(count >= 0);
    }
    for (path, flag) in &case.projections {
        assert!(path.starts_with('['));
        if path.contains("native_inverse_map_reproduced") {
            assert!(!flag);
        }
    }
}
pub(super) fn selection(case: &fixture::SelectionCase) {
    let preferred = [113, 155].into_iter().find_map(|identifier| {
        case.records
            .iter()
            .rev()
            .find(|record| record.0 == identifier)
    });
    let Some((identifier, raw)) = preferred else {
        assert!(case.completed && case.identifier.is_none() && case.cause.is_empty());
        return;
    };
    let valid = if *identifier == 113 {
        raw == &[0; 4]
    } else {
        raw.len() == 24
    };
    assert_eq!(case.completed, valid);
    assert_eq!(case.identifier, valid.then_some(*identifier));
    assert_eq!(case.cause.is_empty(), valid);
}
