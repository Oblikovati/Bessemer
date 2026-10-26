use super::{ModelDecoder, ModelObjectRole};
use crate::archive_codec::{MaterialRead, ObjectReader};
use crate::{BessemerError, FailureClass, Result};

#[derive(Clone, Copy)]
pub(super) enum ModelUnitKind {
    Table,
    Base,
    Coded,
    Energy,
    Length,
    Density,
}

const BASE_UNIT_CLASSES: &[&str] = &[
    "moUserUnits_c",
    "moNumberUserUnits_c",
    "moFloatNumberUserUnits_c",
    "moSpringConstantUnits_c",
    "moStressUnits_c",
    "moGravityUnits_c",
    "moLinearMotorUnits_c",
    "moRotaryMotorUnits_c",
    "moFrequencyUserUnits_c",
];

impl ModelUnitKind {
    pub fn parse(name: &str) -> Option<Self> {
        if BASE_UNIT_CLASSES.contains(&name) {
            return Some(Self::Base);
        }
        match name {
            "moUnitsTable_c" => Some(Self::Table),
            "moAngleUserUnits_c" | "moUnitSysUnits_c" | "moForceUnits_c" => Some(Self::Coded),
            "moPowerUnits_c" | "moEnergyUnits_c" | "moTimeUnits_c" => Some(Self::Energy),
            "moLengthUserUnits_c" | "moPercentUserUnits_c" => Some(Self::Length),
            "moDensityUnits_c" => Some(Self::Density),
            _ => None,
        }
    }
}

pub(super) fn read(
    reader: &mut ModelDecoder<'_>,
    map: &mut ObjectReader,
    kind: ModelUnitKind,
) -> Result<()> {
    if matches!(kind, ModelUnitKind::Table) {
        return table(reader, map);
    }
    base(reader)?;
    match kind {
        ModelUnitKind::Coded => reader.u16("unknown_b0").map(|_| ()),
        ModelUnitKind::Energy => energy(reader),
        ModelUnitKind::Length => length(reader, map),
        ModelUnitKind::Density => density(reader, map),
        ModelUnitKind::Base | ModelUnitKind::Table => Ok(()),
    }
}

fn base(reader: &mut ModelDecoder<'_>) -> Result<()> {
    reader.i32("unknown_0c")?;
    reader.i32("unknown_10")?;
    reader.f64("unknown_18")?;
    reader.i32s(&["unknown_20", "unknown_24"])?;
    if reader.version() > 0x822 {
        reader.f64("unknown_30")?;
    }
    reader.i32("unknown_28")?;
    for (threshold, label) in [
        (0x1aa, "unknown_6c"),
        (0x305, "unknown_70"),
        (0x1041, "unknown_74"),
    ] {
        if reader.version() > threshold {
            reader.i32(label)?;
        }
    }
    late_base(reader)
}

fn late_base(reader: &mut ModelDecoder<'_>) -> Result<()> {
    if reader.version() >= 0x1bd2 {
        reader.f64("unknown_88")?;
        reader.u16("unknown_7c")?;
    }
    if reader.version() > 0x1bd6 {
        reader.i32("unknown_08")?;
    }
    Ok(())
}

fn table(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    if reader.version() < 0x69 {
        return Err(BessemerError::new(
            "model archive",
            "units table",
            FailureClass::Unsupported,
            format!("units table version {}; expected >= 0x69", reader.version()),
        ));
    }
    let count = reader.object_count("items")?;
    for index in 0..count {
        reader.object(&format!("items[{index}]"), map, ModelObjectRole::Any)?;
    }
    Ok(())
}

fn energy(reader: &mut ModelDecoder<'_>) -> Result<()> {
    if reader.version() > 0xdce {
        reader.u16("unknown_b0")?;
    }
    Ok(())
}

fn length(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    reader.u16("unknown_b0")?;
    if reader.version() > 0x926 {
        reader.object(
            "unknown_b8",
            map,
            ModelObjectRole::Declared(&["moLengthUserUnits_c", "moPercentUserUnits_c"]),
        )?;
    }
    Ok(())
}

fn density(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    if reader.version() > 0x92e {
        reader.u16("unknown_c0")?;
        reader.u16("unknown_c4")?;
    }
    if reader.version() <= 0xac6 {
        return Ok(());
    }
    if reader.u8("has_length_unit")? != 0 {
        reader.object("length_unit", map, ModelObjectRole::Any)?;
    }
    if reader.version() > 0x1faa {
        reader.u16("unknown_c8")?;
        reader.i32("unknown_cc")?;
    }
    Ok(())
}

pub(super) fn read_class(
    reader: &mut ModelDecoder<'_>,
    map: &mut ObjectReader,
    name: &str,
    schema: u16,
) -> Result<()> {
    if let Some(kind) = ModelUnitKind::parse(name) {
        return read(reader, map, kind);
    }
    Err(BessemerError::new(
        "model archive",
        "model stream",
        FailureClass::Unsupported,
        format!(
            "runtime class {name:?} schema {schema}; expected an implemented complete model object body"
        ),
    ))
}
