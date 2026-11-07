"""Observe complete independent pattern-extension fixtures (#54)."""
from pathlib import Path
from functools import partial
import json,sys
WORKSPACE=Path(__file__).resolve().parents[3]
for name in ('trim-edge-identifiers','base-flanges','edge-flanges','stock-features','plane-point-roles','reference-axes','structural-members','pattern-extensions'):
    sys.path.insert(0,str(WORKSPACE/'docs/features'/name))
from freeze_trim_edges import ObjectFixtureFamily,REFERENCE,freeze
from freeze_blank_part import source_module
from freeze_display_dimensions import install as install_display
from object_fixture_observer import ReferenceObjectCursor,object_cursor
from pattern_extension_bytes import pattern_object,VERSIONS,CLASSES,FLAGS


def configure(cursor: ReferenceObjectCursor, reference: Path) -> None:
    """Install actual Source readers only in the observed archive."""
    install_display(cursor)
    source_module('patternextensions',reference).install(cursor)
    cursor.readers['moProfileRef_w']=source_module('structuralmembers',reference).profile_reference


def verify(reference: Path, raw: bytes, version: int, kind: int, style: int) -> None:
    """Require EOF and independently verify the containing sketch node flags."""
    cursor=object_cursor(reference,raw,version,partial(configure,reference=reference));root=cursor.object()
    assert cursor.p==len(raw) and root['_class']==CLASSES[kind]
    if kind==1:assert root['pattern']['body']['model_feature']['feature']['node']['unknown_2c']==FLAGS[style]


def fixture_family() -> ObjectFixtureFamily:
    """Cover all six classes at each inherited boundary and signed-count style."""
    return ObjectFixtureFamily(build=pattern_object,verify=verify,configure=partial(configure,reference=REFERENCE),magic=b'PEX1',kinds=6,versions=VERSIONS,edited_kinds=tuple(range(6)))


if __name__=='__main__':
    facts=freeze(REFERENCE,WORKSPACE/'tests/fixtures/pattern-extensions',fixture_family())
    Path(__file__).with_name('reference.json').write_bytes((json.dumps(facts,indent=2)+'\n').encode())
    print(len(facts['cases']),facts['reads'],len(facts['owned']))
