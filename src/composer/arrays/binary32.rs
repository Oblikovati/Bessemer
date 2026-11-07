//! Integer binary32 RN/gradual arithmetic keeps the declared profile independent of MXCSR.

pub(super) fn reconstruct_code(code: u32, width: usize, first: u32, second: u32) -> u32 {
    let maximum = if width == 1 { 255 } else { 65535 };
    let weight = quantization_weight(code, maximum);
    let complement = add_finite(0x3f800000, weight ^ 0x80000000);
    let right = multiply_finite(weight, second);
    let left = multiply_finite(complement, first);
    add_finite(left, right)
}

fn quantization_weight(code: u32, maximum: u32) -> u32 {
    if code == 0 {
        return 0;
    }
    let mut exponent = maximum.leading_zeros() as i32 - code.leading_zeros() as i32;
    if (u64::from(code) << -exponent) < u64::from(maximum) {
        exponent -= 1;
    }
    let numerator = u64::from(code) << (23 - exponent);
    let divisor = u64::from(maximum);
    let quotient = numerator / divisor;
    let remainder = numerator % divisor;
    let increment = remainder * 2 > divisor || (remainder * 2 == divisor && quotient & 1 != 0);
    let mantissa = quotient + u64::from(increment);
    (((exponent + 127) as u32) << 23) | (mantissa as u32 & 0x7fffff)
}

fn finite_parts(word: u32) -> (u64, i32) {
    let exponent = (word >> 23) & 255;
    let fraction = word & 0x7fffff;
    if exponent == 0 {
        return (u64::from(fraction), -149);
    }
    (u64::from(fraction | 0x800000), exponent as i32 - 150)
}

fn multiply_finite(first: u32, second: u32) -> u32 {
    let (first_mantissa, first_exponent) = finite_parts(first);
    let (second_mantissa, second_exponent) = finite_parts(second);
    round_scaled(
        (first ^ second) & 0x80000000,
        first_mantissa * second_mantissa,
        first_exponent + second_exponent,
    )
}

fn add_finite(first: u32, second: u32) -> u32 {
    let (first_mantissa, first_exponent) = finite_parts(first);
    let (second_mantissa, second_exponent) = finite_parts(second);
    let exponent = first_exponent.max(second_exponent);
    let left = shift_jammed(first_mantissa << 3, (exponent - first_exponent) as u32);
    let right = shift_jammed(second_mantissa << 3, (exponent - second_exponent) as u32);
    if (first ^ second) & 0x80000000 == 0 {
        return round_scaled(first & 0x80000000, left + right, exponent - 3);
    }
    if left == right {
        return 0;
    }
    if left > right {
        return round_scaled(first & 0x80000000, left - right, exponent - 3);
    }
    round_scaled(second & 0x80000000, right - left, exponent - 3)
}

fn shift_jammed(mantissa: u64, shift: u32) -> u64 {
    if shift == 0 {
        return mantissa;
    }
    if shift >= 64 {
        return u64::from(mantissa != 0);
    }
    // A sticky low bit preserves the effect of all discarded bits on the final tie decision.
    (mantissa >> shift) | u64::from(mantissa & ((1u64 << shift) - 1) != 0)
}

fn round_scaled(sign: u32, mantissa: u64, exponent: i32) -> u32 {
    if mantissa == 0 {
        return sign;
    }
    let leading = 63 - mantissa.leading_zeros() as i32;
    let result_exponent = leading + exponent;
    if result_exponent < -126 {
        return sign | round_at(mantissa, -149 - exponent) as u32;
    }
    encode_normal(sign, result_exponent, round_at(mantissa, leading - 23))
}

fn encode_normal(sign: u32, mut exponent: i32, mut mantissa: u64) -> u32 {
    if mantissa >= 1 << 24 {
        exponent += 1;
        mantissa >>= 1;
    }
    if exponent > 127 {
        return sign | 0x7f800000;
    }
    sign | (((exponent + 127) as u32) << 23) | (mantissa as u32 & 0x7fffff)
}

fn round_at(mantissa: u64, shift: i32) -> u64 {
    if shift <= 0 {
        return mantissa << -shift;
    }
    if shift > 64 {
        return 0;
    }
    let half = 1u64 << (shift - 1);
    if shift == 64 {
        return u64::from(mantissa > half);
    }
    let quotient = mantissa >> shift;
    let remainder = mantissa & ((1u64 << shift) - 1);
    quotient + u64::from(remainder > half || (remainder == half && quotient & 1 != 0))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn halfway_results_choose_even_across_normal_subnormal_and_overflow_boundaries() {
        assert_eq!(round_at(5, 1), 2);
        assert_eq!(round_at(7, 1), 4);
        assert_eq!(round_at(1 << 63, 64), 0);
        assert_eq!(round_at((1 << 63) + 1, 64), 1);
        assert_eq!(round_at(1, 65), 0);
        assert_eq!(round_scaled(0, 1, -150), 0);
        assert_eq!(round_scaled(0x80000000, 1, -150), 0x80000000);
        assert_eq!(round_scaled(0, 3, -150), 2);
        assert_eq!(round_scaled(0, 0xffffff, -150), 0x00800000);
        assert_eq!(encode_normal(0, 127, 0x1000000), 0x7f800000);
    }

    #[test]
    fn addition_retains_sticky_bits_at_ties_and_obeys_signed_zero_rules() {
        assert_eq!(add_finite(0x3f800000, 0x33800000), 0x3f800000);
        assert_eq!(add_finite(0x3f800001, 0x33800000), 0x3f800002);
        assert_eq!(add_finite(0x3f800000, 0xb3000000), 0x3f800000);
        assert_eq!(add_finite(0x3f800000, 0xb3000001), 0x3f7fffff);
        assert_eq!(add_finite(0x80000000, 0), 0);
        assert_eq!(add_finite(0x80000000, 0x80000000), 0x80000000);
        assert_eq!(add_finite(0x7f7fffff, 0x7f7fffff), 0x7f800000);
    }

    #[test]
    fn multiplication_uses_gradual_underflow_and_preserves_the_zero_sign() {
        assert_eq!(multiply_finite(1, 0x3f000000), 0);
        assert_eq!(multiply_finite(3, 0x3f000000), 2);
        assert_eq!(multiply_finite(0x80000001, 0x3f000000), 0x80000000);
        assert_eq!(multiply_finite(0x80000000, 0x3f800000), 0x80000000);
    }
}
