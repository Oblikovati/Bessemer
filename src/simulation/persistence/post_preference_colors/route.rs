use crate::{BessemerError, FailureClass, Result};

/// One normalized C907 object-member route; these offsets are not wire positions.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u16)]
pub enum SimulationStylePreferenceColorRoute {
    /// Member220, with raw preference alpha replacement.
    Style220 = 0x220,
    /// Member1BC, retaining the original alpha.
    Style1bc = 0x1bc,
    /// Member1CC, retaining the original alpha.
    Style1cc = 0x1cc,
    /// Member1DC, with raw preference alpha replacement.
    Style1dc = 0x1dc,
    /// Member2AC, with raw preference alpha replacement.
    Style2ac = 0x2ac,
    /// Member2BC, retaining the original alpha.
    Style2bc = 0x2bc,
    /// Member1F4, with raw preference alpha replacement.
    Style1f4 = 0x1f4,
    /// Member20C, with raw preference alpha replacement.
    Style20c = 0x20c,
}
impl SimulationStylePreferenceColorRoute {
    /// Normalize only the eight documented members; e.g. `Self::from_style_member(0x220)?`.
    pub fn from_style_member(raw: u32) -> Result<Self> {
        match raw {
            0x220 => Ok(Self::Style220),
            0x1bc => Ok(Self::Style1bc),
            0x1cc => Ok(Self::Style1cc),
            0x1dc => Ok(Self::Style1dc),
            0x2ac => Ok(Self::Style2ac),
            0x2bc => Ok(Self::Style2bc),
            0x1f4 => Ok(Self::Style1f4),
            0x20c => Ok(Self::Style20c),
            other => Err(BessemerError::new(
                "style preference color",
                "style member",
                FailureClass::Unsupported,
                format!("member {other:#x}; expected 220,1BC,1CC,1DC,2AC,2BC,1F4 or20C"),
            )),
        }
    }
    /// Inspect the normalized object member; e.g. `route.style_member()`.
    pub fn style_member(self) -> u32 {
        u32::from(self as u16)
    }
    /// Locate the existing canonical color field; e.g. `route.field_path()`.
    pub fn field_path(self) -> &'static str {
        match self {
            Self::Style220 => "style.color_220",
            Self::Style1bc => "style.color_1bc",
            Self::Style1cc => "style.color_1cc",
            Self::Style1dc => "style.color_1dc",
            Self::Style2ac => "style.color_2ac",
            Self::Style2bc => "style.color_2bc",
            Self::Style1f4 => "style.color_1f4",
            Self::Style20c => "style.color_20c",
        }
    }
    /// Inspect the packed preference source offset; e.g. `route.packed_preference_offset()`.
    pub fn packed_preference_offset(self) -> usize {
        self.offsets().0
    }
    /// Inspect the optional raw alpha source offset; e.g. `route.alpha_preference_offset()`.
    pub fn alpha_preference_offset(self) -> Option<usize> {
        self.offsets().1
    }
    /// Inspect the reached source extent, without requiring it for skipped blocks; e.g. `route.required_preference_bytes()`.
    pub fn required_preference_bytes(self) -> usize {
        let (packed, alpha) = self.offsets();
        alpha.map_or(packed, |offset| packed.max(offset)) + 4
    }
    fn offsets(self) -> (usize, Option<usize>) {
        match self {
            Self::Style220 => (0x1578, Some(0x157c)),
            Self::Style1bc => (0x1588, None),
            Self::Style1cc => (0x158c, None),
            Self::Style1dc => (0x1590, Some(0x1594)),
            Self::Style2ac => (0x15a8, Some(0x15b0)),
            Self::Style2bc => (0x15ac, None),
            Self::Style1f4 => (0x15d0, Some(0x15c8)),
            Self::Style20c => (0x15cc, Some(0x15c4)),
        }
    }
}
