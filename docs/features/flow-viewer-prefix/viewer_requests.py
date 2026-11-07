"""Record121 viewer-query prefixes retain the complete opaque decoded suffix."""
from __future__ import annotations
import json
import struct
import sys
import zlib
from itertools import product
from pathlib import Path
sys.path.insert(0,str(Path(__file__).resolve().parent.parent/'flow-aux-wide-values'))
sys.path.insert(0,str(Path(__file__).resolve().parent.parent/'flow-byte-states'))
import wide_requests as wides
import byte_state_requests as byte_states
import flowrecord121
ROOT=wides.ROOT
SOURCE=wides.SOURCE


def body(initial: int, second: int, arrays: list[list[int]], triplet: tuple[int,int,int], final: int, tail: bytes=b'') -> bytes:
    assert len(arrays)==3 and all(arrays)
    raw=struct.pack('<5i',initial,second,*map(len,arrays))
    raw+=b''.join(struct.pack('<'+'Q'*len(values),*values) for values in arrays)
    return raw+struct.pack('<3Qi',*triplet,final)+tail


def stored(version: int, raw: bytes, level: int=6) -> bytes:
    return struct.pack('<i',version)+zlib.compress(raw,level)


def parse(payload: bytes) -> dict:
    return flowrecord121.read_record_prefix(dict(identifier=121,size=len(payload),payload=payload))


def native_cases() -> tuple[list[tuple[str,bool,bytes]],dict]:
    manifest=SOURCE/'research/flow-record121-prefix-corpus.json'
    evidence=json.loads((SOURCE/'research/flow-record121-prefix-native-evidence.json').read_text())
    assert wides.counted.digest(manifest.read_bytes())==evidence['corpus_sha256']
    rows=json.loads(manifest.read_text());assert len(rows)==1;row=rows[0]
    source=Path(row['path']).read_bytes();assert wides.counted.digest(source)==row['source_sha256']
    record=wides.flowresults.latest(wides.flowresults.read(source),121);payload=bytes(record['payload'])
    assert (record['start'],record['end'])==(row['record_start'],row['record_end'])
    assert payload==source[record['start']+12:record['end']] and wides.counted.digest(payload)==row['payload_sha256']
    value=parse(payload);projection=dict(value);tail=projection.pop('opaque_tail')
    expected=dict(row['projection']);opaque=expected.pop('opaque_tail_evidence')
    assert json.loads(json.dumps(projection))==expected
    assert len(tail)==opaque['byte_count'] and wides.counted.digest(tail)==opaque['sha256'] and tail.hex().startswith(opaque['prefix_hex'])
    proof={key:row[key] for key in ('source_sha256','payload_sha256','record_start','record_end')}
    proof.update(version=value['outer_version'],lengths=value['lengths'],known_prefix_bytes=value['prefix_end'],opaque_bytes=len(tail),
                 opaque_sha256=wides.counted.digest(tail),stored_bytes=len(payload),decoded_bytes=value['decoded_body_bytes'],manifest_sha256=wides.counted.digest(manifest.read_bytes()))
    return [('native-cpt-viewer-query',True,payload)],proof


def scalar_cases() -> list[tuple[str,bool,bytes]]:
    rows=[]
    for position,pattern in product(range(3),byte_states.HEADERS):
        scalars=[0,1,-7];scalars[position]=byte_states.signed(pattern)
        raw=body(scalars[0],scalars[1],[[0],[1,2],[3]],(0,0xffffffffffffffff,0x7ff8000000000042),scalars[2],b'opaque')
        rows.append((f'scalar-{position}-{pattern:08x}',False,stored(3,raw)))
    raw=body(-1,1,[[0],[1],[2]],(3,4,5),-7)
    rows.extend((f'version-{version}',False,stored(version,raw)) for version in (2,3,4,5,6,7,8,255,256,65535,65536,1<<30,2147483647))
    return rows


def pattern_cases() -> list[tuple[str,bool,bytes]]:
    rows=[]
    for index,pattern in enumerate(wides.RAW_PATTERNS):
        complement=pattern^0xffffffffffffffff
        raw=body(1,2,[[pattern],[complement],[pattern^0x8000000000000000]],(pattern,complement,pattern),-1)
        rows.append((f'wide-bits-{index}',False,stored(2,raw)))
    for index,lengths in enumerate(product((1,2,3,8),repeat=3)):
        arrays=[wides.objects.patterns(length,index+ordinal) for ordinal,length in enumerate(lengths)]
        rows.append((f'shape-{index}',False,stored(3,body(0,-7,arrays,(0,1,2),3,b'\xff'))))
    return rows


def length_cases() -> list[tuple[str,bool,bytes]]:
    rows=[]
    for ordinal,count in product(range(3),(1,2,3,7,8,15,16,31,32,63,64,127,128,255,256,257,1023,1024,4095,4096,65535,65536)):
        arrays=[[0],[0],[0]];arrays[ordinal]=wides.objects.patterns(count,count)
        raw=body(-2147483648,2147483647,arrays,(0,0x8000000000000000,0xffffffffffffffff),2147483647)
        rows.append((f'length-array-{ordinal}-n{count}',False,stored(3,raw)))
    return rows


def tail_cases() -> list[tuple[str,bool,bytes]]:
    prefix=body(0,1,[[0],[1],[2]],(3,4,5),6)
    rows=[(f'opaque-{count}',False,stored(3,prefix+wides.counted.pattern_bytes(count,0))) for count in (0,1,2,3,7,8,31,32,255,256,1024,4096,65536)]
    native,_=native_cases();payload=native[0][2];decoded=zlib.decompress(payload[4:]);value=parse(payload)
    for count in (0,1,2,3,7,8,31,32,255,256,4096,65535,141226):
        derived=decoded[:value['prefix_end']]+value['opaque_tail'][:count]
        rows.append((f'derived-native-prefix-opaque-{count}',False,stored(value['outer_version'],derived)))
    return rows


def compression_cases() -> list[tuple[str,bool,bytes]]:
    raw=body(-7,1,[wides.objects.patterns(64,index) for index in range(3)],(0,1,2),-1,bytes(range(256)))
    rows=[(f'level-{level}',False,stored(3,raw,level)) for level in range(10)]
    for strategy,window in product((zlib.Z_DEFAULT_STRATEGY,zlib.Z_FILTERED,zlib.Z_HUFFMAN_ONLY,zlib.Z_RLE,zlib.Z_FIXED),(9,15)):
        compressor=zlib.compressobj(6,zlib.DEFLATED,window,8,strategy)
        rows.append((f'strategy-{strategy}-window-{window}',False,struct.pack('<i',3)+compressor.compress(raw)+compressor.flush()))
    return rows


def cases() -> list[tuple[str,bool,bytes]]:
    native,_=native_cases()
    return native+scalar_cases()+pattern_cases()+length_cases()+tail_cases()+compression_cases()
