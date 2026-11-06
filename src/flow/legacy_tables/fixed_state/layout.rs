use super::*;
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum FixedStateLayout {
    NoRows,
    WidePair,
    Settings,
    SettingsWithFinal,
}
impl TryFrom<i32> for FixedStateLayout {
    type Error = BessemerError;
    fn try_from(version: i32) -> Result<Self> {
        match version {
            0 => Ok(Self::NoRows),
            1 => Ok(Self::WidePair),
            2 => Ok(Self::Settings),
            3 | 4 => Ok(Self::SettingsWithFinal),
            _ => Err(table_failure(
                139,
                FailureClass::Unsupported,
                format!("version {version}; expected fixed-state version zero through four"),
            )),
        }
    }
}
impl FixedStateLayout {
    pub(super) fn stride(self) -> usize {
        match self {
            Self::NoRows => 0,
            Self::WidePair => 16,
            Self::Settings => 2976,
            Self::SettingsWithFinal => 2980,
        }
    }
    pub(super) fn has_settings(self) -> bool {
        matches!(self, Self::Settings | Self::SettingsWithFinal)
    }
    pub(super) fn has_final(self) -> bool {
        self == Self::SettingsWithFinal
    }
}
