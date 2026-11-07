//! Complete Electrical carrier and lexical operations against independent fixtures.
use bessemer::FailureClass;
use bessemer::electrical::*;
#[path = "electrical_projects/limits.rs"]
mod boundary_checks;
#[path = "electrical_projects/carrier.rs"]
mod carrier;
#[path = "electrical_projects/fixture.rs"]
mod fixture;
#[path = "electrical_projects/lexical.rs"]
mod lexical;
use fixture::*;

fn limits() -> ElectricalLimits {
    ElectricalLimits {
        maximum_bytes: 4_000_000,
        maximum_members: 128,
        maximum_member_bytes: 4_000_000,
        maximum_expanded_bytes: 16_000_000,
        maximum_ini_records: 8192,
    }
}

fn parse(raw: &[u8]) -> ElectricalArchive {
    ElectricalArchive::from_bytes(raw, limits()).unwrap()
}

fn ini_limits() -> ElectricalIniLimits {
    ElectricalIniLimits {
        maximum_bytes: 4_000_000,
        maximum_records: 8192,
    }
}

fn assert_send_sync<T: Send + Sync>() {}

#[test]
fn every_public_electrical_snapshot_identity_and_result_type_is_send_and_sync() {
    assert_send_sync::<ElectricalArchive>();
    assert_send_sync::<ElectricalArchiveEdit>();
    assert_send_sync::<ElectricalLimits>();
    assert_send_sync::<ElectricalMember>();
    assert_send_sync::<ElectricalMemberId>();
    assert_send_sync::<ElectricalMemberMetadata<'static>>();
    assert_send_sync::<ElectricalCompression>();
    assert_send_sync::<ElectricalCoverage>();
    assert_send_sync::<ElectricalCheckMode>();
    assert_send_sync::<ElectricalDiagnostic>();
    assert_send_sync::<ElectricalMemberHistory>();
    assert_send_sync::<ElectricalMemberRelation>();
    assert_ini_send_sync();
}

fn assert_ini_send_sync() {
    assert_send_sync::<ElectricalIniDocument>();
    assert_send_sync::<ElectricalIniEdit>();
    assert_send_sync::<ElectricalIniLimits>();
    assert_send_sync::<ElectricalIniRecord>();
    assert_send_sync::<ElectricalIniKind>();
    assert_send_sync::<ElectricalIniCoverage>();
    assert_send_sync::<ElectricalIniCheckMode>();
    assert_send_sync::<ElectricalIniDiagnostic>();
    assert_send_sync::<ElectricalByteHistory>();
    assert_send_sync::<ElectricalByteRelation>();
    assert_send_sync::<ElectricalEditCompletion>();
}
