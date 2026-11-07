"""Freeze complete native-bound thin features against the approved source (#54)."""
from pathlib import Path
from typing import cast
import json
import struct
import hashlib
import sys

WORKSPACE=Path(__file__).resolve().parents[3]
for name in ('trim-edge-identifiers','reference-axes','reference-axis-owners','projected-line-subtypes','thin-extrusions'):
    sys.path.insert(0,str(WORKSPACE/'docs/features'/name))
from freeze_trim_edges import ObjectFixtureFamily, REFERENCE, freeze
from freeze_blank_part import source_module, reference_snapshot
from object_fixture_observer import object_cursor
from thin_feature_bytes import VERSIONS, thin_object


def verify_identity(reference: Path, raw: bytes, version: int, kind: int, style: int) -> None:
    """Require actual shared dimensions/specifications and repeated roots; e.g. verify_identity(source,raw,19000,1,2)."""
    root=cast(dict[str,object],object_cursor(reference,raw,version,None).object())
    if style not in (2,5):return
    first,second,repeated=cast(list[dict[str,object]],root['items'])
    if first is second or repeated is not first:raise ValueError('thin roots; expected distinct roots and repeated first')
    if kind==0:
        if style==2 and first['dimension_08'] is not first['dimension_10']:raise ValueError('thin dimensions; expected actual shared targets')
        return
    if thin_spec(first,kind) is not thin_spec(second,kind):raise ValueError('thin specification; expected actual shared target')


def thin_spec(value: dict[str,object], kind: int) -> object:
    """Select the serializer-specific native specification occurrence; e.g. thin_spec(root,4)."""
    if kind==4:return value['thin_specification_8e8']
    return cast(dict[str,object],value['payload'])['thin_specification_860']


def freeze_bend_boundary(reference: Path, destination: Path) -> dict[str,object]:
    """Verify the full u16 boundary using the real payload reader and a cached specification; e.g. freeze_bend_boundary(source,fixtures)."""
    before=reference_snapshot(reference)
    raw=struct.pack('<2H',1,65535)+struct.pack('<3I',0xffffffff,0x80000000,7)*65535
    archive=source_module('mfcobjects',reference).ObjectArchive(raw,19000)
    archive.feature_context='ordinary_external_xml';spec={'_class':'moThinSpec_c'};archive.map.append(spec)
    result=source_module('thinextrusions',reference).payload(archive)
    if result['thin_specification_860'] is not spec or archive.p!=len(raw):raise ValueError('thin boundary; expected cached spec and exact EOF')
    if result['bend_records_858']!=[[0xffffffff,0x80000000,7]]*65535:raise ValueError('thin boundary; expected 65,535 exact unsigned triples')
    if reference_snapshot(reference)!=before:raise ValueError('thin boundary source; expected stable source hashes')
    destination.mkdir(parents=True,exist_ok=True);(destination/'max-bends.bin').write_bytes(raw)
    return dict(scope='Selected payload boundary, not whole-document acceptance.',bytes=len(raw),count=65535,triple=[0xffffffff,0x80000000,7],sha256=hashlib.sha256(raw).hexdigest(),source_hashes=before)


FAMILY=ObjectFixtureFamily(build=thin_object,verify=verify_identity,magic=b'THN1',kinds=5,versions=VERSIONS,edited_kinds=tuple(range(5)))


if __name__=='__main__':
    facts=freeze(REFERENCE,WORKSPACE/'tests/fixtures/thin-extrusions',FAMILY)
    boundary=freeze_bend_boundary(REFERENCE,WORKSPACE/'tests/fixtures/thin-extrusions')
    Path(__file__).with_name('bend-boundary-reference.json').write_text(json.dumps(boundary,indent=2)+'\n',encoding='utf-8',newline='\n')
    Path(__file__).with_name('reference.json').write_text(json.dumps(facts,indent=2)+'\n',encoding='utf-8',newline='\n')
