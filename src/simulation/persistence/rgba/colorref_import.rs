//! Explicit nearest-even model from specification 14.482/14.524; no ambient floating-point state (#166).
use super::SimulationRgbaColor;

impl SimulationRgbaColor {
    /// Model packed construction with alpha one and an ignored high byte; e.g. `Self::from_colorref_nearest_even(0x00112233)`.
    pub fn from_colorref_nearest_even(packed: u32) -> Self {
        Self::from_bits([
            colorref_component_bits(packed as u8),
            colorref_component_bits((packed >> 8) as u8),
            colorref_component_bits((packed >> 16) as u8),
            0x3f800000,
        ])
    }

    /// Model packed assignment while preserving every alpha bit; e.g. `color.assigned_colorref_nearest_even(0x00112233)`.
    pub fn assigned_colorref_nearest_even(self, packed: u32) -> Self {
        let mut bits = Self::from_colorref_nearest_even(packed).bits();
        bits[3] = self.bits()[3];
        Self::from_bits(bits)
    }
}

fn colorref_component_bits(component: u8) -> u32 {
    if component == 0 {
        return 0;
    }
    // The exact source scale 0x3b808081 is 0x808081 * 2^-31.
    let product = u64::from(component) * 0x808081;
    let mut leading = 63 - product.leading_zeros();
    let mut significand = rounded_significand(product, leading - 23);
    if significand == 1 << 24 {
        significand >>= 1;
        leading += 1;
    }
    ((leading + 96) << 23) | (significand & 0x7fffff)
}

pub(in super::super) fn rounded_significand(product: u64, shift: u32) -> u32 {
    let mut retained = product >> shift;
    if shift == 0 {
        return retained as u32;
    }
    let remainder = product & ((1_u64 << shift) - 1);
    let midpoint = 1_u64 << (shift - 1);
    if remainder > midpoint || (remainder == midpoint && retained & 1 != 0) {
        retained += 1;
    }
    retained as u32
}
