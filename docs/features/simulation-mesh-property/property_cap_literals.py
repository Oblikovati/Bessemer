"""Independent complete payloads on both sides of every Mesh Property fixed count cap (#163)."""
from collections.abc import Callable
from functools import partial
import struct
from owned_thermal import thermal_reference
from owned_property import owned_property_literal
from property_literal_values import assemble_property_blocks, property_words
from property_nested_literals import nested_property_blocks

PropertyCapBuilder = Callable[[int], tuple[int, bytes]]


def property_cap_replace(version: int, profile: int, first: str, last: str, replacement: bytes) -> tuple[int, bytes]:
    """Replace only independently built literal extents, never parser-discovered padding; e.g. property_cap_replace(v, 0, label, label, raw)."""
    case = owned_property_literal(version, profile, profile == 0)
    starts = [span['offset'] for span in case['spans'] if span['label'] == first]
    ends = [span['end'] for span in case['spans'] if span['label'] == last]
    assert len(starts) == len(ends) == 1, f'{first}/{last}; expected unique complete literal boundaries'
    return version, case['raw'][:starts[0]] + replacement + case['raw'][ends[0]:]


def property_root_cap(version: int, label: str, element: bytes, count: int) -> tuple[int, bytes]:
    """Supply every declared fixed-width root element; e.g. property_root_cap(891103, 'mesh.newest_set.count', bytes(4), 10000)."""
    return property_cap_replace(version, 0, label, label, struct.pack('<i', count) + element * count)


def property_blob_cap(count: int) -> tuple[int, bytes]:
    """Retain the complete containing legacy reference after a counted opaque blob; e.g. property_blob_cap(1000000)."""
    prefix = 'mesh.settings.reference0'
    return property_cap_replace(800021, 1, prefix + '.blob_count', prefix + '.blob', struct.pack('<i', count) + b'\xa5' * count)


def property_direct_cap_payload(version: int, extension: str, count: int) -> bytes:
    """Build one complete direct record with a fully supplied nested extension; e.g. property_direct_cap_payload(850030, 'extension38', 10000)."""
    nested, _ = assemble_property_blocks(nested_property_blocks(version, 0, (0, 0)))
    raw = property_words(9, 0)
    if extension == 'extension38': raw += struct.pack('<i', 0)
    return struct.pack('<i', 1) + raw + struct.pack('<ii', 1, count) + nested * count


def property_extension_cap(extension: str, count: int) -> tuple[int, bytes]:
    """Keep the other extension disabled rather than omitting its mode word; e.g. property_extension_cap('extension38', 10000)."""
    version = 850002 if extension == 'extension2c' else 850030
    label = 'mesh.direct_records.count'
    return property_cap_replace(version, 0, label, label, property_direct_cap_payload(version, extension, count))


def property_nested_array_cap(index: int, count: int) -> tuple[int, bytes]:
    """Bound both 100000-entry raw64 arrays independently with complete continuation; e.g. property_nested_array_cap(1, 100000)."""
    counts = (count, 0) if index == 0 else (0, count)
    nested, _ = assemble_property_blocks(nested_property_blocks(850029, 0, counts))
    raw = struct.pack('<i', 1) + property_words(9, 0) + struct.pack('<ii', 1, 1) + nested
    label = 'mesh.direct_records.count'
    return property_cap_replace(850029, 0, label, label, raw)


def property_word_array_cap(index: int, count: int) -> tuple[int, bytes]:
    """Supply both declared counts before either word array, preserving the empty side; e.g. property_word_array_cap(0, 10000)."""
    counts = (count, 0) if index == 0 else (0, count)
    raw = struct.pack('<i', 1) + property_words(5, 0) + struct.pack('<ii', *counts) + bytes(4 * count)
    label = 'mesh.array_objects.count'
    return property_cap_replace(850001, 0, label, label, raw)


def property_child_cap(count: int) -> tuple[int, bytes]:
    """Retain the complete older keyed header and every forty-byte child; e.g. property_child_cap(10000)."""
    raw = struct.pack('<i', 1) + bytes(44) + struct.pack('<i', count) + bytes(40 * count)
    label = 'mesh.map.count'
    return property_cap_replace(890703, 0, label, label, raw)


def property_root_cap_builders() -> tuple[tuple[str, int, PropertyCapBuilder], ...]:
    """Keep the distinct root collections and their documented element widths; e.g. property_root_cap_builders()."""
    controls = (('text_list', 800021, 'mesh.text_list.count', bytes(4)),
                ('references', 800021, 'mesh.settings.references.count', thermal_reference(800021)),
                ('direct_records', 800074, 'mesh.direct_records.count', bytes(36)),
                ('duration', 840017, 'mesh.duration_count', bytes(4)),
                ('array_objects', 850001, 'mesh.array_objects.count', bytes(28)),
                ('keyed_parents', 890703, 'mesh.map.count', bytes(48)),
                ('double_array', 890907, 'mesh.double_array.count', bytes(8)),
                ('newest_keys', 891103, 'mesh.newest_set.count', bytes(4)))
    return tuple((name, 10000, partial(property_root_cap, version, label, element)) for name, version, label, element in controls)


def property_cap_builders() -> tuple[tuple[str, int, PropertyCapBuilder], ...]:
    """Cover all sixteen independent fixed caps with positive and one-above payloads; e.g. property_cap_builders()."""
    return property_root_cap_builders() + (('blob', 1000000, property_blob_cap),
        ('extension2c', 10000, partial(property_extension_cap, 'extension2c')),
        ('extension38', 10000, partial(property_extension_cap, 'extension38')),
        ('nested_array0', 100000, partial(property_nested_array_cap, 0)),
        ('nested_array1', 100000, partial(property_nested_array_cap, 1)),
        ('word_array0', 10000, partial(property_word_array_cap, 0)),
        ('word_array1', 10000, partial(property_word_array_cap, 1)),
        ('keyed_children', 10000, property_child_cap))
