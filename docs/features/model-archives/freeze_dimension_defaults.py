"""Independent direct dimension-default groups from specification 11.102."""
from pathlib import Path
import hashlib
import json
import struct
import sys
from freeze_blank_part import reference_snapshot
from freeze_node_units import base_wire, oracle
from freeze_detail_prefix import fields, character
from owned_name_edit import short_wide_name

WORKSPACE = Path(__file__).resolve().parents[3]
REFERENCE = WORKSPACE.parent / "inventor-interface/solidworks"
sys.path.insert(0, str(REFERENCE))
import modelprefix
import dimensiondefinitions
from mfcobjects import ObjectArchive
from swcontainer import Document

ROOT = [('i','68'),('d','08 10 18 20 28 30 38 40 48 50 58'),('i','60 64 6c 70 74'),
        ('d','78 80'),('i','88 8c'),('d','90 98 a0'),('I','d8'),('i','a8'),('d','b0'),('i','b8 bc c0 c4')]
BASE = [('i','08 0c 10'),('d','18 20'),('i','28'),('d','30 38'),('i','40 44'),('s','48 50'),
        ('d','58 60'),('i','68 6c 70 74 80 84 88 8c'),('H','90'),('i','94 98'),('I','e0')]
KINDS = ['linear','angle','arc_length','radial','hole_callout','ordinate','chamfer','diameter','angular_ordinate','base']
VERSIONS = [0x27cc,0x375d,0x375e,0x3eec,0x3eed,0x3eee,0x3eef,0x3ef8,0x3ef9,19000,0xffffffff]


def line_format() -> bytes:
    return short_wide_name('Dash Ω') + struct.pack('<hfh',-32760,1.25,-2)


def base(version: int) -> bytes:
    length = base_wire(version) + struct.pack('<HH',65526,0)
    angle = base_wire(version) + struct.pack('<H',65526)
    return fields(BASE) + length + angle + character(version) + line_format()*2 + fields([('i','c0 78 9c')])


def suffix(kind: str, version: int) -> bytes:
    if kind == 'linear':
        return fields([('I','108'),('i','100')]) + (fields([('i','104')]) if version > 0x3ef8 else b'')
    if kind in ('angle','hole_callout'): return fields([('I','100')])
    if kind == 'radial': return fields([('I','100'),('i','104')])
    if kind == 'diameter': return fields([('i','100'),('I','104')])
    if kind == 'ordinate': return fields([('i','100'),('I','104'),('d','108')])
    if kind == 'angular_ordinate': return fields([('i','100'),('I','104'),('i','108 110 114')])
    if kind == 'chamfer': return fields([('i','100 104 108 10c'),('d','118 120'),('i','110 128')])
    if kind not in ('arc_length','base'): raise ValueError(f'dimension kind {kind}; expected reviewed member kind')
    return b''


def group(version: int) -> bytes:
    out = fields(ROOT)
    if version > 0x375d: out += fields([('i','d0 d4')])
    if version >= 0x3eef: out += fields([('i','cc')])
    return out + b''.join(base(version)+suffix(kind,version) for kind in KINDS)


def physical(value: dict[str, object]) -> dict[str, object]:
    result = dict(value)
    result['definitions'] = [{key: child for key,child in item.items()
                              if key not in ('kind','member','start','end','unknown_bytes')}
                             for item in value['definitions']]
    return result


def freeze_case(version: int) -> tuple[bytes, int]:
    raw = group(version)
    archive = ObjectArchive(raw,version)
    archive.feature_context = 'ordinary_external_xml'
    expected = oracle('root',physical(dimensiondefinitions.read(archive)))
    if archive.p != len(raw): raise ValueError(f'dimension end {archive.p}; expected {len(raw)}')
    return struct.pack('<BIII',0,version,len(raw),len(expected)) + raw + expected, len(expected.splitlines())


def native_group(destination: Path) -> dict[str, object]:
    root, archive = modelprefix.read(Document(WORKSPACE/'tests/fixtures/blank-part.SLDPRT',table={}),through_dimension_definitions=True)
    start = root['resolved']['dimension_definitions_start']
    raw = archive.b[start:archive.p]
    expected = oracle('root',physical(root['resolved']['dimension_definitions']))
    (destination/'native-dimension-defaults.bin').write_bytes(raw)
    (destination/'native-dimension-defaults.txt').write_bytes(expected)
    return {'start':start,'end':archive.p,'bytes':len(raw),'fields':len(expected.splitlines()),'sha256':hashlib.sha256(raw).hexdigest()}


def generate() -> None:
    before = reference_snapshot(REFERENCE)
    cases = [freeze_case(version) for version in VERSIONS]
    destination = WORKSPACE/'tests/fixtures/model-archives'
    native = native_group(destination)
    if before != reference_snapshot(REFERENCE): raise ValueError('reference sources changed; expected stable hashes')
    payload = b''.join(raw for raw,_ in cases)
    (destination/'dimension-default-cases.bin').write_bytes(payload)
    report = {'cases':len(cases),'fields':sum(count for _,count in cases),'sha256':hashlib.sha256(payload).hexdigest(),
              'native':native,'source_hashes':before,'generator_sha256':hashlib.sha256(Path(__file__).read_bytes()).hexdigest()}
    Path(__file__).with_name('dimension-default-reference.json').write_text(json.dumps(report,sort_keys=True,separators=(',',':'))+'\n')
    print({key:value for key,value in report.items() if key != 'source_hashes'})


if __name__ == '__main__':
    generate()
