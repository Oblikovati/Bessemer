use crate::simulation::SimulationPersistVersion;
use std::sync::Arc;

/// Explicit LBC carrier context; neither variant infers a stored or runtime version.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SimulationLbcPropertyProfile {
    /// Reader field sequence selected by the caller's normalized version.
    VersionSelected(SimulationPersistVersion),
    /// Independently recovered fixed 54-word installed writer sequence, without a version claim.
    InstalledFixedWriter,
}

/// One documented runtime destination, distinct from its physical offset in the stream.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SimulationLbcRuntimeMember(u16);

impl SimulationLbcRuntimeMember {
    /// Original runtime owner offset, e.g. `field.member.runtime_offset()`; this is not a stored byte offset.
    pub fn runtime_offset(self) -> u16 {
        self.0
    }
}

pub(super) fn member_order(
    profile: SimulationLbcPropertyProfile,
) -> Arc<[SimulationLbcRuntimeMember]> {
    let version = match profile {
        SimulationLbcPropertyProfile::VersionSelected(version) => version.value(),
        SimulationLbcPropertyProfile::InstalledFixedWriter => return modern_order(),
    };
    let mut members = base_members();
    if version > 800027 {
        members.push(0x4f8);
    }
    if version >= 800060 {
        members.push(0x550);
    }
    if version >= 820010 {
        members.push(0x498);
    }
    append_extended(&mut members, version);
    members
        .into_iter()
        .map(SimulationLbcRuntimeMember)
        .collect()
}

fn base_members() -> Vec<u16> {
    vec![
        0x438, 0x43c, 0x440, 0x46c, 0x478, 0x47c, 0x480, 0x494, 0x484, 0x488, 0x48c, 0x490, 0x508,
    ]
}

fn append_extended(members: &mut Vec<u16>, version: i32) {
    if version >= 840004 {
        members.extend([0x470, 0x474]);
        members.extend((0x444..0x46c).step_by(4));
        let count = variable_count(version);
        members.extend((0x49c..0x49c + count * 4).step_by(4));
        members.extend((0x4e0..0x4f8).step_by(4));
    }
    if version >= 840013 {
        members.extend([0x520, 0x524, 0x528]);
    }
}

fn variable_count(version: i32) -> u16 {
    if version >= 891101 {
        return 17;
    }
    if version >= 891000 {
        return 16;
    }
    if version >= 890620 {
        return 15;
    }
    14
}

fn modern_order() -> Arc<[SimulationLbcRuntimeMember]> {
    let mut members = base_members();
    members.extend([0x4f8, 0x550, 0x498, 0x470, 0x474]);
    members.extend((0x444..0x46c).step_by(4));
    members.extend((0x49c..0x4e0).step_by(4));
    members.extend((0x4e0..0x4f8).step_by(4));
    members.extend([0x520, 0x524, 0x528]);
    members
        .into_iter()
        .map(SimulationLbcRuntimeMember)
        .collect()
}
