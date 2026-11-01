"""Complete shell Property primitive fields, including scoped inline Material (#159)."""
from functools import partial
from typing import cast
import re
import struct
from thermal_projection import FieldShape, projected_fields, repeated_shape
from material_projection import material_shape
from shell_base_projection import shell_base_shape


def shell_derived_shape(label: str, size: int) -> list[FieldShape]:
    """Split only documented derived/layer primitive groups; e.g. shell_derived_shape('shell.word368_double360', 12)."""
    if label == 'shell.word368_double360':
        assert size == 12, (label, size)
        return [('shell.word368', 0, 4), ('shell.double360', 2, 8)]
    if label in ('shell.words338_33c', 'shell.words350_354', 'shell.words370_374'):
        assert size == 8, (label, size)
        return repeated_shape(label, 0 if label.endswith('370_374') else 1, 4, 2)
    if label.endswith('.two_doubles'):
        assert re.fullmatch(r'layer\d+\.two_doubles', label) and size == 16, (label, size)
        return repeated_shape(label.removesuffix('.two_doubles') + '.doubles', 2, 8, 2)
    if label in ('shell.text300', 'shell.text348', 'shell.text488') or re.fullmatch(r'layer\d+\.descriptor[12]\.text', label):
        return [(label, 3, size)]
    if label == 'shell.double328' or re.fullmatch(r'layer\d+\.descriptor[12]\.double', label):
        assert size == 8, (label, size)
        return [(label, 2, 8)]
    assert label in ('shell.word330', 'shell.subtype334', 'shell.word340', 'shell.word358', 'shell.layer_count', 'shell.word378', 'shell.word480') or re.fullmatch(r'layer\d+\.(material_presence|flags)', label), label
    assert size == 4, (label, size)
    return [(label, 0, 4)]


def scoped_material_shape(label: str, size: int, prefix: str) -> list[FieldShape]:
    """Keep Material's established primitive grammar under its containing layer; e.g. scoped_material_shape('word68', 4, 'layer0.material')."""
    return [(prefix + '.' + path, tag, width) for path, tag, width in material_shape(label, size)]


def inline_shell_material(raw: bytes, version: int, span: dict[str, object]) -> list[bytes]:
    """Require exact bounded Material EOF before expanding into parent-relative fields; e.g. inline_shell_material(raw, 840005, span)."""
    from cosworks_material_layout import parse
    start, end = cast(int, span['offset']), cast(int, span['end'])
    material = parse(raw[start:end], version)
    assert material['success'] and material['end'] == end - start, material
    relocated = [dict(item, offset=cast(int, item['offset']) + start, end=cast(int, item['end']) + start)
                 for item in cast(list[dict[str, object]], material['spans'])]
    shape = partial(scoped_material_shape, prefix=cast(str, span['label']))
    return projected_fields(raw, relocated, shape)


def shell_property_fields(raw: bytes, version: int, parsed: dict[str, object]) -> list[bytes]:
    """Account for every physical byte, including zero-length reference blobs; e.g. shell_property_fields(raw, 891106, parsed)."""
    assert parsed['success'] and parsed['end'] == len(raw), parsed
    fields: list[bytes] = []
    for span in cast(list[dict[str, object]], parsed['spans']):
        label = cast(str, span['label'])
        if re.fullmatch(r'layer\d+\.material', label):
            fields.extend(inline_shell_material(raw, version, span))
            continue
        shape = shell_base_shape if label.startswith(('references.', 'reference', 'base.')) else shell_derived_shape
        fields.extend(projected_fields(raw, [span], shape))
    return fields


def shell_presence_packet(presence: list[bool] | None) -> bytes:
    """Keep unknown, known-empty and actual boolean vectors distinct; e.g. shell_presence_packet([False, True])."""
    if presence is None: return struct.pack('<i', -1)
    assert all(type(value) is bool for value in presence), presence
    return struct.pack('<i', len(presence)) + bytes(presence)


def shell_property_projection(kind: int, version: int, raw: bytes, parsed: dict[str, object], presence: list[bool] | None) -> bytes:
    """Retain explicit context and every typed field/path/span/raw value; e.g. shell_property_projection(0, 891106, raw, parsed, presence)."""
    fields = shell_property_fields(raw, version, parsed)
    return shell_presence_packet(presence) + struct.pack('<III', kind, version, len(raw)) + raw + struct.pack('<I', len(fields)) + b''.join(fields)
