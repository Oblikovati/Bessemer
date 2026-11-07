"""Independent literal USD framing and labeled writer cases (#123)."""
import struct
from dataclasses import dataclass

PROFILE_LABELS = (
    'Win32_BaseBoard SerialNumber', 'Win32_BaseBoard Product',
    'Win32_BIOS SerialNumber', 'Win32_PhysicalMedia SerialNumber',
    'Operating System', 'Update JanusLock Key Entry', 'Product Name',
    'Memory Status', 'CPU Status', 'Virtual Machine or not',
    'Graphic Card', 'CPU Process Type',
)


@dataclass(frozen=True)
class RecordCase:
    header: bytes
    payload: bytes
    width: int
    count: int


def prefix(count: int, width: int, escaped_bytes: int) -> bytes:
    """Build literal escape shapes without consulting the reference length parser."""
    marker = b'\xff\xfe\xff' if width == 2 else b''
    if escaped_bytes == 1:
        return marker + bytes([count])
    if escaped_bytes == 2:
        return marker + b'\xff' + struct.pack('<H', count)
    if escaped_bytes == 4:
        return marker + b'\xff\xff\xff' + struct.pack('<I', count)
    assert escaped_bytes == 8
    return marker + b'\xff' * 7 + struct.pack('<Q', count)


def payload(count: int, width: int) -> bytes:
    """Include counted NUL, opaque narrow octets and unpaired UTF-16 code units."""
    pattern = bytes.fromhex('4100000000d83dd800de2000') if width == 2 else bytes.fromhex('004180ff200a0d')
    length = count * width
    return (pattern * (length // len(pattern) + 1))[:length]


def record_cases() -> list[RecordCase]:
    """Cross canonical and noncanonical widths, including a repeated Unicode marker count."""
    cases = []
    for width in (1, 2):
        for escaped, counts in ((1, (0, 1, 254)), (2, (0, 1, 254, 255, 65533)),
                                (4, (0, 1, 254, 255, 65533, 65534, 65535, 65536)),
                                (8, (0, 1, 254, 255, 65533, 65534, 65535, 65536))):
            cases.extend(RecordCase(prefix(count, width, escaped), payload(count, width), width, count) for count in counts)
    cases.append(RecordCase(prefix(65534, 2, 2), payload(65534, 2), 2, 65534))
    return cases


def canonical_unicode(units: bytes) -> bytes:
    """Choose the documented provider's shortest count branch while retaining raw units."""
    assert len(units) % 2 == 0
    count = len(units) // 2
    escaped = 1 if count < 255 else 2 if count < 65534 else 4 if count < 0xffffffff else 8
    return prefix(count, 2, escaped) + units


def writer_values(adapters: int) -> list[str]:
    """Assemble literal writer structure independently of the Source layout mapper."""
    sep = '-' * 42
    values = [sep, 'host\0suffix', sep, sep, '-123', sep]
    for number in range(adapters):
        values.extend([sep, f'adapter {number}', sep, 'AA BB CC DD EE FF', '6', 'mask', 'gateway', 'wins', 'dhcp', ''])
    values.extend([sep, 'The Customer Profile', sep, 'c1', 'c2', 'c3', 'c4', 'c5', ''])
    for number, label in enumerate(PROFILE_LABELS):
        values.extend([sep, label, sep, f' raw {number}\0\ud800 \u4e2d '])
        if number < len(PROFILE_LABELS) - 1:
            values.append('')
    return values + ['#EOF']


def writer_packet(adapters: int) -> tuple[bytes, list[bytes]]:
    """Preserve independently chosen Unicode payloads and every exact record boundary."""
    values = [value.encode('utf-16-le', 'surrogatepass') for value in writer_values(adapters)]
    return b''.join(canonical_unicode(value) for value in values), values


def edit_cases() -> list[tuple[bytes, int, bytes, bytes]]:
    """Patch a literal middle record independently of the Source iterator's offsets."""
    first = b'\x04#EOF'
    middle = canonical_unicode(bytes.fromhex('4100000000d8'))
    last = bytes.fromhex('ff0100e9')
    base = first + middle + last
    replacements = [case.header + case.payload for case in record_cases()] + [middle]
    return [(base, 1, replacement, first + replacement + last) for replacement in replacements]
