"""Observe complete manager scalar bodies using the actual independent readers (#54)."""
from pathlib import Path
from functools import partial
import json,sys,struct,hashlib
WORKSPACE=Path(__file__).resolve().parents[3]
for name in ('trim-edge-identifiers','manager-scalars'):
    sys.path.insert(0,str(WORKSPACE/'docs/features'/name))
from freeze_trim_edges import ObjectFixtureFamily,REFERENCE,bare_case
from freeze_blank_part import source_module,reference_snapshot
from object_fixture_observer import ReferenceObjectCursor,object_cursor
from scalar_bytes import scalar_object,VERSIONS,CLASSES
from owned_scalar_managers import owned_manager


def configure(cursor: ReferenceObjectCursor, reference: Path) -> None:
    """Bind complete actual scalar readers only in this fixture's ordinary archive."""
    manager=source_module('configurationmanager',reference)
    cursor.readers.update(zip(CLASSES,(manager.sheet_metal_pair,manager.material_property)))


def verify(reference: Path, raw: bytes, version: int, kind: int, style: int) -> None:
    """Require exact EOF and distinct values with actual shared identities."""
    cursor=object_cursor(reference,raw,version,partial(configure,reference=reference));root=cursor.object()
    assert cursor.p==len(raw)
    if style not in (2,5):return
    first,second,repeated=root['items']
    assert first is repeated and first is not second


def fixture_family() -> ObjectFixtureFamily:
    """Cross both complete bodies and six version controls with independent word edits."""
    return ObjectFixtureFamily(build=scalar_object,verify=verify,configure=partial(configure,reference=REFERENCE),magic=b'MSC1',kinds=2,versions=VERSIONS,edited_kinds=(0,1))


def freeze(reference: Path, destination: Path) -> dict[str,object]:
    """Keep bare bodies separate from complete owned manager-map compositions."""
    before=reference_snapshot(reference);destination.mkdir(parents=True,exist_ok=True);family=fixture_family()
    cases=[bare_case(reference,k,v,s,family) for k in range(2) for v in VERSIONS for s in range(6)]
    packet=b'MSC1'+struct.pack('<I',len(cases))+b''.join(struct.pack('<I',r['slots']) for _,r in cases)+b''.join(raw for raw,_ in cases)
    (destination/'cases.bin').write_bytes(packet)
    owned=[owned_manager(reference,destination,k,s,False) for k in range(2) for s in range(6)]
    owned+=[owned_manager(reference,destination,k,2,True) for k in range(2)]
    assert reference_snapshot(reference)==before
    return dict(cases=[r for _,r in cases],owned=owned,source_hashes=before,packet_sha256=hashlib.sha256(packet).hexdigest(),reads=sum(r['reads'] for _,r in cases))


if __name__=='__main__':
    facts=freeze(REFERENCE,WORKSPACE/'tests/fixtures/manager-scalars')
    Path(__file__).with_name('reference.json').write_text(json.dumps(facts,indent=2)+'\n',encoding='utf-8',newline='\n')
    print(len(facts['cases']),facts['reads'],len(facts['owned']))
