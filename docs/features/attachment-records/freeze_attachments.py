"""Observe exact attachment layouts using real independent readers (#54)."""
from pathlib import Path
from functools import partial
import json
import sys
WORKSPACE=Path(__file__).resolve().parents[3]
for name in ('trim-edge-identifiers','component-detail-references','attachment-records'):
    sys.path.insert(0,str(WORKSPACE/'docs/features'/name))
from freeze_trim_edges import ObjectFixtureFamily,REFERENCE,freeze
from freeze_detail_references import configure as configure_details
from freeze_blank_part import source_module
from object_fixture_observer import ReferenceObjectCursor,object_cursor
from attachment_bytes import attachment_graph,VERSIONS


def configure(cursor: ReferenceObjectCursor, reference: Path) -> None:
    """Use the exact data and parametric readers in an explicit part context; e.g. configure(cursor,source)."""
    configure_details(cursor,reference)
    cursor.readers.update(moAttachPointData_c=source_module('displaynotes',reference).attachment_data,
        moParametricPoint_c=source_module('camerafeatures',reference).parametric_point)


def verify(reference: Path, raw: bytes, version: int, kind: int, style: int) -> None:
    """Require real reader EOF for every complete graph; e.g. verify(source,raw,19000,1,3)."""
    cursor=object_cursor(reference,raw,version,partial(configure,reference=reference));cursor.object()
    if cursor.p!=len(raw):raise ValueError(f'attachment graph end {cursor.p}; expected {len(raw)}')


def fixture_family(reference: Path) -> ObjectFixtureFamily:
    """Cross both distinct layouts with the drawing-only historical comparison; e.g. fixture_family(source)."""
    return ObjectFixtureFamily(build=attachment_graph,verify=verify,configure=partial(configure,reference=reference),
        magic=b'ATR1',kinds=2,versions=VERSIONS,edited_kinds=(0,1))


if __name__=='__main__':
    facts=freeze(REFERENCE,WORKSPACE/'tests/fixtures/attachment-records',fixture_family(REFERENCE))
    Path(__file__).with_name('reference.json').write_text(json.dumps(facts,indent=2)+'\n',encoding='utf-8',newline='\n')
