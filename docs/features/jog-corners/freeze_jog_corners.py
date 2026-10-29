"""Observe stored jog options and direct/tagged break-corner framing (#54)."""
from functools import partial
from pathlib import Path
from typing import cast
import sys

WORKSPACE=Path(__file__).resolve().parents[3]
for folder in ('trim-edge-identifiers','base-flanges','edge-flanges','jog-corners'):
    sys.path.insert(0,str(WORKSPACE/'docs/features'/folder))
from freeze_trim_edges import ObjectFixtureFamily,REFERENCE,freeze
from freeze_edge_flanges import configure as configure_edges
from freeze_blank_part import source_module
from object_fixture_observer import ReferenceObjectCursor,object_cursor
from active_edge_fixtures import freeze_active
from jog_fixture_bytes import CLASSES,VERSIONS,OPTIONS,jog_graph


def configure(cursor: ReferenceObjectCursor, reference: Path) -> None:
    """Bind actual complete feature readers on a local cursor; e.g. configure(cursor,source)."""
    configure_edges(cursor,reference)
    module=source_module('jogs',reference)
    cursor.readers.update(zip(CLASSES,(module.read,module.break_corner)))


def verify(reference: Path, raw: bytes, version: int, kind: int, style: int) -> None:
    """Require exact EOF, distinct roots and retained raw jog options; e.g. verify(source,raw,19000,0,5)."""
    cursor=object_cursor(reference,raw,version,partial(configure,reference=reference));root=cast(dict[str,object],cursor.object())
    if cursor.p!=len(raw):raise ValueError(f'jog/corner end {cursor.p}; expected {len(raw)}')
    feature=root['items'][0] if style in (2,5) else root
    if kind==0 and feature['payload']['raw_8f4']!=OPTIONS[style]:raise ValueError(f'jog option {feature["payload"]["raw_8f4"]}; expected {OPTIONS[style]}')
    if style in (2,5) and (root['items'][0] is root['items'][1] or root['items'][0] is not root['items'][2]):
        raise ValueError('jog/corner identities; expected distinct second and repeated first roots')


def fixture_family(reference: Path) -> ObjectFixtureFamily:
    """Cross stored presence and both version gates with full inherited bodies; e.g. fixture_family(source)."""
    return ObjectFixtureFamily(build=jog_graph,verify=verify,configure=partial(configure,reference=reference),
        magic=b'JGC1',kinds=2,versions=VERSIONS,edited_kinds=(0,1))


def freeze_active_jogs(reference: Path, destination: Path) -> list[dict[str,object]]:
    """Select the inherited saved-state path through a complete manager.
    Example: freeze_active_jogs(source,output).
    """
    return freeze_active(reference,destination,kinds=(0,1),build=jog_graph)
