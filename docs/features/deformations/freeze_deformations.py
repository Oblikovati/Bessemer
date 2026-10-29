"""Observe complete deform records and direct geometry through immutable Source (#54)."""
from pathlib import Path
from functools import partial
import json,struct,sys,hashlib
WORKSPACE=Path(__file__).resolve().parents[3]
for name in ('trim-edge-identifiers','base-flanges','edge-flanges','stock-features','drafts','fixed-valued-transforms','deformations'):
    sys.path.insert(0,str(WORKSPACE/'docs/features'/name))
from freeze_trim_edges import ObjectFixtureFamily,REFERENCE,freeze
from freeze_blank_part import source_module
from freeze_display_dimensions import install as install_display
from object_fixture_observer import ReferenceObjectCursor,object_cursor
from primitive_inventory import observe_primitives,verify_primitive_coverage
from owned_circular_fixtures import write_observation
from deform_bytes import deform_object,VERSIONS
from deform_geometry_bytes import geometry_bytes,GEOMETRY_VERSIONS


def configure(cursor: ReferenceObjectCursor, reference: Path) -> None:
    """Install actual complete deform and display readers locally."""
    install_display(cursor)
    source_module('deformations',reference).install(cursor)


def verify(reference: Path, raw: bytes, version: int, kind: int, style: int) -> None:
    """Require full inherited bodies, exact EOF and the documented root class."""
    cursor=object_cursor(reference,raw,version,partial(configure,reference=reference));root=cursor.object()
    assert cursor.p==len(raw) and root['_class']==('moCurvePointRef_w' if kind==15 else 'moDeform_c')


def fixture_family() -> ObjectFixtureFamily:
    """Cross every feature-version boundary and all six geometry/count styles."""
    return ObjectFixtureFamily(build=deform_object,verify=verify,configure=partial(configure,reference=REFERENCE),magic=b'DFM1',kinds=16,versions=VERSIONS,edited_kinds=(13,15))


def freeze_geometry(reference: Path, destination: Path) -> dict[str,object]:
    """Verify independent fixed widths and primitive partitions for every geometry gate."""
    cases=[];facts=[];archive=source_module('swarchive',reference);reader=source_module('deformations',reference)
    for version in GEOMETRY_VERSIONS:
        for style in range(6):
            raw=geometry_bytes(version,style);cursor=object_cursor(reference,raw,19000,None)
            with observe_primitives(archive.Archive,raw) as reads:reader.geometry(cursor)
            assert cursor.p==len(raw);verify_primitive_coverage(raw,reads)
            spans=[r for r in reads if r.end>r.start];oracle=''.join(f'{r.start}\t{r.end}\t{r.kind}\t{r.raw_hex}\n' for r in spans).encode()
            cases.append(struct.pack('<iBII',version,style,len(raw),len(oracle))+raw+oracle)
            facts.append(dict(version=version,style=style,bytes=len(raw),reads=len(spans),sha256=hashlib.sha256(raw).hexdigest()))
    packet=b'DFG1'+struct.pack('<I',len(cases))+b''.join(cases);(destination/'geometry.bin').write_bytes(packet)
    return dict(cases=facts,packet_sha256=hashlib.sha256(packet).hexdigest())


def freeze_matching(reference: Path, destination: Path) -> dict[str,object]:
    """Bind the 112-byte null-pointer subrecord and 68-byte second pass independently."""
    first=struct.pack('<i',1)+bytes(24)+struct.pack('<i',1)+bytes(112)
    raw=first+bytes(68);archive=source_module('swarchive',reference);reader=source_module('deformations',reference)
    cursor=object_cursor(reference,raw,19000,None)
    with observe_primitives(archive.Archive,raw) as reads:
        count=reader.positive_count(cursor,28,'matching record');matches=[reader.match_data(cursor) for _ in range(count)]
        assert cursor.p==144
        for record in matches:reader.match_data_second(cursor,record,10)
    assert cursor.p==len(raw) and len(cursor.map)==1;verify_primitive_coverage(raw,reads)
    return write_observation(destination,'matching',raw,[r for r in reads if r.end>r.start])|dict(first_pass_bytes=144,second_pass_bytes=68,slots=1)


def freeze_dataset(reference: Path, destination: Path) -> dict[str,object]:
    """Keep feature, geometry and wrapper-caller evidence in one stable Source scope."""
    from curve_point_callers import caller_family
    facts=freeze(reference,destination,fixture_family())
    facts['geometry']=freeze_geometry(reference,destination)
    facts['matching']=freeze_matching(reference,destination)
    facts['callers']=freeze(reference,destination/'callers',caller_family(reference))
    return facts


if __name__=='__main__':
    facts=freeze_dataset(REFERENCE,WORKSPACE/'tests/fixtures/deformations')
    Path(__file__).with_name('reference.json').write_bytes((json.dumps(facts,indent=2)+'\n').encode())
