"""Observe real selection-set and selection-item fields and identity (#54)."""
from pathlib import Path
from functools import partial
import json,sys
WORKSPACE=Path(__file__).resolve().parents[3]
for name in ('trim-edge-identifiers','feature-comments','selection-sets'):
    sys.path.insert(0,str(WORKSPACE/'docs/features'/name))
from freeze_trim_edges import ObjectFixtureFamily,REFERENCE,freeze
from object_fixture_observer import ReferenceObjectCursor,object_cursor
from freeze_blank_part import source_module
from selection_set_bytes import selection_object,VERSIONS


def configure(cursor: ReferenceObjectCursor, reference: Path) -> None:
    """Use the actual resolved-reader selection dependency on this local cursor."""
    cursor.readers['moSelectionItemData_c']=source_module('selectionitems',reference).read


def verify(reference: Path, raw: bytes, version: int, kind: int, style: int) -> None:
    """Require the containing set and repeated items to keep shared identity."""
    cursor=object_cursor(reference,raw,version,partial(configure,reference=reference));root=cursor.object()
    assert cursor.p==len(raw)
    if kind==0 and style not in (0,4):
        assert root['items'][0] is root['items'][1]
        assert root['items'][2] is root


def fixture_family() -> ObjectFixtureFamily:
    """Bind the complete node-based records; e.g. fixture_family()."""
    return ObjectFixtureFamily(build=selection_object,verify=verify,configure=partial(configure,reference=REFERENCE),magic=b'SEL1',kinds=2,versions=VERSIONS,edited_kinds=(0,))


if __name__=='__main__':
    facts=freeze(REFERENCE,WORKSPACE/'tests/fixtures/selection-sets',fixture_family())
    Path(__file__).with_name('reference.json').write_text(json.dumps(facts,indent=2)+'\n',encoding='utf-8',newline='\n')
    print(len(facts['cases']),facts['reads'],len(facts['owned']))
