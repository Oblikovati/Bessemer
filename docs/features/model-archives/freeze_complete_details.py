"""Whole detail-definition bodies, with explicit inherited or fresh MFC maps."""
from pathlib import Path
import hashlib
import json
import struct
import sys
from freeze_blank_part import reference_snapshot
from freeze_node_units import object_tag, oracle
from freeze_detail_prefix import prefix
from freeze_dimension_defaults import group
from freeze_annotation_table_labels import annotation, table, labels
from freeze_detail_helpers import surface_finish, note
from freeze_weld_helpers import weld, gtol
from freeze_center_tail import center, tail
from oracle_projection import physical

WORKSPACE = Path(__file__).resolve().parents[3]
REFERENCE = WORKSPACE.parent/'inventor-interface/solidworks'
sys.path.insert(0,str(REFERENCE))
import modelprefix
import detaildefinitions
from mfcobjects import ObjectArchive
from swcontainer import Document


def complete(version: int, tagged: bool) -> bytes:
    body = prefix(version)+group(version)+annotation(version)+table(version)+labels(version)
    body += surface_finish(version)+note(version)+weld(version)+gtol(version,4 if tagged else 2)+center(version)+tail(version)
    return object_tag('moDetailDefs_c')+body if tagged else body


def freeze_case(version: int, tagged: bool) -> tuple[bytes,int]:
    raw = complete(version,tagged)
    archive = ObjectArchive(raw,version)
    archive.feature_context = 'ordinary_external_xml'
    archive.readers = dict(archive.readers,moDetailDefs_c=detaildefinitions.read)
    value = archive.object() if tagged else detaildefinitions.read(archive)
    if archive.p != len(raw): raise ValueError(f'detail body end {archive.p}; expected {len(raw)}')
    expected = oracle('root.object' if tagged else 'root',physical(value))
    return struct.pack('<BIII',int(tagged),version,len(raw),len(expected))+raw+expected,len(expected.splitlines())


def native_body(destination: Path) -> dict[str,object]:
    document = Document(WORKSPACE/'tests/fixtures/blank-part.SLDPRT',table={})
    root,archive = modelprefix.read(document,through_relation_manager=True)
    import modelheader
    modelheader.object_list(archive,1)
    start = archive.p
    value = detaildefinitions.read(archive)
    raw,expected = archive.b[start:archive.p],oracle('root',physical(value))
    (destination/'native-complete-details.bin').write_bytes(raw)
    (destination/'native-complete-details.txt').write_bytes(expected)
    return {'start':start,'end':archive.p,'bytes':len(raw),'fields':len(expected.splitlines()),'sha256':hashlib.sha256(raw).hexdigest()}


def generate() -> None:
    before = reference_snapshot(REFERENCE)
    frozen = [freeze_case(version,tagged) for version in [0x27cc,19000] for tagged in [False,True]]
    destination = WORKSPACE/'tests/fixtures/model-archives'
    native = native_body(destination)
    if before != reference_snapshot(REFERENCE): raise ValueError('reference sources changed; expected stable hashes')
    payload = b''.join(raw for raw,_ in frozen)
    (destination/'complete-detail-cases.bin').write_bytes(payload)
    report = {'cases':len(frozen),'fields':sum(count for _,count in frozen),'sha256':hashlib.sha256(payload).hexdigest(),
              'projection_sha256':hashlib.sha256(Path(__file__).with_name('oracle_projection.py').read_bytes()).hexdigest(),
              'native':native,'source_hashes':before,'generator_sha256':hashlib.sha256(Path(__file__).read_bytes()).hexdigest()}
    Path(__file__).with_name('complete-detail-reference.json').write_text(json.dumps(report,sort_keys=True,separators=(',',':'))+'\n')
    print({key:value for key,value in report.items() if key != 'source_hashes'})


if __name__ == '__main__':
    generate()
