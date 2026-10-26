"""Explicit ordinary feature/environment records and model light payloads."""
from pathlib import Path
import hashlib
import json
import struct
import sys
from freeze_blank_part import reference_snapshot
from freeze_node_units import object_tag, node_wire, oracle
from owned_name_edit import short_wide_name
from oracle_projection import physical

WORKSPACE = Path(__file__).resolve().parents[3]
REFERENCE = WORKSPACE.parent/'inventor-interface/solidworks'
sys.path.insert(0,str(REFERENCE))
import modelprefix
import modelafterdetails
import modelfeatures
import dl_light
from mfcobjects import ObjectArchive
from swcontainer import Document

LIGHTS = ['moLightFeature_c','moAmbientLight_c','moDirectionLight_c','moSpotLight_c','moPointLight_c']


def nodes(children: list[bytes]) -> bytes:
    return struct.pack('<HI',65535,len(children))+b''.join(children)+struct.pack('<i',-19)


def feature(version: int, merged: bool = True, children: list[bytes] | None = None) -> bytes:
    out = node_wire(version)+struct.pack('<i',-1 if merged else 0)
    if merged: out += nodes([b'\0\0',b'\0\0'])
    out += b'\0\0'+nodes(children or [])
    out += struct.pack('<iiIB',-3,5,0xfedcba98,255)+short_wide_name('Ω optional')
    out += struct.pack('<4idi',-7,11,-13,17,-0.0,-19)+b'\0\0'+b'\x01x'+b'\xff'
    out += struct.pack('<HHBIHhf',65530,65529,255,0x89abcdef,65528,-123,1.25)
    out += struct.pack('<iiH',-23,29,0)
    out += struct.pack('<IBBiiiIIHii',0xffffffff,254,253,-31,37,-41,0x89abcdef,0xfedcba98,65527,-43,47)
    return out


def light(version: int, name: str) -> bytes:
    out = feature(version,False)+struct.pack('<i',-53)
    out += struct.pack('<3dI4i',-0.0,1.25,2.5,0x89abcdef,-1,3,-5,7)+short_wide_name('Ω light')
    out += struct.pack('<iiidi d',-11,13,-17,1.5,-19,-0.0)
    out += struct.pack('<IIfIiI',3,0,1.25,1,-23,2)+short_wide_name('Ω property')
    if name == 'moDirectionLight_c': out += struct.pack('<4d',1,2,3,-0.0)
    if name == 'moSpotLight_c': out += struct.pack('<14di2d',*[i/4 for i in range(14)],-29,3.5,3.75)
    if name == 'moPointLight_c': out += struct.pack('<4di2d',1,2,3,4,-31,5,6)
    return object_tag(name)+out


def environment(version: int) -> bytes:
    out = struct.pack('<HIHH',65535,2,0,0)
    out += object_tag('moEnvFolder_c')+feature(version,children=[light(version,'moAmbientLight_c')])+struct.pack('<ii',-3,5)
    out += b'\0\0'+object_tag('suObList')+struct.pack('<HHH',2,0,0)
    if version < 0x3734: out += struct.pack('<I',1)+short_wide_name('Ω plane')+struct.pack('<18d',*[i/8 for i in range(18)])
    return out


def cases() -> list[tuple[int,int,bytes]]:
    result = [(0,v,feature(v,merged)) for v in [0x27cc,19000] for merged in [False,True]]
    result += [(1,v,light(v,name)) for v in [0x27cc,19000,0xffffffff] for name in LIGHTS]
    result += [(2,v,environment(v)) for v in [0x27cc,0x3733,0x3734,19000,0xffffffff]]
    return result


def freeze_case(kind: int, version: int, raw: bytes) -> tuple[bytes,int]:
    archive = ObjectArchive(raw,version)
    archive.feature_context = 'ordinary_external_xml'
    modelafterdetails.install_scoped(archive)
    value = modelfeatures.feature(archive) if kind == 0 else archive.object() if kind == 1 else modelafterdetails.read(archive,through_environment=True)
    if kind == 2:
        value = {key:child for key,child in value.items() if key not in ('environment_start','environment_end','environment_light_counts','end')}
    if archive.p != len(raw): raise ValueError(f'environment case {kind} end {archive.p}; expected {len(raw)}')
    expected = oracle('root.object' if kind == 1 else 'root',physical(value))
    expected = expected.replace(b'.user_property[', b'.user_property.entries[')
    return struct.pack('<BIII',kind,version,len(raw),len(expected))+raw+expected,len(expected.splitlines())


def native_group(destination: Path) -> dict[str,object]:
    root,archive = modelprefix.read(Document(WORKSPACE/'tests/fixtures/blank-part.SLDPRT',table={}),through_environment=True)
    start = root['resolved']['after_details_start']
    value = {key:child for key,child in root['resolved']['after_details'].items()
             if key not in ('environment_start','environment_end','environment_light_counts','end')}
    raw,expected = archive.b[start:archive.p],oracle('root',physical(value))
    expected = expected.replace(b'.user_property[', b'.user_property.entries[')
    (destination/'native-environment.bin').write_bytes(raw)
    (destination/'native-environment.txt').write_bytes(expected)
    return {'start':start,'end':archive.p,'bytes':len(raw),'fields':len(expected.splitlines()),'sha256':hashlib.sha256(raw).hexdigest()}


def generate() -> None:
    before = reference_snapshot(REFERENCE)
    frozen = [freeze_case(*case) for case in cases()]
    destination = WORKSPACE/'tests/fixtures/model-archives'
    native = native_group(destination)
    if before != reference_snapshot(REFERENCE): raise ValueError('reference sources changed; expected stable hashes')
    payload = b''.join(raw for raw,_ in frozen)
    (destination/'environment-cases.bin').write_bytes(payload)
    report = {'cases':len(frozen),'fields':sum(count for _,count in frozen),'sha256':hashlib.sha256(payload).hexdigest(),
              'native':native,'source_hashes':before,'generator_sha256':hashlib.sha256(Path(__file__).read_bytes()).hexdigest()}
    Path(__file__).with_name('environment-reference.json').write_text(json.dumps(report,sort_keys=True,separators=(',',':'))+'\n')
    print({key:value for key,value in report.items() if key != 'source_hashes'})


if __name__ == '__main__':
    generate()
