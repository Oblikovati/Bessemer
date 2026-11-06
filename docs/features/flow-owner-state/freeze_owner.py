"""Freeze raw fields, gated native updates and selection success/empty/failure outcomes."""
import gzip
import json
import struct
from pathlib import Path
import owner_requests as requests
from swarchive import ArchiveError


def blob(raw: bytes) -> bytes:
    return struct.pack('<I',len(raw))+raw


def optional(value: int | None) -> bytes:
    return b'\0' if value is None else b'\1'+struct.pack('<i',value)


def updates_packet(value: dict) -> bytes:
    return b''.join(optional(value['updates'].get(hex(offset))) for offset in requests.OFFSETS)


def case_packet(name: str, identifier: int, native: bool, raw: bytes) -> bytes:
    value=requests.parse(identifier,raw)
    result=blob(name.encode())+optional(identifier if native else None)+struct.pack('<I',identifier)+blob(raw)
    result+=optional(value.get('version'))+bytes((value.get('native_update_gate',True),))
    result+=blob(requests.words(value['words'] if identifier==122 else value['values']))
    result+=optional(value.get('state_byte'))+updates_packet(value)+struct.pack('<I',value['end'])
    return result


def selection_packet(name: str, raw: bytes) -> bytes:
    document=requests.flowresults.read(raw);result=blob(name.encode())+blob(raw)
    try:value=requests.flowauxrecords.select_122_or_434(document)
    except ArchiveError:return result+b'\2'
    if value is None:return result+b'\0'
    identifier=value['identifier'];selected=requests.flowresults.latest(document,identifier)
    ordinal=document['records'].index(selected)
    return result+b'\1'+struct.pack('<II',identifier,ordinal)+updates_packet(value['projection'])


def freeze(root: Path=requests.ROOT) -> None:
    rows=requests.cases();selections=requests.selection_cases();_,native=requests.native_cases()
    packet=b'FOS1'+struct.pack('<I',len(rows))+b''.join(case_packet(*row) for row in rows)
    packet+=struct.pack('<I',len(selections))+b''.join(selection_packet(*row) for row in selections)
    assert len(packet)<=16_000_000
    path=root/'tests/fixtures/flow-owner-state/records.bin.gz'
    path.parent.mkdir(parents=True,exist_ok=True);path.write_bytes(gzip.compress(packet,mtime=0))
    references=('flowauxrecords.py','flowlegacytables.py','flowresults.py')
    proof=dict(cases=len(rows),selection_cases=len(selections),bytes=len(packet),sha256=requests.counted.digest(packet),native=native,
               selections=[dict(name=name,bytes=len(raw),sha256=requests.counted.digest(raw)) for name,raw in selections],
               manifest_sha256=requests.counted.digest((requests.SOURCE/'research/flow-aux-records-corpus.json').read_bytes()),
               references={name:requests.counted.digest((requests.SOURCE/name).read_bytes().replace(b'\r\n',b'\n')) for name in references},
               scope='Complete raw 122/434 fields, exact update gate/projection and 434-before-122 latest-occurrence selection with no parse-error fallback',
               native_runtime_updates_executed=False,engineering_fields_assigned=False,native_application_checked=False)
    path=root/'docs/features/flow-owner-state/baseline-verification.json'
    path.parent.mkdir(parents=True,exist_ok=True);path.write_text(json.dumps(proof,indent=2)+'\n',encoding='utf-8',newline='\n')


if __name__=='__main__':
    freeze()
