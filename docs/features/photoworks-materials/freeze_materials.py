"""Observe complete material properties and direct managers with immutable Source (#54)."""
from pathlib import Path
from functools import partial
import hashlib,struct,sys
WORKSPACE=Path(__file__).resolve().parents[3]
for name in ('trim-edge-identifiers','manager-scalars','photoworks-materials'):
    sys.path.insert(0,str(WORKSPACE/'docs/features'/name))
from freeze_trim_edges import ObjectFixtureFamily,REFERENCE,bare_case
from freeze_blank_part import source_module,reference_snapshot
from object_fixture_observer import ReferenceObjectCursor,object_cursor
from primitive_inventory import observe_primitives,verify_primitive_coverage
from owned_circular_fixtures import write_observation
from material_bytes import material_object,manager_bytes,VERSIONS
from owned_material_managers import owned_material


def configure(cursor: ReferenceObjectCursor, reference: Path) -> None:
    """Bind the complete actual Source property reader locally."""
    cursor.readers=dict(cursor.readers,moPWMaterialProperties_c=source_module('configurationpwmaterials',reference).properties)


def verify(reference: Path, raw: bytes, version: int, kind: int, style: int) -> None:
    """Require complete properties and preserve recursive material-list references."""
    cursor=object_cursor(reference,raw,version,partial(configure,reference=reference));root=cursor.object()
    assert cursor.p==len(raw) and root['_class']=='moPWMaterialProperties_c'
    if style in (3,5):assert root['list_08']['items'][1] is root
    if style==2:assert root['list_08']['items'][0] is root['list_20']['items'][0]


def family() -> ObjectFixtureFamily:
    """Cross modern archive versions and all six scalar/list styles."""
    return ObjectFixtureFamily(build=material_object,verify=verify,configure=partial(configure,reference=REFERENCE),magic=b'PWM1',kinds=1,versions=VERSIONS,edited_kinds=(0,))


def direct_manager(reference: Path, destination: Path, style: int) -> dict[str,object]:
    """Check exact direct-manager EOF without adding an outer object slot."""
    raw,slots=manager_bytes(style);cursor=object_cursor(reference,raw,19000,None)
    archive,reader=(source_module(name,reference) for name in ('swarchive','configurationpwmaterials'))
    with observe_primitives(archive.Archive,raw) as reads:parsed=reader.read(cursor)
    assert cursor.p==len(raw) and len(cursor.map)==slots;verify_primitive_coverage(raw,reads)
    if len(parsed['materials'])==3:assert parsed['materials'][0] is parsed['materials'][2] and parsed['materials'][0] is not parsed['materials'][1]
    return write_observation(destination,f'manager-{style}',raw,[r for r in reads if r.end>r.start])|dict(slots=slots)


def freeze(reference: Path, destination: Path) -> dict[str,object]:
    """Observe every primitive through independent complete readers."""
    before=reference_snapshot(reference);destination.mkdir(parents=True,exist_ok=True);fixture_family=family()
    cases=[bare_case(reference,0,v,s,fixture_family) for v in VERSIONS for s in range(6)]
    packet=b'PWM1'+struct.pack('<I',len(cases))+b''.join(struct.pack('<I',r['slots']) for _,r in cases)+b''.join(frame for frame,_ in cases)
    (destination/'cases.bin').write_bytes(packet);managers=[direct_manager(reference,destination,s) for s in range(6)]
    owned=[owned_material(reference,destination,s,False) for s in range(6)]+[owned_material(reference,destination,2,True)]
    assert before==reference_snapshot(reference)
    return dict(owned=owned,cases=[r for _,r in cases],managers=managers,reads=sum(r['reads'] for _,r in cases),packet_sha256=hashlib.sha256(packet).hexdigest(),source_hashes=before)
