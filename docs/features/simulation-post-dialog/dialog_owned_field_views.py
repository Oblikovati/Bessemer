"""Closed-form expected PostInfo slot roles and float bit aliases for owned literals (#166)."""
import importlib
from typing import cast
from dialog_evidence import dialog_baseline
from dialog_initial_literals import postinfo_block
from dialog_literal_values import DialogLiteralProfile, FLOAT32_PATTERNS


def verify_owned_postinfo_field_views() -> None:
    """Check five full bit rotations without deriving expected field values from the decoder; e.g. verify_owned_postinfo_field_views()."""
    rows: list[dict[str, object]] = []
    decode = importlib.import_module('cosworks_post_info_fields').decode_block
    for rotation in range(5):
        raw = postinfo_block(DialogLiteralProfile(rotation=rotation))
        actual = decode(raw)
        records = cast(list[dict[str, object]], actual['records'])
        assert len(records) == 12, len(records)
        for index, record in enumerate(records): assert_owned_slot(record, index, rotation)
        header = cast(list[dict[str, object]], actual['header_words'])
        assert len(header) == 7 and actual['section_active_count'] == header[5] and actual['iso_active_count'] == header[6]
        assert bytes.fromhex(cast(str, actual['raw_hex'])) == raw
        rows.append(dict(rotation=rotation, fields=actual))
    dialog_baseline('owned-postinfo-fields-reference.json', dict(contexts=5, physical_slots=60, rows=rows))
    from dialog_cut_view_packets import verify_cut_view_packet
    verify_cut_view_packet()


def assert_owned_slot(record: dict[str, object], index: int, rotation: int) -> None:
    """Keep fixed group membership, known/unknown enum views and raw coordinates exact; e.g. assert_owned_slot(record, 0, 1)."""
    offset = 28 + index * 36
    assert (record['offset'], record['end'], record['group'], record['index']) == (offset, offset + 36, 'section' if index < 6 else 'iso', index % 6)
    kind = (index + rotation) % 5
    direction = (-1, 1, 2, -2147483648, 2147483647)[(index + rotation) % 5]
    assert cast(dict[str, object], record['primitive_type'])['signed_view'] == kind
    assert record['primitive_name'] == {0: 'cylinder', 1: 'plane', 2: 'sphere', 3: 'iso'}.get(kind)
    assert cast(dict[str, object], record['cut_direction'])['signed_view'] == direction
    assert record['cut_direction_display_token'] == {-1: 'Yon', 1: 'Hither', 2: 'Both'}.get(direction)
    components = cast(list[dict[str, object]], record['normal_components']) + cast(list[dict[str, object]], record['center_components']) + [cast(dict[str, object], record['shared_scalar'])]
    for component, scalar in enumerate(components):
        bits = FLOAT32_PATTERNS[(index + component + rotation) % 5]
        assert_owned_scalar(scalar, bits, offset + 8 + component * 4)
    bits = FLOAT32_PATTERNS[(index + 6 + rotation) % 5]
    negated = cast(dict[str, object], record['negated_D_view'])
    assert negated['stored_raw_hex'] == bits.to_bytes(4, 'little').hex()
    assert negated['view_raw_hex'] == (bits ^ 0x80000000).to_bytes(4, 'little').hex()


def assert_owned_scalar(scalar: dict[str, object], bits: int, offset: int) -> None:
    """Use closed-form zero/subnormal values and exponent masks for nonfinite patterns; e.g. assert_owned_scalar(scalar, 1, 36)."""
    assert scalar['offset'] == offset
    raw = bits.to_bytes(4, 'little').hex()
    assert scalar['stored_raw_hex'] == scalar['view_raw_hex'] == raw
    expected = None if bits & 0x7f800000 == 0x7f800000 else (2 ** -149 if bits == 1 else 0.0)
    assert scalar['finite_value'] == expected, (bits, scalar)
