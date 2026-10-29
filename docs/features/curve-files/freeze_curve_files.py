"""Observe complete curve-file records through the actual independent reader (#54)."""
from pathlib import Path
from functools import partial
import json,sys
WORKSPACE=Path(__file__).resolve().parents[3]
for name in ('trim-edge-identifiers','reference-axes','reference-axis-owners','radiated-edge-identifiers','helices','curve-files'):
    sys.path.insert(0,str(WORKSPACE/'docs/features'/name))
from freeze_trim_edges import ObjectFixtureFamily,REFERENCE,freeze
from object_fixture_observer import ReferenceObjectCursor,object_cursor
from freeze_blank_part import source_module
from curve_file_bytes import curve_file_object,VERSIONS,COUNTS


def configure(cursor: ReferenceObjectCursor, reference: Path) -> None:
    """Install the actual resolved curve-file dependency on the local cursor."""
    cursor.readers['moCurveInFile_c']=source_module('curvefiles',reference).read


def verify(reference: Path, raw: bytes, version: int, kind: int, style: int) -> None:
    """Require exact endpoints, signed count retention and conditional pointer presence."""
    cursor=object_cursor(reference,raw,version,partial(configure,reference=reference));root=cursor.object()
    assert cursor.p==len(raw)
    assert root['payload']['declared_count']==COUNTS[style]
    assert len(root['payload']['points'])==max(0,COUNTS[style])
    assert root['payload']['reference_798_present']==(version>0x42cd)


def fixture_family() -> ObjectFixtureFamily:
    """Bind complete inherited curves and an independent first-point edit."""
    return ObjectFixtureFamily(build=curve_file_object,verify=verify,configure=partial(configure,reference=REFERENCE),magic=b'CRF1',kinds=1,versions=VERSIONS,edited_kinds=(0,))


if __name__=='__main__':
    facts=freeze(REFERENCE,WORKSPACE/'tests/fixtures/curve-files',fixture_family())
    Path(__file__).with_name('reference.json').write_text(json.dumps(facts,indent=2)+'\n',encoding='utf-8',newline='\n')
    print(len(facts['cases']),facts['reads'],len(facts['owned']))
