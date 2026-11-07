"""Observe containing-node selection and raw frames through actual readers (#54)."""
from functools import partial
from pathlib import Path
from typing import cast
import sys

WORKSPACE=Path(__file__).resolve().parents[3]
for folder in ('trim-edge-identifiers','base-flanges','edge-flanges','flatten-bends'):
    sys.path.insert(0,str(WORKSPACE/'docs/features'/folder))
from freeze_trim_edges import ObjectFixtureFamily,REFERENCE,freeze
from freeze_edge_flanges import configure as configure_edges
from freeze_blank_part import source_module
from object_fixture_observer import ReferenceObjectCursor,object_cursor
from active_edge_fixtures import freeze_active
from flatten_fixture_bytes import CLASSES,VERSIONS,FLAGS,flatten_graph


def configure(cursor: ReferenceObjectCursor, reference: Path) -> None:
    """Use actual complete feature and inherited dependency readers; e.g. configure(cursor,source)."""
    configure_edges(cursor,reference)
    module=source_module('flattenbends',reference)
    cursor.readers.update(zip(CLASSES,(module.read,module.process)))


def verify(reference: Path, raw: bytes, version: int, kind: int, style: int) -> None:
    """Check both EOF and the independently selected containing flags; e.g. verify(source,raw,19000,0,5)."""
    cursor=object_cursor(reference,raw,version,partial(configure,reference=reference));root=cast(dict[str,object],cursor.object())
    if cursor.p!=len(raw):raise ValueError(f'flatten end {cursor.p}; expected {len(raw)}')
    roots=root['items'][:2] if style in (2,5) else [root]
    for feature in roots:
        flags=feature['body']['model_feature']['feature']['node']['flags_28']
        if flags!=FLAGS[style]:raise ValueError(f'containing flags {flags}; expected {FLAGS[style]}')
    if style in (2,5) and root['items'][0] is not root['items'][2]:raise ValueError('flatten identities; expected repeated first root')


def fixture_family(reference: Path) -> ObjectFixtureFamily:
    """Cross both bindings with omitted, zero and present raw-body paths; e.g. fixture_family(source)."""
    return ObjectFixtureFamily(build=flatten_graph,verify=verify,configure=partial(configure,reference=reference),
        magic=b'FBN1',kinds=2,versions=VERSIONS,edited_kinds=(0,1))


def freeze_active_flatten(reference: Path, destination: Path) -> list[dict[str,object]]:
    """Keep saved +850 independent of ordinary archive helper +834.
    Example: freeze_active_flatten(source,output).
    """
    return freeze_active(reference,destination,kinds=(0,1),build=flatten_graph)
