"""Independent weld, geometric-tolerance, datum and tolerance helper bodies."""
from pathlib import Path
import hashlib
import json
import struct
import sys
from freeze_blank_part import reference_snapshot
from freeze_node_units import object_tag, oracle
from freeze_detail_prefix import fields, character
from freeze_detail_helpers import annotation, surface_finish
from owned_name_edit import short_wide_name

WORKSPACE = Path(__file__).resolve().parents[3]
REFERENCE = WORKSPACE.parent/'inventor-interface/solidworks'
sys.path.insert(0,str(REFERENCE))
import modelprefix
import weldgtolhelpers
from mfcobjects import ObjectArchive
from swcontainer import Document


def datum_atom(version: int) -> bytes:
    out = fields([('s','00'),('I','08'),('d','10')])
    if version >= 0x334b: out += fields([('s','18')])
    if 0x3b0e < version < 0x3b1a: out += (struct.pack('<d',-1.25)+short_wide_name('Ωx'))*3
    return out


def datum(version: int, kind: int = 0) -> bytes:
    out = struct.pack('<i',kind)
    if version > 0x373c: out += fields([('I','40')])
    if kind:
        out += datum_atom(version)
    else:
        out += struct.pack('<ii',2,0)
        if version > 0x373c: out += fields([('I','18')])
        out += struct.pack('<i',1)+datum_atom(version)+struct.pack('<i',-7)+datum_atom(version)
    if version > 0x3b19: out += fields([('d','48'),('s','50'),('d','58'),('s','60'),('d','68'),('s','70')])
    return out


def tolerance_values(version: int) -> bytes:
    out = struct.pack('<i',2)
    for _ in range(2):
        out += struct.pack('<i',-3)
        if version >= 0x373f: out += struct.pack('<i',-7)
        out += struct.pack('<Q',0x7ff000000000002a)
        if version >= 0x334b: out += short_wide_name('Ωx')
    out += struct.pack('<11i',*range(-17,-6))
    if version >= 0x373f: out += struct.pack('<i',2)
    out += fields([('i','00 04 08 0c')])*(2 if version >= 0x373f else 1)
    out += fields([('i','58 5c 60 64 68')])
    if version < 0x3724: out += struct.pack('<i',-19)
    out += fields([('i','6c 70')])
    if version > 0x3b10: out += fields([('i','74 78 7c')])
    return out


def tolerance(version: int, present: bool = True) -> bytes:
    return fields([('s','08'),('i','10 14 18')])+struct.pack('<i',-1 if present else 0)+(tolerance_values(version) if present else b'')


def frame(version: int, tolerance_index: int) -> bytes:
    out = fields([('s','08'),('B','28'),('s','30 38'),('B','40'),('s','48 50')])
    out += (struct.pack('<HI',65535,2)+short_wide_name('Ωx')+b'\x01x')*6
    out += fields([('i','20 24')])
    if version >= 0x3715:
        out += struct.pack('<H',65530)+object_tag('moGTolDlgDataTol_c')+tolerance(version)+struct.pack('<H',tolerance_index)
        out += datum(version,-1)+datum(version,0)+datum(version,0)
    if version > 0x3718: out += struct.pack('<i',1)+fields([('i','00'),('s','08 10')])
    return out


def gtol(version: int, tolerance_index: int = 2) -> bytes:
    out = struct.pack('<i',1)+frame(version,tolerance_index)
    out += fields([('i','30 34'),('s','38'),('B','40 4c 50 54'),('i','58'),('B','68'),('s','70 78')])
    if version >= 0x3751: out += fields([('B','80'),('s','88 90'),('B','98'),('s','60')])
    out += fields([('B','9c ac 110 114')])+character(version)+annotation(version)
    out += fields([('B','a0 44 48 a4 a8')])
    if version > 0x3714: out += fields([('B','1e0')])
    return out


