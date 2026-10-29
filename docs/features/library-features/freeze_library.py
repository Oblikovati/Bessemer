"""Observe complete library-feature records with the actual independent readers (#54)."""
from pathlib import Path
from functools import partial
import json,sys
WORKSPACE=Path(__file__).resolve().parents[3]
for name in ('trim-edge-identifiers','reference-axes','reference-axis-owners','base-flanges','edge-flanges','library-features'):
    sys.path.insert(0,str(WORKSPACE/'docs/features'/name))
from freeze_trim_edges import ObjectFixtureFamily,REFERENCE,freeze
from freeze_edge_flanges import configure as configure_edges
from freeze_blank_part import source_module
from object_fixture_observer import ReferenceObjectCursor,object_cursor
from library_bytes import library_object,VERSIONS,CLASSES


def configure(cursor: ReferenceObjectCursor, reference: Path) -> None:
    """Install actual scoped readers and inherited dependencies locally."""
    configure_edges(cursor,reference)
    source_module('libraryfeatures',reference).install(cursor)


def verify(reference: Path, raw: bytes, version: int, kind: int, style: int) -> None:
    """Require exact EOF and the concrete class without hint-only stand-ins."""
    cursor=object_cursor(reference,raw,version,partial(configure,reference=reference));root=cursor.object()
    assert cursor.p==len(raw) and root['_class']==CLASSES[kind]



def fixture_family() -> ObjectFixtureFamily:
    """Cross inherited version gates, strict counts and independent scalar/body-removal goldens."""
    return ObjectFixtureFamily(build=library_object,verify=verify,configure=partial(configure,reference=REFERENCE),magic=b'LIB1',kinds=7,versions=VERSIONS,edited_kinds=(1,2,3,5,6))


if __name__=='__main__':
    facts=freeze(REFERENCE,WORKSPACE/'tests/fixtures/mate-planes',fixture_family())
    Path(__file__).with_name('reference.json').write_text(json.dumps(facts,indent=2)+'\n',encoding='utf-8',newline='\n')
    print(len(facts['cases']),facts['reads'],len(facts['owned']))
