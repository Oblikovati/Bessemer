"""Freeze all primary-class versions and typed element layouts for Flow record439."""
from __future__ import annotations
import gzip
import hashlib
import json
import struct
from pathlib import Path
import cases
import packet
from wire_packet import ROOT,SOURCE,blob


def case_packet(name: str, raw: bytes) -> bytes:
    value=cases.schema.read(raw);rebuilt,expected=packet.record(value)
    assert rebuilt==raw
    return blob(name.encode())+blob(raw)+expected


def freeze(output_root: Path = ROOT) -> None:
    rows=cases.cases();native,provenance=cases.native_rows()
    payload=b'F439'+struct.pack('<I',len(rows))+b''.join(case_packet(*row) for row in rows)
    destination=output_root/'tests/fixtures/flow-primary-table/primary.bin.gz'
    destination.parent.mkdir(parents=True,exist_ok=True)
    destination.write_bytes(gzip.compress(payload,mtime=0))
    proof=dict(cases=len(rows),bytes=len(payload),sha256=hashlib.sha256(payload).hexdigest(),
               reference_sha256=hashlib.sha256((SOURCE/'flowrecord439.py').read_bytes()).hexdigest(),
               native_records=provenance,native_empty_only=all(cases.schema.read(raw)['count']==0 for _,raw in native),
               versions=list(range(4)),typed_widths=[4,8,12,16],
               scope='Complete record439 primary-class version0..3 framing: two groups per item, all raw kind/mode/presence/flag bytes, optional bodies, all four ragged typed layouts and16-word wide blocks with independent nested intervals',
               engineering_fields_assigned=False,native_runtime_reproduced=False,native_application_checked=False)
    report=output_root/'docs/features/flow-primary-table/baseline-verification.json'
    report.parent.mkdir(parents=True,exist_ok=True)
    report.write_text(json.dumps(proof,indent=2)+'\n',encoding='utf-8',newline='\n')


if __name__=='__main__':
    freeze()
