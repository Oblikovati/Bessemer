use super::*;

fn optional_tail_prefix(body: &SimulationShellComposite, size: usize) -> bool {
    body.profile() == SimulationShellCompositeProfile::ExtraWriter
        && body
            .fields()
            .iter()
            .any(|field| field.path.as_ref() == "persistid.blob_count")
        && size == 64
}

#[test]
fn every_shorter_extent_respects_optional_writer_tail_and_all_trailing_bytes_fail() {
    let mut counts = (0, 0);
    for body in contracts::owned_composites() {
        let raw = body.to_bytes();
        for size in 0..raw.len() {
            let expected = check_prefix(&body, &raw, size);
            counts.0 += 1;
            counts.1 += usize::from(expected);
        }
        let trailing = [raw.as_slice(), &[0]].concat();
        assert!(
            SimulationShellComposite::from_bytes(&trailing, body.version(), body.profile(), LIMITS)
                .is_err()
        );
    }
    assert_eq!(counts, (80300, 80));
}

fn check_prefix(body: &SimulationShellComposite, raw: &[u8], size: usize) -> bool {
    let parsed =
        SimulationShellComposite::from_bytes(&raw[..size], body.version(), body.profile(), LIMITS);
    let expected = optional_tail_prefix(body, size);
    assert_eq!(
        parsed.is_ok(),
        expected,
        "{:?} {} {size}",
        body.profile(),
        raw.len()
    );
    expected
}

#[test]
fn historical_four_byte_extra_stays_invalid_in_both_explicit_profiles() {
    let version = SimulationPersistVersion::new(840030).unwrap();
    for profile in [
        SimulationShellCompositeProfile::ExtraReader,
        SimulationShellCompositeProfile::ExtraWriter,
    ] {
        let error =
            SimulationShellComposite::from_bytes(&1_i32.to_le_bytes(), version, profile, LIMITS)
                .unwrap_err();
        assert_eq!(error.class, bessemer::FailureClass::Invalid);
    }
}

#[test]
fn flip_turn_fixed_count_cap_uses_sufficient_complete_records_and_permissive_caller_limit() {
    let version = SimulationPersistVersion::new(0).unwrap();
    let limits = SimulationShellCompositeLimits {
        maximum_entries: 100001,
        ..LIMITS
    };
    for count in [100000_i32, 100001] {
        let mut raw = count.to_le_bytes().to_vec();
        raw.resize(4 + count as usize * 16, 0);
        for profile in [
            SimulationShellCompositeProfile::FlipTurnReader,
            SimulationShellCompositeProfile::FlipTurnWriter,
        ] {
            let result = SimulationShellComposite::from_bytes(&raw, version, profile, limits);
            assert_eq!(result.is_ok(), count == 100000);
            if let Ok(body) = result {
                assert_eq!(body.to_bytes(), raw);
                assert_eq!(body.coverage().declared_entries, 100000);
            }
        }
    }
}

#[test]
fn persist_blob_fixed_count_cap_uses_sufficient_payload_in_both_extra_profiles() {
    let version = SimulationPersistVersion::new(840001).unwrap();
    for count in [1000000_i32, 1000001] {
        let mut raw = vec![0; 64];
        raw[8..12].copy_from_slice(&1_i32.to_le_bytes());
        raw[12..16].copy_from_slice(&4_i32.to_le_bytes());
        raw.extend_from_slice(&[0; 8]);
        raw.extend_from_slice(&count.to_le_bytes());
        raw.resize(raw.len() + count as usize, 0xaa);
        for profile in [
            SimulationShellCompositeProfile::ExtraReader,
            SimulationShellCompositeProfile::ExtraWriter,
        ] {
            let result = SimulationShellComposite::from_bytes(&raw, version, profile, LIMITS);
            assert_eq!(result.is_ok(), count == 1000000);
            if let Ok(body) = result {
                assert_eq!(body.to_bytes(), raw);
            }
        }
    }
}
