use super::*;
use FlowSettingsBodyMember::*;

const WIRE_ORDER: [FlowSettingsBodyMember; 18] = [
    Member34, Member38, Member4c, Member4d, Member50, Member2d, Member44, Member0f, Member10,
    Member11, Member88, Member3c, Member41, Member40, Member1e, Member89, Member8a, Member48,
];

pub(super) fn decode(payload: &[u8]) -> Result<ParsedBody> {
    let version = read_version(payload)?;
    let members: Vec<FlowSettingsBodyMember> = WIRE_ORDER
        .into_iter()
        .filter(|member| introduced(*member) <= version.0)
        .collect();
    validate_extent(payload, version, &members)?;
    let fields = read_fields(payload, &members);
    Ok(ParsedBody {
        version,
        payload: Arc::from(payload),
        fields: Arc::from(fields),
    })
}

fn read_version(payload: &[u8]) -> Result<FlowSettingsBodyVersion> {
    if payload.len() < 8 {
        return Err(body_failure(
            FailureClass::Invalid,
            format!(
                "{} bytes; expected at least eight version bytes",
                payload.len()
            ),
        ));
    }
    let outer = u32::from_le_bytes([payload[0], payload[1], payload[2], payload[3]]);
    if outer != 0 {
        return Err(body_failure(
            FailureClass::Unsupported,
            format!("outer version {outer}; expected 0"),
        ));
    }
    FlowSettingsBodyVersion::try_from(u32::from_le_bytes([
        payload[4], payload[5], payload[6], payload[7],
    ]))
}

fn read_fields(payload: &[u8], members: &[FlowSettingsBodyMember]) -> Vec<FlowSettingsBodyField> {
    let mut offset = 8;
    members
        .iter()
        .map(|member| {
            let start = offset;
            offset += width(*member);
            FlowSettingsBodyField {
                member: *member,
                offset: start,
                value: read_value(*member, &payload[start..offset]),
            }
        })
        .collect()
}

fn introduced(member: FlowSettingsBodyMember) -> u8 {
    match member {
        Member44 => 1,
        Member0f => 2,
        Member88 => 3,
        Member10 => 4,
        Member3c | Member41 => 5,
        Member40 => 6,
        Member11 => 7,
        Member1e => 8,
        Member89 => 9,
        Member8a => 10,
        Member48 => 11,
        _ => 0,
    }
}

pub(super) fn width(member: FlowSettingsBodyMember) -> usize {
    match member {
        Member38 | Member44 | Member3c | Member48 => 4,
        Member50 => 8,
        _ => 1,
    }
}

fn read_value(member: FlowSettingsBodyMember, bytes: &[u8]) -> FlowSettingsBodyValue {
    match member {
        Member38 | Member44 | Member48 => {
            FlowSettingsBodyValue::Raw4([bytes[0], bytes[1], bytes[2], bytes[3]])
        }
        Member3c => FlowSettingsBodyValue::Word(u32::from_le_bytes([
            bytes[0], bytes[1], bytes[2], bytes[3],
        ])),
        Member50 => FlowSettingsBodyValue::Raw8([
            bytes[0], bytes[1], bytes[2], bytes[3], bytes[4], bytes[5], bytes[6], bytes[7],
        ]),
        _ => FlowSettingsBodyValue::Byte(bytes[0]),
    }
}

pub(super) fn value_bytes(value: FlowSettingsBodyValue) -> Vec<u8> {
    match value {
        FlowSettingsBodyValue::Byte(byte) => vec![byte],
        FlowSettingsBodyValue::Word(word) => word.to_le_bytes().to_vec(),
        FlowSettingsBodyValue::Raw4(bytes) => bytes.to_vec(),
        FlowSettingsBodyValue::Raw8(bytes) => bytes.to_vec(),
    }
}

fn validate_extent(
    payload: &[u8],
    version: FlowSettingsBodyVersion,
    members: &[FlowSettingsBodyMember],
) -> Result<()> {
    let expected = 8 + members.iter().map(|member| width(*member)).sum::<usize>();
    if payload.len() != expected {
        return Err(body_failure(
            FailureClass::Invalid,
            format!(
                "version {} has {} bytes; expected exactly {expected}",
                version.0,
                payload.len()
            ),
        ));
    }
    Ok(())
}
