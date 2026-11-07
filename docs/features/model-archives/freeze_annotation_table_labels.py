"""Independent direct annotation, table and view-label default groups."""
from pathlib import Path
import hashlib
import json
import struct
import sys
from freeze_blank_part import reference_snapshot
from freeze_node_units import oracle
from freeze_detail_prefix import fields, character

WORKSPACE = Path(__file__).resolve().parents[3]
REFERENCE = WORKSPACE.parent / 'inventor-interface/solidworks'
sys.path.insert(0,str(REFERENCE))
import modelprefix
import annotationdefinitions
import tabledefinitions
import labeldefinitions
from mfcobjects import ObjectArchive
from swcontainer import Document

READERS = [annotationdefinitions.read, tabledefinitions.read, labeldefinitions.read]
VERSIONS = [[0x27cc,0x2f6f,0x2f70,0x3737,0x3738,0x3769,0x376a,0x3b04,0x3b05,0x3eec,0x3eed,0x3ef3,0x3ef4,19000,0xffffffff],
            [0x27cc,0x2b81,0x2b82,0x375f,0x3760,0x3eec,0x3eed,0x42f1,0x42f2,19000,0xffffffff],
            [0x27cc,0x3b02,0x3b03,0x3eec,0x3eed,19000,0xffffffff]]


def widths() -> bytes:
    return struct.pack('<hf',-32760,1.25)*2


def annotation_base(version: int) -> bytes:
    return fields([('i','08'),('d','10'),('i','18 1c 20 24 34 40'),('H','44')]) + widths() + character(version) + fields([('i','28')])


def annotation_suffixes(version: int) -> list[bytes]:
    gtol = (fields([('i','58')]) if version >= 0x3738 else b'')
    if version >= 0x3b05: gtol += fields([('i','5c')])
    if version > 0x3ef3: gtol += fields([('H','60')])
    return [fields([('i','58 5c 60 64 68 6c'),('s','70 78'),('I','80'),('d','90 88'),('i','b4 b8'),('s','c0'),('d','98 a0 a8'),('i','b0')]),
            fields([('i','58 5c 60'),('d','68 70 78'),('i','80')]),fields([('i','58 68 6c 60 64 5c')]),gtol,
            fields([('i','58 5c'),('d','60 68')]),fields([('i','58')])+character(version),
            fields([('i','58 5c 60'),('d','68')]),b'',fields([('d','58')]),
            fields([('i','58 5c 60 64'),('d','68'),('i','70'),('d','78'),('i','74')])]


def annotation(version: int) -> bytes:
    out = fields([('i','10'),('d','08'),('i','14 1c 20 24 28'),('I','38'),('i','2c 30')])
    if version >= 0x2f70: out += fields([('i','18')])
    if version >= 0x376a: out += fields([('i','34')])
    return out + b''.join(annotation_base(version)+suffix for suffix in annotation_suffixes(version))


def table_base(version: int) -> bytes:
    return fields([('i','2c 28'),('I','08'),('H','0c'),('i','38')]) + widths() + character(version)


def table(version: int) -> bytes:
    bom = fields([('I','40')]) + (fields([('s','48 50 58')]) if version > 0x2b81 else b'')
    suffixes = [b'',fields([('i','40 44 48'),('I','58'),('i','4c'),('s','50')]),bom,
                fields([('i','40 44'),('d','48 50'),('i','58 5c 60 68 6c 70 74 78 64 7c')]),b'',
                fields([('i','40')]) if version > 0x375f else b'',fields([('i','40 44 4c 48')]),
                fields([('i','40 44'),('d','58 60'),('i','48 4c 6c 50 70 68')])]
    if version > 0x42f1: suffixes.append(fields([('I','40')]))
    definitions = [table_base(version)+suffix for suffix in suffixes]
    return fields([('i','08')]) + b''.join(definitions[:4]) + character(version) + b''.join(definitions[4:]) + fields([('d','330 338')])


def label_base(version: int) -> bytes:
    prefix = fields([('i','08 0c 18 14 1c 20 24 28 44 2c 38'),('H','48'),('s','88 90'),('I','5c'),('i','10 58')])
    return prefix + character(version)*2 + widths() + character(version)*4


def labels(version: int) -> bytes:
    detail = fields([('I','cc'),('i','c8')]) if version > 0x3b02 else b''
    suffixes = [detail,fields([('d','c8 d0 d8'),('i','e0 e4 ec f0 e8')]),fields([('i','c8')]),
                fields([('i','c8 cc')]),fields([('i','c8 cc')])]
    definitions = [label_base(version)+suffix for suffix in suffixes]
    return fields([('i','08 0c')]) + b''.join(definitions[:3]) + character(version) + b''.join(definitions[3:])


def physical(value: object) -> object:
    if isinstance(value,dict):
        if set(value) == {'member','data'}: return physical(value['data'])
        return {key:physical(child) for key,child in value.items()
                if key not in ('kind','member','start','end','unknown_bytes','character_format_start')}
    if isinstance(value,list): return [physical(child) for child in value]
    return value


def freeze_case(kind: int, version: int) -> tuple[bytes,int]:
    raw = [annotation,table,labels][kind](version)
    archive = ObjectArchive(raw,version)
    archive.feature_context = 'ordinary_external_xml'
    expected = oracle('root',physical(READERS[kind](archive)))
    if archive.p != len(raw): raise ValueError(f'group {kind} end {archive.p}; expected {len(raw)}')
    return struct.pack('<BIII',kind,version,len(raw),len(expected)) + raw + expected,len(expected.splitlines())


def native_groups(destination: Path) -> list[dict[str,object]]:
    document = Document(WORKSPACE/'tests/fixtures/blank-part.SLDPRT',table={})
    groups = []
    for name,flag in [('annotation','through_annotation_definitions'),('table','through_table_definitions'),('label','through_label_definitions')]:
        root,archive = modelprefix.read(document,**{flag:True})
        start = root['resolved'][name+'_definitions_start']
        raw = archive.b[start:archive.p]
        expected = oracle('root',physical(root['resolved'][name+'_definitions']))
        (destination/f'native-{name}-defaults.bin').write_bytes(raw)
        (destination/f'native-{name}-defaults.txt').write_bytes(expected)
        groups.append({'group':name,'start':start,'end':archive.p,'bytes':len(raw),'fields':len(expected.splitlines()),'sha256':hashlib.sha256(raw).hexdigest()})
    return groups


def generate() -> None:
    before = reference_snapshot(REFERENCE)
    cases = [freeze_case(kind,version) for kind,versions in enumerate(VERSIONS) for version in versions]
    destination = WORKSPACE/'tests/fixtures/model-archives'
    native = native_groups(destination)
    if before != reference_snapshot(REFERENCE): raise ValueError('reference sources changed; expected stable hashes')
    payload = b''.join(raw for raw,_ in cases)
    (destination/'annotation-table-label-cases.bin').write_bytes(payload)
    report = {'cases':len(cases),'fields':sum(count for _,count in cases),'sha256':hashlib.sha256(payload).hexdigest(),
              'native':native,'source_hashes':before,'generator_sha256':hashlib.sha256(Path(__file__).read_bytes()).hexdigest()}
    Path(__file__).with_name('annotation-table-label-reference.json').write_text(json.dumps(report,sort_keys=True,separators=(',',':'))+'\n')
    print({key:value for key,value in report.items() if key != 'source_hashes'})


if __name__ == '__main__':
    generate()
