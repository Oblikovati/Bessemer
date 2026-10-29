"""Observe complete 3D-view bodies through actual independent readers (#54)."""
from pathlib import Path
from functools import partial
import json,sys
WORKSPACE=Path(__file__).resolve().parents[3]
for name in ('trim-edge-identifiers','base-flanges','edge-flanges','stock-features','manager-scalars','three-dimensional-views'):
    sys.path.insert(0,str(WORKSPACE/'docs/features'/name))
from freeze_trim_edges import ObjectFixtureFamily,REFERENCE,bare_case
from freeze_blank_part import reference_snapshot
from owned_view_managers import owned
import struct,hashlib
from freeze_blank_part import source_module
from object_fixture_observer import ReferenceObjectCursor,object_cursor
from view_bytes import view_object,VERSIONS


def configure(cursor: ReferenceObjectCursor, reference: Path) -> None:
    """Install the actual modern derived-view reader locally."""
    cursor.readers=dict(cursor.readers,mo3dView_c=source_module('configuration3dviews',reference).view)


def verify(reference: Path, raw: bytes, version: int, kind: int, style: int) -> None:
    """Require exact EOF and cached references to the actual containing object."""
    cursor=object_cursor(reference,raw,version,partial(configure,reference=reference));root=cursor.object()
    assert cursor.p==len(raw) and root['_class']=='mo3dView_c'
    if style!=0:assert root['base']['list_390']['items'][1] is root


def fixture_family() -> ObjectFixtureFamily:
    """Cross the modern version boundary and all count/map styles."""
    return ObjectFixtureFamily(build=view_object,verify=verify,configure=partial(configure,reference=REFERENCE),magic=b'TDV1',kinds=1,versions=VERSIONS,edited_kinds=(0,))


def freeze(reference: Path, destination: Path, family: ObjectFixtureFamily) -> dict[str,object]:
    """Bind complete direct graphs and complete CMgr fixtures to unchanged Source."""
    before=reference_snapshot(reference);destination.mkdir(parents=True,exist_ok=True)
    cases=[bare_case(reference,0,v,s,family) for v in VERSIONS for s in range(6)]
    packet=b'TDV1'+struct.pack('<I',len(cases))+b''.join(struct.pack('<I',row['slots']) for _,row in cases)+b''.join(frame for frame,_ in cases)
    (destination/'cases.bin').write_bytes(packet)
    saved=[owned(reference,destination,s,False) for s in range(6)]+[owned(reference,destination,2,True)]
    assert before==reference_snapshot(reference)
    return dict(cases=[row for _,row in cases],owned=saved,source_hashes=before,packet_sha256=hashlib.sha256(packet).hexdigest(),reads=sum(row['reads'] for _,row in cases))


if __name__=='__main__':
    facts=freeze(REFERENCE,WORKSPACE/'tests/fixtures/three-dimensional-views',fixture_family())
    Path(__file__).with_name('reference.json').write_bytes((json.dumps(facts,indent=2)+'\n').encode())
