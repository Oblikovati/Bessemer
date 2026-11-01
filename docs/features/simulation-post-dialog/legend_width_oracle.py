"""Independent exact-rational oracle for C854's explicit nearest-even projection (#166)."""
from fractions import Fraction


def width_fraction(bits: int) -> Fraction:
    """Decode any nonnegative finite binary32 pattern as an exact rational; e.g. width_fraction(0x3f800000)."""
    assert 0 <= bits < 0x7f800000, bits
    exponent, fraction = bits >> 23, bits & 0x7fffff
    significand = fraction if exponent == 0 else fraction + 0x800000
    power = -149 if exponent == 0 else exponent - 150
    return Fraction(significand * (1 << power)) if power >= 0 else Fraction(significand, 1 << -power)


def nearest_width_fraction(value: Fraction) -> int:
    """Find adjacent encodings by monotonic binary search and resolve midpoint parity; e.g. nearest_width_fraction(Fraction(1))."""
    assert 0 <= value <= width_fraction(0x7f7fffff), value
    if value == 0: return 0
    low, high = 0, 0x7f7fffff
    while high - low > 1:
        middle = (low + high) // 2
        if width_fraction(middle) <= value: low = middle
        else: high = middle
    lower_error, upper_error = value - width_fraction(low), width_fraction(high) - value
    if lower_error == upper_error: return low if low % 2 == 0 else high
    return low if lower_error < upper_error else high


def rational_width_patterns(font: int, selector: int) -> tuple[int, int, int, int, int]:
    """Round each exact rational stage separately, preserving native branch identity; e.g. rational_width_patterns(1, 0)."""
    assert -2147483648 <= font <= 2147483647 and -2147483648 <= selector <= 2147483647, (font, selector)
    converted = nearest_width_fraction(Fraction(abs(font)))
    scaled = nearest_width_fraction(width_fraction(converted) * width_fraction(0x3a83126f))
    initial = nearest_width_fraction(width_fraction(scaled) * Fraction(5, 8))
    factor, operation = (4, 2) if selector == 0 else (2, 1) if selector == 1 else (1, 0)
    selected = nearest_width_fraction(width_fraction(initial) * factor)
    sign = 0x80000000 if font < 0 else 0
    return (*(bits | sign for bits in (converted, scaled, initial, selected)), operation)


def verify_width_rational_domain() -> None:
    """Check midpoint parity, integer conversion ties and whole-domain normal/finite bounds; e.g. verify_width_rational_domain()."""
    from dialog_evidence import dialog_baseline
    assert nearest_width_fraction(Fraction(0)) == 0 and width_fraction(0x3f800000) == 1
    for lower in (0, 1, 0x7ffffe, 0x7fffff, 0x800000, 0x3f7fffff, 0x3f800000, 0x4b800000):
        midpoint = (width_fraction(lower) + width_fraction(lower + 1)) / 2
        assert nearest_width_fraction(midpoint) == (lower if lower % 2 == 0 else lower + 1)
        assert nearest_width_fraction(width_fraction(lower)) == lower
    assert rational_width_patterns(16777217, 2)[0] == 0x4b800000
    assert rational_width_patterns(16777219, 2)[0] == 0x4b800002
    assert Fraction(1, 1024) < width_fraction(0x3a83126f) < Fraction(3, 2048)
    assert Fraction(1, 2) < width_fraction(0x3f200000) < 1
    assert (2**24 - 1)**2 < 2**48 and 2**31 * Fraction(3, 2048) * 4 < 2**24
    dialog_baseline('legend-width-domain-reference.json', dict(rounding='nearest-even',
        converted_nonzero_magnitude_bounds=[1, 2**31], nonzero_width_lower_bound_power=-11,
        width_upper_bound_power=24, lower_bound_normal=True, upper_bound_finite=True,
        exact_significand_product_bits=48, native_sse_state_inferred=False,
        proof='Converted nonzero signed32 magnitudes lie in[1,2^31]. Scale lies between2^-10 and3*2^-11;5/8 lies between1/2 and1. Representable closed bounds keep rounded nonzero stages at least2^-11 and at most3*2^20 before final scaling, hence below2^24 afterward. Products of two24-bit significands fit48 bits.'))
