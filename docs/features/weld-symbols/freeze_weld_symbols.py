"""Independent complete weld-symbol observations with actual scoped readers (#54)."""
from pathlib import Path
from functools import partial
from typing import cast
import json
import sys
WORKSPACE=Path(__file__).resolve().parents[3]
for name in ('trim-edge-identifiers','component-detail-references','weld-symbols'):
    sys.path.insert(0,str(WORKSPACE/'docs/features'/name))
from freeze_trim_edges import ObjectFixtureFamily,REFERENCE,freeze
from freeze_detail_references import configure as configure_details
from freeze_blank_part import source_module
from object_fixture_observer import ReferenceObjectCursor,object_cursor
from weld_symbol_bytes import symbol_graph,VERSIONS


def configure(cursor: ReferenceObjectCursor, reference: Path) -> None:
    """Install actual complete symbol and direct-text dependencies locally; e.g. configure(cursor,source)."""
    configure_details(cursor,reference)
    source_module('sketchprefix',reference).install_scoped_readers(cursor)
    symbols=source_module('weldsymbols',reference)
    cursor.readers.update(moWeldSymbol_c=symbols.read,moCosmeticWeldBeadSymbol_c=symbols.cosmetic,
        moSFSymbol_c=symbols.surface_finish,moCosmeticWeldBeadAttachPoint_c=symbols.attachment)


def verify(reference: Path, raw: bytes, version: int, kind: int, style: int) -> None:
    """Require exact graph endpoints and genuine repeated roots; e.g. verify(source,raw,19000,2,5)."""
    cursor=object_cursor(reference,raw,version,partial(configure,reference=reference));root=cast(dict[str,object],cursor.object())
    if cursor.p!=len(raw):raise ValueError(f'weld symbol end {cursor.p}; expected {len(raw)}')
    if style not in (2,5):return
    items=cast(list[dict[str,object]],root['items']);first,second,repeated=items if style==2 else items[1:]
    if first is second or first is not repeated:raise ValueError(f'weld symbol roots {(first is second,first is repeated)}; expected (False,True)')


def fixture_family(reference: Path) -> ObjectFixtureFamily:
    """Cross constructor-bound classes and strict finish version gates; e.g. fixture_family(source)."""
    return ObjectFixtureFamily(build=symbol_graph,verify=verify,configure=partial(configure,reference=reference),
        magic=b'WSY1',kinds=4,versions=VERSIONS,edited_kinds=(0,1,2,3))


if __name__=='__main__':
    facts=freeze(REFERENCE,WORKSPACE/'tests/fixtures/weld-symbols',fixture_family(REFERENCE))
    Path(__file__).with_name('reference.json').write_text(json.dumps(facts,indent=2)+'\n',encoding='utf-8',newline='\n')
