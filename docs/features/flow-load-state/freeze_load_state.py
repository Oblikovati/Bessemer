"""Freeze complete record11 load-state fields, Unicode units and native-mode boundaries."""
from __future__ import annotations
import gzip
import hashlib
import json
import struct
from pathlib import Path
import load_requests
import load_snapshot
from freeze_items import field_packet,path_bytes
from wire_packet import ROOT,SOURCE,blob,optional


def text_packet(value: load_snapshot.TextSpan) -> bytes:
    return path_bytes(value.path)+struct.pack('<IIII',value.start,value.end,value.units,value.prefix_units)+blob(value.prefix_raw)


def case_packet(name: str, mode: bool, origin: int, raw: bytes) -> bytes:
    value=load_requests.schema.read(raw,load_mode=mode);expected,texts=load_snapshot.record(value)
    assert bytes(expected.raw)==raw,name
    packet=blob(name.encode())+bytes((mode,origin))+blob(raw)
    packet+=struct.pack('<iIiII',value['version'],value['count'],value['member_130'],value['array_width'],value['end'])
    packet+=struct.pack('<I',len(expected.fields))+b''.join(map(field_packet,expected.fields))
    packet+=struct.pack('<I',len(expected.scopes))+b''.join(path_bytes(path)+struct.pack('<II',start,end) for path,start,end in expected.scopes)
    packet+=struct.pack('<I',len(expected.counts))+struct.pack('<'+'I'*len(expected.counts),*expected.counts)
    packet+=struct.pack('<I',len(texts))+b''.join(map(text_packet,texts))
    tail=next((start for path,start,_ in expected.scopes if path==('mode_tail',)),None)
    return packet+optional(tail,'I')+b'\1\0\0'


def freeze(output_root: Path = ROOT) -> None:
    rows=load_requests.cases();_,native=load_requests.native_cases()
    payload=b'FL11'+struct.pack('<I',len(rows))+b''.join(case_packet(*row) for row in rows)
    assert len(payload)<=16_000_000
    destination=output_root/'tests/fixtures/flow-load-state/state.bin.gz'
    destination.parent.mkdir(parents=True,exist_ok=True);destination.write_bytes(gzip.compress(payload,mtime=0))
    proof=dict(cases=len(rows),bytes=len(payload),sha256=hashlib.sha256(payload).hexdigest(),
               references={name:hashlib.sha256((SOURCE/name).read_bytes()).hexdigest() for name in ('flowrecord11.py','flowlegacytables.py')},
               native=native,member_order=list(load_snapshot.MEMBER_ORDER),load_modes=[False,True],
               scope='Complete record11 version18 raw framing under explicit load modes, all scalar/array/string values and intervals, all65536 UTF16 units and exact first-NUL prefix units; original native false-mode payload and separately labelled derived true-mode prefix',
               native_owner_mode_verified=False,runtime_transformations_reproduced=False,engineering_fields_assigned=False,native_application_checked=False)
    report=output_root/'docs/features/flow-load-state/baseline-verification.json'
    report.parent.mkdir(parents=True,exist_ok=True);report.write_text(json.dumps(proof,indent=2)+'\n',encoding='utf-8',newline='\n')


if __name__=='__main__':
    freeze()
