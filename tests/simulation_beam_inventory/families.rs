use super::*;
use bessemer::{Result, model_archives::ModelArchiveLimits};

const LIMITS: SimulationThermalLimits = SimulationThermalLimits {
    maximum_bytes: 1000000,
    maximum_fields: 100000,
    maximum_entries: 2000000,
    maximum_text_bytes: 1000000,
    embedded: ModelArchiveLimits {
        maximum_bytes: 100000,
        maximum_fields: 50000,
        maximum_entries: 100000,
        maximum_depth: 256,
        maximum_objects: 20000,
    },
};

pub(super) fn reconstruct(
    payload: &PayloadView<'_>,
    argument: &CandidateArgument,
) -> Result<Vec<u8>> {
    if payload.family != PayloadFamily::JointList {
        assert_eq!(argument.expected_count, None);
    }
    match payload.family {
        PayloadFamily::Manager => manager(payload.raw, argument.version),
        PayloadFamily::Beam => beam(payload.raw, argument.version),
        PayloadFamily::Material => material(payload.raw, argument.version),
        _ => group(payload, argument),
    }
}

fn manager(raw: &[u8], version: SimulationPersistVersion) -> Result<Vec<u8>> {
    let profile = SimulationManagerPropertyProfile::Beam(version);
    let body = SimulationManagerProperty::from_bytes(raw, profile)?;
    let values: Vec<i32> = body.fields().iter().map(|field| field.value).collect();
    let constructed = SimulationManagerProperty::from_values(&values, profile)?;
    assert_eq!(constructed.profile(), body.profile());
    for (actual, source) in constructed.fields().iter().zip(body.fields()) {
        assert_eq!(actual.value, source.value);
        assert_eq!(actual.member, source.member);
        assert_eq!(actual.stream_offset, source.stream_offset);
    }
    Ok(constructed.to_bytes())
}

fn beam(raw: &[u8], version: SimulationPersistVersion) -> Result<Vec<u8>> {
    let body = SimulationBeamProperty::from_bytes(raw, version, LIMITS)?;
    let constructed = SimulationBeamProperty::from_fields(body.fields().to_vec(), version, LIMITS)?;
    assert_eq!(constructed.fields(), body.fields());
    assert_eq!(constructed.field_offsets(), body.field_offsets());
    assert_eq!(
        body.coverage().completion,
        SimulationBeamCompletion::CompletedBody
    );
    Ok(constructed.to_bytes())
}

fn material(raw: &[u8], version: SimulationPersistVersion) -> Result<Vec<u8>> {
    let limits = SimulationMaterialLimits {
        maximum_bytes: LIMITS.maximum_bytes,
        maximum_fields: LIMITS.maximum_fields,
        maximum_entries: LIMITS.maximum_entries,
        maximum_text_bytes: LIMITS.maximum_text_bytes,
    };
    let body = SimulationStudyMaterial::from_bytes(raw, version, limits)?;
    let constructed =
        SimulationStudyMaterial::from_fields(body.fields().to_vec(), version, limits)?;
    assert_eq!(constructed.fields(), body.fields());
    assert_eq!(constructed.field_offsets(), body.field_offsets());
    assert_eq!(
        body.coverage().completion,
        SimulationMaterialCompletion::CompletedBody
    );
    Ok(constructed.to_bytes())
}

fn group(payload: &PayloadView<'_>, argument: &CandidateArgument) -> Result<Vec<u8>> {
    let kind = group_kind(payload.family, argument.expected_count);
    let body = SimulationBeamGroupBody::from_bytes(payload.raw, argument.version, kind, LIMITS)?;
    let constructed = SimulationBeamGroupBody::from_fields(
        body.fields().to_vec(),
        argument.version,
        kind,
        LIMITS,
    )?;
    assert_eq!(constructed.fields(), body.fields());
    assert_eq!(constructed.field_offsets(), body.field_offsets());
    assert_eq!(constructed.kind(), kind);
    assert_eq!(
        body.coverage().completion,
        SimulationBeamGroupCompletion::CompletedBody
    );
    Ok(constructed.to_bytes())
}

fn group_kind(family: PayloadFamily, expected_count: Option<usize>) -> SimulationBeamGroupKind {
    match family {
        PayloadFamily::JointGroup => SimulationBeamGroupKind::JointGroup,
        PayloadFamily::Joint => SimulationBeamGroupKind::Joint,
        PayloadFamily::JointList => SimulationBeamGroupKind::JointList {
            expected_joint_count: expected_count,
        },
        _ => panic!("{:?}; expected a joint/group/list family", family),
    }
}
