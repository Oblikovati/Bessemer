"""Freeze complete field/interval expectations for the shared conditional-item grammar."""
from __future__ import annotations
import gzip
import hashlib
import json
import struct
from pathlib import Path
import requests
import snapshot
from wire_packet import ROOT,SOURCE,blob,optional


def path_bytes(path: snapshot.Path) -> bytes:
    return blob(json.dumps(path,separators=(',',':')).encode())


def field_packet(value: snapshot.StoredField) -> bytes:
    raw=struct.pack('<'+value.encoding*len(value.values),*value.values)
    tag={'B':0,'i':1,'I':2,'Q':3}[value.encoding]
    return path_bytes(value.path)+struct.pack('<IBI',value.offset,tag,len(value.values))+blob(raw)


def case_packet(name: str, identifier: int, stored: bytes) -> bytes:
    decoded,value=requests.parse(identifier,stored);expected=snapshot.record(identifier,value)
    assert bytes(expected.raw)==decoded,name
    raw=blob(name.encode())+struct.pack('<IiI',identifier,value['version'],expected.wide_scale)
    raw+=optional(value['version'] if identifier==128 else None,'i')+blob(stored)+blob(decoded)
    raw+=struct.pack('<I',len(expected.fields))+b''.join(map(field_packet,expected.fields))
    raw+=struct.pack('<I',len(expected.scopes))+b''.join(path_bytes(path)+struct.pack('<II',start,end) for path,start,end in expected.scopes)
    raw+=struct.pack('<I',len(expected.counts))+struct.pack('<'+'I'*len(expected.counts),*expected.counts)
    raw+=struct.pack('<I',len(expected.projections))+b''.join(path_bytes(path)+bytes((flag,)) for path,flag in expected.projections)
    return raw+b'\1\0\0'


def freeze(output_root: Path = ROOT) -> None:
    rows=requests.cases();_,native=requests.native_cases()
    packets=[freeze_record(output_root,rows,identifier) for identifier in (126,315,128)]
    references=['flowrecord126.py','flowrecord315.py','flowrecord128.py','flowresultdefinitions.py','flowlegacytables.py']
    proof=dict(cases=len(rows),bytes=sum(packet['bytes'] for packet in packets),packets=packets,
               references={name:hashlib.sha256((SOURCE/name).read_bytes()).hexdigest() for name in references},
               native_records=native,identifiers=[126,315,128],item_versions=list(range(6)),block_element_bytes=[72,48],
               scope='Complete conditional-item framing and all raw values, fields, scope intervals, count offsets and native mode projections; raw126, whole-zlib315 and raw-version-plus-zlib128',
               native_runtime_reproduced=False,engineering_fields_assigned=False,native_application_checked=False)
    report=output_root/'docs/features/flow-conditional-items/baseline-verification.json'
    report.parent.mkdir(parents=True,exist_ok=True);report.write_text(json.dumps(proof,indent=2)+'\n',encoding='utf-8',newline='\n')


def freeze_record(output_root: Path, rows: list[tuple[str,int,bytes]], identifier: int) -> dict:
    selected=[row for row in rows if row[1]==identifier]
    payload=b'FCI1'+struct.pack('<I',len(selected))+b''.join(case_packet(*row) for row in selected)
    assert len(payload)<=16_000_000
    destination=output_root/f'tests/fixtures/flow-conditional-items/record{identifier}.bin.gz'
    destination.parent.mkdir(parents=True,exist_ok=True);destination.write_bytes(gzip.compress(payload,mtime=0))
    return dict(identifier=identifier,cases=len(selected),bytes=len(payload),sha256=hashlib.sha256(payload).hexdigest())


if __name__=='__main__':
    freeze()
