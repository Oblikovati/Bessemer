"""Freeze complete decoded record444 projections and separately verified native bytes."""
import gzip
import json
import struct
from pathlib import Path
import wide_requests as requests


def blob(raw: bytes) -> bytes:
    return struct.pack('<I',len(raw))+raw


def case_packet(name: str, native: bool, stored: bytes) -> bytes:
    value=requests.parse(stored);raw=requests.zlib.decompress(stored)
    assert value['end']==len(raw)
    provenance=b'\0' if not native else b'\1'+struct.pack('<i',value['count'])
    legacy=value['discarded_legacy_i32'];optional=b'\0' if legacy is None else b'\1'+struct.pack('<i',legacy)
    result=blob(name.encode())+provenance+blob(stored)+blob(raw)+struct.pack('<i',value['version'])+optional
    result+=struct.pack('<iI',value['count'],value['end'])+blob(struct.pack('<'+'Q'*value['count'],*value['patterns']))
    return result+bytes((value['native_values_discarded'],))


def freeze(root: Path=requests.ROOT) -> None:
    rows=requests.cases();_,native=requests.native_cases()
    packet=b'FAW1'+struct.pack('<I',len(rows))+b''.join(case_packet(*row) for row in rows)
    assert len(packet)<=16_000_000
    path=root/'tests/fixtures/flow-aux-wide-values/record444.bin.gz'
    path.parent.mkdir(parents=True,exist_ok=True);path.write_bytes(gzip.compress(packet,mtime=0))
    references=('flowauxrecords.py','flowresultdefinitions.py','flowlegacytables.py')
    proof=dict(cases=len(rows),bytes=len(packet),sha256=requests.counted.digest(packet),native=native,
               references={name:requests.counted.digest((requests.SOURCE/name).read_bytes().replace(b'\r\n',b'\n')) for name in references},
               helper_lf_sha256=requests.counted.digest(Path(requests.objects.__file__).read_bytes().replace(b'\r\n',b'\n')),
               compression_generator=dict(build_version=requests.zlib.ZLIB_VERSION,runtime_version=requests.zlib.ZLIB_RUNTIME_VERSION),
               scope='Whole-zlib versions 0 and 1; optional version-zero discarded signed word; nonnegative count and N raw wide patterns through EOF',
               native_values_discarded=True,native_runtime_population_reproduced=False,engineering_fields_assigned=False,native_application_checked=False)
    path=root/'docs/features/flow-aux-wide-values/baseline-verification.json'
    path.parent.mkdir(parents=True,exist_ok=True);path.write_text(json.dumps(proof,indent=2)+'\n',encoding='utf-8',newline='\n')


if __name__=='__main__':
    freeze()
