"""Independent complete body-feature/reference-plane/edge fixture oracle."""
import hashlib,json
from pathlib import Path
from freeze_views import REFERENCE,WORKSPACE,object_tag,view_oracle
from freeze_blank_part import reference_snapshot
from freeze_boundary_references import base as boundary_base
from sketch_tail_fixture_bytes import WireMap,P
import body_feature_fixture_bytes as wire
import reference_plane_fixture_bytes as planes
import bodyfeatures,referenceplanes,modelboundary
from mfcobjects import ObjectArchive

def freeze(version,raw):
    a=ObjectArchive(raw,version)
    a.feature_context='ordinary_external_xml';a.model_root_class='moPart_c'
    bodyfeatures.install_scoped(a);referenceplanes.install_scoped(a)
    a.readers.update(moEndSpec_c=bodyfeatures.end_spec,moRevEndSpec_c=bodyfeatures.revolution_end_spec,moFromEndSpec_c=bodyfeatures.from_end_spec)
    value=a.object()
    if a.p!=len(raw):raise ValueError(f'body end {a.p}; expected {len(raw)}')
    oracle=view_oracle('root.object',value)
    return P('<BIII',0,version,len(raw),len(oracle))+raw+oracle,len(oracle.splitlines())

def generate():
    before=reference_snapshot(REFERENCE)
    versions=[0x27cc,0x2b83,0x2b84,0x2ba5,0x2ba6,0x2f85,0x2f86,0x3b00,0x3b01,0x3f04,0x3f05,19000]
    records=[]
    for version in versions:
        for rich in [False,True]:
            records += [freeze(version,wire.tagged(version,name,rich)) for name in wire.CLASSES]
            records += [freeze(version,planes.tagged(version,name,rich)) for name in planes.CLASSES]
            records.append(freeze(version,planes.plane(version,rich)))
            for name,serializer in [('moEndSpec_c',wire.end_spec),('moRevEndSpec_c',wire.revolution_end)]:
                w=WireMap();records.append(freeze(version,w.new(name,lambda:serializer(w,version,rich))))
        for kind in [-2**31,0,1,2,3,4,5,6,2**31-1]:
            records.append(freeze(version,object_tag('moFromEndSpec_c')+wire.from_spec(version,kind)))
        edge=object_tag('moSilhEdgeRef_c')+boundary_base(version,False)+bytes(6)+P('<I2d',0xffffffff,-0.0,1.25)
        records.append(freeze(version,edge))
    packet=b''.join(raw for raw,_ in records)
    edits={f'body-feature-{name}.bin':wire.tagged(19000,'moBoss_c',True,value) for name,value in [('original',-2**31),('edited',12345)]}
    for raw in edits.values():freeze(19000,raw)
    if before!=reference_snapshot(REFERENCE):raise ValueError('reference changed; expected stable source hashes')
    (WORKSPACE/'tests/fixtures/model-archives/body-feature-cases.bin').write_bytes(packet)
    for name,raw in edits.items():(WORKSPACE/'tests/fixtures/model-archives'/name).write_bytes(raw)
    report={'cases':len(records),'fields':sum(n for _,n in records),'sha256':hashlib.sha256(packet).hexdigest(),'source_hashes':before}
    Path(__file__).with_name('body-feature-cases.json').write_bytes((json.dumps(report)+'\n').encode())
    print({k:v for k,v in report.items() if k!='source_hashes'})

if __name__=='__main__':generate()