def weld(version: int) -> bytes:
    out = b'\xff'
    for row in range(2): out += fields([('s',' '.join(format(base+8*row,'x') for base in range(0x10,0x90,0x10)))])
    out += fields([('s','90'),('B','98 9c a0 a4'),('s','a8'),('B','b0 b4'),('i','b8 c0'),('B','c8 d0 e0'),
                   ('i','bc c4'),('B','cc d4 e4'),('i','d8'),('B','dc e8'),('s','218 220 228 230 238'),('B','240 244 248 24c 250 254 258')])
    return out+character(version)+fields([('B','150')])+character(version)+fields([('B','154')])+character(version)+surface_finish(version)*2


def cases() -> list[tuple[int,int,bytes]]:
    versions = [0x27cc,0x3714,0x3715,0x3718,0x3719,0x3723,0x3724,0x373e,0x373f,0x3750,0x3751,
                0x3b0e,0x3b0f,0x3b10,0x3b11,0x3b19,0x3b1a,0x3eec,0x3eed,19000,0xffffffff]
    datums = [0x334a,0x334b,0x373c,0x373d,0x3b0e,0x3b0f,0x3b19,0x3b1a,19000]
    welds = [0x27cc,0x3eec,0x3eed,0x42e3,0x42e4,0x46bf,0x46c0,19000]
    return ([(0,v,weld(v)) for v in welds]+[(1,v,gtol(v)) for v in versions]+
            [(2,v,datum(v)) for v in datums]+[(3,v,tolerance(v,p)) for v in versions for p in (False,True)]+
            [(4,19000,object_tag('moGTolDataHelper_c')+gtol(19000,4))])


def freeze_case(kind: int, version: int, raw: bytes) -> tuple[bytes,int]:
    archive = ObjectArchive(raw,version)
    archive.feature_context = 'ordinary_external_xml'
    archive.readers = dict(archive.readers,moGTolDlgDataTol_c=weldgtolhelpers.tolerance,moGTolDataHelper_c=weldgtolhelpers.gtol)
    value = [lambda: weldgtolhelpers.weld(archive),lambda: weldgtolhelpers.gtol(archive),
             lambda: weldgtolhelpers.datum(archive),lambda: weldgtolhelpers.tolerance(archive),lambda: archive.object()][kind]()
    if archive.p != len(raw): raise ValueError(f'helper {kind} end {archive.p}; expected {len(raw)}')
    expected = oracle('root.object' if kind == 4 else 'root',value)
    return struct.pack('<BIII',kind,version,len(raw),len(expected))+raw+expected,len(expected.splitlines())


def native_group(destination: Path) -> dict[str,object]:
    root,archive = modelprefix.read(Document(WORKSPACE/'tests/fixtures/blank-part.SLDPRT',table={}),through_weld_gtol_helpers=True)
    start = root['resolved']['weld_gtol_helpers_start']
    value = {'helpers':[item['data'] for item in root['resolved']['weld_gtol_helpers']['helpers']]}
    raw,expected = archive.b[start:archive.p],oracle('root',value)
    (destination/'native-weld-helpers.bin').write_bytes(raw)
    (destination/'native-weld-helpers.txt').write_bytes(expected)
    return {'start':start,'end':archive.p,'bytes':len(raw),'fields':len(expected.splitlines()),'sha256':hashlib.sha256(raw).hexdigest()}


def generate() -> None:
    before = reference_snapshot(REFERENCE)
    frozen = [freeze_case(*case) for case in cases()]
    destination = WORKSPACE/'tests/fixtures/model-archives'
    native = native_group(destination)
    if before != reference_snapshot(REFERENCE): raise ValueError('reference sources changed; expected stable hashes')
    payload = b''.join(raw for raw,_ in frozen)
    (destination/'weld-helper-cases.bin').write_bytes(payload)
    report = {'cases':len(frozen),'fields':sum(count for _,count in frozen),'sha256':hashlib.sha256(payload).hexdigest(),
              'native':native,'source_hashes':before,'generator_sha256':hashlib.sha256(Path(__file__).read_bytes()).hexdigest()}
    Path(__file__).with_name('weld-helper-reference.json').write_text(json.dumps(report,sort_keys=True,separators=(',',':'))+'\n')
    print({key:value for key,value in report.items() if key != 'source_hashes'})


if __name__ == '__main__':
    generate()
