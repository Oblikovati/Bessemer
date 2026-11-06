"""Complete record-12 payloads, with raw bytes independent of runtime index meaning."""
from __future__ import annotations
import hashlib
import json
import struct
import sys
from pathlib import Path
ROOT=Path(__file__).resolve().parents[3]
SOURCE=ROOT.parent/'inventor-interface'/'solidworks'
sys.path.insert(0,str(SOURCE))
import flowrecord12
import flowresults


def digest(raw: bytes) -> str:
    return hashlib.sha256(raw).hexdigest()


def payload(values: bytes) -> bytes:
    return struct.pack('<i',len(values))+values


def parse(raw: bytes) -> dict:
    return flowrecord12.read_record(dict(identifier=12,size=len(raw),payload=raw))


def native_cases() -> tuple[list[tuple[str,bool,bytes]],dict]:
    manifest=SOURCE/'research/flow-record12-corpus.json'
    evidence=json.loads((SOURCE/'research/flow-record12-native-evidence.json').read_text())
    assert digest(manifest.read_bytes())==evidence['corpus_sha256']
    rows=json.loads(manifest.read_text());assert len(rows)==1
    entry=rows[0];source=Path(entry['path']).read_bytes()
    assert digest(source)==entry['source_sha256']
    record=flowresults.latest(flowresults.read(source),12)
    assert (record['start'],record['end'])==(entry['record_start'],entry['record_end'])
    raw=source[record['start']+12:record['end']]
    assert raw==record['payload'] and digest(raw)==entry['payload_sha256']
    value=parse(raw);projection=dict(value);values=projection.pop('values')
    expected=dict(entry['projection']);assert bytes.fromhex(expected.pop('values_hex'))==values
    expected.pop('byte_frequencies');assert projection==expected
    proof={key:entry[key] for key in ('source_sha256','payload_sha256','record_start','record_end')}
    proof.update(count=len(values),payload_bytes=len(raw),manifest_sha256=digest(manifest.read_bytes()))
    return [('native-cfld',True,raw)],proof


def generated_cases() -> list[tuple[str,bool,bytes]]:
    rows=[(f'byte-{value}',False,payload(bytes((value,)))) for value in range(256)]
    rows += [(f'triplet-{value}',False,payload(bytes((value,255-value,value^128)))) for value in range(256)]
    for length in (0,1,2,3,4,7,8,15,16,31,32,63,64,127,128,255,256,257,1023,1024,4095,4096,65535,65536):
        for pattern in range(7):
            rows.append((f'length-{length}-pattern-{pattern}',False,payload(pattern_bytes(length,pattern))))
    return rows


def pattern_bytes(length: int, pattern: int) -> bytes:
    cycles=(bytes(range(256)),bytes(reversed(range(256))),b'\0',b'\xff',b'\x7f\x80',
            bytes(1<<index for index in range(8)),bytes(255^(1<<index) for index in range(8)))
    cycle=cycles[pattern]
    return (cycle*(length//len(cycle)+1))[:length]



def cases() -> list[tuple[str,bool,bytes]]:
    native,_=native_cases()
    return native+generated_cases()
