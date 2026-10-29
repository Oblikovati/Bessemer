"""Observe complete smart-component graphs with actual independent Source readers (#54)."""
from pathlib import Path
from functools import partial
import json,sys
WORKSPACE=Path(__file__).resolve().parents[3]
for name in ('trim-edge-identifiers','base-flanges','edge-flanges','stock-features','smart-components'):
    sys.path.insert(0,str(WORKSPACE/'docs/features'/name))
from freeze_trim_edges import ObjectFixtureFamily,REFERENCE,freeze
from freeze_blank_part import source_module
from object_fixture_observer import ReferenceObjectCursor,object_cursor
from smart_bytes import smart_object,VERSIONS,CLASSES


def configure(cursor: ReferenceObjectCursor, reference: Path) -> None:
    """Install the actual scoped smart-component readers in this archive only."""
    source_module('smartcomponents',reference).install(cursor)


def verify(reference: Path, raw: bytes, version: int, kind: int, style: int) -> None:
    """Require complete bodies and exact EOF with no preloaded class-only objects."""
    cursor=object_cursor(reference,raw,version,partial(configure,reference=reference));root=cursor.object()
    assert cursor.p==len(raw) and root['_class']==CLASSES[kind]


def fixture_family() -> ObjectFixtureFamily:
    """Cross ten class bindings, nine inherited versions and six graph styles."""
    return ObjectFixtureFamily(build=smart_object,verify=verify,configure=partial(configure,reference=REFERENCE),magic=b'SMC1',kinds=10,versions=VERSIONS,edited_kinds=tuple(range(10)))


if __name__=='__main__':
    facts=freeze(REFERENCE,WORKSPACE/'tests/fixtures/smart-components',fixture_family())
    Path(__file__).with_name('reference.json').write_bytes((json.dumps(facts,indent=2)+'\n').encode())
