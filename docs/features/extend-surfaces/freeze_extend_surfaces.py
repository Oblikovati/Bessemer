"""Observe complete extended-surface records using the actual independent reader (#54)."""
from pathlib import Path
import json,sys
WORKSPACE=Path(__file__).resolve().parents[3]
for name in ('trim-edge-identifiers','reference-axes','reference-axis-owners','base-flanges','edge-flanges','revolved-surfaces','extend-surfaces'):
    sys.path.insert(0,str(WORKSPACE/'docs/features'/name))
from freeze_trim_edges import ObjectFixtureFamily,REFERENCE,freeze
from object_fixture_observer import object_cursor
from extend_surface_bytes import extend_surface_object,VERSIONS,MODES


def verify(reference: Path, raw: bytes, version: int, kind: int, style: int) -> None:
    """Require complete boundaries and exactly one mode-selected pointer occurrence."""
    cursor=object_cursor(reference,raw,version,None);root=cursor.object()
    assert cursor.p==len(raw)
    payload=root.get('extend_data_7b0') if kind==0 else root if kind==1 else None
    if payload is None:return
    expected={0:'distance_28',1:'reference_20',2:'reference_18'}.get(MODES[style])
    actual=[key for key in payload if key.startswith(('distance_','reference_'))]
    assert actual==([expected] if expected else [])


def fixture_family() -> ObjectFixtureFamily:
    """Bind complete inherited surfaces, data and empty native keepers."""
    return ObjectFixtureFamily(build=extend_surface_object,verify=verify,magic=b'EXS1',kinds=3,versions=VERSIONS,edited_kinds=(0,1))


if __name__=='__main__':
    facts=freeze(REFERENCE,WORKSPACE/'tests/fixtures/extend-surfaces',fixture_family())
    Path(__file__).with_name('reference.json').write_text(json.dumps(facts,indent=2)+'\n',encoding='utf-8',newline='\n')
    print(len(facts['cases']),facts['reads'],len(facts['owned']))
