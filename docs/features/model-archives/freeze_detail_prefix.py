"""Independent detail-default prefix and character-format wire cases."""
from pathlib import Path
import hashlib
import json
import struct
import sys
from freeze_blank_part import reference_snapshot
from freeze_node_units import oracle
from owned_name_edit import short_wide_name

WORKSPACE = Path(__file__).resolve().parents[3]
REFERENCE = WORKSPACE.parent / "inventor-interface/solidworks"
sys.path.insert(0, str(REFERENCE))
import modelprefix
import detaildefinitions
from mfcobjects import ObjectArchive
from swcontainer import Document

EARLY = [('i','10'),('s','13a0'),('d','28'),('B','30'),('I','20'),('i','24'),('d','08'),
         ('B','38 14 18 1c 34'),('i','1388'),('I','ec f0 f4'),('d','f8 100 108'),
         ('I','110 114 118'),('d','120 128 130'),('i','138'),('d','140 148'),
         ('i','e8 150 154 158 164 168 16c 170 174')]
MIDDLE = [('B','3c'),('i','138c 1390 1394 1398'),('B','40 44'),('H','e0 e2'),
          ('i','4c 48'),('d','2bc0'),('i','2bc8'),('s','2bd8'),('i','2be0'),
          ('s','2be8'),('i','2bf0 178 17c 1a8 1ac 1b0')]
VERSIONS = [0x27cc,0x2bb3,0x2bb4,0x374e,0x374f,0x3b0f,0x3b10,0x3eec,0x3eed,19000,0xffffffff]


def scalar(kind: str, offset: int) -> bytes:
    if kind == 's': return short_wide_name('Ωx')
    if kind == 'd': return struct.pack('<Q', 0x8000000000000000 if offset % 2 else 0x7ff000000000002a)
    value = {'i':-offset-7,'I':0xf0000000|offset,'B':(offset+3)%256,'H':65500+offset%20}[kind]
    return struct.pack('<'+kind, value)


def fields(recipe: list[tuple[str, str]]) -> bytes:
    return b''.join(scalar(kind,int(offset,16)) for kind,offsets in recipe for offset in offsets.split())


def character(version: int) -> bytes:
    out = short_wide_name('Ω Font')
    out += fields([('d','10'),('i','18'),('d','20'),('I','30'),('d','28')])
    out += struct.pack('<Q',0x8000000000000000) + fields([('d','40 48 50')])
    if version >= 0x3eed: out += fields([('I','58')])
    return out


def prefix(version: int) -> bytes:
    out = fields(EARLY+MIDDLE)
    if version > 0x2bb3: out += fields([('i','1b4')])
    out += fields([('i','180 184 188 18c 190 194')])
    out += struct.pack('<i',2) + short_wide_name('Ωx') + b'\x01x'
    out += fields([('i','198 1a0 1a4')])
    rows, columns = (3,8) if version > 0x374e else (2,5)
    for base in (0x228,0x288):
        for row in range(3 if base == 0x288 else rows):
            for column in range(columns): out += scalar('i',base+row*0x20+column*4)
    if version > 0x3b0f: out += struct.pack('<HI',65535,2) + short_wide_name('Ωx') + b'\x01x'
    return out + character(version)


def physical(value: dict[str, object]) -> dict[str, object]:
    return {key: child for key, child in value.items() if key != 'character_format_start'}


def freeze_case(kind: int, version: int, raw: bytes) -> tuple[bytes, int]:
    archive = ObjectArchive(raw,version)
    archive.feature_context = 'ordinary_external_xml'
    value = detaildefinitions.prefix(archive) if kind == 0 else detaildefinitions.character_format(archive)
    if archive.p != len(raw): raise ValueError(f'case {kind} end {archive.p}; expected {len(raw)}')
    expected = oracle('root',physical(value))
    return struct.pack('<BIII',kind,version,len(raw),len(expected)) + raw + expected, len(expected.splitlines())


def native_prefix(destination: Path) -> dict[str, object]:
    root, archive = modelprefix.read(Document(WORKSPACE/'tests/fixtures/blank-part.SLDPRT',table={}),through_detail_prefix=True)
    start = root['resolved']['detail_prefix_start']
    raw = archive.b[start:archive.p]
    expected = oracle('root',physical(root['resolved']['detail_prefix']))
    (destination/'native-detail-prefix.bin').write_bytes(raw)
    (destination/'native-detail-prefix.txt').write_bytes(expected)
    return {'start':start,'end':archive.p,'bytes':len(raw),'fields':len(expected.splitlines()),'sha256':hashlib.sha256(raw).hexdigest()}


def generate() -> None:
    before = reference_snapshot(REFERENCE)
    cases = [freeze_case(kind,version,prefix(version) if kind == 0 else character(version))
             for version in VERSIONS for kind in (0,1)]
    destination = WORKSPACE/'tests/fixtures/model-archives'
    native = native_prefix(destination)
    if before != reference_snapshot(REFERENCE): raise ValueError('reference sources changed; expected stable hashes')
    payload = b''.join(raw for raw,_ in cases)
    (destination/'detail-prefix-cases.bin').write_bytes(payload)
    report = {'cases':len(cases),'fields':sum(count for _,count in cases),'sha256':hashlib.sha256(payload).hexdigest(),
              'native':native,'source_hashes':before,'generator_sha256':hashlib.sha256(Path(__file__).read_bytes()).hexdigest()}
    Path(__file__).with_name('detail-prefix-reference.json').write_text(json.dumps(report,sort_keys=True,separators=(',',':'))+'\n')
    print({key:value for key,value in report.items() if key != 'source_hashes'})


if __name__ == '__main__':
    generate()
