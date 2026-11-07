//! Conditional UCOMISS/JNE behavior from specification 14.484; stored bitwise equality remains distinct (#166).
use super::SimulationRgbaColor;

impl SimulationRgbaColor {
    /// Model all-channel equality with masked exceptions and DAZ off; e.g. `left.equal_masked_exceptions_daz_off(right)`.
    pub fn equal_masked_exceptions_daz_off(self, other: Self) -> bool {
        self.bits()
            .into_iter()
            .zip(other.bits())
            .all(|(left, right)| {
                unordered_nan(left)
                    || unordered_nan(right)
                    || left == right
                    || both_signed_zeros(left, right)
            })
    }

    /// Return the complement of the explicit native comparison model; e.g. `left.different_masked_exceptions_daz_off(right)`.
    pub fn different_masked_exceptions_daz_off(self, other: Self) -> bool {
        !self.equal_masked_exceptions_daz_off(other)
    }
}

pub(in crate::simulation::persistence) fn unordered_nan(bits: u32) -> bool {
    bits & 0x7f800000 == 0x7f800000 && bits & 0x7fffff != 0
}

fn both_signed_zeros(left: u32, right: u32) -> bool {
    left & 0x7fffffff == 0 && right & 0x7fffffff == 0
}
