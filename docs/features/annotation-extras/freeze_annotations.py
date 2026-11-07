"""Observe all complete annotation bodies using actual immutable Source readers (#54)."""
from pathlib import Path
from functools import partial
import json,sys
WORKSPACE=Path(__file__).resolve().parents[3]
for name in ('trim-edge-identifiers','base-flanges','edge-flanges','stock-features','component-detail-references','display-notes','fixed-valued-transforms','annotation-extras'):
    sys.path.insert(0,str(WORKSPACE/'docs/features'/name))
from freeze_trim_edges import ObjectFixtureFamily,REFERENCE,freeze
from freeze_blank_part import source_module
from freeze_display_dimensions import install as install_display
from object_fixture_observer import ReferenceObjectCursor,object_cursor
from annotation_bytes import annotation_object,VERSIONS,CLASSES


def configure(cursor: ReferenceObjectCursor, reference: Path) -> None:
    """Install complete annotation, sketch and display readers locally."""
    install_display(cursor)
    source_module('annotationextras',reference).install(cursor)
    source_module('sketchprefix',reference).install_scoped_readers(cursor)
    cursor.readers['sgSketch']=source_module('assemblyprofiles',reference).sketch


def verify(reference: Path, raw: bytes, version: int, kind: int, style: int) -> None:
    """Require complete root framing and exact EOF, without preloaded objects."""
    cursor=object_cursor(reference,raw,version,partial(configure,reference=reference));root=cursor.object()
    assert cursor.p==len(raw) and root['_class']==CLASSES[kind]


def fixture_family() -> ObjectFixtureFamily:
    """Cross all ten bindings and eleven selected version boundaries."""
    return ObjectFixtureFamily(build=annotation_object,verify=verify,configure=partial(configure,reference=REFERENCE),magic=b'ANX1',kinds=10,versions=VERSIONS,edited_kinds=tuple(range(10)))


if __name__=='__main__':
    facts=freeze(REFERENCE,WORKSPACE/'tests/fixtures/annotation-extras',fixture_family())
    Path(__file__).with_name('reference.json').write_bytes((json.dumps(facts,indent=2)+'\n').encode())
