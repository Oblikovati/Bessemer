"""Complete record316 bodies and exact whole-zlib wrappers before typed API work."""
from __future__ import annotations
import json
import struct
import sys
import zlib
from pathlib import Path
sys.path.insert(0,str(Path(__file__).resolve().parent.parent/'flow-counted-bytes'))
import counted_requests as counted
import flowauxrecords
import flowresults
ROOT=counted.ROOT
SOURCE=counted.SOURCE
HEADERS=tuple(sorted({0,1,0x7fffffff,0x80000000,0xffffffff,*[1<<bit for bit in range(32)],*[0xffffffff^(1<<bit) for bit in range(32)]}))


def signed(pattern: int) -> int:
    return pattern if pattern<0x80000000 else pattern-0x100000000


def decoded(header: int, values: bytes) -> bytes:
    return struct.pack('<ii',header,len(values))+values


def parse(stored: bytes) -> dict:
    return flowauxrecords.read_record(dict(identifier=316,size=len(stored),payload=stored))


def native_cases() -> tuple[list[tuple[str,bool,bytes]],dict]:
    manifest=SOURCE/'research/flow-aux-records-corpus.json'
    evidence=json.loads((SOURCE/'research/flow-aux-records-native-evidence.json').read_text())
    assert counted.digest(manifest.read_bytes())==evidence['corpus_sha256']
    rows=json.loads(manifest.read_text())['records']
    entries=[row for row in rows if row['identifier']==316];assert len(entries)==1
    row=entries[0];source=Path(row['path']).read_bytes()
    assert counted.digest(source)==row['source_sha256']
    record=flowresults.latest(flowresults.read(source),316)
    assert (record['start'],record['end'])==(row['record_start'],row['record_end'])
    stored=source[record['start']+12:record['end']]
    assert stored==record['payload'] and counted.digest(stored)==row['payload_sha256']
    value=parse(stored);projection=dict(value);raw=projection.pop('raw_bytes')
    expected=dict(row['projection']);byte_projection=expected.pop('raw_bytes')
    assert projection==expected and raw==bytes.fromhex(byte_projection['raw_hex'])
    assert len(raw)==byte_projection['byte_count']
    proof={key:row[key] for key in ('source_sha256','payload_sha256','record_start','record_end')}
    proof.update(header=value['header_i32'],count=value['count'],decoded_bytes=value['end'],stored_bytes=len(stored),
                 decoded_sha256=counted.digest(decoded(value['header_i32'],raw)),manifest_sha256=counted.digest(manifest.read_bytes()))
    return [('native-fld',True,stored)],proof


def generated_cases() -> list[tuple[str,bool,bytes]]:
    rows=[]
    for header in HEADERS:
        for values in (b'',b'\xff',b'\0\x7f\x80\xff',bytes(range(256))):
            rows.append((f'header-{header:08x}-n{len(values)}',False,zlib.compress(decoded(signed(header),values))))
    for value in range(256):
        for values in (bytes((value,)),bytes((value,255-value,value^128))):
            rows.append((f'byte-{value}-n{len(values)}',False,zlib.compress(decoded(signed(0x80000000|value),values))))
    for name,_,raw in [row for row in counted.generated_cases() if row[0].startswith('length-')]:
        header=signed(HEADERS[len(rows)%len(HEADERS)])
        rows.append((name,False,zlib.compress(struct.pack('<i',header)+raw)))
    rows.extend(compression_cases())
    return rows


def compression_cases() -> list[tuple[str,bool,bytes]]:
    raw=decoded(-2147483648,bytes(range(256))*4)
    rows=[(f'level-{level}',False,zlib.compress(raw,level)) for level in range(10)]
    for strategy in (zlib.Z_DEFAULT_STRATEGY,zlib.Z_FILTERED,zlib.Z_HUFFMAN_ONLY,zlib.Z_RLE,zlib.Z_FIXED):
        for window in (9,15):
            compressor=zlib.compressobj(6,zlib.DEFLATED,window,8,strategy)
            rows.append((f'strategy-{strategy}-window-{window}',False,compressor.compress(raw)+compressor.flush()))
    return rows


def cases() -> list[tuple[str,bool,bytes]]:
    native,_=native_cases()
    return native+generated_cases()
