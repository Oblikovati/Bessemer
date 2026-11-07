"""Record148 raw vectors preserve length-table order and present-empty arrays."""
from __future__ import annotations
import json
import struct
import sys
from pathlib import Path
sys.path.insert(0,str(Path(__file__).resolve().parent.parent/'flow-aux-wide-values'))
import wide_requests as wides
ROOT=wides.ROOT
SOURCE=wides.SOURCE


def payload(arrays: list[list[int]]) -> bytes:
    raw=struct.pack('<ii',0,len(arrays))+struct.pack('<'+'i'*len(arrays),*map(len,arrays))
    return raw+b''.join(struct.pack('<'+'Q'*len(values),*values) for values in arrays)


def parse(raw: bytes) -> dict:
    return wides.flowauxrecords.read_record(dict(identifier=148,size=len(raw),payload=raw))


def native_cases() -> tuple[list[tuple[str,bool,bytes]],dict]:
    manifest=SOURCE/'research/flow-aux-records-corpus.json'
    evidence=json.loads((SOURCE/'research/flow-aux-records-native-evidence.json').read_text())
    assert wides.counted.digest(manifest.read_bytes())==evidence['corpus_sha256']
    entries=[row for row in json.loads(manifest.read_text())['records'] if row['identifier']==148];assert len(entries)==1
    row=entries[0];source=Path(row['path']).read_bytes();assert wides.counted.digest(source)==row['source_sha256']
    record=wides.flowresults.latest(wides.flowresults.read(source),148);raw=bytes(record['payload'])
    assert (record['start'],record['end'])==(row['record_start'],row['record_end'])
    assert raw==source[record['start']+12:record['end']] and wides.counted.digest(raw)==row['payload_sha256']
    value=parse(raw);assert json.loads(json.dumps(value))==row['projection']
    assert payload(value['arrays'])==raw
    proof={key:row[key] for key in ('source_sha256','payload_sha256','record_start','record_end')}
    proof.update(count=value['count'],lengths=value['lengths'],patterns=value['arrays'],payload_bytes=len(raw),manifest_sha256=wides.counted.digest(manifest.read_bytes()))
    return [('native-fld',True,raw)],proof


def generated_cases() -> list[tuple[str,bool,bytes]]:
    rows=[(f'pattern-{index}',False,payload([[pattern]])) for index,pattern in enumerate(wides.RAW_PATTERNS)]
    for first in range(8):
        for second in range(8):
            arrays=[wides.objects.patterns(first,first),[],wides.objects.patterns(second,second+17)]
            rows.append((f'ragged-{first}-{second}',False,payload(arrays)))
    rows+=count_cases()+length_cases()
    for count in (3,7,32):
        for index in range(count):
            arrays=[[] for _ in range(count)];arrays[index]=wides.RAW_PATTERNS
            rows.append((f'populated-slot-{count}-{index}',False,payload(arrays)))
    rows.append(('mixed-bit-vectors',False,payload([wides.RAW_PATTERNS,[],list(reversed(wides.RAW_PATTERNS)),[0]])))
    rows.append(('large-unequal-vectors',False,payload([wides.objects.patterns(4096,0),[],wides.objects.patterns(4095,37)])))
    return rows


def count_cases() -> list[tuple[str,bool,bytes]]:
    rows=[]
    for count in (0,1,2,3,7,8,15,16,31,32,127,128,255,256,257,1024,4096):
        profiles=[[[] for _ in range(count)],[wides.objects.patterns(index%3,index) for index in range(count)]]
        for position in sorted({0,count//2,count-1}):
            if count==0:continue
            arrays=[[] for _ in range(count)];arrays[position]=[0xffffffffffffffff];profiles.append(arrays)
        rows.extend((f'vector-count-{count}-profile-{index}',False,payload(arrays)) for index,arrays in enumerate(profiles))
    return rows


def length_cases() -> list[tuple[str,bool,bytes]]:
    rows=[]
    for count in (0,1,2,3,7,8,15,16,31,32,63,64,127,128,255,256,257,1023,1024,4095,4096,65535,65536):
        for index,values in enumerate((wides.objects.patterns(count,count),[0]*count,[0xffffffffffffffff]*count)):
            rows.append((f'array-length-{count}-profile-{index}',False,payload([[],values,[]])))
    return rows


def cases() -> list[tuple[str,bool,bytes]]:
    native,_=native_cases()
    return native+generated_cases()
