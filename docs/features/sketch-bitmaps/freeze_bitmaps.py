"""Observe complete bitmap records through independent source readers (#54)."""
from pathlib import Path
import json,sys
WORKSPACE=Path(__file__).resolve().parents[3]
for name in ('trim-edge-identifiers','sketch-bitmaps'):
    sys.path.insert(0,str(WORKSPACE/'docs/features'/name))
from freeze_trim_edges import ObjectFixtureFamily,REFERENCE,freeze
from object_fixture_observer import object_cursor
from bitmap_bytes import bitmap_object,VERSIONS,PLANE_PRESENCE


def verify(reference: Path, raw: bytes, version: int, kind: int, style: int) -> None:
    """Require complete byte-plane boundaries and signed transform presence."""
    cursor=object_cursor(reference,raw,version,None);root=cursor.object()
    assert cursor.p==len(raw)
    payload=root['data_360'] if kind==0 else root
    if PLANE_PRESENCE[style]:
        assert len(payload['byte_plane_40'])==payload['raw_48']*payload['raw_4c']


def fixture_family() -> ObjectFixtureFamily:
    """Keep both complete bitmap classes and independent word edits."""
    return ObjectFixtureFamily(build=bitmap_object,verify=verify,magic=b'BMP1',kinds=2,versions=VERSIONS,edited_kinds=(0,1))


if __name__=='__main__':
    facts=freeze(REFERENCE,WORKSPACE/'tests/fixtures/sketch-bitmaps',fixture_family())
    Path(__file__).with_name('reference.json').write_text(json.dumps(facts,indent=2)+'\n',encoding='utf-8',newline='\n')
    print(len(facts['cases']),facts['reads'],len(facts['owned']))
