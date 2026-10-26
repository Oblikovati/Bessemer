"""Ordinary model/part tail records with explicit saved node/part context."""
from pathlib import Path
import hashlib
import json
import struct
import sys
from freeze_blank_part import reference_snapshot
from freeze_node_units import object_tag, oracle
from freeze_environment import feature, nodes
from owned_name_edit import short_wide_name
from oracle_projection import physical

WORKSPACE = Path(__file__).resolve().parents[3]
REFERENCE = WORKSPACE.parent/'inventor-interface/solidworks'
sys.path.insert(0,str(REFERENCE))
import modelprefix, modelpostviews, modelarrays, modellayers, modelreftail, sharedmodeltail, partresolved
from mfcobjects import ObjectArchive
from swcontainer import Document


def postviews(version: int, visual: bytes) -> bytes:
    colors = struct.pack('<i',1)+visual+struct.pack('<i',-7) if version == 19000 else struct.pack('<i',0)
    out = struct.pack('<Bii',255,-3,5)+object_tag('moFeatColorTab_c')+colors+b'\0\0'
    out += struct.pack('<ii',-11,-1)+nodes([b'\0\0'])+struct.pack('<i',13)
    out += object_tag('moSketchBlockMgr_c')+feature(version)+bytes(range(16))+struct.pack('<iHiii',-17,4,1,-19,1)+bytes(range(16))
    out += struct.pack('<i',1)+bytes(range(16))+bytes(range(16,32))+struct.pack('<i',-23)+short_wide_name('Ω block')
    out += object_tag('suObList')+struct.pack('<HH',1,0)+struct.pack('<iiiII',-29,31,2,0x89abcdef,0xfedcba98)
    out += object_tag('moFlatPatternColorTab_c')+struct.pack('<8I',*range(8))+struct.pack('<I',0xffffffff)
    if version > 0x2baf: out += struct.pack('<iI',1,0xabcdef01)
    return out


def layer_group() -> bytes:
    out = object_tag('moLayerMgr_c')+struct.pack('<i',1)+object_tag('moLayer_c')+short_wide_name('Ω layer')+b'\x01x'
    out += struct.pack('<IBhfiHH',0x89abcdef,255,-123,1.25,-7,4,2)+object_tag('moLayerMask_c')
    return out+struct.pack('<ii3Ii',3,-3,0,2,0xffffffff,-11)


def increment() -> bytes:
    return struct.pack('<BHB',255,0,255)+short_wide_name('Ω increment')+struct.pack('<iiiBH BdiBd',-3,5,-7,254,0,253,-0.0,-11,252,1.25)


def reference_tail(version: int, wide: bool) -> bytes:
    out = struct.pack('<H',1)+object_tag('moForwardedFeatRefs_c')+struct.pack('<iHi',-3,0,-7)
    out += struct.pack('<iHi',1,0,-11)+struct.pack('<i',1)+feature(version)+struct.pack('<iHH',2,0,0)
    out += struct.pack('<i',2)+struct.pack('<'+str(8 if wide else 6)+'I',*range(8 if wide else 6))
    return out+struct.pack('<ii',-13,1)+increment()+struct.pack('<ii',-17,1)+increment()


def shared_tail(version: int) -> bytes:
    out = struct.pack('<iii4diH',-3,5,-1,1,2,3,4,-7,0)+object_tag('suObList')+struct.pack('<HHi',1,0,-11)
    if version > 0x27d3: out += short_wide_name('Ω first')+b'\x01x'
    if version > 0x27de:
        out += object_tag('moCThreadRefMgr_c')+struct.pack('<iiiHH',1,-13,2,0,0)
        if version > 0x3755: out += struct.pack('<i',-17)
    if version > 0x2b65: out += object_tag('moBendNoteFormat_c')+short_wide_name('Ω bend')+b'\0\0'
    if version > 0x3b05: out += struct.pack('<i',1)+b'\x01a\x01b'+struct.pack('<i4I',2,0x89abcdef,0xfedcba98,1,2)
    if 0x3ef1 < version < 0x3ef4: out += struct.pack('<H',65530)
    return out


def mass_properties() -> bytes:
    return struct.pack('<12d5id4i',*[i/4 for i in range(12)],-1,3,-5,7,-9,-0.0,-11,13,-17,19)+short_wide_name('Ω mass')+b'\x01x'+struct.pack('<i12diH',-23,*[i/8 for i in range(12)],-29,65530)


