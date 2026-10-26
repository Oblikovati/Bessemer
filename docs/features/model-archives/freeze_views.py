"""Complete ordinary view records with explicit transient-list reference scope."""
from pathlib import Path
import hashlib
import json
import struct
import sys
from freeze_blank_part import reference_snapshot
from freeze_node_units import object_tag, oracle
from owned_name_edit import short_wide_name
from oracle_projection import physical

WORKSPACE = Path(__file__).resolve().parents[3]
REFERENCE = WORKSPACE.parent/'inventor-interface/solidworks'
sys.path.insert(0,str(REFERENCE))
import modelprefix
import modelviews
from mfcobjects import ObjectArchive
from swcontainer import Document


def view_oracle(path: str, value: object) -> bytes:
    rows = oracle(path,physical(value)).splitlines(keepends=True)
    metadata = (b'.start',b'.end',b'.unknown_bytes')
    return b''.join(row for row in rows if not row.split(b'=',1)[0].endswith(metadata))


def transform(matrix: bool, flags: int) -> bytes:
    out = bytes([255 if matrix else 0])
    if matrix: out += struct.pack('<9Q',0,1<<63,0x7ff000000000002a,1,2,3,4,5,6)
    out += struct.pack('<4dB',-0.0,1.25,-2.5,3.75,flags)
    if flags & 0xf0 in (0x10,0x20): out += struct.pack('<4d',1,2,3,4)
    if flags & 0xf0 == 0x20: out += struct.pack('<2d',5,6)
    if flags & 0xf0 == 0x40: out += struct.pack('<i25d',-7,*[i/8 for i in range(25)])
    return out


def view(version: int, matrix: bool, flags: int) -> bytes:
    out = object_tag('moView_c')+short_wide_name('Ω view')+transform(matrix,flags)+transform(not matrix,flags)
    out += struct.pack('<HidiHHii',65530,-3,-0.0,2,0,0,-5,7)
    out += struct.pack('<iHIHH',-1,65535,2,0,2)
    out += struct.pack('<idiHH',-11,1.25,255,1,2)+struct.pack('<i',-13)
    if version >= 0x3764: out += struct.pack('<i',-17)
    return out


def freeze_case(version: int, matrix: bool, flags: int) -> tuple[bytes,int]:
    raw = view(version,matrix,flags)
    archive = ObjectArchive(raw,version)
    archive.feature_context = 'ordinary_external_xml'
    archive.readers = dict(archive.readers,moView_c=modelviews.view)
    value = archive.object()
    if archive.p != len(raw): raise ValueError(f'view end {archive.p}; expected {len(raw)}')
    expected = view_oracle('root.object',value)
    return struct.pack('<BIII',0,version,len(raw),len(expected))+raw+expected,len(expected.splitlines())


def native_group(destination: Path) -> dict[str,object]:
    root,archive = modelprefix.read(Document(WORKSPACE/'tests/fixtures/blank-part.SLDPRT',table={}),through_views=True)
    start = root['resolved']['views']['start']
    raw,expected = archive.b[start:archive.p],view_oracle('root',root['resolved']['views'])
    (destination/'native-views.bin').write_bytes(raw)
    (destination/'native-views.txt').write_bytes(expected)
    return {'start':start,'end':archive.p,'bytes':len(raw),'fields':len(expected.splitlines()),'sha256':hashlib.sha256(raw).hexdigest()}


def generate() -> None:
    before = reference_snapshot(REFERENCE)
    cases = [freeze_case(v,m,f) for v in [0x27cc,0x3763,0x3764,19000] for m in [False,True] for f in [7,0x1a,0x2b,0x4c,0x8d]]
    destination = WORKSPACE/'tests/fixtures/model-archives'
    native = native_group(destination)
    if before != reference_snapshot(REFERENCE): raise ValueError('reference sources changed; expected stable hashes')
    payload = b''.join(raw for raw,_ in cases)
    (destination/'view-cases.bin').write_bytes(payload)
    report = {'cases':len(cases),'fields':sum(count for _,count in cases),'sha256':hashlib.sha256(payload).hexdigest(),
              'native':native,'source_hashes':before,'generator_sha256':hashlib.sha256(Path(__file__).read_bytes()).hexdigest()}
    Path(__file__).with_name('view-reference.json').write_text(json.dumps(report,sort_keys=True,separators=(',',':'))+'\n')
    print({key:value for key,value in report.items() if key != 'source_hashes'})


if __name__ == '__main__':
    generate()
