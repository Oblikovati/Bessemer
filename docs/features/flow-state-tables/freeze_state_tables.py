"""Freeze complete raw state tables without assigning native owner or engineering meaning."""
from __future__ import annotations
import gzip
import hashlib
import json
import struct
from pathlib import Path
import state_requests
import state_snapshot
from freeze_items import field_packet,path_bytes
from wire_packet import ROOT,SOURCE,blob,optional


def case_packet(name: str, identifier: int, context: int | None, raw: bytes) -> bytes:
    value=state_requests.parse(identifier,raw,context);expected,quotients=state_snapshot.record(identifier,value)
    assert bytes(expected.raw)==raw,name
    packet=blob(name.encode())+struct.pack('<Ii',identifier,value['version'])+optional(context,'i')+blob(raw)
    packet+=struct.pack('<I',len(expected.fields))+b''.join(map(field_packet,expected.fields))
    packet+=struct.pack('<I',len(expected.scopes))+b''.join(path_bytes(path)+struct.pack('<II',start,end) for path,start,end in expected.scopes)
    packet+=struct.pack('<I',len(expected.counts))+struct.pack('<'+'I'*len(expected.counts),*expected.counts)
    packet+=struct.pack('<I',len(expected.projections))+b''.join(path_bytes(path)+bytes((flag,)) for path,flag in expected.projections)
    packet+=struct.pack('<I',len(quotients))+b''.join(path_bytes(path)+optional(quotient,'i') for path,quotient in quotients)
    return packet+b'\1\0\0'


def freeze_record(output_root: Path, rows: list[tuple[str,int,int | None,bytes]], identifier: int) -> dict:
    selected=[row for row in rows if row[1]==identifier]
    payload=b'FST2'+struct.pack('<I',len(selected))+b''.join(case_packet(*row) for row in selected)
    assert len(payload)<=16_000_000
    destination=output_root/f'tests/fixtures/flow-state-tables/record{identifier}.bin.gz'
    destination.parent.mkdir(parents=True,exist_ok=True);destination.write_bytes(gzip.compress(payload,mtime=0))
    return dict(identifier=identifier,cases=len(selected),bytes=len(payload),sha256=hashlib.sha256(payload).hexdigest())


def freeze(output_root: Path = ROOT) -> None:
    rows=state_requests.cases();_,native=state_requests.native_cases()
    packets=[freeze_record(output_root,rows,identifier) for identifier in state_requests.SCHEMAS]
    references=[f'flowrecord{identifier}.py' for identifier in state_requests.SCHEMAS]+['flowlegacytables.py']
    proof=dict(cases=len(rows),bytes=sum(packet['bytes'] for packet in packets),packets=packets,native=native,
               references={name:hashlib.sha256((SOURCE/name).read_bytes()).hexdigest() for name in references},
               scope='Complete raw129 tagged rows,138 explicit-count state,139 fixed settings rows,417 independently counted word/wide groups and432 stored/recomputed-version flags; all fields, scopes, count offsets and derived quotients',
               native_context_verified=False,native_runtime_reproduced=False,engineering_fields_assigned=False,native_application_checked=False)
    report=output_root/'docs/features/flow-state-tables/baseline-verification.json'
    report.parent.mkdir(parents=True,exist_ok=True);report.write_text(json.dumps(proof,indent=2)+'\n',encoding='utf-8',newline='\n')


if __name__=='__main__':
    freeze()
