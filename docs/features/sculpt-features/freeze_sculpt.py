"""Observe complete bodies with the actual independently maintained readers (#54)."""
from pathlib import Path
from functools import partial
import json,sys
WORKSPACE=Path(__file__).resolve().parents[3]
for name in ('trim-edge-identifiers','base-flanges','edge-flanges','stock-features','sculpt-features'):
    sys.path.insert(0,str(WORKSPACE/'docs/features'/name))
from freeze_trim_edges import ObjectFixtureFamily,REFERENCE,freeze
from freeze_blank_part import source_module
from freeze_display_dimensions import install as install_display
from object_fixture_observer import ReferenceObjectCursor,object_cursor
from sculpt_bytes import sculpt_object,VERSIONS,CLASSES


def configure(cursor: ReferenceObjectCursor, reference: Path) -> None:
    """Install the actual five class readers in this archive only."""
    install_display(cursor)
    source_module('volumefeatures',reference).install(cursor)
    cursor.readers['moDisplayScalarDim_c']=source_module('sketchgeometry',reference).scalar_display_dimension


def verify(reference: Path, raw: bytes, version: int, kind: int, style: int) -> None:
    """Require complete bodies, exact EOF and shared roots and reference targets."""
    cursor=object_cursor(reference,raw,version,partial(configure,reference=reference));root=cursor.object()
    assert cursor.p==len(raw)
    assert root['_class']==CLASSES[kind]



def fixture_family() -> ObjectFixtureFamily:
    """Cross all five complete class bindings and inherited version boundaries."""
    return ObjectFixtureFamily(build=sculpt_object,verify=verify,configure=partial(configure,reference=REFERENCE),magic=b'SCP1',kinds=5,versions=VERSIONS,edited_kinds=(0,1,2,3))


if __name__=='__main__':
    facts=freeze(REFERENCE,WORKSPACE/'tests/fixtures/body-extensions',fixture_family())
    Path(__file__).with_name('reference.json').write_text(json.dumps(facts,indent=2)+'\n',encoding='utf-8',newline='\n')
    print(len(facts['cases']),facts['reads'],len(facts['owned']))
