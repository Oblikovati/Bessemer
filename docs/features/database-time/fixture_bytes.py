"""Independent arbitrary-precision oracles for C-3597 packed time (#223)."""
from pathlib import Path
import hashlib
import json
import struct

ROOT = Path(__file__).resolve().parents[3]
BOUNDARIES = (-2147483648, -2147483647, -65537, -65536, -32769, -129, -128,
    -68, -33, -3, -2, -1, 0, 1, 2, 3, 15, 16, 23, 24, 31, 32, 59, 60, 61,
    62, 63, 64, 127, 128, 1979, 1980, 2026, 2107, 2108, 65535, 65536, 2147483647)


def packed_word(components: tuple[int, ...]) -> int:
    """Use unbounded integers and one final modulo, rather than native wrapping steps; e.g. packed_word((0,)*6)."""
    year, month, day, hour, minute, seconds = components
    date = (((year + 68) * 16 + month) * 2097152) + day * 65536
    clock = ((hour * 64 + minute) * 32) + seconds // 2
    return (date | clock) % 4294967296


def extracted_components(word: int) -> tuple[int, ...]:
    """Extract mixed-radix bit fields by division/remainder; e.g. extracted_components(0)."""
    return (word // 33554432 + 1980, word // 2097152 % 16, word // 65536 % 32,
        word // 2048 % 32, word // 32 % 64, word % 32 * 2)


def input_vectors() -> list[tuple[int, ...]]:
    """Include raw boundaries, a zeroed callback record and deterministic full-width inputs; e.g. input_vectors()."""
    baseline = (2026, 9, 24, 8, 9, 12)
    vectors = [(0,) * 6, baseline]
    for component in range(6):
        for value in BOUNDARIES:
            changed = list(baseline)
            changed[component] = value
            vectors.append(tuple(changed))
    seed = 0x3597d00d
    for _ in range(8192):
        components = []
        for _ in range(6):
            seed = (1664525 * seed + 1013904223) % 4294967296
            components.append(seed if seed < 2147483648 else seed - 4294967296)
        vectors.append(tuple(components))
    return vectors


def time_packet() -> bytes:
    """Persist all inputs, expected words and extracted fields; e.g. time_packet()."""
    vectors = input_vectors()
    records = [struct.pack('<6iI6i', *components, packed_word(components),
        *extracted_components(packed_word(components))) for components in vectors]
    return b'DBT1' + struct.pack('<I', len(records)) + b''.join(records)


def verify_time_packet() -> None:
    """Compare the frozen independent packet and hash; e.g. python fixture_bytes.py."""
    packet = time_packet()
    reference = json.loads(Path(__file__).with_name('fixture-reference.json').read_text(encoding='utf-8'))
    assert packet == (ROOT / 'tests/fixtures/database-time/cases.bin').read_bytes()
    assert hashlib.sha256(packet).hexdigest() == reference['packet_sha256']
    assert len(input_vectors()) == reference['cases']


if __name__ == '__main__':
    verify_time_packet()
