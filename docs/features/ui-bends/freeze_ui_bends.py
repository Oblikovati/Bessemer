"""Observe both UI-bend helper layouts through actual independent readers (#54)."""
from functools import partial
from pathlib import Path
from typing import cast
import sys

WORKSPACE=Path(__file__).resolve().parents[3]
for folder in ('trim-edge-identifiers','base-flanges','edge-flanges','ui-bends'):
    sys.path.insert(0,str(WORKSPACE/'docs/features'/folder))
from freeze_trim_edges import ObjectFixtureFamily,REFERENCE,freeze
from freeze_edge_flanges import configure as configure_edges
from freeze_blank_part import source_module
from object_fixture_observer import ReferenceObjectCursor,object_cursor
from active_edge_fixtures import freeze_active
from ui_fixture_bytes import CLASSES,VERSIONS,ui_graph,active_ui_graph


def configure(cursor: ReferenceObjectCursor, reference: Path) -> None:
    """Bind actual complete line and feature serializers on a local cursor; e.g. configure(cursor,source)."""
    configure_edges(cursor,reference)
    module=source_module('uibends',reference)
    cursor.readers.update(zip(CLASSES,(module.read,module.ruled_line,module.sketch_line)))


def verify(reference: Path, raw: bytes, version: int, kind: int, style: int) -> None:
    """Require exact EOF and distinct/repeated roots; e.g. verify(source,raw,19000,0,5)."""
    cursor=object_cursor(reference,raw,version,partial(configure,reference=reference));root=cast(dict[str,object],cursor.object())
    if cursor.p!=len(raw):raise ValueError(f'UI-bend end {cursor.p}; expected {len(raw)}')
    if style not in (2,5):return
    items=root['items']
    if items[0] is items[1] or items[0] is not items[2]:raise ValueError('UI-bend identities; expected distinct second and repeated first roots')


def fixture_family(reference: Path) -> ObjectFixtureFamily:
    """Cross all three bindings with the strict late-word gate; e.g. fixture_family(source)."""
    return ObjectFixtureFamily(build=ui_graph,verify=verify,configure=partial(configure,reference=reference),
        magic=b'UIB1',kinds=3,versions=VERSIONS,edited_kinds=(0,1,2))


def freeze_active_ui(reference: Path, destination: Path) -> list[dict[str,object]]:
    """Use a fully decoded manager for every signed count and edit.
    Example: freeze_active_ui(source,output).
    """
    return freeze_active(reference,destination,kinds=tuple(range(6)),build=active_ui_graph)
