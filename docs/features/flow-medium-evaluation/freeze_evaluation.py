"""Freeze documented arithmetic separately from exact-real interpolation accuracy."""
from __future__ import annotations
import gzip
import hashlib
import json
import struct
from pathlib import Path
import cases
import oracle
from wire_packet import ROOT,SOURCE,blob,optional
import flowmediumevaluate as schema
from swarchive import ArchiveError


def outcome(raw: bytes, ordinal: int, query_pattern: int) -> bytes:
    parsed=cases.objects.schema.read_kind1_body(raw,string_mode=0)
    query=oracle.number(query_pattern)
    try:
        value=schema.evaluate_kind1_curve(parsed,ordinal,query)
    except ArchiveError:
        return b'\0'
    curve=parsed['curves'][ordinal];branch,indices=oracle.branch(curve,query)
    exact=oracle.exact_value(curve,query,indices)
    interval=b'\0' if indices is None else b'\1'+struct.pack('<II',*indices)
    return b'\1'+struct.pack('<QBB',oracle.bits(value),oracle.classification(value),branch)+interval+struct.pack('<Q',oracle.bits(float(exact)))+optional(oracle.error_bound(value,exact),'Q')


def case_packet(name: str, raw: bytes, ordinal: int, query_pattern: int) -> bytes:
    return cases.objects.case_packet(name,raw)+struct.pack('<iQ',ordinal,query_pattern)+outcome(raw,ordinal,query_pattern)


def freeze(output_root: Path = ROOT) -> None:
    rows=cases.cases()
    packet=b'FEV1'+struct.pack('<I',len(rows))+b''.join(case_packet(*row) for row in rows)
    destination=output_root/'tests/fixtures/flow-medium-evaluation/curves.bin.gz'
    destination.parent.mkdir(parents=True,exist_ok=True)
    destination.write_bytes(gzip.compress(packet,mtime=0))
    statuses=[outcome(raw,ordinal,query) for _,raw,ordinal,query in rows]
    proof=dict(cases=len(rows),bytes=len(packet),sha256=hashlib.sha256(packet).hexdigest(),
               accepted=sum(row[0] for row in statuses),rejected=sum(not row[0] for row in statuses),
               nonfinite_arithmetic=sum(row[0] and row[9]!=0 for row in statuses),
               reference_sha256=hashlib.sha256((SOURCE/'flowmediumevaluate.py').read_bytes()).hexdigest(),
               body_reference_sha256=hashlib.sha256((SOURCE/'flowmediumobjects.py').read_bytes()).hexdigest(),
               scope='Documented kind-one accessors1/2/4, full body fields, finite input guards, endpoint clamps, exclusive-upper-bound search and binary64 operation order; exact-real barycentric interpolation is an independent accuracy oracle',
               nan_comparison='Classification only; arithmetic NaN payload/sign is not portable',native_application_validated=False)
    report=output_root/'docs/features/flow-medium-evaluation/baseline-verification.json'
    report.parent.mkdir(parents=True,exist_ok=True)
    report.write_text(json.dumps(proof,indent=2)+'\n',encoding='utf-8',newline='\n')


if __name__=='__main__':
    freeze()
