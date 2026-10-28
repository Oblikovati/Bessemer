"""Observe complete C-288 feature graphs through actual reference readers (#54)."""
from functools import partial
from pathlib import Path
from typing import cast
import sys

WORKSPACE=Path(__file__).resolve().parents[3]
for folder in ('trim-edge-identifiers','base-flanges','edge-flanges','fold-transforms','sheet-folds'):
    sys.path.insert(0,str(WORKSPACE/'docs/features'/folder))
from freeze_trim_edges import ObjectFixtureFamily,REFERENCE,freeze
from freeze_edge_flanges import configure as configure_edges
from freeze_blank_part import source_module
from object_fixture_observer import ReferenceObjectCursor,object_cursor
from sheet_fold_bytes import CLASSES,VERSIONS,sheet_fold_graph


def configure(cursor: ReferenceObjectCursor, reference: Path) -> None:
    """Bind actual complete readers on each independent cursor; e.g. configure(cursor,source)."""
    configure_edges(cursor,reference)
    module=source_module('sm3dbends',reference)
    cursor.readers.update(zip(CLASSES,(module.read,module.fold,module.unfold,module.object_int_map)))


def verify(reference: Path, raw: bytes, version: int, kind: int, style: int) -> None:
    """Require complete EOF and exact repeated-root identities; e.g. verify(source,raw,19000,2,5)."""
    cursor=object_cursor(reference,raw,version,partial(configure,reference=reference))
    root=cast(dict[str,object],cursor.object())
    if cursor.p!=len(raw):raise ValueError(f'fold end {cursor.p}; expected {len(raw)}')
    if style not in (2,5):return
    items=root['items']
    if items[0] is items[1] or items[0] is not items[2]:
        raise ValueError('fold identities; expected distinct second and repeated first roots')


def fixture_family(reference: Path) -> ObjectFixtureFamily:
    """Cover all four native bindings and both raw-body paths; e.g. fixture_family(source)."""
    return ObjectFixtureFamily(build=sheet_fold_graph,verify=verify,configure=partial(configure,reference=reference),
        magic=b'SFD1',kinds=4,versions=VERSIONS,edited_kinds=tuple(range(4)))
