/// Preserved ContourType integer with the bounded conversion policy from specification 14.463.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SimulationPostInfoContourCode(i32);

impl SimulationPostInfoContourCode {
    /// Preserve a stored code without assigning a visual meaning; e.g. `Self::from_raw(5)`.
    pub const fn from_raw(code: i32) -> Self {
        Self(code)
    }

    /// Return the original signed code; e.g. `Self::from_raw(-1).raw()`.
    pub const fn raw(self) -> i32 {
        self.0
    }

    /// Apply the forward conversion without rewriting the code; e.g. `Self::from_raw(5).preference_integer()`.
    pub const fn preference_integer(self) -> i32 {
        match self.0 {
            0 => 0,
            1 => 1,
            4 => 2,
            5 | 6 => 3,
            _ => 6,
        }
    }

    /// Apply the reverse conversion, including its noninvertible fallback; e.g. `Self::from_preference(3).raw()`.
    pub const fn from_preference(preference: i32) -> Self {
        Self(match preference {
            0 => 0,
            1 => 1,
            2 => 4,
            _ => 6,
        })
    }

    /// Apply the documented default with caller-supplied plot context; e.g. `Self::default_from_preference(0, Some(22)).raw()`.
    pub const fn default_from_preference(preference: i32, plot_type: Option<i32>) -> Self {
        if matches!(plot_type, Some(22 | 57 | 70)) {
            return Self(4);
        }
        Self::from_preference(preference)
    }
}
