"""Complete authenticated Electrical templates and owned ZIP boundary cases (#231)."""
from dataclasses import dataclass
import hashlib
import io
import json
from pathlib import Path
import zipfile
import zlib

OWNED_EXTRA = bytes.fromhex('feca030078797a')
OWNED_MEMBER_COMMENT = b'member\0comment'


@dataclass(frozen=True)
class ExpectedEntry:
    name: str
    flags: int
    method: int
    crc: int
    payload: bytes
    extra: bytes
    comment: bytes


@dataclass(frozen=True)
class ElectricalCase:
    name: str
    raw: bytes
    entries: tuple[ExpectedEntry, ...] | None = None
    comment: bytes | None = None


def native_cases(fixtures: Path, manifest: Path) -> tuple[ElectricalCase, ...]:
    """Authenticate retained templates under injected roots; e.g. native_cases(fixtures,manifest)."""
    report = json.loads(manifest.read_text())
    cases: list[ElectricalCase] = []
    for row in report['archives']:
        raw = (fixtures / row['file']).read_bytes()
        assert len(raw) == row['bytes']
        assert hashlib.sha256(raw).hexdigest() == row['sha256']
        cases.append(ElectricalCase(row['file'], raw))
    assert len(cases) == 4
    return tuple(cases)


def owned_archive(entries: tuple[tuple[str, bytes], ...], method: int, comment: bytes) -> bytes:
    """Use a separate ZIP implementation with deterministic metadata; e.g. owned_archive(entries,0,b'')."""
    output = io.BytesIO()
    with zipfile.ZipFile(output, 'w') as archive:
        archive.comment = comment
        for name, payload in entries:
            info = zipfile.ZipInfo('fixture', (2026, 1, 2, 3, 4, 6))
            info.filename = name
            info.orig_filename = name
            info.compress_type = method
            info.extra = OWNED_EXTRA
            info.comment = OWNED_MEMBER_COMMENT
            info.create_system = 3
            info.external_attr = 0o100600 << 16
            archive.writestr(info, payload)
    return output.getvalue()


def duplicate_members() -> tuple[tuple[str, bytes], ...]:
    """Exercise duplicate INI occurrences and opaque path-like names; e.g. duplicate_members()."""
    first = b'[Section]\r\r\nKey= first=second \r\r\nKey=other\nkey=lower\n;comment\n'
    second = b'=value\r#k=x\n leading = left \n[]\nK=\0end'
    return (('Archive.ini', first), ('Archive.ini', second),
            ('../outside.bin', b'\0\xffraw'), ('C:\\folder\\opaque.bin', b'opaque'))


def owned_cases() -> tuple[ElectricalCase, ...]:
    """Cover empty, duplicate, control-line and non-ASCII cases; e.g. owned_cases()."""
    controls = b'[Controls]\vK=A\fK=B\x1cK=C\x1dK=D\x1eK=E\r\n'
    comment = b'beforePK\x05\x06' + bytes(24) + b'after'
    return (owned_case('owned-empty', (), 0, b'empty'),
            owned_case('owned-duplicates', duplicate_members(), 0, b'duplicates'),
            owned_case('owned-controls', (('Archive.ini', controls),), 8, comment),
            owned_case('owned-non-ascii', (('Archive.ini', b'\xff\xfe'),
                           ('资料/未知.bin', bytes(range(256)))), 8, b'raw'))


def owned_case(name: str, entries: tuple[tuple[str, bytes], ...], method: int, comment: bytes) -> ElectricalCase:
    """Keep construction inputs independent of ZIP readback; e.g. owned_case(name,entries,0,b'')."""
    expected = tuple(ExpectedEntry(name, 0 if name.isascii() else 0x800, method,
                     zlib.crc32(payload), payload, OWNED_EXTRA, OWNED_MEMBER_COMMENT)
                     for name, payload in entries)
    return ElectricalCase(name, owned_archive(entries, method, comment), expected, comment)


def expected_entries(case: ElectricalCase) -> tuple[tuple[ExpectedEntry, ...], bytes]:
    """Use authenticated native readback or independent owned inputs; e.g. expected_entries(case)."""
    if case.entries is not None:
        assert case.comment is not None
        return case.entries, case.comment
    with zipfile.ZipFile(io.BytesIO(case.raw)) as archive:
        entries = tuple(ExpectedEntry(info.filename, info.flag_bits, info.compress_type,
                        info.CRC, archive.read(info), info.extra, info.comment)
                        for info in archive.infolist())
        return entries, archive.comment
