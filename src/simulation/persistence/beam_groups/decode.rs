use super::super::{load_base, load_reader::BinaryLoadReader};
use super::SimulationBeamGroupKind;
use crate::{Result, simulation::invalid};

pub(super) fn read(reader: &mut BinaryLoadReader<'_>, kind: SimulationBeamGroupKind) -> Result<()> {
    match kind {
        SimulationBeamGroupKind::JointGroup => group(reader),
        SimulationBeamGroupKind::Joint => joint(reader),
        SimulationBeamGroupKind::JointList {
            expected_joint_count,
        } => joint_list(reader, expected_joint_count),
        SimulationBeamGroupKind::BeamFolder => {
            reader.signed("folder.word20")?;
            reader.text("folder.text28")
        }
    }
}

fn group(reader: &mut BinaryLoadReader<'_>) -> Result<()> {
    reader.signed("group.wordd0")?;
    reader.text("group.textc8")?;
    reader.double("group.doubleb8")?;
    for word in ["wordc0", "worda8", "wordac"] {
        reader.signed(&format!("group.{word}"))?;
    }
    group_gates(reader)
}

fn group_gates(reader: &mut BinaryLoadReader<'_>) -> Result<()> {
    for (gate, word) in [
        (810023, "worde0"),
        (810024, "wordb0"),
        (810027, "wordb4"),
        (830029, "word50"),
    ] {
        if reader.version >= gate {
            reader.signed(&format!("group.{word}"))?;
        }
    }
    if reader.version >= 810014 {
        load_base::reference_container(reader, "group.references", "group.references")?;
    }
    if reader.version >= 840023 {
        reader.double("group.saved_version_double")?;
    }
    Ok(())
}

fn joint(reader: &mut BinaryLoadReader<'_>) -> Result<()> {
    reader.signed("joint.wordf0")?;
    reader.signed("joint.wordf4")?;
    if reader.version >= 810021 {
        reader.words("joint.identity", 2)?;
    }
    if (810000..=810030).contains(&reader.version) {
        load_base::reference_container(reader, "joint.references", "joint.references")?;
    }
    let count = reader.count("joint.beam_count", 10000, 4)?;
    reader.words("joint.beam_ids", count)?;
    if reader.version >= 810022 {
        reader.doubles("joint.three_doubles", 3)?;
    }
    if reader.version >= 810030 {
        reader.double("joint.doublef8")?;
    }
    Ok(())
}

fn joint_list(reader: &mut BinaryLoadReader<'_>, expected: Option<usize>) -> Result<()> {
    let count = reader.count("joint_list.count", 10000, 4)?;
    if expected.is_some_and(|expected| expected != count) {
        return Err(invalid(
            "beam JointList child count",
            format!("stored {count}; expected independent count {expected:?}"),
        ));
    }
    for index in 0..count {
        let prefix = format!("joint_list.group{index}");
        let pairs = reader.count(&format!("{prefix}.count"), 10000, 8)?;
        reader.words(&format!("{prefix}.pairs"), pairs * 2)?;
    }
    Ok(())
}
