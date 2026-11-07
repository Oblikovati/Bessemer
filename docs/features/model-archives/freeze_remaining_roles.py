"""Bug #46 segment, annotation-component and increment role boundaries."""
import json, hashlib, struct
from freeze_views import WORKSPACE, REFERENCE, object_tag, view, view_oracle
from freeze_blank_part import reference_snapshot
from freeze_component_references import base
from sketch_reference_fixture_bytes import handle
from freeze_sketch_tail import install
from mfcobjects import ObjectArchive
import dimensionincrements
import modelpostviews

P = struct.pack

def plane(version):
    return object_tag('moCompRefPlane_c')+base(version, False)+P('<iIHi', -2**31, 0xffffffff, 0, -1)

def annotation(version):
    out = object_tag('moAnnotationView_c')+view(version, False, 0x2b)[len(object_tag('moView_c')):]
    out += P('<i', -3)+plane(version)+P('<HidHiiH', 0, 5, -0.0, 0, -7, 11, 0)
    return out+b''.join(P('<i', -13-index) for index, gate in enumerate([0x2b7e,0x2b9a,0x2ba0,0x2bbb]) if version>gate)

def freeze(mode, version, raw):
    loaded = [{'_class':'moSkDimHandleValG2_c'}, {'_class':'moDisplayDistanceDim_c'}] if mode else []
    source = ObjectArchive(raw, version, inherited_map=[None]+loaded)
    source.feature_context = 'ordinary_external_xml'
    install(source)
    source.readers['moAnnotationView_c'] = modelpostviews.annotation_view
    value = dimensionincrements.read(source) if mode else source.object()
    if source.p != len(raw): raise ValueError(f'end {source.p}; expected {len(raw)}')
    oracle = view_oracle('root' if mode else 'root.object', value)
    return P('<BIII',mode,version,len(raw),len(oracle))+raw+oracle, len(oracle.splitlines())

def generate():
    before = reference_snapshot(REFERENCE)
    records=[]
    for version in [0x27cc,19000]:
        section=object_tag('sgPGMSectionHandles_c')+handle('sgSplineHandle',-1)+P('<HH',4,0)
        increment=P('<BHB3iBHBdiBd',255,1,0,-1,7,-2**31,254,2,253,-0.0,-1,252,1.25)
        records += [freeze(0,version,raw) for raw in [plane(version),annotation(version),section]]
        records.append(freeze(1,version,increment))
    if before!=reference_snapshot(REFERENCE): raise ValueError('reference changed; expected stable source hashes')
    packet=b''.join(raw for raw,_ in records)
    (WORKSPACE/'tests/fixtures/model-archives/remaining-role-cases.bin').write_bytes(packet)
    report={'cases':len(records),'fields':sum(n for _,n in records),'sha256':hashlib.sha256(packet).hexdigest()}
    (WORKSPACE/'docs/features/model-archives/remaining-role-cases.json').write_bytes((json.dumps(report)+'\n').encode())
    print(report)

if __name__=='__main__':generate()
