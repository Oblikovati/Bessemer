"""Independent ASCII lexical expectations for documented Archive.ini records (#231)."""
from dataclasses import dataclass
from enum import IntEnum


class IniKind(IntEnum):
    OTHER = 0
    SECTION = 1
    FIELD = 2


@dataclass(frozen=True)
class IniRecord:
    raw: bytes
    section: bytes | None
    kind: IniKind
    key: bytes | None
    value: bytes | None


def classify_line(raw: str, section: bytes | None) -> IniRecord:
    """Match lexical ordering without normalization; e.g. classify_line('K= V\r\n',None)."""
    body = raw.rstrip('\r\n')
    kind = IniKind.OTHER
    key: bytes | None = None
    value: bytes | None = None
    if body.startswith('[') and body.endswith(']'):
        section = body[1:-1].encode('ascii')
        kind = IniKind.SECTION
    elif '=' in body and not body.startswith((';', '#')):
        left, right = body.split('=', 1)
        key, value = left.encode('ascii'), right.encode('ascii')
        kind = IniKind.FIELD
    return IniRecord(raw.encode('ascii'), section, kind, key, value)


def ini_records(payload: bytes) -> tuple[IniRecord, ...] | None:
    """Retain Python's documented lexical view exactly; e.g. ini_records(b'[A]\r\r\nK=v\n')."""
    if not payload.isascii():
        return None
    section: bytes | None = None
    records: list[IniRecord] = []
    for raw in payload.decode('ascii').splitlines(keepends=True):
        record = classify_line(raw, section)
        section = record.section
        records.append(record)
    return tuple(records)
