"""Freeze every record148 array and interval after the complete length table."""
import gzip
import json
import struct
from pathlib import Path
import vector_requests as requests


def blob(raw: bytes) -> bytes:
    return struct.pack('<I',len(raw))+raw


def vector_packet(values: tuple[int,...], start: int) -> bytes:
    raw=struct.pack('<'+'Q'*len(values),*values)
    return struct.pack('<iII',len(values),start,start+len(raw))+blob(raw)


def case_packet(name: str, native: bool, raw: bytes) -> bytes:
    value=requests.parse(raw);assert raw==requests.payload(value['arrays']) and value['end']==len(raw)
    provenance=b'\0' if not native else b'\1'+struct.pack('<i',value['count'])
    result=blob(name.encode())+provenance+blob(raw)+struct.pack('<iiII',value['version'],value['count'],value['end'],len(value['arrays']))
    start=8+4*value['count']
    for length,values in zip(value['lengths'],value['arrays']):
        assert length==len(values);result+=vector_packet(values,start);start+=length*8
    assert start==len(raw)
    return result


def freeze(root: Path=requests.ROOT) -> None:
    rows=requests.cases();_,native=requests.native_cases()
    packet=b'FWV1'+struct.pack('<I',len(rows))+b''.join(case_packet(*row) for row in rows)
    assert len(packet)<=16_000_000
    path=root/'tests/fixtures/flow-wide-vectors/record148.bin.gz'
    path.parent.mkdir(parents=True,exist_ok=True);path.write_bytes(gzip.compress(packet,mtime=0))
    references=('flowauxrecords.py','flowlegacytables.py')
    proof=dict(cases=len(rows),bytes=len(packet),sha256=requests.wides.counted.digest(packet),native=native,
               references={name:requests.wides.counted.digest((requests.SOURCE/name).read_bytes().replace(b'\r\n',b'\n')) for name in references},
               helper_lf_sha256={Path(module.__file__).name:requests.wides.counted.digest(Path(module.__file__).read_bytes().replace(b'\r\n',b'\n')) for module in (requests.wides,requests.wides.objects)},
               scope='Complete raw version-zero vector count, all signed lengths before array bodies, all raw wide patterns and exact EOF',
               native_runtime_population_reproduced=False,engineering_fields_assigned=False,native_application_checked=False)
    path=root/'docs/features/flow-wide-vectors/baseline-verification.json'
    path.parent.mkdir(parents=True,exist_ok=True);path.write_text(json.dumps(proof,indent=2)+'\n',encoding='utf-8',newline='\n')


if __name__=='__main__':
    freeze()
