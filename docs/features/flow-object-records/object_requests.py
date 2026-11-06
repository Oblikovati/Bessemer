"""Complete record-304 rows, preserving discarded fields and conditional trailers."""
from __future__ import annotations
import hashlib
import json
import struct
import sys
from pathlib import Path
ROOT=Path(__file__).resolve().parents[3]
SOURCE=ROOT.parent/'inventor-interface'/'solidworks'
sys.path.insert(0,str(SOURCE))
import flowrecord304
import flowresults


def digest(raw: bytes) -> str:
    return hashlib.sha256(raw).hexdigest()


def parse(raw: bytes) -> dict:
    return flowrecord304.read_record(dict(identifier=304,size=len(raw),payload=raw))


def signed(pattern: int) -> bytes:
    return struct.pack('<I',pattern&0xffffffff)


def row_wire(version: int, pattern: int) -> bytes:
    raw=signed(pattern)
    if version<5:raw+=b''.join(signed(pattern^mask) for mask in (0,0xffffffff,0x80000000,0x7fffffff))
    raw+=bytes((pattern+index)&255 for index in range(16))
    if version>=4:raw+=signed(~pattern)
    if version>=6:raw+=bytes((pattern&255,))
    if version>=7:raw+=bytes((pattern^index)&255 for index in range(56))
    return raw


def payload(version: int, patterns: list[int]) -> bytes:
    raw=struct.pack('<ii',version,len(patterns))+b''.join(row_wire(version,p) for p in patterns)
    if patterns and version<5:raw+=signed(0x80000001)+bytes(reversed(range(16)))
    return raw


def normalize(value: object) -> object:
    if isinstance(value,bytes):return dict(raw_hex=value.hex(),byte_count=len(value))
    if isinstance(value,dict):return {key:normalize(item) for key,item in value.items()}
    if isinstance(value,(tuple,list)):return [normalize(item) for item in value]
    return value


def native_cases() -> tuple[list[tuple[str,bool,bytes]],list[dict]]:
    manifest=SOURCE/'research/flow-record304-corpus.json'
    evidence=json.loads((SOURCE/'research/flow-record304-native-evidence.json').read_text())
    assert digest(manifest.read_bytes())==evidence['corpus_sha256']
    rows=[];proof=[]
    for index,entry in enumerate(json.loads(manifest.read_text())):
        source=Path(entry['path']).read_bytes();assert digest(source)==entry['source_sha256']
        record=flowresults.latest(flowresults.read(source),304);raw=bytes(record['payload'])
        assert (record['start'],record['end'])==(entry['record_start'],entry['record_end'])
        assert raw==source[record['start']+12:record['end']] and digest(raw)==entry['payload_sha256']
        assert normalize(parse(raw))==entry['projection']
        rows.append((f'native-{index}',True,raw))
        proof.append({key:entry[key] for key in ('source_sha256','payload_sha256','record_start','record_end')})
    return rows,proof


def generated_cases() -> list[tuple[str,bool,bytes]]:
    patterns=[0,0xffffffff,0x80000000,0x7fffffff]+[1<<bit for bit in range(32)]+[~(1<<bit) for bit in range(32)]
    rows=[]
    for version in range(8):
        rows.extend((f'v{version}-pattern-{index}',False,payload(version,[pattern])) for index,pattern in enumerate(patterns))
        rows.extend((f'v{version}-byte-{byte}',False,payload(version,[byte,255-byte])) for byte in range(256))
        for count in (0,1,2,3,7,16,31,32,255,256,257,1024):
            rows.append((f'v{version}-count-{count}',False,payload(version,[patterns[index%len(patterns)] for index in range(count)])))
    return rows


def cases() -> list[tuple[str,bool,bytes]]:
    native,_=native_cases()
    return native+generated_cases()
