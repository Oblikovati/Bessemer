"""Independent typed physical PostInfo partition from the reviewed fixed460 schema (#166)."""
from thermal_projection import FieldShape


def postinfo_field_shapes(size: int) -> list[FieldShape]:
    """Keep every header and all twelve fixed records without using active counts as lengths; e.g. postinfo_field_shapes(460)."""
    assert size == 460, f'{size}; expected exact PostInfo460 partition'
    header = [(f'post_info.header[{index}]', 0 if index in (2, 5, 6) else 7, 4) for index in range(7)]
    records = [field for group in ('section', 'iso') for index in range(6) for field in postinfo_record_shapes(group, index)]
    return header + records


def postinfo_record_shapes(group: str, index: int) -> list[FieldShape]:
    """Retain raw enum values and exact float32 bits instead of imposing geometry meanings; e.g. postinfo_record_shapes('iso', 0)."""
    prefix = f'post_info.{group}[{index}]'
    fields = [(prefix + '.primitive_type', 0, 4), (prefix + '.cut_direction', 0, 4)]
    fields += [(f'{prefix}.normal[{component}]', 8, 4) for component in range(3)]
    fields += [(f'{prefix}.center[{component}]', 8, 4) for component in range(3)]
    return fields + [(prefix + '.shared_scalar', 8, 4)]
