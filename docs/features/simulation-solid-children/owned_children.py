"""Independent component literals and static folder field vectors from C-730 (#149)."""
import struct
from owned_components import signed_words
from owned_thermal import persisted_literal

StaticField = tuple[str, int, bytes]


def child_text(profile: int) -> bytes:
    """Keep null, empty, unpaired and odd raw frames distinct; e.g. child_text(3)."""
    return persisted_literal((None, b'\0\0', b'\x00\xd8\0\0', b'\x33\0\0')[profile])


def component_literal(version: int, profile: int) -> bytes:
    """Encode only the direct component text/word body; e.g. component_literal(840013, 2)."""
    raw = child_text(profile) + signed_words(-2147483648, 2147483647, -1)
    if version >= 800066: raw += signed_words(2)
    if version >= 840013: raw += signed_words(-3)
    return raw


def folder_fields(version: int, profile: int) -> list[StaticField]:
    """Keep a reviewed literal field vector rather than claiming a native folder replay; e.g. folder_fields(870017, 1)."""
    values = ((), (b'A\0\0\0', b'A\0\0\0', b'\x00\xd8\0\0'),
              (None, b'\x33\0\0'), (b'\0\0',))[profile]
    fields: list[StaticField] = [('folder.word58', 0, signed_words(-2147483648)),
                                ('folder.text60', 3, child_text(profile)),
                                ('folder.list68.count', 0, signed_words(len(values)))]
    fields.extend((f'folder.list68.{index}', 3, persisted_literal(value)) for index, value in enumerate(values))
    if version >= 840014: fields.append(('folder.worda0', 0, signed_words(2)))
    if version >= 870017: fields.append(('folder.wordac', 0, signed_words(-3)))
    return fields


def child_versions() -> list[int]:
    """Include every component/folder threshold neighbor; e.g. child_versions()."""
    gates = (800066, 840013, 840014, 870017)
    return sorted({0, 2147483647, *(gate + delta for gate in gates for delta in (-1, 0, 1))})


def owned_component_headers() -> list[tuple[int, int, bytes]]:
    """Keep four raw profiles for each component context; e.g. owned_component_headers()."""
    return [(profile, version, component_literal(version, profile)) for version in child_versions() for profile in range(4)]


def folder_projection_case(version: int, profile: int) -> bytes:
    """Export complete static literal fields, offsets and bytes; e.g. folder_projection_case(840014, 2)."""
    fields = folder_fields(version, profile)
    raw = b''.join(value for _, _, value in fields)
    projected = bytearray()
    offset = 0
    for label, tag, value in fields:
        name = label.encode()
        projected.extend(struct.pack('<I', len(name)) + name + struct.pack('<III', tag, offset, offset + len(value)) + value)
        offset += len(value)
    return struct.pack('<II', version, len(raw)) + raw + struct.pack('<I', len(fields)) + projected
