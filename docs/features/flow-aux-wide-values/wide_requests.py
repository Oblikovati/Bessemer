"""Complete record444 bodies with raw values retained despite native discard behavior."""
from __future__ import annotations
import json
import struct
import sys
import zlib
from pathlib import Path
sys.path.insert(0,str(Path(__file__).resolve().parent.parent/'flow-counted-bytes'))
sys.path.insert(0,str(Path(__file__).resolve().parent.parent/'flow-medium-objects'))
import counted_requests as counted
import freeze_objects as objects
import flowauxrecords
import flowresults
ROOT=counted.ROOT
SOURCE=counted.SOURCE
RAW_PATTERNS=objects.PATTERNS+[0xffffffffffffffff]


def decoded(version: int, legacy: int, values: list[int]) -> bytes:
    raw=struct.pack('<i',version)
    if version==0:raw+=struct.pack('<i',legacy)
    return raw+struct.pack('<i',len(values))+struct.pack('<'+'Q'*len(values),*values)


def parse(stored: bytes) -> dict:
    return flowauxrecords.read_record(dict(identifier=444,size=len(stored),payload=stored))


def native_cases() -> tuple[list[tuple[str,bool,bytes]],dict]:
    manifest=SOURCE/'research/flow-aux-records-corpus.json'
    evidence=json.loads((SOURCE/'research/flow-aux-records-native-evidence.json').read_text())
    assert counted.digest(manifest.read_bytes())==evidence['corpus_sha256']
    entries=[row for row in json.loads(manifest.read_text())['records'] if row['identifier']==444];assert len(entries)==1
    row=entries[0];source=Path(row['path']).read_bytes();assert counted.digest(source)==row['source_sha256']
    record=flowresults.latest(flowresults.read(source),444);stored=bytes(record['payload'])
    assert (record['start'],record['end'])==(row['record_start'],row['record_end'])
    assert stored==source[record['start']+12:record['end']] and counted.digest(stored)==row['payload_sha256']
    value=parse(stored);projection=json.loads(json.dumps(value));assert projection==row['projection']
    raw=zlib.decompress(stored);assert raw==decoded(value['version'],value['discarded_legacy_i32'],value['patterns'])
    proof={key:row[key] for key in ('source_sha256','payload_sha256','record_start','record_end')}
    proof.update(version=value['version'],legacy=value['discarded_legacy_i32'],count=value['count'],stored_bytes=len(stored),
                 decoded_bytes=len(raw),decoded_sha256=counted.digest(raw),manifest_sha256=counted.digest(manifest.read_bytes()))
    return [('native-fld',True,stored)],proof


def generated_cases() -> list[tuple[str,bool,bytes]]:
    rows=[];legacy_patterns=sorted({0,0xffffffff,*[1<<bit for bit in range(32)],*[0xffffffff^(1<<bit) for bit in range(32)]})
    for pattern in legacy_patterns:
        legacy=pattern if pattern<0x80000000 else pattern-0x100000000
        for values in ([],[0xffffffffffffffff],RAW_PATTERNS):
            rows.append((f'legacy-{pattern:08x}-n{len(values)}',False,zlib.compress(decoded(0,legacy,values))))
    for version in (0,1):
        for index,pattern in enumerate(RAW_PATTERNS):
            rows.append((f'v{version}-bits-{index}',False,zlib.compress(decoded(version,-7,[pattern]))))
        for count in (0,1,2,3,7,8,15,16,31,32,63,64,127,128,255,256,257,1023,1024,4095,4096,65535,65536):
            rows.append((f'v{version}-count-{count}',False,zlib.compress(decoded(version,-2147483648,objects.patterns(count,count)))))
    return rows+compression_cases()


def compression_cases() -> list[tuple[str,bool,bytes]]:
    rows=[]
    for version in (0,1):
        raw=decoded(version,2147483647,objects.patterns(1024,7))
        rows.extend((f'v{version}-level-{level}',False,zlib.compress(raw,level)) for level in range(10))
        for strategy in (zlib.Z_DEFAULT_STRATEGY,zlib.Z_FILTERED,zlib.Z_HUFFMAN_ONLY,zlib.Z_RLE,zlib.Z_FIXED):
            for window in (9,15):
                compressor=zlib.compressobj(6,zlib.DEFLATED,window,8,strategy)
                rows.append((f'v{version}-strategy-{strategy}-window-{window}',False,compressor.compress(raw)+compressor.flush()))
    return rows


def cases() -> list[tuple[str,bool,bytes]]:
    native,_=native_cases()
    return native+generated_cases()
