"""Independent primitive expansion and record scoping for counted MeshControl bodies (#161)."""
from functools import partial
from typing import cast
import struct
from thermal_projection import FieldShape, projected_fields
from shell_base_projection import shell_base_shape

DERIVED_GROUPS: dict[str, tuple[tuple[str, int, int], ...]] = {
    'mesh.two_doubles_three_words': (('double370', 2, 8), ('double378', 2, 8), ('word380', 0, 4), ('word388', 0, 4), ('word24', 0, 4)),
    'mesh.word320_double328': (('word320', 0, 4), ('double328', 2, 8)),
    'mesh.words330_334': (('word330', 0, 4), ('word334', 0, 4)),
    'mesh.words3c8_3cc': (('word3c8', 0, 4), ('word3cc', 0, 4)),
    'mesh.two_doubles_five_words': (('double390', 2, 8), ('double398', 2, 8), ('word3a0', 0, 4), ('word3a4', 0, 4),
                                   ('word3a8', 0, 4), ('word368', 0, 4), ('word36c', 0, 4)),
}


def mesh_record_span_shape(label: str, size: int, prefix: str) -> list[FieldShape]:
    """Expand only documented physical fields under their containing record; e.g. mesh_record_span_shape('base.word198', 4, 'record0')."""
    if label.startswith(('references.', 'reference', 'base.')):
        return [(prefix + path.removeprefix('property'), tag, width) for path, tag, width in shell_base_shape(label, size)]
    if label in DERIVED_GROUPS:
        shape = DERIVED_GROUPS[label]
        assert sum(width for _, _, width in shape) == size, (label, size)
        return [(prefix + '.mesh.' + name, tag, width) for name, tag, width in shape]
    if label in ('mesh.text2e8', 'mesh.text2f0', 'mesh.text310', 'mesh.text318'):
        return [(prefix + '.' + label, 3, size)]
    assert label == 'mesh.word384' and size == 4, f'{label}/{size}; expected a documented MeshControl field'
    return [(prefix + '.' + label, 0, 4)]


def mesh_span_owner(offset: int, records: list[dict[str, int]]) -> int:
    """Require one physical record owner for every non-count span; e.g. mesh_span_owner(4, records)."""
    owners = [record['index'] for record in records if record['start'] <= offset < record['end']]
    assert len(owners) == 1, f'{offset}/{owners}; expected exactly one containing record'
    return owners[0]


def mesh_record_fields(raw: bytes, parsed: dict[str, object]) -> list[bytes]:
    """Retain each equal-payload occurrence as separate scoped fields; e.g. mesh_record_fields(raw, parsed)."""
    assert parsed['success'] and parsed['end'] == len(raw), parsed
    fields: list[bytes] = []
    records = cast(list[dict[str, int]], parsed['records'])
    for span in cast(list[dict[str, object]], parsed['spans']):
        if span['label'] == 'mesh_control.count':
            assert span['offset'] == 0 and span['end'] == 4, span
            fields.extend(projected_fields(raw, [span]))
            continue
        owner = mesh_span_owner(cast(int, span['offset']), records)
        fields.extend(projected_fields(raw, [span], partial(mesh_record_span_shape, prefix=f'record{owner}')))
    return fields


def mesh_record_projection(version: int, raw: bytes, parsed: dict[str, object]) -> bytes:
    """Frame full typed values and original absolute spans under the sole declared reader grammar; e.g. mesh_record_projection(version, raw, parsed)."""
    fields = mesh_record_fields(raw, parsed)
    return struct.pack('<III', 0, version, len(raw)) + raw + struct.pack('<I', len(fields)) + b''.join(fields)
