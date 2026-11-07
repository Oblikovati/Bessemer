//! Bounded nearest-even GetCOLORREF model from specification 14.481 (#166).
use super::colorref_import::rounded_significand;
use super::{SimulationColorChannel, SimulationRgbaColor};

const RGB_CHANNELS: [SimulationColorChannel; 3] = [
    SimulationColorChannel::Red,
    SimulationColorChannel::Green,
    SimulationColorChannel::Blue,
];

/// Why an RGB channel lies outside the explicit nearest-even converter model.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SimulationColorrefDomainFailure {
    /// Input is infinity or NaN.
    NonfiniteInput,
    /// Input is a nonzero subnormal whose native denormal handling is unobserved.
    SubnormalInput,
    /// Rounded multiplication overflows binary32.
    NonfiniteProduct,
    /// Rounded multiplication falls outside the signed32 conversion interval.
    Signed32ProductRange,
}

/// Conditional packed projection with its original RGBA value retained.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SimulationColorrefProjection {
    /// All three RGB channels satisfy the declared model.
    Projected {
        /// Original color, including ignored alpha.
        original: SimulationRgbaColor,
        /// Truncated byte channels packed as 0x00BBGGRR.
        packed: u32,
    },
    /// A required channel check is outside the model, so no packed value is published.
    OutsideModelDomain {
        /// Original unchanged color.
        original: SimulationRgbaColor,
        /// First RGB channel outside the declared model.
        channel: SimulationColorChannel,
        /// Explicit domain exclusion.
        reason: SimulationColorrefDomainFailure,
    },
}

impl SimulationRgbaColor {
    /// Project RGB under explicit nearest-even multiplication and signed truncation; e.g. `color.colorref_nearest_even()`.
    pub fn colorref_nearest_even(self) -> SimulationColorrefProjection {
        let mut packed = 0;
        for (index, channel) in RGB_CHANNELS.into_iter().enumerate() {
            match colorref_component_byte(self.channel_bits(channel)) {
                Ok(byte) => packed |= u32::from(byte) << (8 * index),
                Err(reason) => {
                    return SimulationColorrefProjection::OutsideModelDomain {
                        original: self,
                        channel,
                        reason,
                    };
                }
            }
        }
        SimulationColorrefProjection::Projected {
            original: self,
            packed,
        }
    }
}

fn colorref_component_byte(bits: u32) -> Result<u8, SimulationColorrefDomainFailure> {
    let exponent = (bits >> 23) & 255;
    let fraction = bits & 0x7fffff;
    require_input_domain(exponent, fraction)?;
    if exponent == 0 {
        return Ok(0);
    }
    let product = u64::from(0x800000 | fraction) * 255;
    let leading = 63 - product.leading_zeros();
    let mut significand = rounded_significand(product, leading - 23);
    let mut product_exponent = exponent + leading - 23;
    if significand == 1 << 24 {
        significand >>= 1;
        product_exponent += 1;
    }
    signed_low_byte(bits >> 31 != 0, product_exponent, significand)
}

fn require_input_domain(
    exponent: u32,
    fraction: u32,
) -> Result<(), SimulationColorrefDomainFailure> {
    if exponent == 255 {
        return Err(SimulationColorrefDomainFailure::NonfiniteInput);
    }
    if exponent == 0 && fraction != 0 {
        return Err(SimulationColorrefDomainFailure::SubnormalInput);
    }
    Ok(())
}

fn signed_low_byte(
    negative: bool,
    exponent: u32,
    significand: u32,
) -> Result<u8, SimulationColorrefDomainFailure> {
    if exponent >= 255 {
        return Err(SimulationColorrefDomainFailure::NonfiniteProduct);
    }
    if exponent > 158 || (exponent == 158 && !(negative && significand == 0x800000)) {
        return Err(SimulationColorrefDomainFailure::Signed32ProductRange);
    }
    let magnitude = integer_magnitude(exponent, significand);
    Ok(if negative {
        0_u32.wrapping_sub(magnitude) as u8
    } else {
        magnitude as u8
    })
}

fn integer_magnitude(exponent: u32, significand: u32) -> u32 {
    if exponent < 127 {
        return 0;
    }
    if exponent < 150 {
        return significand >> (150 - exponent);
    }
    significand << (exponent - 150)
}
