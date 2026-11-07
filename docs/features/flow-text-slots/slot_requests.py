"""Record311 strict UTF-16 slots and existing-state retention without header dispatch."""
from __future__ import annotations
import json
import struct
import sys
from pathlib import Path
sys.path.insert(0,str(Path(__file__).resolve().parent.parent/'flow-message-lists'))
sys.path.insert(0,str(Path(__file__).resolve().parent.parent/'flow-object-records'))
import message_requests as messages
import object_requests as objects
import flowauxrecords
ROOT=objects.ROOT
SOURCE=objects.SOURCE
OLD_TEXTS=('', 'old', '\0hidden', '\ud800', '\udc00\0tail', '\U0001f680', 'e\u0301', '\u00e9')


def encoded(text: str) -> bytes:
    return text.encode('utf-16le','surrogatepass')


def existing(seed: int) -> list[str]:
    return [OLD_TEXTS[(seed+index)%len(OLD_TEXTS)] for index in range(50)]


def payload(header: int, slots: list[tuple[int,...]]) -> bytes:
    assert len(slots)==50
    # The strict transfer matches modern record301 text; the record311 header stays opaque.
    return struct.pack('<i',header)+b''.join(messages.text(3,units) for units in slots)


def parse(raw: bytes) -> dict:
    return flowauxrecords.read_record(dict(identifier=311,size=len(raw),payload=raw),string_mode=0)


def native_cases() -> tuple[list[tuple[str,bool,bytes,list[str]]],dict]:
    manifest=SOURCE/'research/flow-aux-records-corpus.json'
    evidence=json.loads((SOURCE/'research/flow-aux-records-native-evidence.json').read_text())
    assert objects.digest(manifest.read_bytes())==evidence['corpus_sha256']
    rows=[row for row in json.loads(manifest.read_text())['records'] if row['identifier']==311];assert len(rows)==1
    row=rows[0];source=Path(row['path']).read_bytes();assert objects.digest(source)==row['source_sha256']
    record=objects.flowresults.latest(objects.flowresults.read(source),311);raw=bytes(record['payload'])
    assert (record['start'],record['end'])==(row['record_start'],row['record_end'])
    assert raw==source[record['start']+12:record['end']] and objects.digest(raw)==row['payload_sha256']
    value=parse(raw);assert objects.normalize(value)==row['projection']
    proof={key:row[key] for key in ('source_sha256','payload_sha256','record_start','record_end')}
    proof.update(header=value['header_i32'],slots=len(value['slots']),units=sum(slot['stored_units'] for slot in value['slots']),
                 payload_bytes=len(raw),manifest_sha256=objects.digest(manifest.read_bytes()),mode_assumption=0,native_owner_mode_verified=False)
    return [('native-fld',True,raw,existing(0))],proof


def single_slot(header: int, ordinal: int, units: tuple[int,...]) -> bytes:
    slots=[() for _ in range(50)];slots[ordinal]=units
    return payload(header,slots)


def generated_cases() -> list[tuple[str,bool,bytes,list[str]]]:
    rows=[]
    for ordinal in range(50):
        rows.extend((f'slot-{ordinal}-text-{index}',False,single_slot(-7,ordinal,units),existing(index)) for index,units in enumerate(messages.MODERN))
    headers=sorted({0,0xffffffff,*[1<<bit for bit in range(32)],*[0xffffffff^(1<<bit) for bit in range(32)]})
    for index,pattern in enumerate(headers):
        header=pattern if pattern<0x80000000 else pattern-0x100000000
        slots=[messages.MODERN[(index+ordinal)%len(messages.MODERN)] for ordinal in range(50)]
        rows.append((f'header-{pattern:08x}',False,payload(header,slots),existing(index)))
    rows.append(('all-units-prefix',False,single_slot(2147483647,0,tuple(range(1,65536))+(0,)),existing(2)))
    rows.append(('all-units-hidden',False,single_slot(-2147483648,49,(0,)+tuple(range(65535,0,-1))+(0,)),existing(3)))
    return rows+length_cases()


def length_cases() -> list[tuple[str,bool,bytes,list[str]]]:
    rows=[]
    for count in (0,1,2,3,7,8,15,16,31,32,127,128,255,256,257,1024,4096,65536):
        units=tuple(index%65535+1 for index in range(max(0,count-1)))+(0,) if count else ()
        rows.append((f'length-{count}-prefix',False,single_slot(-1,49,units),existing(count)))
        hidden=(0,)+units[1:] if units else ()
        rows.append((f'length-{count}-hidden',False,single_slot(1,0,hidden),existing(count+1)))
    return rows


def cases() -> list[tuple[str,bool,bytes,list[str]]]:
    native,_=native_cases()
    return native+generated_cases()
