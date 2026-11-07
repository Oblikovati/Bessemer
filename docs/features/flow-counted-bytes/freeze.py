"""Freeze exact record-12 schema results and separately audited native provenance."""
import gzip
import json
import struct
from pathlib import Path
import counted_requests as requests


def blob(raw: bytes) -> bytes:
    return struct.pack('<I',len(raw))+raw


def case_packet(name: str, native: bool, raw: bytes) -> bytes:
    value=requests.parse(raw)
    assert raw==requests.payload(value['values']) and value['end']==len(raw)
    provenance=b'\0' if not native else b'\1'+struct.pack('<i',value['count'])
    result=blob(name.encode())+provenance+blob(raw)+struct.pack('<iI',value['count'],value['end'])+blob(value['values'])
    return result+bytes(value[key] for key in ('complete_payload_framing','engineering_fields_assigned','runtime_index_mapping_reproduced'))


def freeze(root: Path=requests.ROOT) -> None:
    rows=requests.cases();_,native=requests.native_cases()
    packet=b'FC12'+struct.pack('<I',len(rows))+b''.join(case_packet(*row) for row in rows)
    assert len(packet)<=16_000_000
    path=root/'tests/fixtures/flow-counted-bytes/record12.bin.gz'
    path.parent.mkdir(parents=True,exist_ok=True);path.write_bytes(gzip.compress(packet,mtime=0))
    references=('flowrecord12.py','flowlegacytables.py')
    proof=dict(cases=len(rows),bytes=len(packet),sha256=requests.digest(packet),native=native,
               references={name:requests.digest((requests.SOURCE/name).read_bytes().replace(b'\r\n',b'\n')) for name in references},
               scope='Signed nonnegative stored count and exactly N raw bytes through EOF; no version prefix',
               native_runtime_index_mapping_reproduced=False,engineering_fields_assigned=False,native_application_checked=False)
    path=root/'docs/features/flow-counted-bytes/baseline-verification.json'
    path.parent.mkdir(parents=True,exist_ok=True);path.write_text(json.dumps(proof,indent=2)+'\n',encoding='utf-8',newline='\n')


if __name__=='__main__':
    freeze()
