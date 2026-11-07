"""Observe complete decal graphs and owned maps through actual immutable Source (#54)."""
from pathlib import Path
from functools import partial
import hashlib,struct,sys
WORKSPACE=Path(__file__).resolve().parents[3]
for name in ('trim-edge-identifiers','manager-scalars','photoworks-materials','configuration-decals'):
    sys.path.insert(0,str(WORKSPACE/'docs/features'/name))
from freeze_trim_edges import ObjectFixtureFamily,REFERENCE,bare_case
from freeze_blank_part import source_module,reference_snapshot
from object_fixture_observer import ReferenceObjectCursor,object_cursor
from primitive_inventory import observe_primitives,verify_primitive_coverage
from owned_circular_fixtures import write_observation
from decal_bytes import decal_object,decal_map,VERSIONS
from owned_decal_managers import owned_decal


def configure(cursor: ReferenceObjectCursor, reference: Path) -> None:
    """Select the actual complete readers verified by C-366 through C-368."""
    cursor.readers=dict(cursor.readers,moDecalProperties_c=source_module('configurationdecals',reference).properties)
    for flag in ('configuration_decal_faces','configuration_decal_references','configuration_decal_ids'):setattr(cursor,flag,True)


def verify(reference: Path, raw: bytes, version: int, kind: int, style: int) -> None:
    """Require exact EOF and preserve recursive decal-list identities."""
    cursor=object_cursor(reference,raw,version,partial(configure,reference=reference));root=cursor.object()
    assert cursor.p==len(raw) and root['_class']=='moDecalProperties_c'
    if style in (2,3,5):
        assert root['list_08']['items'][1] is root and root['list_20']['items'][0] is root['list_08']['items'][0]


def family() -> ObjectFixtureFamily:
    """Cross every scoped fresh dependency and all scalar/count styles."""
    return ObjectFixtureFamily(build=decal_object,verify=verify,configure=partial(configure,reference=REFERENCE),magic=b'DCL1',kinds=10,versions=VERSIONS,edited_kinds=tuple(range(10)))


def direct_map(reference: Path, destination: Path, kind: int, style: int) -> dict[str,object]:
    """Check both signed counts and nested association arrays without an outer header."""
    raw,slots=decal_map(kind,19000,style);cursor=object_cursor(reference,raw,19000,partial(configure,reference=reference))
    archive,reader=(source_module(name,reference) for name in ('swarchive','configurationdecals'))
    with observe_primitives(archive.Archive,raw) as reads:parsed=reader.owned_map(cursor)
    assert cursor.p==len(raw) and len(cursor.map)==slots;verify_primitive_coverage(raw,reads)
    if style in (2,5):assert parsed['objects'][0]['object'] is parsed['objects'][2]['object']
    return write_observation(destination,f'map-{kind}-{style}',raw,[r for r in reads if r.end>r.start])|dict(slots=slots)


def freeze(reference: Path, destination: Path) -> dict[str,object]:
    """Keep direct bodies, direct maps and complete CMgr contexts on one Source pin."""
    before=reference_snapshot(reference);destination.mkdir(parents=True,exist_ok=True);fixture_family=family()
    cases=[bare_case(reference,k,v,s,fixture_family) for k in range(10) for v in VERSIONS for s in range(6)]
    packet=b'DCL1'+struct.pack('<I',len(cases))+b''.join(struct.pack('<I',r['slots']) for _,r in cases)+b''.join(frame for frame,_ in cases)
    (destination/'cases.bin').write_bytes(packet);maps=[direct_map(reference,destination,k,s) for k in range(10) for s in range(6)]
    owned=[owned_decal(reference,destination,k,s,False) for k in range(10) for s in range(6)]
    owned+=[owned_decal(reference,destination,k,2,True) for k in range(10)];assert before==reference_snapshot(reference)
    return dict(cases=[r for _,r in cases],maps=maps,owned=owned,reads=sum(r['reads'] for _,r in cases),packet_sha256=hashlib.sha256(packet).hexdigest(),source_hashes=before)
