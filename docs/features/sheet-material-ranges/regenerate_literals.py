"""Reproduce the static C-3559 literal corpus without Bessemer or Source codecs."""
from pathlib import Path
import hashlib
import json
import struct

ROOT = Path(__file__).resolve().parents[3]


def literal_material_cases() -> dict[str, bytes]:
    """Return separately framed material and direct records; e.g. literal_material_cases()['empty.bin']."""
    reference = json.loads(Path(__file__).with_name('literal-fixture-reference.json').read_text(encoding='utf-8'))
    words: list[int] = reference['integer_words']
    bits = [int(value, 16) for value in reference['binary64_patterns']]
    records = b''.join(struct.pack('<iQiQiBQ', words[i % 6], bits[i % 12], words[(i+1) % 6],
        bits[(i+1) % 12], words[(i+2) % 6], i, bits[(i+2) % 12]) for i in range(256))
    name = b'moSMParamsFromMat_c'
    tag = struct.pack('<HHH', 65535, 1, len(name)) + name
    text = 'material Ω'.encode('utf-16-le')
    table = b'\xff\xfe\xff' + bytes([len(text) // 2]) + text
    parent = struct.pack('<ii', -2147483648, 2147483647) + table + struct.pack('<i', 256) + records
    empty = struct.pack('<ii', 0, -1) + b'\xff\xfe\xff\x00' + bytes(4)
    return {'ranges.bin': records, 'parent.bin': tag + parent, 'empty.bin': tag + empty}


def verify_literal_material_cases() -> None:
    """Compare owned bytes and frozen hashes; e.g. python regenerate_literals.py."""
    reference = json.loads(Path(__file__).with_name('literal-fixture-reference.json').read_text(encoding='utf-8'))
    for name, raw in literal_material_cases().items():
        assert raw == (ROOT / 'tests/fixtures/sheet-material-ranges' / name).read_bytes(), name
        assert hashlib.sha256(raw).hexdigest() == reference['files'][name], name


if __name__ == '__main__':
    verify_literal_material_cases()
