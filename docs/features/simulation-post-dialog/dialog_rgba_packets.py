"""Complete color path/span/bit/finite-value expectations from pinned Source readers (#166)."""
import importlib
import struct
from typing import cast
from native_carriers import digest
from dialog_evidence import FEATURE, dialog_baseline
from dialog_edit_oracle import DialogEditField, decode_edit_field
from dialog_owned import dialog_owned_cases
from dialog_projection import dialog_fields
from dialog_operation_packets import accepted_dialog_native_cases, persist_dialog_packets


def rgba_group_packet(fields: list[DialogEditField], ordinal: int) -> bytes:
    """Require four independently observed physical fields before applying the color helper; e.g. rgba_group_packet(fields, 5)."""
    first = fields[ordinal]
    prefix = first.path.removesuffix('.component_0')
    group = fields[ordinal:ordinal + 4]
    assert len(group) == 4, (prefix, len(group), 'expected four components')
    for index, field in enumerate(group):
        assert (field.path, field.tag, field.start, field.end) == (f'{prefix}.component_{index}', 8, first.start + index * 4, first.start + index * 4 + 4)
    raw = b''.join(field.wire for field in group)
    view = importlib.import_module('cosworks_post_color_fields').decode_rgba(raw)
    path = prefix.encode('utf-8')
    return (struct.pack('<I', len(path)) + path + struct.pack('<II', first.start, group[-1].end)
        + raw + rgba_finite_packet(cast(dict[str, object], view)))


def rgba_finite_packet(view: dict[str, object]) -> bytes:
    """Encode finite values through independent Source numeric observations, retaining negative zero; e.g. rgba_finite_packet(view)."""
    channels = cast(dict[str, dict[str, object]], view['channels'])
    mask, values = 0, bytearray()
    for index, name in enumerate(('red', 'green', 'blue', 'alpha')):
        value = channels[name]['finite_value']
        if value is not None: mask |= 1 << index
        values.extend(struct.pack('<f', cast(float, value)) if value is not None else bytes(4))
    return struct.pack('<I', mask) + bytes(values)


def rgba_context_packet(family: int, version: int, raw: bytes) -> tuple[bytes, int]:
    """Reparse the complete original context and retain every color in physical order; e.g. rgba_context_packet(2, 700006, raw)."""
    fields = [decode_edit_field(field) for field in dialog_fields(family, version, raw)]
    groups = [rgba_group_packet(fields, index) for index, field in enumerate(fields) if field.path.endswith('.component_0')]
    assert groups, (family, version, 'expected stored style/legend colors')
    return struct.pack('<III', family, version, len(groups)) + b''.join(groups), len(groups)


def rgba_projection_packet(cases: list[tuple[int, int, bytes]]) -> tuple[bytes, int]:
    """Keep the complete owned/native argument denominator without merging repeated observations; e.g. rgba_projection_packet(cases)."""
    rows = [rgba_context_packet(*case) for case in cases]
    return struct.pack('<II', 1, len(rows)) + b''.join(row[0] for row in rows), sum(row[1] for row in rows)


def verify_rgba_packets() -> None:
    """Freeze all accepted owned/native parent and style color views with their independent finite channels; e.g. verify_rgba_packets()."""
    owned = [(family, version, raw) for family, version, _, raw in dialog_owned_cases() if family in (0, 2)]
    original = (FEATURE.parents[2] / 'target/simulation-post-dialog-native.bin').read_bytes()
    native = [case for case in accepted_dialog_native_cases(original) if case[0] in (0, 2)]
    assert (len(owned), len(native)) == (425, 276), (len(owned), len(native))
    owned_packet, owned_colors = rgba_projection_packet(owned)
    native_packet, native_colors = rgba_projection_packet(native)
    packets = persist_dialog_packets(FEATURE.parents[2] / 'tests/fixtures/simulation-post-dialog', {'rgba-views.bin': owned_packet})
    packets.update(persist_dialog_packets(FEATURE.parents[2] / 'target', {'simulation-post-rgba-native.bin': native_packet}))
    dialog_baseline('rgba-operation-packets-reference.json', dict(owned_contexts=len(owned), native_helper_contexts=len(native),
        owned_colors=owned_colors, native_color_observations=native_colors, native_input_sha256=digest(original),
        additional_stored_byte_credit=0, packets=packets))
