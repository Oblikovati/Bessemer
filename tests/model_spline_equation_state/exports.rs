use super::*;
use std::io::Write;

pub(super) fn export_actual_equation_result(result: &PartResolvedFeaturesEdit) {
    let Some(destination) = std::env::var_os("BESSEMER_EQUATION_RESULT_PACKET") else {
        return;
    };
    let raw = result.archive().to_bytes().unwrap();
    let document = equation_fixture_document(&raw).to_bytes().unwrap();
    let state = result.archive().spline_equation_states().unwrap().remove(0);
    let mut packet = Vec::new();
    put_frame(&mut packet, &document);
    put_frame(&mut packet, &raw);
    put_state(&mut packet, &state);
    let mut output = std::fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(destination)
        .unwrap();
    output.write_all(&packet).unwrap();
}

fn put_state(packet: &mut Vec<u8>, state: &ModelSplineEquationStoredState) {
    packet.extend_from_slice(&state.flags_word().to_le_bytes());
    packet.extend_from_slice(&state.compatibility_word().to_le_bytes());
    packet.extend_from_slice(&state.saved_option().to_le_bytes());
    for role in [
        ModelSplineEquationTextRole::XExpression,
        ModelSplineEquationTextRole::YExpression,
        ModelSplineEquationTextRole::ZExpression,
        ModelSplineEquationTextRole::DeclaredCharacters,
        ModelSplineEquationTextRole::RangeStart,
        ModelSplineEquationTextRole::RangeEnd,
    ] {
        put_frame(packet, state.text(role).text().as_bytes());
    }
}

fn put_frame(packet: &mut Vec<u8>, bytes: &[u8]) {
    packet.extend_from_slice(&u32::try_from(bytes.len()).unwrap().to_le_bytes());
    packet.extend_from_slice(bytes);
}
