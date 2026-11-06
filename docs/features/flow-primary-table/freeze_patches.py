"""Freeze independently rebuilt fixed-shape primary-table edits."""
from __future__ import annotations
import gzip
import hashlib
import json
import struct
from pathlib import Path
import patch_oracle as oracle
from wire_packet import ROOT,SOURCE,blob


def entry(request: tuple) -> bytes:
    name,field,before,after,raw,expected=request
    oracle.validate_patch(field,before,after,raw,expected)
    path=json.dumps(field.path,separators=(',',':')).encode()
    return blob(name.encode())+blob(path)+struct.pack('<IB',field.offset,field.width)+blob(raw)+blob(expected)


def freeze(output_root: Path = ROOT) -> None:
    requests=[request for name,raw in oracle.requests() for request in oracle.patch_cases(name,raw)]
    payload=b'F4EP'+struct.pack('<I',len(requests))+b''.join(map(entry,requests))
    destination=output_root/'tests/fixtures/flow-primary-table/patches.bin.gz'
    destination.parent.mkdir(parents=True,exist_ok=True)
    destination.write_bytes(gzip.compress(payload,mtime=0))
    proof=dict(cases=len(requests),bytes=len(payload),sha256=hashlib.sha256(payload).hexdigest(),
               reference_sha256=hashlib.sha256((SOURCE/'flowrecord439.py').read_bytes()).hexdigest(),
               scope='Independent Source-tree mutation and complete serialization, exact changed field and unchanged remainder; versions0..3, all four typed layouts, both groups, absent bodies, stored flags and unused tags',
               excludes=['Structural resizing','Populated native editing','Bessemer typed API acceptance'])
    report=output_root/'docs/features/flow-primary-table/patch-verification.json'
    report.parent.mkdir(parents=True,exist_ok=True)
    report.write_text(json.dumps(proof,indent=2)+'\n',encoding='utf-8',newline='\n')


if __name__=='__main__':
    freeze()
