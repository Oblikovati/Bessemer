use super::super::rgba::rounded_significand;
use super::SimulationLegendWidthOperation;

pub(super) fn initial_width_patterns(font: i32) -> [u32; 3] {
    let converted = signed_font_nearest_even(font);
    let scaled = width_product_nearest_even(converted, 0x3a83126f);
    let initial = width_product_nearest_even(scaled, 0x3f200000);
    [converted, scaled, initial]
}

fn signed_font_nearest_even(font: i32) -> u32 {
    let magnitude = font.unsigned_abs();
    if magnitude == 0 {
        return 0;
    }
    let leading = 31 - magnitude.leading_zeros();
    let significand = if leading <= 23 {
        magnitude << (23 - leading)
    } else {
        rounded_significand(u64::from(magnitude), leading - 23)
    };
    encode_width_normal((font as u32) & 0x80000000, leading + 127, significand)
}

fn width_product_nearest_even(bits: u32, factor: u32) -> u32 {
    if bits & 0x7fffffff == 0 {
        return bits;
    }
    // Signed32 font magnitudes and the bound constants keep every nonzero stage normal.
    let product =
        u64::from((bits & 0x7fffff) | 0x800000) * u64::from((factor & 0x7fffff) | 0x800000);
    let leading = 63 - product.leading_zeros();
    let exponent = ((bits >> 23) & 255) + (factor >> 23) - 127 + leading - 46;
    let significand = rounded_significand(product, leading - 23);
    encode_width_normal(bits & 0x80000000, exponent, significand)
}

fn encode_width_normal(sign: u32, exponent: u32, significand: u32) -> u32 {
    if significand == 1 << 24 {
        return sign | ((exponent + 1) << 23);
    }
    sign | (exponent << 23) | (significand & 0x7fffff)
}

pub(super) fn selected_width_pattern(
    initial: u32,
    selector: i32,
) -> (SimulationLegendWidthOperation, u32) {
    if selector == 0 {
        return (
            SimulationLegendWidthOperation::MultiplyByFour,
            width_product_nearest_even(initial, 0x40800000),
        );
    }
    if selector == 1 {
        let doubled = if initial & 0x7fffffff == 0 {
            initial
        } else {
            initial + (1 << 23)
        };
        return (SimulationLegendWidthOperation::AddInitialToItself, doubled);
    }
    (SimulationLegendWidthOperation::KeepInitial, initial)
}
