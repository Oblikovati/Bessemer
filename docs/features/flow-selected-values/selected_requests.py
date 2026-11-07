"""Complete real record402/316 inputs and fresh record112 selections."""
from __future__ import annotations
from dataclasses import dataclass,replace
import json
import struct
import sys
import zlib
from pathlib import Path
FEATURES=Path(__file__).resolve().parent.parent
sys.path.insert(0,str(FEATURES/'flow-counted-bytes'))
import counted_requests as counted
sys.path.insert(0,str(FEATURES/'flow-link-records'))
import freeze_links as links_fixture
sys.path.insert(0,str(FEATURES/'flow-medium-objects'))
import freeze_objects as wide_fixture
import flowrecord112 as schema
import flowauxrecords
import flowlinkrecords
import flowresults
ROOT=counted.ROOT
SOURCE=counted.SOURCE
ACCEPTED=(0,2,3,4,6,7,10,11,12,13,14,15,30,31,32,50,51,52,60,61,62,70,72,73,74,76,77,80,82,83,84,88,90,91,92)


@dataclass(frozen=True)
class SelectionSpec:
    name: str
    compressed_links: bool
    links: bytes
    states: bytes
    values: bytes
    native: bool=False


def parse(spec: SelectionSpec) -> tuple[dict,dict,dict,dict]:
    links=flowlinkrecords.read_record(dict(identifier=402,size=len(spec.links),payload=spec.links),
                                    compression='zlib' if spec.compressed_links else 'none')
    states=flowauxrecords.read_record(dict(identifier=316,size=len(spec.states),payload=spec.states))
    context=schema.derive_context(links,states,fresh_map=True)
    value=schema.read_record(dict(identifier=112,size=len(spec.values),payload=spec.values),context=context)
    return links,states,context,value


def from_inputs(name: str, version: int, codes: bytes, flags: bytes, seed: int) -> SelectionSpec:
    raw=struct.pack('<ii',version,len(codes))
    raw+=b''.join(links_fixture.entry_wire(index,'zero',code) for index,code in enumerate(codes))
    raw+=links_fixture.tail_wire(version,'zero')
    state_body=struct.pack('<ii',-seed-1,len(flags))+flags
    links=flowlinkrecords.read_decoded(raw);states=flowauxrecords.read_decoded_316(state_body)
    context=schema.derive_context(links,states,fresh_map=True)
    patterns=wide_fixture.patterns(context['selected_count'],seed)
    compressed=bool(seed&1)
    return SelectionSpec(name,compressed,zlib.compress(raw) if compressed else raw,
                         zlib.compress(state_body),links_fixture.numbers(patterns,'Q'))


def grid_cases() -> list[SelectionSpec]:
    rows=[]
    for version in range(10):
        flags=bytes(range(256)) if version in (0,9) else bytes((0,1,127,128,129,255))
        for code in range(256):
            rows.append(from_inputs(f'grid-v{version}-code-{code:02x}',version,
                                    bytes((code,))*len(flags),flags,version*256+code))
    return rows


def shape_cases() -> list[SelectionSpec]:
    rows=[]
    for version in range(10):
        rows.append(from_inputs(f'empty-v{version}',version,b'',b'',0))
        rows.append(from_inputs(f'absent-states-v{version}',version,bytes((0,4,92,255)),b'',1))
        flags=bytes((0,1,127,128,129,255)[index%6] for index in range(256))
        rows.append(from_inputs(f'mixed-v{version}',version,bytes(range(256)),flags,2))
    selected=from_inputs('selected-zero-distinct-from-absent',9,bytes((0,1,4,5)),bytes((128,255,128,0)),0)
    rows.append(replace(selected,values=bytes(16)))
    for seed in range(len(wide_fixture.PATTERNS)):
        rows.append(from_inputs(f'pattern-{seed}',9,b'\0',b'\x80',seed))
    selected=from_inputs('all-ones-pattern',9,b'\0',b'\x80',0)
    rows.append(replace(selected,values=struct.pack('<Q',0xffffffffffffffff)))
    return rows


def audited_input(row: dict) -> dict:
    source=Path(row['path']).read_bytes()
    assert counted.digest(source)==row['source_sha256']
    record=flowresults.latest(flowresults.read(source),row['identifier'])
    assert (record['start'],record['end'])==(row['record_start'],row['record_end'])
    assert counted.digest(record['payload'])==row['payload_sha256']
    return record


def native_cases() -> tuple[list[SelectionSpec],dict]:
    manifest=SOURCE/'research/flow-record112-corpus.json'
    evidence=json.loads((SOURCE/'research/flow-record112-native-evidence.json').read_text())
    assert counted.digest(manifest.read_bytes())==evidence['corpus_sha256']
    original=json.loads(manifest.read_text())
    inputs={row['identifier']:audited_input(row) for row in original['inputs']}
    spec=SelectionSpec('native-cpt-fld',True,inputs[402]['payload'],inputs[316]['payload'],inputs[112]['payload'],True)
    _,_,context,value=parse(spec)
    assert json.loads(json.dumps(context))==original['context']
    assert json.loads(json.dumps(value))==original['projection']
    proof=dict(inputs=original['inputs'],link_count=context['link_count'],selected_count=context['selected_count'],
               value_bytes=value['end'],manifest_sha256=counted.digest(manifest.read_bytes()),
               automatic_pairing_proven=False,tag_equality_is_observation_only=original['tag_equality_is_observation_only'])
    return [spec],proof


def cases() -> list[SelectionSpec]:
    native,_=native_cases()
    return native+grid_cases()+shape_cases()
