"""Independent value projection for complete neutral-partition field bytes."""
import struct


def neutral_pointer_values(raw: bytes) -> list[str]:
    values: list[str] = []
    offset = 0
    try:
        while offset < len(raw):
            first = struct.unpack_from('>h', raw, offset)[0]
            offset += 2
            if first < 0:
                high = struct.unpack_from('>H', raw, offset)[0]
                offset += 2
                first = (-first-1) | (high << 15)
            values.append(f'i:{first}')
    except struct.error as error:
        raise ValueError(f'pointer bytes {raw.hex()}; expected complete one/two-word indices') from error
    return values


def neutral_field_values(kind: str, raw: bytes) -> list[str]:
    if kind == 'p':
        return neutral_pointer_values(raw)
    if kind in ('u', 'c', 'l'):
        return [f'i:{value}' for value in raw]
    widths = {'d': 4, 'n': 2, 'w': 2, 'f': 8, 'i': 16, 'v': 24, 'h': 24, 'b': 48}
    if kind not in widths or len(raw) % widths[kind]:
        raise ValueError(f'neutral field kind {kind!r}, bytes {len(raw)}; expected a supported complete partition field')
    shape, width = {'d': ('i', 4), 'n': ('h', 2), 'w': ('H', 2)}.get(kind, ('Q', 8))
    values = struct.unpack('>'+shape*(len(raw)//width), raw)
    return [f'i:{value}' if kind in ('d', 'n', 'w') else f'f:{value:016x}' for value in values]
