use bessemer::model_archives::{
    ModelArchiveLimits, SimulationEmbeddedArchive, SimulationEmbeddedProfile,
};

pub(super) fn framed_string_archive(
    extended: bool,
    limits: ModelArchiveLimits,
) -> SimulationEmbeddedArchive {
    let name = b"moCStringHandle_c";
    let mut bytes = 14000_u32.to_le_bytes().to_vec();
    bytes.extend_from_slice(&[0xff, 0xff, 7, 0]);
    bytes.extend_from_slice(&(name.len() as u16).to_le_bytes());
    bytes.extend_from_slice(name);
    bytes.extend_from_slice(if extended {
        &[0xff, 1, 0, b'x']
    } else {
        &[1, b'x']
    });
    SimulationEmbeddedArchive::from_bytes(&bytes, SimulationEmbeddedProfile::Modern, limits)
        .unwrap()
}

pub(super) fn assert_wire_value(
    value: &bessemer::simulation::SimulationThermalValue,
    expected: &SimulationEmbeddedArchive,
) {
    let bessemer::simulation::SimulationThermalValue::Embedded(actual) = value else {
        panic!("expected complete embedded value");
    };
    assert_eq!(actual.to_bytes().unwrap(), expected.to_bytes().unwrap());
}
