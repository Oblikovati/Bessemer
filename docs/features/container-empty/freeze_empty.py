"""Independent empty-container wires and the supplied registered signature corpus."""
from __future__ import annotations
import hashlib
import json
from pathlib import Path
import struct
import sys
from collections.abc import Iterator
from typing import TypedDict
ROOT = Path(__file__).resolve().parents[3]
SOURCE = ROOT.parent / 'inventor-interface/solidworks'
sys.path.insert(0, str(SOURCE))
import swcontainer
FIXTURES = ('blank-part.SLDPRT', 'blank-assembly.SLDASM', 'blank-drawing.SLDDRW')


class SignatureValues(TypedDict):
    local: str
    central: str
    end: str
    b1: int
    b2: int
    b3: int


def hex_word(value: object) -> str:
    if not isinstance(value, str) or len(value) != 8:
        raise ValueError(f'signature word {value!r}; expected exactly eight hexadecimal characters')
    try:
        raw = bytes.fromhex(value)
    except ValueError as error:
        raise ValueError(f'signature word {value!r}; expected exactly eight hexadecimal characters') from error
    if len(raw) != 4:
        raise ValueError(f'signature word {value!r}; expected exactly four decoded bytes')
    return raw.hex()


def filler_byte(value: object) -> int:
    if type(value) is not int or not 0 <= value <= 255:
        raise ValueError(f'filler byte {value!r}; expected an unsigned integer in 0..=255')
    return value


def signature_values(value: object) -> SignatureValues:
    if not isinstance(value, dict):
        raise ValueError(f'signatures {value!r}; expected an object with local, central and end hexadecimal words')
    return dict(local=hex_word(value.get('local')), central=hex_word(value.get('central')), end=hex_word(value.get('end')), b1=filler_byte(value.get('b1')), b2=filler_byte(value.get('b2')), b3=filler_byte(value.get('b3')))


def digest(raw: bytes) -> str:
    return hashlib.sha256(raw).hexdigest()


def table() -> dict[str, SignatureValues]:
    raw: object = json.loads((SOURCE / 'cache/keytable.json').read_text(encoding='utf-8'))
    if not isinstance(raw, dict) or len(raw) != 1000:
        raise ValueError(f'key table type {type(raw).__name__}; expected an object containing exactly 1000 entries')
    result = {hex_word(key): signature_values(value) for key, value in raw.items()}
    if len(result) != 1000:
        raise ValueError(f'normalized keys {len(result)}; expected 1000 distinct four-byte keys')
    return result


def signature_row(key: str, value: SignatureValues) -> bytes:
    result = bytes.fromhex(key) + b''.join(bytes.fromhex(value[name]) for name in ('local', 'central', 'end'))
    assert len(result) == 16, (key, len(result))
    return result


def empty_wire(key: str, value: SignatureValues) -> bytes:
    return bytes.fromhex(key) + struct.pack('>I', 4) + bytes.fromhex(value['end']) + struct.pack('<HHHHIIH', 0, 0, 0, 0, 0, 0, 0)


def accepted_rows() -> list[bytes]:
    signatures = table()
    rows = []
    for key, value in sorted(signatures.items()):
        wire = empty_wire(key, value)
        document = swcontainer.Document(wire, table=signatures)
        assert not document.entries and document.cd_at == 8 and document.cd_size == 0
        rows.append(signature_row(key, value) + wire)
    return rows


def filler_wire(key: str, value: SignatureValues, before: int, after: int, at_end: bool) -> bytes:
    signatures = bytes.fromhex(value['end'])
    byte = bytes([next(word for word in range(256) if word not in signatures)])
    start = before if at_end else 0
    header = bytes.fromhex(key) + struct.pack('>I', 4)
    end = signatures + struct.pack('<HHHHIIH', 0, 0, 0, 0, before - start, start, 0)
    return header + byte * before + end + byte * after


def trailing_variants(key: str, value: SignatureValues, before: int) -> Iterator[bytes]:
    for after in (0, 1, 255):
        for at_end in (False, True):
            yield filler_wire(key, value, before, after, at_end)


def filler_wires(key: str, value: SignatureValues) -> Iterator[bytes]:
    for before in (0, 1, 255):
        yield from trailing_variants(key, value, before)


def native_rows() -> list[bytes]:
    signatures = table()
    result = []
    for index, name in enumerate(FIXTURES):
        raw = (ROOT / 'tests/fixtures' / name).read_bytes()
        document = swcontainer.Document(raw, table=signatures)
        key = raw[:4].hex()
        offsets = (document.entries[0].local_at, document.cd_at, document.end_at)
        result.append(bytes([index]) + signature_row(key, signatures[key]) + struct.pack('<QQQ', *offsets))
    return result


def reference_builder_failure() -> str:
    signatures = table()
    key = next(iter(signatures))
    try:
        swcontainer.build([], key, signatures, filler=False)
    except UnboundLocalError as error:
        return str(error)
    raise AssertionError('Expected the separately tracked reference empty-builder failure')


def write_packet(root: Path, filename: str, magic: bytes, rows: list[bytes]) -> dict[str, object]:
    raw = magic + struct.pack('<I', len(rows)) + b''.join(rows)
    path = root / 'tests/fixtures/container-empty' / filename
    path.parent.mkdir(parents=True, exist_ok=True)
    path.write_bytes(raw)
    return dict(cases=len(rows), bytes=len(raw), sha256=digest(raw))


def freeze(root: Path = ROOT) -> dict[str, object]:
    rows = accepted_rows()
    packets = {'empty': write_packet(root, 'empty.bin', b'CES1', rows)}
    packets['native'] = write_packet(root, 'native.bin', b'CEN1', native_rows())
    proof = dict(bug=251, reference_bug=252, packets=packets, reference_builder_failure=reference_builder_failure())
    proof['source_raw_sha256'] = {name: digest((SOURCE / name).read_bytes()) for name in ('swcontainer.py', 'cache/keytable.json')}
    proof['native_raw_sha256'] = {name: digest((ROOT / 'tests/fixtures' / name).read_bytes()) for name in FIXTURES}
    proof['scope'] = 'Independent Source-positive empty-container baseline; no reader fix or native application execution'
    output = root / 'docs/features/container-empty/baseline.json'
    output.parent.mkdir(parents=True, exist_ok=True)
    output.write_text(json.dumps(proof, indent=2) + '\n', encoding='utf-8')
    return proof


if __name__ == '__main__':
    freeze()
