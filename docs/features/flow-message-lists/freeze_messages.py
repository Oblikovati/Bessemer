"""Freeze complete message-list framing without inventing legacy text interpretation."""
from __future__ import annotations
import gzip
import hashlib
import json
import struct
from pathlib import Path
import message_requests
import message_snapshot
from freeze_items import field_packet,path_bytes
from wire_packet import ROOT,SOURCE,blob,optional


def text_packet(value: message_snapshot.MessageText) -> bytes:
    packet=path_bytes(value.path)+struct.pack('<III',value.start,value.end,value.count)+bytes((value.legacy,value.prefix is not None))
    return packet+(blob(value.prefix) if value.prefix is not None else b'')


def case_packet(name: str, mode: int | None, native: bool, raw: bytes) -> bytes:
    value=message_requests.parse(raw,mode);expected,texts=message_snapshot.record(value)
    assert bytes(expected.raw)==raw,name
    packet=blob(name.encode())+struct.pack('<i',value['version'])+optional(mode,'i')+bytes((native,))+blob(raw)+struct.pack('<I',value['count'])
    packet+=struct.pack('<I',len(expected.fields))+b''.join(map(field_packet,expected.fields))
    packet+=struct.pack('<I',len(expected.scopes))+b''.join(path_bytes(path)+struct.pack('<II',start,end) for path,start,end in expected.scopes)
    packet+=struct.pack('<I',len(expected.counts))+struct.pack('<'+'I'*len(expected.counts),*expected.counts)
    packet+=struct.pack('<I',len(texts))+b''.join(map(text_packet,texts))
    return packet+b'\1\0\0'


def freeze(output_root: Path = ROOT) -> None:
    rows=message_requests.cases();_,native=message_requests.native_cases()
    payload=b'FM31'+struct.pack('<I',len(rows))+b''.join(case_packet(*row) for row in rows)
    assert len(payload)<=16_000_000
    destination=output_root/'tests/fixtures/flow-message-lists/messages.bin.gz'
    destination.parent.mkdir(parents=True,exist_ok=True);destination.write_bytes(gzip.compress(payload,mtime=0))
    references=('flowrecord301.py','flowmetadataprefix.py','flowlegacytables.py')
    proof=dict(cases=len(rows),bytes=len(payload),sha256=hashlib.sha256(payload).hexdigest(),native=native,
               references={name:hashlib.sha256((SOURCE/name).read_bytes()).hexdigest() for name in references},
               scope='Complete versions0..3, raw member words, primary and independent counted string lists, all fields/scopes/count offsets, modern fullUTF16 and exact first-NUL prefix bytes, legacy exactlyN raw bytes without text projection',
               versions=[0,1,2,3],modern_mode_assumption=0,native_mode_provenance_verified=False,legacy_string_interpretation_reproduced=False,engineering_fields_assigned=False,native_application_checked=False)
    report=output_root/'docs/features/flow-message-lists/baseline-verification.json'
    report.parent.mkdir(parents=True,exist_ok=True);report.write_text(json.dumps(proof,indent=2)+'\n',encoding='utf-8',newline='\n')


if __name__=='__main__':
    freeze()
