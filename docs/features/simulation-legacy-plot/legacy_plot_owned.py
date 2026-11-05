"""Independent literal historical layouts with populated text and fixed-buffer boundaries (#174)."""
import importlib
import struct
from legacy_plot_evidence import legacy_plot_baseline

OWNED_VERSIONS = (0, 700000, 700001, 700002, 700003, 700004, 700005, 700006, 700017, 700018, 700334, 700335, 2147483647)
WIDE_POSITIONS = ((0x30, 40, 128), (0x13c, 308, 128), (0x23c, 564, 128), (0x33c, 820, 128), (0x454, 1100, 64))
TEXT_PAYLOADS = (b'', b'\x00\xd8', b'A\0\0\0B\0', b'\xff\xdf' * 512)


def owned_legacy_fixed(variant: int) -> bytes:
    """Populate all fixed bytes without relying on the reference field table; e.g. owned_legacy_fixed(2)."""
    raw = bytearray((index * 37 + variant * 53) % 256 for index in range(1344))
    for _, offset, limit in WIDE_POSITIONS:
        raw[offset:offset + limit * 2] = b'\0\xd8' * limit
        if variant == 0: continue
        stop = (0, limit - 1, limit // 2)[variant - 1]
        raw[offset + stop * 2:offset + stop * 2 + 2] = b'\0\0'
    return bytes(raw)


def owned_legacy_plot(version: int, variant: int) -> bytes:
    """Build all historical windows with literal sizes and raw counted texts; e.g. owned_legacy_plot(700018, 2)."""
    if version <= 700002:
        size = 1360 if version == 700002 else 1328
        return bytes((index * 41 + variant * 59) % 256 for index in range(size))
    raw = owned_legacy_fixed(variant)
    for threshold in (700003, 700005, 700006):
        if version < threshold: continue
        payload = TEXT_PAYLOADS[variant]
        raw += struct.pack('<i', len(payload) // 2) + payload
    if version >= 700018: raw += bytes.fromhex('01000080ffffffff')
    return raw


def owned_legacy_cases() -> list[tuple[int, int, bytes]]:
    """Retain every version boundary and four distinct payload profiles; e.g. owned_legacy_cases()."""
    return [(version, variant, owned_legacy_plot(version, variant)) for version in OWNED_VERSIONS for variant in range(4)]


def verify_legacy_owned_grammar() -> None:
    """Check literal total widths and all bounded raw views against the frozen source; e.g. verify_legacy_owned_grammar()."""
    model = importlib.import_module('cosworks_legacy_plot_record')
    for version, variant, raw in owned_legacy_cases():
        result = model.parse(raw, version)
        assert result['success'] and result['end'] == len(raw), (version, variant, result)
        assert result['named_stream_dispatch_enabled'] == (version < 700335)
        if version < 700004: continue
        views = model.fixed_wide_views(result, version=version)
        assert [(view['member'], view['offset'], view['code_unit_limit']) for view in views] == list(WIDE_POSITIONS)
        for view in views:
            assert bytes.fromhex(view['prefix_raw_hex'] + view['remainder_raw_hex']) == raw[view['offset']:view['end']]
            assert view['terminator_present'] == (variant != 0), (version, variant, view)


def verify_legacy_owned_failures() -> None:
    """Check every proper prefix, trailing byte and invalid declared character count; e.g. verify_legacy_owned_failures()."""
    parse = importlib.import_module('cosworks_legacy_plot_record').parse
    prefixes = 0
    for version, variant, raw in owned_legacy_cases():
        for end in range(len(raw)):
            assert not parse(raw[:end], version)['success'], (version, variant, end)
            prefixes += 1
        assert not parse(raw + b'X', version)['success'], (version, variant)
        if version <= 700002: continue
        for count in (-2147483648, -1, 513, 2147483647):
            mutated = raw[:1344] + struct.pack('<i', count) + raw[1348:]
            assert not parse(mutated, version)['success'], (version, variant, count)
    legacy_plot_baseline('owned-grammar-reference.json', dict(cases=len(owned_legacy_cases()),
        proper_prefixes=prefixes, variants=4, versions=list(OWNED_VERSIONS), character_profile_cap=512))
