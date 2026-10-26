"""Center/datum helpers, native SWIFT field order and detail-tail wire cases."""
from pathlib import Path
import hashlib
import json
import struct
import sys
from freeze_blank_part import reference_snapshot
from freeze_node_units import oracle
from freeze_detail_prefix import fields, character
from freeze_detail_helpers import annotation

WORKSPACE = Path(__file__).resolve().parents[3]
REFERENCE = WORKSPACE.parent/'inventor-interface/solidworks'
sys.path.insert(0,str(REFERENCE))
import modelprefix
import centerhelpers
import detaildefinitions
from mfcobjects import ObjectArchive
from swcontainer import Document


def center_mark(version: int) -> bytes:
    return fields([('B','08'),('d','10'),('B','18'),('d','20'),('B','28 30 34 38 3c 2c')])+annotation(version)+fields([('I','148'),('B','40 44 48 4c'),('d','78')])


def datum_feature(version: int) -> bytes:
    return fields([('B','08 0c 10'),('i','20 24'),('s','18 28'),('i','30 34 38')])+annotation(version)


def datum_target(version: int) -> bytes:
    out = fields([('s','18 20 28 08 10'),('i','30 40'),('d','50 58'),('i','44'),('B','34 3c 38 48 4c'),('i','60'),('d','68'),('i','70'),('B','74 90 b0')])
    return out+struct.pack('<3Q',0x8000000000000000,0x7ff000000000002a,1)+annotation(version)


def swift(version: int) -> bytes:
    out = b''
    for index,(kind,member) in enumerate(centerhelpers.SWIFT_LAYOUT):
        if (member == 'c' and version <= 0x3739) or (member == '3f8' and version < 0x3aff) or (member == '418' and version <= 0x46b5): continue
        out += struct.pack('<d',-index/16) if kind == 'f64' else struct.pack('<i',-index-7)
    return out


def center(version: int) -> bytes:
    return center_mark(version)+datum_feature(version)+datum_target(version)+swift(version)


def tail(version: int) -> bytes:
    out = struct.pack('<8I',0,0x80000000,0x3fa00000,1,0x7f7fffff,0xff800000,0x3f800000,0x40000000)
    out += fields([('d','2bd0'),('i','2bf4 2bf8'),('B','2bfc 2c00'),('i','2c0c 2c10 50 2c04 2c08 80'),
                   ('d','88'),('d','2c18 2c20 2c28 2c30 2c38'),('i','2c40 2c44 2c48 2c4c 2c50 2c54 2c58 2c5c 98')])
    out += character(version)*2+fields([('i','2c64 2c68 19c 1bc'),('s','1c0 1c8 1d0 1d8'),('i','1e0 1e4 1e8 1f8'),('s','1f0')])
    if version > 0x27d1: out += fields([('i','1fc')])
    if version > 0x2bb5: out += fields([('i','15c 160')])
    if version > 0x335e: out += fields([('i','218'),('s','200 208 210')])
    if version > 0x373f: out += fields([('i','220 224')])
    if version >= 0x3b15: out += fields([('i','2c6c')])
    if version > 0x41a1: out += fields([('i','21c')])
    if version > 0x42e5: out += fields([('i','1b8')])
    return out


def physical(value: object) -> object:
    if isinstance(value,dict): return {key:physical(child) for key,child in value.items() if key not in ('member','kind','start','end','unknown_bytes','type')}
    if isinstance(value,list): return [physical(child) for child in value]
    return value


def freeze_case(kind: int, version: int) -> tuple[bytes,int]:
    raw = center(version) if kind == 0 else tail(version)
    archive = ObjectArchive(raw,version)
    archive.feature_context = 'ordinary_external_xml'
    value = centerhelpers.read(archive) if kind == 0 else detaildefinitions.tail(archive)
    if kind == 0: value = {'helpers':[item['data'] for item in value['helpers']]}
    expected = oracle('root',physical(value))
    if archive.p != len(raw): raise ValueError(f'group {kind} end {archive.p}; expected {len(raw)}')
    return struct.pack('<BIII',kind,version,len(raw),len(expected))+raw+expected,len(expected.splitlines())


def native_groups(destination: Path) -> list[dict[str,object]]:
    root,archive = modelprefix.read(Document(WORKSPACE/'tests/fixtures/blank-part.SLDPRT',table={}),through_complete_detail_definitions=True)
    records = [('center',root['resolved']['center_helpers_start'],root['resolved']['detail_tail_start'],
                {'helpers':[item['data'] for item in root['resolved']['center_helpers']['helpers']]}),
               ('detail-tail',root['resolved']['detail_tail_start'],archive.p,root['resolved']['detail_tail'])]
    result = []
    for name,start,end,value in records:
        raw,expected = archive.b[start:end],oracle('root',physical(value))
        (destination/f'native-{name}.bin').write_bytes(raw)
        (destination/f'native-{name}.txt').write_bytes(expected)
        result.append({'group':name,'start':start,'end':end,'bytes':len(raw),'fields':len(expected.splitlines()),'sha256':hashlib.sha256(raw).hexdigest()})
    return result


def generate() -> None:
    before = reference_snapshot(REFERENCE)
    center_versions = [0x27cc,0x3739,0x373a,0x3afe,0x3aff,0x3eec,0x3eed,0x46b5,0x46b6,19000,0xffffffff]
    tail_versions = [0x27cc,0x27d1,0x27d2,0x2bb5,0x2bb6,0x335e,0x335f,0x373f,0x3740,0x3b14,0x3b15,0x3eec,0x3eed,0x41a1,0x41a2,0x42e5,0x42e6,19000,0xffffffff]
    cases = [freeze_case(kind,v) for kind,versions in enumerate([center_versions,tail_versions]) for v in versions]
    destination = WORKSPACE/'tests/fixtures/model-archives'
    native = native_groups(destination)
    if before != reference_snapshot(REFERENCE): raise ValueError('reference sources changed; expected stable hashes')
    payload = b''.join(raw for raw,_ in cases)
    (destination/'center-tail-cases.bin').write_bytes(payload)
    report = {'cases':len(cases),'fields':sum(count for _,count in cases),'sha256':hashlib.sha256(payload).hexdigest(),
              'native':native,'swift_case_order':'reference native load/store layout; ordinal-distinct values; not an independent field-order oracle',
              'source_hashes':before,'generator_sha256':hashlib.sha256(Path(__file__).read_bytes()).hexdigest()}
    Path(__file__).with_name('center-tail-reference.json').write_text(json.dumps(report,sort_keys=True,separators=(',',':'))+'\n')
    print({key:value for key,value in report.items() if key != 'source_hashes'})


if __name__ == '__main__':
    generate()
