"""Compose material fixtures through the existing independently reviewed sheet builder."""
from pathlib import Path
import struct
import sys

ROOT = Path(__file__).resolve().parents[3]
for name in ('sheet-metal', 'model-archives', 'reference-axes', 'sweeps', 'resolved-features', 'circular-patterns'):
    sys.path.insert(0, str(ROOT / 'docs/features' / name))
from sheet_fixture_bytes import SheetFixtureOptions, sheet_object, allowance_body, allowance_tail
from fillet_fixture_bytes import WireMap
from regenerate_literals import literal_material_cases


def material_variants() -> list[bytes]:
    """Independently splice ten individual expected edits; e.g. material_variants()[0]."""
    original = literal_material_cases()['parent.bin']
    tag = 6 + len(b'moSMParamsFromMat_c')
    table = tag + 8
    records = table + 4 + len('material Ω'.encode('utf-16-le')) + 4
    positions = [(tag, '<i', 12345), (tag+4, '<i', -7),
        (records, '<i', 12345), (records+4, '<Q', 0x8000000000000001),
        (records+12, '<i', -7), (records+16, '<Q', 0x7ff0000000000001),
        (records+24, '<i', 12345), (records+28, '<B', 254), (records+29, '<Q', 0xfff8000000000055)]
    variants = [original]
    for offset, kind, value in positions:
        raw = bytearray(original)
        struct.pack_into(kind, raw, offset, value)
        variants.append(bytes(raw))
    text = 'replacement material Ω'.encode('utf-16-le')
    variants.insert(3, original[:table] + b'\xff\xfe\xff' + bytes([len(text)//2]) + text + original[records-4:])
    return variants


def sheet_with_material(kind: int, state: int, material: bytes) -> bytes:
    """Replace one structurally identified null material occurrence; e.g. sheet_with_material(0, 1, raw)."""
    options = SheetFixtureOptions(bool(state), False, 2)
    feature, _ = sheet_object(kind, 19000, options, 96)
    allowance = allowance_body(WireMap(), 19000, {}, options)
    assert feature.count(allowance) == 1, 'expected one complete reviewed allowance body'
    tail = allowance_tail(19000, False)
    offset = feature.index(allowance) + len(allowance) - len(tail) + 4
    assert feature[offset:offset+2] == bytes(2), 'expected null material after four empty CStrings'
    # Rich=false leaves no new/cached object references after the insertion.
    feature = feature[:offset] + material + feature[offset+2:]
    return struct.pack('<iH', -123, 1) + feature + (bytes(4) if state else b'')


def owned_material_packet() -> bytes:
    """Encode four containing contexts with original and ten edits each; e.g. owned_material_packet()[:4]."""
    records: list[bytes] = []
    for kind in range(2):
        for state in range(2):
            for edit, material in enumerate(material_variants()):
                raw = sheet_with_material(kind, state, material)
                records.append(bytes([kind, state, edit]) + struct.pack('<I', len(raw)) + raw)
    return b'BMR1' + struct.pack('<I', len(records)) + b''.join(records)


if __name__ == '__main__':
    (ROOT / 'tests/fixtures/sheet-material-ranges/owned-cases.bin').write_bytes(owned_material_packet())