def part_tail(version: int, cached: bool) -> bytes:
    out = struct.pack('<iBiii',-3,255,5,-7,2)+struct.pack('<iiid',1,-11,13,1.25)+mass_properties()+struct.pack('<iii',0,-17,19)
    out += object_tag('moPrtExplViewManager_c')+feature(version)+b'\0\0'+object_tag('moExplPrtId_c')+short_wide_name('Ω explode')
    out += struct.pack('<idiiHHH',-23,-0.0,-29,31,1,65530,0)+struct.pack('<iii',-37,41,-1)
    out += struct.pack('<iiH iH',2,-43,0,-43,0)+struct.pack('<iiB iB',2,-47,0,-47,255)
    out += struct.pack('<i',1)+object_tag('moPerBodySheetMetalDataNew_c')+struct.pack('<H2IiiHH4i',2,0x89abcdef,0xfedcba98,-53,59,0,0,-61,67,-71,73)
    if cached:
        out += struct.pack('<iii',2,-1,1)+object_tag('moCachedBoundingBoxData_c')+struct.pack('<iii6dii',-79,83,-1,1,2,3,4,5,6,-89,0)
    return out+struct.pack('<iiiii2i',-97,101,-103,-1,2,-107,-107)


def cases(visual: bytes) -> list[tuple[int,int,bytes]]:
    shared_versions = [0x27cc,0x27d3,0x27d4,0x27de,0x27df,0x2b65,0x2b66,0x3755,0x3756,0x3b05,0x3b06,0x3ef1,0x3ef2,0x3ef3,0x3ef4,19000]
    return ([(0,v,postviews(v,visual)) for v in [0x27cc,0x2baf,0x2bb0,19000]]+
            [(1,19000,struct.pack('<HH',1,0)*8),(2,19000,layer_group())]+
            [(k,v,reference_tail(v,k==4)) for k in [3,4] for v in [0x27cc,19000]]+
            [(5,v,shared_tail(v)) for v in shared_versions]+
            [(k,v,part_tail(v,k==6)) for k in [6,7] for v in [0x27cc,19000]])


def read_case(kind: int, archive: ObjectArchive) -> object:
    if kind == 0: return modelpostviews.read(archive)
    if kind == 1: return modelarrays.read(archive)
    if kind == 2: return modellayers.read(archive)
    if kind in (3,4): return modelreftail.read(archive,node_flags=0x100 if kind==4 else 0,root_class='moPart_c')
    if kind == 5: return sharedmodeltail.read(archive)
    return partresolved.read(archive,raw_1614=0 if kind==6 else 1,root_class='moPart_c')


def physical_rows(path: str, value: object, rows: dict[str,str], seen: set[int]) -> None:
    if isinstance(value,(dict,list)):
        if id(value) in seen: return
        seen.add(id(value))
    if isinstance(value,dict):
        for key,child in value.items():
            if key.startswith('_') or key in ('start','end','unknown_bytes','class_counts','new_classes','printable','mode','offset'): continue
            physical_rows(path+'.'+key,child,rows,seen)
    elif isinstance(value,(list,tuple,bytes)):
        for index,child in enumerate(value): physical_rows(f'{path}[{index}]',child,rows,seen)
    elif value is not None:
        from generate_blank_scalars import flatten
        flatten(path,value,rows)


def expected(value: object) -> bytes:
    rows: dict[str,str] = {}
    physical_rows('root',value,rows,set())
    return ''.join(f'{path}={value}\n' for path,value in sorted(rows.items())).encode()


def freeze_case(kind: int, version: int, raw: bytes) -> tuple[bytes,int]:
    archive = ObjectArchive(raw,version)
    archive.feature_context = 'ordinary_external_xml'
    value = read_case(kind,archive)
    if archive.p != len(raw): raise ValueError(f'tail {kind} end {archive.p}; expected {len(raw)}')
    oracle = expected(value)
    return struct.pack('<BIII',kind,version,len(raw),len(oracle))+raw+oracle,len(oracle.splitlines())


def generate() -> None:
    before = reference_snapshot(REFERENCE)
    document = Document(WORKSPACE/'tests/fixtures/blank-part.SLDPRT',table={})
    visual = document.read('Contents/Config-0')[108:953]
    frozen = [freeze_case(*case) for case in cases(visual)]
    root,archive = modelprefix.read(document,through_part_resolved=True)
    start = root['resolved']['postviews']['start']
    raw = archive.b[start:]
    if before != reference_snapshot(REFERENCE): raise ValueError('reference sources changed; expected stable hashes')
    destination = WORKSPACE/'tests/fixtures/model-archives'
    payload = b''.join(wire for wire,_ in frozen)
    (destination/'model-tail-cases.bin').write_bytes(payload)
    (destination/'native-model-tails.bin').write_bytes(raw)
    report = {'cases':len(frozen),'fields':sum(count for _,count in frozen),'sha256':hashlib.sha256(payload).hexdigest(),
              'native':{'start':start,'end':archive.p,'bytes':len(raw),'sha256':hashlib.sha256(raw).hexdigest()},
              'source_hashes':before,'generator_sha256':hashlib.sha256(Path(__file__).read_bytes()).hexdigest()}
    Path(__file__).with_name('model-tail-reference.json').write_text(json.dumps(report,sort_keys=True,separators=(',',':'))+'\n')
    print({key:value for key,value in report.items() if key != 'source_hashes'})


if __name__ == '__main__':
    generate()
