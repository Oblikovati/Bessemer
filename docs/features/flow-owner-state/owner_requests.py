"""Record122/434 stored fields and documented latest-occurrence update selection."""
from __future__ import annotations
import json
from itertools import product
import struct
import sys
from pathlib import Path
sys.path.insert(0,str(Path(__file__).resolve().parent.parent/'flow-counted-bytes'))
import counted_requests as counted
import flowauxrecords
import flowresults
ROOT=counted.ROOT
SOURCE=counted.SOURCE
OFFSETS=(8,12,16,20,24,28,32)
PATTERNS=sorted({0,0xffffffff,*[1<<bit for bit in range(32)],*[0xffffffff^(1<<bit) for bit in range(32)]})


def words(values: list[int]) -> bytes:
    return struct.pack('<'+'I'*len(values),*(value&0xffffffff for value in values))


def legacy(position: int, pattern: int) -> bytes:
    values=[3]+list(range(1,50));values[position]=pattern
    return words(values)


def modern(version: int, values: list[int], state: int) -> bytes:
    return struct.pack('<i',version)+words(values)+bytes((state,))


def parse(identifier: int, raw: bytes) -> dict:
    return flowauxrecords.read_record(dict(identifier=identifier,size=len(raw),payload=raw))


def native_cases() -> tuple[list[tuple[str,int,bool,bytes]],list[dict]]:
    manifest=SOURCE/'research/flow-aux-records-corpus.json'
    evidence=json.loads((SOURCE/'research/flow-aux-records-native-evidence.json').read_text())
    assert counted.digest(manifest.read_bytes())==evidence['corpus_sha256']
    rows=[];proof=[]
    for index,row in enumerate(json.loads(manifest.read_text())['records']):
        if row['identifier'] not in (122,434):continue
        source=Path(row['path']).read_bytes();assert counted.digest(source)==row['source_sha256']
        identifier=row['identifier'];record=flowresults.latest(flowresults.read(source),identifier);raw=bytes(record['payload'])
        assert (record['start'],record['end'])==(row['record_start'],row['record_end'])
        assert raw==source[record['start']+12:record['end']] and counted.digest(raw)==row['payload_sha256']
        assert json.loads(json.dumps(parse(identifier,raw)))==row['projection']
        rows.append((f'native-{index}',identifier,True,raw))
        proof.append({key:row[key] for key in ('identifier','source_sha256','payload_sha256','record_start','record_end')})
    return rows,proof


def generated_cases() -> list[tuple[str,int,bool,bytes]]:
    rows=[(f'word-{position}-bits-{pattern:08x}',122,False,legacy(position,pattern)) for position in range(50) for pattern in PATTERNS]
    for gate in (0,1,0x7fffffff,0x80000000,0xfffffffe,0xffffffff):
        rows.append((f'gate-{gate:08x}',122,False,words([gate]+[(0x80000000+index*65537)&0xffffffff for index in range(49)])))
    for version in (0,1):
        for position,pattern in product(range(6),PATTERNS):
            values=list(range(6));values[position]=pattern
            rows.append((f'v{version}-member-{position}-bits-{pattern:08x}',434,False,modern(version,values,254)))
        rows.extend((f'v{version}-state-{state}',434,False,modern(version,[0,1,-1,-2147483648,2147483647,7],state)) for state in range(256))
    return rows


def cases() -> list[tuple[str,int,bool,bytes]]:
    native,_=native_cases()
    return native+generated_cases()


def carrier(records: list[tuple[int,bytes]]) -> bytes:
    return struct.pack('<H',8)+b''.join(struct.pack('<IQ',identifier,len(raw))+raw for identifier,raw in records)


def selection_cases() -> list[tuple[str,bytes]]:
    first=(122,legacy(1,0x80000000));disabled=(122,legacy(0,0xffffffff))
    modern0=(434,modern(0,[1,2,3,4,5,6],0));modern1=(434,modern(1,[-1,-2,-3,-4,-5,-6],255))
    bad122=(122,b'');bad434=(434,b'');unknown=(999,b'opaque')
    bad_version=(434,modern(2,list(range(6)),0))
    scenarios=[[],[first],[disabled],[modern0],[modern1],[first,modern0],[modern0,first],
               [first,modern0,modern1],[modern1,first,modern0],[first,bad122],[bad122,first],
               [first,bad434],[bad434,first],[bad122,modern0],[bad434,modern1],
               [modern1,bad434],[disabled,bad434],[unknown],[unknown,first,unknown,modern0,unknown],
               [first,bad_version],[modern0,bad_version],[first,(434,modern0[1][:-1])],
               [(122,first[1][:-1])],[(122,first[1]+b"\0"),modern0],[unknown,bad_version,modern1]]
    rows=[(f'selection-{index}',carrier(records)) for index,records in enumerate(scenarios)]
    native,_=native_cases();assert [row[1] for row in native]==[122,434,122]
    records=[(identifier,raw) for _,identifier,_,raw in native]
    rows.extend((('native-cpt-projection',carrier([records[1],records[0]])),('native-fld-projection',carrier(records[2:]))))
    return rows
