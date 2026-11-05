"""Declared-wire sweep-twist literals and an operation-local primitive oracle (#211)."""
from pathlib import Path
from itertools import product
from typing import Callable, Protocol, cast
import hashlib
import json
import struct
from fixture_object_map import object_tag

REFERENCE: Path | None = None
CLASS = 'moTwistDataForBiDirectionalSweep_c'
VERSIONS = (0x27CC, 0x27D9, 0x27DA, 19000)
DOUBLE_PAIRS = ((0, 0x8000000000000000), (1, 0x8000000000000001),
                (0x7FF0000000000000, 0xFFF0000000000000),
                (0x7FF8000000001234, 0xFFF8000000005678),
                (0x7FF0000000000001, 0xFFF0000000000001),
                (0x3FF0000000000000, 0x400921FB54442D18))
TwistWireCase = tuple[int, int, int, int, bytes]


class PrimitiveArchive(Protocol):
    p: int
    def u8(self) -> int: ...
    def f64(self) -> float: ...
    def unk(self, start: int) -> None: ...


class TwistObjectArchive(PrimitiveArchive, Protocol):
    feature_context: str
    readers: dict[str, Callable[..., object]]
    map: list[object]
    def object(self) -> dict[str, object]: ...


def declared_twist_reader(archive: PrimitiveArchive, schema: int = 1) -> dict[str, int | float]:
    """Extend only a fixture archive with the documented primitives; e.g. declared_twist_reader(a)."""
    from modelcomponents import need_context
    from swarchive import ArchiveError
    need_context(archive)
    if schema != 1:
        raise ArchiveError(f'twist schema {schema}; expected schema1')
    start = archive.p
    result = dict(raw_08=archive.u8(), raw_10=archive.f64(), raw_18=archive.f64())
    archive.unk(start)
    return result


def twist_archive(raw: bytes, version: int, extended: bool) -> TwistObjectArchive:
    """Keep original Source code untouched and use a local table; e.g. twist_archive(raw,19000,True)."""
    from mfcobjects import ObjectArchive
    archive = cast(TwistObjectArchive, ObjectArchive(raw, version))
    archive.feature_context = 'ordinary_external_xml'
    archive.readers = dict(archive.readers)
    if extended:
        archive.readers[CLASS] = declared_twist_reader
    return archive


def twist_record(value: int, first: int, second: int, schema: int = 1) -> bytes:
    """Use a literal17-byte body without runtime-structure padding; e.g. twist_record(255,0,1)."""
    body = struct.pack('<BQQ', value, first, second)
    assert len(body) == 17
    return object_tag(CLASS, schema) + body


def twist_cases() -> list[TwistWireCase]:
    """Cross every saved byte and IEEE boundary pair; e.g. twist_cases()."""
    return [(version, value, pair[0], pair[1], twist_record(value, *pair))
            for version, value, pair in product(VERSIONS, range(256), DOUBLE_PAIRS)]


def read_twist(raw: bytes, version: int, extended: bool = True) -> tuple[dict[str, object], int]:
    """Require complete original framing plus the declared extension; e.g. read_twist(raw,19000)."""
    from swarchive import ArchiveError
    archive = twist_archive(raw, version, extended)
    result = archive.object()
    if archive.p != len(raw):
        raise ArchiveError(f'twist consumed {archive.p} of {len(raw)} bytes; expected exact standalone EOF')
    return result, len(archive.map)


def write_twist_fixtures(destination: Path) -> None:
    """Emit deterministic declared-wire records only; e.g. write_twist_fixtures(output)."""
    rows = twist_cases()
    packet = bytearray(b'BWT1' + struct.pack('<I', len(rows)))
    for version, value, first, second, raw in rows:
        packet.extend(struct.pack('<IBQQI', version, value, first, second, len(raw)) + raw)
    destination.mkdir(parents=True, exist_ok=True)
    (destination / 'bare-cases.bin').write_bytes(packet)
    report = dict(cases=len(rows), body_bytes=17, versions=VERSIONS, values=256,
                  double_pairs=6, packet_sha256=hashlib.sha256(packet).hexdigest(),
                  oracle='Original MFC framing/primitives with a fixture-only declared-wire reader; not original production class parsing')
    (destination / 'manifest.json').write_text(json.dumps(report, indent=2) + '\n', encoding='utf-8', newline='\n')
