"""Frozen explicit native timestamp requests and owned container metadata."""
from __future__ import annotations
import gzip
import hashlib
import json
from pathlib import Path
import struct
import sys
from collections.abc import Iterator
ROOT = Path(__file__).resolve().parents[3]
SOURCE = ROOT.parent / 'inventor-interface/solidworks'
sys.path.insert(0, str(SOURCE))
import ziptimestamps
from swcontainer import Document
Calendar = tuple[int, int, int, int, int, int]
Request = tuple[Calendar | None, int]
FIXTURES = ('blank-part.SLDPRT', 'blank-assembly.SLDASM', 'blank-drawing.SLDDRW')


def digest(raw: bytes) -> str:
    return hashlib.sha256(raw).hexdigest()


def patterns() -> tuple[int, ...]:
    values = {0, 1, 2, 31, 32, 63, 64, 79, 80, 81, 125, 126, 1979, 1980, 1981, 2026, 0xffffffff}
    values.update(1 << bit for bit in range(32))
    values.update(0xffffffff ^ (1 << bit) for bit in range(32))
    return tuple(sorted(values))


def varied(base: Calendar, axis: int, value: int) -> Calendar:
    fields = list(base)
    fields[axis] = value
    return (fields[0], fields[1], fields[2], fields[3], fields[4], fields[5])


def request_candidates() -> Iterator[Request]:
    base: Calendar = (20, 40, 13, 29, 9, 2026)
    yield None, 0
    for value in patterns():
        yield None, value
        yield (0xffffffff,) * 6, value
    for axis in range(6):
        for value in patterns():
            yield varied(base, axis, value), 0
    for year in range(4097):
        yield varied(base, 5, year), 0
    yield from pair_requests(base)


def pair_requests(base: Calendar) -> Iterator[Request]:
    for first, second in ((0, 1), (1, 2), (4, 5)):
        yield from paired_values(base, first, second)


def paired_values(base: Calendar, first: int, second: int) -> Iterator[Request]:
    for left in patterns():
        for right in patterns():
            yield varied(varied(base, first, left), second, right), 0


def requests() -> list[Request]:
    return list(dict.fromkeys(request_candidates()))


def expected(calendar: Calendar | None, override: int) -> int:
    return ziptimestamps.native_timestamp(calendar, packed_date=override)


def oracle(calendar: Calendar | None, override: int) -> int:
    if override or calendar is None:
        return override
    second, minute, hour, day, month, year = calendar
    adjusted = year - 1980 if year > 1980 else year - 80 if year > 80 else year
    date = ((adjusted << 25) + ((month + 1) << 21) + (day << 16)) & 0xffffffff
    time = ((hour << 11) + (minute << 5) + (second >> 1)) & 0xffffffff
    return date | time


def request_packet(request: Request) -> bytes:
    calendar, override = request
    prefix = bytes([calendar is not None])
    if calendar is not None:
        prefix += struct.pack('<6I', *calendar)
    return prefix + struct.pack('<II', override, expected(calendar, override))


def native_rows() -> list[tuple[int, str, int, int, int]]:
    rows = []
    for ordinal, filename in enumerate(FIXTURES):
        raw = (ROOT / 'tests/fixtures' / filename).read_bytes()
        document = Document(raw)
        for entry in document.entries:
            local, central = entry.local_at + 10, entry.central[0] + 12
            first, second = (struct.unpack_from('<I', raw, offset)[0] for offset in (local, central))
            assert first == second, (filename, entry.name, first, second)
            rows.append((ordinal, entry.name, local, central, first))
    return rows


def native_packet(row: tuple[int, str, int, int, int]) -> bytes:
    ordinal, name, local, central, packed = row
    text = name.encode('utf-8')
    return struct.pack('<BI', ordinal, len(text)) + text + struct.pack('<QQI', local, central, packed)


def write_packet(root: Path, name: str, magic: bytes, rows: list[bytes]) -> dict[str, object]:
    raw = magic + struct.pack('<I', len(rows)) + b''.join(rows)
    assert len(raw) <= 16_000_000, (name, len(raw))
    path = root / 'tests/fixtures/container-timestamps' / (name + '.bin.gz')
    path.parent.mkdir(parents=True, exist_ok=True)
    path.write_bytes(gzip.compress(raw, mtime=0))
    return dict(cases=len(rows), bytes=len(raw), sha256=digest(raw))


def freeze(root: Path = ROOT) -> dict[str, object]:
    assert Path(ziptimestamps.__file__).resolve() == SOURCE / 'ziptimestamps.py'
    cases = requests()
    assert all(expected(*request) == oracle(*request) for request in cases)
    packets = {'requests': write_packet(root, 'requests', b'ZTR1', [request_packet(row) for row in cases])}
    packets['native'] = write_packet(root, 'native', b'ZTN1', [native_packet(row) for row in native_rows()])
    proof = dict(issue=250, packets=packets, source_raw_sha256={name: digest((SOURCE / name).read_bytes()) for name in ('ziptimestamps.py', 'test_ziptimestamps.py', 'swcontainer.py')})
    proof['native_raw_sha256'] = {name: digest((ROOT / 'tests/fixtures' / name).read_bytes()) for name in FIXTURES}
    proof['scope'] = 'Frozen reference arithmetic and owned raw header intervals only; no new runtime API or native writer execution'
    output = root / 'docs/features/container-timestamps/baseline.json'
    output.parent.mkdir(parents=True, exist_ok=True)
    output.write_text(json.dumps(proof, indent=2) + '\n', encoding='utf-8')
    return proof


if __name__ == '__main__':
    freeze()
