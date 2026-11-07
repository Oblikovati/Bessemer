"""Owned whole-zlib record316 packets with complete original decoded projections."""
import gzip
import json
import struct
import zlib
from pathlib import Path
import byte_state_requests as requests


def blob(raw: bytes) -> bytes:
    return struct.pack('<I',len(raw))+raw


def case_packet(name: str, native: bool, stored: bytes) -> bytes:
    value=requests.parse(stored)
    raw=requests.decoded(value['header_i32'],value['raw_bytes'])
    assert value['end']==len(raw) and value['count']==len(value['raw_bytes'])
    provenance=b'\0' if not native else b'\1'+struct.pack('<i',value['count'])
    result=blob(name.encode())+provenance+blob(stored)+blob(raw)
    result+=struct.pack('<iiI',value['header_i32'],value['count'],value['end'])+blob(value['raw_bytes'])
    return result+bytes((value['native_header_dispatch'],))


def freeze(root: Path=requests.ROOT) -> None:
    rows=requests.cases();_,native=requests.native_cases()
    packet=b'FBS1'+struct.pack('<I',len(rows))+b''.join(case_packet(*row) for row in rows)
    assert len(packet)<=16_000_000
    path=root/'tests/fixtures/flow-byte-states/record316.bin.gz'
    path.parent.mkdir(parents=True,exist_ok=True);path.write_bytes(gzip.compress(packet,mtime=0))
    references=('flowauxrecords.py','flowresultdefinitions.py','flowlegacytables.py')
    proof=dict(cases=len(rows),bytes=len(packet),sha256=requests.counted.digest(packet),native=native,
               references={name:requests.counted.digest((requests.SOURCE/name).read_bytes().replace(b'\r\n',b'\n')) for name in references},
               compression_generator=dict(build_version=zlib.ZLIB_VERSION,runtime_version=zlib.ZLIB_RUNTIME_VERSION),
               scope='Exact whole-zlib stream; arbitrary signed header without dispatch, nonnegative stored count and exactly N raw bytes through decoded EOF',
               native_header_dispatch=False,runtime_index_mapping_reproduced=False,engineering_fields_assigned=False,native_application_checked=False)
    path=root/'docs/features/flow-byte-states/baseline-verification.json'
    path.parent.mkdir(parents=True,exist_ok=True);path.write_text(json.dumps(proof,indent=2)+'\n',encoding='utf-8',newline='\n')


if __name__=='__main__':
    freeze()
