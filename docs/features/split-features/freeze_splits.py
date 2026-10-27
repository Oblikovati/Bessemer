"""Freeze complete split graphs and native loaded-map interpretations (#54)."""
from pathlib import Path
from typing import cast
import json
import sys

WORKSPACE=Path(__file__).resolve().parents[3]
for folder in ('trim-edge-identifiers','reference-axes','reference-axis-owners','reference-points','coordinate-systems','component-coordinate-systems','split-features'):
    sys.path.insert(0,str(WORKSPACE/'docs/features'/folder))
from freeze_trim_edges import ObjectFixtureFamily, REFERENCE, freeze, owned
from freeze_blank_part import reference_snapshot
from object_fixture_observer import object_cursor
from split_fixture_bytes import VERSIONS, PAIRS, COUNTS, split_object


def verify_identity(reference: Path, raw: bytes, version: int, kind: int, style: int) -> None:
    """Check source target identities and native map filtering without changing raw pairs; e.g. verify_identity(source,raw,19000,0,2)."""
    root=cast(dict[str,object],object_cursor(reference,raw,version,None).object())
    if style in (2,5):
        first,second,repeated=cast(list[dict[str,object]],root['items'])
        if first is second or repeated is not first:raise ValueError('split roots; expected distinct equal roots and repeated first')
        root=first
    if kind==0:
        payload=cast(dict[str,object],root['payload'])
        expected={k:v for k,v in PAIRS[style] if k>=0 and v>=0}
        if payload['map_9a8_count']!=COUNTS[style] or payload['raw_map_9a8']!=list(PAIRS[style]) or payload['loaded_map_9a8']!=expected:
            raise ValueError(f'split map {payload}; expected raw pairs {PAIRS[style]} and native result {expected}')


FAMILY=ObjectFixtureFamily(build=split_object,verify=verify_identity,magic=b'SPL1',kinds=4,
                          versions=VERSIONS,edited_kinds=(0,1))


def freeze_split_fixtures(reference: Path, destination: Path) -> dict[str,object]:
    """Also exercise an isolated body-absent parent edit with a populated native map; e.g. freeze_split_fixtures(source,output)."""
    facts=freeze(reference,destination,FAMILY)
    extra=owned(reference,destination,0,4,True,'owned-0-4-edited',FAMILY)
    if facts['source_hashes']!=reference_snapshot(reference):
        raise ValueError('split fixture source changed; expected one stable snapshot across all owned edits')
    cast(list[dict[str,object]],facts['owned']).append(extra)
    return facts


if __name__=='__main__':
    facts=freeze_split_fixtures(REFERENCE,WORKSPACE/'tests/fixtures/split-features')
    Path(__file__).with_name('reference.json').write_text(json.dumps(facts,indent=2)+'\n',encoding='utf-8',newline='\n')
