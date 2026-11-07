"""Independent complete Source-field serialization and exact modern/legacy text projections."""
from __future__ import annotations
from dataclasses import dataclass
import struct
import message_requests
from snapshot import Snapshot
Path=tuple[str | int,...]


@dataclass(frozen=True)
class MessageText:
    path: Path
    start: int
    end: int
    count: int
    legacy: bool
    prefix: bytes | None


def text(out: Snapshot, texts: list[MessageText], path: Path, value: dict, legacy: bool) -> None:
    raw=value['raw_bytes'];count=value['stored_count' if legacy else 'stored_units']
    units=raw if legacy else struct.unpack('<'+'H'*count,raw)
    with out.scope(path,value):
        out.count(path+('stored_count' if legacy else 'stored_units',),count)
        if not legacy:assert value['text_start']==len(out.raw)
        out.put(path+('raw_bytes' if legacy else 'raw_units',),'B' if legacy else 'H',units)
    prefix=prefix_bytes(value,units,legacy)
    texts.append(MessageText(path,value['start'],value['end'],count,legacy,prefix))


def prefix_bytes(value: dict, units: tuple[int,...] | bytes, legacy: bool) -> bytes | None:
    if legacy:
        assert value['text'] is None and value['legacy_byte_transfer'];return None
    assert not units or units[-1]==0
    end=next((index for index,unit in enumerate(units) if unit==0),len(units))
    prefix=value['raw_bytes'][:end*2]
    assert value['text'].encode('utf-16-le',errors='surrogatepass')==prefix
    return prefix


def string_list(out: Snapshot, texts: list[MessageText], path: Path, values: list, legacy: bool) -> None:
    with out.scope(path):
        out.count(path+('count',),len(values))
        for index,value in enumerate(values):text(out,texts,path+(index,),value,legacy)


def item(out: Snapshot, texts: list[MessageText], path: Path, value: dict, legacy: bool) -> None:
    with out.scope(path,value):
        out.scalar(path+('member_8_pattern',),value['member_8_pattern'],'I')
        text(out,texts,path+('text',),value['text'],legacy)
        string_list(out,texts,path+('first_strings',),value['first_strings'],legacy)
        if value['second_strings'] is not None:string_list(out,texts,path+('second_strings',),value['second_strings'],legacy)


def record(value: dict) -> tuple[Snapshot,list[MessageText]]:
    out=Snapshot();texts=[];legacy=value['version']==1
    with out.scope((),dict(start=0,end=value['end'])):
        out.scalar(('version',),value['version']);out.count(('count',),value['count'])
        for index,entry in enumerate(value['items']):
            assert entry['ordinal']==index;item(out,texts,('items',index),entry,legacy)
    assert value['complete_payload_framing'] and not value['legacy_string_interpretation_reproduced'] and not value['engineering_fields_assigned']
    return out,texts
