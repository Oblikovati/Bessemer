"""Independent surface-finish and note helper fields, maps and object cycles."""
from pathlib import Path
import hashlib
import json
import struct
import sys
from freeze_blank_part import reference_snapshot
from freeze_node_units import object_tag, oracle
from freeze_detail_prefix import fields, character

WORKSPACE = Path(__file__).resolve().parents[3]
REFERENCE = WORKSPACE.parent/'inventor-interface/solidworks'
sys.path.insert(0,str(REFERENCE))
import modelprefix
import detaildatahelpers
from mfcobjects import ObjectArchive
from swcontainer import Document


def annotation(version: int) -> bytes:
    return fields([('i','68 6c 70 74'),('B','78')]) + character(version) + fields([
        ('B','7c'),('i','80'),('d','88'),('i','90'),('B','94'),('i','98'),('d','a0'),('i','a8')])


def surface_finish(version: int) -> bytes:
    out = fields([('i','08'),('s','10 20 18 28 30 38 40 48 50 58'),('B','68 6c 70 74'),
                  ('i','78'),('B','a0 a4 a8 ac'),('d','b0'),('B','b8')])
    out += character(version) + fields([('B','120 124')]) + annotation(version)
    if version > 0x42e3: out += fields([('i','7c'),('d','80')])
    if version > 0x46bf: out += fields([('s','88 90'),('i','98 9c')])
    return out


def helper_map(version: int, populated: bool) -> bytes:
    if not populated: return struct.pack('<Q',0)
    out = struct.pack('<QiiHii',4,-11,1,0,-11,2)
    out += object_tag('moSFDataHelper_c') + surface_finish(version)
    return out + struct.pack('<iiHii',7,4,2,9,99)


def note(version: int, populated: bool = False) -> bytes:
    out = fields([('s','08 18 20 28'),('B','10 30 34 38 3c 40'),('i','44'),('d','50'),
                  ('B','58'),('i','5c 60 78 7c 80 84 88 8c'),('d','90 98'),
                  ('B','a0 a4 a8 ac b0 b4 b8 bc c0 12c 130 134')])
    out += character(version) + fields([('i','240 244')])
    if version > 0x2f4a: out += helper_map(version,populated)
    out += fields([('B','138 13c 140')]) + annotation(version)
    out += fields([('B','48 128'),('i','14c'),('d','68'),('i','218 21c'),('s','220'),
                   ('i','230 234 238 23c'),('d','70 228'),('i','4c'),('B','144')])
    if version > 0x3373: out += fields([('B','148')])
    return out


def cases() -> list[tuple[int,int,bytes]]:
    sf_versions = [0x27cc,0x3eec,0x3eed,0x42e3,0x42e4,0x46bf,0x46c0,19000,0xffffffff]
    note_versions = [0x27cc,0x2f4a,0x2f4b,0x3373,0x3374,0x3eec,0x3eed,19000,0xffffffff]
    return [(0,v,surface_finish(v)) for v in sf_versions] + [(1,v,note(v)) for v in note_versions] + [
        (2,v,object_tag('moNoteDataHelper_c')+note(v,True)) for v in [0x2f4b,0x3eed,19000]]


def freeze_case(kind: int, version: int, raw: bytes) -> tuple[bytes,int]:
    archive = ObjectArchive(raw,version)
    archive.feature_context = 'ordinary_external_xml'
    archive.readers = dict(archive.readers,moSFDataHelper_c=detaildatahelpers.surface_finish,moNoteDataHelper_c=detaildatahelpers.note)
    value = detaildatahelpers.surface_finish(archive) if kind == 0 else detaildatahelpers.note(archive) if kind == 1 else archive.object()
    if archive.p != len(raw): raise ValueError(f'helper {kind} end {archive.p}; expected {len(raw)}')
    expected = oracle('root.object' if kind == 2 else 'root',value)
    return struct.pack('<BIII',kind,version,len(raw),len(expected)) + raw + expected,len(expected.splitlines())


def native_group(destination: Path) -> dict[str,object]:
    root,archive = modelprefix.read(Document(WORKSPACE/'tests/fixtures/blank-part.SLDPRT',table={}),through_detail_data_helpers=True)
    start = root['resolved']['detail_data_helpers_start']
    value = {'helpers':[item['data'] for item in root['resolved']['detail_data_helpers']['helpers']]}
    raw,expected = archive.b[start:archive.p],oracle('root',value)
    (destination/'native-detail-helpers.bin').write_bytes(raw)
    (destination/'native-detail-helpers.txt').write_bytes(expected)
    return {'start':start,'end':archive.p,'bytes':len(raw),'fields':len(expected.splitlines()),'sha256':hashlib.sha256(raw).hexdigest()}


def generate() -> None:
    before = reference_snapshot(REFERENCE)
    frozen = [freeze_case(*case) for case in cases()]
    destination = WORKSPACE/'tests/fixtures/model-archives'
    native = native_group(destination)
    if before != reference_snapshot(REFERENCE): raise ValueError('reference sources changed; expected stable hashes')
    payload = b''.join(raw for raw,_ in frozen)
    (destination/'detail-helper-cases.bin').write_bytes(payload)
    report = {'cases':len(frozen),'fields':sum(count for _,count in frozen),'sha256':hashlib.sha256(payload).hexdigest(),
              'native':native,'source_hashes':before,'generator_sha256':hashlib.sha256(Path(__file__).read_bytes()).hexdigest()}
    Path(__file__).with_name('detail-helper-reference.json').write_text(json.dumps(report,sort_keys=True,separators=(',',':'))+'\n')
    print({key:value for key,value in report.items() if key != 'source_hashes'})


if __name__ == '__main__':
    generate()
